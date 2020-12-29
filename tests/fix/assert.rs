use std::ptr;

#[allow(clippy::cast_ptr_alignment)]
pub unsafe fn memcpy_16(src: *const u8, dst: *mut u8, len: usize) {
    assert!(len <= 16);
    let len_u8 = len as u8;

    if len_u8 >= 8 {
        let offset = len - 8;
        let t2 = ptr::read_unaligned(src.add(offset) as *const u64);
        let t1 = ptr::read_unaligned(src as *const u64);
        ptr::write_unaligned(dst.add(offset) as *mut u64, t2);
        ptr::write_unaligned(dst as *mut u64, t1);
    } else if len_u8 >= 4 {
        debug_assert!(len_u8 < 8);
        let offset = len - 4;
        let t2 = ptr::read_unaligned(src.add(offset) as *const u32);
        let t1 = ptr::read_unaligned(src as *const u32);
        ptr::write_unaligned(dst.add(offset) as *mut u32, t2);
        ptr::write_unaligned(dst as *mut u32, t1);
    } else if len_u8 >= 2 {
        debug_assert!(len_u8 < 4);
        let offset = len - 2;
        let t2 = ptr::read_unaligned(src.add(offset) as *const u16);
        let t1 = ptr::read_unaligned(src as *const u16);
        ptr::write_unaligned(dst.add(offset) as *mut u16, t2);
        ptr::write_unaligned(dst as *mut u16, t1);
    } else if len_u8 >= 1 {
        debug_assert_eq!(len_u8, 1);
        *dst = *src;
    }
}
