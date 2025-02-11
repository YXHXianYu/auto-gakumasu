use std::process::Command;
use crate::config::get_config;
use crate::sleep::sleep;
use crate::adb_println;

pub fn click(x: u32, y: u32) {
    adb_println!("Clicking screen at ({}, {})...", x, y);

    Command::new("adb")
        .args(&["shell", "input", "tap", &format!("{} {}", x, y)])
        .spawn()
        .expect("Failed to click screen.");

    sleep(get_config().default_wait_time);

    adb_println!("Clicked.");
}

pub fn swipe(x1: u32, y1: u32, x2: u32, y2: u32) {
    adb_println!("Swiping from ({}, {}) to ({}, {})...", x1, y1, x2, y2);

    Command::new("adb")
        .args(&["shell", "input", "swipe", &format!("{} {} {} {}", x1, y1, x2, y2)])
        .spawn()
        .expect("Failed to swipe.");

    adb_println!("Swiped.");
}
