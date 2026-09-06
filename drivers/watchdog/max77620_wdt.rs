//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/max77620_wdt.c
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
// Maxim MAX77620 Watchdog Driver
//
// Copyright (C) 2016 NVIDIA CORPORATION. All rights reserved.
// Copyright (C) 2022 Luca Ceresoli
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
// Author: Luca Ceresoli <luca.ceresoli@bootlin.com>
//

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
//
// struct max77620_variant - Data specific to a chip variant
// @reg_onoff_cnfg2:     ONOFF_CNFG2 register offset
// @reg_cnfg_glbl2:      CNFG_GLBL2 register offset
// @reg_cnfg_glbl3:      CNFG_GLBL3 register offset
// @wdtc_mask:           WDTC bit mask in CNFG_GLBL3 (=bits to update to ping the watchdog)
// @bit_wd_rst_wk:       WD_RST_WK bit offset within ONOFF_CNFG2
// @cnfg_glbl2_cfg_bits: configuration bits to enable in CNFG_GLBL2 register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77620_variant {
    pub reg_onoff_cnfg2: u8,
    pub reg_cnfg_glbl2: u8,
    pub reg_cnfg_glbl3: u8,
    pub wdtc_mask: u8,
    pub bit_wd_rst_wk: u8,
    pub cnfg_glbl2_cfg_bits: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77620_wdt {
    pub dev: *mut device,
    pub rmap: *mut regmap,
    pub drv_data: *const max77620_variant,
    pub wdt_dev: watchdog_device,
}

    static const struct max77620_variant max77620_wdt_data = {
    .reg_onoff_cnfg2     = MAX77620_REG_ONOFFCNFG2,
    .reg_cnfg_glbl2      = MAX77620_REG_CNFGGLBL2,
    .reg_cnfg_glbl3      = MAX77620_REG_CNFGGLBL3,
    .wdtc_mask           = MAX77620_WDTC_MASK,
    .bit_wd_rst_wk       = MAX77620_ONOFFCNFG2_WD_RST_WK,
// Set WDT clear in OFF and sleep mode
    .cnfg_glbl2_cfg_bits = MAX77620_WDTSLPC | MAX77620_WDTOFFC,
    };
    static const struct max77620_variant max77714_wdt_data = {
    .reg_onoff_cnfg2     = MAX77714_CNFG2_ONOFF,
    .reg_cnfg_glbl2      = MAX77714_CNFG_GLBL2,
    .reg_cnfg_glbl3      = MAX77714_CNFG_GLBL3,
    .wdtc_mask           = MAX77714_WDTC,
    .bit_wd_rst_wk       = MAX77714_WD_RST_WK,
// Set WDT clear in sleep mode (there is no WDTOFFC on MAX77714)
    .cnfg_glbl2_cfg_bits = MAX77714_WDTSLPC,
    };
#[no_mangle]
unsafe extern "C" fn max77620_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int max77620_wdt_start(struct watchdog_device *wdt_dev)
    {
    struct max77620_wdt *wdt = watchdog_get_drvdata(wdt_dev);
    return regmap_update_bits(wdt.rmap, wdt.drv_data.reg_cnfg_glbl2,
    MAX77620_WDTEN, MAX77620_WDTEN);
    }
#[no_mangle]
unsafe extern "C" fn max77620_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int max77620_wdt_stop(struct watchdog_device *wdt_dev)
    {
    struct max77620_wdt *wdt = watchdog_get_drvdata(wdt_dev);
    return regmap_update_bits(wdt.rmap, wdt.drv_data.reg_cnfg_glbl2,
    MAX77620_WDTEN, 0);
    }
