extern crate linux_embedded_hal as hal;
extern crate isl229125;
use std::{thread, time};

fn main(){
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut led_counter = isl229125::Isl29125::new(dev);
    led_counter.set_operating_mode(isl229125::OperationModes::GreenOnly).unwrap();
    let id = led_counter.verify_device_id();
    match led_counter.read_all_registers(){
        Ok(values) => println!("register dumb: {:?}", values),
        Err(_) => println!("Cant read registers")
    }
    match id{
        Ok(id) =>{ 
            println!("Find Correct device: {:?}", id);
            loop{

                match led_counter.read_led_counters(){
                    Ok(_) => {
                        print!("Red Counts: {:?}", led_counter.led_counts.red);
                        //println!("Green Counts: {:?}", led_counter.led_counts.green);
                        //println!("Blue Counts: {:?}", led_counter.led_counts.blue);
                    }
                    Err(_) => println!("Cant read counters"),
                }
                thread::sleep(time::Duration::from_secs(1));
            }


        } 
        Err(e) => print!("Something went wrong: {:?}", e)
    }
}