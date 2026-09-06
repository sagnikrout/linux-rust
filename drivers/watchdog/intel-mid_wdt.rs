//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/intel-mid_wdt.c
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
// intel-mid_wdt: generic Intel MID SCU watchdog driver
//
// Platforms supported so far:
// - Merrifield only
//
// Copyright (C) 2014 Intel Corporation. All rights reserved.
// Contact: David Cohen <david.a.cohen@linux.intel.com>
//

pub const IPC_WATCHDOG: c_uint = 0xf8;
pub const MID_WDT_PRETIMEOUT: c_int = 15;

pub const MID_WDT_TIMEOUT_MAX: c_int = 170;
pub const MID_WDT_DEFAULT_TIMEOUT: c_int = 90;
// SCU watchdog messages
    enum {
    SCU_WATCHDOG_START = 0,
    SCU_WATCHDOG_STOP,
    SCU_WATCHDOG_KEEPALIVE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_wdt {
    pub wd: watchdog_device,
    pub dev: *mut device,
    pub scu: *mut intel_scu_ipc_dev,
}

    static inline int
    wdt_command(struct mid_wdt *mid, int sub, const void *in, size_t inlen, size_t size)
    {
    struct intel_scu_ipc_dev *scu = mid.scu;
    return intel_scu_ipc_dev_command_with_size(scu, IPC_WATCHDOG, sub, in,
    inlen, size, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn wdt_start(wd: *mut watchdog_device) -> c_int {
    static int wdt_start(struct watchdog_device *wd)
    {
    struct mid_wdt *mid = watchdog_get_drvdata(wd);
    int ret, in_size;
    let mut timeout: c_int = wd.timeout;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_wd_start {
    pub pretimeout: u32,
    pub timeout: u32,
    pub }: } ipc_wd_start = { timeout - MID_WDT_PRETIMEOUT, timeout,
//
// SCU expects the input size for watchdog IPC to be 2 which is the
// size of the structure in dwords. SCU IPC normally takes bytes
// but this is a special case where we specify size to be different
// than inlen.
//
    pub 4): in_size = DIV_ROUND_UP(sizeof(ipc_wd_start),,
    ret = wdt_command(mid, SCU_WATCHDOG_START, &ipc_wd_start,
    pub in_size): sizeof(ipc_wd_start),,
    if (ret)
    pub ret): dev_crit(mid->dev, "error starting watchdog: %d\n",,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn wdt_ping(wd: *mut watchdog_device) -> c_int {
    static int wdt_ping(struct watchdog_device *wd)
    {
    pub watchdog_get_drvdata(wd): *mut *mut mid_wdt mid =,
    pub ret: c_int,
    pub 0): ret = wdt_command(mid, SCU_WATCHDOG_KEEPALIVE, NULL, 0,,
    if (ret)
    pub ret): dev_crit(mid->dev, "Error executing keepalive: %d\n",,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn wdt_stop(wd: *mut watchdog_device) -> c_int {
    static int wdt_stop(struct watchdog_device *wd)
    {
    pub watchdog_get_drvdata(wd): *mut *mut mid_wdt mid =,
    pub ret: c_int,
    pub 0): ret = wdt_command(mid, SCU_WATCHDOG_STOP, NULL, 0,,
    if (ret)
    pub ret): dev_crit(mid->dev, "Error stopping watchdog: %d\n",,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn mid_wdt_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mid_wdt_irq(int irq, void *dev_id)
    {
    pub Watchdog"): panic("Kernel,
// This code should not be reached
    pub IRQ_HANDLED: return,
    }
    static const struct watchdog_info mid_wdt_info = {
    .identity = "Intel MID SCU watchdog",
    .options = WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE,
}

    static const struct watchdog_ops mid_wdt_ops = {
    .owner = THIS_MODULE,
    .start = wdt_start,
    .stop = wdt_stop,
    .ping = wdt_ping,
    };
#[no_mangle]
unsafe extern "C" fn mid_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int mid_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdt_dev;
    struct intel_mid_wdt_pdata *pdata = dev_get_platdata(dev);
    struct mid_wdt *mid;
    int ret;
    if (!pdata) {
    dev_err(dev, "missing platform data\n");
    return -EINVAL;
    }
    if (pdata.probe) {
    ret = pdata.probe(pdev);
    if (ret)
    return ret;
    }
    mid = devm_kzalloc(dev, sizeof(*mid), GFP_KERNEL);
    if (!mid)
    return -ENOMEM;
    mid.dev = dev;
    wdt_dev = &mid.wd;
    wdt_dev.info = &mid_wdt_info;
    wdt_dev.ops = &mid_wdt_ops;
    wdt_dev.min_timeout = MID_WDT_TIMEOUT_MIN;
    wdt_dev.max_timeout = MID_WDT_TIMEOUT_MAX;
    wdt_dev.timeout = MID_WDT_DEFAULT_TIMEOUT;
    wdt_dev.parent = dev;
    watchdog_set_nowayout(wdt_dev, WATCHDOG_NOWAYOUT);
    watchdog_set_drvdata(wdt_dev, mid);
    mid.scu = devm_intel_scu_ipc_dev_get(dev);
    if (!mid.scu)
    return -EPROBE_DEFER;
    ret = devm_request_irq(dev, pdata.irq, mid_wdt_irq,
    IRQF_SHARED | IRQF_NO_SUSPEND, "watchdog",
    wdt_dev);
    if (ret)
    return ret;
//
// The firmware followed by U-Boot leaves the watchdog running
// with the default threshold which may vary. When we get here
// we should make a decision to prevent any side effects before
// user space daemon will take care of it. The best option,
// taking into consideration that there is no way to read values
// back from hardware, is to enforce watchdog being run with
// deterministic values.
//
    ret = wdt_start(wdt_dev);
    if (ret)
    return ret;
// Make sure the watchdog is serviced
    set_bit(WDOG_HW_RUNNING, &wdt_dev.status);
    ret = devm_watchdog_register_device(dev, wdt_dev);
    if (ret)
    return ret;
    dev_info(dev, "Intel MID watchdog device probed\n");
    return 0;
    }
    static struct platform_driver mid_wdt_driver = {
    .probe		= mid_wdt_probe,
    .driver		= {
    .name	= "intel_mid_wdt",
    },
    };
    module_platform_driver(mid_wdt_driver);
    MODULE_AUTHOR("David Cohen <david.a.cohen@linux.intel.com>");
    MODULE_DESCRIPTION("Watchdog Driver for Intel MID platform");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:intel_mid_wdt");
