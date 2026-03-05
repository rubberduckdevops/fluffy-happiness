# AM2302/DHT22 Temperature and Humidity Sensor Reader

A Rust application for reading temperature and humidity data from an AM2302 (DHT22) digital sensor using GPIO on Linux-based systems (e.g., Raspberry Pi).

## Overview

This project implements a low-level driver for the AM2302/DHT22 temperature and humidity sensor. It communicates with the sensor using GPIO pins, reading digital signals and decoding them into temperature and humidity readings.

## Features

- Direct GPIO communication with AM2302/DHT22 sensors
- Robust error handling with checksum validation
- Support for negative temperature readings
- Configurable GPIO pin selection
- Automatic retry mechanism for failed reads

## Hardware Requirements

- Linux-based system with GPIO support (e.g., Raspberry Pi)
- AM2302/DHT22 temperature and humidity sensor
- Connection to GPIO pins (default: GPIO4)

## Dependencies

- `gpio-cdev` (v0.2) - Character device GPIO interface for Linux

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd sensor
```

2. Build the project:
```bash
cargo build --release
```

## Usage

Run the application:
```bash
cargo run --release
```

By default, the application:
- Uses GPIO pin 4
- Takes 30 readings with 5-second intervals between each reading
- Prints temperature and humidity data to the console

### Configuring GPIO Pin

Edit the `gpio_number` variable in [src/main.rs:33](src/main.rs#L33) to use a different GPIO pin:
```rust
let gpio_number = 4;  // Change to your GPIO pin number
```

## How It Works

### Signal Protocol

The AM2302 sensor uses a single-wire digital communication protocol:

1. **Initialization**: The MCU pulls the data line low for 1-10ms to signal the sensor
2. **Response**: The sensor responds with a series of digital pulses
3. **Data Transfer**: 40 bits of data are transmitted (humidity high, humidity low, temperature high, temperature low, checksum)
4. **Decoding**: Pulse width determines if a bit is 0 (short pulse) or 1 (long pulse)

### Project Structure

- [src/main.rs](src/main.rs) - Main application entry point and reading loop
- [src/am2302.rs](src/am2302.rs) - Data decoding and validation logic for AM2302 sensor readings
- [src/cdev.rs](src/cdev.rs) - GPIO character device interface and signal timing
- [src/binutils.rs](src/binutils.rs) - Binary to numeric conversion utilities

### Data Format

The sensor returns 40 bits of data:
- Bits 0-15: Humidity (16 bits, divided by 10 to get percentage)
- Bits 16-31: Temperature (16 bits, divided by 10 to get degrees Celsius)
- Bits 32-39: Checksum (sum of first 4 bytes)

### Error Handling

The application handles several types of errors:
- **WrongBitsCount**: Incorrect number of bits received
- **MalformedData**: Invalid binary data format
- **ParityBitMismatch**: Checksum validation failed
- **OutOfSpecValue**: Reading outside sensor specifications

### Sensor Specifications

- **Temperature Range**: -40°C to 80°C
- **Humidity Range**: 0% to 100% RH
- **Accuracy**: ±0.5°C, ±2% RH

## Testing

Run the test suite:
```bash
cargo test
```

The test suite includes:
- Binary data validation
- Checksum verification
- Positive and negative temperature handling
- Edge case handling (overflow, malformed data)

## References

- [AM2302 Datasheet (Aosong)](http://akizukidenshi.com/download/ds/aosong/AM2302.pdf)
- [DHT22 Datasheet (Adafruit)](https://cdn-shop.adafruit.com/datasheets/Digital+humidity+and+temperature+sensor+AM2302.pdf)

## License

This project is provided as-is for educational and personal use.

## Troubleshooting

**No readings or failed reads:**
- Verify GPIO pin connection and number
- Check sensor power supply (3.3V or 5V depending on model)
- Ensure proper pull-up resistor (4.7k© - 10k©) on data line
- Run with appropriate permissions to access GPIO (`sudo` may be required)

**Intermittent readings:**
- The sensor requires 2 seconds between readings; the default 5-second interval provides margin
- Check for electrical interference or loose connections
- Verify sensor is not damaged or moisture-exposed
