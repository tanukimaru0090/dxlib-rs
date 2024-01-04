use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DxError<T> {
    message: String,
    res: T,
}
impl<T> DxError<T> {
    pub fn get_res(self) -> T {
        self.res
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
    pub fn new(message: &str, res: T) -> DxError<T> {
        DxError {
            message: message.to_string(),
            res,
        }
    }
}
impl fmt::Display for DxError<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DxError: {} {}", self.message, self.res)
    }
}
impl error::Error for DxError<i32> {}
