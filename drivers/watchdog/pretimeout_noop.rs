//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/pretimeout_noop.c
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
// Copyright (C) 2015-2016 Mentor Graphics
//

//
// pretimeout_noop - No operation on watchdog pretimeout event
// @wdd: watchdog_device
//
// This function prints a message about pretimeout to kernel log.
//
#[no_mangle]
unsafe extern "C" fn pretimeout_noop(wdd: *mut watchdog_device) {
    static void pretimeout_noop(struct watchdog_device *wdd)
    {
    pr_alert("watchdog%d: pretimeout event\n", wdd.id);
    }
    static struct watchdog_governor watchdog_gov_noop = {
    .name		= "noop",
    .pretimeout	= pretimeout_noop,
    };
#[no_mangle]
unsafe extern "C" fn watchdog_gov_noop_register() -> int __init {
    static int __init watchdog_gov_noop_register(void)
    {
    return watchdog_register_governor(&watchdog_gov_noop);
    }
#[no_mangle]
unsafe extern "C" fn watchdog_gov_noop_unregister() -> void __exit {
    static void __exit watchdog_gov_noop_unregister(void)
    {
    watchdog_unregister_governor(&watchdog_gov_noop);
    }
    module_init(watchdog_gov_noop_register);
    module_exit(watchdog_gov_noop_unregister);
    MODULE_AUTHOR("Vladimir Zapolskiy <vladimir_zapolskiy@mentor.com>");
    MODULE_DESCRIPTION("Panic watchdog pretimeout governor");
    MODULE_LICENSE("GPL");
