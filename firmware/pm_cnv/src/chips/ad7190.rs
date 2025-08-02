use core::f64;
use std::time::Duration;

use bitvec::{
    bitvec, field::BitField, order::Msb0, prelude::BitVec, slice::BitSlice, view::BitView,
};
use rsiot::components_config::spi_master::Operation;

pub struct SpiOperations {}

impl SpiOperations {
    pub fn reset() -> Vec<Operation> {
        vec![
            Operation::Write(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
            Operation::Delay(Duration::from_millis(5)),
        ]
    }

    pub fn read_status_register() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::Status,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 1)
    }

    pub fn read_mode_register() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::Mode,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 3)
    }

    pub fn write_mode_register(value: &ModeRegister) -> Operation {
        let mut write_buffer = vec![];

        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Write,
            reg_address: CRRegAddress::Mode,
        };
        write_buffer.push(comm_reg.encode());

        let mode_reg = value.encode();
        write_buffer.extend(mode_reg);

        Operation::Write(write_buffer)
    }

    pub fn read_id_register() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::Id,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 1)
    }

    pub fn read_configuration_register() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::Configuration,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 3)
    }

    pub fn write_configuration_register(value: &ConfigurationRegister) -> Operation {
        let mut write_buffer = vec![];

        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Write,
            reg_address: CRRegAddress::Configuration,
        };
        write_buffer.push(comm_reg.encode());

        let conf_reg = value.encode();
        write_buffer.extend(conf_reg);

        Operation::Write(write_buffer)
    }

    pub fn read_data_register() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::Data,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 4)
    }

    pub fn read_gpocon_register() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::Gpocon,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 1)
    }

    pub fn write_gpocon_register(value: &GPOCONRegister) -> Operation {
        let mut write_buffer = vec![];

        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Write,
            reg_address: CRRegAddress::Gpocon,
        };
        write_buffer.push(comm_reg.encode());

        let gpocon_reg = value.encode();
        write_buffer.push(gpocon_reg);

        Operation::Write(write_buffer)
    }

    pub fn read_fullscale_registers() -> Operation {
        let comm_reg = CommunicationRegister {
            read_write: CRReadWrite::Read,
            reg_address: CRRegAddress::FullScale,
        };
        let bytes = vec![comm_reg.encode()];

        Operation::WriteRead(bytes, 3)
    }
}

// Communication register --------------------------------------------------------------------------
const RW: usize = 1;
const RS2: usize = 2;
const RS1: usize = 3;
const RS0: usize = 4;

pub struct CommunicationRegister {
    pub read_write: CRReadWrite,
    pub reg_address: CRRegAddress,
}

