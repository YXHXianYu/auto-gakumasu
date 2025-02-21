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

    let aah_wrapper = AahWrapper::new(get_config().clone());
    aah_wrapper.close_emulator_ads();
    aah_wrapper.start_game();

    aah_wrapper.collect_coin();
    aah_wrapper.do_job();
    aah_wrapper.strength_support_card();
    aah_wrapper.do_capsule_toys();
    aah_wrapper.buy_something_in_coin_shop();
    aah_wrapper.get_task_rewards();
    aah_wrapper.get_pass_rewards();

    aah_wrapper.try_to_participate_in_competition();
    aah_wrapper.try_to_buy_something_in_ap_shop();

    aah_wrapper.collect_club_rewards();

    update_record_of_execution();

    task_println!("Daily task finished.");

    // end_emulator();
}

impl AahWrapper {
    fn collect_club_rewards(&self) {
        task_println!("Trying to get club rewards.");

        self.click_scaled(505, 939, get_config().wait_time);
        self.click_scaled(467, 305, get_config().wait_time);

        self.click_scaled(290, 908, get_config().wait_time);

        // collect club rewards
        self.click_scaled(320, 915, get_config().wait_time);

        // request new club rewards
        self.click_scaled(285, 839, get_config().wait_time);
        // === select LuoJi Yellow Book ===
        self.click_scaled(210, 500, get_config().wait_time);
        // click confirm
        self.click_scaled(390, 900, get_config().wait_time);
        self.click_scaled(390, 900, get_config().wait_time);

        // give others club rewards
        for _ in 0..5 {
            self.click_scaled(447, 733, get_config().wait_time);
            self.click_scaled(390, 918, get_config().wait_time);
            self.click_scaled(527,663, get_config().wait_time);
        }

        self.back_to_main_menu();
    }

    fn try_to_participate_in_competition(&self) {
        task_println!("Trying to join competition.");

        self.click_scaled(380, 890, 0.0);

        task_println!("Do you want to join competition?");
        task_println!("Press `enter` or input `y`/`yes` to continue, or input other to skip.");
        let input = get_input_from_stdin();

        self.click_scaled(277, 905, get_config().wait_time);

        if input == "y" || input == "yes" || input == "" {
            task_println!("Joining competition.");
            self.participate_in_competition();
        } else {
            task_println!("Skip joining competition.");
        }
    }

    fn participate_in_competition(&self) {
        task_println!("Participating in competition.");

        // go to competition page
        self.click_scaled(380, 890, get_config().wait_time);
        self.click_scaled(381, 683, get_config().wait_time_long);

        task_println!("Preparing to participate in competition for {} rounds.", get_config().competition_rounds);
        for _ in 0..get_config().competition_rounds {
            self.participate_in_competition_single_round();
        }
        task_println!("Competition finished.");

        self.back_to_main_menu();
    }

    fn participate_in_competition_single_round(&self) {
        task_println!("Participating in competition single round.");

        // select player
        self.click_scaled(307, 816, get_config().wait_time);

        // start competition and skip
        self.click_scaled(280, 840, 8.0);
        self.click_scaled(458, 814, 8.0);
        self.click_scaled(282, 812, 3.0);

        // skip animation
        self.click_scaled(275, 652, 3.0);
        self.click_scaled(275, 652, 3.0);

        // get rewards
        self.click_scaled(283, 900, 5.0);
        self.click_scaled(283, 900, get_config().wait_time);
        self.click_scaled(283, 900, 5.0);
    }

    fn do_capsule_toys(&self) {
        task_println!("Doing capsule toys.");

        self.click_scaled(492, 720, get_config().wait_time);
        self.click_scaled(416, 645, get_config().wait_time);

        self.click_scaled(282, 854, get_config().wait_time);
        self.swipe_scaled(364, 694, 170, 695, get_config().wait_time);
        self.click_scaled(453, 695, get_config().wait_time);
        
        self.click_scaled(390, 900, get_config().wait_time);
        self.click_scaled(283, 900, get_config().wait_time);

        self.back_to_main_menu();

    }

    fn try_to_buy_something_in_ap_shop(&self) {
        task_println!("Trying to buy something in AP shop.");

        println!("Do you want to buy something in AP shop?");
        println!("Press `enter` to skip! Input `y`/`yes` to continue, or input other to skip.");
        let input = get_input_from_stdin();

        if input == "y" || input == "yes" {
            task_println!("Buying something in AP shop.");
            self.buy_something_in_ap_shop();
        } else {
            task_println!("Skip buying something in AP shop.");
        }
    }

    fn buy_something_in_ap_shop(&self) {
        task_println!("Buying something in AP shop.");

        // go to AP shop
        self.click_scaled(490, 720, 3.0);
        self.click_scaled(155, 740, 3.0);
        self.click_scaled(400, 847, get_config().wait_time);

        // buy 再生成
        for _ in 0..2 {
            self.click_scaled(474, 319, get_config().wait_time_short);
            self.click_scaled(392, 916, get_config().wait_time_short);
        }

        // buy 再挑战
        for _ in 0..2 {
            self.click_scaled(345, 317, get_config().wait_time_short);
            self.click_scaled(392, 916, get_config().wait_time_short);
        }

        // buy 笔记数量提升
        for _ in 0..2 {
            self.click_scaled(212, 317, get_config().wait_time_short);
            self.click_scaled(392, 916, get_config().wait_time_short);
        }

        // buy 支援强化Pt提升
        for _ in 0..2 {
            self.click_scaled(78, 315, get_config().wait_time_short);
            self.click_scaled(392, 916, get_config().wait_time_short);
        }

        self.back_to_main_menu();
    }

