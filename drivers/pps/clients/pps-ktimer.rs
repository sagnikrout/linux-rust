//! Automatically rewritten from C to Rust
//! Source: drivers/pps/clients/pps-ktimer.c
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
// pps-ktimer.c -- kernel timer test client
//
// Copyright (C) 2005-2006   Rodolfo Giometti <giometti@linux.it>
//

//
// Global variables
//
    static struct pps_device *pps;
    static struct timer_list ktimer;
//
// The kernel timer
//
#[no_mangle]
unsafe extern "C" fn pps_ktimer_event(unused: *mut timer_list) {
    static void pps_ktimer_event(struct timer_list *unused)
    {
    struct pps_event_time ts;
// First of all we get the time stamp...
    pps_get_ts(&ts);
    pps_event(pps, &ts, PPS_CAPTUREASSERT, core::ptr::null_mut());
    mod_timer(&ktimer, jiffies + HZ);
    }
//
// The PPS info struct
//
    static struct pps_source_info pps_ktimer_info = {
    .name		= "ktimer",
    .path		= "",
    .mode		= PPS_CAPTUREASSERT | PPS_OFFSETASSERT |
    PPS_ECHOASSERT |
    PPS_CANWAIT | PPS_TSFMT_TSPEC,
    .owner		= THIS_MODULE,
    };
//
// Module staff
//
#[no_mangle]
unsafe extern "C" fn pps_ktimer_exit() -> void __exit {
    static void __exit pps_ktimer_exit(void)
    {
    dev_dbg(&pps.dev, "ktimer PPS source unregistered\n");
    timer_delete_sync(&ktimer);
    pps_unregister_source(pps);
    }
#[no_mangle]
unsafe extern "C" fn pps_ktimer_init() -> int __init {
    static int __init pps_ktimer_init(void)
    {
    pps = pps_register_source(&pps_ktimer_info,
    PPS_CAPTUREASSERT | PPS_OFFSETASSERT);
    if (IS_ERR(pps)) {
    pr_err("cannot register PPS source\n");
    return PTR_ERR(pps);
    }
    timer_setup(&ktimer, pps_ktimer_event, 0);
    mod_timer(&ktimer, jiffies + HZ);
    dev_dbg(&pps.dev, "ktimer PPS source registered\n");
    return 0;
    }
    module_init(pps_ktimer_init);
    module_exit(pps_ktimer_exit);
    MODULE_AUTHOR("Rodolfo Giometti <giometti@linux.it>");
    MODULE_DESCRIPTION("dummy PPS source by using a kernel timer (just for debug)");
    MODULE_LICENSE("GPL");
