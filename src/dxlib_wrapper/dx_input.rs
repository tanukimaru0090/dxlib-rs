use crate::dx_common::dxlib::*;
use core::ffi::CStr;
pub struct DxInputProperty {}
pub enum DxInputStyle {
    Brink,
    NoBrink,
}

#[derive(Clone)]
pub struct DxInputData {
    input_buf: [i8; 200],
    input_str: String,
}
#[derive(Clone)]
pub struct DxInput {
    input_data: DxInputData,
}
impl Default for DxInputStyle {
    fn default() -> DxInputStyle {
        DxInputStyle::Brink
    }
}
impl DxInput {
    pub fn new(style: DxInputStyle) -> DxInput {
        unsafe {
            let result = match style {
                DxInputStyle::Brink => dx_SetKeyInputCursorBrinkFlag(TRUE),
                DxInputStyle::NoBrink => dx_SetKeyInputCursorBrinkFlag(FALSE),
                _ => -1,
            };
        }

        DxInput {
            input_data: DxInputData {
                input_buf: [0; 200],
                input_str: String::new(),
            },
        }
    }
    pub fn wait_input_key(&mut self, x: i32, y: i32, cancel_flag: i32) {
        unsafe {
            dx_KeyInputSingleCharString(
                x,
                y,
                199,
                self.input_data.input_buf.as_mut_ptr(),
                cancel_flag,
            );
            let mut buf = CStr::from_ptr(self.input_data.input_buf.as_ptr());
            self.input_data.input_str = buf.to_str().unwrap().to_string();
        }
    }
    pub fn wait_input_key_japanise(&mut self, x: i32, y: i32, cancel_flag: i32) {
        unsafe {
            dx_KeyInputString(
                x,
                y,
                199,
                self.input_data.input_buf.as_mut_ptr(),
                cancel_flag,
            );
            let mut buf = CStr::from_ptr(self.input_data.input_buf.as_ptr());
            self.input_data.input_str = buf.to_str().unwrap().to_string();
        }
    }
    pub fn get_input_str(&self) -> Option<String> {
        if self.input_data.input_str.clone().is_empty() {
            None
        } else {
            Some(self.input_data.input_str.clone())
        }
    }
}
