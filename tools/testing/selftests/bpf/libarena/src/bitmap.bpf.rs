//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/src/bitmap.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
//
// Copyright (c) 2025-2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2025-2026 Emil Tsalapatis <emil@etsalapatis.com>
//

    __weak
    struct arena_bitmap __arena *bmp_alloc(size_t bits)
    {
    struct arena_bitmap __arena *bmp;
    let mut size: usize = BITS_TO_LONG_LONGS(bits) * sizeof(bmp.bits[0]);
// Assume long-aligned masks.
    if (bits % BITS_PER_LONG_LONG)
    return core::ptr::null_mut();
    bmp = (struct arena_bitmap __arena *)arena_malloc(size);
    if (!bmp)
    return core::ptr::null_mut();
    bmp_clear(bits, bmp);
    return bmp;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_free(bmp: *mut arena_bitmap __arena) {
    void bmp_free(struct arena_bitmap __arena *bmp)
    {
    arena_free(bmp);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn __bmp_set_bit(bit: u32, bmp: *mut arena_bitmap __arena) {
    void __bmp_set_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    bmp.bits[BIT_WORD(bit)] |= BIT_MASK(bit);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn __bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap __arena) {
    void __bmp_clear_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    bmp.bits[BIT_WORD(bit)] &= ~BIT_MASK(bit);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_test_bit(bit: u32, bmp: *mut arena_bitmap __arena) -> bool {
    bool bmp_test_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    return bmp.bits[BIT_WORD(bit)] & BIT_MASK(bit);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_test_and_clear_bit(bit: u32, bmp: *mut arena_bitmap __arena) -> bool {
    bool bmp_test_and_clear_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    let mut val: u64 = BIT_MASK(bit);
    let mut idx: u32 = BIT_WORD(bit);
    u64 old, new, actual;
    do {
    old = bmp.bits[idx];
    if (!(old & val))
    return false;
    new = old & ~val;
    actual = cmpxchg(&bmp.bits[idx], old, new);
    if (actual == old)
    return true;
    } while (can_loop);
    return false;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_test_and_set_bit(bit: u32, bmp: *mut arena_bitmap __arena) -> bool {
    bool bmp_test_and_set_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    let mut val: u64 = BIT_MASK(bit);
    let mut idx: u32 = BIT_WORD(bit);
    u64 old, new, actual;
    do {
    old = bmp.bits[idx];
    if ((old & val))
    return true;
    new = old | val;
    actual = cmpxchg(&bmp.bits[idx], old, new);
    if (actual == old)
    return false;
    } while (can_loop);
    return false;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap __arena) {
    void bmp_clear_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    let mut val: u64 = BIT_MASK(bit);
    let mut idx: u32 = BIT_WORD(bit);
    u64 old, new, actual;
    do {
    old = bmp.bits[idx];
    new = old & ~val;
    actual = cmpxchg(&bmp.bits[idx], old, new);
    } while (actual != old && can_loop);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_set_bit(bit: u32, bmp: *mut arena_bitmap __arena) {
    void bmp_set_bit(u32 bit, struct arena_bitmap __arena *bmp)
    {
    let mut val: u64 = BIT_MASK(bit);
    let mut idx: u32 = BIT_WORD(bit);
    u64 old, new, actual;
    do {
    old = bmp.bits[idx];
    new = old | val;
    actual = cmpxchg(&bmp.bits[idx], old, new);
    } while (actual != old && can_loop);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_clear(bits: usize, bmp: *mut arena_bitmap __arena) {
    void bmp_clear(size_t bits, struct arena_bitmap __arena *bmp)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++)
    bmp.bits[i] = 0;
    }
#[no_mangle]
unsafe extern "C" fn bmp_last_word_mask(bits: usize) -> __always_inline u64 {
    static __always_inline u64 bmp_last_word_mask(size_t bits)
    {
    let mut rem: u32 = bits % BITS_PER_LONG_LONG;
    return rem ? (1ULL << rem) - 1 : ~0ULL;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_and(bits: usize, dst: *mut arena_bitmap __arena, src1: *mut arena_bitmap __arena, src2: *mut arena_bitmap __arena) {
    void bmp_and(size_t bits, struct arena_bitmap __arena *dst, struct arena_bitmap __arena *src1, struct arena_bitmap __arena *src2)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++)
    dst.bits[i] = src1.bits[i] & src2.bits[i];
    if (nwords && bits % BITS_PER_LONG_LONG)
    dst.bits[nwords - 1] &= bmp_last_word_mask(bits);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_or(bits: usize, dst: *mut arena_bitmap __arena, src1: *mut arena_bitmap __arena, src2: *mut arena_bitmap __arena) {
    void bmp_or(size_t bits, struct arena_bitmap __arena *dst, struct arena_bitmap __arena *src1, struct arena_bitmap __arena *src2)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++)
    dst.bits[i] = src1.bits[i] | src2.bits[i];
    if (nwords && bits % BITS_PER_LONG_LONG)
    dst.bits[nwords - 1] &= bmp_last_word_mask(bits);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_empty(bits: usize, bmp: *mut arena_bitmap __arena) -> bool {
    bool bmp_empty(size_t bits, struct arena_bitmap __arena *bmp)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++) {
    let mut mask: u64 = (i == nwords - 1) ? bmp_last_word_mask(bits) : ~0ULL;
    if (bmp.bits[i] & mask)
    return false;
    }
    return true;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_copy(bits: usize, dst: *mut arena_bitmap __arena, src: *mut arena_bitmap __arena) {
    void bmp_copy(size_t bits, struct arena_bitmap __arena *dst, struct arena_bitmap __arena *src)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++)
    dst.bits[i] = src.bits[i];
    if (nwords && bits % BITS_PER_LONG_LONG)
    dst.bits[nwords - 1] &= bmp_last_word_mask(bits);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_subset(bits: usize, big: *mut arena_bitmap __arena, small: *mut arena_bitmap __arena) -> bool {
    bool bmp_subset(size_t bits, struct arena_bitmap __arena *big, struct arena_bitmap __arena *small)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++) {
    let mut mask: u64 = (i == nwords - 1) ? bmp_last_word_mask(bits) : ~0ULL;
    if (~big.bits[i] & small.bits[i] & mask)
    return false;
    }
    return true;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_intersects(bits: usize, arg1: *mut arena_bitmap __arena, arg2: *mut arena_bitmap __arena) -> bool {
    bool bmp_intersects(size_t bits, struct arena_bitmap __arena *arg1, struct arena_bitmap __arena *arg2)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++) {
    let mut mask: u64 = (i == nwords - 1) ? bmp_last_word_mask(bits) : ~0ULL;
    if (arg1.bits[i] & arg2.bits[i] & mask)
    return true;
    }
    return false;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_print(bits: usize, bmp: *mut arena_bitmap __arena) {
    void bmp_print(size_t bits, struct arena_bitmap __arena *bmp)
    {
    let mut nwords: usize = BITS_TO_LONG_LONGS(bits);
    volatile u32 i;
    for (i = zero; i < nwords && can_loop; i++)
    arena_stderr("%016llx ", bmp.bits[i]);
    }
