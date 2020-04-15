extern crate linux_embedded_hal as hal;
extern crate isl229125;


fn main(){
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut led_counter = isl229125::Isl29125::new(dev);
    led_counter.set_operating_mode(isl229125::OperationModes::Green_Red_Blue).unwrap();
    let id = led_counter.verify_device_id();
    match id{
        Ok(id) =>{ 
            println!("Find Correct device: {:?}", id);
            loop{

                match led_counter.read_led_counters(){
                    Ok(_) => {
                        println!("Red Counts: {:?}", led_counter.led_counts.red);
                        println!("Green Counts: {:?}", led_counter.led_counts.green);
                        println!("Blue Counts: {:?}", led_counter.led_counts.blue);
                    }
                    Err(_) => println!("Cant read counters"),
                }
            }


        } 
        Err(e) => print!("Something went wrong: {:?}", e)
    }
}