use std::{path::Path, sync::Arc};
use aah_core::{resource::Resource, AAH};
use anyhow::{anyhow, Error};
use image::DynamicImage;
use std::time::Duration;
use crate::{adb_println, task_println, prelude::*};

pub struct AahWrapper {
    pub aah: AAH,
    config: Config,
}

impl AahWrapper {
    pub fn new(config: Config) -> Self {
        let resource_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources");
        task_println!("Loading resource from {:?}", resource_path);

        let resource = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(Resource::try_init_by_specific_dir(resource_path))
            .expect("failed to load resource");

        let aah = AAH::connect_with_adb_controller(
            config.host.clone(),
            Arc::new(resource.into())
        ).expect("failed to connect to the device");

        AahWrapper { aah, config }
    }

    pub fn click_scaled(&self, x: u32, y: u32, wait_time: f64) {
    
        let x = (x as f64 * self.config.scale_x) as u32;
        let y = (y as f64 * self.config.scale_y) as u32;
    
        self.click(x, y, wait_time);
    }
    
    pub fn click(&self, x: u32, y: u32, wait_time: f64) {
        // adb_println!("Clicking screen at ({}, {})...", x, y);
    
        // FIXME: whether to use `click` or `click_scaled`?
        if let Err(e) = self.aah.click(x, y) {
            adb_println!("Failed to click screen: {:?}", e);
        }

    
        sleep(wait_time);
    }
    
    pub fn swipe_scaled(&self, x1: u32, y1: u32, x2: u32, y2: u32, wait_time: f64) {
        let x1 = (x1 as f64 * self.config.scale_x) as u32;
        let y1 = (y1 as f64 * self.config.scale_y) as u32;
        let x2 = (x2 as f64 * self.config.scale_x) as u32;
        let y2 = (y2 as f64 * self.config.scale_y) as u32;
    
        self.swipe(x1, y1, x2, y2, wait_time);
    }
    
    pub fn swipe(&self, x1: u32, y1: u32, x2: u32, y2: u32, wait_time: f64) {
        // adb_println!("Swiping from ({}, {}) to ({}, {})...", x1, y1, x2, y2);
    
        let res = self.aah.swipe(
            (x1, y1), (x2 as i32, y2 as i32), Duration::from_secs_f32(0.5), 0.0, 0.0
        );
        if let Err(e) = res {
            adb_println!("Failed to swipe: {:?}", e);
        }
        
        sleep(wait_time);
    }

    pub fn screencap(&self) -> Result<DynamicImage, anyhow::Error> {
        match self.aah.screencap() {
            Ok(img) => Ok(img),
            Err(e) => Err(anyhow::anyhow!("Failed to capture screen: {:?}", e))
        }
    }

    pub fn find(&self, target: DynamicImage) -> Result<(i32, i32, f32), Error> {
        let pic = self.screencap().unwrap();
        match template_match(&pic, &target) {
            Some((x, y, v)) => Ok((x, y, v)),
            None => Err(anyhow!("Not found target image!")),
        }
    }

    pub fn find_and_click(&self, str: &str) -> Result<(), Error> {
        let target = open_image(str)?;
        let ans = self.find(target);
        match ans {
            Ok((x, y, _)) => {
                adb_println!("Clicking at ({}, {}) for \"{}\"", x, y, str);
                self.click(x as u32, y as u32, 0.0);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    pub fn find_and_click_until(&self, str: &str, wait_time: f64, retry_times: i32) -> Result<(), Error> {
        for idx in 0..retry_times {
            adb_println!("Trying to find target image: \"{}\" for the {}-th time", str, idx + 1);
            if self.find_and_click(str).is_ok() {
                adb_println!("Found target image: \"{}\"", str);
                return Ok(());
            }
            sleep(wait_time);
        }
        adb_println!("Not found target image: \"{}\"", str);
        Err(anyhow!("Not found target image!"))
    }
    
    pub fn find_and_click_until_default(&self, str: &str) -> Result<(), Error> {
        self.find_and_click_until(str, get_config().retry_wait_time, get_config().retry_max_times)
    }

    pub fn find_and_click_util_and_sleep(&self, str: &str, wait_time: f64, retry_times: i32, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_until(str, wait_time, retry_times)?;
        sleep(sleep_time);
        Ok(())
    }

    pub fn find_and_click_until_default_and_sleep(&self, str: &str, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_until(str, get_config().retry_wait_time, get_config().retry_max_times)?;
        sleep(sleep_time);
        Ok(())
    }

    pub fn fcus(&self, str: &str, wait_time: f64, retry_times: i32, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_util_and_sleep(str, wait_time, retry_times, sleep_time)
    }

    pub fn fcuds(&self, str: &str, sleep_time: f64) -> Result<(), Error> {
        self.find_and_click_until_default_and_sleep(str, sleep_time)
    }
}