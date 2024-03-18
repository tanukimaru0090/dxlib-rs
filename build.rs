use std::env;
use std::path::Path;

fn main() {
    // プロジェクトのディレクトリパスを取得
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_path = Path::new(&project_dir);

    // 必要なDLLの名前を指定
    let dxlib_dll_name = "DxLib_x64.dll";

    // DLLのパスを構築
    let dll_path = project_path.join(dxlib_dll_name);

    // DLLの存在を確認
    if !dll_path.exists() {
        panic!("{} が見つかりません。プロジェクトのディレクトリに {} を配置してください。", dxlib_dll_name, dxlib_dll_name);
    }
}
