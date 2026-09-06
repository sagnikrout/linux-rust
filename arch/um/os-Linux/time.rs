//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/time.c
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
// Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2012-2014 Cisco Systems
// Copyright (C) 2000 - 2007 Jeff Dike (jdike{addtoit,linux.intel}.com)
//

    static timer_t event_high_res_timer[CONFIG_NR_CPUS] = { 0 };
#[no_mangle]
pub unsafe extern "C" fn timespec_to_ns(ts: *const timespec) -> c_longlong {
    static inline long long timespec_to_ns(const struct timespec *ts)
    {
    return ((long long) ts.tv_sec * UM_NSEC_PER_SEC) + ts.tv_nsec;
    }
#[no_mangle]
pub unsafe extern "C" fn os_persistent_clock_emulation() -> c_longlong {
    long long os_persistent_clock_emulation(void)
    {
    struct timespec realtime_tp;
    clock_gettime(CLOCK_REALTIME, &realtime_tp);
    return timespec_to_ns(&realtime_tp);
    }

//
// os_timer_create() - create an new posix (interval) timer
//
#[no_mangle]
pub unsafe extern "C" fn os_timer_create() -> c_int {
    int os_timer_create(void)
    {
    let mut cpu: c_int = uml_curr_cpu();
    timer_t *t = &event_high_res_timer[cpu];
    struct sigevent sev = {
    .sigev_notify = SIGEV_THREAD_ID,
    .sigev_signo = SIGALRM,
    .sigev_value.sival_ptr = t,
    .sigev_notify_thread_id = gettid(),
    };
    if (timer_create(CLOCK_MONOTONIC, &sev, t) == -1)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_timer_set_interval(cpu: c_int, nsecs: c_ulonglong) -> c_int {
    int os_timer_set_interval(int cpu, unsigned long long nsecs)
    {
    struct itimerspec its;
    its.it_value.tv_sec = nsecs / UM_NSEC_PER_SEC;
    its.it_value.tv_nsec = nsecs % UM_NSEC_PER_SEC;
    its.it_interval.tv_sec = nsecs / UM_NSEC_PER_SEC;
    its.it_interval.tv_nsec = nsecs % UM_NSEC_PER_SEC;
    if (timer_settime(event_high_res_timer[cpu], 0, &its, core::ptr::null_mut()) == -1)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_timer_one_shot(cpu: c_int, nsecs: c_ulonglong) -> c_int {
    int os_timer_one_shot(int cpu, unsigned long long nsecs)
    {
    struct itimerspec its = {
    .it_value.tv_sec = nsecs / UM_NSEC_PER_SEC,
    .it_value.tv_nsec = nsecs % UM_NSEC_PER_SEC,
    .it_interval.tv_sec = 0,
    .it_interval.tv_nsec = 0, // we cheat here
    };
    timer_settime(event_high_res_timer[cpu], 0, &its, core::ptr::null_mut());
    return 0;
    }
//
// os_timer_disable() - disable the posix (interval) timer
// @cpu: the CPU for which the timer is to be disabled
//
#[no_mangle]
pub unsafe extern "C" fn os_timer_disable(cpu: c_int) {
    void os_timer_disable(int cpu)
    {
    struct itimerspec its;
    memset(&its, 0, sizeof(struct itimerspec));
    timer_settime(event_high_res_timer[cpu], 0, &its, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn os_nsecs() -> c_longlong {
    long long os_nsecs(void)
    {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC,&ts);
    return timespec_to_ns(&ts);
    }
    static __thread int wake_signals;
#[no_mangle]
pub unsafe extern "C" fn os_idle_prepare() {
    void os_idle_prepare(void)
    {
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGALRM);
    sigaddset(&set, IPI_SIGNAL);
//
// We need to use signalfd rather than sigsuspend in idle sleep
// because the IPI signal is a real-time signal that carries data,
// and unlike handling SIGALRM, we cannot simply flag it in
// signals_pending.
//
    wake_signals = signalfd(-1, &set, SFD_CLOEXEC);
    if (wake_signals < 0)
    panic("Failed to create signal FD, errno = %d", errno);
    }
//
// os_idle_sleep() - sleep until interrupted
//
#[no_mangle]
pub unsafe extern "C" fn os_idle_sleep() {
    void os_idle_sleep(void)
    {
    sigset_t set;
//
// Block SIGALRM while performing the need_resched check.
// Note that, because IRQs are disabled, the IPI signal is
// already blocked.
//
    sigemptyset(&set);
    sigaddset(&set, SIGALRM);
    sigprocmask(SIG_BLOCK, &set, core::ptr::null_mut());
//
// Because disabling IRQs does not block SIGALRM, it is also
// necessary to check for any pending timer alarms.
//
    if (!uml_need_resched() && !timer_alarm_pending())
    os_poll(1, &wake_signals);
// Restore the signal mask.
    sigprocmask(SIG_UNBLOCK, &set, core::ptr::null_mut());
    }
