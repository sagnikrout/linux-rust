//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/leap-a-day.c
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


// Leap second stress test
// by: John Stultz (john.stultz@linaro.org)
// (C) Copyright IBM 2012
// (C) Copyright 2013, 2015 Linaro Limited
// Licensed under the GPLv2
//
// This test signals the kernel to insert a leap second
// every day at midnight GMT. This allows for stressing the
// kernel's leap-second behavior, as well as how well applications
// handle the leap-second discontinuity.
//
// Usage: leap-a-day [-w] [-i <num>] [-t]
//
// Options:
// -w:	Only set the leap-second flag and wait for the leap second
// each iteration, instead of advancing the time. By default the
// date is set to 10 seconds before midnight GMT, which speeds up
// the number of leapsecond transitions tested, but because it
// calls settimeofday frequently, advancing the time by 24 hours
// every ~16 seconds, it may cause application disruption.
//
// -i:	Number of iterations to run (-1 = infinite, default: 10)
//
// -t:	Print TAI time.
//
// Other notes: Disabling NTP prior to running this is advised, as the two
// may conflict in their commands to the kernel.
//
// To build:
// $ gcc leap-a-day.c -o leap-a-day -lrt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

pub const CLOCK_TAI: c_int = 11;
    time_t next_leap;
    int error_found;
