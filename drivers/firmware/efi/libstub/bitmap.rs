//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/bitmap.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


#[no_mangle]
pub unsafe extern "C" fn __bitmap_set(map: *mut c_ulong, start: c_uint, len: c_int) {
    void __bitmap_set(unsigned long *map, unsigned int start, int len)
    {
    unsigned long *p = map + BIT_WORD(start);
    let mut size: c_uint = start + len;
    let mut bits_to_set: c_int = BITS_PER_LONG - (start % BITS_PER_LONG);
    let mut mask_to_set: c_ulong = BITMAP_FIRST_WORD_MASK(start);
    while (len - bits_to_set >= 0) {
// p |= mask_to_set;
    len -= bits_to_set;
    bits_to_set = BITS_PER_LONG;
    mask_to_set = ~0UL;
    p++;
    }
    if (len) {
    mask_to_set &= BITMAP_LAST_WORD_MASK(size);
// p |= mask_to_set;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __bitmap_clear(map: *mut c_ulong, start: c_uint, len: c_int) {
    void __bitmap_clear(unsigned long *map, unsigned int start, int len)
    {
    unsigned long *p = map + BIT_WORD(start);
    let mut size: c_uint = start + len;
    let mut bits_to_clear: c_int = BITS_PER_LONG - (start % BITS_PER_LONG);
    let mut mask_to_clear: c_ulong = BITMAP_FIRST_WORD_MASK(start);
    while (len - bits_to_clear >= 0) {
// p &= ~mask_to_clear;
    len -= bits_to_clear;
    bits_to_clear = BITS_PER_LONG;
    mask_to_clear = ~0UL;
    p++;
    }
    if (len) {
    mask_to_clear &= BITMAP_LAST_WORD_MASK(size);
// p &= ~mask_to_clear;
    }
    }
