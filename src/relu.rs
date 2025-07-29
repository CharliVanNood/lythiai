pub struct RELU {
    size: usize
}
impl RELU {
    pub fn new(size: usize) -> Self {
        Self {
            size
        }
    }

    pub fn info(&self) {
        println!("info")
    }
}