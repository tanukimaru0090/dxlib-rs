extern crate alloc;
use crate::dx_common::dxlib::*;
use crate::dx_error::*;
use crate::my_file::MyFile;
use std::default::Default;
// ビデオモード時のデータ
pub struct DxVideoModeData {
    width: i32,    // クライアントサイズのwidth
    height: i32,   // クライアントサイズのheight
    colorbit: i32, // カラービット
}

//ウィンドウプロパティ
pub enum DxWindowStyle {
    Mode0 = 0,   // デフォルト
    Mode1 = 1,   // タイトルバーなし
    Mode2 = 2,   // タイトルバーなし枠なし
    Mode3 = 3,   // 枠なし
    Mode4 = 4,   // 何もなし
    Mode5 = 5,   // 最小化なし
    Mode6 = 6,   // ツールバーウィンドウ
    Mode7 = 7,   // 最大化つきウィンドウ、初期状態が通常サイズ
    Mode8 = 8,   // 最大化付きウィンドウ、初期状態が最大化状態
    Mode9 = 9,   // デフォルトに緑の立体化なし
    Mode10 = 10, // 最大化つきウィンドウ、緑の立体化なし
    Mode11 = 11, // 閉じるなし、最小化なし
}
impl Default for DxWindowStyle {
    fn default() -> Self {
        DxWindowStyle::Mode0
    }
}

pub struct DxWindow {
    videomode_data: DxVideoModeData,
    title: String,
    style: DxWindowStyle,
    window_dest_flag: bool,
}

//ウィンドウクラス
impl DxWindow {
    //DxLib_Init()を呼出し後の初期化処理(デフォルト値を使う場合のみ)
    pub fn initialize(&self) {
        unsafe {
            dx_SetDrawScreen(DX_SCREEN_BACK);
            //dx_SetUse3DFlag(TRUE);
            dx_SetAlwaysRunFlag(TRUE);
        }
    }
    //DxLibの終了処理
    pub fn finalize(&self) {
        unsafe {}
    }
    //新しくウィンドウ情報を空で初期化
    pub fn new() -> DxWindow {
        DxWindow {
            videomode_data: DxVideoModeData {
                width: 0,
                height: 0,
                colorbit: 0,
            },
            title: String::new(),
            style: DxWindowStyle::default(),
            window_dest_flag: false,
        }
    }

    //新しくウィンドウを作成
    pub fn create_window(
        &mut self,
        videomode: DxVideoModeData,
        title: &str,
        style: DxWindowStyle,
    ) -> Result<&mut DxWindow, DxErrorType> {
        unsafe {
            //Dxlib_Initを呼ぶ前に必要な処理
            {
                dx_SetWindowStyleMode(style as i32);
                dx_SetWaitVSyncFlag(TRUE);
                dx_SetUseCharCodeFormat(DX_CHARCODEFORMAT_UTF8);
                dx_SetMainWindowText(title);
                dx_SetGraphMode(videomode.width, videomode.height, videomode.colorbit, 120);
                dx_ChangeWindowMode(TRUE);
            }
            let dxinit = self.dx_init();
            match dxinit {
                Ok(val) => {
                    MyFile::new("./game.GAME_LOG").file_write_all(val);
                }
                Err(err) => match err {
                    DxErrorType::DxLibE(dxlibe) => {
                        MyFile::new("./game.GAME_LOG").file_write_all(dxlibe.get_message());
                    }
                    _ => {}
                },
            }
            self.initialize();
        }
        Ok(self)
    }
    // DxLib_End関数のエラーハンドリング
    pub fn dx_end(&self) -> Result<String, DxErrorType> {
        unsafe {
            let res = dx_DxLib_End();
            if res == -1 {
                Err(DxErrorType::DxLibE(DxError::new(
                    "DxLib_End: Error Please! open Log.txt immediately\n",
                    res,
                )))
            } else {
                Ok("DxLib_End: Success!\n".to_string())
            }
        }
    }
    // DxLib_End関数のエラーハンドリング
    pub fn dx_init(&self) -> Result<String, DxErrorType> {
        unsafe {
            let res = dx_DxLib_Init();
            if res == -1 {
                Err(DxErrorType::DxLibE(DxError::new(
                    "DxLib_Init: Error Please! open Log.txt immediately\n",
                    res,
                )))
            } else {
                Ok("DxLib_Init: Success!\n".to_string())
            }
        }
    }

    pub fn is_dest(&self) -> bool {
        return self.window_dest_flag;
    }
    //ウィンドウが開いてるかどうか判定する
    pub fn is_open(&mut self) -> bool {
        unsafe {
            if dx_ProcessMessage() == 0 {
                return true;
            }
            return false;
        }
    }
    pub fn videomode(width: i32, height: i32, color_bit: i32) -> DxVideoModeData {
        DxVideoModeData {
            width: width,
            height: height,
            colorbit: color_bit,
        }
    }
}
impl Drop for DxWindow {
    fn drop(&mut self) {
        self.finalize();
        unsafe {
            let dxend = self.dx_end();
            match dxend {
                // 失敗したときの処理
                Ok(val) => {
                    MyFile::new("./game.GAME_LOG").file_write_append(val);
                }
                Err(err) => match err {
                    DxErrorType::DxLibE(dxlibe) => {
                        MyFile::new("./game.GAME_LOG").file_write_append(dxlibe.get_message());
                    }
                    _ => {}
                },
            }
        }
    }
}