impl CommunicationRegister {
    pub fn encode(&self) -> u8 {
        let mut bytes: u8 = 0;
        let bits = bytes.view_bits_mut::<Msb0>();

        bits.set(RW, self.read_write.encode());

        let reg_address = self.reg_address.encode();
        bits.set(RS2, reg_address[0]);
        bits.set(RS1, reg_address[1]);
        bits.set(RS0, reg_address[2]);

        bytes
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CRReadWrite {
    #[default]
    Read,
    Write,
}
impl CRReadWrite {
    pub fn encode(&self) -> bool {
        match self {
            CRReadWrite::Write => false,
            CRReadWrite::Read => true,
        }
    }
}

/// Register address bits
#[derive(Clone, Debug, Default, PartialEq)]
pub enum CRRegAddress {
    #[default]
    Status,
    Mode,
    Configuration,
    Data,
    Id,
    Gpocon,
    Offset,
    FullScale,
}
impl CRRegAddress {
    pub fn encode(&self) -> [bool; 3] {
        match self {
            CRRegAddress::Status => [false, false, false],
            CRRegAddress::Mode => [false, false, true],
            CRRegAddress::Configuration => [false, true, false],
            CRRegAddress::Data => [false, true, true],
            CRRegAddress::Id => [true, false, false],
            CRRegAddress::Gpocon => [true, false, true],
            CRRegAddress::Offset => [true, true, false],
            CRRegAddress::FullScale => [true, true, true],
        }
    }
}

// Status register ---------------------------------------------------------------------------------
const RDY: usize = 0;
const ERR: usize = 1;
const NOREF: usize = 2;
const PARITY: usize = 3;
const CHD2: usize = 5;
const CHD1: usize = 6;
const CHD0: usize = 7;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatusRegister {
    pub ready: SRReady,
    pub error: SRError,
    pub no_reference: SRNoReference,
    pub parity: SRParity,
    pub channel: SRChannel,
}
impl StatusRegister {
    pub fn decode(bytes: u8) -> Self {
        let bits = bytes.view_bits::<Msb0>();
        Self {
            ready: SRReady::decode(bits[RDY]),
            error: SRError::decode(bits[ERR]),
            no_reference: SRNoReference::decode(bits[NOREF]),
            parity: SRParity::decode(bits[PARITY]),
            channel: SRChannel::decode(bits[CHD2], bits[CHD1], bits[CHD0]),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SRReady {
    Ready,
    #[default]
    NotReady,
}
impl SRReady {
    pub fn decode(bit: bool) -> Self {
        if bit {
            Self::NotReady
        } else {
            Self::Ready
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SRError {
    #[default]
    NoError,
    Error,
}
impl SRError {
    pub fn decode(bit: bool) -> Self {
        if bit {
            Self::Error
        } else {
            Self::NoError
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SRNoReference {
    #[default]
    ReferenceOk,
    NoReference,
}
impl SRNoReference {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::ReferenceOk,
            true => Self::NoReference,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SRParity {
    #[default]
    EvenNumberOf1,
    OddNumberOf1,
}
impl SRParity {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::EvenNumberOf1,
            true => Self::OddNumberOf1,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum SRChannel {
    #[default]
    Ain1Ain2,
    Ain3Ain4,
    Temperature,
    Ain2Ain2,
    Ain1Aincom,
    Ain2Aincom,
    Ain3Aincom,
    Ain4Aincom,
}
impl SRChannel {
    pub fn decode(chd2: bool, chd1: bool, chd0: bool) -> Self {
        match (chd2, chd1, chd0) {
            (false, false, false) => Self::Ain1Ain2,
            (false, false, true) => Self::Ain3Ain4,
            (false, true, false) => Self::Temperature,
            (false, true, true) => Self::Ain2Ain2,
            (true, false, false) => Self::Ain1Aincom,
            (true, false, true) => Self::Ain2Aincom,
            (true, true, false) => Self::Ain3Aincom,
            (true, true, true) => Self::Ain4Aincom,
        }
    }
}

// Mode register -----------------------------------------------------------------------------------

const MD2: usize = 0;
const MD1: usize = 1;
const MD0: usize = 2;
const DAT_STA: usize = 3;
const CLK1: usize = 4;
const CLK0: usize = 5;
const SYNC3: usize = 8;
const ENPAR: usize = 10;
const SINGLE: usize = 12;
const REJ60: usize = 13;
const FS9: usize = 14;
const FS8: usize = 15;
const FS7: usize = 16;
const FS6: usize = 17;
const FS5: usize = 18;
const FS4: usize = 19;
const FS3: usize = 20;
const FS2: usize = 21;
const FS1: usize = 22;
const FS0: usize = 23;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModeRegister {
    /// Mode
    pub mode: MRMode,
    /// 1 = contents of the status register are transmitted along with each data register read
    pub transmit_status: MRTransmitStatus,
    /// Clock source
    pub clock_source: MRClockSource,
    pub filter: MRFilter,
    pub parity: MRParity,
    pub single: MRSingle,
    pub reject60: MRReject60,
    pub filter_word: MRFilterWord,
}
impl ModeRegister {
    pub fn decode(bytes: &[u8]) -> Self {
        if bytes.len() != 3 {
            panic!("Invalid bytes length");
        }
        let bits = bytes.view_bits::<Msb0>();

        Self {
            mode: MRMode::decode(bits[MD2], bits[MD1], bits[MD0]),
            transmit_status: MRTransmitStatus::decode(bits[DAT_STA]),
            clock_source: MRClockSource::decode(bits[CLK1], bits[CLK0]),
            filter: MRFilter::decode(bits[SYNC3]),
            parity: MRParity::decode(bits[ENPAR]),
            single: MRSingle::decode(bits[SINGLE]),
            reject60: MRReject60::decode(bits[REJ60]),
            filter_word: MRFilterWord::decode(&bits[FS9..=FS0]),
        }
    }
    pub fn encode(&self) -> [u8; 3] {
        let mut bytes = [0; 3];
        let bits = bytes.view_bits_mut::<Msb0>();

        let mode = self.mode.encode();
        bits.set(MD2, mode[0]);
        bits.set(MD1, mode[1]);
        bits.set(MD0, mode[2]);

        bits.set(DAT_STA, self.transmit_status.encode());

        let clock_source = self.clock_source.encode();
        bits.set(CLK1, clock_source[0]);
        bits.set(CLK0, clock_source[1]);

        bits.set(SYNC3, self.filter.encode());

        bits.set(ENPAR, self.parity.encode());

        bits.set(SINGLE, self.single.encode());

        bits.set(REJ60, self.reject60.encode());

        let filter_word = self.filter_word.encode();
        bits.set(FS9, filter_word[0]);
        bits.set(FS8, filter_word[1]);
        bits.set(FS7, filter_word[2]);
        bits.set(FS6, filter_word[3]);
        bits.set(FS5, filter_word[4]);
        bits.set(FS4, filter_word[5]);
        bits.set(FS3, filter_word[6]);
        bits.set(FS2, filter_word[7]);
        bits.set(FS1, filter_word[8]);
        bits.set(FS0, filter_word[9]);

        bytes
    }
}

/// Mode
#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRMode {
    #[default]
    ContinuosConversion,
    SingleConversion,
    Idle,
    PowerDown,
    InternalZeroScaleCalibration,
    InternalFullScaleCalibration,
    SystemZeroScaleCalibration,
    SystemFullScaleCalibration,
}
impl MRMode {
    pub fn decode(md2: bool, md1: bool, md0: bool) -> Self {
        match (md2, md1, md0) {
            (false, false, false) => MRMode::ContinuosConversion,
            (false, false, true) => MRMode::SingleConversion,
            (false, true, false) => MRMode::Idle,
            (false, true, true) => MRMode::PowerDown,
            (true, false, false) => MRMode::InternalZeroScaleCalibration,
            (true, false, true) => MRMode::InternalFullScaleCalibration,
            (true, true, false) => MRMode::SystemZeroScaleCalibration,
            (true, true, true) => MRMode::SystemFullScaleCalibration,
        }
    }
    pub fn encode(&self) -> [bool; 3] {
        match self {
            MRMode::ContinuosConversion => [false, false, false],
            MRMode::SingleConversion => [false, false, true],
            MRMode::Idle => [false, true, false],
            MRMode::PowerDown => [false, true, true],
            MRMode::InternalZeroScaleCalibration => [true, false, false],
            MRMode::InternalFullScaleCalibration => [true, false, true],
            MRMode::SystemZeroScaleCalibration => [true, true, false],
            MRMode::SystemFullScaleCalibration => [true, true, true],
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRTransmitStatus {
    #[default]
    Disabled,
    Enabled,
}
impl MRTransmitStatus {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }
    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

/// Clock source
#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRClockSource {
    ExternalCrystal,
    ExternalClock,
    #[default]
    InternalClock,
    InternalClock2,
}
impl MRClockSource {
    pub fn decode(clk1: bool, clk0: bool) -> Self {
        match (clk1, clk0) {
            (false, false) => MRClockSource::ExternalCrystal,
            (false, true) => MRClockSource::ExternalClock,
            (true, false) => MRClockSource::InternalClock,
            (true, true) => MRClockSource::InternalClock2,
        }
    }

    pub fn encode(&self) -> [bool; 2] {
        match self {
            MRClockSource::ExternalCrystal => [false, false],
            MRClockSource::ExternalClock => [false, true],
            MRClockSource::InternalClock => [true, false],
            MRClockSource::InternalClock2 => [true, true],
        }
    }
}

/// Filter
#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRFilter {
    Sync3,
    #[default]
    Sync4,
}
impl MRFilter {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => MRFilter::Sync4,
            true => MRFilter::Sync3,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            MRFilter::Sync4 => false,
            MRFilter::Sync3 => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRParity {
    #[default]
    Disabled,
    Enabled,
}
impl MRParity {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRSingle {
    #[default]
    Disabled,
    Enabled,
}
impl MRSingle {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => MRSingle::Disabled,
            true => MRSingle::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum MRReject60 {
    #[default]
    Disabled,
    Enabled,
}
impl MRReject60 {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MRFilterWord(pub u32);
impl MRFilterWord {
    pub fn decode(bits: &BitSlice<u8, Msb0>) -> Self {
        let bits = bits.load_be::<u32>();
        Self(bits)
    }

    pub fn encode(&self) -> BitVec<u8, Msb0> {
        let mut bits = bitvec![u8, Msb0; 0; 10];
        bits.store_be(self.0);
        bits
    }
}

// Configuration register --------------------------------------------------------------------------

const CHOP: usize = 0;
const REFSEL: usize = 3;
const CH7: usize = 8;
const CH6: usize = 9;
const CH5: usize = 10;
const CH4: usize = 11;
const CH3: usize = 12;
const CH2: usize = 13;
const CH1: usize = 14;
const CH0: usize = 15;
const BURN: usize = 16;
const REFDET: usize = 17;
const BUFFER: usize = 19;
const U_B: usize = 20;
const G2: usize = 21;
const G1: usize = 22;
const G0: usize = 23;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigurationRegister {
    pub chop: CONChop,
    pub refsel: CONRefSel,
    pub channel_selected: CONChannelSelect,
    pub burn: CONBurn,
    pub ref_det: CONRefDet,
    pub buffer: CONBuffer,
    pub polarity: CONPolarity,
    pub gain: CONGain,
}
impl ConfigurationRegister {
    pub fn decode(bytes: &[u8]) -> Self {
        if bytes.len() != 3 {
            panic!("Invalid bytes length");
        }
        let bits = bytes.view_bits::<Msb0>();

        Self {
            chop: CONChop::decode(bits[CHOP]),
            refsel: CONRefSel::decode(bits[REFSEL]),
            channel_selected: CONChannelSelect::decode(&bits[CH7..=CH0]),
            burn: CONBurn::decode(bits[BURN]),
            ref_det: CONRefDet::decode(bits[REFDET]),
            buffer: CONBuffer::decode(bits[BUFFER]),
            polarity: CONPolarity::decode(bits[U_B]),
            gain: CONGain::decode(bits[G2], bits[G1], bits[G0]),
        }
    }

    pub fn encode(&self) -> [u8; 3] {
        let mut bytes = [0; 3];
        let bits = bytes.view_bits_mut::<Msb0>();

        bits.set(CHOP, self.chop.encode());

        bits.set(REFSEL, self.refsel.encode());

        let channel_selected = self.channel_selected.encode();
        bits.set(CH7, channel_selected[0]);
        bits.set(CH6, channel_selected[1]);
        bits.set(CH5, channel_selected[2]);
        bits.set(CH4, channel_selected[3]);
        bits.set(CH3, channel_selected[4]);
        bits.set(CH2, channel_selected[5]);
        bits.set(CH1, channel_selected[6]);
        bits.set(CH0, channel_selected[7]);

        bits.set(BURN, self.burn.encode());

        bits.set(REFDET, self.ref_det.encode());

        bits.set(BUFFER, self.buffer.encode());

        bits.set(U_B, self.polarity.encode());

        let gain = self.gain.encode();
        bits.set(G2, gain[0]);
        bits.set(G1, gain[1]);
        bits.set(G0, gain[2]);

        bytes
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CONChop {
    #[default]
    Disabled,
    Enabled,
}
impl CONChop {
    pub fn decode(chop: bool) -> Self {
        match chop {
            false => CONChop::Disabled,
            true => CONChop::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            CONChop::Disabled => false,
            CONChop::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CONRefSel {
    #[default]
    RefIn1,
    RefIn2,
}
impl CONRefSel {
    pub fn decode(chop: bool) -> Self {
        match chop {
            false => Self::RefIn1,
            true => Self::RefIn2,
        }
    }
    pub fn encode(&self) -> bool {
        match self {
            Self::RefIn1 => false,
            Self::RefIn2 => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CONChannelSelect {
    pub ain1_ain2: bool,
    pub ain3_ain4: bool,
    pub temperature: bool,
    pub ain2_ain2: bool,
    pub ain1_aincom: bool,
    pub ain2_aincom: bool,
    pub ain3_aincom: bool,
    pub ain4_aincom: bool,
}
impl CONChannelSelect {
    pub fn decode(bits: &BitSlice<u8, Msb0>) -> Self {
        Self {
            ain1_ain2: bits[7],
            ain3_ain4: bits[6],
            temperature: bits[5],
            ain2_ain2: bits[4],
            ain1_aincom: bits[3],
            ain2_aincom: bits[2],
            ain3_aincom: bits[1],
            ain4_aincom: bits[0],
        }
    }

    pub fn encode(&self) -> [bool; 8] {
        let mut bits = [false; 8];

        bits[7] = self.ain1_ain2;
        bits[6] = self.ain3_ain4;
        bits[5] = self.temperature;
        bits[4] = self.ain2_ain2;
        bits[3] = self.ain1_aincom;
        bits[2] = self.ain2_aincom;
        bits[1] = self.ain3_aincom;
        bits[0] = self.ain4_aincom;

        bits
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CONBurn {
    #[default]
    Disabled,
    Enabled,
}
impl CONBurn {
    pub fn decode(burn: bool) -> Self {
        match burn {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CONRefDet {
    #[default]
    Disabled,
    Enabled,
}
impl CONRefDet {
    pub fn decode(ref_det: bool) -> Self {
        match ref_det {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }
    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CONBuffer {
    #[default]
    Disabled,
    Enabled,
}
impl CONBuffer {
    pub fn decode(buffer: bool) -> Self {
        match buffer {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }
    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CONPolarity {
    #[default]
    Bipolar,
    Unipolar,
}
impl CONPolarity {
    pub fn decode(u_b: bool) -> Self {
        match u_b {
            false => Self::Bipolar,
            true => Self::Unipolar,
        }
    }
    pub fn encode(&self) -> bool {
        match self {
            Self::Bipolar => false,
            Self::Unipolar => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CONGain {
    _1,
    Reserved,
    _8,
    _16,
    _32,
    _64,
    #[default]
    _128,
}
impl CONGain {
    pub fn decode(g2: bool, g1: bool, g0: bool) -> Self {
        match (g2, g1, g0) {
            (false, false, false) => Self::_1,
            (false, false, true) => Self::Reserved,
            (false, true, false) => Self::Reserved,
            (false, true, true) => Self::_8,
            (true, false, false) => Self::_16,
            (true, false, true) => Self::_32,
            (true, true, false) => Self::_64,
            (true, true, true) => Self::_128,
        }
    }

    pub fn encode(&self) -> [bool; 3] {
        match self {
            Self::_1 => [false, false, false],
            Self::Reserved => [false, false, true],
            Self::_8 => [false, true, true],
            Self::_16 => [true, false, false],
            Self::_32 => [true, false, true],
            Self::_64 => [true, true, false],
            Self::_128 => [true, true, true],
        }
    }
}
impl From<CONGain> for f64 {
    fn from(value: CONGain) -> Self {
        match value {
            CONGain::_1 => 1.0,
            CONGain::Reserved => f64::INFINITY,
            CONGain::_8 => 8.0,
            CONGain::_16 => 16.0,
            CONGain::_32 => 32.0,
            CONGain::_64 => 64.0,
            CONGain::_128 => 128.0,
        }
    }
}

// GPOCON register ---------------------------------------------------------------------------------

const BPDSW: usize = 1;
const GP32EN: usize = 2;
const GP10EN: usize = 3;
const P3DAT: usize = 4;
const P2DAT: usize = 5;
const P1DAT: usize = 6;
const P0DAT: usize = 7;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GPOCONRegister {
    pub bpdsw: GPBpdsw,
    pub gp10en: GPB10en,
    pub gp32en: GPB32en,
    pub p0_state: GPPinOutputState,
    pub p1_state: GPPinOutputState,
    pub p2_state: GPPinOutputState,
    pub p3_state: GPPinOutputState,
}
impl GPOCONRegister {
    pub fn decode(byte: u8) -> Self {
        let bits = byte.view_bits::<Msb0>();

        Self {
            bpdsw: GPBpdsw::decode(bits[BPDSW]),
            gp10en: GPB10en::decode(bits[GP10EN]),
            gp32en: GPB32en::decode(bits[GP32EN]),
            p0_state: GPPinOutputState::decode(bits[P0DAT]),
            p1_state: GPPinOutputState::decode(bits[P1DAT]),
            p2_state: GPPinOutputState::decode(bits[P2DAT]),
            p3_state: GPPinOutputState::decode(bits[P3DAT]),
        }
    }

    pub fn encode(&self) -> u8 {
        let mut bytes: u8 = 0;
        let bits = bytes.view_bits_mut::<Msb0>();

        bits.set(BPDSW, self.bpdsw.encode());

        bits.set(GP32EN, self.gp32en.encode());

        bits.set(GP10EN, self.gp10en.encode());

        bits.set(P0DAT, self.p0_state.encode());

        bits.set(P1DAT, self.p1_state.encode());

        bits.set(P2DAT, self.p2_state.encode());

        bits.set(P3DAT, self.p3_state.encode());

        bytes
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum GPBpdsw {
    #[default]
    Off,
    On,
}
impl GPBpdsw {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Off,
            true => Self::On,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            GPBpdsw::Off => false,
            GPBpdsw::On => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum GPB32en {
    #[default]
    Disabled,
    Enabled,
}
impl GPB32en {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum GPB10en {
    #[default]
    Disabled,
    Enabled,
}
impl GPB10en {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Disabled,
            true => Self::Enabled,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum GPPinOutputState {
    #[default]
    Off,
    On,
}
impl GPPinOutputState {
    pub fn decode(bit: bool) -> Self {
        match bit {
            false => Self::Off,
            true => Self::On,
        }
    }

    pub fn encode(&self) -> bool {
        match self {
            Self::Off => false,
            Self::On => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let conf_register = ConfigurationRegister {
            chop: CONChop::Enabled,
            refsel: CONRefSel::RefIn1,
            channel_selected: CONChannelSelect {
                ain1_ain2: true,
                ain3_ain4: false,
                temperature: true,
                ain2_ain2: false,
                ain1_aincom: false,
                ain2_aincom: false,
                ain3_aincom: false,
                ain4_aincom: false,
            },
            burn: CONBurn::Disabled,
            ref_det: CONRefDet::Enabled,
            buffer: CONBuffer::Enabled,
            polarity: CONPolarity::Bipolar,
            gain: CONGain::_16,
        };

        let encode = conf_register.encode();

        println!("Encoded: {encode:x?}");
        let decode = ConfigurationRegister::decode(&encode);
        assert_eq!(conf_register, decode);
    }
}
