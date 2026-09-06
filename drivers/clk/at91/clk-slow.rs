//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-slow.c
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
// drivers/clk/at91/clk-slow.c
//
// Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sam9260_slow {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn clk_sam9260_slow_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_sam9260_slow_get_parent(struct clk_hw *hw)
    {
    struct clk_sam9260_slow *slowck = to_clk_sam9260_slow(hw);
    unsigned int status;
    regmap_read(slowck.regmap, AT91_PMC_SR, &status);
    return status & AT91_PMC_OSCSEL ? 1 : 0;
    }
    static const struct clk_ops sam9260_slow_ops = {
    .get_parent = clk_sam9260_slow_get_parent,
    };
    struct clk_hw * __init
    at91_clk_register_sam9260_slow(struct regmap *regmap,
    const char *name,
    const char **parent_names,
    int num_parents)
    {
    struct clk_sam9260_slow *slowck;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    if (!name)
    return ERR_PTR(-EINVAL);
    if (!parent_names || !num_parents)
    return ERR_PTR(-EINVAL);
    slowck = kzalloc_obj(*slowck);
    if (!slowck)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &sam9260_slow_ops;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = 0;
    slowck.hw.init = &init;
    slowck.regmap = regmap;
    hw = &slowck.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &slowck.hw);
    if (ret) {
    kfree(slowck);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
