trait Queue<T> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> Option<T>;
    fn size(&self) -> usize;
}

struct ArrayQueue<T> where T: Default + Copy {
    a: Box<[T]>,
    n: usize,
    j: usize
}

impl<T> ArrayQueue<T> where T: Default + Copy + std::fmt::Debug {
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    pub fn with_capacity(n: usize) -> Self {
        let a = vec![Default::default(); n].into_boxed_slice();
        ArrayQueue { a, n: 0, j: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.a.len()
    }

    fn resize(&mut self) {
        let len = usize::max(self.n * 2, 4);
        if len == self.a.len() {
            return; // NOOP
        }
        let mut a = vec![Default::default(); len].into_boxed_slice();

        let mut k = 0;
        for i in self.j..(self.j + self.n) {
            let i = i % self.a.len();
            a[k] = self.a[i];
            k += 1;
        }
        self.a = a;
        self.j = 0;
    }

    fn debug(&self, tag: &str) {
        print!("{}: ", tag);
        for i in 0..(self.a.len() - 1) {
            print!("{:?}, ", self.a[i]);
        }
        println!("[END] j={}, n={}, a.len={}", self.j, self.n, self.a.len());
    }
}

impl<T> Queue<T> for ArrayQueue<T> where T: Default + Copy + std::fmt::Debug {
    fn add(&mut self, x: T) {
        if self.a.len() < self.n + 1 {
            self.resize();
        }
        self.a[(self.j + self.n) % self.a.len()] = x;
        self.n += 1;
    }

    fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let x = self.a[self.j];
        self.j = (self.j + 1) % self.a.len();
        self.n -= 1;

        if (self.n * 3) < self.a.len() {
            self.resize();
        }

        Some(x)
    }

    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity() {
        let a: ArrayQueue<i32> = ArrayQueue::new();
        assert_eq!(a.capacity(), 16);

        let a: ArrayQueue<i32> = ArrayQueue::with_capacity(8);
        assert_eq!(a.capacity(), 8);
    }

    #[test]
    fn test_add_remove() {
        let mut a = ArrayQueue::new();
        a.add("a");
        a.add("b");
        a.add("c");

        assert_eq!(a.size(), 3);
        assert_eq!(a.remove(), Some("a"));
        assert_eq!(a.size(), 2);

        a.add("d");
        assert_eq!(a.size(), 3);

        assert_eq!(a.remove(), Some("b"));
        assert_eq!(a.remove(), Some("c"));
        assert_eq!(a.remove(), Some("d"));
        assert_eq!(a.remove(), None);
        assert_eq!(a.size(), 0);
    }

    #[test]
    fn test_resize() {
        let mut a = ArrayQueue::with_capacity(4);
        a.add("a");
        a.add("b");
        a.add("c");
        a.add("d");

        assert_eq!(a.size(), 4);
        assert_eq!(a.capacity(), 4);

        a.add("e");
        assert_eq!(a.size(), 5);
        assert_eq!(a.capacity(), 8);

        a.remove();
        a.remove();
        a.remove();

        assert_eq!(a.size(), 2);
        assert_eq!(a.capacity(), 4);
    }
}
