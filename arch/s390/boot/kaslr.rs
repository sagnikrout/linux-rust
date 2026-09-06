//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/kaslr.c
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
// Copyright IBM Corp. 2019
//

pub const PRNG_MODE_TDES: c_int = 1;
pub const PRNG_MODE_SHA512: c_int = 2;
pub const PRNG_MODE_TRNG: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prno_parm {
    pub res: u32,
    pub reseed_counter: u32,
    pub stream_bytes: u64,
    pub V: [u8; 112],
    pub C: [u8; 112],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prng_parm {
    pub parm_block: [u8; 32],
    pub reseed_counter: u32,
    pub byte_counter: u64,
}

#[no_mangle]
unsafe extern "C" fn check_prng() -> c_int {
    static int check_prng(void)
    {
    if (!cpacf_query_func(CPACF_KMC, CPACF_KMC_PRNG)) {
    boot_warn("KASLR disabled: CPU has no PRNG\n");
    return 0;
    }
    if (cpacf_query_func(CPACF_PRNO, CPACF_PRNO_TRNG))
    return PRNG_MODE_TRNG;
    if (cpacf_query_func(CPACF_PRNO, CPACF_PRNO_SHA512_DRNG_GEN))
    return PRNG_MODE_SHA512;
    else
    return PRNG_MODE_TDES;
    }
#[no_mangle]
pub unsafe extern "C" fn get_random(limit: c_ulong, value: *mut c_ulong) -> c_int {
    int get_random(unsigned long limit, unsigned long *value)
    {
    struct prng_parm prng = {
// initial parameter block for tdes mode, copied from libica
    .parm_block = {
    0x0F, 0x2B, 0x8E, 0x63, 0x8C, 0x8E, 0xD2, 0x52,
    0x64, 0xB7, 0xA0, 0x7B, 0x75, 0x28, 0xB8, 0xF4,
    0x75, 0x5F, 0xD2, 0xA6, 0x8D, 0x97, 0x11, 0xFF,
    0x49, 0xD8, 0x23, 0xF3, 0x7E, 0x21, 0xEC, 0xA0
    },
    };
    unsigned long seed, random;
    struct prno_parm prno;
    __u64 entropy[4];
    int mode, i;
    mode = check_prng();
    seed = get_tod_clock_fast();
    switch (mode) {
    case PRNG_MODE_TRNG:
    cpacf_trng(core::ptr::null_mut(), 0, (u8 *) &random, sizeof(random));
    break;
    case PRNG_MODE_SHA512:
    cpacf_prno(CPACF_PRNO_SHA512_DRNG_SEED, &prno, core::ptr::null_mut(), 0,
    (u8 *) &seed, sizeof(seed));
    cpacf_prno(CPACF_PRNO_SHA512_DRNG_GEN, &prno, (u8 *) &random,
    sizeof(random), core::ptr::null_mut(), 0);
    break;
    case PRNG_MODE_TDES:
// add entropy
// (unsigned long *) prng.parm_block ^= seed;
    for (i = 0; i < 16; i++) {
    cpacf_kmc(CPACF_KMC_PRNG, prng.parm_block,
    (u8 *) entropy, (u8 *) entropy,
    sizeof(entropy));
    memcpy(prng.parm_block, entropy, sizeof(entropy));
    }
    random = seed;
    cpacf_kmc(CPACF_KMC_PRNG, prng.parm_block, (u8 *) &random,
    (u8 *) &random, sizeof(random));
    break;
    default:
    return -1;
    }
// value = random % limit;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sort_reserved_ranges(res: *mut reserved_range, size: c_ulong) {
    static void sort_reserved_ranges(struct reserved_range *res, unsigned long size)
    {
    struct reserved_range tmp;
    int i, j;
    for (i = 1; i < size; i++) {
    tmp = res[i];
    for (j = i - 1; j >= 0 && res[j].start > tmp.start; j--)
    res[j + 1] = res[j];
    res[j + 1] = tmp;
    }
    }
    static unsigned long iterate_valid_positions(unsigned long size, unsigned long align,
    unsigned long _min, unsigned long _max,
    struct reserved_range *res, size_t res_count,
    bool pos_count, unsigned long find_pos)
    {
    unsigned long start, end, tmp_end, range_pos, pos = 0;
    struct reserved_range *res_end = res + res_count;
    struct reserved_range *skip_res;
    int i;
    align = max(align, 8UL);
    _min = round_up(_min, align);
    for_each_physmem_usable_range(i, &start, &end) {
    if (_min >= end)
    continue;
    start = round_up(start, align);
    if (start >= _max)
    break;
    start = max(_min, start);
    end = min(_max, end);
    while (start + size <= end) {
// skip reserved ranges below the start
    while (res && res.end <= start) {
    res++;
    if (res >= res_end)
    res = core::ptr::null_mut();
    }
    skip_res = core::ptr::null_mut();
    tmp_end = end;
// has intersecting reserved range
    if (res && res.start < end) {
    skip_res = res;
    tmp_end = res.start;
    }
    if (start + size <= tmp_end) {
    range_pos = (tmp_end - start - size) / align + 1;
    if (pos_count) {
    pos += range_pos;
    } else {
    if (range_pos >= find_pos)
    return start + (find_pos - 1) * align;
    find_pos -= range_pos;
    }
    }
    if (!skip_res)
    break;
    start = round_up(skip_res.end, align);
    }
    }
    return pos_count ? pos : 0;
    }
//
// Two types of decompressor memory allocations/reserves are considered
// differently.
//
// "Static" or "single" allocations are done via physmem_alloc_range() and
// physmem_reserve(), and they are listed in physmem_info.reserved[]. Each
// type of "static" allocation can only have one allocation per type and
// cannot have chains.
//
// On the other hand, "dynamic" or "repetitive" allocations are done via
// physmem_alloc_or_die(). These allocations are tightly packed together
// top down from the end of online memory. physmem_alloc_pos represents
// current position where those allocations start.
//
// Functions randomize_within_range() and iterate_valid_positions()
// only consider "dynamic" allocations by never looking above
// physmem_alloc_pos. "Static" allocations, however, are explicitly
// considered by checking the "res" (reserves) array. The first
// reserved_range of a "dynamic" allocation may also be checked along the
// way, but it will always be above the maximum value anyway.
//
    unsigned long randomize_within_range(unsigned long size, unsigned long align,
    unsigned long min, unsigned long max)
    {
    struct reserved_range res[RR_MAX];
    unsigned long max_pos, pos;
    memcpy(res, physmem_info.reserved, sizeof(res));
    sort_reserved_ranges(res, ARRAY_SIZE(res));
    max = min(max, get_physmem_alloc_pos());
    max_pos = iterate_valid_positions(size, align, min, max, res, ARRAY_SIZE(res), true, 0);
    if (!max_pos)
    return 0;
    if (get_random(max_pos, &pos))
    return 0;
    return iterate_valid_positions(size, align, min, max, res, ARRAY_SIZE(res), false, pos + 1);
    }
