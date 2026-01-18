# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust embedded-hal driver library for the ISL29125 RGB light sensor IC. The library provides a platform-agnostic interface for communicating with the ISL29125 via I2C, following the embedded-hal traits pattern.

## Build and Test Commands

```bash
# Build the library
cargo build

# Run tests (uses embedded-hal-mock for I2C simulation)
cargo test

# Build with verbose output
cargo build --verbose

# Run tests with verbose output
cargo test --verbose

# Build the Raspberry Pi examples
cargo build --example raspi
cargo build --example raspi_influxdb

# Cross-compile for Raspberry Pi 3 (32-bit)
cargo build --example raspi --release --target armv7-unknown-linux-gnueabihf
cargo build --example raspi_influxdb --release --target armv7-unknown-linux-gnueabihf

# Cross-compile for Raspberry Pi 3 (64-bit)
cargo build --example raspi --release --target aarch64-unknown-linux-gnu
cargo build --example raspi_influxdb --release --target aarch64-unknown-linux-gnu
```

## CI/CD

The repository has two GitHub Actions workflows:

1. **rust.yml** - Runs on every push/PR to master:
   - Builds the library
   - Runs unit tests

2. **build-raspi.yml** - Builds Raspberry Pi binaries:
   - Cross-compiles both `raspi` and `raspi_influxdb` examples for armv7 (32-bit) and aarch64 (64-bit)
   - Uploads build artifacts for easy download
   - Attaches binaries to GitHub releases

## Architecture

### Core Driver (`src/lib.rs`)

The `Isl29125<I2C>` struct is the main driver that wraps an I2C peripheral:
- Generic over any I2C implementation that satisfies `embedded_hal::blocking::i2c::{Write, WriteRead}`
- Uses const device address `0b100_0100` and device ID `0x7D`
- Maintains `led_counts: LedCounter` state containing last read RGB values

### Register Communication Pattern

All register access goes through private helper methods:
- `read_register()` - single register read
- `read_all_registers()` - bulk read of all 15 registers starting from DEVICE_ID
- Direct I2C operations for writes (e.g., in `set_operating_mode()`)

Register addresses are defined as constants in the private `RegisterMap` struct (lines 102-120).

### LED Counter Reading

`read_led_counters()` performs a bulk 6-byte read starting from `GREEN_DATA_LOW_BYTE` register. The read order is Green-Red-Blue (2 bytes each), and `set_count_values()` reconstructs the u16 values from low/high byte pairs.

### Operating Modes

The sensor supports multiple operating modes via `OperationModes` enum:
- PowerDown, StandBy
- Single color: GreenOnly, RedOnly, BlueOnly
- Multi-color: Green_Red_Blue, Green_Red, Green_Blue

Mode is set via `set_operating_mode()` which performs a read-modify-write on CONFIGURATION1 register.

### Testing Strategy

Tests use `embedded-hal-mock` to create I2C transaction expectations without hardware:
- `should_return_correct_device_id()` - verifies device ID read
- `should_return_wrong_device_id_error()` - tests error handling
- `should_update_led_counters()` - validates counter value parsing

### Examples

#### `examples/raspi.rs` - Basic Usage

Demonstrates basic usage on Raspberry Pi:
1. Create I2C device at `/dev/i2c-1`
2. Initialize `Isl29125` driver
3. Set operating mode (e.g., GreenOnly)
4. Verify device ID
5. Read LED counters in a loop and print to console

#### `examples/raspi_influxdb.rs` - InfluxDB Integration

Demonstrates integration with InfluxDB v3 for time-series data logging:
1. Reads configuration from environment variables (INFLUXDB_HOST, INFLUXDB_TOKEN, etc.)
2. Initializes sensor and sets RGB mode
3. Continuously reads RGB values at configurable intervals
4. Sends data to InfluxDB v3 using HTTP API with line protocol format
5. Uses async/await with tokio runtime
6. Tags data with configurable location

Dependencies: `reqwest`, `tokio` (dev-dependencies only)
Note: Uses InfluxDB v2 API endpoint which is compatible with InfluxDB v3

## Development Notes

- The crate uses `#![deny(unsafe_code)]` - all unsafe code is forbidden
- Uses Rust 2018 edition
- Depends on `embedded-hal 0.2.3` for traits
- Dev dependencies include `linux-embedded-hal 0.3` for hardware examples and `embedded-hal-mock 0.4` for testing
- The .cargo/config file is empty and can be removed or configured for cross-compilation targets
