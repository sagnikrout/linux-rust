//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/leapcrash.c
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


// Demo leapsecond deadlock
// by: John Stultz (john.stultz@linaro.org)
// (C) Copyright IBM 2012
// (C) Copyright 2013, 2015 Linaro Limited
// Licensed under the GPL
//
// This test demonstrates leapsecond deadlock that is possible
// on kernels from 2.6.26 to 3.3.
//
// WARNING: THIS WILL LIKELY HARD HANG SYSTEMS AND MAY LOSE DATA
// RUN AT YOUR OWN RISK!
// To build:
// $ gcc leapcrash.c -o leapcrash -lrt
//

// clear NTP time_status & time_state
#[no_mangle]
pub unsafe extern "C" fn clear_time_state() -> c_int {
    int clear_time_state(void)
    {
    struct timex tx;
    int ret;
//
// We have to call adjtime twice here, as kernels
// prior to 6b1859dba01c7 (included in 3.5 and
// -stable), had an issue with the state machine
// and wouldn't clear the STA_INS/DEL flag directly.
//
    tx.modes = ADJ_STATUS;
    tx.status = STA_PLL;
    ret = adjtimex(&tx);
    tx.modes = ADJ_STATUS;
    tx.status = 0;
    ret = adjtimex(&tx);
    return ret;
    }
// Make sure we cleanup on ctrl-c
#[no_mangle]
pub unsafe extern "C" fn handler(unused: c_int) {
    void handler(int unused)
    {
    clear_time_state();
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct timex tx;
    struct timespec ts;
    time_t next_leap;
    let mut count: c_int = 0;
    setbuf(stdout, core::ptr::null_mut());
    signal(SIGINT, handler);
    signal(SIGKILL, handler);
    printf("This runs for a few minutes. Press ctrl-c to stop\n");
    clear_time_state();
// Get the current time
    clock_gettime(CLOCK_REALTIME, &ts);
// Calculate the next possible leap second 23:59:60 GMT
    next_leap = ts.tv_sec;
    next_leap += 86400 - (next_leap % 86400);
    for (count = 0; count < 20; count++) {
    struct timeval tv;
// set the time to 2 seconds before the leap
    tv.tv_sec = next_leap - 2;
    tv.tv_usec = 0;
    if (settimeofday(&tv, core::ptr::null_mut())) {
    printf("Error: You're likely not running with proper (ie: root) permissions\n");
    ksft_exit_fail();
    }
    tx.modes = 0;
    adjtimex(&tx);
// hammer on adjtime w/ STA_INS
    while (tx.time.tv_sec < next_leap + 1) {
// Set the leap second insert flag
    tx.modes = ADJ_STATUS;
    tx.status = STA_INS;
    adjtimex(&tx);
    }
    clear_time_state();
    printf(".");
    fflush(stdout);
    }
    printf("[OK]\n");
    ksft_exit_pass();
    }
