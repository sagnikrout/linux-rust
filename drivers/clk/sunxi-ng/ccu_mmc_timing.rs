//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi-ng/ccu_mmc_timing.c
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
// Copyright (c) 2017 Chen-Yu Tsai. All rights reserved.
//

//
// sunxi_ccu_set_mmc_timing_mode - Configure the MMC clock timing mode
// @clk: clock to be configured
// @new_mode: true for new timing mode introduced in A83T and later
//
// Return: %0 on success, %-ENOTSUPP if the clock does not support
// switching modes.
//
#[no_mangle]
pub unsafe extern "C" fn sunxi_ccu_set_mmc_timing_mode(clk: *mut clk, new_mode: bool) -> c_int {
    int sunxi_ccu_set_mmc_timing_mode(struct clk *clk, bool new_mode)
    {
    struct clk_hw *hw = __clk_get_hw(clk);
    struct ccu_common *cm = hw_to_ccu_common(hw);
    unsigned long flags;
    u32 val;
    if (!(cm.features & CCU_FEATURE_MMC_TIMING_SWITCH))
    return -ENOTSUPP;
    spin_lock_irqsave(cm.lock, flags);
    val = readl(cm.base + cm.reg);
    if (new_mode)
    val |= CCU_MMC_NEW_TIMING_MODE;
    else
    val &= ~CCU_MMC_NEW_TIMING_MODE;
    writel(val, cm.base + cm.reg);
    spin_unlock_irqrestore(cm.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(sunxi_ccu_set_mmc_timing_mode);
//
// sunxi_ccu_get_mmc_timing_mode: Get the current MMC clock timing mode
// @clk: clock to query
//
// Return: %0 if the clock is in old timing mode, > %0 if it is in
// new timing mode, and %-ENOTSUPP if the clock does not support
// this function.
//
#[no_mangle]
pub unsafe extern "C" fn sunxi_ccu_get_mmc_timing_mode(clk: *mut clk) -> c_int {
    int sunxi_ccu_get_mmc_timing_mode(struct clk *clk)
    {
    struct clk_hw *hw = __clk_get_hw(clk);
    struct ccu_common *cm = hw_to_ccu_common(hw);
    if (!(cm.features & CCU_FEATURE_MMC_TIMING_SWITCH))
    return -ENOTSUPP;
    return !!(readl(cm.base + cm.reg) & CCU_MMC_NEW_TIMING_MODE);
    }
    EXPORT_SYMBOL_GPL(sunxi_ccu_get_mmc_timing_mode);
