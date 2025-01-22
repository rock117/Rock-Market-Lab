use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Range<T: PartialOrd> {
    pub min: T,
    pub max: T,
}

impl<T: PartialOrd> Range<T> {
    pub fn new(min: T, max: T) -> Self{
        Self {min, max}
    }
}