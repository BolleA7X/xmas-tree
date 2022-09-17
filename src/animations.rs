pub struct Interface {
    pub leds_state: [bool; 14],
    pub wait_time_ms: u16
}

pub fn init_animations() -> Interface {
    let iface = Interface {leds_state: [false; 14], wait_time_ms: 0};
    iface
}