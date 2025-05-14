use std::{collections::HashMap, fmt::Debug, time::Duration};

use async_trait::async_trait;
use bitvec::{field::BitField, order::Msb0, view::BitView};
use rsiot::{
    components_config::{
        master_device::{self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
        spi_master::{self},
    },
    message::{Message, MsgDataBound},
};
use strum::FromRepr;
use tokio::sync::{broadcast, mpsc};
use tracing::info;

use crate::chips::ad7190::{self, SRChannel};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_input: fn(&Message<TMsg>, &mut Buffer),
    pub fn_output: fn(&mut Buffer) -> Vec<Message<TMsg>>,
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
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
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
                    ad7190::SpiOperations::reset(),
                );
                requests.push(request);

                // Первый запрос к чипу возвращает почему-то ff. Запрашиваем статус
                // let spi_operations = vec![ad7190::SpiOperations::read_status_register()];
                // let request = spi_master::FieldbusRequest::new(
                //     RequestKind::ReadStatusRegister,
                //     spi_operations,
                // );
                // requests.push(request);

                // Регистр конфигурации
                let reg = &buffer.write_registers.conf_register;
                let spi_operations = vec![ad7190::SpiOperations::write_configuration_register(reg)];
                let request = spi_master::FieldbusRequest::new(
                    RequestKind::WriteConfigurationRegister,
                    spi_operations,
                );
                requests.push(request);

                // Регистр режима работы
                let reg = &buffer.write_registers.mode_register;
                let spi_operations = vec![ad7190::SpiOperations::write_mode_register(reg)];
                let request = spi_master::FieldbusRequest::new(
                    RequestKind::WriteModeRegister,
                    spi_operations,
                );
                requests.push(request);

                // Читаем все регистры
                let spi_operations = vec![
                    ad7190::SpiOperations::read_status_register(),
                    ad7190::SpiOperations::read_mode_register(),
                    ad7190::SpiOperations::read_configuration_register(),
                    ad7190::SpiOperations::read_gpocon_register(),
                    ad7190::SpiOperations::read_fullscale_registers(),
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

                        // let spi_operations =
                        //     vec![ad7190::SpiOperations::read_configuration_register()];
                        // let request = spi_master::FieldbusRequest::new(
                        //     RequestKind::ReadConfigurationRegister,
                        //     spi_operations,
                        // );
                        // requests.push(request);

                        // Записываем регистр конфигурации, если он изменился
                        if buffer.write_registers.conf_register
                            != buffer.read_registers.conf_register
                        {
                            let reg = &buffer.write_registers.conf_register;
                            let spi_operations =
                                vec![ad7190::SpiOperations::write_configuration_register(reg)];
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
                                vec![ad7190::SpiOperations::write_mode_register(reg)];
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
                                vec![ad7190::SpiOperations::write_gpocon_register(reg)];
                            let request = spi_master::FieldbusRequest::new(
                                RequestKind::WriteGPOCONRegister,
                                spi_operations,
                            );
                            requests.push(request);
                        }

                        // Читаем все регистры
                        let spi_operations = vec![
                            ad7190::SpiOperations::read_status_register(),
                            ad7190::SpiOperations::read_mode_register(),
                            ad7190::SpiOperations::read_configuration_register(),
                            ad7190::SpiOperations::read_gpocon_register(),
                            ad7190::SpiOperations::read_fullscale_registers(),
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
                    period: Duration::from_millis(50),
                    fn_requests: |_| {
                        let mut requests = vec![];

                        let spi_operations = vec![ad7190::SpiOperations::read_data_register()];
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
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.into();
                let response_payload = response.payload;

                match request_kind {
                    RequestKind::Reset => (),
                    RequestKind::ReadIdRegister => {
                        info!("Read ID Register: {:x?}", response_payload[0]);
                    }
                    RequestKind::ReadConfigurationRegister => {
                        let con_reg = ad7190::ConfigurationRegister::decode(&response_payload[0]);
                        buffer.read_registers.conf_register = con_reg;
                    }
                    RequestKind::RequestAllRegisters => {
                        let status_register =
                            ad7190::StatusRegister::decode(response_payload[0][0]);
                        buffer.read_registers.status_register = status_register;

                        let mode_reg = ad7190::ModeRegister::decode(&response_payload[1]);
                        buffer.read_registers.mode_register = mode_reg;

                        let con_reg = ad7190::ConfigurationRegister::decode(&response_payload[2]);
                        buffer.read_registers.conf_register = con_reg;

                        let gpocon = ad7190::GPOCONRegister::decode(response_payload[3][0]);
                        buffer.read_registers.gpocon_register = gpocon;

                        // info!("Read data: {:x?}", buffer.read_registers);
                    }
                    RequestKind::WriteConfigurationRegister => (),
                    RequestKind::WriteModeRegister => (),
                    RequestKind::WriteGPOCONRegister => (),
                    RequestKind::ReadStatusRegister => (),
                    RequestKind::ReadDataRegister => {
                        let status_register =
                            ad7190::StatusRegister::decode(response_payload[0][3]);

                        if let ad7190::SRReady::NotReady = status_register.ready {
                            return Ok(());
                        }

                        let bytes = &response_payload[0][0..=2];
                        let bits = bytes.view_bits::<Msb0>();
                        let value = bits.load_be::<u32>();

                        let value = match status_register.channel {
                            SRChannel::Temperature => code_to_temp(value),
                            _ => code_to_voltage(
                                value,
                                buffer.read_registers.conf_register.gain,
                                buffer.read_registers.conf_register.polarity,
                            ),
                        };

                        buffer
                            .read_registers
                            .data_registers
                            .insert(status_register.channel.clone(), value);
                        // info!("Data: {:?}", buffer.read_registers.data_registers);
                    }
                }
                Ok(())
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                write_registers: WriteRegisters {
                    conf_register: ad7190::ConfigurationRegister {
                        chop: ad7190::CONChop::Enabled,
                        refsel: ad7190::CONRefSel::RefIn1,
                        channel_selected: ad7190::CONChannelSelect {
                            ain1_ain2: true,
                            ain3_ain4: true,
                            temperature: true,
                            ain2_ain2: false,
                            ain1_aincom: false,
                            ain2_aincom: false,
                            ain3_aincom: false,
                            ain4_aincom: false,
                        },
                        burn: ad7190::CONBurn::Disabled,
                        ref_det: ad7190::CONRefDet::Enabled,
                        buffer: ad7190::CONBuffer::Enabled,
                        polarity: ad7190::CONPolarity::Bipolar,
                        gain: ad7190::CONGain::_128,
                    },
                    mode_register: ad7190::ModeRegister {
                        mode: ad7190::MRMode::ContinuosConversion,
                        transmit_status: ad7190::MRTransmitStatus::Enabled,
                        clock_source: ad7190::MRClockSource::InternalClock,
                        filter: ad7190::MRFilter::Sync4,
                        parity: ad7190::MRParity::Enabled,
                        single: ad7190::MRSingle::Disabled,
                        reject60: ad7190::MRReject60::Disabled,
                        filter_word: ad7190::MRFilterWord(50),
                    },
                    gpocon_register: ad7190::GPOCONRegister {
                        bpdsw: ad7190::GPBpdsw::On,
                        gp10en: ad7190::GPB10en::Disabled,
                        gp32en: ad7190::GPB32en::Disabled,
                        p0_state: ad7190::GPPinOutputState::Off,
                        p1_state: ad7190::GPPinOutputState::Off,
                        p2_state: ad7190::GPPinOutputState::Off,
                        p3_state: ad7190::GPPinOutputState::Off,
                    },
                },
                ..Default::default()
            },
        };
        device
            .spawn(
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await
            .unwrap();
        Ok(())
    }
}

