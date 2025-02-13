use std::io::{self, Read};
use crate::prelude::*;
use chrono::Local;
use crate::{task_println, welcome_println};

pub fn work() {
    welcome_println!("Welcome to auto-gakumasu!");

    do_daily_task();

    println!("Press any key to continue...");
    let _ = io::stdin().read_exact(&mut [0u8]).unwrap();
}

pub fn do_daily_task() {
    if check_if_executed_today() {
        task_println!("Today's task has been executed. Skip.");
        return;
    }
    task_println!("Today's task has not been executed. Start.");

    start_emulator();
    start_game();

    collect_coin();
    do_job();
    strength_support_card();
    do_capsule_toys();
    buy_something_in_coin_shop();
    get_task_rewards();
    get_pass_rewards();

    try_to_participate_in_competition();

    collect_club_rewards();

    update_record_of_execution();

    task_println!("Daily task finished.");

    // end_emulator();
}

fn check_if_executed_today() -> bool {
    let config = get_config();
    let today = Local::now().date_naive().to_string();
    let record_of_execution = config.record_of_execution.clone();

    if record_of_execution.is_empty() {
        return false;
    } else if record_of_execution.last().unwrap() == &today {
        return true;
    } else {
        return false;
    }
}

fn update_record_of_execution() {
    task_println!("Updating record of execution.");
    let today = Local::now().date_naive().to_string();
    let mut config = get_config();
    config.record_of_execution.push(today);
    Config::update(config).unwrap();
}

