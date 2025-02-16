use std::{path::Path, sync::Arc};
use aah_core::{resource::Resource, AAH};
use std::time::Duration;
use crate::{adb_println, config::Config, sleep::sleep, task_println};

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
}