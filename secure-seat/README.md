# secure-seat

## 1. Function: 
The car will be turn on the alert if the driver without seat belts on while the car is moving.
    
## 2. Solution
* Subscribe 2 signals (`Speed` and `IsBelted`)
* When value of `isbelted` or `speed` update, re-check if vehicle need to turn on/off alert

### 2.1. Steps detail
* Init:
    * `is_belted`: value from sensor `Vehicle.Cabin.Seat.Row1.Pos1.IsBelted`
    * `speed`: value from sensor `Vehicle.Speed`
    * `is_alerting` = false
    * Safe condition: `is_belted = true || speed = 0`
    * Subscribe `Speed`, `IsBelted`
* On change events:
    * Update value of `speed` and `is_belted`
    * if (Safe condition) && `is_alerting` --> turn OFF alert
    * if !(Safe condition) && `!is_alerting` --> turn ON alert
* Turn on alert:
    * `Vehicle.Body.Lights.IsHazardOn` --> True
    * `Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed` --> 100
    * `Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed` --> 100
    * `is_alerting` = true
* Turn off alert:
    * `Vehicle.Body.Lights.IsHazardOn` --> False
    * `Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed` --> 0
    * `Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed` --> 0
    * `is_alerting` = false

## 3. Implementation
> Task is a green thread, that helps program runs concurrency in Rust; We can create a task by `tokio::spawn()`

* We need to create 2 tasks to manage the subscriptions of `Speed` and `IsBelted` signal
* We need some common variables that are used in both 2 tasks:
    * `vehicle`: `KuksaClient`, which performs get/set method on vehicle signal
    * `speed`: current value of `Speed` signal
    * `is_belted`: current value of `IsBelted` signal
    * `is_alerting`: true if the car is performing an alert
* We need to call `asyc` functions on `vehicle`, so the best solution in this case is using an other task to manage shared data. [Read more here](https://tokio.rs/tokio/tutorial/shared-state)

### 3.1. Tasks description

#### 3.1.1. Shared data task
* Contain shared data
* Receive message from other tasks, perform action on shared data corresponding to that message, send result (if needed)

#### 3.1.2. IsBealted subscription task
* Subscribe `IsBelted` signal
* On `IsBelted` change event:
    * Update state of `is_belted` by send message to shared data task
    * Check safe condition and turn on/off alert

#### 3.1.3. Speed subscription task
* Subscribe `Speed` signal
* On `Speed` change event:
    * Update state of `speed` by send message to shared data task
    * Check safe condition and turn on/off alert

### 3.2. Tasks communication
* Tasks will communicate through message, which is sent through a channel
* There are many types of channel which are used in diffrent situations. In this implementation, we will use 2 types:
    * `mpsc`: multi-producer single-consumer, both speed task and isbelted task can send messages, only shared data task can receive these messages; number of message is not limited
    * `oneshot`: single-producer single-consumer, this can be used to send result from shared data task to task which sent the message, only 1 value is allowed

#### 3.2.1. Define message
* `enum Command` specifies message type
* Shared data task will perform actions according to the `Command` it recieves

#### 3.2.2. Create mpsc channel
* In the main thread, create an `mpsc` channel, that will return 2 values: `tx` (Sender) and `rx` (Reciever)
* `tx` can be cloned and use in many tasks (speed task and isbelted task) to send message to shared data task
* `rx` can not be cloned, we will use it in shared data task to receive message from 2 other task

#### 3.2.3. Example of communication
For example, we want to turn on the hazard light (call the `set_target_value` on `vehicle`), follow these steps:

Add message definition into `enum Command`:
    ```
    enum Command {
        // other message types
        SetTargetValue {
            signal: String,
            value: String,
            resp: Responder<Result<(), ClientError>>,
        },
    }
    ```
    `resp` is an Sender of `oneshot` channel, shared data task will send result by using it

**Sender Side (eg: speed task)**
1. We need `tx` - a Sender of `mpsc` channel 
2. Create a `oneshot` channel to receive result (success / fail)
    ```
    let (resp_tx, resp_rx) = oneshot::channel();
    ```
3. Create an message:
    ```
    let cmd = Command::SetTargetValue {
        signal: IS_HAZARD_ON.to_string(), // hazard signal
        value: "true".to_string(),
        resp: resp_tx, // oneshot Sender
    };
    ```
4. Send message:
    ```
    let _ = tx.send(cmd).await;
    ```
5. Receive result:
    ```
    resp_rx.await // this is oneshot Receiver
    ```

**Receiver Side (shared data task)**
1. Create a `KuksaClient` instance:
    ```
    let mut vehicle = KuksaClient::new(SERVER_ASSREDD);

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    };
    ```
2. Receive messages
    ```
    while let Some(message) = rx.recv().await {
        // process message here
    }
    ```
3. Perform action for each message:
    ```
    while let Some(message) = rx.recv().await {
        match message {
            // ...
            Command::SetCurrentValue {
                signal,
                value,
                resp,
            } => {
                let response = vehicle.set_current_value(&signal, &value).await;
                let _ = resp.send(response);
            }
            // ...
        }
    }
    ```

## 4. Demo
* Run source code: `cargo run`
* Open other terminal, install kuksa-client 0.3.0: `pip install kuksa-client==0.3.0`
* Open kuksa-client: `kuksa-client --ip 127.0.0.1 --port 55555 --protocol grpc --insecure`
* Change seatbelt status: `setValue Vehicle.Cabin.Seat.Row1.Pos1.IsBelted <value>`
    * `<value>`: true/false
* Change speed: `setValue Vehicle.Speed <value>`
    * `<value>`: uint (0,1,2,...)
