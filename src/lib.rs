
use std::ops::Add;

#[cfg(test)]
mod tests {
    #[test]
    fn check_add() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum F2 {
    False,
    True,
}

impl F2 {
    #[doc = /**
     * new
     * convert a Boolean to Field F2
     */]
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
            return self;
        }

        F2::True
    }
}
