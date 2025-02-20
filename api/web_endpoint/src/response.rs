use serde_derive::Serialize;

#[derive(Serialize)]
pub struct WebResponse<Data> {
    pub data: Data,
    pub success: bool,
}

impl<Data> WebResponse<Data> {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            success: true,
        }
    }
}