#[derive(FromRepr)]
pub enum RequestKind {
    RequestAllRegisters,
    WriteConfigurationRegister,
    WriteModeRegister,
    WriteGPOCONRegister,
    ReadConfigurationRegister,
    ReadStatusRegister,
    ReadDataRegister,
    Reset,
    ReadIdRegister,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}

impl From<u8> for RequestKind {
    fn from(value: u8) -> Self {
        RequestKind::from_repr(value as usize).unwrap()
    }
}

// Buffer ------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub write_registers: WriteRegisters,
    pub read_registers: ReadRegisters,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WriteRegisters {
    pub conf_register: ad7190::ConfigurationRegister,
    pub mode_register: ad7190::ModeRegister,
    pub gpocon_register: ad7190::GPOCONRegister,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReadRegisters {
    pub status_register: ad7190::StatusRegister,
    pub mode_register: ad7190::ModeRegister,
    pub conf_register: ad7190::ConfigurationRegister,
    pub data_registers: HashMap<ad7190::SRChannel, f64>,
    pub gpocon_register: ad7190::GPOCONRegister,
}

impl Buffer {}

impl BufferBound for Buffer {}

/// Преобразовываем полученный код с АЦП в напряжение
///
/// Напряжение выражено как доля от Vref
fn code_to_voltage(code: u32, gain: ad7190::CONGain, polarity: ad7190::CONPolarity) -> f64 {
    let code = code as f64;
    let gain: f64 = gain.into();
    const RESOLUTION: f64 = 24_f64;

    match polarity {
        ad7190::CONPolarity::Bipolar => {
            let coef = 2_f64.powf(RESOLUTION - 1.0);
            (code / coef - 1.0) / gain
        }
        ad7190::CONPolarity::Unipolar => {
            let coef = 2_f64.powf(RESOLUTION);
            code / (coef * gain)
        }
    }
}

/// Преобразовываем полученный код с АЦП в температуру
///
/// Температура выражена в Кельвинах
fn code_to_temp(code: u32) -> f64 {
    const CODE_AT_0K: f64 = 0x800000 as f64;

    const SENSITIVITY: f64 = 2815.0;

    (code as f64 - CODE_AT_0K) / SENSITIVITY
}
