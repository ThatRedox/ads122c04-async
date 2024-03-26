#![doc = include_str!("../README.md")]
#![no_std]

mod registers;

use embedded_hal_async::i2c::{I2c, SevenBitAddress};
pub use registers::*;

pub struct ADS122C04<I: I2c<SevenBitAddress>> {
    i2c: I,
    address: SevenBitAddress,
}

impl<I: I2c<SevenBitAddress>> ADS122C04<I> {
    /// Create a new device from an I2C peripheral and address.
    pub fn new(i2c: I, address: SevenBitAddress) -> Self {
        Self {
            i2c,
            address,
        }
    }
    
    /// Reset the device
    pub async fn reset(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0b0000_0110]).await
    }
    
    /// Start or restart conversions
    pub async fn start_sync(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0b0000_1000]).await
    }
    
    /// Enter power down mode
    pub async fn power_down(&mut self) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0b0000_0010]).await
    }
    
    /// Read data by command
    pub async fn read_data<const N: usize>(&mut self) -> Result<[u8; N], I::Error> {
        let mut out = [0u8; N];
        self.i2c.write_read(self.address, &[0b0001_0000], &mut out).await?;
        Ok(out)
    }
    
    async fn read_reg_raw(&mut self, reg: u8) -> Result<u8, I::Error> {
        let cmd = 0b0010_0000 | (reg << 2);
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.address, &[cmd], &mut buf).await?;
        Ok(buf[0])
    }
    
    async fn write_reg_raw(&mut self, reg: u8, value: u8) -> Result<(), I::Error> {
        let cmd = 0b0100_0000 | (reg << 2);
        self.i2c.write(self.address, &[cmd, value]).await
    }
    
    /// Read register 0
    pub async fn read_reg0(&mut self) -> Result<Register0, I::Error> {
        let value = self.read_reg_raw(0).await?;
        let mux = value >> 4;
        let gain = (value >> 1) & 0b111;
        let pga_bypass = value & 0b1;

        Ok(Register0 {
            mux: Mux::try_from(mux).unwrap_or(Mux::A0A1),
            gain: Gain::try_from(gain).unwrap_or(Gain::X1),
            pga_bypass: pga_bypass != 0,
        })
    }
    
    /// Read register 1
    pub async fn read_reg1(&mut self) -> Result<Register1, I::Error> {
        let value = self.read_reg_raw(1).await?;
        let dr = (value >> 5) & 0b111;
        let mode = (value >> 4) != 0;
        let cm = (value >> 3) & 0b1;
        let vref = (value >> 1) & 0b11;
        let ts = value & 0b1;
        
        Ok(Register1 {
            data_rate: DataRate::from_dr_mode(dr, mode),
            conversion_mode: ConversionMode::try_from(cm).unwrap_or(ConversionMode::Single),
            voltage_reference: Vref::try_from(vref).unwrap_or(Vref::Internal),
            temperature_sensor_mode: ts != 0,
        })
    }
    
    /// Read register 2
    pub async fn read_reg2(&mut self) -> Result<Register2, I::Error> {
        let value = self.read_reg_raw(2).await?;
        let drdy = (value >> 7) != 0;
        let dcnt = (value >> 6) != 0;
        let crc = (value >> 4) & 0b11;
        let bcs = (value >> 3) != 0;
        let idac = value & 0b11;
        
        Ok(Register2 {
            data_ready: drdy,
            data_count_enable: dcnt,
            data_integrity_mode: DataIntegrityMode::try_from(crc).unwrap_or(DataIntegrityMode::Disabled),
            burn_out_source_enable: bcs,
            current_dac: CurrentDac::try_from(idac).unwrap_or(CurrentDac::Off),
        })
    }
    
    /// Read register 3
    pub async fn read_reg3(&mut self) -> Result<Register3, I::Error> {
        let value = self.read_reg_raw(3).await?;
        let i1mux = (value >> 5) & 0b111;
        let i2mux = (value >> 2) & 0b111;
        
        Ok(Register3 {
            current_mux_1: CurrentMux::try_from(i1mux).unwrap_or(CurrentMux::Disabled),
            current_mux_2: CurrentMux::try_from(i2mux).unwrap_or(CurrentMux::Disabled),
        })
    }
    
    /// Write register 0
    pub async fn write_reg0(&mut self, value: Register0) -> Result<(), I::Error> {
        self.write_reg_raw(0, value.into()).await
    }
    
    /// Write register 1
    pub async fn write_reg1(&mut self, value: Register1) -> Result<(), I::Error> {
        self.write_reg_raw(1, value.into()).await
    }
    
    /// Write register 2
    pub async fn write_reg2(&mut self, value: Register2) -> Result<(), I::Error> {
        self.write_reg_raw(2, value.into()).await
    }
    
    /// Write register 3
    pub async fn write_reg3(&mut self, value: Register3) -> Result<(), I::Error> {
        self.write_reg_raw(3, value.into()).await
    }
}
