use mouse_rs::{types::keys::Keys, Mouse};
use std::{thread, time};

fn main() {
    let mouse = Mouse::new();
    loop {
        let mut position = mouse.get_position().expect("Unable to retrieve mouse position");
        mouse.move_to(position.x+1, position.y+1).expect("Unable to move mouse");
        thread::sleep(time::Duration::from_millis(500));
        position = mouse.get_position().expect("Unable to retrieve mouse position");
        mouse.move_to(position.x-1, position.y-1).expect("Unable to move mouse");
        thread::sleep(time::Duration::from_millis(10000));
        mouse.press(&Keys::LEFT).expect("Unable to press button");
        mouse.release(&Keys::LEFT).expect("Unable to release button");
        thread::sleep(time::Duration::from_millis(500));
    }
}