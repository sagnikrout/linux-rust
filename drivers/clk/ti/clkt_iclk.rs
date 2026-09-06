//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/clkt_iclk.c
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
// OMAP2/3 interface clock control
//
// Copyright (C) 2011 Nokia Corporation
// Paul Walmsley
//

// Register offsets
pub const OMAP24XX_CM_FCLKEN2: c_uint = 0x04;
pub const CM_AUTOIDLE: c_uint = 0x30;
pub const CM_ICLKEN: c_uint = 0x10;
pub const CM_IDLEST: c_uint = 0x20;
pub const OMAP24XX_CM_IDLEST_VAL: c_int = 0;
// Private functions
// XXX
#[no_mangle]
pub unsafe extern "C" fn omap2_clkt_iclk_allow_idle(clk: *mut clk_hw_omap) {
    void omap2_clkt_iclk_allow_idle(struct clk_hw_omap *clk)
    {
    u32 v;
    struct clk_omap_reg r;
    memcpy(&r, &clk.enable_reg, sizeof(r));
    r.offset ^= (CM_AUTOIDLE ^ CM_ICLKEN);
    v = ti_clk_ll_ops.clk_readl(&r);
    v |= (1 << clk.enable_bit);
    ti_clk_ll_ops.clk_writel(v, &r);
    }
// XXX
#[no_mangle]
pub unsafe extern "C" fn omap2_clkt_iclk_deny_idle(clk: *mut clk_hw_omap) {
    void omap2_clkt_iclk_deny_idle(struct clk_hw_omap *clk)
    {
    u32 v;
    struct clk_omap_reg r;
    memcpy(&r, &clk.enable_reg, sizeof(r));
    r.offset ^= (CM_AUTOIDLE ^ CM_ICLKEN);
    v = ti_clk_ll_ops.clk_readl(&r);
    v &= ~(1 << clk.enable_bit);
    ti_clk_ll_ops.clk_writel(v, &r);
    }
//
// omap2430_clk_i2chs_find_idlest - return CM_IDLEST info for 2430 I2CHS
// @clk: struct clk * being enabled
// @idlest_reg: void __iomem ** to store CM_IDLEST reg address into
// @idlest_bit: pointer to a u8 to store the CM_IDLEST bit shift into
// @idlest_val: pointer to a u8 to store the CM_IDLEST indicator
//
// OMAP2430 I2CHS CM_IDLEST bits are in CM_IDLEST1_CORE, but the
// CM_*CLKEN bits are in CM_{I,F}CLKEN2_CORE.  This custom function
// passes back the correct CM_IDLEST register address for I2CHS
// modules.  No return value.
//
    static void omap2430_clk_i2chs_find_idlest(struct clk_hw_omap *clk,
    struct clk_omap_reg *idlest_reg,
    u8 *idlest_bit,
    u8 *idlest_val)
    {
    memcpy(idlest_reg, &clk.enable_reg, sizeof(*idlest_reg));
    idlest_reg.offset ^= (OMAP24XX_CM_FCLKEN2 ^ CM_IDLEST);
// idlest_bit = clk->enable_bit;
// idlest_val = OMAP24XX_CM_IDLEST_VAL;
    }
// Public data
    const struct clk_hw_omap_ops clkhwops_iclk = {
    .allow_idle	= omap2_clkt_iclk_allow_idle,
    .deny_idle	= omap2_clkt_iclk_deny_idle,
    };
    const struct clk_hw_omap_ops clkhwops_iclk_wait = {
    .allow_idle	= omap2_clkt_iclk_allow_idle,
    .deny_idle	= omap2_clkt_iclk_deny_idle,
    .find_idlest	= omap2_clk_dflt_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
// 2430 I2CHS has non-standard IDLEST register
    const struct clk_hw_omap_ops clkhwops_omap2430_i2chs_wait = {
    .find_idlest	= omap2430_clk_i2chs_find_idlest,
    .find_companion	= omap2_clk_dflt_find_companion,
    };
