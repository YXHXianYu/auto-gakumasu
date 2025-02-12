use std::process::Command;
use crate::config::get_config;
use crate::sleep::sleep;
use crate::adb_println;

pub fn click_scaled(x: u32, y: u32, wait_time: f64) {
    let config = get_config();

    let x = (x as f64 * config.scale_x) as u32;
    let y = (y as f64 * config.scale_y) as u32;

    click(x, y, wait_time);
}

pub fn click(x: u32, y: u32, wait_time: f64) {
    adb_println!("Clicking screen at ({}, {})...", x, y);

    Command::new("adb")
        .args(&["shell", "input", "tap", &format!("{} {}", x, y)])
        .spawn()
        .expect("Failed to click screen.");

    sleep(wait_time);
}

pub fn swipe_scaled(x1: u32, y1: u32, x2: u32, y2: u32, wait_time: f64) {
    let config = get_config();

    let x1 = (x1 as f64 * config.scale_x) as u32;
    let y1 = (y1 as f64 * config.scale_y) as u32;
    let x2 = (x2 as f64 * config.scale_x) as u32;
    let y2 = (y2 as f64 * config.scale_y) as u32;

    swipe(x1, y1, x2, y2, wait_time);
}

pub fn swipe(x1: u32, y1: u32, x2: u32, y2: u32, wait_time: f64) {
    adb_println!("Swiping from ({}, {}) to ({}, {})...", x1, y1, x2, y2);

    Command::new("adb")
        .args(&["shell", "input", "swipe", &format!("{} {} {} {}", x1, y1, x2, y2)])
        .spawn()
        .expect("Failed to swipe.");
    
    sleep(wait_time);
}
