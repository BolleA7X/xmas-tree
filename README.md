# XMAS TREE

This project implements a Christmas tree using an Arduino Uno and LEDs.

## PREREQUISITES

### Hardware

* x1 Arduino Uno
* x1 breadboard
* xN red LEDs
* xN green LEDs
* xM 220 Ohm resistors
* xQ jumper wires male-to-male

### Software

* rustc / cargo
* gcc-avr
* avrdude

## PREREQUISITES INSTALLATION

### rustc and cargo

To download and run *rustup*, execute the following command:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

*rustup* allows to install and configure the Rust compiler (*rustc*) and package manager (*cargo*).

### AVR toolchain and flash programmer

The following commands allow to install the necessary AVR tools:

    sudo apt update
    sudo apt install gcc-avr
    sudo apt install avrdude

AVR is the architecture that the Arduino Uno microcontroller is based on (*ATmega328P* microcontroller).

## PROJECT SETUP

### Dependencies

In the project *Cargo.toml* file:

1. Add the dependency for the panic handler under the ```[dependencies]``` section:
    ```
    [dependencies]
    panic-halt = "0.2.0"
    ```
2. Add the dependency for the AVR HAL at the end of the file:  
    ```
    [dependencies.harduino-hal]
    git = "https://github.com/Rahix/avr-hal"
    rev = "4cbb163"
    ```
3. Remove the ```edition``` field from the top of the file

At the end run the following command to setup the project

    cargo update

### Target

1. Create a folder called ```.cargo``` in the project root directory and ```cd``` into it
2. Inside the new folder create a file called ```config.toml```
3. Write the following lines in the ```config.toml``` file:
    ```
    [build]
    target = "avr-atmega328p.json"

    [unstable]
    build-std = ["core"]
    ```
4. Download the ```avr-atmega328p.json``` file from https://github.com/Rahix/avr-hal/tree/main/avr-specs inside the project root directory

### Rust buildchain version
 
Run the command:

    rustup override set nightly-2021-01-07

This will set the rust buildchain to a version which is compatible with the AVR toolchain.