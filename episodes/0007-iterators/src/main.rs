struct Iter<'a, T: 'a> {
    vec: &'a [T],
    index: usize,
    f: Box<Fn(T) -> bool>
}

impl<'a, T: 'a> Iter<'a, T> {
    fn new<F>(vec: &'a [T], f: F) -> Self
    where
        F: Fn(T) -> bool + 'static
    {
        Self {
            vec,
            index: 0,
            f: Box::new(f)
        }
    }
}

impl<'a, T: 'a + Copy> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.vec.len() - 1 {
            let prev = self.vec[self.index];
            self.index += 1;
            if (self.f)(prev) {
                return Some(self.vec[self.index]);
            }
        }
        None
    }
}

fn main() {
    let vec = vec![4u8, 5, 6, 7, 1, 9, 0, 3];
    let mut iter = Iter::new(&vec, |x| x % 3 == 0);

    for num in iter {
        print!("{}, ", num);
    }
    println!();
}
