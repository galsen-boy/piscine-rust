	#[derive(Copy, Clone, Debug)]
pub struct Collatz {
    pub v: u64,
}
impl Iterator for Collatz {
    type Item = Collatz;
    fn next(&mut self) -> Option<Self::Item> {
        let current_value = *self;
        if self.v <= 1 {
            return None;
        }
        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }
        Some(current_value)
    }
}
impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}
pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
    
}

