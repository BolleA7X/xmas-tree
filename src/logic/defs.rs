use crate::logic::interface::Interface;

pub const LEDS_N: usize = 7;
pub const STEPS_N_MAX: usize = 7;
pub const ANIMATIONS_N: usize = 4;

pub struct Animation {
    pub steps: [Interface; STEPS_N_MAX],
    pub steps_n: u8,
    pub repetitions: u8,
    pub step_index: usize
}

pub struct Sequence {
    pub animations: [Animation; ANIMATIONS_N],
    pub repet_count: u8,
    pub anim_index: usize
}