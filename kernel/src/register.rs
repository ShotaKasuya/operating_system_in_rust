use core::marker::PhantomData;
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub struct MemMapRegister<T> {
    value: *mut T,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T> MemMapRegister<T> where T: Copy + Default {
    pub fn new(value: *mut T, len: usize) -> Self {
        Self {
            value,
            len,
            _phantom: PhantomData,
        }
    }

    pub fn read(&self) -> T {
        unsafe {
            let mut tmp: T = Default::default();
            let src = from_raw_parts(self.value as *const u8, self.len);
            let dst = from_raw_parts_mut(&mut tmp as *mut T as *mut u8, self.len);
            for i in 0..self.len {
                dst[i] = src[i]
            }
            tmp
        }
    }

    pub fn write(&self, value: &T) {
        unsafe {
            let src = from_raw_parts(value as *const T as *const u8, self.len);
            let dst = from_raw_parts_mut(self.value as *mut u8, self.len);
            for i in 0..self.len {
                dst[i] = src[i];
            }
        }
    }
}

// struct DefaultBitmap<T> {
//     data: [T; 1],
// }
//
// impl<T> DefaultBitmap<T> {}
