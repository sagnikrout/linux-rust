//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-fixed-rate.c
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
// Copyright (C) 2016 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//

    struct clk_hw *uniphier_clk_register_fixed_rate(struct device *dev,
    const char *name,
    const struct uniphier_clk_fixed_rate_data *data)
    {
    struct clk_fixed_rate *fixed;
    struct clk_init_data init;
    int ret;
// allocate fixed-rate clock
    fixed = devm_kzalloc(dev, sizeof(*fixed), GFP_KERNEL);
    if (!fixed)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_fixed_rate_ops;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    fixed.fixed_rate = data.fixed_rate;
    fixed.hw.init = &init;
    ret = devm_clk_hw_register(dev, &fixed.hw);
    if (ret)
    return ERR_PTR(ret);
    return &fixed.hw;
    }
