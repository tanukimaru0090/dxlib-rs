use crate::dx_common::dxlib::*;
use crate::dx_error::*;
use crate::dx_resouce::*;

#[derive(Debug, Clone)]
pub struct DxMusicData {
    music_path: String,
    music_handle: i32,
}
impl DxMusicData {
    pub fn new(music_path: &str) -> DxMusicData {
        return DxMusicData {
            music_path: String::from(music_path),
            music_handle: 0,
        };
    }
    fn new_with_params() {}
}
#[derive(Debug, Clone)]
pub struct DxMusic {
    data: DxMusicData,
}

impl DxResouce for DxMusic {
    type Config = DxMusicData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, DxErrorType> {
        self.data = config.clone();
        let path = self.data.music_path.clone();
        let handle = unsafe { dx_LoadMusicMem(&path) };
        if handle == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "サウンドハンドルの生成に失敗しました",
                handle,
            )));
        } else {
            self.data.music_handle = handle;
            return Ok(self);
        }
    }
    fn get(&self) -> Result<Self::GetVal, DxErrorType> {
        if self.data.music_handle == -1 {
            return Err(DxErrorType::ResouceE(DxResouceError::new(
                "サウンドハンドルが無効です",
                self.data.music_handle,
            )));
        } else {
            return Ok(self.data.music_handle);
        }
    }
    fn delete(&mut self) -> Result<&mut Self, DxErrorType> {
        let res = unsafe { dx_DeleteMusicMem(self.data.music_handle) };
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

impl DxMusic {
    pub fn new() -> DxMusic {
        return DxMusic {
            data: DxMusicData {
                music_handle: 0,
                music_path: String::new(),
            },
        };
    }
}
