use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2(pub f64, pub f64);

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Vec2(0., 0.);

        for item in iter {
            sum.0 += item.0;
            sum.1 += item.1;
        }

        sum
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl Vec2 {
    pub fn mag(self) -> f64 {
        self.mag_sq().sqrt()
    }

    pub fn mag_sq(self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn norm(self) -> Vec2 {
        let mag = self.mag();
        if mag == 0. {
            Vec2(0., 0.)
        } else {
            Vec2(self.0 / mag, self.1 / mag)
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.0, self.1))
    }
}
