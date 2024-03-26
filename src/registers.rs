use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum Register {
    Reg0(Register0) = 0,
    Reg1(Register1) = 1,
    Reg2(Register2) = 2,
    Reg3(Register3) = 3,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Register0 {
    pub mux: Mux,
    pub gain: Gain,
    pub pga_bypass: bool,
}

impl From<Register0> for u8 {
    #[inline(always)]
    fn from(value: Register0) -> Self {
        let mux = value.mux as u8;
        let gain = value.gain as u8;
        let pga_bypass = value.pga_bypass as u8;
        (mux << 4) | (gain << 1) | pga_bypass
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Register1 {
    pub data_rate: DataRate,
    pub conversion_mode: ConversionMode,
    pub voltage_reference: Vref,
    pub temperature_sensor_mode: bool,
}

impl From<Register1> for u8 {
    #[inline(always)]
    fn from(value: Register1) -> Self {
        let dr = (value.data_rate as u8) & 0x0F;
        let mode = (((value.data_rate as u8) & 0xF0) != 0) as u8;
        let cm = value.conversion_mode as u8;
        let vref = value.voltage_reference as u8;
        let ts = value.temperature_sensor_mode as u8;

        (dr << 5) | (mode << 4) | (cm << 3) | (vref << 1) | ts
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Register2 {
    pub data_ready: bool,
    pub data_count_enable: bool,
    pub data_integrity_mode: DataIntegrityMode,
    pub burn_out_source_enable: bool,
    pub current_dac: CurrentDac,
}

impl From<Register2> for u8 {
    #[inline(always)]
    fn from(value: Register2) -> Self {
        let drdy = value.data_ready as u8;
        let dcnt = value.data_count_enable as u8;
        let crc = value.data_integrity_mode as u8;
        let bcs = value.burn_out_source_enable as u8;
        let idac = value.current_dac as u8;

        (drdy << 7) | (dcnt << 6) | (crc << 4) | (bcs << 3) | idac
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Register3 {
    pub current_mux_1: CurrentMux,
    pub current_mux_2: CurrentMux,
}

impl From<Register3> for u8 {
    #[inline(always)]
    fn from(value: Register3) -> Self {
        let cm1 = value.current_mux_1 as u8;
        let cm2 = value.current_mux_2 as u8;
        (cm1 << 5) | (cm2 << 2)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum Mux {
    /// AINp = A0, AINn = A1
    A0A1 = 0,
    /// AINp = A0, AINn = A2
    A0A2 = 1,
    /// AINp = A0, AINn = A3
    A0A3 = 2,
    /// AINp = A1, AINn = A0
    A1A0 = 3,
    /// AINp = A1, AINn = A2
    A1A2 = 4,
    /// AINp = A1, AINn = A3
    A1A3 = 5,
    /// AINp = A2, AINn = A3
    A2A3 = 6,
    /// AINp = A3, AINn = A2
    A3A2 = 7,
    /// AINp = A0, AINn = Vss
    A0Vss = 8,
    /// AINp = A1, AINn = Vss
    A1Vss = 9,
    /// AINp = A2, AINn = Vss
    A2Vss = 10,
    /// AINp = A3, AINn = Vss
    A3Vss = 11,
    /// `(Vrefp - Vrefn) / 4`
    Vref = 12,
    /// `(Avdd - Avss) / 4`
    Supply = 13,
    /// Both AINp and AINn shorted to `(AVDD + AVSS)/2`
    Shorted = 14,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum Gain {
    X1 = 0,
    X2 = 1,
    X4 = 2,
    X8 = 3,
    X16 = 4,
    X32 = 5,
    X64 = 6,
    X128 = 7,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum DataRate {
    N20 = 0x00,
    N45 = 0x01,
    N90 = 0x02,
    N175 = 0x03,
    N330 = 0x04,
    N600 = 0x05,
    N1000 = 0x06,
    
    T40 = 0x10,
    T90 = 0x11,
    T180 = 0x12,
    T350 = 0x13,
    T660 = 0x14,
    T1200 = 0x15,
    T2000 = 0x16,
}

impl DataRate {
    pub(crate) fn from_dr_mode(data_rate: u8, turbo_mode: bool) -> Self {
        match (turbo_mode, data_rate) {
            (false, 0x00) => DataRate::N20,
            (false, 0x01) => DataRate::N45,
            (false, 0x02) => DataRate::N90,
            (false, 0x03) => DataRate::N175,
            (false, 0x04) => DataRate::N330,
            (false, 0x05) => DataRate::N600,
            (false, 0x06) => DataRate::N1000,
            (false, _) => DataRate::N20,
            (true, 0x00) => DataRate::T40,
            (true, 0x01) => DataRate::T90,
            (true, 0x02) => DataRate::T180,
            (true, 0x03) => DataRate::T350,
            (true, 0x04) => DataRate::T660,
            (true, 0x05) => DataRate::T1200,
            (true, 0x06) => DataRate::T2000,
            (true, _) => DataRate::T40,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum ConversionMode {
    Single = 0,
    Continuous = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum Vref {
    Internal = 0,
    External = 1,
    Supply = 2,
    Supply2 = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum DataIntegrityMode {
    Disabled = 0,
    InvertedData = 1,
    Crc16 = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum CurrentDac {
    Off = 0,
    I10uA = 1,
    I50uA = 2,
    I100uA = 3,
    I250uA = 4,
    I500uA = 5,
    I1000uA = 6,
    I1500uA = 7,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum CurrentMux {
    Disabled = 0,
    Ain0 = 1,
    Ain1 = 2,
    Ain2 = 3,
    Ain3 = 4,
    Refp = 5,
    Refn = 6,
}
