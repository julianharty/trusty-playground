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

// UNSAFE: caller must ensure `ptr` is non-null, aligned, and valid for `len` bytes.
pub unsafe fn unsafe_slice_from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    std::slice::from_raw_parts(ptr, len)
}

// UNSAFE: caller must ensure `dest` has length >= `src.len()`.
pub unsafe fn unsafe_copy_into_buffer(src: &[u8], dest: &mut [u8]) -> usize {
    if dest.len() < src.len() {
        panic!("destination buffer too small");
    }

    let src_ptr = src.as_ptr();
    let dest_ptr = dest.as_mut_ptr();
    let copy_len = src.len();

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
    #[should_panic(expected = "destination buffer too small")]
    fn unsafe_copy_into_buffer_panics_when_dest_too_small() {
        let src = vec![1, 2, 3, 4];
        let mut dest = vec![0; 2];

        unsafe {
            let _ = unsafe_copy_into_buffer(&src, &mut dest);
        }
    }
}

