extern crate linux_embedded_hal as hal;
extern crate isl229125;

use std::{env, thread, time};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let influxdb_host = env::var("INFLUXDB_HOST")
        .unwrap_or_else(|_| "https://us-east-1-1.aws.cloud2.influxdata.com".to_string());
    let influxdb_token = env::var("INFLUXDB_TOKEN")
        .expect("INFLUXDB_TOKEN environment variable must be set");
    let influxdb_database = env::var("INFLUXDB_DATABASE")
        .unwrap_or_else(|_| "rgb-sensor".to_string());
    let sensor_location = env::var("SENSOR_LOCATION")
        .unwrap_or_else(|_| "default".to_string());
    let read_interval_secs: u64 = env::var("READ_INTERVAL_SECS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("READ_INTERVAL_SECS must be a valid number");

    println!("Initializing ISL29125 RGB sensor...");

    // Initialize I2C device
    let dev = hal::I2cdev::new("/dev/i2c-1")?;
    let mut sensor = isl229125::Isl29125::new(dev);

    // Verify device ID
    let id = sensor.verify_device_id()?;
    println!("Found ISL29125 with device ID: 0x{:02X}", id);

    // Set operating mode to read all RGB channels
    sensor.set_operating_mode(isl229125::OperationModes::Green_Red_Blue)?;
    println!("Set operating mode to RGB");

    // Initialize HTTP client for InfluxDB
    let client = Client::new();
    let write_url = format!("{}/api/v2/write?bucket={}&precision=s",
                           influxdb_host.trim_end_matches('/'),
                           influxdb_database);

    println!("Connecting to InfluxDB at {}...", influxdb_host);
    println!("Starting data collection (interval: {}s, location: {})",
             read_interval_secs, sensor_location);
    println!("Press Ctrl+C to stop");

    loop {
        // Read LED counters from sensor
        match sensor.read_led_counters() {
            Ok(_) => {
                let red = sensor.led_counts.red.unwrap_or(0);
                let green = sensor.led_counts.green.unwrap_or(0);
                let blue = sensor.led_counts.blue.unwrap_or(0);

                println!("R: {}, G: {}, B: {}", red, green, blue);

                // Build line protocol data
                let line_protocol = format!(
                    "rgb_sensor,location={} red={}i,green={}i,blue={}i",
                    sensor_location, red, green, blue
                );

                // Write to InfluxDB using HTTP API
                match client
                    .post(&write_url)
                    .header("Authorization", format!("Token {}", influxdb_token))
                    .header("Content-Type", "text/plain; charset=utf-8")
                    .body(line_protocol)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            println!("✓ Data written to InfluxDB");
                        } else {
                            eprintln!("✗ InfluxDB returned error: {} - {}",
                                     response.status(),
                                     response.text().await.unwrap_or_default());
                        }
                    }
                    Err(e) => eprintln!("✗ Failed to write to InfluxDB: {}", e),
                }
            }
            Err(e) => eprintln!("✗ Failed to read sensor: {:?}", e),
        }

        thread::sleep(time::Duration::from_secs(read_interval_secs));
    }
}
