use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Display)]
pub enum Period {
    Day,
    Week,
    Month,
}