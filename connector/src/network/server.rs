use std::mem::MaybeUninit;
use std::ptr;

pub fn connect() {

}

pub struct HeapVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl <T, const N: usize> HeapVec<T, N> {

    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        unsafe { self.data[self.len].as_mut_ptr().write(value) };
        self.len += 1;
        Ok( () )
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                let value = ptr::read( self.data[self.len].as_ptr() );
                Some(value)
            }
        }
    }

}

impl<T, const N: usize> Drop for HeapVec<T, N> {

    // type Target = [T];

    fn drop(&mut self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len)
        }
    }

}