    fn buy_something_in_coin_shop(&self) {
        task_println!("Buying something in coin shop.");

        self.click_scaled(492, 720, get_config().wait_time);
        self.click_scaled(157, 736, get_config().wait_time);
        self.click_scaled(78, 319, get_config().wait_time);
        self.click_scaled(400, 900, get_config().wait_time);

        self.back_to_main_menu();
    }

    fn get_task_rewards(&self) {
        task_println!("Getting task rewards.");

        self.click_scaled(501, 438, get_config().wait_time);
        self.click_scaled(288, 848, get_config().wait_time);
        self.click_scaled(278, 898, get_config().wait_time);
        self.click_scaled(278, 898, get_config().wait_time);

        self.back_to_main_menu();
    }

    fn get_pass_rewards(&self) {
        task_println!("Getting pass rewards.");

        self.click_scaled(492, 616, get_config().wait_time);
        self.click_scaled(280, 837, get_config().wait_time);
        self.click_scaled(280, 900, get_config().wait_time);

        self.back_to_main_menu();
    }

    fn strength_support_card(&self) {
        task_println!("Strength support card.");

        self.click_scaled(179, 891, get_config().wait_time);
        self.click_scaled(269, 722, get_config().wait_time);

        self.swipe_scaled(279, 661, 279, 418, get_config().wait_time);
        self.swipe_scaled(279, 661, 279, 418, get_config().wait_time);
        self.swipe_scaled(279, 661, 279, 418, get_config().wait_time);

        self.click_scaled(105, 338, get_config().wait_time);
        self.click_scaled(105, 338, get_config().wait_time);
        self.click_scaled(171, 842, get_config().wait_time);
        self.click_scaled(400, 900, get_config().wait_time);

        self.back_to_main_menu();
    }

    fn do_job(&self) {
        task_println!("Doing job.");
        // collect
        self.click_scaled(53, 529, get_config().wait_time);
        self.click_scaled(286, 900, get_config().wait_time);
        self.click_scaled(286, 900, get_config().wait_time);
        self.click_scaled(286, 900, get_config().wait_time);
        self.click_scaled(286, 900, get_config().wait_time);
        self.click_scaled(286, 900, get_config().wait_time);

        // MARK: new job 1
        self.click_scaled(145, 534, get_config().wait_time);
        self.click_scaled(188, 616, get_config().wait_time);
        self.click_scaled(290, 845, get_config().wait_time);
        self.click_scaled(400, 900, get_config().wait_time);
        self.click_scaled(290, 845, get_config().wait_time);
        self.click_scaled(400, 900, get_config().wait_time);
        self.click_scaled(290, 845, get_config().wait_time);

        // MARK: new job 2
        self.click_scaled(416, 517, get_config().wait_time);
        self.click_scaled(422, 732, get_config().wait_time);
        self.click_scaled(290, 845, get_config().wait_time);
        self.click_scaled(400, 900, get_config().wait_time);
        self.click_scaled(290, 845, get_config().wait_time);
        self.click_scaled(400, 900, get_config().wait_time);
        self.click_scaled(290, 845, get_config().wait_time);

        self.back_to_main_menu();
    }

    fn collect_coin(&self) {
        task_println!("Collecting coin.");
        self.click_scaled(53, 441, get_config().wait_time);
        self.click_scaled(286, 900, get_config().wait_time);
    }

    fn back_to_main_menu(&self) {
        task_println!("Back to main menu.");
        self.click_scaled(93, 958, get_config().wait_time);
    }

    fn close_emulator_ads(&self) {
        task_println!("Auto close ads.");
        self.click(1320, 228, get_config().wait_time);
    }

    fn start_game(&self) {
        task_println!("Starting kuyo & game.");

        // start kuyo
        self.fcuds("kuyo/kuyo.png", get_config().wait_time_long).unwrap();
        task_println!("Kuyo started.");

        // click the button 1
        self.fcuds("kuyo/tab_3.png", get_config().wait_time_long).unwrap();
        task_println!("Button 1 clicked.");

        // click the button 2
        // self.click(886, 687, get_config().wait_time_long);
        self.fcuds("kuyo/k_space_start.png", get_config().wait_time_long).unwrap();
        task_println!("Button 2 clicked.");

        self.click_scaled(386, 586, get_config().wait_time_long);
        task_println!("更新游戏版本");

        self.click_scaled(383, 587, get_config().wait_time_long);
        task_println!("更新汉化包");

        self.fcuds("kuyo/k_space_start.png", get_config().wait_time_long).unwrap();
        task_println!("Button 2 clicked.");

        // wait for gakumasu start & click any position
        task_println!("Waiting for game start...");
        sleep(get_config().wait_time_start_game_1);

        task_println!("Game started. Prepare to click any position.");
        self.click(540, 1300, get_config().wait_time);

        sleep(get_config().wait_time_start_game_2);

        self.click_scaled(290, 500, get_config().wait_time);

        // need to add more click to skip
        //   some animation of first time start every day
        self.click_scaled(290, 500, get_config().wait_time_short);
        self.click_scaled(290, 500, get_config().wait_time_short);
        self.click_scaled(290, 500, get_config().wait_time_short);
        self.click_scaled(290, 500, get_config().wait_time_short);
        self.click_scaled(290, 500, get_config().wait_time_short);
        self.click_scaled(290, 500, get_config().wait_time_short);
        self.click_scaled(290, 500, get_config().wait_time_short);

        // skip new activity
        self.click_scaled(276, 921, get_config().wait_time_short);
        self.click_scaled(276, 921, get_config().wait_time_short);
        self.click_scaled(276, 921, get_config().wait_time_short);
        self.click_scaled(276, 921, get_config().wait_time_short);

        task_println!("Game started.");
    }

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