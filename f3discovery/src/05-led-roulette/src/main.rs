#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let step: u16 = 50;

    let mut inds = (0..8).cycle();
    let mut prev_idx = inds.next().unwrap();
    leds[prev_idx].on().ok();
    delay.delay_ms(2 * step);

    loop {
        let idx = inds.next().unwrap();      
        leds[idx].on().ok();
        delay.delay_ms(step);
        leds[prev_idx].off().ok();
        prev_idx = idx;
        delay.delay_ms(step);
    }
}
