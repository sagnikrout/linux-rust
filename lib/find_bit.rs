//! Automatically rewritten from C to Rust
//! Source: lib/find_bit.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// bit search implementation
//
// Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// Copyright (C) 2008 IBM Corporation
// 'find_last_bit' is written by Rusty Russell <rusty@rustcorp.com.au>
// (Inspired by David Howell's find_next_bit implementation)
//
// Rewritten by Yury Norov <yury.norov@gmail.com> to decrease
// size and improve performance, 2015.
//

//
// Common helper for find_bit() function family
// @FETCH: The expression that fetches and pre-processes each word of bitmap(s)
// @MUNGE: The expression that post-processes a word containing found bit (may be empty)
// @size: The bitmap size in bits
//

    ({										\
    unsigned long idx, val, sz = (size);					\
    \
    for (idx = 0; idx * BITS_PER_LONG < sz; idx++) {			\
    val = (FETCH);							\
    if (val) {							\
    sz = min(idx * BITS_PER_LONG + __ffs(MUNGE(val)), sz);	\
    break;							\
    }								\
    }									\
    \
    sz;									\
    })
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

    ({										\
    unsigned long sz = (size), nr = (num), idx, w, tmp = 0;			\
    \
    for (idx = 0; (idx + 1) * BITS_PER_LONG <= sz; idx++) {			\
    if (idx * BITS_PER_LONG + nr >= sz)				\
    goto out;						\
    \
    tmp = (FETCH);							\
    w = hweight_long(tmp);						\
    if (w > nr)							\
    goto found;						\
    \
    nr -= w;							\
    }									\
    \
    if (sz % BITS_PER_LONG)							\
    tmp = (FETCH) & BITMAP_LAST_WORD_MASK(sz);			\
    found:										\
    sz = idx * BITS_PER_LONG + fns(tmp, nr);				\
    out:										\
    sz;									\
    })

//
// Find the first set bit in a memory region.
//
#[no_mangle]
pub unsafe extern "C" fn _find_first_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long _find_first_bit(const unsigned long *addr, unsigned long size)
    {
    return FIND_FIRST_BIT(addr[idx], /* nop */, size);
    }
    EXPORT_SYMBOL(_find_first_bit);

//
// Find the first set bit in two memory regions.
//
    unsigned long _find_first_and_bit(const unsigned long *addr1,
    const unsigned long *addr2,
    unsigned long size)
    {
    return FIND_FIRST_BIT(addr1[idx] & addr2[idx], /* nop */, size);
    }
    EXPORT_SYMBOL(_find_first_and_bit);

//
// Find the first bit set in 1st memory region and unset in 2nd.
//
    unsigned long _find_first_andnot_bit(const unsigned long *addr1,
    const unsigned long *addr2,
    unsigned long size)
    {
    return FIND_FIRST_BIT(addr1[idx] & ~addr2[idx], /* nop */, size);
    }
    EXPORT_SYMBOL(_find_first_andnot_bit);
//
// Find the first set bit in three memory regions.
//
    unsigned long _find_first_and_and_bit(const unsigned long *addr1,
    const unsigned long *addr2,
    const unsigned long *addr3,
    unsigned long size)
    {
    return FIND_FIRST_BIT(addr1[idx] & addr2[idx] & addr3[idx], /* nop */, size);
    }
    EXPORT_SYMBOL(_find_first_and_and_bit);

//
// Find the first cleared bit in a memory region.
//
#[no_mangle]
pub unsafe extern "C" fn _find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long _find_first_zero_bit(const unsigned long *addr, unsigned long size)
    {
    return FIND_FIRST_BIT(~addr[idx], /* nop */, size);
    }
    EXPORT_SYMBOL(_find_first_zero_bit);

