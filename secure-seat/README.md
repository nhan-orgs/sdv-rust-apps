# secure-seat

## 1. Function: 
The car will be turn on the alert if the driver without seat belts on while the car is moving.
    
## 2. Solution
* Init:
    * seatbelt: `Vehicle.Cabin.Seat.Row1.Pos1.IsBelted` (sensor)
    * speed: `Vehicle.Speed` (sensor)
    * Alert = false
    * Safe condition: `seatbelt = true || speed = 0`
    * Unsafe condition: `speed > 0 && seatbelt = false`
    * Subscribe `speed`, `seatbelt`
* On change:
    * Update value of `speed` and `seatbelt`
    * if (Safety condition) && Alert --> turn OFF alert
    * if (Unsafe condition) && !Alert --> turn ON alert
* Turn on alert:
    * Alert = true
    * Save current status of hazard, fan right, fan left
    * `Vehicle.Body.Lights.IsHazardOn` --> True
    * `Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed` --> 100
    * `Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed` --> 100
* Turn off alert:
    * Alert = false
    * Set status of hazard, fan right, fan left as saved status

## 3. Demo
* Open kuksa-client: `kuksa-client --ip 127.0.0.1 --port 55555 --protocol grpc --insecure`
* Change seatbelt status: `setValue Vehicle.Cabin.Seat.Row1.Pos1.IsBelted <value>`
    * `<value>`: true/false
* Change speed: `setValue Vehicle.Speed <value>`
    * `<value>`: uint (0,1,2,...)

## 4. How to run source code

