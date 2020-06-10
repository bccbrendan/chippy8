


pub struct Input {
}


impl Input {
    pub fn new() -> Self {
        Input {
        }
    }

    // TODO 
    pub fn poll(&self) -> Option<[bool; 16]> {
        Some([false; 16])
    }
}
