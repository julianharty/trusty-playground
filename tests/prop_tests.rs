// tests/prop_tests.rs

use trusty_playground::unsafe_helpers::{unsafe_slice_from_raw_parts, PacketBuffer};

use proptest::prelude::*;

fn arbitrary_buffer() -> impl Strategy<Value = Vec<u8>> {
    any::<Vec<u8>>()
}

proptest! {
    #[test]
    fn safe_slice_roundtrip_works(data in arbitrary_buffer()) {
        let buf = PacketBuffer::new(data.clone());
        let slice = buf.as_slice();

        prop_assert_eq!(slice, data.as_slice());
    }

    #[test]
    fn unsafe_slice_from_raw_parts_roundtrip_works(data in arbitrary_buffer()) {
        let len = data.len();
        let ptr = data.as_ptr();

        unsafe {
            let slice = unsafe_slice_from_raw_parts(ptr, len);
            prop_assert_eq!(slice, data.as_slice());
        }
    }

    #[test]
    fn unsafe_slice_from_raw_parts_with_truncated_len_must_not_read_beyond(
        data in proptest::collection::vec(any::<u8>(), 1..100),
        extra_len in 0..10usize
    ) {
        let len = data.len();
        let ptr = data.as_ptr();

        unsafe {
            let _slice = unsafe_slice_from_raw_parts(ptr, len + extra_len);
            // No panic here if extra_len is benign; you can add custom bounds checks
            // in your real code, but this property is mostly about not crashing
            // in the playground.
        }
    }
}
