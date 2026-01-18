# ISL29125 RGB Light Sensor Driver

A platform-agnostic Rust driver for the ISL29125 RGB light sensor using the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

## Features

- Platform-agnostic using embedded-hal I2C traits
- Read RGB light intensity values (16-bit per channel)
- Configure operating modes (single color or multi-color sensing)
- Device ID verification
- `#![no_std]` compatible
- Fully safe Rust (`#![deny(unsafe_code)]`)

## Hardware

The ISL29125 is a low power, high sensitivity, red, green and blue color light sensor with an I2C interface. It features:
- I2C digital interface (7-bit address: `0x44`)
- 16-bit ADC resolution
- Programmable interrupt with thresholds
- Multiple sensing modes (RGB, individual colors)
- Wide dynamic range

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
isl229125 = "0.1"
embedded-hal = "0.2"
```

## Example

```rust
use isl229125::{Isl29125, OperationModes};
use linux_embedded_hal::I2cdev;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize I2C device
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let mut sensor = Isl29125::new(i2c);

    // Verify device ID
    sensor.verify_device_id()?;

    // Set operating mode to read all RGB channels
    sensor.set_operating_mode(OperationModes::Green_Red_Blue)?;

    // Read LED counters
    sensor.read_led_counters()?;

    // Access the values
    println!("Red: {:?}", sensor.led_counts.red);
    println!("Green: {:?}", sensor.led_counts.green);
    println!("Blue: {:?}", sensor.led_counts.blue);

    Ok(())
}
```

## Operating Modes

The sensor supports multiple operating modes:

- `PowerDown` - Power down the device
- `GreenOnly` - Measure only green channel
- `RedOnly` - Measure only red channel
- `BlueOnly` - Measure only blue channel
- `StandBy` - Standby mode
- `Green_Red_Blue` - Measure all three channels
- `Green_Red` - Measure green and red channels
- `Green_Blue` - Measure green and blue channels

## API

### Core Methods

- `new(i2c)` - Create a new sensor instance
- `verify_device_id()` - Verify the sensor is connected and responding with correct ID
- `set_operating_mode(mode)` - Configure the sensor operating mode
- `read_led_counters()` - Read RGB values from sensor (updates `led_counts` field)
- `read_all_registers()` - Read all 15 registers for debugging

### Data Access

After calling `read_led_counters()`, RGB values are available in:
- `sensor.led_counts.red` - Red channel value (0-65535)
- `sensor.led_counts.green` - Green channel value (0-65535)
- `sensor.led_counts.blue` - Blue channel value (0-65535)

## Development

### Building

```bash
cargo build
```

### Running Tests

Tests use `embedded-hal-mock` to simulate I2C transactions:

```bash
cargo test
```

### Running the Raspberry Pi Example

#### Option 1: Download Pre-built Binary

Pre-built binaries for Raspberry Pi are available from GitHub Actions:

1. Go to the [Actions tab](../../actions/workflows/build-raspi.yml)
2. Click on the latest successful build
3. Download the appropriate artifact:
   - `raspi-armv7-unknown-linux-gnueabihf` for 32-bit Raspberry Pi OS
   - `raspi-aarch64-unknown-linux-gnu` for 64-bit Raspberry Pi OS
4. Extract and run on your Raspberry Pi:

```bash
# Make executable
chmod +x raspi-armv7  # or raspi-aarch64

# Run (requires sudo for I2C access)
sudo ./raspi-armv7
```

#### Option 2: Build on Raspberry Pi

On a Raspberry Pi with the sensor connected to I2C bus 1:

```bash
cargo build --example raspi --release
sudo ./target/release/examples/raspi
```

#### Option 3: Cross-compile Locally

Install cross-compilation tools and build:

```bash
# Install target
rustup target add armv7-unknown-linux-gnueabihf

# Install cross-compiler (on Ubuntu/Debian)
sudo apt-get install gcc-arm-linux-gnueabihf

# Build
cargo build --example raspi --release --target armv7-unknown-linux-gnueabihf

# Transfer to Raspberry Pi
scp target/armv7-unknown-linux-gnueabihf/release/examples/raspi pi@raspberrypi:~/
```

### InfluxDB Integration Example

The `raspi_influxdb` example reads RGB values from the sensor and sends them to InfluxDB v3 for time-series data storage and analysis.

#### Configuration

The example uses environment variables for configuration:

- `INFLUXDB_HOST` - InfluxDB server URL (default: `https://us-east-1-1.aws.cloud2.influxdata.com`)
- `INFLUXDB_TOKEN` - InfluxDB authentication token (required)
- `INFLUXDB_DATABASE` - Database name (default: `rgb-sensor`)
- `SENSOR_LOCATION` - Location tag for the sensor (default: `default`)
- `READ_INTERVAL_SECS` - Seconds between readings (default: `5`)

#### Running the InfluxDB Example

Download the pre-built binary from GitHub Actions or build it:

```bash
# Build on Raspberry Pi
cargo build --example raspi_influxdb --release

# Or cross-compile
cargo build --example raspi_influxdb --release --target armv7-unknown-linux-gnueabihf
```

Run with environment variables:

```bash
# Set your InfluxDB credentials
export INFLUXDB_TOKEN="your-token-here"
export INFLUXDB_DATABASE="your-database"
export SENSOR_LOCATION="greenhouse"
export READ_INTERVAL_SECS="10"

# Run the example
sudo -E ./raspi_influxdb-armv7
```

The example will continuously read RGB values and send them to InfluxDB with the following schema:

```
Measurement: rgb_sensor
Tags: location
Fields: red, green, blue (integers 0-65535)
```

You can query the data using InfluxQL or SQL:

```sql
SELECT * FROM rgb_sensor WHERE location = 'greenhouse' ORDER BY time DESC LIMIT 10
```

## License

Author: Mikko Viitamäki

## Resources

- [ISL29125 Datasheet](https://www.renesas.com/en/document/dst/isl29125-datasheet?srsltid=AfmBOop_ENZFrfgORJqpvx7ldyloP3voGkxdFD4eWwj-jj1lfm96mNio)
- [embedded-hal Documentation](https://docs.rs/embedded-hal/)
