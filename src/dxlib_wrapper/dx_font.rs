use crate::dx_error::*;
use crate::dx_resouce::*;
use dxlib::*;
use dxlib_ffi::*;
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Drop;
use winapi::um::wingdi::{AddFontResourceExA, RemoveFontResourceExA, FR_PRIVATE};
#[derive(Debug, Clone)]
pub struct DxFontData {
    font_name: String,
    font_handle: i32,
    font_path: String,
    font_size: i32,
    font_thick: i32,
    font_type: i32,
}

impl DxFontData {
    pub fn new(
        font_path: &str,
        font_name: &str,
        font_size: i32,
        font_thick: i32,
        font_type: i32,
    ) -> DxFontData {
        DxFontData {
            font_path: String::from(font_path),
            font_name: String::from(font_name),
            font_size,
            font_thick,
            font_type,
            font_handle: 0,
        }
    }
}
impl DxResouce for DxFont {
    type Config = DxFontData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, DxErrorType> {
        unsafe {
            self.data = config.clone();
            let name = self.data.font_name.clone();
            let path = self.data.font_path.clone();
            let size = self.data.font_size;
            let thick = self.data.font_thick;
            let font_type = self.data.font_type;
            let res = self.add_resouce_data(&path);
            match res {
                Ok(_) => {
                    let handle = dx_CreateFontToHandle(&name, size, thick, font_type);
                    self.data.font_handle = handle;
                    if handle == -1 {
                        return Err(DxErrorType::ResouceE(DxResouceError::new(
                            "フォントハンドルの生成に失敗しました",
                            handle,
                        )));
                    }
                }
                Err(err) => match err {
                    DxErrorType::ResouceE(resouce) => {
                        return Err(DxErrorType::ResouceE(DxResouceError::new(
                            &resouce.get_message(),
                            -1,
                        )))
                    }
                    _ => {}
                },
            }
        }

        return Ok(self);
    }
    fn get(&self) -> Result<Self::GetVal, DxErrorType> {
        if self.data.font_handle != -1 {
            return Ok(self.data.font_handle);
        } else {
            return Err(DxErrorType::ResouceE(DxResouceError::new("", -1)));
        }
    }
    fn delete(&mut self) -> Result<&mut Self, DxErrorType> {
        let path = self.data.font_path.clone();
        let res = self.remove_resouce_data(&path);
        match res {
            Ok(_) => {
                let res = unsafe { dx_DeleteFontToHandle(self.data.font_handle) };
                if res == -1 {
                    return Err(DxErrorType::ResouceE(DxResouceError::new(
                        "フォントハンドルの削除に失敗しました",
                        res,
                    )));
                }
            }
            Err(val) => return Err(DxErrorType::ResouceE(DxResouceError::new("", -1))),
        }
        Ok(self)
    }
}

pub struct DxFont {
    data: DxFontData,
}

impl DxFont {
    pub fn new() -> DxFont {
        return {
            DxFont {
                data: DxFontData {
                    font_name: String::new(),
                    font_handle: 0,
                    font_path: String::new(),
                    font_size: 0,
                    font_thick: 0,
                    font_type: 0,
                },
            }
        };
    }
    pub fn add_resouce_data(&mut self, path: &str) -> Result<(), DxErrorType> {
        let result = unsafe {
            AddFontResourceExA(path.to_cstring().as_ptr(), FR_PRIVATE, std::ptr::null_mut())
        };
        if result == 0 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "フォントリソースの追加に失敗しました",
                result,
            )));
        }
        Ok(())
    }
    pub fn remove_resouce_data(&self, path: &str) -> Result<(), DxErrorType> {
        // フォントリソースを削除する
        let result = unsafe {
            RemoveFontResourceExA(path.to_cstring().as_ptr(), FR_PRIVATE, std::ptr::null_mut())
        };
        if result == 0 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "フォントリソースの削除に失敗しました",
                result,
            )));
        }
        Ok(())
    }
}
impl Drop for DxFont {
    fn drop(&mut self) {}
}
