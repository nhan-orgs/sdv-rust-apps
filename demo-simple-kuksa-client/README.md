# demo-simple-kuksa-client

## Introduction
This project is used for unit test the library `simple-kuksa-client` in Rust

## Prepare
* Clone the kuksa-broker server from [github](https://github.com/eclipse-kuksa/kuksa-databroker) and run:
    ```
    cargo run --bin databroker -- --metadata /home/vy/v-work/kuksa-databroker/data/vss-core/vss_release_4.0.json
    ```
* Clone the `simple-kuksa-client` library from [github](https://github.com/nhan-orgs/sdv-rust-lib)
* In `Cargo.toml`, replace the path of library
    ```
    [dependencies]
    simple-kuksa-client = { path = "../../simple-kuksa-client" }
    ```

## How to run
You can read the `./kuksa-databroker/data/vss-core/vss_release_4.0.json` to know valid paths

### get
* Run: `cargo run --bin demo-get`
* Modify line 14 in `src/demo-get.rs` to test get method on other path

### publish
* Run: `cargo run --bin demo-publish`
* Modify line 15 by replace pair of path and value in str in `src/demo-publish.rs` to test set method on other path

### subscribe
* Run: `cargo run --bin demo-subscribe`
* Modify the path in line 14 in `src/demo-get.rs` to test subscribe method on other path