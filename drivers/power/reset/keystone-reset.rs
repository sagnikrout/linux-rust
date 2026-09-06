//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/keystone-reset.c
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
// TI keystone reboot driver
//
// Copyright (C) 2014 Texas Instruments Incorporated. https://www.ti.com
//
// Author: Ivan Khoronzhuk <ivan.khoronzhuk@ti.com>
//

pub const RSCTRL_RG: c_uint = 0x4;
pub const RSCFG_RG: c_uint = 0x8;
pub const RSISO_RG: c_uint = 0xc;
pub const RSCTRL_KEY_MASK: c_uint = 0x0000ffff;

pub const RSCTRL_KEY: c_uint = 0x5a69;
pub const RSMUX_OMODE_MASK: c_uint = 0xe;
pub const RSMUX_OMODE_RESET_ON: c_uint = 0xa;
pub const RSMUX_OMODE_RESET_OFF: c_uint = 0x0;
pub const RSMUX_LOCK_SET: c_uint = 0x1;
pub const RSCFG_RSTYPE_SOFT: c_uint = 0x300f;
pub const RSCFG_RSTYPE_HARD: c_uint = 0x0;
pub const WDT_MUX_NUMBER: c_uint = 0x4;
    static int rspll_offset;
    static struct regmap *pllctrl_regs;
//
// rsctrl_enable_rspll_write - enable access to RSCTRL, RSCFG
// To be able to access to RSCTRL, RSCFG registers
// we have to write a key before
//
#[no_mangle]
pub unsafe extern "C" fn rsctrl_enable_rspll_write() -> c_int {
    static inline int rsctrl_enable_rspll_write(void)
    {
    return regmap_update_bits(pllctrl_regs, rspll_offset + RSCTRL_RG,
    RSCTRL_KEY_MASK, RSCTRL_KEY);
    }
#[no_mangle]
unsafe extern "C" fn rsctrl_restart_handler(data: *mut sys_off_data) -> c_int {
    static int rsctrl_restart_handler(struct sys_off_data *data)
    {
// enable write access to RSTCTRL
    rsctrl_enable_rspll_write();
// reset the SOC
    regmap_update_bits(pllctrl_regs, rspll_offset + RSCTRL_RG,
    RSCTRL_RESET_MASK, 0);
    return NOTIFY_DONE;
    }
    static const struct of_device_id rsctrl_of_match[] = {
    {.compatible = "ti,keystone-reset", },
    {},
    };
    MODULE_DEVICE_TABLE(of, rsctrl_of_match);
#[no_mangle]
unsafe extern "C" fn rsctrl_probe(pdev: *mut platform_device) -> c_int {
    static int rsctrl_probe(struct platform_device *pdev)
    {
    int i;
    int ret;
    u32 val;
    unsigned int rg;
    u32 rsmux_offset;
    struct regmap *devctrl_regs;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    if (!np)
    return -ENODEV;
// get regmaps
    pllctrl_regs = syscon_regmap_lookup_by_phandle_args(np, "ti,syscon-pll",
    1, &rspll_offset);
    if (IS_ERR(pllctrl_regs))
    return PTR_ERR(pllctrl_regs);
    devctrl_regs = syscon_regmap_lookup_by_phandle_args(np, "ti,syscon-dev",
    1, &rsmux_offset);
    if (IS_ERR(devctrl_regs))
    return PTR_ERR(devctrl_regs);
// set soft/hard reset
    val = of_property_read_bool(np, "ti,soft-reset");
    val = val ? RSCFG_RSTYPE_SOFT : RSCFG_RSTYPE_HARD;
    ret = rsctrl_enable_rspll_write();
    if (ret)
    return ret;
    ret = regmap_write(pllctrl_regs, rspll_offset + RSCFG_RG, val);
    if (ret)
    return ret;
// disable a reset isolation for all module clocks
    ret = regmap_write(pllctrl_regs, rspll_offset + RSISO_RG, 0);
    if (ret)
    return ret;
// enable a reset for watchdogs from wdt-list
    for (i = 0; i < WDT_MUX_NUMBER; i++) {
    ret = of_property_read_u32_index(np, "ti,wdt-list", i, &val);
    if (ret == -EOVERFLOW && !i) {
    dev_err(dev, "ti,wdt-list property has to contain at"
    "least one entry\n");
    return -EINVAL;
    } else if (ret) {
    break;
    }
    if (val >= WDT_MUX_NUMBER) {
    dev_err(dev, "ti,wdt-list property can contain "
    "only numbers < 4\n");
    return -EINVAL;
    }
    rg = rsmux_offset + val * 4;
    ret = regmap_update_bits(devctrl_regs, rg, RSMUX_OMODE_MASK,
    RSMUX_OMODE_RESET_ON |
    RSMUX_LOCK_SET);
    if (ret)
    return ret;
    }
    ret = devm_register_sys_off_handler(dev, SYS_OFF_MODE_RESTART, 128,
    rsctrl_restart_handler, core::ptr::null_mut());
    if (ret)
    dev_err(dev, "cannot register restart handler (err=%d)\n", ret);
    return ret;
    }
    static struct platform_driver rsctrl_driver = {
    .probe = rsctrl_probe,
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = rsctrl_of_match,
    },
    };
    module_platform_driver(rsctrl_driver);
    MODULE_AUTHOR("Ivan Khoronzhuk <ivan.khoronzhuk@ti.com>");
    MODULE_DESCRIPTION("Texas Instruments keystone reset driver");
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
