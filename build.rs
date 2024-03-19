

use std::env;
use std::path::PathBuf;

fn main()->Result<(),Box<dyn std::error::Error>> {
    let env_name = "DXLIB_DLL";
    let dir = env::var(env_name);
    if let Some(err) = dir.err(){
          println!("{:}",err);
          println!("環境変数{:}が設定されていません。",env_name);
    }
    Ok(())
}
