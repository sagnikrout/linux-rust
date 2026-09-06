//! Automatically rewritten from C to Rust
//! Source: drivers/soc/mediatek/mtk-infracfg.c
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
// Copyright (c) 2015 Pengutronix, Sascha Hauer <kernel@pengutronix.de>
//

pub const MTK_POLL_DELAY_US: c_int = 10;

//
// mtk_infracfg_set_bus_protection - enable bus protection
// @infracfg: The infracfg regmap
// @mask: The mask containing the protection bits to be enabled.
// @reg_update: The boolean flag determines to set the protection bits
// by regmap_update_bits with enable register(PROTECTEN) or
// by regmap_write with set register(PROTECTEN_SET).
//
// This function enables the bus protection bits for disabled power
// domains so that the system does not hang when some unit accesses the
// bus while in power down.
//
    int mtk_infracfg_set_bus_protection(struct regmap *infracfg, u32 mask,
    bool reg_update)
    {
    u32 val;
    int ret;
    if (reg_update)
    regmap_update_bits(infracfg, INFRA_TOPAXI_PROTECTEN, mask,
    mask);
    else
    regmap_write(infracfg, INFRA_TOPAXI_PROTECTEN_SET, mask);
    ret = regmap_read_poll_timeout(infracfg, INFRA_TOPAXI_PROTECTSTA1,
    val, (val & mask) == mask,
    MTK_POLL_DELAY_US, MTK_POLL_TIMEOUT);
    return ret;
    }
//
// mtk_infracfg_clear_bus_protection - disable bus protection
// @infracfg: The infracfg regmap
// @mask: The mask containing the protection bits to be disabled.
// @reg_update: The boolean flag determines to clear the protection bits
// by regmap_update_bits with enable register(PROTECTEN) or
// by regmap_write with clear register(PROTECTEN_CLR).
//
// This function disables the bus protection bits previously enabled with
// mtk_infracfg_set_bus_protection.
//
    int mtk_infracfg_clear_bus_protection(struct regmap *infracfg, u32 mask,
    bool reg_update)
    {
    int ret;
    u32 val;
    if (reg_update)
    regmap_update_bits(infracfg, INFRA_TOPAXI_PROTECTEN, mask, 0);
    else
    regmap_write(infracfg, INFRA_TOPAXI_PROTECTEN_CLR, mask);
    ret = regmap_read_poll_timeout(infracfg, INFRA_TOPAXI_PROTECTSTA1,
    val, !(val & mask),
    MTK_POLL_DELAY_US, MTK_POLL_TIMEOUT);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_infracfg_init() -> int __init {
    static int __init mtk_infracfg_init(void)
    {
    struct regmap *infracfg;
//
// MT8192 has an experimental path to route GPU traffic to the DSU's
// Accelerator Coherency Port, which is inadvertently enabled by
// default. It turns out not to work, so disable it to prevent spurious
// GPU faults.
//
    infracfg = syscon_regmap_lookup_by_compatible("mediatek,mt8192-infracfg");
    if (!IS_ERR(infracfg))
    regmap_set_bits(infracfg, MT8192_INFRA_CTRL,
    MT8192_INFRA_CTRL_DISABLE_MFG2ACP);
    return 0;
    }
    postcore_initcall(mtk_infracfg_init);
