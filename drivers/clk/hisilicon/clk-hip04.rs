//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clk-hip04.c
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
//
// Hisilicon HiP04 clock driver
//
// Copyright (c) 2013-2014 Hisilicon Limited.
// Copyright (c) 2013-2014 Linaro Limited.
//
// Author: Haojian Zhuang <haojian.zhuang@linaro.org>
//

// fixed rate clocks
    static struct hisi_fixed_rate_clock hip04_fixed_rate_clks[] __initdata = {
    { HIP04_OSC50M,   "osc50m",   core::ptr::null_mut(), 0, 50000000, },
    { HIP04_CLK_50M,  "clk50m",   core::ptr::null_mut(), 0, 50000000, },
    { HIP04_CLK_168M, "clk168m",  core::ptr::null_mut(), 0, 168750000, },
    };
#[no_mangle]
unsafe extern "C" fn hip04_clk_init(np: *mut device_node) -> void __init {
    static void __init hip04_clk_init(struct device_node *np)
    {
    struct hisi_clock_data *clk_data;
    clk_data = hisi_clk_init(np, HIP04_NR_CLKS);
    if (!clk_data)
    return;
    hisi_clk_register_fixed_rate(hip04_fixed_rate_clks,
    ARRAY_SIZE(hip04_fixed_rate_clks),
    clk_data);
    }
    CLK_OF_DECLARE(hip04_clk, "hisilicon,hip04-clock", hip04_clk_init);
