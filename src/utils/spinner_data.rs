#[derive(Debug, Clone)]
pub struct SpinnerData {
    pub frames: Vec<&'static str>,
    pub interval: u16,
}
