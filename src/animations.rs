const LEDS_N: usize = 7;

pub struct Interface {
    pub leds_state: [(usize, bool); LEDS_N],
    pub wait_time_ms: u16
}

pub fn init_interface() -> Interface {
    let iface = Interface { wait_time_ms: 0, leds_state: [
        (2, false),
        (3, false),
        (4, false),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
    ] };
    iface
}