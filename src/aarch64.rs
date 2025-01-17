use std::arch::aarch64::{uint8x16_t, vaddv_u8, vandq_u8, vceqq_u8, vdupq_n_u8, vget_high_u8, vget_low_u8, vld1q_s8, vld1q_u8, vshlq_u8};

/// Equivalent to MoveMask in x86
#[inline]
pub fn _mm_movemask_aarch64(input: uint8x16_t) -> u32 {
    const UC_SHIFT: [i8; 16] = [-7, -6, -5, -4, -3, -2, -1, 0, -7, -6, -5, -4, -3, -2, -1, 0];
    // Fills a vector with UC_SHIFT
    let vshift = unsafe { vld1q_s8(UC_SHIFT.as_ptr()) };
    // Fills a vector with 0x80 and performs AND on the input vector
    let vmask = unsafe { vandq_u8(input, vdupq_n_u8(0x80)) };
    // Shift-left vmask using UC_SHIFT
    let vmask = unsafe { vshlq_u8(vmask, vshift) };

    // Takes the lower 64 bits of vmask and add all bytes together
    let mut out: u32 = unsafe { vaddv_u8(vget_low_u8(vmask)) }.into();
    // Takes the higher 64 bits of vmask, add all bytes together then shift left by 8 and add the result to out
    out += unsafe { (vaddv_u8(vget_high_u8(vmask)) as u32) << 8 };

    out
}