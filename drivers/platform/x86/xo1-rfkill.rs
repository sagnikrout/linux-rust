//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/xo1-rfkill.c
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
// Support for rfkill through the OLPC XO-1 laptop embedded controller
//
// Copyright (C) 2010 One Laptop per Child
//

    static bool card_blocked;
#[no_mangle]
unsafe extern "C" fn rfkill_set_block(data: *mut c_void, blocked: bool) -> c_int {
    static int rfkill_set_block(void *data, bool blocked)
    {
    unsigned char cmd;
    int r;
    if (blocked == card_blocked)
    return 0;
    if (blocked)
    cmd = EC_WLAN_ENTER_RESET;
    else
    cmd = EC_WLAN_LEAVE_RESET;
    r = olpc_ec_cmd(cmd, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    if (r == 0)
    card_blocked = blocked;
    return r;
    }
    static const struct rfkill_ops rfkill_ops = {
    .set_block = rfkill_set_block,
    };
#[no_mangle]
unsafe extern "C" fn xo1_rfkill_probe(pdev: *mut platform_device) -> c_int {
    static int xo1_rfkill_probe(struct platform_device *pdev)
    {
    struct rfkill *rfk;
    int r;
    rfk = rfkill_alloc(pdev.name, &pdev.dev, RFKILL_TYPE_WLAN,
    &rfkill_ops, core::ptr::null_mut());
    if (!rfk)
    return -ENOMEM;
    r = rfkill_register(rfk);
    if (r) {
    rfkill_destroy(rfk);
    return r;
    }
    platform_set_drvdata(pdev, rfk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xo1_rfkill_remove(pdev: *mut platform_device) {
    static void xo1_rfkill_remove(struct platform_device *pdev)
    {
    struct rfkill *rfk = platform_get_drvdata(pdev);
    rfkill_unregister(rfk);
    rfkill_destroy(rfk);
    }
    static struct platform_driver xo1_rfkill_driver = {
    .driver = {
    .name = "xo1-rfkill",
    },
    .probe		= xo1_rfkill_probe,
    .remove		= xo1_rfkill_remove,
    };
    module_platform_driver(xo1_rfkill_driver);
    MODULE_AUTHOR("Daniel Drake <dsd@laptop.org>");
    MODULE_DESCRIPTION("OLPC XO-1 software RF kill switch");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:xo1-rfkill");
