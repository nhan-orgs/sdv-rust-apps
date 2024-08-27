pub struct Constants;

impl Constants {
    pub const SERVER_ADDRESS: &'static str = "http://127.0.0.1:55555";
    pub const MAX_MESSAGES: usize = 10;

    pub const IS_BELTED: &'static str = "Vehicle.Cabin.Seat.Row1.Pos1.IsBelted";
    pub const SPEED: &'static str = "Vehicle.Speed";
    pub const IS_HAZARD_ON: &'static str = "Vehicle.Body.Lights.IsHazardOn";
    pub const LEFT_FAN_SPEED: &'static str = "Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed";
    pub const RIGHT_FAN_SPEED: &'static str = "Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed";

    pub const MAX_FAN_SPEED: &'static str = "100";
    pub const MIN_FAN_SPEED: &'static str = "0";
}
