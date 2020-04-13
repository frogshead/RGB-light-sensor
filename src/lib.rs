#![deny(unsafe_code)]

extern crate embedded_hal;

use embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};
const DEVICE_ADDRESS: u8 = 0b100_0100;
const DEVICE_ID: u8 = 0x7D;

#[derive(Debug)]
pub enum Error<E> {
    I2C(E),
    InvalidInputData,
    WrongDeviceId,
}
struct LedCounter {
    red: Option<u16>,
    green: Option<u16>,
    blue: Option<u16>,
}
pub struct Isl29125<I2C> {
    i2c: I2C,
    led_counts: LedCounter,
}

impl<I2C, E> Isl29125<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Isl29125 {
            i2c,
            led_counts: LedCounter {
                red: None,
                green: None,
                blue: None,
            },
        }
    }

    pub fn verify_device_id(&mut self) -> Result<u8, Error<E>> {
        let id = self.read_register(RegisterMap::DEVICE_ID);
        match id {
            Ok(id) => {
                if id == DEVICE_ID {
                    return Ok(id);
                }
                Err(Error::WrongDeviceId)
            }
            Err(e) => Err(e),
        }
    }
    fn read_led_counters(&mut self) -> Result<(), Error<E>>{
        let mut data: [u8; 6] = [0, 0, 0,0,0,0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[RegisterMap::GREEN_DATA_LOW_BYTE], &mut data)
            .map_err(Error::I2C)
            .and(Ok(self.set_count_values(data)))
            
            
        
        
    }

    fn set_count_values(&mut self, values: [u8; 6]) {
        self.led_counts.green = Some((((values[1] & 0x00FF) as u16) << 8) | (values[0] & 0xFF) as u16);
        self.led_counts.red =   Some((((values[3] & 0x00FF) as u16) << 8) | (values[2] & 0xFF) as u16);
        self.led_counts.blue = Some((((values[5] & 0x00FF) as u16) << 8) | (values[3] & 0xFF) as u16);
        

    }
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}

struct RegisterMap;
impl RegisterMap {
    const DEVICE_ID: u8 = 0x00;
    const RESET: u8 = 0x00;
    const CONFIGURATION1: u8 = 0x01;
    const CONFIGURATION2: u8 = 0x02;
    const CONFIGURATION3: u8 = 0x03;
    const LOW_THRESHOLD_LOW_BYTE: u8 = 0x04;
    const LOW_THRESHOLD_HIGH_BYTE: u8 = 0x05;
    const HIGH_THRESHOLD_LOW_BYTE: u8 = 0x06;
    const HIGH_THRESHOLD_HIGH_BYTE: u8 = 0x07;
    const STATUS_FLAGS: u8 = 0x08;
    const GREEN_DATA_LOW_BYTE: u8 = 0x09;
    const GREEN_DATA_HIGH_BYTE: u8 = 0x0a;
    const RED_DATA_LOW_BYTE: u8 = 0x0b;
    const RED_DATA_HIGH_BYTE: u8 = 0x0c;
    const BLUE_DATA_LOW_BYTE: u8 = 0x0d;
    const BLUE_DATA_HIGH_BYTE: u8 = 0x0e;
}

#[allow(non_camel_case_types)]
pub enum OperationModes {
    PowerDown,
    GreenOnly,
    RedOnly,
    BlueOnly,
    StandBy,
    Green_Red_Blue,
    Green_Red,
    Green_Blue,
}

#[allow(non_camel_case_types)]
pub enum RgbDataSensingRange {
    _375_Lux,
    _10_000_Lux,
}
#[allow(non_camel_case_types)]
pub enum RgbStartSyncedAtIntPin {
    ADC_StartAtI2cWrite0x01,
    ADC_StartAtRisingInt,
}
pub enum InterruptThresholdAssignment {
    NoInterrupt,
    GreenInterrupt,
    RedInterrupt,
    BlueInterrupt,
}
pub enum InterruptPersistControl {
    One,
    Two,
    Four,
    Eight,
}

pub enum RgbConversionDone {
    Disable,
    Enable,
}

pub enum AdcResolution {
    _16bits,
    _12bits,
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock;
    use super::*;
    use embedded_hal_mock as hal;
    #[test]
    fn should_return_correct_device_id() {
        let expectations = [hal::i2c::Transaction::write_read(
            DEVICE_ADDRESS,
            vec![0x00],
            vec![0x7d],
        )];
        let i2c_dev = hal::i2c::Mock::new(&expectations);
        let id = Isl29125::new(i2c_dev).verify_device_id();
        assert_eq!(0x7D, id.unwrap());
    }

    #[test]
    fn should_return_wrong_device_id_error() {
        let expectations = [hal::i2c::Transaction::write_read(
            DEVICE_ADDRESS,
            vec![0x00],
            vec![0x75],
        )];
        let i2c_dev = hal::i2c::Mock::new(&expectations);
        let id = Isl29125::new(i2c_dev).verify_device_id();
        match id {
            Err(Error::WrongDeviceId) => (),
            _ => panic!("Should return wrong device ID"),
        }
    }

    #[test]
    fn should_update_led_counters() {
        let expectations = [hal::i2c::Transaction::write_read(
            DEVICE_ADDRESS,
            vec![0x00],
            vec![0x75],
        )];
        let i2c_dev = hal::i2c::Mock::new(&expectations);
        let mut isl29125 = Isl29125::new(i2c_dev);
        let data = [0xff, 0x00, 0x00, 0x00,0x00,0x00];
        isl29125.set_count_values(data);
        assert_eq!(isl29125.led_counts.green, Some(0x00ff));
        assert_eq!(isl29125.led_counts.blue, Some(0x00));
        assert_eq!(isl29125.led_counts.red, Some(0x00));


        
        
        
    }
}
