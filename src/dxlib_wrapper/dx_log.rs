use crate::dx_error::*;
pub struct DxLogData {}
impl DxLogData {}

struct DxLog {}
impl DxLog {
    fn new() -> DxLog {
        return DxLog {};
    }
    fn add(&mut self, text: String) -> Result<&mut DxLog, String> {
        return Ok(self);
    }
    fn add_error(&mut self, text: String, err_type: &DxErrorType) -> Result<&mut DxLog, String> {
        return Ok(self);
    }
}
