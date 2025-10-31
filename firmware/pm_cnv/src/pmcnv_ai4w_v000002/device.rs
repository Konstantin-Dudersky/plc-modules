use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bitvec::{field::BitField, order::Msb0, view::BitView};
use rsiot::{
    components_config::{
        master_device::{self, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
        spi_master::{self},
    },
    executor::MsgBusInput,
    message::{Message, MsgDataBound},
};
use tokio::sync::mpsc;
use tracing::info;

use crate::chips::ad7193;

use super::{
    buffer::{Buffer, WriteRegisters},
    request_kind::RequestKind,
};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_input: fn(&TMsg, &mut Buffer),
    pub fn_output: fn(&mut Buffer) -> Vec<TMsg>,
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse>
    for Device<TMsg>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: MsgBusInput<TMsg>,
        ch_tx_device_to_fieldbus: mpsc::Sender<spi_master::FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<spi_master::FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<
            TMsg,
            spi_master::FieldbusRequest,
            spi_master::FieldbusResponse,
            Buffer,
        > = DeviceBase {
            fn_init_requests: |buffer| {
                let mut requests = vec![];

                let request = spi_master::FieldbusRequest::new(
                    RequestKind::Reset,
                    ad7193::SpiOperations::reset(),
                );
                requests.push(request);

                // Регистр конфигурации
                let reg = &buffer.write_registers.conf_register;
                let spi_operations = vec![ad7193::SpiOperations::write_configuration_register(reg)];
                let request = spi_master::FieldbusRequest::new(
                    RequestKind::WriteConfigurationRegister,
                    spi_operations,
                );
                requests.push(request);

                // Регистр режима работы
                let reg = &buffer.write_registers.mode_register;
                let spi_operations = vec![ad7193::SpiOperations::write_mode_register(reg)];
                let request = spi_master::FieldbusRequest::new(
                    RequestKind::WriteModeRegister,
                    spi_operations,
                );
                requests.push(request);

                // Читаем все регистры
                let spi_operations = vec![
                    ad7193::SpiOperations::read_status_register(),
                    ad7193::SpiOperations::read_mode_register(),
                    ad7193::SpiOperations::read_configuration_register(),
                    ad7193::SpiOperations::read_gpocon_register(),
                    ad7193::SpiOperations::read_fullscale_registers(),
                ];
                let request = spi_master::FieldbusRequest::new(
                    RequestKind::RequestAllRegisters,
                    spi_operations,
                );
                requests.push(request);

                requests
            },
            periodic_requests: vec![
                ConfigPeriodicRequest {
                    period: Duration::from_millis(1000),
                    fn_requests: |buffer| {
                        let mut requests = vec![];

                        // Записываем регистр конфигурации, если он изменился
                        if buffer.write_registers.conf_register
                            != buffer.read_registers.conf_register
                        {
                            let reg = &buffer.write_registers.conf_register;
                            let spi_operations =
                                vec![ad7193::SpiOperations::write_configuration_register(reg)];
                            let request = spi_master::FieldbusRequest::new(
                                RequestKind::WriteConfigurationRegister,
                                spi_operations,
                            );
                            requests.push(request);
                        }

                        // Записываем регистр режима работы, если он изменился
                        if buffer.write_registers.mode_register
                            != buffer.read_registers.mode_register
                        {
                            let reg = &buffer.write_registers.mode_register;
                            let spi_operations =
                                vec![ad7193::SpiOperations::write_mode_register(reg)];
                            let request = spi_master::FieldbusRequest::new(
                                RequestKind::WriteModeRegister,
                                spi_operations,
                            );
                            requests.push(request);
                        }

                        // Записываем регистр GPOCON, если он изменился
                        if buffer.write_registers.gpocon_register
                            != buffer.read_registers.gpocon_register
                        {
                            let reg = &buffer.write_registers.gpocon_register;
                            let spi_operations =
                                vec![ad7193::SpiOperations::write_gpocon_register(reg)];
                            let request = spi_master::FieldbusRequest::new(
                                RequestKind::WriteGPOCONRegister,
                                spi_operations,
                            );
                            requests.push(request);
                        }

                        // Читаем все регистры
                        let spi_operations = vec![
                            ad7193::SpiOperations::read_status_register(),
                            ad7193::SpiOperations::read_mode_register(),
                            ad7193::SpiOperations::read_configuration_register(),
                            ad7193::SpiOperations::read_gpocon_register(),
                            ad7193::SpiOperations::read_fullscale_registers(),
                        ];
                        let request = spi_master::FieldbusRequest::new(
                            RequestKind::RequestAllRegisters,
                            spi_operations,
                        );
                        requests.push(request);

                        Ok(requests)
                    },
                },
                // Периодический запрос сконвертированных данных
                ConfigPeriodicRequest {
                    period: Duration::from_millis(100),
                    fn_requests: |_| {
                        let mut requests = vec![];

                        let spi_operations = vec![ad7193::SpiOperations::read_data_register()];
                        let request = spi_master::FieldbusRequest::new(
                            RequestKind::ReadDataRegister,
                            spi_operations,
                        );
                        requests.push(request);

                        Ok(requests)
                    },
                },
            ],
            fn_msgs_to_buffer: self.fn_input,
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.try_into()?;
                let response_payload = response.payload;

                match request_kind {
                    RequestKind::Reset => (),
                    RequestKind::ReadIdRegister => {
                        info!("Read ID Register: {:x?}", response_payload[0]);
                    }
                    RequestKind::ReadConfigurationRegister => {
                        let con_reg = ad7193::ConfigurationRegister::decode(&response_payload[0]);
                        buffer.read_registers.conf_register = con_reg;
                    }
                    RequestKind::RequestAllRegisters => {
                        let status_register = ad7193::StatusRegister::decode(
                            response_payload[0][0],
                            &buffer.write_registers.conf_register.pseudo,
                        );
                        buffer.read_registers.status_register = status_register;

                        let mode_reg = ad7193::ModeRegister::decode(&response_payload[1]);
                        buffer.read_registers.mode_register = mode_reg;

                        let con_reg = ad7193::ConfigurationRegister::decode(&response_payload[2]);
                        buffer.read_registers.conf_register = con_reg;

                        let gpocon = ad7193::GPOCONRegister::decode(response_payload[3][0]);
                        buffer.read_registers.gpocon_register = gpocon;
                    }
                    RequestKind::WriteConfigurationRegister => (),
                    RequestKind::WriteModeRegister => (),
                    RequestKind::WriteGPOCONRegister => (),
                    RequestKind::ReadStatusRegister => (),
                    RequestKind::ReadDataRegister => {
                        let status_register = ad7193::StatusRegister::decode(
                            response_payload[0][3],
                            &buffer.write_registers.conf_register.pseudo,
                        );

                        if let ad7193::SRReady::NotReady = status_register.ready {
                            return Ok(false);
                        }

                        let bytes = &response_payload[0][0..=2];
                        let bits = bytes.view_bits::<Msb0>();
                        let value = bits.load_be::<u32>();

                        let value = match status_register.channel {
                            ad7193::SRChannel::Temperature => code_to_temp(value),
                            _ => code_to_voltage(
                                value,
                                buffer.read_registers.conf_register.gain,
                                buffer.read_registers.conf_register.polarity,
                            ),
                        };

                        buffer
                            .read_registers
                            .data_registers
                            .set_field(status_register.channel, value);
                    }
                }
                Ok(false)
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                write_registers: WriteRegisters {
                    conf_register: ad7193::ConfigurationRegister {
                        chop: ad7193::CONChop::Enabled,
                        refsel: ad7193::CONRefSel::RefIn1,
                        pseudo: ad7193::CONPseudo::Disabled,
                        channel_selected: ad7193::CONChannelSelect {
                            ch0: true,
                            ch1: true,
                            ch2: true,
                            ch3: true,
                            ch4: true,
                            ch5: true,
                            ch6: true,
                            ch7: true,
                            temp: true,
                            short: false,
                        },
                        burn: ad7193::CONBurn::Disabled,
                        ref_det: ad7193::CONRefDet::Enabled,
                        buffer: ad7193::CONBuffer::Enabled,
                        polarity: ad7193::CONPolarity::Bipolar,
                        gain: ad7193::CONGain::_128,
                    },
                    mode_register: ad7193::ModeRegister {
                        mode: ad7193::MRMode::ContinuosConversion,
                        transmit_status: ad7193::MRTransmitStatus::Enabled,
                        clock_source: ad7193::MRClockSource::InternalClock,
                        fast_settling_filter: ad7193::MRFastSettlingFilter::NoAveraging,
                        filter: ad7193::MRFilter::Sync4,
                        parity: ad7193::MRParity::Enabled,
                        clock_divide: ad7193::MRClockDivide::Disabled,
                        single: ad7193::MRSingle::Disabled,
                        reject60: ad7193::MRReject60::Disabled,
                        filter_word: ad7193::MRFilterWord(50),
                    },
                    gpocon_register: ad7193::GPOCONRegister {
                        bpdsw: ad7193::GPBpdsw::On,
                        gp10en: ad7193::GPB10en::Disabled,
                        gp32en: ad7193::GPB32en::Disabled,
                        p0_state: ad7193::GPPinOutputState::Off,
                        p1_state: ad7193::GPPinOutputState::Off,
                        p2_state: ad7193::GPPinOutputState::Off,
                        p3_state: ad7193::GPPinOutputState::Off,
                    },
                },
                ..Default::default()
            },
        };
        device
            .spawn(
                "ai4w".to_string(),
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await?;
        Err(master_device::Error::EndExecution)
    }
}

/// Преобразуем полученный код с АЦП в напряжение
///
/// Напряжение выражено как доля от Vref
fn code_to_voltage(code: u32, gain: ad7193::CONGain, polarity: ad7193::CONPolarity) -> f64 {
    let code = code as f64;
    let gain: f64 = gain.into();
    const RESOLUTION: f64 = 24_f64;

    match polarity {
        ad7193::CONPolarity::Bipolar => {
            let coef = 2_f64.powf(RESOLUTION - 1.0);
            (code / coef - 1.0) / gain
        }
        ad7193::CONPolarity::Unipolar => {
            let coef = 2_f64.powf(RESOLUTION);
            code / (coef * gain)
        }
    }
}

/// Преобразуем полученный код с АЦП в температуру
///
/// Температура выражена в Кельвинах
fn code_to_temp(code: u32) -> f64 {
    const CODE_AT_0K: f64 = 0x800000 as f64;

    const SENSITIVITY: f64 = 2815.0;

    (code as f64 - CODE_AT_0K) / SENSITIVITY
}