// returns 1 if a <= b, 0 otherwise
#[no_mangle]
pub unsafe extern "C" fn in_order(a: timespec, b: timespec) -> c_int {
    static inline int in_order(struct timespec a, struct timespec b)
    {
    if (a.tv_sec < b.tv_sec)
    return 1;
    if (a.tv_sec > b.tv_sec)
    return 0;
    if (a.tv_nsec > b.tv_nsec)
    return 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn timespec_add(ts: timespec, ns: c_ulonglong) -> timespec {
    struct timespec timespec_add(struct timespec ts, unsigned long long ns)
    {
    ts.tv_nsec += ns;
    while (ts.tv_nsec >= NSEC_PER_SEC) {
    ts.tv_nsec -= NSEC_PER_SEC;
    ts.tv_sec++;
    }
    return ts;
    }
    char *time_state_str(int state)
    {
    switch (state) {
    case TIME_OK:	return "TIME_OK";
    case TIME_INS:	return "TIME_INS";
    case TIME_DEL:	return "TIME_DEL";
    case TIME_OOP:	return "TIME_OOP";
    case TIME_WAIT:	return "TIME_WAIT";
    case TIME_BAD:	return "TIME_BAD";
    }
    return "ERROR";
    }
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
// Clear maxerror, as it can cause UNSYNC to be set
    tx.modes = ADJ_MAXERROR;
    tx.maxerror = 0;
    ret = adjtimex(&tx);
// Clear the status
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
pub unsafe extern "C" fn sigalarm(signo: c_int) {
    void sigalarm(int signo)
    {
    struct timex tx;
    int ret;
    tx.modes = 0;
    ret = adjtimex(&tx);
    if (tx.time.tv_sec < next_leap) {
    printf("Error: Early timer expiration! (Should be %ld)\n", next_leap);
    error_found = 1;
    printf("adjtimex: %10ld sec + %6ld us (%i)\t%s\n",
    tx.time.tv_sec,
    tx.time.tv_usec,
    tx.tai,
    time_state_str(ret));
    }
    if (ret != TIME_WAIT) {
    printf("Error: Timer seeing incorrect NTP state? (Should be TIME_WAIT)\n");
    error_found = 1;
    printf("adjtimex: %10ld sec + %6ld us (%i)\t%s\n",
    tx.time.tv_sec,
    tx.time.tv_usec,
    tx.tai,
    time_state_str(ret));
    }
    }
// Test for known hrtimer failure
#[no_mangle]
pub unsafe extern "C" fn test_hrtimer_failure() {
    void test_hrtimer_failure(void)
    {
    struct timespec now, target;
    clock_gettime(CLOCK_REALTIME, &now);
    target = timespec_add(now, NSEC_PER_SEC/2);
    clock_nanosleep(CLOCK_REALTIME, TIMER_ABSTIME, &target, core::ptr::null_mut());
    clock_gettime(CLOCK_REALTIME, &now);
    if (!in_order(target, now)) {
    printf("ERROR: hrtimer early expiration failure observed.\n");
    error_found = 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    timer_t tm1;
    struct itimerspec its1;
    struct sigevent se;
    struct sigaction act;
    let mut signum: c_int = SIGRTMAX;
    let mut settime: c_int = 1;
    let mut tai_time: c_int = 0;
    let mut insert: c_int = 1;
    let mut iterations: c_int = 10;
    int opt;
// Process arguments
    while ((opt = getopt(argc, argv, "wti:")) != -1) {
    switch (opt) {
    case 'w':
    printf("Only setting leap-flag, not changing time. It could take up to a day for leap to trigger.\n");
    settime = 0;
    break;
    case 'i':
    iterations = atoi(optarg);
    break;
    case 't':
    tai_time = 1;
    break;
    default:
    printf("Usage: %s [-w] [-i <iterations>]\n", argv[0]);
    printf("	-w: Set flag and wait for leap second each iteration");
    printf("	    (default sets time to right before leapsecond)\n");
    printf("	-i: Number of iterations (-1 = infinite, default is 10)\n");
    printf("	-t: Print TAI time\n");
    exit(-1);
    }
    }
// Make sure TAI support is present if -t was used
    if (tai_time) {
    struct timespec ts;
    if (clock_gettime(CLOCK_TAI, &ts)) {
    printf("System doesn't support CLOCK_TAI\n");
    ksft_exit_fail();
    }
    }
    signal(SIGINT, handler);
    signal(SIGKILL, handler);
// Set up timer signal handler:
    sigfillset(&act.sa_mask);
    act.sa_flags = 0;
    act.sa_handler = sigalarm;
    sigaction(signum, &act, core::ptr::null_mut());
    if (iterations < 0)
    printf("This runs continuously. Press ctrl-c to stop\n");
    else
    printf("Running for %i iterations. Press ctrl-c to stop\n", iterations);
    printf("\n");
    while (1) {
    int ret;
    struct timespec ts;
    struct timex tx;
    time_t now;
// Get the current time
    clock_gettime(CLOCK_REALTIME, &ts);
// Calculate the next possible leap second 23:59:60 GMT
    next_leap = ts.tv_sec;
    next_leap += 86400 - (next_leap % 86400);
    if (settime) {
    struct timeval tv;
    tv.tv_sec = next_leap - 10;
    tv.tv_usec = 0;
    settimeofday(&tv, core::ptr::null_mut());
    printf("Setting time to %s", ctime(&tv.tv_sec));
    }
// Reset NTP time state
    clear_time_state();
// Set the leap second insert flag
    tx.modes = ADJ_STATUS;
    if (insert)
    tx.status = STA_INS;
    else
    tx.status = STA_DEL;
    ret = adjtimex(&tx);
    if (ret < 0) {
    printf("Error: Problem setting STA_INS/STA_DEL!: %s\n",
    time_state_str(ret));
    ksft_exit_fail();
    }
// Validate STA_INS was set
    tx.modes = 0;
    ret = adjtimex(&tx);
    if (tx.status != STA_INS && tx.status != STA_DEL) {
    printf("Error: STA_INS/STA_DEL not set!: %s\n",
    time_state_str(ret));
    ksft_exit_fail();
    }
    if (tai_time) {
    printf("Using TAI time,"
    " no inconsistencies should be seen!\n");
    }
    printf("Scheduling leap second for %s", ctime(&next_leap));
// Set up timer
    printf("Setting timer for %ld -  %s", next_leap, ctime(&next_leap));
    memset(&se, 0, sizeof(se));
    se.sigev_notify = SIGEV_SIGNAL;
    se.sigev_signo = signum;
    se.sigev_value.sival_int = 0;
    if (timer_create(CLOCK_REALTIME, &se, &tm1) == -1) {
    printf("Error: timer_create failed\n");
    ksft_exit_fail();
    }
    its1.it_value.tv_sec = next_leap;
    its1.it_value.tv_nsec = 0;
    its1.it_interval.tv_sec = 0;
    its1.it_interval.tv_nsec = 0;
    timer_settime(tm1, TIMER_ABSTIME, &its1, core::ptr::null_mut());
// Wake up 3 seconds before leap
    ts.tv_sec = next_leap - 3;
    ts.tv_nsec = 0;
    while (clock_nanosleep(CLOCK_REALTIME, TIMER_ABSTIME, &ts, core::ptr::null_mut()))
    printf("Something woke us up, returning to sleep\n");
// Validate STA_INS is still set
    tx.modes = 0;
    ret = adjtimex(&tx);
    if (tx.status != STA_INS && tx.status != STA_DEL) {
    printf("Something cleared STA_INS/STA_DEL, setting it again.\n");
    tx.modes = ADJ_STATUS;
    if (insert)
    tx.status = STA_INS;
    else
    tx.status = STA_DEL;
    ret = adjtimex(&tx);
    }
// Check adjtimex output every half second
    now = tx.time.tv_sec;
    while (now < next_leap + 2) {
    char buf[26];
    struct timespec tai;
    int ret;
    tx.modes = 0;
    ret = adjtimex(&tx);
    if (tai_time) {
    clock_gettime(CLOCK_TAI, &tai);
    printf("%ld sec, %9ld ns\t%s\n",
    tai.tv_sec,
    tai.tv_nsec,
    time_state_str(ret));
    } else {
    ctime_r(&tx.time.tv_sec, buf);
    buf[strlen(buf)-1] = 0; /*remove trailing\n */
    printf("%s + %6ld us (%i)\t%s\n",
    buf,
    tx.time.tv_usec,
    tx.tai,
    time_state_str(ret));
    }
    now = tx.time.tv_sec;
// Sleep for another half second
    ts.tv_sec = 0;
    ts.tv_nsec = NSEC_PER_SEC / 2;
    clock_nanosleep(CLOCK_MONOTONIC, 0, &ts, core::ptr::null_mut());
    }
// Switch to using other mode
    insert = !insert;
// Note if kernel has known hrtimer failure
    test_hrtimer_failure();
    printf("Leap complete\n");
    if (error_found) {
    printf("Errors observed\n");
    clear_time_state();
    ksft_exit_fail();
    }
    printf("\n");
    if ((iterations != -1) && !(--iterations))
    break;
    }
    clear_time_state();
    ksft_exit_pass();
    }