pub fn get_input_from_stdin() -> String {
    println!("Waiting for your input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn collect_club_rewards() {
    task_println!("Trying to get club rewards.");

    click_scaled(505, 939, get_config().wait_time);
    click_scaled(467, 305, get_config().wait_time);

    // collect club rewards
    click_scaled(320, 915, get_config().wait_time);

    // request new club rewards
    click_scaled(285, 839, get_config().wait_time);
    // === select LuoJi Yellow Book ===
    click_scaled(210, 500, get_config().wait_time);
    // click confirm
    click_scaled(390, 900, get_config().wait_time);
    click_scaled(390, 900, get_config().wait_time);

    // give others club rewards
    for _ in 0..5 {
        click_scaled(447, 733, get_config().wait_time);
        click_scaled(390, 918, get_config().wait_time);
        click_scaled(527,663, get_config().wait_time);
    }

    back_to_main_menu();
}

pub fn try_to_participate_in_competition() {
    task_println!("Trying to join competition.");

    click_scaled(380, 890, 0.0);

    task_println!("Do you want to join competition?");
    task_println!("Please `enter` or input `y`/`yes` to continue, or input other to skip.");
    let input = get_input_from_stdin();

    click_scaled(277, 905, get_config().wait_time);

    if input == "y" || input == "yes" || input == "" {
        task_println!("Joining competition.");
        participate_in_competition();
    } else {
        task_println!("Skip joining competition.");
    }
}

pub fn participate_in_competition() {
    task_println!("Participating in competition.");

    // go to competition page
    click_scaled(380, 890, get_config().wait_time);
    click_scaled(381, 683, get_config().wait_time_long);

    task_println!("Preparing to participate in competition for {} rounds.", get_config().competition_rounds);
    for _ in 0..get_config().competition_rounds {
        participate_in_competition_single_round();
    }
    task_println!("Competition finished.");

    back_to_main_menu();
}

fn participate_in_competition_single_round() {
    task_println!("Participating in competition single round.");

    // select player
    click_scaled(307, 816, get_config().wait_time);

    // start competition and skip
    click_scaled(280, 840, 7.0);
    click_scaled(458, 814, 7.0);
    click_scaled(282, 812, get_config().wait_time);

    // skip animation
    click_scaled(275, 652, get_config().wait_time);
    click_scaled(275, 652, get_config().wait_time);

    // get rewards
    click_scaled(283, 900, 5.0);
    click_scaled(283, 900, get_config().wait_time);
    click_scaled(283, 900, 5.0);
}

pub fn do_capsule_toys() {
    task_println!("Doing capsule toys.");
    /*
    492 720
    2s
    416 645
    2s

    282 854
    1s
    364 694 to 170 695
    1s
    453 695
    1s

    390 900
    1s
    283 900
    1s

    call back_to_main()
     */
    click_scaled(492, 720, get_config().wait_time);
    click_scaled(416, 645, get_config().wait_time);

    click_scaled(282, 854, get_config().wait_time);
    swipe_scaled(364, 694, 170, 695, get_config().wait_time);
    click_scaled(453, 695, get_config().wait_time);
    
    click_scaled(390, 900, get_config().wait_time);
    click_scaled(283, 900, get_config().wait_time);

    back_to_main_menu();

}

pub fn buy_something_in_coin_shop() {
    task_println!("Buying something in coin shop.");
    /*
    492 720
    2s
    157 736
    2s
    78 319
    1s
    400 900
    1s
    call back_to_main()
     */

    click_scaled(492, 720, get_config().wait_time);
    click_scaled(157, 736, get_config().wait_time);
    click_scaled(78, 319, get_config().wait_time);
    click_scaled(400, 900, get_config().wait_time);

    back_to_main_menu();
}

pub fn get_task_rewards() {
    task_println!("Getting task rewards.");
    /*
    get_task_rewards()
    501 438
    2s
    288 848
    2s
    278 898
    1s
    278 898
    1s
    call back_to_main()
     */

    click_scaled(501, 438, get_config().wait_time);
    click_scaled(288, 848, get_config().wait_time);
    click_scaled(278, 898, get_config().wait_time);
    click_scaled(278, 898, get_config().wait_time);

    back_to_main_menu();
}

pub fn get_pass_rewards() {
    task_println!("Getting pass rewards.");
    /*
    492 616
    2s
    280 837
    2s
    280 900
    1s
    call back_to_main()
    */

    click_scaled(492, 616, get_config().wait_time);
    click_scaled(280, 837, get_config().wait_time);
    click_scaled(280, 900, get_config().wait_time);

    back_to_main_menu();
}

pub fn strength_support_card() {
    task_println!("Strength support card.");
    /*
    179 891
    1s
    269 722
    1s

    279 661 to 279 418
    1s
    279 661 to 279 418
    1s
    279 661 to 279 418
    1s
    279 661 to 279 418
    1s
    279 661 to 279 418
    1s

    105 338
    1s
    105 338
    1s
    171 842
    1s
    400 900
    1s

    call back_to_main()
     */

    click_scaled(179, 891, get_config().wait_time);
    click_scaled(269, 722, get_config().wait_time);

    swipe_scaled(279, 661, 279, 418, get_config().wait_time);
    swipe_scaled(279, 661, 279, 418, get_config().wait_time);
    swipe_scaled(279, 661, 279, 418, get_config().wait_time);

    click_scaled(105, 338, get_config().wait_time);
    click_scaled(105, 338, get_config().wait_time);
    click_scaled(171, 842, get_config().wait_time);
    click_scaled(400, 900, get_config().wait_time);

    back_to_main_menu();
}

pub fn do_job() {
    task_println!("Doing job.");
    // collect
    click_scaled(53, 529, get_config().wait_time);
    click_scaled(286, 900, get_config().wait_time);
    click_scaled(286, 900, get_config().wait_time);
    click_scaled(286, 900, get_config().wait_time);
    click_scaled(286, 900, get_config().wait_time);
    click_scaled(286, 900, get_config().wait_time);

    /*
    MARK: new job 1
    145 534
    1s
    188 616
    1s
    290 845
    1s
    400 900
    1s
    290 845
    1s
    400 900
    1s
    290 845
    1s
     */
    click_scaled(145, 534, get_config().wait_time);
    click_scaled(188, 616, get_config().wait_time);
    click_scaled(290, 845, get_config().wait_time);
    click_scaled(400, 900, get_config().wait_time);
    click_scaled(290, 845, get_config().wait_time);
    click_scaled(400, 900, get_config().wait_time);
    click_scaled(290, 845, get_config().wait_time);

    /*
    MARK: new job 2
    416 517
    1s
    422 732
    1s
    290 845
    1s
    400 900
    1s
    290 845
    1s
    400 900
    1s
    290 845
    1s
     */
    click_scaled(416, 517, get_config().wait_time);
    click_scaled(422, 732, get_config().wait_time);
    click_scaled(290, 845, get_config().wait_time);
    click_scaled(400, 900, get_config().wait_time);
    click_scaled(290, 845, get_config().wait_time);
    click_scaled(400, 900, get_config().wait_time);
    click_scaled(290, 845, get_config().wait_time);

    back_to_main_menu();
}

pub fn collect_coin() {
    task_println!("Collecting coin.");
    click_scaled(53, 441, get_config().wait_time);
    click_scaled(286, 900, get_config().wait_time);
}

pub fn back_to_main_menu() {
    task_println!("Back to main menu.");
    click_scaled(93, 958, get_config().wait_time);
}

pub fn start_game() {
    task_println!("Starting kuyo & game.");

    // start kuyo
    click(504, 249, get_config().wait_time_long);
    task_println!("Kuyo started.");

    // click the button 1
    click(540, 1900, get_config().wait_time_long);
    task_println!("Button 1 clicked.");

    // click the button 2
    click(886, 687, get_config().wait_time_long);
    task_println!("Button 2 clicked.");

    // wait for gakumasu start & click any position
    task_println!("Waiting for game start...");
    sleep(get_config().wait_time_start_game_1);

    task_println!("Game started. Prepare to click any position.");
    click(540, 1300, get_config().wait_time);

    sleep(get_config().wait_time_start_game_2);

    click_scaled(290, 500, get_config().wait_time);

    // need to add more click to skip
    //   some animation of first time start every day
    click_scaled(290, 500, get_config().wait_time);
    click_scaled(290, 500, get_config().wait_time);
    click_scaled(290, 500, get_config().wait_time);
    click_scaled(290, 500, get_config().wait_time);
    click_scaled(290, 500, get_config().wait_time);
    click_scaled(290, 500, get_config().wait_time);
    click_scaled(290, 500, get_config().wait_time);

    task_println!("Game started.");
}

pub fn start_emulator() {
    task_println!("Starting emulator...");

    // 启动模拟器
    run_command_async(vec![
        format!("{}/{}", get_config().emulator_path, get_config().emulator_name).as_str(),
    ]);

    let wait_time = get_config().wait_time_start_emulator;

    task_println!("Emulator started. Waiting for {} seconds...", wait_time);
    
    sleep(wait_time);

    task_println!("{} seconds passed. Emulator is ready.", wait_time);

    click(1320, 228, get_config().wait_time);

    task_println!("Auto close ads.");
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