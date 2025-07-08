// src/ffi.rs (새 파일)
use super::*;

#[no_mangle]                       // 심볼 이름 그대로 노출
pub extern "C" fn AecNew(
    frame_size: usize,
    filter_length: i32,
    sample_rate: u32,
    enable_preprocess: bool,
) -> *mut Aec {
    // 안전: Box -> raw pointer
    Box::into_raw(Box::new(Aec::new(&AecConfig {
        frame_size,
        filter_length,
        sample_rate,
        enable_preprocess,
    })))
}

#[no_mangle]
pub extern "C" fn AecCancelEcho(
    ctx: *mut Aec,
    rec: *const i16,
    echo: *const i16,
    out: *mut i16,
    len: usize,
) {
    assert!(!ctx.is_null());
    let aec = unsafe { &*(ctx) };
    let rec = unsafe { std::slice::from_raw_parts(rec, len) };
    let echo = unsafe { std::slice::from_raw_parts(echo, len) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, len) };
    aec.cancel_echo(rec, echo, out);
}

#[no_mangle]
pub extern "C" fn AecDestroy(ctx: *mut Aec) {
    if !ctx.is_null() {
        unsafe { drop(Box::from_raw(ctx)); }
    }
}