#[no_mangle]
pub unsafe extern "C" fn _find_next_bit(addr: *const c_ulong, nbits: c_ulong, start: c_ulong) -> c_ulong {
    unsigned long _find_next_bit(const unsigned long *addr, unsigned long nbits, unsigned long start)
    {
    return FIND_NEXT_BIT(addr[idx], /* nop */, nbits, start);
    }
    EXPORT_SYMBOL(_find_next_bit);

#[no_mangle]
pub unsafe extern "C" fn __find_nth_bit(addr: *const c_ulong, size: c_ulong, n: c_ulong) -> c_ulong {
    unsigned long __find_nth_bit(const unsigned long *addr, unsigned long size, unsigned long n)
    {
    return FIND_NTH_BIT(addr[idx], size, n);
    }
    EXPORT_SYMBOL(__find_nth_bit);
    unsigned long __find_nth_and_bit(const unsigned long *addr1, const unsigned long *addr2,
    unsigned long size, unsigned long n)
    {
    return FIND_NTH_BIT(addr1[idx] & addr2[idx], size, n);
    }
    EXPORT_SYMBOL(__find_nth_and_bit);
    unsigned long __find_nth_and_andnot_bit(const unsigned long *addr1,
    const unsigned long *addr2,
    const unsigned long *addr3,
    unsigned long size, unsigned long n)
    {
    return FIND_NTH_BIT(addr1[idx] & addr2[idx] & ~addr3[idx], size, n);
    }
    EXPORT_SYMBOL(__find_nth_and_andnot_bit);

    unsigned long _find_next_and_bit(const unsigned long *addr1, const unsigned long *addr2,
    unsigned long nbits, unsigned long start)
    {
    return FIND_NEXT_BIT(addr1[idx] & addr2[idx], /* nop */, nbits, start);
    }
    EXPORT_SYMBOL(_find_next_and_bit);

    unsigned long _find_next_andnot_bit(const unsigned long *addr1, const unsigned long *addr2,
    unsigned long nbits, unsigned long start)
    {
    return FIND_NEXT_BIT(addr1[idx] & ~addr2[idx], /* nop */, nbits, start);
    }
    EXPORT_SYMBOL(_find_next_andnot_bit);

    unsigned long _find_next_or_bit(const unsigned long *addr1, const unsigned long *addr2,
    unsigned long nbits, unsigned long start)
    {
    return FIND_NEXT_BIT(addr1[idx] | addr2[idx], /* nop */, nbits, start);
    }
    EXPORT_SYMBOL(_find_next_or_bit);

    unsigned long _find_next_zero_bit(const unsigned long *addr, unsigned long nbits,
    unsigned long start)
    {
    return FIND_NEXT_BIT(~addr[idx], /* nop */, nbits, start);
    }
    EXPORT_SYMBOL(_find_next_zero_bit);

#[no_mangle]
pub unsafe extern "C" fn _find_last_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long _find_last_bit(const unsigned long *addr, unsigned long size)
    {
    if (size) {
    let mut val: c_ulong = BITMAP_LAST_WORD_MASK(size);
    let mut idx: c_ulong = (size-1) / BITS_PER_LONG;
    do {
    val &= addr[idx];
    if (val)
    return idx * BITS_PER_LONG + __fls(val);
    val = ~0ul;
    } while (idx--);
    }
    return size;
    }
    EXPORT_SYMBOL(_find_last_bit);

    unsigned long find_next_clump8(unsigned long *clump, const unsigned long *addr,
    unsigned long size, unsigned long offset)
    {
    offset = find_next_bit(addr, size, offset);
    if (offset == size)
    return size;
    offset = round_down(offset, 8);
// clump = bitmap_get_value8(addr, offset);
    return offset;
    }
    EXPORT_SYMBOL(find_next_clump8);

//
// Find the first cleared bit in an LE memory region.
//
#[no_mangle]
pub unsafe extern "C" fn _find_first_zero_bit_le(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long _find_first_zero_bit_le(const unsigned long *addr, unsigned long size)
    {
    return FIND_FIRST_BIT(~addr[idx], swab, size);
    }
    EXPORT_SYMBOL(_find_first_zero_bit_le);

    unsigned long _find_next_zero_bit_le(const unsigned long *addr,
    unsigned long size, unsigned long offset)
    {
    return FIND_NEXT_BIT(~addr[idx], swab, size, offset);
    }
    EXPORT_SYMBOL(_find_next_zero_bit_le);

    unsigned long _find_next_bit_le(const unsigned long *addr,
    unsigned long size, unsigned long offset)
    {
    return FIND_NEXT_BIT(addr[idx], swab, size, offset);
    }
    EXPORT_SYMBOL(_find_next_bit_le);

//
// find_random_bit - find a set bit at random position
// @addr: The address to base the search on
// @size: The bitmap size in bits
//
// Returns: a position of a random set bit; >= @size otherwise
//
#[no_mangle]
pub unsafe extern "C" fn find_random_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long find_random_bit(const unsigned long *addr, unsigned long size)
    {
    let mut w: c_int = bitmap_weight(addr, size);
    switch (w) {
    case 0:
    return size;
    case 1:
// Performance trick for single-bit bitmaps
    return find_first_bit(addr, size);
    default:
    return find_nth_bit(addr, size, get_random_u32_below(w));
    }
    }
    EXPORT_SYMBOL(find_random_bit);
