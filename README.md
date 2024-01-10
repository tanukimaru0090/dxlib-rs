# dxlib-rs。。
・ "tanukimaru0090/dxlib-ffi" をさらにラップしたもの。
・　まだ開発中なので安定性が低く、未定義な動作も多いです。
また、Live2Dの関数群はどういうわけか使えません。


#使い方

myproject/Cargo.toml
```toml
[dependencies]
dxlib-rs = { git = "https://github.com/tanukimaru0090/dxlib-rs", branch = "beta"}
```

src / main.rs。
```Rust
use dxlib_rs ::{
          *,
          dxlib::*,
};
fn main()->Result<(),Box<dyn std::error::Error>>{
 change_window_mode(TRUE)?;
 dxlib_init()?;
 wait_key();
 dxlib_end()?;
 Ok(())
}
```
utilsなどを使う場合、
myproject/Cargo.toml
```toml
[dependencies]
dxlib-rs = { git = "https://github.com/tanukimaru0090/dxlib-rs", branch = "beta",features = ["utils"]}
```

src / main.rs。
```Rust
use dxlib_rs ::{
          *,
          dxlib::*,
          utils::{KeyBoard,Fps}
};
fn main()->Result<(),Box<dyn std::error::Error>>{
 change_window_mode(TRUE)?;
 dxlib_init()?;
 wait_key();
 dxlib_end()?;
 Ok(())
}
``` 





最後に、 「DxLib_x64.dll」を実行時のディレクトリに置くことで実行できるはずです。
DxLib_x64.dllはDXライブラリの公式サイトのC#版DXライブラリをダウンロードすることで使うことができます。

## フォークさせてもらった、ライブラリ
elipmoc/rust_dxlib
## License
MIT License

Copyright (c) 2018 らいパン粉

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


## License
MIT
