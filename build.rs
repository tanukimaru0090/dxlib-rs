

use std::env;
use std::path::PathBuf;

fn main() {
    // DLLの名前を指定
    let dll_name = "DxLib_x64.dll";

    // 利用者側のプロジェクトのディレクトリパスを取得
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_path = PathBuf::from(project_dir);

    // DLLのパスを構築
    let dll_path = project_path.join(dll_name);

    // DLLの存在を確認
    if !dll_path.exists() {
        panic!("{} が見つかりません。利用者側のプロジェクトのディレクトリに {} を配置してください。", dll_name, dll_name);
    }
}
