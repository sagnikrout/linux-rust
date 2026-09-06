//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/find.c
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
// Common helper for find_next_bit() function family
// @FETCH: The expression that fetches and pre-processes each word of bitmap(s)
// @MUNGE: The expression that post-processes a word containing found bit (may be empty)
// @size: The bitmap size in bits
// @start: The bitnumber to start searching at
//

    ({										\
    unsigned long mask, idx, tmp, sz = (size), __start = (start);		\
    \
    if (unlikely(__start >= sz))						\
    goto out;							\
    \
    mask = MUNGE(BITMAP_FIRST_WORD_MASK(__start));				\
    idx = __start / BITS_PER_LONG;						\
    \
    for (tmp = (FETCH) & mask; !tmp; tmp = (FETCH)) {			\
    if ((idx + 1) * BITS_PER_LONG >= sz)				\
    goto out;						\
    idx++;								\
    }									\
    \
    sz = min(idx * BITS_PER_LONG + __ffs(MUNGE(tmp)), sz);			\
    out:										\
    sz;									\
    })
#[no_mangle]
pub unsafe extern "C" fn _find_next_bit(addr: *const c_ulong, nbits: c_ulong, start: c_ulong) -> c_ulong {
    unsigned long _find_next_bit(const unsigned long *addr, unsigned long nbits, unsigned long start)
    {
    return FIND_NEXT_BIT(addr[idx], /* nop */, nbits, start);
    }
    unsigned long _find_next_zero_bit(const unsigned long *addr, unsigned long nbits,
    unsigned long start)
    {
    return FIND_NEXT_BIT(~addr[idx], /* nop */, nbits, start);
    }
