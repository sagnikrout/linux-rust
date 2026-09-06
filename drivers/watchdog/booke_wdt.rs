//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/booke_wdt.c
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
// Watchdog timer for PowerPC Book-E systems
//
// Author: Matthew McClintock
// Maintainer: Kumar Gala <galak@kernel.crashing.org>
//
// Copyright 2005, 2008, 2010-2011 Freescale Semiconductor Inc.
//

// If the kernel parameter wdt=1, the watchdog will be enabled at boot.
// Also, the wdt_period sets the watchdog timer period timeout.
// For E500 cpus the wdt_period sets which bit changing from 0->1 will
// trigger a watchdog timeout. This watchdog timeout will occur 3 times, the
// first time nothing will happen, the second time a watchdog exception will
// occur, and the final time the board will reset.
//

    static bool booke_wdt_enabled;
    module_param(booke_wdt_enabled, bool, 0);
    let mut booke_wdt_period: static int = CONFIG_BOOKE_WDT_DEFAULT_TIMEOUT;
    module_param(booke_wdt_period, int, 0);
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");

// For the specified period, determine the number of seconds
// corresponding to the reset time.  There will be a watchdog
// exception at approximately 3/5 of this time.
//
// The formula to calculate this is given by:
// 2.5 * (2^(63-period+1)) / timebase_freq
//
// In order to simplify things, we assume that period is
// at least 1.  This will still result in a very long timeout.
//
#[no_mangle]
unsafe extern "C" fn period_to_sec(period: c_uint) -> c_ulonglong {
    static unsigned long long period_to_sec(unsigned int period)
    {
    let mut tmp: c_ulonglong = 1ULL << (64 - period);
    let mut tmp2: c_ulong = ppc_tb_freq;
// tmp may be a very large number and we don't want to overflow,
// so divide the timebase freq instead of multiplying tmp
//
    tmp2 = tmp2 / 5 * 2;
    do_div(tmp, tmp2);
    return tmp;
    }
//
// This procedure will find the highest period which will give a timeout
// greater than the one required. e.g. for a bus speed of 66666666 and
// a parameter of 2 secs, then this procedure will return a value of 38.
//
#[no_mangle]
unsafe extern "C" fn sec_to_period(secs: c_uint) -> c_uint {
    static unsigned int sec_to_period(unsigned int secs)
    {
    unsigned int period;
    for (period = 63; period > 0; period--) {
    if (period_to_sec(period) >= secs)
    return period;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn period_to_sec(period: c_uint) -> c_ulonglong {
    static unsigned long long period_to_sec(unsigned int period)
    {
    return period;
    }
#[no_mangle]
unsafe extern "C" fn sec_to_period(secs: c_uint) -> c_uint {
    static unsigned int sec_to_period(unsigned int secs)
    {
    return secs;
    }

#[no_mangle]
unsafe extern "C" fn __booke_wdt_set(data: *mut c_void) {
    static void __booke_wdt_set(void *data)
    {
    u32 val;
    struct watchdog_device *wdog = data;
    val = mfspr(SPRN_TCR);
    val &= ~WDTP_MASK;
    val |= WDTP(sec_to_period(wdog.timeout));
    mtspr(SPRN_TCR, val);
    }
#[no_mangle]
unsafe extern "C" fn booke_wdt_set(data: *mut c_void) {
    static void booke_wdt_set(void *data)
    {
    on_each_cpu(__booke_wdt_set, data, 0);
    }
#[no_mangle]
unsafe extern "C" fn __booke_wdt_ping(data: *mut c_void) {
    static void __booke_wdt_ping(void *data)
    {
    mtspr(SPRN_TSR, TSR_ENW|TSR_WIS);
    }
#[no_mangle]
unsafe extern "C" fn booke_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int booke_wdt_ping(struct watchdog_device *wdog)
    {
    on_each_cpu(__booke_wdt_ping, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __booke_wdt_enable(data: *mut c_void) {
    static void __booke_wdt_enable(void *data)
    {
    u32 val;
    struct watchdog_device *wdog = data;
// clear status before enabling watchdog
    __booke_wdt_ping(core::ptr::null_mut());
    val = mfspr(SPRN_TCR);
    val &= ~WDTP_MASK;
    val |= (TCR_WIE|TCR_WRC(WRC_CHIP)|WDTP(sec_to_period(wdog.timeout)));
    mtspr(SPRN_TCR, val);
    }
//
// __booke_wdt_disable - disable the watchdog on the given CPU
// @data: unused
//
// This function is called on each CPU.  It disables the watchdog on that CPU.
//
// TCR[WRC] cannot be changed once it has been set to non-zero, but we can
// effectively disable the watchdog by setting its period to the maximum value.
//
#[no_mangle]
unsafe extern "C" fn __booke_wdt_disable(data: *mut c_void) {
    static void __booke_wdt_disable(void *data)
    {
    u32 val;
    val = mfspr(SPRN_TCR);
    val &= ~(TCR_WIE | WDTP_MASK);
    mtspr(SPRN_TCR, val);
// clear status to make sure nothing is pending
    __booke_wdt_ping(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn booke_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int booke_wdt_start(struct watchdog_device *wdog)
    {
    on_each_cpu(__booke_wdt_enable, wdog, 0);
    pr_debug("watchdog enabled (timeout = %u sec)\n", wdog.timeout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn booke_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int booke_wdt_stop(struct watchdog_device *wdog)
    {
    on_each_cpu(__booke_wdt_disable, core::ptr::null_mut(), 0);
    pr_debug("watchdog disabled\n");
    return 0;
    }
    static int booke_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    wdt_dev.timeout = timeout;
    booke_wdt_set(wdt_dev);
    return 0;
    }
    static struct watchdog_info booke_wdt_info __ro_after_init = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING,
    .identity = "PowerPC Book-E Watchdog",
    };
    static const struct watchdog_ops booke_wdt_ops = {
    .owner = THIS_MODULE,
    .start = booke_wdt_start,
    .stop = booke_wdt_stop,
    .ping = booke_wdt_ping,
    .set_timeout = booke_wdt_set_timeout,
    };
    static struct watchdog_device booke_wdt_dev = {
    .info = &booke_wdt_info,
    .ops = &booke_wdt_ops,
    .min_timeout = 1,
    };
#[no_mangle]
unsafe extern "C" fn booke_wdt_exit() -> void __exit {
    static void __exit booke_wdt_exit(void)
    {
    watchdog_unregister_device(&booke_wdt_dev);
    }
#[no_mangle]
unsafe extern "C" fn booke_wdt_init() -> int __init {
    static int __init booke_wdt_init(void)
    {
    let mut ret: c_int = 0;
    pr_info("powerpc book-e watchdog driver loaded\n");
    booke_wdt_info.firmware_version = cur_cpu_spec.pvr_value;
    booke_wdt_set_timeout(&booke_wdt_dev,
    period_to_sec(booke_wdt_period));
    watchdog_set_nowayout(&booke_wdt_dev, nowayout);
    booke_wdt_dev.max_timeout = MAX_WDT_TIMEOUT;
    if (booke_wdt_enabled)
    booke_wdt_start(&booke_wdt_dev);
    ret = watchdog_register_device(&booke_wdt_dev);
    return ret;
    }
    module_init(booke_wdt_init);
    module_exit(booke_wdt_exit);
    MODULE_ALIAS("booke_wdt");
    MODULE_DESCRIPTION("PowerPC Book-E watchdog driver");
    MODULE_LICENSE("GPL");
