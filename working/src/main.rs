
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

    fn nuke_via_haiku(self) {
        print!("life of a struct\nmethods respect borrow checker\nbye bye bye")
    }
}

fn main() {
    let mut x = Complex64::new(1.0, 0.0);

    println!("The norm of {:?} is {}", x, x.norm());

    println!("The norm of {} is {}", x, x.norm());
}

impl std::fmt::Display for Complex64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let sign = if self.im >= 0.0 { "+" } else { "-" };
        write!{f, "{:.1}{}{:.1}i", self.re, sign, self.im.abs()}?;
        Ok(())
    }
}