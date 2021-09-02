use core::mem::size_of;
use core::ptr::NonNull;


pub fn ptrdistance<T>(start: *const T, end: *const T) -> usize {
    let size = size_of::<T>();
    if size == 0 {
        (end as usize).wrapping_sub(start as usize)
    } else {
        (end as usize - start as usize) / size
    }
}

pub trait PointerExt: Copy {
    unsafe fn offset(self, i: isize) -> Self;
    unsafe fn add(self, i: usize) -> Self {
        self.offset(i as isize)
    }
    unsafe fn sub(self, i: usize) -> Self {
        self.offset((i as isize).wrapping_neg())
    }
    //increment the pointer by 1, return a new value
    // ++ptr
    unsafe fn pre_inc(&mut self) -> Self {
        *self = self.offset(1);
        *self
    }
    
    //increment the pointer by 1 , return old value
    //ptr++
    unsafe fn post_inc(&mut self) -> Self {
        let cur = self.clone();
        *self = self.offset(1);
        cur
    }
    //--ptr
    unsafe fn pre_dec(&mut self) -> Self {
        *self = self.offset(-1);
        *self
    }
    //ptr--
    unsafe fn post_dec(&mut self) -> Self {
        let cur = *self;
        *self = self.offset(1);
        cur
    }
    unsafe fn inc(&mut self) {
        *self = self.offset(1);
    }
    unsafe fn dec(&mut self) {
        *self = self.offset(-1);
    }
    unsafe fn stride_offset(self, s: isize, idx: usize) -> Self {
        self.offset(s + idx as isize)
    }
}

impl<T> PointerExt for *const T {
    unsafe fn offset(self, i: isize) -> Self {
        self.offset(i)
    }
    unsafe fn add(self, i: usize) -> Self {
        self.add(i)
    }
    unsafe fn sub(self, i: usize) -> Self {
        self.sub(i)
    }
}

impl<T> PointerExt for *mut T {
    unsafe fn offset(self, i: isize) -> Self {
        self.offset(i)
    }
    unsafe fn add(self, i: usize) -> Self {
        self.add(i)
    }
    unsafe fn sub(self, i: usize) -> Self {
        self.sub(i)
    }
}
impl<T> PointerExt for NonNull<T> {
    unsafe fn offset(self, i: isize) -> Self {
        NonNull::new_unchecked(self.as_ptr().offset(i))
    }
}
#[cfg(test)]
mod tests {
    use super::PointerExt;
    use core::ptr::NonNull;
    #[test]
    fn run() {
        unsafe {
            let mut num = [0; 16];
            let mut ptr = num.as_mut_ptr();
            let end = ptr.offset(4);
            let mut i = 0;
            while ptr != end {
                *ptr.post_inc() = i;
                i += 1;
            }
            assert_eq!(&num[..8], &[0, 1, 2, 3, 0, 0, 0, 0]);
        }
    }
    #[test]
    fn nonnull_sub_run() {
        unsafe {
            let mut num = [0; 16];
            let mut ptr = num.as_mut_ptr().add(num.len());
            let nptr = NonNull::new(num.as_mut_ptr()).unwrap();
            let mut nend = nptr.add(num.len());
            let mut i = 0;
            while nptr != nend {
                nend = nend.sub(1);
                ptr = ptr.sub(1);
                assert_eq!(nend.as_ptr(), ptr);
                *nend.as_ptr() = i;
                i += 1;
            }
            assert_eq!(&num[..8], &[15, 14, 13, 12, 11, 10, 9, 8]);

        }
    }
}
