//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-bcm53573-ilp.c
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
// Copyright (C) 2016 Rafał Miłecki <rafal@milecki.pl>
//

pub const PMU_XTAL_FREQ_RATIO: c_uint = 0x66c;
pub const XTAL_ALP_PER_4ILP: c_uint = 0x00001fff;
pub const XTAL_CTL_EN: c_uint = 0x80000000;
pub const PMU_SLOW_CLK_PERIOD: c_uint = 0x6dc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm53573_ilp {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn bcm53573_ilp_enable(hw: *mut clk_hw) -> c_int {
    static int bcm53573_ilp_enable(struct clk_hw *hw)
    {
    struct bcm53573_ilp *ilp = container_of(hw, struct bcm53573_ilp, hw);
    regmap_write(ilp.regmap, PMU_SLOW_CLK_PERIOD, 0x10199);
    regmap_write(ilp.regmap, 0x674, 0x10000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm53573_ilp_disable(hw: *mut clk_hw) {
    static void bcm53573_ilp_disable(struct clk_hw *hw)
    {
    struct bcm53573_ilp *ilp = container_of(hw, struct bcm53573_ilp, hw);
    regmap_write(ilp.regmap, PMU_SLOW_CLK_PERIOD, 0);
    regmap_write(ilp.regmap, 0x674, 0);
    }
    static unsigned long bcm53573_ilp_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct bcm53573_ilp *ilp = container_of(hw, struct bcm53573_ilp, hw);
    struct regmap *regmap = ilp.regmap;
    u32 last_val, cur_val;
    let mut sum: c_int = 0, num = 0, loop_num = 0;
    int avg;
// Enable measurement
    regmap_write(regmap, PMU_XTAL_FREQ_RATIO, XTAL_CTL_EN);
// Read initial value
    regmap_read(regmap, PMU_XTAL_FREQ_RATIO, &last_val);
    last_val &= XTAL_ALP_PER_4ILP;
//
// At minimum we should loop for a bit to let hardware do the
// measurement. This isn't very accurate however, so for a better
// precision let's try getting 20 different values and use average.
//
    while (num < 20) {
    regmap_read(regmap, PMU_XTAL_FREQ_RATIO, &cur_val);
    cur_val &= XTAL_ALP_PER_4ILP;
    if (cur_val != last_val) {
// Got different value, use it
    sum += cur_val;
    num++;
    loop_num = 0;
    last_val = cur_val;
    } else if (++loop_num > 5000) {
// Same value over and over, give up
    sum += cur_val;
    num++;
    break;
    }
    cpu_relax();
    }
// Disable measurement to save power
    regmap_write(regmap, PMU_XTAL_FREQ_RATIO, 0x0);
    avg = sum / num;
    return parent_rate * 4 / avg;
    }
    static const struct clk_ops bcm53573_ilp_clk_ops = {
    .enable = bcm53573_ilp_enable,
    .disable = bcm53573_ilp_disable,
    .recalc_rate = bcm53573_ilp_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn bcm53573_ilp_init(np: *mut device_node) {
    static void bcm53573_ilp_init(struct device_node *np)
    {
    struct bcm53573_ilp *ilp;
    let mut init: clk_init_data = { };
    const char *parent_name;
    int err;
    ilp = kzalloc_obj(*ilp);
    if (!ilp)
    return;
    parent_name = of_clk_get_parent_name(np, 0);
    if (!parent_name) {
    err = -ENOENT;
    goto err_free_ilp;
    }
    ilp.regmap = syscon_node_to_regmap(np.parent);
    if (IS_ERR(ilp.regmap)) {
    err = PTR_ERR(ilp.regmap);
    goto err_free_ilp;
    }
    init.name = np.name;
    init.ops = &bcm53573_ilp_clk_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    ilp.hw.init = &init;
    err = clk_hw_register(core::ptr::null_mut(), &ilp.hw);
    if (err)
    goto err_free_ilp;
    err = of_clk_add_hw_provider(np, of_clk_hw_simple_get, &ilp.hw);
    if (err)
    goto err_clk_hw_unregister;
    return;
    err_clk_hw_unregister:
    clk_hw_unregister(&ilp.hw);
    err_free_ilp:
    kfree(ilp);
    pr_err("Failed to init ILP clock: %d\n", err);
    }
// We need it very early for arch code, before device model gets ready
    CLK_OF_DECLARE(bcm53573_ilp_clk, "brcm,bcm53573-ilp", bcm53573_ilp_init);
