use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LamportTimestamp(u64);

impl LamportTimestamp {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn tick(&mut self) -> Self {
        self.0 += 1;
        *self
    }

    pub fn merge(&mut self, other: Self) -> Self {
        self.advance_to(other);
        self.tick()
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub fn advance_to(&mut self, timestamp: Self) {
        self.0 = self.0.max(timestamp.0);
    }
}

impl fmt::Display for LamportTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
