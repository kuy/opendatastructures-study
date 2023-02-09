trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, x: T) -> T;
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize) -> T;
}

struct ArrayStack<T> where T: Default + Copy {
    a: Box<[T]>,
    n: usize
}

impl<T> ArrayStack<T> where T: Default + Copy {
    pub fn new() -> Self {
        ArrayStack { a: Box::new([T::default(); 4]), n: 0 }
    }

    pub fn resize(&mut self) {
        let new_cap = usize::max(self.n * 2, 4);
        if new_cap == self.capacity() {
            return; // NOOP
        }
        let mut new_a = vec![Default::default(); new_cap].into_boxed_slice();
        for i in 0..(self.n - 1) {
            new_a[i] = self.a[i];
        }
        self.a = new_a;
    }

    pub fn capacity(&self) -> usize {
        self.a.len()
    }
}

impl<T> List<T> for ArrayStack<T> where T: Default + Copy {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> T {
        self.a[i]
    }

    fn set(&mut self, i: usize, x: T) -> T {
        let old = self.a[i];
        self.a[i] = x;
        old
    }

    fn add(&mut self, i: usize, x: T) {
        if self.capacity() < self.n + 1 {
            self.resize();
        }

        let mut j = self.n;
        while j > i {
            self.a[j] = self.a[j - 1];
            j -= 1;
        }
        self.a[i] = x;
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> T {
        let ret = self.a[i];
        self.n -= 1;

        let mut j = i;
        while j < self.n {
            self.a[j] = self.a[j + 1];
            j += 1;
        }

        if (self.n * 3) < self.capacity() {
            self.resize();
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let mut a = ArrayStack::new();
        a.add(0, 1);
        a.add(1, 2);

        assert_eq!(a.size(), 2);
    }

    #[test]
    fn test_get() {
        let mut a = ArrayStack::new();
        a.set(0, 1);

        let actual = a.get(0);
        let expected = 1;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set() {
        let mut a = ArrayStack::new();
        a.set(0, 1);

        let old = a.set(0, 2);

        assert_eq!(old, 1);
        assert_eq!(a.get(0), 2);
    }

    #[test]
    fn test_add() {
        let mut a = ArrayStack::new();
        a.add(0, 1);
        a.add(1, 2);
        a.add(1, 3);

        assert_eq!(a.get(0), 1);
        assert_eq!(a.get(1), 3);
        assert_eq!(a.get(2), 2);
    }

    #[test]
    fn test_remove() {
        let mut a = ArrayStack::new();
        a.add(0, 1);
        a.add(1, 2);
        a.remove(0);

        assert_eq!(a.get(0), 2);
        assert_eq!(a.size(), 1);
    }

    #[test]
    fn test_resize() {
        let mut a = ArrayStack::new();
        a.add(0, 1);
        a.add(1, 2);
        a.add(2, 3);
        a.add(3, 4);

        assert_eq!(a.capacity(), 4);

        a.add(4, 5);

        assert_eq!(a.capacity(), 8);

        a.add(5, 6);

        assert_eq!(a.capacity(), 8);

        a.remove(0);
        a.remove(0);
        a.remove(0);
        a.remove(0);

        assert_eq!(a.capacity(), 4);
    }
}
