//! Automatically rewritten from C to Rust
//! Source: kernel/time/timekeeping_debug.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// debugfs file to track time spent in suspend
//
// Copyright (c) 2011, Google, Inc.
//

pub const NUM_BINS: c_int = 32;
// Incremented every time mg_floor is updated
    DEFINE_PER_CPU(unsigned long, timekeeping_mg_floor_swaps);
    static unsigned int sleep_time_bin[NUM_BINS] = {0};
#[no_mangle]
unsafe extern "C" fn tk_debug_sleep_time_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tk_debug_sleep_time_show(struct seq_file *s, void *data)
    {
    unsigned int bin;
    seq_puts(s, "      time (secs)        count\n");
    seq_puts(s, "------------------------------\n");
    for (bin = 0; bin < 32; bin++) {
    if (sleep_time_bin[bin] == 0)
    continue;
    seq_printf(s, "%10u - %-10u %4u\n",
    bin ? 1 << (bin - 1) : 0, 1 << bin,
    sleep_time_bin[bin]);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tk_debug_sleep_time);
#[no_mangle]
unsafe extern "C" fn tk_debug_sleep_time_init() -> int __init {
    static int __init tk_debug_sleep_time_init(void)
    {
    debugfs_create_file("sleep_time", 0444, core::ptr::null_mut(), core::ptr::null_mut(),
    &tk_debug_sleep_time_fops);
    return 0;
    }
    late_initcall(tk_debug_sleep_time_init);
#[no_mangle]
pub unsafe extern "C" fn tk_debug_account_sleep_time(t: *const timespec64) {
    void tk_debug_account_sleep_time(const struct timespec64 *t)
    {
// Cap bin index so we don't overflow the array
    let mut bin: c_int = min(fls(t.tv_sec), NUM_BINS-1);
    sleep_time_bin[bin]++;
    pm_deferred_pr_dbg("Timekeeping suspended for %lld.%03lu seconds\n",
    (s64)t.tv_sec, t.tv_nsec / NSEC_PER_MSEC);
    }
#[no_mangle]
pub unsafe extern "C" fn timekeeping_get_mg_floor_swaps() -> c_ulong {
    unsigned long timekeeping_get_mg_floor_swaps(void)
    {
    let mut sum: c_ulong = 0;
    int cpu;
    for_each_possible_cpu(cpu)
    sum += data_race(per_cpu(timekeeping_mg_floor_swaps, cpu));
    return sum;
    }
