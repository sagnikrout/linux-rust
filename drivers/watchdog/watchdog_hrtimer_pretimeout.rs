//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/watchdog_hrtimer_pretimeout.c
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
// (c) Copyright 2021 Hewlett Packard Enterprise Development LP.
//

#[no_mangle]
unsafe extern "C" fn watchdog_hrtimer_pretimeout(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart watchdog_hrtimer_pretimeout(struct hrtimer *timer)
    {
    struct watchdog_core_data *wd_data;
    wd_data = container_of(timer, struct watchdog_core_data, pretimeout_timer);
    watchdog_notify_pretimeout(wd_data.wdd);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hrtimer_pretimeout_init(wdd: *mut watchdog_device) {
    void watchdog_hrtimer_pretimeout_init(struct watchdog_device *wdd)
    {
    struct watchdog_core_data *wd_data = wdd.wd_data;
    hrtimer_setup(&wd_data.pretimeout_timer, watchdog_hrtimer_pretimeout, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hrtimer_pretimeout_start(wdd: *mut watchdog_device) {
    void watchdog_hrtimer_pretimeout_start(struct watchdog_device *wdd)
    {
    if (!(wdd.info.options & WDIOF_PRETIMEOUT) &&
    !watchdog_pretimeout_invalid(wdd, wdd.pretimeout))
    hrtimer_start(&wdd.wd_data.pretimeout_timer,
    ktime_set(wdd.timeout - wdd.pretimeout, 0),
    HRTIMER_MODE_REL);
    else
    hrtimer_cancel(&wdd.wd_data.pretimeout_timer);
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hrtimer_pretimeout_stop(wdd: *mut watchdog_device) {
    void watchdog_hrtimer_pretimeout_stop(struct watchdog_device *wdd)
    {
    hrtimer_cancel(&wdd.wd_data.pretimeout_timer);
    }
