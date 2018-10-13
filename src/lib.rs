use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// A circular buffer.
pub struct CircleBuffer<T> where T: Clone {
    capacity: usize,
    vec: Vec<T>,
    cur_start: usize,
}

impl<T> CircleBuffer<T> where T: Clone {
    /// Creates a new empty `CircleBuffer<T>` with capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut cbuf: CircleBuffer<i32> = CircleBuffer::with_capacity(3);
    /// ```
    pub fn with_capacity(capacity: usize) -> CircleBuffer<T> {
        CircleBuffer {
            capacity: capacity,
            vec: Vec::with_capacity(capacity * 2 - 1),
            cur_start: 0,
        }
    }

    /// Returns the capacity of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut cbuf: CircleBuffer<i32> = CircleBuffer::with_capacity(3);
    /// assert_eq!(3, cbuf.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the current number of elements in the buffer.
    ///
    /// # Examples
    /// 
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut cbuf: CircleBuffer<i32> = CircleBuffer::with_capacity(10);
    /// cbuf.push(1);
    /// cbuf.push(1);
    /// cbuf.push(1);
    ///
    /// assert_eq!(cbuf.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        let len = self.vec.len();
        if len > self.capacity {
            self.capacity
        }else{
            len
        }
    }

    /// Returns true if the buffer contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut cbuf: CircleBuffer<i32> = CircleBuffer::with_capacity(3);
    /// assert!(cbuf.is_empty());
    ///
    /// cbuf.push(1);
    /// assert!(!cbuf.is_empty())
    /// ```
    pub fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }

    /// Pushes a new element into the buffer.
    /// Once the capacity is reached, pushing new items will overwrite old ones.
    ///
    /// # Examples
    /// 
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut cbuf: CircleBuffer<i32> = CircleBuffer::with_capacity(3);
    /// cbuf.push(1);
    /// cbuf.push(2);
    /// cbuf.push(3);
    /// cbuf.push(4);
    ///
    /// assert_eq!(cbuf.len(), 3);
    ///
    /// assert_eq!(cbuf[0], 2);
    /// assert_eq!(cbuf[1], 3);
    /// assert_eq!(cbuf[2], 4);
    ///
    /// let mut sum = 0;
    /// for x in cbuf.as_slice() {
    ///     sum += x;
    /// }
    /// assert_eq!(sum, 9);
    /// ```
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

    /// Extracts a slice containing the entire buffer.
    pub fn as_slice(&self) -> &[T] {
        if self.vec.len() < self.capacity {
            self.vec.as_slice()
        }else{
            &self.vec.as_slice()[self.cur_start..self.cur_start + self.capacity]
        }
    }

    /// Extracts a mutable slice of the entire buffer.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.vec.len() < self.capacity {
            self.vec.as_mut_slice()
        }else{
            &mut self.vec.as_mut_slice()[self.cur_start..self.cur_start + self.capacity]
        }
    }

    /// Returns an iterator over the buffer's contents.
    /// The iterator goes from the most recently pushed items to the oldest ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut buffer = CircleBuffer::with_capacity(3);
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    /// let add1: Vec<i32> = buffer.iter().map(|x| x + 1).collect();
    /// assert_eq!(add1, vec![2, 3, 4]);
    ///
    /// buffer.push(4);
    /// buffer.push(5);
    /// let add2: Vec<i32> = buffer.iter().map(|x| x + 2).collect();
    /// assert_eq!(add2, vec![5, 6, 7]);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.vec[self.cur_start..self.cur_start + self.len()].iter()
    }

    /// Returns a mutable iterator over the buffer's contents.
    /// The iterator goes from the most recently pushed items to the oldest ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circle_buffer::CircleBuffer;
    ///
    /// let mut buffer = CircleBuffer::with_capacity(3);
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    /// for x in buffer.iter_mut() {
    ///     *x += 1;
    /// }
    /// assert_eq!(buffer.as_slice(), &[2, 3, 4]);
    ///
    /// buffer.push(4);
    /// buffer.push(5);
    /// for x in buffer.iter_mut() {
    ///     *x += 2;
    /// }
    /// assert_eq!(buffer.as_slice(), &[6, 6, 7]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let end_index = self.cur_start + self.len();
        self.vec[self.cur_start..end_index].iter_mut()
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

    #[test]
    fn test_iter() {
        let mut buffer = CircleBuffer::with_capacity(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        let add1: Vec<i32> = buffer.iter().map(|x| x + 1).collect();
        assert_eq!(add1, vec![2, 3, 4]);

        buffer.push(4);
        buffer.push(5);
        let add2: Vec<i32> = buffer.iter().map(|x| x + 2).collect();
        assert_eq!(add2, vec![5, 6, 7]);
    }

    #[test]
    fn test_iter_mut() {
        let mut buffer = CircleBuffer::with_capacity(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        for x in buffer.iter_mut() {
            *x += 1;
        }
        assert_eq!(buffer.as_slice(), &[2, 3, 4]);

        buffer.push(4);
        buffer.push(5);
        for x in buffer.iter_mut() {
            *x += 2;
        }
        assert_eq!(buffer.as_slice(), &[6, 6, 7]);
    }
}
