#[derive(Debug)]
pub struct CircleBuffer<T> where T: Clone {
    size: usize,
    vec: Vec<T>,
    cur_start: usize,
}

impl<T> CircleBuffer<T> where T: Clone {
    pub fn with_capacity(capacity: usize) -> CircleBuffer<T> {
        CircleBuffer {
            size: capacity,
            vec: Vec::with_capacity(capacity * 2 - 1),
            cur_start: 0,
        }
    }

    pub fn push(&mut self, value: T){
        if self.vec.len() < self.size {
            self.vec.push(value);

        } else if self.vec.len() < self.size * 2 - 1 {
            let v = value.clone();
            self.vec.push(value);
            self.vec[self.cur_start] = v;

            self.cur_start += 1;

        } else {
            let v = value.clone();

            let index = self.cur_start + self.size;
            if index < self.size * 2 - 1 {
                self.vec[index] = value;
            }

            self.vec[self.cur_start] = v;

            self.cur_start += 1;
            if self.cur_start >= self.size {
                self.cur_start = 0;
            }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        if self.vec.len() < self.size {
            self.vec.as_slice()
        }else{
            &self.vec.as_slice()[self.cur_start..self.cur_start + self.size]
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.vec.len() < self.size {
            self.vec.as_mut_slice()
        }else{
            &mut self.vec.as_mut_slice()[self.cur_start..self.cur_start + self.size]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_buffer() {
        let mut queue = CircleBuffer::with_capacity(3);
        queue.push(1);
        assert_eq!(&[1], queue.as_slice());
        queue.push(2);
        assert_eq!(&[1, 2], queue.as_slice());
        queue.push(3);
        assert_eq!(&[1, 2, 3], queue.as_slice());
        queue.push(4);
        assert_eq!(&[2, 3, 4], queue.as_slice());
        queue.push(5);
        assert_eq!(&[3, 4, 5], queue.as_slice());
        queue.push(6);
        assert_eq!(&[4, 5, 6], queue.as_slice());
        queue.push(7);
        assert_eq!(&[5, 6, 7], queue.as_slice());
        queue.push(8);
        assert_eq!(&[6, 7, 8], queue.as_slice());
        queue.push(9);
        assert_eq!(&[7, 8, 9], queue.as_slice());
        queue.push(10);
        assert_eq!(&[8, 9, 10], queue.as_slice());
        queue.push(11);
        assert_eq!(&[9, 10, 11], queue.as_slice());
        queue.push(12);
        assert_eq!(&[10, 11, 12], queue.as_slice());
        queue.push(13);
        assert_eq!(&[11, 12, 13], queue.as_slice());
        queue.push(14);
        assert_eq!(&[12, 13, 14], queue.as_slice());
        queue.push(15);
        assert_eq!(&[13, 14, 15], queue.as_slice());
        queue.push(16);
        assert_eq!(&[14, 15, 16], queue.as_slice());
        queue.push(17);
        assert_eq!(&[15, 16, 17], queue.as_slice());
    }
}
