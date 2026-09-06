//! Automatically rewritten from C to Rust
//! Source: drivers/soc/gemini/soc-gemini.c
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
// Copyright (C) 2017 Linaro Ltd.
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2, as
// published by the Free Software Foundation.
//

pub const GLOBAL_WORD_ID: c_uint = 0x00;
pub const GEMINI_GLOBAL_ARB1_CTRL: c_uint = 0x2c;

pub const GEMINI_ARB1_BURST_SHIFT: c_int = 16;
// These all define the priority on the BUS2 backplane

pub const GEMINI_DEFAULT_BURST_SIZE: c_uint = 0x20;

    GEMINI_ARB1_GMAC1_HIGH_PRIO)
#[no_mangle]
unsafe extern "C" fn gemini_soc_init() -> int __init {
    static int __init gemini_soc_init(void)
    {
    struct regmap *map;
    u32 rev;
    u32 val;
    int ret;
// Multiplatform guard, only proceed on Gemini
    if (!of_machine_is_compatible("cortina,gemini"))
    return 0;
    map = syscon_regmap_lookup_by_compatible("cortina,gemini-syscon");
    if (IS_ERR(map))
    return PTR_ERR(map);
    ret = regmap_read(map, GLOBAL_WORD_ID, &rev);
    if (ret)
    return ret;
    val = (GEMINI_DEFAULT_BURST_SIZE << GEMINI_ARB1_BURST_SHIFT) |
    GEMINI_DEFAULT_PRIO;
// Set up system arbitration
    regmap_update_bits(map,
    GEMINI_GLOBAL_ARB1_CTRL,
    GEMINI_ARB1_BURST_MASK | GEMINI_ARB1_PRIO_MASK,
    val);
    pr_info("Gemini SoC %04x revision %02x, set arbitration %08x\n",
    rev >> 8, rev & 0xff, val);
    return 0;
    }
    subsys_initcall(gemini_soc_init);
