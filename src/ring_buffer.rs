use std::fmt::{Display, Formatter};
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::borrow::Borrow;
use std::iter::Iterator;

pub struct RingBuffer<T, const SIZE : usize>
{
    //storage is an array, because arrays _guarantee_ an initialized fixed size.
    //It's inside a box, because that way we can collect() into it without copying from heap to
    //stack.
    storage : Box<[T;SIZE]>,
    index : usize,
}

#[derive(Debug)]
pub struct RingBufferInitError;
impl Display for RingBufferInitError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"Not enough input to initialize the ring buffer")
    }
}
impl Error for RingBufferInitError {}

impl<T, const SIZE : usize> RingBuffer<T, SIZE> 
{
    pub fn push_pop(mut self,new_entry : T) -> (Self, T) {
        let old_value = std::mem::replace(&mut self.storage[self.index], new_entry);
        self.index = (self.index + 1) % SIZE;
        (self, old_value)
    }

    pub fn new<I>(mut input : I) -> Result<(Self,I), RingBufferInitError>
        where I : Iterator<Item=T>
    {
        let slice = input.by_ref().take(SIZE).collect::<Box<[T]>>();
        let array = slice.try_into().map_err(|_| RingBufferInitError)?;
        Ok((RingBuffer { storage : array, index : 0 },input))
    }

    pub fn new_init(init : T) -> Self 
        where T : Copy 
    {
        RingBuffer { storage : Box::new([init;SIZE]), index : 0 }
    }

    pub fn get(&self, index : usize) -> Option<&T> {
        if index >= SIZE {
            None
        }
        else {
            Some(&self.storage[self.get_arr_index_wrapped(index)])
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= SIZE {
            None
        }
        else {
            Some(&mut self.storage[self.get_arr_index_wrapped(index)])
        }
    }

    pub fn iter(&self) -> RingBufferIterator<T,SIZE> {
        RingBufferIterator { ring_buffer : self, index : 0 }
    }

    fn get_arr_index_wrapped(&self, index : usize) -> usize {
        (self.index + index) % SIZE
    }
}

impl<T, const SIZE :usize> Index<usize> for RingBuffer<T, SIZE> {
    type Output = T;
    fn index(&self, index : usize) -> &T {
        self.get(index).expect("Ring buffer index out of bounds")
    }
}

impl<T, const SIZE : usize> IndexMut<usize> for RingBuffer<T, SIZE> {
    fn index_mut(&mut self, index :usize) -> &mut T {
        self.get_mut(index).expect("Ring buffer index out of bounds")
    }
}

pub struct RingBufferIterator<'a, T, const SIZE :usize> {
    ring_buffer : &'a RingBuffer<T, SIZE>,
    index : usize,
}

impl<'b, T, const SIZE : usize> Iterator for RingBufferIterator<'b, T, SIZE> {
    type Item = &'b T;
    fn next(&mut self) -> Option<&'b T> {
        let result = self.ring_buffer.get(self.index);
        if result.is_some() {
            self.index += 1;
        }
        result
    }
}

impl<T, const SIZE : usize> IntoIterator for RingBuffer<T, SIZE> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let slice = self.storage as Box<[T]>;
        let vec : Vec<T> = slice.into();
        let mut deque : std::collections::VecDeque<T> = vec.into();
        deque.rotate_left(self.index);
        deque.into_iter()
    }
}

impl<'b, T, const SIZE : usize> IntoIterator for &'b RingBuffer<T, SIZE> {
    type Item = &'b T;
    type IntoIter = RingBufferIterator<'b, T, SIZE>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
