use crate::common::DxError;
use crate::*;
pub fn set_always_run_flag(flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetAlwaysRunFlag(flag) };
    if res == -1 {
        return Err(DxError::new(" Erorr", res));
    } else {
        return Ok(res);
    }
}

pub fn clear_draw_screen() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_ClearDrawScreen() };
    if res == -1 {
        return Err(DxError::new(" Erorr", res));
    } else {
        return Ok(res);
    }
}
pub fn set_graph_mode(
    size_x: i32,
    size_y: i32,
    color_bit_num: i32,
    refresh_rate: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetGraphMode(size_x, size_y, color_bit_num, refresh_rate) };
    if res == -1 {
        return Err(DxError::new(" Erorr", res));
    } else {
        return Ok(res);
    }
}

pub fn screen_flip() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_ScreenFlip() };
    if res == -1 {
        return Err(DxError::new(" Erorr", res));
    } else {
        return Ok(res);
    }
}
pub fn set_draw_screen(draw_screen: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetDrawScreen(draw_screen) };
    if res == -1 {
        return Err(DxError::new("Error", res));
    } else {
        return Ok(res);
    }
}

pub fn dxlib_init() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DxLib_Init() };
    if res == -1 {
        return Err(DxError::new("DxLib_Init() Erorr", res));
    } else {
        return Ok(res);
    }
}
pub fn dxlib_end() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DxLib_End() };
    if res == -1 {
        return Err(DxError::new("DxLib_End() Error", res));
    } else {
        return Ok(res);
    }
}
pub fn process_message() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_ProcessMessage() };
    if res == -1 {
        return Err(DxError::new("ProcessMessage() Error", res));
    } else {
        return Ok(res);
    }
}
pub fn draw_string(x: i32, y: i32, string: &str, color: u32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawString(x, y, string, color) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn get_color(red: i32, green: i32, blue: i32) -> Option<u32> {
    if red < 0 || green < 0 || blue < 0 || red > 255 || green > 255 || blue > 255 {
        return None;
    }
    Some(unsafe { dx_GetColor(red, green, blue) })
}
pub fn change_window_mode(flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_ChangeWindowMode(flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_use_charcode_format(char_code_format: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetUseCharCodeFormat(char_code_format) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn load_graph(file_name: &str) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_LoadGraph(file_name) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn draw_graph(x: i32, y: i32, gr_handle: i32, trans_flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawGraph(x, y, gr_handle, trans_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn draw_extend_graph(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    gr_handle: i32,
    trans_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawExtendGraph(x1, y1, x2, y2, gr_handle, trans_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

//３Ｄ図形描画関係関数
pub fn draw_line_3d(pos1: VECTOR, pos2: VECTOR, color: u32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawLine3D(pos1, pos2, color) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn draw_triangle_3d(
    pos1: VECTOR,
    pos2: VECTOR,
    pos3: VECTOR,
    color: u32,
    fill_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawTriangle3D(pos1, pos2, pos3, color, fill_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

pub fn draw_sphere_3d(
    center_pos: VECTOR,
    r: f32,
    div_num: i32,
    dif_color: u32,
    spc_color: u32,
    fill_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawSphere3D(center_pos, r, div_num, spc_color, spc_color, fill_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

pub fn draw_capsule_3d(
    pos1: VECTOR,
    pos2: VECTOR,
    r: f32,
    div_num: i32,
    dif_color: u32,
    spc_color: u32,
    fill_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawCapsule3D(pos1, pos2, r, div_num, dif_color, spc_color, fill_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

pub fn draw_cone_3d(
    top_pos: VECTOR,
    bottom_pos: VECTOR,
    r: f32,
    div_num: i32,
    dif_color: u32,
    spc_color: u32,
    fill_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe {
        dx_DrawCone3D(
            top_pos, bottom_pos, r, div_num, dif_color, spc_color, fill_flag,
        )
    };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

pub fn draw_bill_board_3d(
    pos: VECTOR,
    cx: f32,
    cy: f32,
    size: f32,
    angle: f32,
    gr_handle: i32,
    trans_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawBillboard3D(pos, cx, cy, size, angle, gr_handle, trans_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn draw_polygon_indexed_3d(
    vertex: *mut VERTEX3D,
    vertex_num: i32,
    indices: *mut u16,
    polygon_num: i32,
    gr_handle: i32,
    trans_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe {
        dx_DrawPolygonIndexed3D(
            vertex,
            vertex_num,
            indices,
            polygon_num,
            gr_handle,
            trans_flag,
        )
    };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn dx_draw_polygon_3d(
    vertex: *mut VERTEX3D,
    polygon_num: i32,
    gr_handle: i32,
    trans_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawPolygon3D(vertex, polygon_num, gr_handle, trans_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

pub fn set_material_use_vert_dif_color(use_flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetMaterialUseVertDifColor(use_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_material_use_vert_spc_color(use_flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetMaterialUseVertSpcColor(use_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_material_param(material: MATERIALPARAM) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetMaterialParam(material) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

// Zバッファを使うかどうかのフラグ
pub fn set_use_zbuffer_3d(flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetUseZBuffer3D(flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
// Zバッファへの書き込みするかどうかのフラグ
pub fn set_write_zbuffer_3d(flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetWriteZBuffer3D(flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_use_back_culling(flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetUseBackCulling(flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_texture_address_mode_uv(mode_u: i32, mode_v: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetTextureAddressModeUV(mode_u, mode_v) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_fog_enable(flag: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetFogEnable(flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_fog_color(red: i32, green: i32, blue: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetFogColor(red, green, blue) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn set_fog_start_end(start: f32, end: f32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetFogStartEnd(start, end) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn get_colorf(red: f32, green: f32, blue: f32, alpha: f32) -> Option<COLOR_F> {
    if red < 0.0 || green < 0.0 || blue < 0.0 || red > 255.0 || green > 255.0 || blue > 255.0 {
        return None;
    }
    let res = unsafe { dx_GetColorF(red, green, blue, alpha) };
    return Some(res);
}
pub fn get_coloru8(red: i32, green: i32, blue: i32, alpha: i32) -> Option<COLOR_U8> {
    if red < 0 || green < 0 || blue < 0 || red > 255 || green > 255 || blue > 255 {
        return None;
    }

    let res = unsafe { dx_GetColorU8(red, green, blue, alpha) };

    return Some(res);
}

// 図形描画関数

/// 線を描画
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawLine(x1, y1, x2, y2, color) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// 線を描画(アンチエイリアス効果付き)
pub fn draw_line_aa(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawLineAA(x1, y1, x2, y2, color) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// 四角を描画
pub fn draw_box(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: Color,
    fill_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawBox(x1, y1, x2, y2, color, fill_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// 四角を描画(アンチエイリアス効果付き)
pub fn draw_box_aa(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: Color,
    fill_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawBoxAA(x1, y1, x2, y2, color, fill_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

pub fn draw_pixel(x: i32, y: i32, color: u32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DrawPixel(x, y, color) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// 指定点の色を取得
pub fn get_pixel(x: i32, y: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_GetPixel(x, y) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

/// ミリ秒単位の精度を持つカウンタの現在値を得る
pub fn get_now_count() -> i32 {
    let res = unsafe { dx_GetNowCount() };
    return res;
}
/// GetNowCountの高精度バージョン
pub fn get_now_hiperformance_count() -> i64 {
    let res = unsafe { dx_GetNowHiPerformanceCount() };
    return res;
}

/// 指定の時間だけ処理をとめる
pub fn wait_timer(wait_time: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_WaitTimer(wait_time) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// ディスプレイの垂直同期信号を指定回数待つ
//pub fn dx_WaitVSync(async_num:i32) -> i32;
/// キーの入力待ち
pub fn wait_key() -> i32 {
    let res = unsafe { dx_WaitKey() };
    return res;
}

// 音楽再生関数
pub fn load_music_mem(file_name: &str) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_LoadMusicMem(file_name) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn play_music_mem(
    music_handle: i32,
    play_type: i32,
    top_position_flag: i32,
) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_PlayMusicMem(music_handle, play_type, top_position_flag) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn delete_music_mem(music_handle: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_DeleteMusicMem(music_handle) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルを演奏(再生)する
pub fn play_music(file_name: &str, play_type: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_PlayMusic(file_name, play_type) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルが演奏(再生)中かの情報を取得する
pub fn check_music() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_CheckMusic() };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)停止
pub fn stop_music() -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_StopMusic() };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)の音量を設定する
pub fn set_volume_music(volume: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetVolumeMusic(volume) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

/// 現在時刻を取得する
pub fn get_date_time(data_buf: &mut DATEDATA) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_GetDateTime(data_buf) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}

/// 乱数を取得する
pub fn get_rand(rand_max: i32) -> i32 {
    let res = unsafe { dx_GetRand(rand_max) };
    return res;
}
/// 乱数の初期値を設定する
pub fn s_rand(seed: i32) -> i32 {
    let res = unsafe { dx_SRand(seed) };
    return res;
}

pub fn set_main_window_text(window_text: &str) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_SetMainWindowText(window_text) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn mv1_load_model(file_name: &str) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_MV1LoadModel(file_name) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn mv1_delete_model(m_handle: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_MV1DeleteModel(m_handle) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
pub fn mv1_draw_model(m_handle: i32) -> Result<i32, DxError<i32>> {
    let res = unsafe { dx_MV1DrawModel(m_handle) };
    if res == -1 {
        return Err(DxError::new("", res));
    } else {
        return Ok(res);
    }
}
