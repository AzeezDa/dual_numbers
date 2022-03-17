use std::ops::*;

/// # `Dual`
/// Struct that represents the Dual Numbers of the form a + bɛ, where ɛ * ɛ = 0.
/// 
/// Read more at: https://en.wikipedia.org/wiki/Dual_number
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dual<T> {
    pub a: T,
    pub b: T
}

impl<T: Neg<Output = T> + Copy> Dual<T> {
    /// # `new`
    /// Using the given numerical values a, b, return the `Dual` representing a + bɛ
    pub fn new(a: T, b: T) -> Self {
        Dual { a, b }
    }

    /// # `conjugate`
    /// Returns the conjugate a - bɛ of the `Dual`
    pub fn conjugate(&self) -> Self {
        Dual { a: self.a, b: -self.b }
    }
}

// #### OPERATORS ####

impl<T: Add<Output = T> + Copy> Add for Dual<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Dual<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a + other.a,
            b: self.b + other.b
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Dual<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            a: self.a - other.a,
            b: self.b - other.b
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Dual<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a - other.a,
            b: self.b - other.b
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul for Dual<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a,
            b: self.a * other.b + self.b * other.a
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> MulAssign for Dual<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a * other.a,
            b: self.a * other.b + self.b * other.a
        }
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy> Div for Dual<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            a: self.a / other.a,
            b: (self.b * other.a - self.a * other.b) / (other.a * other.a)
        }
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy> DivAssign for Dual<T> {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a / other.a,
            b: (self.b * other.a - self.a * other.b) / (other.a * other.a)
        }
    }
}