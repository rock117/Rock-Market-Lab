use derive_more::Display;

#[derive(Debug, Copy, Clone, Display)]
pub enum ThsIndexType {
    N, // -概念指数
    I, // -行业指数
    R, // -地域指数
    S, // -同花顺特色指数
    ST, // -同花顺风格指数
    TH, // -同花顺主题指数
    BB, // -同花顺宽基指数
}

impl ThsIndexType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::N => "N",
            Self::I => "I",
            Self::R => "R",
            Self::S => "S",
            Self::ST => "ST",
            Self::TH => "TH",
            Self::BB => "BB",
        }
    }
}