use std::env;
use std::fs;
fn main(){
       let current =  env::current_dir();
       let dxlib_dll = format!("{:}\\DxLib_x64.dll",current.unwrap().to_str().unwrap());
       match fs::metadata(dxlib_dll.clone()){
            Ok(_) => {
            }
            Err(err) => {
                println!("{:?}",err);
                panic!("{:?}が見つかりませんでした",dxlib_dll);
            }
       }
}
