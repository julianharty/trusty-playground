// src/unsafe_helpers.rs

pub struct PacketBuffer {
    buf: Vec<u8>,
}

impl PacketBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        PacketBuffer { buf: data }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

// The returned slice borrows from the memory `ptr` points to.
// We express that with an explicit lifetime `'a`.
pub unsafe fn unsafe_slice_from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    std::slice::from_raw_parts(ptr, len)
}

pub unsafe fn unsafe_copy_into_buffer(src: &[u8], dest: &mut [u8]) -> usize {
    let src_ptr = src.as_ptr();
    let dest_ptr = dest.as_mut_ptr();

    let copy_len = std::cmp::min(src.len(), dest.len());
    std::ptr::copy_nonoverlapping(src_ptr, dest_ptr, copy_len);

    copy_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_slice_works() {
        let buf = PacketBuffer::new(vec![1, 2, 3, 4]);
        let slice = buf.as_slice();

        assert_eq!(slice, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsafe_slice_from_raw_parts_works_safely() {
        let data = vec![1, 2, 3];
        let ptr = data.as_ptr();
        let len = data.len();

        unsafe {
            let slice = unsafe_slice_from_raw_parts(ptr, len);
            assert_eq!(slice, &[1, 2, 3]);
        }
    }

    #[test]
    #[should_panic]
    fn unsafe_slice_from_raw_parts_panic_on_dangling() {
        let ptr: *const u8;
        let len = 10;

        {
            let data = vec![1, 2, 3];
            ptr = data.as_ptr();
            // NOTE: creating a slice after `data` is dropped is UB; we keep this
            // as an example of "what *not* to do" for Miri/fuzzing experiments.
        } // `data` is dropped here

        unsafe {
            let _slice = unsafe_slice_from_raw_parts(ptr, len);
        }
    }

    #[test]
    fn unsafe_copy_into_buffer_works() {
        let src = vec![1, 2, 3, 4, 5];
        let mut dest = vec![0; 8];

        unsafe {
            let n = unsafe_copy_into_buffer(&src, &mut dest);
            assert_eq!(n, 5);
            assert_eq!(dest, [1, 2, 3, 4, 5, 0, 0, 0]);
        }
    }
   #[test]
   #[should_panic]
   fn unsafe_copy_into_buffer_panic_on_null_dest() {
       let src = vec![1, 2, 3];
       let ptr: *mut u8 = std::ptr::null_mut();

       unsafe {
           let _ = unsafe_copy_into_buffer(&src, std::slice::from_raw_parts_mut(ptr, 4));
       }
   }
}

