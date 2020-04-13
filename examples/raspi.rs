extern crate linux_embedded_hal as hal;
extern crate isl229125;


fn main(){
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut led_counter = isl229125::Isl29125::new(dev);
    let id = led_counter.verify_device_id();
    match id{
        Ok(id) =>{ 
            print!("Find Correct device: {:?}", id);
            match led_counter.read_led_counters(){
                Ok(_) => println!("Counts: {:?}", led_counter.led_counts.green),
                Err(_) => println!("Cant read counters"),
            }


        } 
        Err(e) => print!("Something went wrong: {:?}", e)
    }
}