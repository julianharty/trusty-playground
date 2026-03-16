// tests/prop_tests.rs

use proptest::prelude::*;
use proptest::test_runner::Config as ProptestConfig;

use trusty_playground::unsafe_helpers::{unsafe_slice_from_raw_parts, PacketBuffer};

fn arbitrary_buffer() -> impl Strategy<Value = Vec<u8>> {
    any::<Vec<u8>>()
}

proptest! {
    #![proptest_config({
        let mut cfg = ProptestConfig::default();
        if cfg!(miri) {
            cfg.cases = 8;
            cfg.failure_persistence = None;
        }
        cfg
    })]

    // ===========================
    // 1. Safe properties (no UB)
    // ===========================

    /// PacketBuffer::as_slice is a pure view over the inner Vec.
    #[test]
    fn safe_slice_roundtrip_works(data in arbitrary_buffer()) {
        let buf = PacketBuffer::new(data.clone());
        let slice = buf.as_slice();

        prop_assert_eq!(slice, data.as_slice());
    }

    /// unsafe_slice_from_raw_parts behaves like &data[..] when called soundly.
    #[test]
    fn unsafe_slice_from_raw_parts_roundtrip_works(data in arbitrary_buffer()) {
        let len = data.len();
        let ptr = data.as_ptr();

        unsafe {
            let slice = unsafe_slice_from_raw_parts(ptr, len);
            prop_assert_eq!(slice.len(), len);
            prop_assert_eq!(slice, data.as_slice());
        }
    }

    /// When we choose a sub-slice via safe indexing and then call the unsafe
    /// helper with that pointer+len, we get back exactly that sub-slice.
    #[test]
    fn unsafe_slice_from_raw_parts_respects_subslices(
        data in proptest::collection::vec(any::<u8>(), 0..100),
        lo in 0usize..100,
        hi in 0usize..100,
    ) {
        let len = data.len();
        let lo = lo.min(len);
        let hi = hi.min(len);
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };

        let sub = &data[lo..hi];
        let ptr = sub.as_ptr();
        let sub_len = sub.len();

        unsafe {
            let slice = unsafe_slice_from_raw_parts(ptr, sub_len);
            prop_assert_eq!(slice, sub);
        }
    }

    /// If we conceptually request "len + extra" bytes but clamp to the actual
    /// allocation length before calling the unsafe helper, we never expose
    /// bytes beyond the true buffer and the slice contents are consistent.
    #[test]
    fn unsafe_slice_from_raw_parts_with_truncated_len_must_not_read_beyond(
        data in proptest::collection::vec(any::<u8>(), 1..100),
        extra_len in 0usize..10,
    ) {
        let len = data.len();
        let requested_len = len + extra_len;
        let clamped_len = requested_len.min(len);

        let ptr = data.as_ptr();

        unsafe {
            let slice = unsafe_slice_from_raw_parts(ptr, clamped_len);

            prop_assert_eq!(slice.len(), clamped_len);
            prop_assert_eq!(slice, &data[..clamped_len]);
        }
    }

    /// A simple "packet" model: first byte is a header, rest is body.
    /// Rebuilding a PacketBuffer from header+body and viewing it as a slice
    /// must match the original bytes.
    #[test]
    fn packet_like_header_body_roundtrip(
        // Require at least 1 byte so there is a header.
        bytes in proptest::collection::vec(any::<u8>(), 1..100),
    ) {
        let header = bytes[0];
        let body = &bytes[1..];

        // Rebuild a PacketBuffer from the same structure.
        let mut rebuilt = Vec::with_capacity(bytes.len());
        rebuilt.push(header);
        rebuilt.extend_from_slice(body);

        let buf = PacketBuffer::new(rebuilt);
        let view = buf.as_slice();

        prop_assert_eq!(view, bytes.as_slice());
    }

    /// Two "packets" concatenated into one buffer: splitting the
    /// PacketBuffer's slice at the boundary must recover the original
    /// packet payloads.
    #[test]
    fn packet_like_concatenated_roundtrip(
        packet1 in proptest::collection::vec(any::<u8>(), 0..50),
        packet2 in proptest::collection::vec(any::<u8>(), 0..50),
    ) {
        let mut combined = Vec::with_capacity(packet1.len() + packet2.len());
        combined.extend_from_slice(&packet1);
        combined.extend_from_slice(&packet2);

        let buf = PacketBuffer::new(combined);
        let view = buf.as_slice();

        let split = packet1.len();
        let (view1, view2) = view.split_at(split);

        prop_assert_eq!(view1, packet1.as_slice());
        prop_assert_eq!(view2, packet2.as_slice());
    }

    // =======================================================
    // 2. Deliberately UB examples (for teaching & exploration)
    //    - DO NOT RUN UNDER MIRI
    //    - Opt-in only even under `cargo test`
    // =======================================================

    /// Intentionally violates the contract of unsafe_slice_from_raw_parts by
    /// asking for more bytes than the underlying allocation has.
    #[test]
    #[ignore]                 // opt-in only
    #[cfg_attr(miri, ignore)] // and never under Miri
    fn unsafe_slice_from_raw_parts_intentionally_ub(
        data in proptest::collection::vec(any::<u8>(), 1..100),
        extra_len in 1..10usize,
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

