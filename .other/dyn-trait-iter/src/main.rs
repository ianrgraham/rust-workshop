trait A {
    fn next_usize(&mut self) -> Option<usize>;
}

impl Iterator for dyn A {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.next_usize()
    }
}

struct B {
    idx: usize,
    data: Box<[usize]>
}

impl A for B {
    fn next_usize(&mut self) -> Option<usize> {
        if self.idx < self.data.len() {
            let data = self.data[self.idx];
            self.idx += 1;
            Some(data)
        }
        else {
            None
        }
    }
}

fn main() {
    let a = Box::new(B{idx: 0, data: Box::new([0, 1, 2, 3])});

    for i in a as Box<dyn A> {
        println!("{i}");
    }
}
