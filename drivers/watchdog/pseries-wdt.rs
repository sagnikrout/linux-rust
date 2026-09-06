//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/pseries-wdt.c
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
// Copyright (c) 2022 International Business Machines, Inc.
//

    static const unsigned long pseries_wdt_action[] = {
    [0] = PSERIES_WDTF_ACTION_HARD_POWEROFF,
    [1] = PSERIES_WDTF_ACTION_HARD_RESTART,
    [2] = PSERIES_WDTF_ACTION_DUMP_RESTART,
    };
pub const WATCHDOG_ACTION: c_int = 1;
    let mut action: static unsigned int = WATCHDOG_ACTION;
    module_param(action, uint, 0444);
    MODULE_PARM_DESC(action, "Action taken when watchdog expires (default="
    __MODULE_STRING(WATCHDOG_ACTION) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0444);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
pub const WATCHDOG_TIMEOUT: c_int = 60;
    let mut timeout: static unsigned int = WATCHDOG_TIMEOUT;
    module_param(timeout, uint, 0444);
    MODULE_PARM_DESC(timeout, "Initial watchdog timeout in seconds (default="
    __MODULE_STRING(WATCHDOG_TIMEOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_wdt {
    pub wd: watchdog_device,
    pub action: c_ulong,
    pub /: *mut *mut unsigned long num; / Watchdog numbers are 1-based,
}

#[no_mangle]
unsafe extern "C" fn pseries_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int pseries_wdt_start(struct watchdog_device *wdd)
    {
    struct pseries_wdt *pw = watchdog_get_drvdata(wdd);
    struct device *dev = wdd.parent;
    unsigned long flags, msecs;
    long rc;
    flags = pw.action | PSERIES_WDTF_OP_START;
    msecs = wdd.timeout * MSEC_PER_SEC;
    rc = plpar_hcall_norets(H_WATCHDOG, flags, pw.num, msecs);
    if (rc != H_SUCCESS) {
    dev_crit(dev, "H_WATCHDOG: %ld: failed to start timer %lu",
    rc, pw.num);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int pseries_wdt_stop(struct watchdog_device *wdd)
    {
    struct pseries_wdt *pw = watchdog_get_drvdata(wdd);
    struct device *dev = wdd.parent;
    long rc;
    rc = plpar_hcall_norets(H_WATCHDOG, PSERIES_WDTF_OP_STOP, pw.num);
    if (rc != H_SUCCESS && rc != H_NOOP) {
    dev_crit(dev, "H_WATCHDOG: %ld: failed to stop timer %lu",
    rc, pw.num);
    return -EIO;
    }
    return 0;
    }
    static struct watchdog_info pseries_wdt_info = {
    .identity = DRV_NAME,
    .options = WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE | WDIOF_SETTIMEOUT
    | WDIOF_PRETIMEOUT,
    };
    static const struct watchdog_ops pseries_wdt_ops = {
    .owner = THIS_MODULE,
    .start = pseries_wdt_start,
    .stop = pseries_wdt_stop,
    };
#[no_mangle]
unsafe extern "C" fn pseries_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int pseries_wdt_probe(struct platform_device *pdev)
    {
    unsigned long ret[PLPAR_HCALL_BUFSIZE] = { 0 };
    struct pseries_wdt *pw;
    unsigned long cap;
    long msecs, rc;
    int err;
    rc = plpar_hcall(H_WATCHDOG, ret, PSERIES_WDTF_OP_QUERY);
    if (rc == H_FUNCTION)
    return -ENODEV;
    if (rc != H_SUCCESS)
    return -EIO;
    cap = ret[0];
    pw = devm_kzalloc(&pdev.dev, sizeof(*pw), GFP_KERNEL);
    if (!pw)
    return -ENOMEM;
//
// Assume watchdogNumber 1 for now.  If we ever support
// multiple timers we will need to devise a way to choose a
// distinct watchdogNumber for each platform device at device
// registration time.
//
    pw.num = 1;
    if (PSERIES_WDTQ_MAX_NUMBER(cap) < pw.num)
    return -ENODEV;
    if (action >= ARRAY_SIZE(pseries_wdt_action))
    return -EINVAL;
    pw.action = pseries_wdt_action[action];
    pw.wd.parent = &pdev.dev;
    pw.wd.info = &pseries_wdt_info;
    pw.wd.ops = &pseries_wdt_ops;
    msecs = PSERIES_WDTQ_MIN_TIMEOUT(cap);
    pw.wd.min_timeout = DIV_ROUND_UP(msecs, MSEC_PER_SEC);
    pw.wd.max_timeout = UINT_MAX / 1000;	/* from linux/watchdog.h */
    pw.wd.timeout = timeout;
    if (watchdog_init_timeout(&pw.wd, 0, core::ptr::null_mut()))
    return -EINVAL;
    watchdog_set_nowayout(&pw.wd, nowayout);
    watchdog_stop_on_reboot(&pw.wd);
    watchdog_stop_on_unregister(&pw.wd);
    watchdog_set_drvdata(&pw.wd, pw);
    err = devm_watchdog_register_device(&pdev.dev, &pw.wd);
    if (err)
    return err;
    platform_set_drvdata(pdev, &pw.wd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_wdt_suspend(pdev: *mut platform_device, state: pm_message_t) -> c_int {
    static int pseries_wdt_suspend(struct platform_device *pdev, pm_message_t state)
    {
    struct watchdog_device *wd = platform_get_drvdata(pdev);
    if (watchdog_active(wd))
    return pseries_wdt_stop(wd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_wdt_resume(pdev: *mut platform_device) -> c_int {
    static int pseries_wdt_resume(struct platform_device *pdev)
    {
    struct watchdog_device *wd = platform_get_drvdata(pdev);
    if (watchdog_active(wd))
    return pseries_wdt_start(wd);
    return 0;
    }
    static const struct platform_device_id pseries_wdt_id[] = {
    { .name = "pseries-wdt" },
    {}
    };
    MODULE_DEVICE_TABLE(platform, pseries_wdt_id);
    static struct platform_driver pseries_wdt_driver = {
    .driver = {
    .name = DRV_NAME,
    },
    .id_table = pseries_wdt_id,
    .probe = pseries_wdt_probe,
    .resume = pseries_wdt_resume,
    .suspend = pseries_wdt_suspend,
    };
    module_platform_driver(pseries_wdt_driver);
    MODULE_AUTHOR("Alexey Kardashevskiy");
    MODULE_AUTHOR("Scott Cheloha");
    MODULE_DESCRIPTION("POWER Architecture Platform Watchdog Driver");
    MODULE_LICENSE("GPL");
