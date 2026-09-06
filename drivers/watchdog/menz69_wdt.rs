//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/menz69_wdt.c
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
// Watchdog driver for the MEN z069 IP-Core
//
// Copyright (C) 2018 Johannes Thumshirn <jth@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct men_z069_drv {
    pub wdt: watchdog_device,
    pub base: *mut void __iomem,
    pub mem: *mut resource,
}

pub const MEN_Z069_WTR: c_uint = 0x10;

pub const MEN_Z069_WTR_WDET_MASK: c_uint = 0x7fff;
pub const MEN_Z069_WVR: c_uint = 0x14;

pub const MEN_Z069_WDT_COUNTER_MIN: c_int = 1;
pub const MEN_Z069_WDT_COUNTER_MAX: c_uint = 0x7fff;
pub const MEN_Z069_DEFAULT_TIMEOUT: c_int = 30;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn men_z069_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int men_z069_wdt_start(struct watchdog_device *wdt)
    {
    struct men_z069_drv *drv = watchdog_get_drvdata(wdt);
    u16 val;
    val = readw(drv.base + MEN_Z069_WTR);
    val |= MEN_Z069_WTR_WDEN;
    writew(val, drv.base + MEN_Z069_WTR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn men_z069_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int men_z069_wdt_stop(struct watchdog_device *wdt)
    {
    struct men_z069_drv *drv = watchdog_get_drvdata(wdt);
    u16 val;
    val = readw(drv.base + MEN_Z069_WTR);
    val &= ~MEN_Z069_WTR_WDEN;
    writew(val, drv.base + MEN_Z069_WTR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn men_z069_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int men_z069_wdt_ping(struct watchdog_device *wdt)
    {
    struct men_z069_drv *drv = watchdog_get_drvdata(wdt);
    u16 val;
// The watchdog trigger value toggles between 0x5555 and 0xaaaa
    val = readw(drv.base + MEN_Z069_WVR);
    val ^= 0xffff;
    writew(val, drv.base + MEN_Z069_WVR);
    return 0;
    }
    static int men_z069_wdt_set_timeout(struct watchdog_device *wdt,
    unsigned int timeout)
    {
    struct men_z069_drv *drv = watchdog_get_drvdata(wdt);
    u16 reg, val, ena;
    wdt.timeout = timeout;
    val = timeout * MEN_Z069_TIMER_FREQ;
    reg = readw(drv.base + MEN_Z069_WTR);
    ena = reg & MEN_Z069_WTR_WDEN;
    reg = ena | val;
    writew(reg, drv.base + MEN_Z069_WTR);
    return 0;
    }
    static const struct watchdog_info men_z069_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "MEN z069 Watchdog",
    };
    static const struct watchdog_ops men_z069_ops = {
    .owner = THIS_MODULE,
    .start = men_z069_wdt_start,
    .stop = men_z069_wdt_stop,
    .ping = men_z069_wdt_ping,
    .set_timeout = men_z069_wdt_set_timeout,
    };
    static int men_z069_probe(struct mcb_device *dev,
    const struct mcb_device_id *id)
    {
    struct men_z069_drv *drv;
    struct resource *mem;
    drv = devm_kzalloc(&dev.dev, sizeof(struct men_z069_drv), GFP_KERNEL);
    if (!drv)
    return -ENOMEM;
    mem = mcb_request_mem(dev, "z069-wdt");
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    drv.base = devm_ioremap(&dev.dev, mem.start, resource_size(mem));
    if (drv.base == core::ptr::null_mut())
    goto release_mem;
    drv.mem = mem;
    drv.wdt.info = &men_z069_info;
    drv.wdt.ops = &men_z069_ops;
    drv.wdt.timeout = MEN_Z069_DEFAULT_TIMEOUT;
    drv.wdt.min_timeout = 1;
    drv.wdt.max_timeout = MEN_Z069_WDT_COUNTER_MAX / MEN_Z069_TIMER_FREQ;
    watchdog_init_timeout(&drv.wdt, 0, &dev.dev);
    watchdog_set_nowayout(&drv.wdt, nowayout);
    watchdog_set_drvdata(&drv.wdt, drv);
    drv.wdt.parent = &dev.dev;
    mcb_set_drvdata(dev, drv);
    return watchdog_register_device(&drv.wdt);
    release_mem:
    mcb_release_mem(mem);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn men_z069_remove(dev: *mut mcb_device) {
    static void men_z069_remove(struct mcb_device *dev)
    {
    struct men_z069_drv *drv = mcb_get_drvdata(dev);
    watchdog_unregister_device(&drv.wdt);
    mcb_release_mem(drv.mem);
    }
    static const struct mcb_device_id men_z069_ids[] = {
    { .device = 0x45 },
    { }
    };
    MODULE_DEVICE_TABLE(mcb, men_z069_ids);
    static struct mcb_driver men_z069_driver = {
    .driver = {
    .name = "z069-wdt",
    },
    .probe = men_z069_probe,
    .remove = men_z069_remove,
    .id_table = men_z069_ids,
    };
    module_mcb_driver(men_z069_driver);
    MODULE_AUTHOR("Johannes Thumshirn <jth@kernel.org>");
    MODULE_DESCRIPTION("Watchdog driver for the MEN z069 IP-Core");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MCB");
