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
    use std::f64::consts::PI;
    const TEST_MIN_WINDOW_WIDTH: i32 = 640;
    const TEST_MIN_WINDOW_HEIGHT: i32 = 480;
    const TEST_MAX_WINDOW_WIDTH: i32 = 1280;
    const TEST_MAX_WINDOW_HEIGHT: i32 = 800;
    const TEST_WINDOW_TITLE: &str = "test-window";

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        change_window_mode(TRUE)?;
        set_use_charcode_format(DX_CHARCODEFORMAT_UTF8)?;
        set_main_window_text(TEST_WINDOW_TITLE)?;
        set_graph_mode(TEST_MAX_WINDOW_WIDTH, TEST_MAX_WINDOW_HEIGHT, 32, 120)?;
        dxlib_init()?;
        set_always_run_flag(TRUE)?;
        set_draw_screen(DX_SCREEN_BACK)?;
        let mut fps = Fps::new();
        let mut key = KeyBoard::new();
        //let mut img = load_graph("/Users/daruma/Downloads/kisida.jpg")?;
        play_music("/Users/daruma/Downloads/touhou-music.mp3", DX_PLAYTYPE_BACK)?;
        let (mut x, mut y, mut size_x, mut size_y) = (
            (TEST_MAX_WINDOW_WIDTH - 500) / 2,
            (TEST_MAX_WINDOW_HEIGHT - 500) / 2,
            500,
            500,
        );
        let (mut red, mut green, mut blue) = (10, 10, 10);
        let (mut angle, mut radius, mut time, mut color) = (
            0.0,
            5.0,
            10,
            get_color(red, green, blue).ok_or("カラーコードが無効です")?,
        );
        loop {
            if let Err(err) = process_message() {
                println!("ウィンドウが閉じられたよーーー: {}", err);
                break;
            }
            key.update()?;

            // 四角形の中心座標を計算
            let center_x = x + size_x / 2;
            let center_y = y + size_y / 2;

            let angle_rad = angle * PI / 180.0; // 度からラジアンに変換
            let mut points = vec![
                (x, y),
                (x + size_x, y),
                (x + size_x, y + size_y),
                (x, y + size_y),
            ];
            for point in points.iter_mut() {
                // 中心を原点とした座標に変換
                let mut x_shifted = point.0 - center_x;
                let mut y_shifted = point.1 - center_y;

                // 座標を回転
                let x_new = x_shifted as f64 * angle_rad.cos() - y_shifted as f64 * angle_rad.sin();
                let y_new = x_shifted as f64 * angle_rad.sin() + y_shifted as f64 * angle_rad.cos();

                // 中心を加えて座標を戻す
                point.0 = (x_new as i32 + center_x) as i32;
                point.1 = (y_new as i32 + center_y) as i32;
            }

            clear_draw_screen()?;

            // 回転後の座標で四角形を描画
            draw_box(
                points[0].0,
                points[0].1,
                points[1].0,
                points[1].1,
                color,
                FALSE,
            )?;
            draw_box(
                points[1].0,
                points[1].1,
                points[2].0,
                points[2].1,
                color,
                FALSE,
            )?;
            draw_box(
                points[2].0,
                points[2].1,
                points[3].0,
                points[3].1,
                color,
                FALSE,
            )?;
            draw_box(
                points[3].0,
                points[3].1,
                points[0].0,
                points[0].1,
                color,
                FALSE,
            )?;
            angle += 1.0;
            color = get_color(red, green, blue).ok_or("カラーコードが無効です")?;
            if red < 255 && green < 255 && blue < 255 {
                red += 1;
                green += 1;
                blue += 1;
            } else {
                red = 0;
                green = 0;
                blue = 0;
            }
            fps.wait()?;
            fps.draw(get_color(255, 255, 255).ok_or("カラーコードが無効です")?);
            screen_flip()?;
        }
        dxlib_end()?;
        Ok(())
    }
}
