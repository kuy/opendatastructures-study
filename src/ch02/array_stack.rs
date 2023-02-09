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
        ArrayStack { a: Box::new([T::default(); 16]), n: 0 }
    }
}

impl<T> List<T> for ArrayStack<T> where T: Default + Copy {
    fn size(&self) -> usize {
        return self.n;
    }

    fn get(&self, i: usize) -> T {
        return self.a[i];
    }

    fn set(&mut self, i: usize, x: T) -> T {
        let old = self.a[i];
        self.a[i] = x;
        return old;
    }

    fn add(&mut self, i: usize, x: T) {
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
}
