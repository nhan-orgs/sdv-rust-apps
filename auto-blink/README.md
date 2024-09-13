# auto-blink

## Introduction
* This application will switch the status of light (on/off) after every 1 second
* Method: `set_target_value`

## How to run

### Basic version
* You can view source code in `src/bin/basic.rs`
    * Ignore the error of `set_target_value` because this is the simplest version code - only use for understanding the idea of the application and Rust syntax for beginner.
* Run basic version:
    ```
    cd ./auto-blink
    cargo run --bin basic
    ```
* Note: Choose suitable version of VSS signal by uncommenting coresponding `LIGHT_SIGNAL` (line 8 or 9)

### Advanced version
* You can view source code in `src/bin/advanced.rs`
    * If the `set_target_value` return an error, notify and exit the program
* Run advanced version:
    ```
    cd ./auto-blink
    cargo run --bin advanced
    ```
* Note: Choose suitable version of VSS signal by uncommenting coresponding `LIGHT_SIGNAL` (line 8 or 9)