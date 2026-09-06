//! Automatically rewritten from C to Rust
//! Source: drivers/char/hangcheck-timer.c
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
// hangcheck-timer.c
//
// Driver for a little io fencing timer.
//
// Copyright (C) 2002, 2003 Oracle.  All rights reserved.
//
// Author: Joel Becker <joel.becker@oracle.com>
//
// The hangcheck-timer driver uses the TSC to catch delays that
// jiffies does not notice.  A timer is set.  When the timer fires, it
// checks whether it was delayed and if that delay exceeds a given
// margin of error.  The hangcheck_tick module parameter takes the timer
// duration in seconds.  The hangcheck_margin parameter defines the
// margin of error, in seconds.  The defaults are 60 seconds for the
// timer and 180 seconds for the margin of error.  IOW, a timer is set
// for 60 seconds.  When the timer fires, the callback checks the
// actual duration that the timer waited.  If the duration exceeds the
// allotted time and margin (here 60 + 180, or 240 seconds), the machine
// is restarted.  A healthy machine will have the duration match the
// expected timeout very closely.
//

    let mut hangcheck_tick: static int = DEFAULT_IOFENCE_TICK;
    let mut hangcheck_margin: static int = DEFAULT_IOFENCE_MARGIN;
    static int hangcheck_reboot;  /* Defaults to not reboot */
    static int hangcheck_dump_tasks;  /* Defaults to not dumping SysRQ T */
// options - modular
    module_param(hangcheck_tick, int, 0);
    MODULE_PARM_DESC(hangcheck_tick, "Timer delay.");
    module_param(hangcheck_margin, int, 0);
    MODULE_PARM_DESC(hangcheck_margin, "If the hangcheck timer has been delayed more than hangcheck_margin seconds, the driver will fire.");
    module_param(hangcheck_reboot, int, 0);
    MODULE_PARM_DESC(hangcheck_reboot, "If nonzero, the machine will reboot when the timer margin is exceeded.");
    module_param(hangcheck_dump_tasks, int, 0);
    MODULE_PARM_DESC(hangcheck_dump_tasks, "If nonzero, the machine will dump the system task state when the timer margin is exceeded.");
    MODULE_AUTHOR("Oracle");
    MODULE_DESCRIPTION("Hangcheck-timer detects when the system has gone out to lunch past a certain margin.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(VERSION_STR);
// options - nonmodular

#[no_mangle]
unsafe extern "C" fn hangcheck_parse_tick(str: *mut c_char) -> int __init {
    static int __init hangcheck_parse_tick(char *str)
    {
    int par;
    if (get_option(&str, &par))
    hangcheck_tick = par;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hangcheck_parse_margin(str: *mut c_char) -> int __init {
    static int __init hangcheck_parse_margin(char *str)
    {
    int par;
    if (get_option(&str, &par))
    hangcheck_margin = par;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hangcheck_parse_reboot(str: *mut c_char) -> int __init {
    static int __init hangcheck_parse_reboot(char *str)
    {
    int par;
    if (get_option(&str, &par))
    hangcheck_reboot = par;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hangcheck_parse_dump_tasks(str: *mut c_char) -> int __init {
    static int __init hangcheck_parse_dump_tasks(char *str)
    {
    int par;
    if (get_option(&str, &par))
    hangcheck_dump_tasks = par;
    return 1;
    }
    __setup("hcheck_tick", hangcheck_parse_tick);
    __setup("hcheck_margin", hangcheck_parse_margin);
    __setup("hcheck_reboot", hangcheck_parse_reboot);
    __setup("hcheck_dump_tasks", hangcheck_parse_dump_tasks);

// Last time scheduled
    static unsigned long long hangcheck_tsc, hangcheck_tsc_margin;
    static void hangcheck_fire(struct timer_list *);
    static DEFINE_TIMER(hangcheck_ticktock, hangcheck_fire);
#[no_mangle]
unsafe extern "C" fn hangcheck_fire(unused: *mut timer_list) {
    static void hangcheck_fire(struct timer_list *unused)
    {
    unsigned long long cur_tsc, tsc_diff;
    cur_tsc = ktime_get_ns();
    if (cur_tsc > hangcheck_tsc)
    tsc_diff = cur_tsc - hangcheck_tsc;
    else
    tsc_diff = (cur_tsc + (~0ULL - hangcheck_tsc)); /* or something */
    if (tsc_diff > hangcheck_tsc_margin) {
    if (hangcheck_dump_tasks) {
    pr_crit("Hangcheck: Task state:\n");

    handle_sysrq('t');

    }
    if (hangcheck_reboot) {
    pr_crit("Hangcheck: hangcheck is restarting the machine.\n");
    emergency_restart();
    } else {
    pr_crit("Hangcheck: hangcheck value past margin!\n");
    }
    }

//
// Enable to investigate delays in detail
//
    pr_debug("Hangcheck: called %lld ns since last time (%lld ns overshoot)\n",
    tsc_diff, tsc_diff - hangcheck_tick*TIMER_FREQ);

    mod_timer(&hangcheck_ticktock, jiffies + (hangcheck_tick*HZ));
    hangcheck_tsc = ktime_get_ns();
    }
#[no_mangle]
unsafe extern "C" fn hangcheck_init() -> int __init {
    static int __init hangcheck_init(void)
    {
    pr_debug("Hangcheck: starting hangcheck timer %s (tick is %d seconds, margin is %d seconds).\n",
    VERSION_STR, hangcheck_tick, hangcheck_margin);
    hangcheck_tsc_margin =
    (unsigned long long)hangcheck_margin + hangcheck_tick;
    hangcheck_tsc_margin *= TIMER_FREQ;
    hangcheck_tsc = ktime_get_ns();
    mod_timer(&hangcheck_ticktock, jiffies + (hangcheck_tick*HZ));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hangcheck_exit() -> void __exit {
    static void __exit hangcheck_exit(void)
    {
    timer_delete_sync(&hangcheck_ticktock);
    pr_debug("Hangcheck: Stopped hangcheck timer.\n");
    }
    module_init(hangcheck_init);
    module_exit(hangcheck_exit);
