use crate::logic::defs::{Animation, Sequence, LEDS_N, ANIMATIONS_N};
use crate::logic::animations::{LEFT_RIGHT, BOT_TOP, ALL, CIRCLE};

#[derive(Copy, Clone)]
pub struct Interface {
    pub leds_state: [(usize, bool); LEDS_N],
    pub wait_time_ms: u16
}

pub const EMPTY_INTERFACE: Interface = Interface {
    leds_state: [(2, false), (3, false), (4, false), (5, false), (6, false), (7, false), (8, false)],
    wait_time_ms: 0
};

static mut SEQUENCE: Sequence = Sequence {
    animations: [LEFT_RIGHT, BOT_TOP, ALL, CIRCLE],
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