//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-utils.c
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
// Copyright (c) 2018, NVIDIA CORPORATION.  All rights reserved.
//

    int div_frac_get(unsigned long rate, unsigned parent_rate, u8 width,
    u8 frac_width, u8 flags)
    {
    let mut divider_ux1: u64 = parent_rate;
    int mul;
    if (!rate)
    return 0;
    mul = 1 << frac_width;
    if (!(flags & TEGRA_DIVIDER_INT))
    divider_ux1 *= mul;
    if (flags & TEGRA_DIVIDER_ROUND_UP)
    divider_ux1 += rate - 1;
    do_div(divider_ux1, rate);
    if (flags & TEGRA_DIVIDER_INT)
    divider_ux1 *= mul;
    if (divider_ux1 < mul)
    return 0;
    divider_ux1 -= mul;
    if (divider_ux1 > div_mask(width))
    return div_mask(width);
    return divider_ux1;
    }
