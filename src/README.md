# XMAS TREE

## OVERVIEW

The design choice was to separate as much as possible the high level logic (animations logic) from the low level logic (Arduino code for LEDs control), in order to implement the high level logic in a modular way. By doing so:
* animations can be added, removed or changed without having side effects on other parts of the code
* the animations logic can be ported with little effort to other platforms and hardware

## BLOCK DIAGRAM

The image below shows a block diagram of the code.

<img src="../img/block-diagram.png" width="500">

The top section (encircled in blue) is the animation logic, while the bottom section (encircled in black) is the Arduino logic.

### HIGH LEVEL LOGIC

An ```Animation``` is composed of an array where each element has two components called ```Step``` and ```ΔT```. A ```Step``` is a list of tuples that represent the state of the desired digital pins (high or low) at any given time, while ```ΔT``` specifies the duration of this state. A ```step index``` is used to keep track of the current step of the animation.

A ```Sequence``` is a collection of Animations. An ```animation index``` is used to keep track of the current animation. A ```repetition counter``` is also used to allow Animations to be repeated multiple times before passing to the next one.

The high level logic exposes a function called ```exec_next_step``` for the low level logic to be used. This function performs the following operations:

1. return the current ```Step``` and ```ΔT``` to the low level logic (their combination is also called ```Interface```)
2. update ```step index```, ```repetition counter``` and ```animation index``` depending on their current value:
    * increment the ```step index```; if the last step of the current animation is reached, the ```step index``` is reset to zero and the ```repetition counter``` is incremented
    * if the ```repetition counter``` reaches its maximum value for the current animation, the counter is reset to zero and the ```animation index``` is incremented
    * if all steps and repetitions of the last animation have been performed, the ```animation index``` is reset to zero to restart the sequence of animations

### LOW LEVEL LOGIC

The Arduino code is an infinite loop that performs the following operations:

1. call ```exec_next_step``` and retrieve the current ```Step``` and ```ΔT```
2. iterate over ```Step```, which is a list of tuples in the form *(pin_id, pin_state)*, and turns on/off the digital pins output according to the specified state (high/low)
3. sleep for the number of milliseconds specified by ```ΔT```