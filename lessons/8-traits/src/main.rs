use std::ops;

#[derive(Debug)]
struct Complex64 {
    re: f32,
    im: f32,
}

impl Complex64 {
    fn new(re: f32, im: f32) -> Self {
        Complex64 { re, im }
    }

    fn norm(&self) -> f32 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    fn rotate(&mut self, ang: f32) {
        let sin = ang.sin();
        let cos = ang.cos();
        self.re = cos * self.re - sin * self.im;
        self.im = sin * self.re + cos * self.im;
    }
}

// This is an example implementation of the Display trait for our Complex64 struct.
impl std::fmt::Display for Complex64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let sign = if self.im >= 0.0 { "+" } else { "-" };

        write!{f, "{:.1}{}{:.1}i", self.re, sign, self.im.abs()}?;
        Ok(())
    }
}

impl ops::Add for Complex64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl ops::AddAssign for Complex64 {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl ops::Sub for Complex64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl ops::SubAssign for Complex64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl ops::Mul for Complex64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl ops::MulAssign for Complex64 {
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

impl ops::Neg for Complex64 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}


fn main() {
    let mut x = Complex64::new(0.0, 1.0);

    println!("x = {}", x);

    let x_squared = x * x;

    println!("x^2 = {}", x*x);
}
