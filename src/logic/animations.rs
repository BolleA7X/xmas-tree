use crate::logic::defs::Animation;
use crate::logic::interface::{Interface, EMPTY_INTERFACE};

pub const LEFT_RIGHT: Animation = Animation {
    step_index: 0,
    steps_n: 2,
    repetitions: 10,
    steps: [
        Interface {
            leds_state: [(2, true), (3, false), (4, true), (5, false), (6, true), (7, false), (8, true)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, false), (3, true), (4, false), (5, true), (6, false), (7, true), (8, true)],
            wait_time_ms: 300
        },
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
    ]
};

pub const BOT_TOP: Animation = Animation {
    step_index: 0,
    steps_n: 7,
    repetitions: 4,
    steps: [
        Interface {
            leds_state: [(2, true), (3, true), (4, false), (5, false), (6, false), (7, false), (8, false)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, true), (5, true), (6, false), (7, false), (8, false)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, true), (7, true), (8, false)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, false), (7, false), (8, true)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, true), (7, true), (8, false)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, true), (5, true), (6, false), (7, false), (8, false)],
            wait_time_ms: 300
        },
        Interface {
            leds_state: [(2, true), (3, true), (4, false), (5, false), (6, false), (7, false), (8, false)],
            wait_time_ms: 300
        },
    ]
};

pub const ALL: Animation = Animation {
    step_index: 0,
    steps_n: 2,
    repetitions: 4,
    steps: [
        Interface {
            leds_state: [(2, true), (3, true), (4, true), (5, true), (6, true), (7, true), (8, true)],
            wait_time_ms: 1000
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, false), (7, false), (8, false)],
            wait_time_ms: 1000
        },
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
        EMPTY_INTERFACE,
    ]
};

pub const CIRCLE: Animation = Animation {
    step_index: 0,
    steps_n: 7,
    repetitions: 8,
    steps: [
        Interface {
            leds_state: [(2, true), (3, false), (4, false), (5, false), (6, false), (7, false), (8, false)],
            wait_time_ms: 100
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, true), (5, false), (6, false), (7, false), (8, false)],
            wait_time_ms: 100
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, true), (7, false), (8, false)],
            wait_time_ms: 100
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, false), (7, false), (8, true)],
            wait_time_ms: 100
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, false), (6, false), (7, true), (8, false)],
            wait_time_ms: 100
        },
        Interface {
            leds_state: [(2, false), (3, false), (4, false), (5, true), (6, false), (7, false), (8, false)],
            wait_time_ms: 100
        },
        Interface {
            leds_state: [(2, false), (3, true), (4, false), (5, false), (6, false), (7, false), (8, false)],
            wait_time_ms: 100
        },
    ]
};