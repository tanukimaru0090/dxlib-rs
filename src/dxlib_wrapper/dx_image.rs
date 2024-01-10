use crate::dx_common::dxlib::*;
use crate::dx_error::*;
use crate::dx_resouce::*;
use std::slice;

#[derive(Debug, Clone)]
pub struct DxImageData {
    image_path: String,
    image_handle: i32,
}
impl DxImageData {
    pub fn new(image_path: &str) -> DxImageData {
        return DxImageData {
            image_path: String::from(image_path),
            image_handle: 0,
        };
    }
}
#[derive(Debug, Clone)]
pub struct DxImage {
    data: DxImageData,
}

impl DxResouce for DxImage {
    type Config = DxImageData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, DxErrorType> {
        self.data = config.clone();
        let path = self.data.image_path.clone();
        let handle = unsafe { dx_LoadGraph(&path) };
        if handle == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "画像ハンドルの生成に失敗しました",
                handle,
            )));
        } else {
            self.data.image_handle = handle;
            return Ok(self);
        }
    }
    fn get(&self) -> Result<Self::GetVal, DxErrorType> {
        if self.data.image_handle == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "画像ハンドルが無効です",
                self.data.image_handle,
            )));
        } else {
            return Ok(self.data.image_handle);
        }
    }
    fn delete(&mut self) -> Result<&mut Self, DxErrorType> {
        if self.data.image_handle != -1 {
            let res = unsafe { dx_DeleteGraph(self.data.image_handle) };
            if res == -1 {
                return Err(DxErrorType::ResouceE(DxResouceError::new(
                    "画像ハンドルの削除に失敗しました",
                    res,
                )));
            }
        }
        return Ok(self);
    }
}
impl DxImage {
    pub fn new() -> DxImage {
        return DxImage {
            data: DxImageData {
                image_handle: 0,
                image_path: String::new(),
            },
        };
    }
}

#[derive(Debug, Clone)]
pub struct DxDivImageData {
    image_allnum: i32,
    image_xnum: i32,
    image_ynum: i32,
    image_xsize: i32,
    image_ysize: i32,
    image_handlebuf: *mut i32,
    image_path: String,
    image_handle: i32,
}
impl DxDivImageData {
    pub fn new(
        image_path: &str,
        image_allnum: i32,
        image_xnum: i32,
        image_ynum: i32,
        image_xsize: i32,
        image_ysize: i32,
        image_handlebuf: *mut i32,
    ) -> DxDivImageData {
        return DxDivImageData {
            image_path: String::from(image_path),
            image_allnum,
            image_handle: 0,
            image_xnum,
            image_ynum,
            image_xsize,
            image_ysize,
            image_handlebuf,
        };
    }
    fn new_with_params() {}
}
#[derive(Debug, Clone)]
pub struct DxDivImage {
    data: DxDivImageData,
}

impl DxResouce for DxDivImage {
    type Config = DxDivImageData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, DxErrorType> {
        self.data = config.clone();
        let path = self.data.image_path.clone();
        let res = unsafe {
            dx_LoadDivGraph(
                &path,
                self.data.image_allnum,
                self.data.image_xnum,
                self.data.image_ynum,
                self.data.image_xsize,
                self.data.image_ysize,
                self.data.image_handlebuf,
            )
        };
        if res == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "画像ハンドルの生成に失敗しました",
                res,
            )));
        } else {
            return Ok(self);
        }
    }
    fn get(&self) -> Result<Self::GetVal, DxErrorType> {
        return Ok(self.data.image_handle);
    }
    fn delete(&mut self) -> Result<&mut Self, DxErrorType> {
        let data_slice = unsafe {
            slice::from_raw_parts_mut(self.data.image_handlebuf, self.data.image_allnum as usize)
        };
        for &mut handle in data_slice {
            if handle != -1 {
                let res = unsafe { dx_DeleteGraph(handle) };
                if res == -1 {
                    return Err(DxErrorType::ResouceE(DxResouceError::new(
                        "画像ハンドルが無効です",
                        handle,
                    )));
                }
            }
        }
        Ok(self)
    }
}
impl DxDivImage {
    pub fn new() -> DxDivImage {
        return DxDivImage {
            data: DxDivImageData {
                image_handle: 0,
                image_path: String::new(),
                image_allnum: 0,
                image_xnum: 0,
                image_ynum: 0,
                image_xsize: 0,
                image_ysize: 0,
                image_handlebuf: std::ptr::null_mut(),
            },
        };
    }
}
