# FCS - Flight Control System

This is a software designed for my own rocket's flight computer which is based on the ESP32-S3.

## Requirements:

- rust
- cargo
- espup - install with `cargo install espup && espup install`
- espflash - install with `cargo install espflash --version 3.3.0`

You can just run the [setup.sh](./setup.sh) file.

## Build & Flash:

You can just run [build_and_flash.sh](./build_and_flash.sh).

## Components:

- ESP32-S3
- BMP280
- MPU6050
- PCA9685
- nRF24L01
- Arduino (ESP32-S3) compatible SD-Card reader
- a GPS module (e.g. GY-GPS6MV2)
- 2x simple N-MOSFET (though please check if it is usable with a chip such as this)
- DHT11
