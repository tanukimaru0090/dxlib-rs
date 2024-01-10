use crate::dx_error::*;
use crate::dx_font::*;
use crate::dx_image::*;
use crate::dx_music::*;
use crate::dx_sound::*;
// リソースの種類
pub enum DxResouceType {
    Font(DxFont),
    Sound(DxSound),
    Music(DxMusic),
    Image(DxImage),
}
// リソース関係
pub trait DxResouce {
    type Config; // 設定
    type GetVal; // ゲッターでの戻り値の型
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, DxErrorType>;
    fn get(&self) -> Result<Self::GetVal, DxErrorType>;
    fn delete(&mut self) -> Result<&mut Self, DxErrorType>;
}
// リソースの管理
pub trait DxResouceMgr<T: DxResouce> {
    type MgrObj; // 管理対象のオブジェクト
}
