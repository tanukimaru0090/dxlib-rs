pub mod common;
pub mod dxlib;
pub mod my_file;
#[cfg(feature = "utils")]
pub mod utils;

pub use common::dxlib::*;
pub use common::dxlib_const_variables::*;
pub use common::dxlib_types::*;

#[cfg(test)]
mod tests {
    use crate::dxlib::*;
    use crate::utils::{Fps, KeyBoard};
    use crate::*;
    use std::env::args;
    use std::f64::consts::PI;
    const TEST_MIN_WINDOW_WIDTH: i32 = 640;
    const TEST_MIN_WINDOW_HEIGHT: i32 = 480;
    const TEST_MAX_WINDOW_WIDTH: i32 = 1280;
    const TEST_MAX_WINDOW_HEIGHT: i32 = 800;
    const TEST_WINDOW_TITLE: &str = "test-window";
    const TEST_GRAPH_PATH: &str = "/Users/daruma/Downloads/irisu203/irisu203/photo.png";
    const TEST_MODEL_PATH: &str =
        "/Users/daruma/Downloads/3Dsample/3Dsample/dat/Lat式ミク/Lat式ミクVer2.3_Normal.pmd";
    const TEST_MUSIC_PATH: &str = "/Users/daruma/Downloads/touhou-sinki.wav";
    #[test]
    fn init() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        dxlib_init()?;
        wait_key();
        dxlib_end()?;
        Ok(())
    }

    #[test]
    fn mv1() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        dxlib_init()?;
        let model_handle = mv1_load_model(TEST_MODEL_PATH)?;
        mv1_draw_model(model_handle)?;
        wait_key();
        dxlib_end()?;
        Ok(())
    }
    #[test]
    fn string1() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        dxlib_init()?;
        draw_string(0, 0, "hello world!", get_color(255, 255, 255).unwrap())?;
        wait_key();
        dxlib_end()?;
        Ok(())
    }
    #[test]
    fn string2() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        dxlib_init()?;
        draw_string(0, 0, "ハロー ワールド!", get_color(255, 255, 255).unwrap())?;
        wait_key();
        dxlib_end()?;
        Ok(())
    }
    #[test]
    fn graph1() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        dxlib_init()?;
        let img_handle = load_graph(TEST_GRAPH_PATH)?;
        draw_graph(0, 0, img_handle, TRUE)?;
        wait_key();
        dxlib_end()?;
        Ok(())
    }
    #[test]
    fn graph2() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        dxlib_init()?;
        let img_handle = load_graph(TEST_GRAPH_PATH)?;
        draw_extend_graph(0, 0, 100, 100, img_handle, TRUE)?;
        wait_key();
        dxlib_end()?;
        Ok(())
    }
    #[test]
    fn sound1() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        dxlib_init()?;
        wait_key();
        dxlib_end()?;

        Ok(())
    }
    #[test]
    fn sound2() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        dxlib_init()?;
        wait_key();
        dxlib_end()?;

        Ok(())
    }
    #[test]
    fn music1() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        dxlib_init()?;
        let music_handle = load_music_mem(TEST_MUSIC_PATH)?;
        play_music_mem(music_handle, DX_PLAYTYPE_BACK, 1)?;
        wait_key();
        dxlib_end()?;

        Ok(())
    }
    #[test]
    fn music2() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        dxlib_init()?;
        wait_key();
        dxlib_end()?;

        Ok(())
    }
    #[test]
    fn test_dxlib() -> Result<(), Box<dyn std::error::Error>> {
        let args: Vec<String> = args().collect();
        let input: String = args[1].to_string();
        match input.as_str() {
            "music1" => {
                music1()?;
            }
            "music2" => {
                music2()?;
            }
            "sound1" => {
                sound1()?;
            }
            "sound2" => {
                sound2()?;
            }
            "string1" => {
                string1()?;
            }
            "string2" => {
                string2()?;
            }
            "" => {}
            "" => {}
            "" => {}
            "graph1" => {
                graph1()?;
            }
            "graph2" => {
                graph2()?;
            }
            _ => {}
        }
        Ok(())
    }
}
