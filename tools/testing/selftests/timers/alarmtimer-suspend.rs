//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/alarmtimer-suspend.c
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


// alarmtimer suspend test
// John Stultz (john.stultz@linaro.org)
// (C) Copyright Linaro 2013
// Licensed under the GPLv2
//
// This test makes sure the alarmtimer & RTC wakeup code is
// functioning.
//
// To build:
// $ gcc alarmtimer-suspend.c -o alarmtimer-suspend -lrt
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

pub const SUSPEND_SECS: c_int = 15;
    int alarmcount;
    int alarm_clock_id;
    struct timespec start_time;
#[no_mangle]
pub unsafe extern "C" fn timespec_sub(a: timespec, b: timespec) -> c_longlong {
    long long timespec_sub(struct timespec a, struct timespec b)
    {
    let mut ret: c_longlong = NSEC_PER_SEC * b.tv_sec + b.tv_nsec;
    ret -= NSEC_PER_SEC * a.tv_sec + a.tv_nsec;
    return ret;
    }
    int final_ret;
#[no_mangle]
pub unsafe extern "C" fn sigalarm(signo: c_int) {
    void sigalarm(int signo)
    {
    long long delta_ns;
    struct timespec ts;
    clock_gettime(alarm_clock_id, &ts);
    alarmcount++;
    delta_ns = timespec_sub(start_time, ts);
    delta_ns -= NSEC_PER_SEC * SUSPEND_SECS * alarmcount;
    printf("ALARM(%i): %ld:%ld latency: %lld ns ", alarmcount, ts.tv_sec,
    ts.tv_nsec, delta_ns);
    if (delta_ns > UNREASONABLE_LAT) {
    printf("[FAIL]\n");
    final_ret = -1;
    } else
    printf("[OK]\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    timer_t tm1;
    struct itimerspec its1, its2;
    struct sigevent se;
    struct sigaction act;
    let mut signum: c_int = SIGRTMAX;
// Set up signal handler:
    sigfillset(&act.sa_mask);
    act.sa_flags = 0;
    act.sa_handler = sigalarm;
    sigaction(signum, &act, core::ptr::null_mut());
// Set up timer:
    memset(&se, 0, sizeof(se));
    se.sigev_notify = SIGEV_SIGNAL;
    se.sigev_signo = signum;
    se.sigev_value.sival_int = 0;
    for (alarm_clock_id = CLOCK_REALTIME_ALARM;
    alarm_clock_id <= CLOCK_BOOTTIME_ALARM;
    alarm_clock_id++) {
    alarmcount = 0;
    if (timer_create(alarm_clock_id, &se, &tm1) == -1) {
    printf("timer_create failed, %s unsupported?: %s\n",
    clock_name(alarm_clock_id), strerror(errno));
    break;
    }
    clock_gettime(alarm_clock_id, &start_time);
    printf("Start time (%s): %ld:%ld\n", clock_name(alarm_clock_id),
    start_time.tv_sec, start_time.tv_nsec);
    printf("Setting alarm for every %i seconds\n", SUSPEND_SECS);
    its1.it_value = start_time;
    its1.it_value.tv_sec += SUSPEND_SECS;
    its1.it_interval.tv_sec = SUSPEND_SECS;
    its1.it_interval.tv_nsec = 0;
    timer_settime(tm1, TIMER_ABSTIME, &its1, &its2);
    while (alarmcount < 5)
    sleep(1); /* First 5 alarms, do nothing */
    printf("Starting suspend loops\n");
    while (alarmcount < 10) {
    int ret;
    sleep(3);
    ret = system("echo mem > /sys/power/state");
    if (ret)
    break;
    }
    timer_delete(tm1);
    }
    if (final_ret)
    ksft_exit_fail();
    ksft_exit_pass();
    }
