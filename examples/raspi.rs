extern crate linux_embedded_hal as hal;
extern crate isl229125;
use std::{thread, time};

fn main(){
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut led_counter = isl229125::Isl29125::new(dev);
    led_counter.set_operating_mode(isl229125::OperationModes::Green_Red_Blue).unwrap();
    let id = led_counter.verify_device_id();
    match led_counter.read_all_registers(){
        Ok(values) => println!("register dump: {:?}", values),
        Err(_) => println!("Cant read registers")
    }
    match id{
        Ok(id) => {
            println!("Found correct device: 0x{:02X}", id);
            loop{

                match led_counter.read_led_counters(){
                    Ok(_) => {
                        println!("R: {:5}, G: {:5}, B: {:5}",
                                 led_counter.led_counts.red.unwrap_or(0),
                                 led_counter.led_counts.green.unwrap_or(0),
                                 led_counter.led_counts.blue.unwrap_or(0));
                    }
                    Err(_) => println!("Can't read counters"),
                }
                thread::sleep(time::Duration::from_secs(1));
            }


        } 
        Err(e) => println!("Something went wrong: {:?}", e)
    }
}