#[no_mangle]
#[used]
static mut _VESSEL_BUFFER: [u8; 4096] = [0u8; 4096];

fn _vessel_write_safe<T: AsRef<[u8]>>(data: T) {
    use core::ptr::slice_from_raw_parts_mut;

    let data = data.as_ref();

    unsafe {
        slice_from_raw_parts_mut(_VESSEL_BUFFER.as_mut_ptr(), data.len())
            .as_mut()
            .unwrap()
            .copy_from_slice(data);
    }
}

#[no_mangle]
pub extern "C" fn _vessel_read() -> usize {
    _vessel_write_safe("hello there");
    "hello there".as_bytes().len() + 1
}
