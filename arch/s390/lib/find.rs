//! Automatically rewritten from C to Rust
//! Source: arch/s390/lib/find.c
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


// SPDX-License-Identifier: GPL-2.0
//
// MSB0 numbered special bitops handling.
//
// The bits are numbered:
// |0..............63|64............127|128...........191|192...........255|
//
// The reason for this bit numbering is the fact that the hardware sets bits
// in a bitmap starting at bit 0 (MSB) and we don't want to scan the bitmap
// from the 'wrong end'.
//

#[no_mangle]
pub unsafe extern "C" fn find_first_bit_inv(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long find_first_bit_inv(const unsigned long *addr, unsigned long size)
    {
    const unsigned long *p = addr;
    let mut result: c_ulong = 0;
    unsigned long tmp;
    while (size & ~(BITS_PER_LONG - 1)) {
    if ((tmp = *(p++)))
    goto found;
    result += BITS_PER_LONG;
    size -= BITS_PER_LONG;
    }
    if (!size)
    return result;
    tmp = (*p) & (~0UL << (BITS_PER_LONG - size));
    if (!tmp)		/* Are any bits set? */
    return result + size;	/* Nope. */
    found:
    return result + (__fls(tmp) ^ (BITS_PER_LONG - 1));
    }
    EXPORT_SYMBOL(find_first_bit_inv);
    unsigned long find_next_bit_inv(const unsigned long *addr, unsigned long size,
    unsigned long offset)
    {
    const unsigned long *p = addr + (offset / BITS_PER_LONG);
    let mut result: c_ulong = offset & ~(BITS_PER_LONG - 1);
    unsigned long tmp;
    if (offset >= size)
    return size;
    size -= result;
    offset %= BITS_PER_LONG;
    if (offset) {
    tmp = *(p++);
    tmp &= (~0UL >> offset);
    if (size < BITS_PER_LONG)
    goto found_first;
    if (tmp)
    goto found_middle;
    size -= BITS_PER_LONG;
    result += BITS_PER_LONG;
    }
    while (size & ~(BITS_PER_LONG-1)) {
    if ((tmp = *(p++)))
    goto found_middle;
    result += BITS_PER_LONG;
    size -= BITS_PER_LONG;
    }
    if (!size)
    return result;
    tmp = *p;
    found_first:
    tmp &= (~0UL << (BITS_PER_LONG - size));
    if (!tmp)		/* Are any bits set? */
    return result + size;	/* Nope. */
    found_middle:
    return result + (__fls(tmp) ^ (BITS_PER_LONG - 1));
    }
    EXPORT_SYMBOL(find_next_bit_inv);
