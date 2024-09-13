# sdv-rust-apps
> These apps run on 3.0 signal, databroker version 0.3.0

## Prepare
* Clone the kuksa-broker server from [github](https://github.com/eclipse-kuksa/kuksa-databroker) and run:
    ```
    cargo run --bin databroker -- --metadata /home/vy/v-work/kuksa-databroker/data/vss-core/vss_release_3.0.json
    ```
* Clone the `simple-kuksa-client` library from [github](https://github.com/nhan-orgs/sdv-rust-lib)
* In `Cargo.toml`, replace the path of library
    ```
    [dependencies]
    simple-kuksa-client = { path = "../../sdv-rust-lib" }
    ```
* You can read the `./kuksa-databroker/data/vss-core/vss_release_3.0.json` to know valid signals