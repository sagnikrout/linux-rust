//! Automatically rewritten from C to Rust
//! Source: lib/math/reciprocal_div.c
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
// For a description of the algorithm please have a look at
// include/linux/reciprocal_div.h
//
#[no_mangle]
pub unsafe extern "C" fn reciprocal_value(d: u32) -> reciprocal_value {
    struct reciprocal_value reciprocal_value(u32 d)
    {
    struct reciprocal_value R;
    u64 m;
    int l;
    l = fls(d - 1);
    m = ((1ULL << 32) * ((1ULL << l) - d));
    do_div(m, d);
    ++m;
    R.m = (u32)m;
    R.sh1 = min(l, 1);
    R.sh2 = max(l - 1, 0);
    return R;
    }
    EXPORT_SYMBOL(reciprocal_value);
#[no_mangle]
pub unsafe extern "C" fn reciprocal_value_adv(d: u32, prec: u8) -> reciprocal_value_adv {
    struct reciprocal_value_adv reciprocal_value_adv(u32 d, u8 prec)
    {
    struct reciprocal_value_adv R;
    u32 l, post_shift;
    u64 mhigh, mlow;
// ceil(log2(d))
    l = fls(d - 1);
// NOTE: mlow/mhigh could overflow u64 when l == 32. This case needs to
// be handled before calling "reciprocal_value_adv", please see the
// comment at include/linux/reciprocal_div.h.
//
    WARN(l == 32,
    "ceil(log2(0x%08x)) == 32, %s doesn't support such divisor",
    d, __func__);
    post_shift = l;
    mlow = 1ULL << (32 + l);
    do_div(mlow, d);
    mhigh = (1ULL << (32 + l)) + (1ULL << (32 + l - prec));
    do_div(mhigh, d);
    for (; post_shift > 0; post_shift--) {
    let mut lo: u64 = mlow >> 1, hi = mhigh >> 1;
    if (lo >= hi)
    break;
    mlow = lo;
    mhigh = hi;
    }
    R.m = (u32)mhigh;
    R.sh = post_shift;
    R.exp = l;
    R.is_wide_m = mhigh > U32_MAX;
    return R;
    }
    EXPORT_SYMBOL(reciprocal_value_adv);
