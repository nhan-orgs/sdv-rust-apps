# SMART WIPERS
> Python source: [Demo smart wipers](https://digitalauto.netlify.app/model/STLWzk1WyqVVLbfymb4f/library/prototype/wn6bU7ncCT5WSTfXu66m/view/code)

Function: Force the wipers off while the hood is open

## 1. Basic version
* Path: `smart-wipers/src/bin/basic.rs`
* Run cmd: `cargo run --bin basic`
* Idea: Use an infinite loop with a delay duration, turn off the wipers if the hood and the wipers are open at the same time
* Methods: `get_current_value`, `set_target_value`

## 2. Medium version
* Path: `smart-wipers/src/bin/medium.rs`
* Run cmd: `cargo run --bin medium`
* Idea:
    * Subscribe the hood
    * Whenever the hood is updated, turn off the wipers if the hood and the wipers are currently open
* Methods: `get_current_value`, `set_target_value`, `subscribe_current_value`
* Do not manage case open wipers while the hood is open

## 3. Advanced version
* Path: `smart-wipers/src/bin/advanced.rs`
* Run cmd: `cargo run --bin advanced`
* Idea:
    * Subscribe both the hood and the wipers; 
    * Whenever the hood or the wipers are updated, turn off the wipers if the hood and the wipers are currently open
* Methods: `get_current_value`, `set_target_value`, `subscribe_current_value`
* Note: 
    * To ensure the subscription of the hood and wipers are work independently, create 2 tasks (use `tokio::spawn()`) to manage each of them
    * Use `Arc` and `Mutex` (`tokio`) to share `KuksaClient` instance among tasks