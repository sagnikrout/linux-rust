//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sun4v_wdt.c
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
// sun4v watchdog timer
// (c) Copyright 2016 Oracle Corporation
//
// Implement a simple watchdog driver using the built-in sun4v hypervisor
// watchdog support. If time expires, the hypervisor stops or bounces
// the guest domain.
//

pub const WDT_TIMEOUT: c_int = 60;
pub const WDT_MAX_TIMEOUT: c_int = 31536000;
pub const WDT_MIN_TIMEOUT: c_int = 1;

    static unsigned int timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds (default="
    __MODULE_STRING(WDT_TIMEOUT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, S_IRUGO);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn sun4v_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int sun4v_wdt_stop(struct watchdog_device *wdd)
    {
    sun4v_mach_set_watchdog(0, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4v_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int sun4v_wdt_ping(struct watchdog_device *wdd)
    {
    int hverr;
//
// HV watchdog timer will round up the timeout
// passed in to the nearest multiple of the
// watchdog resolution in milliseconds.
//
    hverr = sun4v_mach_set_watchdog(wdd.timeout * 1000, core::ptr::null_mut());
    if (hverr == HV_EINVAL)
    return -EINVAL;
    return 0;
    }
    static int sun4v_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    wdd.timeout = timeout;
    return 0;
    }
    static const struct watchdog_info sun4v_wdt_ident = {
    .options =	WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    .identity =	"sun4v hypervisor watchdog",
    .firmware_version = 0,
    };
    static const struct watchdog_ops sun4v_wdt_ops = {
    .owner =	THIS_MODULE,
    .start =	sun4v_wdt_ping,
    .stop =		sun4v_wdt_stop,
    .ping =		sun4v_wdt_ping,
    .set_timeout =	sun4v_wdt_set_timeout,
    };
    static struct watchdog_device wdd = {
    .info = &sun4v_wdt_ident,
    .ops = &sun4v_wdt_ops,
    .min_timeout = WDT_MIN_TIMEOUT,
    .max_timeout = WDT_MAX_TIMEOUT,
    .timeout = WDT_TIMEOUT,
    };
#[no_mangle]
unsafe extern "C" fn sun4v_wdt_init() -> int __init {
    static int __init sun4v_wdt_init(void)
    {
    struct mdesc_handle *handle;
    u64 node;
    const u64 *value;
    let mut err: c_int = 0;
    let mut major: c_ulong = 1, minor = 1;
//
// There are 2 properties that can be set from the control
// domain for the watchdog.
// watchdog-resolution
// watchdog-max-timeout
//
// We can expect a handle to be returned otherwise something
// serious is wrong. Correct to return -ENODEV here.
//
    handle = mdesc_grab();
    if (!handle)
    return -ENODEV;
    node = mdesc_node_by_name(handle, MDESC_NODE_NULL, "platform");
    err = -ENODEV;
    if (node == MDESC_NODE_NULL)
    goto out_release;
//
// This is a safe way to validate if we are on the right
// platform.
//
    if (sun4v_hvapi_register(HV_GRP_CORE, major, &minor))
    goto out_hv_unreg;
// Allow value of watchdog-resolution up to 1s (default)
    value = mdesc_get_property(handle, node, "watchdog-resolution", core::ptr::null_mut());
    err = -EINVAL;
    if (value) {
    if (*value == 0 ||
// value > WDT_DEFAULT_RESOLUTION_MS)
    goto out_hv_unreg;
    }
    value = mdesc_get_property(handle, node, "watchdog-max-timeout", core::ptr::null_mut());
    if (value) {
//
// If the property value (in ms) is smaller than
// min_timeout, return -EINVAL.
//
    if (*value < wdd.min_timeout * 1000)
    goto out_hv_unreg;
//
// If the property value is smaller than
// default max_timeout  then set watchdog max_timeout to
// the value of the property in seconds.
//
    if (*value < wdd.max_timeout * 1000)
    wdd.max_timeout = *value  / 1000;
    }
    watchdog_init_timeout(&wdd, timeout, core::ptr::null_mut());
    watchdog_set_nowayout(&wdd, nowayout);
    err = watchdog_register_device(&wdd);
    if (err)
    goto out_hv_unreg;
    pr_info("initialized (timeout=%ds, nowayout=%d)\n",
    wdd.timeout, nowayout);
    mdesc_release(handle);
    return 0;
    out_hv_unreg:
    sun4v_hvapi_unregister(HV_GRP_CORE);
    out_release:
    mdesc_release(handle);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sun4v_wdt_exit() -> void __exit {
    static void __exit sun4v_wdt_exit(void)
    {
    sun4v_hvapi_unregister(HV_GRP_CORE);
    watchdog_unregister_device(&wdd);
    }
    module_init(sun4v_wdt_init);
    module_exit(sun4v_wdt_exit);
    MODULE_AUTHOR("Wim Coekaerts <wim.coekaerts@oracle.com>");
    MODULE_DESCRIPTION("sun4v watchdog driver");
    MODULE_LICENSE("GPL");
