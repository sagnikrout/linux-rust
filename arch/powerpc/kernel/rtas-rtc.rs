//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/rtas-rtc.c
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

#[no_mangle]
pub unsafe extern "C" fn rtas_get_boot_time() -> time64_t __init {
    time64_t __init rtas_get_boot_time(void)
    {
    int ret[8];
    int error;
    unsigned int wait_time;
    u64 max_wait_tb;
    max_wait_tb = get_tb() + tb_ticks_per_usec * 1000 * MAX_RTC_WAIT;
    do {
    error = rtas_call(rtas_function_token(RTAS_FN_GET_TIME_OF_DAY), 0, 8, ret);
    wait_time = rtas_busy_delay_time(error);
    if (wait_time) {
// This is boot time so we spin.
    udelay(wait_time*1000);
    }
    } while (wait_time && (get_tb() < max_wait_tb));
    if (error != 0) {
    printk_ratelimited(KERN_WARNING
    "error: reading the clock failed (%d)\n",
    error);
    return 0;
    }
    return mktime64(ret[0], ret[1], ret[2], ret[3], ret[4], ret[5]);
    }
// NOTE: get_rtc_time will get an error if executed in interrupt context
// and if a delay is needed to read the clock.  In this case we just
// silently return without updating rtc_tm.
//
#[no_mangle]
pub unsafe extern "C" fn rtas_get_rtc_time(rtc_tm: *mut rtc_time) {
    void rtas_get_rtc_time(struct rtc_time *rtc_tm)
    {
    int ret[8];
    int error;
    unsigned int wait_time;
    u64 max_wait_tb;
    max_wait_tb = get_tb() + tb_ticks_per_usec * 1000 * MAX_RTC_WAIT;
    do {
    error = rtas_call(rtas_function_token(RTAS_FN_GET_TIME_OF_DAY), 0, 8, ret);
    wait_time = rtas_busy_delay_time(error);
    if (wait_time) {
    if (in_interrupt()) {
    memset(rtc_tm, 0, sizeof(struct rtc_time));
    printk_ratelimited(KERN_WARNING
    "error: reading clock "
    "would delay interrupt\n");
    return;	/* delay not allowed */
    }
    msleep(wait_time);
    }
    } while (wait_time && (get_tb() < max_wait_tb));
    if (error != 0) {
    printk_ratelimited(KERN_WARNING
    "error: reading the clock failed (%d)\n",
    error);
    return;
    }
    rtc_tm.tm_sec = ret[5];
    rtc_tm.tm_min = ret[4];
    rtc_tm.tm_hour = ret[3];
    rtc_tm.tm_mday = ret[2];
    rtc_tm.tm_mon = ret[1] - 1;
    rtc_tm.tm_year = ret[0] - 1900;
    }
#[no_mangle]
pub unsafe extern "C" fn rtas_set_rtc_time(tm: *mut rtc_time) -> c_int {
    int rtas_set_rtc_time(struct rtc_time *tm)
    {
    int error, wait_time;
    u64 max_wait_tb;
    max_wait_tb = get_tb() + tb_ticks_per_usec * 1000 * MAX_RTC_WAIT;
    do {
    error = rtas_call(rtas_function_token(RTAS_FN_SET_TIME_OF_DAY), 7, 1, core::ptr::null_mut(),
    tm.tm_year + 1900, tm.tm_mon + 1,
    tm.tm_mday, tm.tm_hour, tm.tm_min,
    tm.tm_sec, 0);
    wait_time = rtas_busy_delay_time(error);
    if (wait_time) {
    if (in_interrupt())
    return 1;	/* probably decrementer */
    msleep(wait_time);
    }
    } while (wait_time && (get_tb() < max_wait_tb));
    if (error != 0)
    printk_ratelimited(KERN_WARNING
    "error: setting the clock failed (%d)\n",
    error);
    return 0;
    }
