extern crate alloc;
use crate::dx_common::dxlib::*;
use crate::dx_error::*;
use alloc::vec::Vec;
use std::thread::sleep;
use std::time::Duration;
const LIST_LEN_MAX: usize = 120; // Maximum number of frames to track
const FPS: i32 = 60; // Target FPS
const UPINTVL: i32 = 60; // Update interval

pub struct DxFps {
    counter: i32,
    fps: f32,
    list: Vec<u32>,
}

impl DxFps {
    pub fn new() -> DxFps {
        DxFps {
            counter: 0,
            fps: 0.0,
            list: Vec::new(),
        }
    }

    pub fn wait(&mut self) -> Result<(), DxErrorType> {
        self.counter += 1;
        let wait_time = self.get_wait_time()?;
        sleep(Duration::from_millis(wait_time as u64));
        self.regist();
        if self.counter == UPINTVL {
            self.update_average();
            self.counter = 0;
        }
        Ok(())
    }

    pub fn get_wait_time(&self) -> Result<i32, DxErrorType> {
        unsafe {
            let len = self.list.len();
            if len == 0 {
                return Ok(0);
            }
            let should_took_time = (1000.0 / FPS as f32) as i32 * (len as i32 + 1);

            let actually_took_time = dx_GetNowCount() - self.list[0] as i32;
            let wait_time = should_took_time - actually_took_time as i32;
            if wait_time > 0 {
                Ok(wait_time as i32)
            } else {
                Ok(0)
            }
        }
    }

    pub fn update_average(&mut self) {
        let len = self.list.len();
        if len < LIST_LEN_MAX || len <= 1 {
            return;
        }
        let last_frame_time = self.list[len - 1];
        let first_frame_time = self.list[0];
        let elapsed_time = last_frame_time - first_frame_time;
        let fps = if elapsed_time > 0 {
            (len as f32 - 1.0) / (elapsed_time as f32 / 1000.0)
        } else {
            0.0
        };
        self.fps = fps.round();
    }

    pub fn draw(&self, color: Color) {
        unsafe {
            dx_DrawString(0, 0, &format!("{}{}{}", "@fps[", self.fps, "]"), color);
        }
    }

    pub fn regist(&mut self) {
        unsafe {
            self.list.push(dx_GetNowCount().try_into().unwrap());
            if self.list.len() > LIST_LEN_MAX {
                self.list.remove(0);
            }
        }
    }
}
