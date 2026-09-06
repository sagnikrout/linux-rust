//! Automatically rewritten from C to Rust
//! Source: samples/livepatch/livepatch-callbacks-busymod.c
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
// Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
//
// livepatch-callbacks-busymod.c - (un)patching callbacks demo support module
//
// Purpose
// -------
//
// Simple module to demonstrate livepatch (un)patching callbacks.
//
// Usage
// -----
//
// This module is not intended to be standalone.  See the "Usage"
// section of livepatch-callbacks-mod.c.
//

    static int sleep_secs;
    module_param(sleep_secs, int, 0644);
    MODULE_PARM_DESC(sleep_secs, "sleep_secs (default=0)");
    static void busymod_work_func(struct work_struct *work);
    static DECLARE_DELAYED_WORK(work, busymod_work_func);
#[no_mangle]
unsafe extern "C" fn busymod_work_func(work: *mut work_struct) {
    static void busymod_work_func(struct work_struct *work)
    {
    pr_info("%s, sleeping %d seconds ...\n", __func__, sleep_secs);
    msleep(sleep_secs * 1000);
    pr_info("%s exit\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn livepatch_callbacks_mod_init() -> c_int {
    static int livepatch_callbacks_mod_init(void)
    {
    pr_info("%s\n", __func__);
    schedule_delayed_work(&work, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn livepatch_callbacks_mod_exit() {
    static void livepatch_callbacks_mod_exit(void)
    {
    cancel_delayed_work_sync(&work);
    pr_info("%s\n", __func__);
    }
    module_init(livepatch_callbacks_mod_init);
    module_exit(livepatch_callbacks_mod_exit);
    MODULE_DESCRIPTION("Live patching demo for (un)patching callbacks, support module");
    MODULE_LICENSE("GPL");
