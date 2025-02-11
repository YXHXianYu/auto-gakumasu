pub use crate::prelude::*;
use crate::task_println;

pub fn daily_task() {
    start_emulator();
    start_game();
    // end_emulator();
}

pub fn start_game() {
    task_println!("Starting kuyo & game...");

    // start kuyo
    click(504, 249);
    sleep(get_config().default_wait_time);
    task_println!("Kuyo started.");

    // click the button 1
    click(540, 1900);
    sleep(get_config().default_wait_time);
    task_println!("Button 1 clicked.");

    // click the button 2
    click(886, 687);
    sleep(get_config().default_wait_time);
    task_println!("Button 2 clicked.");

    // wait for gakumasu start & click any position
    task_println!("Waiting for game start...");
    sleep(30.0);

    task_println!("Game started. Prepare to click any position.");
    click(540, 1300);

    task_println!("Game started.");
}

pub fn start_emulator() {
    task_println!("Starting emulator...");

    // 启动模拟器
    run_command_async(vec![
        format!("{}/{}", get_config().emulator_path, get_config().emulator_name).as_str(),
    ]);

    let wait_time = 30.0;

    task_println!("Emulator started. Waiting for {} seconds...", wait_time);
    
    sleep(wait_time);

    task_println!("{} seconds passed. Emulator is ready.", wait_time);
}

pub fn end_emulator() {
    task_println!("Ending emulator...");

    run_command(vec![
        "taskkill",
        "/IM",
        &get_config().emulator_name,
        "/F",
    ]);

    task_println!("Emulator ended.");
}