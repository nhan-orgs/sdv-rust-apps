# SMART WIPERS
> Python source: [Demo smart wipers](https://digitalauto.netlify.app/model/STLWzk1WyqVVLbfymb4f/library/prototype/wn6bU7ncCT5WSTfXu66m/view/code)

Function: Force the wipers off while the hood is open

## 1. Basic version
* Path: `smart-wipers/src/bin/basic-ver.rs`
* Run cmd: `cargo run --bin basic-ver`
* Idea:
    * Methods: `get`, `set`
    * Use an infinite loop with a delay duration, turn off the wipers if the hood and the wipers are open at the same time

## 2. Hood subcribe version
* Path: `smart-wipers/src/bin/hood-sub-ver.rs`
* Run cmd: `cargo run --bin hood-sub-ver`
* Idea:
    * Methods: `get`, `set`, `subscribe`
    * Subscribe the hood, whenever the hood is updated, turn off the wipers if the hood and the wipers are currently open
* Do not manage case open wipers while the hood is open

## 3. Hood and wipers subcribe version
* Path: `smart-wipers/src/bin/subscribe-ver.rs`
* Run cmd: `cargo run --bin subscribe-ver`
* Idea:
    * Methods: `get`, `set`, `subscribe`
    * Subscribe both the hood and the wipers; Whenever the hood or the wipers are updated, turn off the wipers if the hood and the wipers are currently open
* Note: 
    * To ensure the subscription of the hood and wipers are work independently, create 2 threads to manage each of them
    * Use `Arc` and `Mutex` (`tokio`) to share `KuksaClient` instance among threads