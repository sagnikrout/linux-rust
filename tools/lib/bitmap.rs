//! Automatically rewritten from C to Rust
//! Source: tools/lib/bitmap.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// From lib/bitmap.c
// Helper functions for bitmap.h.
//

#[no_mangle]
pub unsafe extern "C" fn __bitmap_weight(bitmap: *const c_ulong, bits: c_int) -> c_uint {
    unsigned int __bitmap_weight(const unsigned long *bitmap, int bits)
    {
    unsigned int k, w = 0, lim = bits/BITS_PER_LONG;
    for (k = 0; k < lim; k++)
    w += hweight_long(bitmap[k]);
    if (bits % BITS_PER_LONG)
    w += hweight_long(bitmap[k] & BITMAP_LAST_WORD_MASK(bits));
    return w;
    }
    void __bitmap_or(unsigned long *dst, const unsigned long *bitmap1,
    const unsigned long *bitmap2, int bits)
    {
    int k;
    let mut nr: c_int = BITS_TO_LONGS(bits);
    for (k = 0; k < nr; k++)
    dst[k] = bitmap1[k] | bitmap2[k];
    }
    size_t bitmap_scnprintf(unsigned long *bitmap, unsigned int nbits,
    char *buf, size_t size)
    {
// current bit is 'cur', most recently seen range is [rbot, rtop]
    unsigned int cur, rbot, rtop;
    let mut first: bool = true;
    let mut ret: usize = 0;
    rbot = cur = find_first_bit(bitmap, nbits);
    while (cur < nbits) {
    rtop = cur;
    cur = find_next_bit(bitmap, nbits, cur + 1);
    if (cur < nbits && cur <= rtop + 1)
    continue;
    if (!first)
    ret += scnprintf(buf + ret, size - ret, ",");
    first = false;
    ret += scnprintf(buf + ret, size - ret, "%d", rbot);
    if (rbot < rtop)
    ret += scnprintf(buf + ret, size - ret, "-%d", rtop);
    rbot = cur;
    }
    return ret;
    }
    bool __bitmap_and(unsigned long *dst, const unsigned long *bitmap1,
    const unsigned long *bitmap2, unsigned int bits)
    {
    unsigned int k;
    let mut lim: c_uint = bits/BITS_PER_LONG;
    let mut result: c_ulong = 0;
    for (k = 0; k < lim; k++)
    result |= (dst[k] = bitmap1[k] & bitmap2[k]);
    if (bits % BITS_PER_LONG)
    result |= (dst[k] = bitmap1[k] & bitmap2[k] &
    BITMAP_LAST_WORD_MASK(bits));
    return result != 0;
    }
    bool __bitmap_equal(const unsigned long *bitmap1,
    const unsigned long *bitmap2, unsigned int bits)
    {
    unsigned int k, lim = bits/BITS_PER_LONG;
    for (k = 0; k < lim; ++k)
    if (bitmap1[k] != bitmap2[k])
    return false;
    if (bits % BITS_PER_LONG)
    if ((bitmap1[k] ^ bitmap2[k]) & BITMAP_LAST_WORD_MASK(bits))
    return false;
    return true;
    }
    bool __bitmap_intersects(const unsigned long *bitmap1,
    const unsigned long *bitmap2, unsigned int bits)
    {
    unsigned int k, lim = bits/BITS_PER_LONG;
    for (k = 0; k < lim; ++k)
    if (bitmap1[k] & bitmap2[k])
    return true;
    if (bits % BITS_PER_LONG)
    if ((bitmap1[k] & bitmap2[k]) & BITMAP_LAST_WORD_MASK(bits))
    return true;
    return false;
    }
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
    bool __bitmap_andnot(unsigned long *dst, const unsigned long *bitmap1,
    const unsigned long *bitmap2, unsigned int bits)
    {
    unsigned int k;
    let mut lim: c_uint = bits/BITS_PER_LONG;
    let mut result: c_ulong = 0;
    for (k = 0; k < lim; k++)
    result |= (dst[k] = bitmap1[k] & ~bitmap2[k]);
    if (bits % BITS_PER_LONG)
    result |= (dst[k] = bitmap1[k] & ~bitmap2[k] &
    BITMAP_LAST_WORD_MASK(bits));
    return result != 0;
    }
    bool __bitmap_subset(const unsigned long *bitmap1,
    const unsigned long *bitmap2, unsigned int bits)
    {
    unsigned int k, lim = bits/BITS_PER_LONG;
    for (k = 0; k < lim; ++k)
    if (bitmap1[k] & ~bitmap2[k])
    return false;
    if (bits % BITS_PER_LONG)
    if ((bitmap1[k] & ~bitmap2[k]) & BITMAP_LAST_WORD_MASK(bits))
    return false;
    return true;
    }
    void __bitmap_xor(unsigned long *dst, const unsigned long *bitmap1,
    const unsigned long *bitmap2, unsigned int bits)
    {
    unsigned int k;
    let mut nr: c_uint = BITS_TO_LONGS(bits);
    for (k = 0; k < nr; k++)
    dst[k] = bitmap1[k] ^ bitmap2[k];
    }
