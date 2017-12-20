
use std::ops::Add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add() {
        assert_eq!(F2::True + F2::True, F2::False);
        assert_eq!(F2::True + F2::False, F2::True);
        assert_eq!(F2::False + F2::True, F2::True);
        assert_eq!(F2::False + F2::False, F2::False);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum F2 {
    False,
    True,
}

impl F2 {
    pub fn new(value: bool) -> F2 {
        if value {
            return F2::True;
        }

        F2::False
    }
}

impl Add for F2 {
    type Output = Self;

    fn add(self, f: Self) -> Self {
        if self == f {
            return F2::False;
        }
        F2::True
    }
}
