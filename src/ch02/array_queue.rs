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

impl<T> ArrayQueue<T> where T: Default + Copy {
    pub fn new() -> Self {
        Self::with_capacity(4)
    }

    pub fn with_capacity(n: usize) -> Self {
        let a = vec![Default::default(); n].into_boxed_slice();
        ArrayQueue { a, n: 0, j: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.a.len()
    }
}

impl<T> Queue<T> for ArrayQueue<T> where T: Default + Copy {
    fn add(&mut self, x: T) {
        // TODO: resize
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
        // TODO: resize
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
        assert_eq!(a.capacity(), 4);

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
}
