use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time {
    // Internally, time is stored as seconds. SI baby!
    seconds: f64,
}

impl Time {
    pub const fn zero() -> Self {
        Self { seconds: 0.0 }
    }
}

impl Time {
    pub const fn from_seconds(seconds: f64) -> Self {
        Self { seconds }
    }

    pub fn from_milliseconds(millis: f64) -> Self {
        Self::from_seconds(millis / 1e3)
    }

    pub fn from_microseconds(micros: f64) -> Self {
        Self::from_seconds(micros / 1e6)
    }

    pub fn from_nanoseconds(nanos: f64) -> Self {
        Self::from_seconds(nanos / 1e9)
    }

    pub fn from_femtoseconds(femtos: f64) -> Self {
        Self::from_seconds(femtos / 1e12)
    }

    pub fn from_picoseconds(picos: f64) -> Self {
        Self::from_seconds(picos / 1e15)
    }
}

impl Time {
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    pub fn milliseconds(&self) -> f64 {
        self.seconds * 1e3
    }

    pub fn microseconds(&self) -> f64 {
        self.seconds * 1e6
    }

    pub fn nanoseconds(&self) -> f64 {
        self.seconds * 1e9
    }

    pub fn femtoseconds(&self) -> f64 {
        self.seconds * 1e12
    }

    pub fn picoseconds(&self) -> f64 {
        self.seconds * 1e15
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_seconds(self.seconds + rhs.seconds)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.seconds += rhs.seconds
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_seconds(self.seconds - rhs.seconds)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.seconds -= rhs.seconds
    }
}

impl Mul for Time {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_seconds(self.seconds * rhs.seconds)
    }
}

impl MulAssign for Time {
    fn mul_assign(&mut self, rhs: Self) {
        self.seconds *= rhs.seconds
    }
}

impl Div for Time {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_seconds(self.seconds / rhs.seconds)
    }
}

impl DivAssign for Time {
    fn div_assign(&mut self, rhs: Self) {
        self.seconds /= rhs.seconds
    }
}
