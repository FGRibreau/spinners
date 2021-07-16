#[derive(Debug, Clone)]
pub struct SpinnerData {
    pub name: String,
    pub frames: Vec<&'static str>,
    pub interval: u16,
}

