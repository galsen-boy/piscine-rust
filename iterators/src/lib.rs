#[derive(Copy, Clone, Debug)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            None
        } else {
            let current = self;
            self.v = if self.v % 2 == 0 {
                self.v / 2
            } else {
                self.v * 3 + 1
            };
            Some(current)
        }
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
