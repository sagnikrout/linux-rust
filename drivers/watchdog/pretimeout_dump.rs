//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/pretimeout_dump.c
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
// Copyright 2026 Google LLC
//

//
// pretimeout_dump - Dump on watchdog pretimeout event
// @wdd: watchdog_device
//
// Dump all cpu backtrace on pretimeout event.
//
#[no_mangle]
unsafe extern "C" fn pretimeout_dump(wdd: *mut watchdog_device) {
    static void pretimeout_dump(struct watchdog_device *wdd)
    {
    pr_alert("watchdog%d: pretimeout event\n", wdd.id);
    if (!trigger_all_cpu_backtrace())
    pr_alert("trigger_all_cpu_backtrace() isn't available\n");
    }
    static struct watchdog_governor watchdog_gov_dump = {
    .name		= "dump",
    .pretimeout	= pretimeout_dump,
    };
#[no_mangle]
unsafe extern "C" fn watchdog_gov_dump_register() -> int __init {
    static int __init watchdog_gov_dump_register(void)
    {
    return watchdog_register_governor(&watchdog_gov_dump);
    }
#[no_mangle]
unsafe extern "C" fn watchdog_gov_dump_unregister() -> void __exit {
    static void __exit watchdog_gov_dump_unregister(void)
    {
    watchdog_unregister_governor(&watchdog_gov_dump);
    }
    module_init(watchdog_gov_dump_register);
    module_exit(watchdog_gov_dump_unregister);
    MODULE_AUTHOR("Tzung-Bi Shih <tzungbi@kernel.org>");
    MODULE_DESCRIPTION("Dump watchdog pretimeout governor");
    MODULE_LICENSE("GPL");
