use derive_more::Display;
use serde::Deserialize;
#[derive(Deserialize, Copy, Clone, Debug, Display)]
pub enum MainbzType {
    ///按产品
    P,
    /// D按地区
    D,
    /// I按行业
    I,
}
