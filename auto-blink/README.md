# auto-blink

## Introduction
* This application will switch the status of light (on/off) after every 1 second
* Method: `set_target_value`

## How to run
```
cd ./auto-blink
cargo run
```
* Choose the correct version of VSS signal by uncommenting coresponding `LIGHT_SIGNAL` (line 8 or 9)
* Ignore the error of `set_target_value` because this is the simplest version code - only use for understanding the idea of the application and Rust syntax for beginner.