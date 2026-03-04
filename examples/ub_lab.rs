// examples/ub_lab.rs

use trusty_playground::unsafe_helpers::unsafe_slice_from_raw_parts;

fn main() {
    // 1) Dangling pointer experiment
    unsafe {
        let ptr: *const u8;
        let len = 10;

        {
            let data = vec![1, 2, 3];
            ptr = data.as_ptr();
        } // data dropped here

        // UB: constructing a slice from a dangling pointer
        let _slice = unsafe_slice_from_raw_parts::<'_>(ptr, len);
        // Miri should flag this as UB.
    }
}
