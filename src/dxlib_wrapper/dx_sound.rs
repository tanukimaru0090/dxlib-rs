use crate::dx_common::dxlib::*;
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
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, String> {
        self.data = config.clone();
        let path = self.data.sound_path.clone();
        let handle = unsafe { dx_LoadSoundMem(&path) };
        if handle != -1 {
            self.data.sound_handle = handle;
            return Ok(self);
        } else if handle == -1 {
            return Err("サウンドハンドルの生成に失敗しました".to_string());
        }

        return Ok(self);
    }
    fn get(&self) -> Result<Self::GetVal, String> {
        if self.data.sound_handle != -1 {
            return Ok(self.data.sound_handle);
        } else {
            return Err("サウンドハンドルが無効です".to_string());
        }
    }
    fn delete(&mut self) -> Result<&mut Self, String> {
        let res = unsafe { dx_DeleteSoundMem(self.data.sound_handle) };
        if res != -1 {
            return Ok(self);
        } else {
            return Err("サウンドハンドルの削除に失敗しました".to_string());
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

/*
use crate::dx_common::dxlib::*;
use crate::dx_resouce::*;
// DxLibでの音声再生時のプレイタイプ
pub enum DxPlayType {
    Loop,
    Back,
    Normal,
}
#[derive(Debug, Clone)]
pub struct DxSound {
    //snd_handle:i32
    resouce: DxResouceData,
    myhandle1: i32,
}
impl DxResouce for DxSound {
    fn open_from_file(&mut self, path: &str) -> Result<DxSound, u32> {
        //fn my_load_memorys<T>(&self,path:&str)->T;
        let mut data = self.my_load_sound_mem(path);
        //return DxResouceData {path:path.to_string(),data:0};
        return Ok(DxSound {
            resouce: DxResouceData {
                path: path.to_string(),
                data: data,
                data_handle: Vec::new(),
            },
            myhandle1: data,
        });
    }
}

impl DxSound {
    // 新しくリソースデータを空で初期化し、スマートポインタを返す
    pub fn new() -> Box<DxSound> {
        return Box::new(DxSound {
            resouce: DxResouceData {
                path: String::new(),
                data: 0,
                data_handle: Vec::new(),
            },
            myhandle1: 0,
        });
    }
    // open_from_fileでメモリ上に読み込まれたハンドルを再生
    pub fn play_sound_memorys(&self, handle: i32, play_type: DxPlayType) -> Option<i32> {
        match play_type {
            DxPlayType::Normal => Some(dx_PlaySoundMem(handle, DX_PLAYTYPE_NORMAL)),
            DxPlayType::Back => Some(dx_PlaySoundMem(handle, DX_PLAYTYPE_BACK)),
            DxPlayType::Loop => Some(dx_PlaySoundMem(handle, DX_PLAYTYPE_LOOP)),
            _ => None,
        }
    }
    // dx_LoadSoundMemのヘルパー関数
    // 指定の音声ファイルをメンバの管理下に追加する
    pub fn my_load_sound_mem(&mut self, file_path: &str) -> i32 {
        let result = dx_LoadSoundMem(file_path);
        if result != -1 {
            self.resouce.data_handle.push(result);
        }
        return result;
    }
    // 管理下にあるハンドルをすべて解放する
    pub fn release(&mut self) {
        unsafe {
            if !self.resouce.data_handle.is_empty() {
                for item in self.resouce.data_handle.clone() {
                    dx_DeleteSoundMem(item.clone());
                }
            }
        }
    }
    pub fn get_handle(&self) -> i32 {
        return self.myhandle1;
    }
    pub fn set_handle(&mut self) {
        self.open_from_file("./touhou01.wav");
    }
}
*/
