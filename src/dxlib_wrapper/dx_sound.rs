use crate::dx_common::dxlib::*;
use crate::dx_error::*;
use crate::dx_resouce::*;
#[derive(Debug, Clone)]
pub struct DxSoundData {
    sound_path: String,
    sound_handle: i32,
}
impl DxSoundData {
    pub fn new(path: &str) -> DxSoundData {
        return DxSoundData {
            sound_path: String::from(path),
            sound_handle: 0,
        };
    }
    fn new_with_params() {}
}
#[derive(Debug, Clone)]
pub struct DxSound {
    data: DxSoundData,
}

impl DxResouce for DxSound {
    type Config = DxSoundData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, DxErrorType> {
        self.data = config.clone();
        let path = self.data.sound_path.clone();
        let handle = unsafe { dx_LoadSoundMem(&path) };
        if handle == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "サウンドハンドルの生成に失敗しました",
                handle,
            )));
        } else {
            self.data.sound_handle = handle;
            return Ok(self);
        }
    }
    fn get(&self) -> Result<Self::GetVal, DxErrorType> {
        if self.data.sound_handle == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "サウンドハンドルが無効です",
                self.data.sound_handle,
            )));
        } else {
            return Ok(self.data.sound_handle);
        }
    }
    fn delete(&mut self) -> Result<&mut Self, DxErrorType> {
        let res = unsafe { dx_DeleteSoundMem(self.data.sound_handle) };
        if res == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "サウンドハンドルの削除に失敗しました",
                res,
            )));
        } else {
            return Ok(self);
        }
    }
}

impl DxSound {
    pub fn new() -> DxSound {
        return DxSound {
            data: DxSoundData {
                sound_handle: 0,
                sound_path: String::new(),
            },
        };
    }
}
