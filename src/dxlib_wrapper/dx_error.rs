use std::error;
use std::fmt;

#[derive(Debug)]
pub struct DxResouceError {
    message: String,
    res: i32,
}
#[derive(Debug)]
pub struct DxFileError {
    message: String,
    res: i32,
}
impl DxFileError {}
#[derive(Debug)]
pub struct DxError {
    message: String,
    res: i32,
}
impl DxError {
    pub fn get_res(&self) -> i32 {
        self.res
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
    pub fn new(message: &str, res: i32) -> DxError {
        DxError {
            message: message.to_string(),
            res,
        }
    }
}
impl fmt::Display for DxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DxError: {} {}", self.message, self.res)
    }
}
impl error::Error for DxError {}

impl DxResouceError {
    pub fn get_res(&self) -> i32 {
        self.res
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
    pub fn new(message: &str, res: i32) -> DxResouceError {
        DxResouceError {
            message: message.to_string(),
            res,
        }
    }
}
impl fmt::Display for DxResouceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DxResouceError: {} {}", self.message, self.res)
    }
}
impl error::Error for DxResouceError {}

impl DxFileError {
    pub fn get_res(&self) -> i32 {
        self.res
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn new(message: &str, res: i32) -> DxFileError {
        DxFileError {
            message: message.to_string(),
            res,
        }
    }
}
impl fmt::Display for DxFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DxFileError: {} {}", self.message, self.res)
    }
}
impl error::Error for DxFileError {}

#[derive(Debug)]
pub enum DxErrorType {
    ResouceE(DxResouceError),
    FileE(DxFileError),
    DxLibE(DxError),
}
pub fn test_error() -> Result<(), DxErrorType> {
    Err(DxErrorType::ResouceE(DxResouceError::new(
        "画像の読み込みに失敗したよ!",
        0,
    )))
}