#[no_mangle]
unsafe extern "C" fn max77620_wdt_ping(wdt_dev: *mut watchdog_device) -> c_int {
    static int max77620_wdt_ping(struct watchdog_device *wdt_dev)
    {
    struct max77620_wdt *wdt = watchdog_get_drvdata(wdt_dev);
    return regmap_update_bits(wdt.rmap, wdt.drv_data.reg_cnfg_glbl3,
    wdt.drv_data.wdtc_mask, 0x1);
    }
    static int max77620_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    struct max77620_wdt *wdt = watchdog_get_drvdata(wdt_dev);
    unsigned int wdt_timeout;
    u8 regval;
    int ret;
    switch (timeout) {
    case 0 ... 2:
    regval = MAX77620_TWD_2s;
    wdt_timeout = 2;
    break;
    case 3 ... 16:
    regval = MAX77620_TWD_16s;
    wdt_timeout = 16;
    break;
    case 17 ... 64:
    regval = MAX77620_TWD_64s;
    wdt_timeout = 64;
    break;
    default:
    regval = MAX77620_TWD_128s;
    wdt_timeout = 128;
    break;
    }
//
// "If the value of TWD needs to be changed, clear the system
// watchdog timer first [...], then change the value of TWD."
// (MAX77714 datasheet but applies to MAX77620 too)
//
    ret = regmap_update_bits(wdt.rmap, wdt.drv_data.reg_cnfg_glbl3,
    wdt.drv_data.wdtc_mask, 0x1);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(wdt.rmap, wdt.drv_data.reg_cnfg_glbl2,
    MAX77620_TWD_MASK, regval);
    if (ret < 0)
    return ret;
    wdt_dev.timeout = wdt_timeout;
    return 0;
    }
    static const struct watchdog_info max77620_wdt_info = {
    .identity = "max77620-watchdog",
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops max77620_wdt_ops = {
    .start		= max77620_wdt_start,
    .stop		= max77620_wdt_stop,
    .ping		= max77620_wdt_ping,
    .set_timeout	= max77620_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn max77620_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int max77620_wdt_probe(struct platform_device *pdev)
    {
    const struct platform_device_id *id = platform_get_device_id(pdev);
    struct device *dev = &pdev.dev;
    struct max77620_wdt *wdt;
    struct watchdog_device *wdt_dev;
    unsigned int regval;
    int ret;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.dev = dev;
    wdt.drv_data = (const struct max77620_variant *) id.driver_data;
    wdt.rmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!wdt.rmap) {
    dev_err(wdt.dev, "Failed to get parent regmap\n");
    return -ENODEV;
    }
    wdt_dev = &wdt.wdt_dev;
    wdt_dev.info = &max77620_wdt_info;
    wdt_dev.ops = &max77620_wdt_ops;
    wdt_dev.min_timeout = 2;
    wdt_dev.max_timeout = 128;
    wdt_dev.max_hw_heartbeat_ms = 128 * 1000;
    platform_set_drvdata(pdev, wdt);
// Enable WD_RST_WK - WDT expire results in a restart
    ret = regmap_update_bits(wdt.rmap, wdt.drv_data.reg_onoff_cnfg2,
    wdt.drv_data.bit_wd_rst_wk,
    wdt.drv_data.bit_wd_rst_wk);
    if (ret < 0) {
    dev_err(wdt.dev, "Failed to set WD_RST_WK: %d\n", ret);
    return ret;
    }
// Set the "auto WDT clear" bits available on the chip
    ret = regmap_update_bits(wdt.rmap, wdt.drv_data.reg_cnfg_glbl2,
    wdt.drv_data.cnfg_glbl2_cfg_bits,
    wdt.drv_data.cnfg_glbl2_cfg_bits);
    if (ret < 0) {
    dev_err(wdt.dev, "Failed to set WDT OFF mode: %d\n", ret);
    return ret;
    }
// Check if WDT running and if yes then set flags properly
    ret = regmap_read(wdt.rmap, wdt.drv_data.reg_cnfg_glbl2, &regval);
    if (ret < 0) {
    dev_err(wdt.dev, "Failed to read WDT CFG register: %d\n", ret);
    return ret;
    }
    switch (regval & MAX77620_TWD_MASK) {
    case MAX77620_TWD_2s:
    wdt_dev.timeout = 2;
    break;
    case MAX77620_TWD_16s:
    wdt_dev.timeout = 16;
    break;
    case MAX77620_TWD_64s:
    wdt_dev.timeout = 64;
    break;
    default:
    wdt_dev.timeout = 128;
    break;
    }
    if (regval & MAX77620_WDTEN)
    set_bit(WDOG_HW_RUNNING, &wdt_dev.status);
    watchdog_set_nowayout(wdt_dev, nowayout);
    watchdog_set_drvdata(wdt_dev, wdt);
    watchdog_stop_on_unregister(wdt_dev);
    return devm_watchdog_register_device(dev, wdt_dev);
    }
    static const struct platform_device_id max77620_wdt_devtype[] = {
    { .name = "max77620-watchdog", .driver_data = (kernel_ulong_t)&max77620_wdt_data },
    { .name = "max77714-watchdog", .driver_data = (kernel_ulong_t)&max77714_wdt_data },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max77620_wdt_devtype);
    static struct platform_driver max77620_wdt_driver = {
    .driver	= {
    .name	= "max77620-watchdog",
    },
    .probe	= max77620_wdt_probe,
    .id_table = max77620_wdt_devtype,
    };
    module_platform_driver(max77620_wdt_driver);
    MODULE_DESCRIPTION("Max77620 watchdog timer driver");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started "
    "(default=" __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_AUTHOR("Luca Ceresoli <luca.ceresoli@bootlin.com>");
    MODULE_LICENSE("GPL v2");
