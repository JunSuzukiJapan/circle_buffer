use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct CircleBuffer<T> where T: Clone {
    capacity: usize,
    vec: Vec<T>,
    cur_start: usize,
}

impl<T> CircleBuffer<T> where T: Clone {
    pub fn with_capacity(capacity: usize) -> CircleBuffer<T> {
        CircleBuffer {
            capacity: capacity,
            vec: Vec::with_capacity(capacity * 2 - 1),
            cur_start: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        let len = self.vec.len();
        if len > self.capacity {
            self.capacity
        }else{
            len
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }

    pub fn push(&mut self, value: T){
        if self.vec.len() < self.capacity {
            self.vec.push(value);

        } else if self.vec.len() < self.capacity * 2 - 1 {
            let v = value.clone();
            self.vec.push(value);
            self.vec[self.cur_start] = v;

            self.cur_start += 1;

        } else {
            let v = value.clone();

            let index = self.cur_start + self.capacity;
            if index < self.capacity * 2 - 1 {
                self.vec[index] = value;
            }

            self.vec[self.cur_start] = v;

            self.cur_start += 1;
            if self.cur_start >= self.capacity {
                self.cur_start = 0;
            }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        if self.vec.len() < self.capacity {
            self.vec.as_slice()
        }else{
            &self.vec.as_slice()[self.cur_start..self.cur_start + self.capacity]
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.vec.len() < self.capacity {
            self.vec.as_mut_slice()
        }else{
            &mut self.vec.as_mut_slice()[self.cur_start..self.cur_start + self.capacity]
        }
    }
}

impl<T> Index<usize> for CircleBuffer<T> where T: Clone {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        assert!(index < self.vec.len());
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<usize> for CircleBuffer<T> where T: Clone {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.vec.len());
        &mut self.as_mut_slice()[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut buffer = CircleBuffer::with_capacity(3);
        buffer.push(1);
        assert_eq!(&[1], buffer.as_slice());
        buffer.push(2);
        assert_eq!(&[1, 2], buffer.as_slice());
        buffer.push(3);
        assert_eq!(&[1, 2, 3], buffer.as_slice());
        buffer.push(4);
        assert_eq!(&[2, 3, 4], buffer.as_slice());
        buffer.push(5);
        assert_eq!(&[3, 4, 5], buffer.as_slice());
        buffer.push(6);
        assert_eq!(&[4, 5, 6], buffer.as_slice());
        buffer.push(7);
        assert_eq!(&[5, 6, 7], buffer.as_slice());
        buffer.push(8);
        assert_eq!(&[6, 7, 8], buffer.as_slice());
        buffer.push(9);
        assert_eq!(&[7, 8, 9], buffer.as_slice());
        buffer.push(10);
        assert_eq!(&[8, 9, 10], buffer.as_slice());
        buffer.push(11);
        assert_eq!(&[9, 10, 11], buffer.as_slice());
        buffer.push(12);
        assert_eq!(&[10, 11, 12], buffer.as_slice());
        buffer.push(13);
        assert_eq!(&[11, 12, 13], buffer.as_slice());
        buffer.push(14);
        assert_eq!(&[12, 13, 14], buffer.as_slice());
        buffer.push(15);
        assert_eq!(&[13, 14, 15], buffer.as_slice());
        buffer.push(16);
        assert_eq!(&[14, 15, 16], buffer.as_slice());
        buffer.push(17);
        assert_eq!(&[15, 16, 17], buffer.as_slice());
    }

    #[test]
    fn test_capacity() {
        let mut buffer = CircleBuffer::with_capacity(3);
        assert_eq!(3, buffer.capacity());
        buffer.push(1);
        assert_eq!(3, buffer.capacity());
        buffer.push(2);
        assert_eq!(3, buffer.capacity());
        buffer.push(3);
        assert_eq!(3, buffer.capacity());
        buffer.push(4);
        assert_eq!(3, buffer.capacity());
        buffer.push(5);
        assert_eq!(3, buffer.capacity());
    }

    #[test]
    fn test_len() {
        let mut buffer = CircleBuffer::with_capacity(3);
        assert_eq!(0, buffer.len());
        buffer.push(1);
        assert_eq!(1, buffer.len());
        buffer.push(2);
        assert_eq!(2, buffer.len());
        buffer.push(3);
        assert_eq!(3, buffer.len());
        buffer.push(4);
        assert_eq!(3, buffer.len());
        buffer.push(5);
        assert_eq!(3, buffer.len());
    }

    #[test]
    fn test_is_empty() {
        let mut buffer = CircleBuffer::with_capacity(3);
        assert_eq!(true, buffer.is_empty());
        buffer.push(1);
        assert_eq!(false, buffer.is_empty());
    }

    #[test]
    fn test_index() {
        let mut buffer = CircleBuffer::with_capacity(3);
        buffer.push(1);
        assert_eq!(1, buffer[0]);
        buffer.push(2);
        assert_eq!(1, buffer[0]);
        assert_eq!(2, buffer[1]);
        buffer.push(3);
        assert_eq!(1, buffer[0]);
        assert_eq!(2, buffer[1]);
        assert_eq!(3, buffer[2]);
        buffer.push(4);
        assert_eq!(2, buffer[0]);
        assert_eq!(3, buffer[1]);
        assert_eq!(4, buffer[2]);
        buffer.push(5);
        assert_eq!(3, buffer[0]);
        assert_eq!(4, buffer[1]);
        assert_eq!(5, buffer[2]);
    }

    #[test]
    fn test_index_mut() {
        let mut buffer = CircleBuffer::with_capacity(3);
        buffer.push(1);
        assert_eq!(1, buffer[0]);
        buffer.push(2);
        assert_eq!(1, buffer[0]);
        assert_eq!(2, buffer[1]);
        buffer.push(3);
        assert_eq!(1, buffer[0]);
        assert_eq!(2, buffer[1]);
        assert_eq!(3, buffer[2]);
        buffer.push(4);
        assert_eq!(2, buffer[0]);
        assert_eq!(3, buffer[1]);
        assert_eq!(4, buffer[2]);

        buffer[0] = 1;
        assert_eq!(1, buffer[0]);
        assert_eq!(3, buffer[1]);
        assert_eq!(4, buffer[2]);
    }
}
