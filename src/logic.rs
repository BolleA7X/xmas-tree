const LEDS_N: usize = 7;
const STEPS_N_MAX: usize = 7;
const ANIMATIONS_N: usize = 3;

/* === LOW LEVEL INTERFACE === */

#[derive(Copy, Clone)]
pub struct Interface {
    pub leds_state: [(usize, bool); LEDS_N],
    pub wait_time_ms: u16
}

pub const EMPTY_INTERFACE: Interface = Interface {
    leds_state: [(2, false), (3, false), (4, false), (5, false), (6, false), (7, false), (8, false)],
    wait_time_ms: 0
};

/* === ANIMATIONS === */

struct Animation {
    steps: [Interface; STEPS_N_MAX],
    steps_n: u8,
    repetitions: u8,
    step_index: usize
}

struct Sequence {
    animations: [Animation; ANIMATIONS_N],
    repet_count: u8,
    anim_index: usize
}

const LEFT_RIGHT: Animation = Animation {
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

const BOT_TOP: Animation = Animation {
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

const ALL: Animation = Animation {
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

/* === ANIMATION LOGIC === */

static mut SEQUENCE: Sequence = Sequence {
    animations: [LEFT_RIGHT, BOT_TOP, ALL],
    repet_count: 0,
    anim_index: 0
};

pub fn exec_next_step(iface: &mut Interface) {
    // Access current animation info
    let mut curr_animation: &mut Animation;
    unsafe { curr_animation = &mut SEQUENCE.animations[SEQUENCE.anim_index]; }

    // Return the current step to the low level logic
    *iface = curr_animation.steps[curr_animation.step_index];

    // Go to the next step of the animation. If it was the last step, increment the repetition count
    curr_animation.step_index = (curr_animation.step_index + 1) % (curr_animation.steps_n as usize);
    if curr_animation.step_index == 0 {
        unsafe { SEQUENCE.repet_count += 1; }
    }
    // If it also was the last repetition, go to the next animation
    unsafe {
        if SEQUENCE.repet_count == curr_animation.repetitions {
            SEQUENCE.repet_count = 0;
            SEQUENCE.anim_index = (SEQUENCE.anim_index + 1) % ANIMATIONS_N;
        }
    }
}