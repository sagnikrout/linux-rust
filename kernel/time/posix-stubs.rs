//! Automatically rewritten from C to Rust
//! Source: kernel/time/posix-stubs.c
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
// Dummy stubs used when CONFIG_POSIX_TIMERS=n
//
// Created by:  Nicolas Pitre, July 2016
// Copyright:   (C) 2016 Linaro Limited
//

//
// We preserve minimal support for CLOCK_REALTIME and CLOCK_MONOTONIC
// as it is easy to remain compatible with little code. CLOCK_BOOTTIME
// is also included for convenience as at least systemd uses it.
//
    SYSCALL_DEFINE2(clock_settime, const clockid_t, which_clock,
    const struct __kernel_timespec __user *, tp)
    {
    struct timespec64 new_tp;
    if (which_clock != CLOCK_REALTIME)
    return -EINVAL;
    if (get_timespec64(&new_tp, tp))
    return -EFAULT;
    return do_sys_settimeofday64(&new_tp, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn do_clock_gettime(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    static int do_clock_gettime(clockid_t which_clock, struct timespec64 *tp)
    {
    switch (which_clock) {
    case CLOCK_REALTIME:
    ktime_get_real_ts64(tp);
    break;
    case CLOCK_MONOTONIC:
    ktime_get_ts64(tp);
    timens_add_monotonic(tp);
    break;
    case CLOCK_BOOTTIME:
    ktime_get_boottime_ts64(tp);
    timens_add_boottime(tp);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    SYSCALL_DEFINE2(clock_gettime, const clockid_t, which_clock,
    struct __kernel_timespec __user *, tp)
    {
    int ret;
    struct timespec64 kernel_tp;
    ret = do_clock_gettime(which_clock, &kernel_tp);
    if (ret)
    return ret;
    if (put_timespec64(&kernel_tp, tp))
    return -EFAULT;
    return 0;
    }
    SYSCALL_DEFINE2(clock_getres, const clockid_t, which_clock, struct __kernel_timespec __user *, tp)
    {
    struct timespec64 rtn_tp = {
    .tv_sec = 0,
    .tv_nsec = hrtimer_resolution,
    };
    switch (which_clock) {
    case CLOCK_REALTIME:
    case CLOCK_MONOTONIC:
    case CLOCK_BOOTTIME:
    if (put_timespec64(&rtn_tp, tp))
    return -EFAULT;
    return 0;
    default:
    return -EINVAL;
    }
    }
    SYSCALL_DEFINE4(clock_nanosleep, const clockid_t, which_clock, int, flags,
    const struct __kernel_timespec __user *, rqtp,
    struct __kernel_timespec __user *, rmtp)
    {
    struct timespec64 t;
    ktime_t texp;
    switch (which_clock) {
    case CLOCK_REALTIME:
    case CLOCK_MONOTONIC:
    case CLOCK_BOOTTIME:
    break;
    default:
    return -EINVAL;
    }
    if (get_timespec64(&t, rqtp))
    return -EFAULT;
    if (!timespec64_valid(&t))
    return -EINVAL;
    if (flags & TIMER_ABSTIME)
    rmtp = core::ptr::null_mut();
    current.restart_block.fn = do_no_restart_syscall;
    current.restart_block.nanosleep.type = rmtp ? TT_NATIVE : TT_NONE;
    current.restart_block.nanosleep.rmtp = rmtp;
    texp = timespec64_to_ktime(t);
    if (flags & TIMER_ABSTIME)
    texp = timens_ktime_to_host(which_clock, texp);
    return hrtimer_nanosleep(texp, flags & TIMER_ABSTIME ?
    HRTIMER_MODE_ABS : HRTIMER_MODE_REL,
    which_clock);
    }

    SYSCALL_DEFINE2(clock_settime32, const clockid_t, which_clock,
    struct old_timespec32 __user *, tp)
    {
    struct timespec64 new_tp;
    if (which_clock != CLOCK_REALTIME)
    return -EINVAL;
    if (get_old_timespec32(&new_tp, tp))
    return -EFAULT;
    return do_sys_settimeofday64(&new_tp, core::ptr::null_mut());
    }
    SYSCALL_DEFINE2(clock_gettime32, clockid_t, which_clock,
    struct old_timespec32 __user *, tp)
    {
    int ret;
    struct timespec64 kernel_tp;
    ret = do_clock_gettime(which_clock, &kernel_tp);
    if (ret)
    return ret;
    if (put_old_timespec32(&kernel_tp, tp))
    return -EFAULT;
    return 0;
    }
    SYSCALL_DEFINE2(clock_getres_time32, clockid_t, which_clock,
    struct old_timespec32 __user *, tp)
    {
    struct timespec64 rtn_tp = {
    .tv_sec = 0,
    .tv_nsec = hrtimer_resolution,
    };
    switch (which_clock) {
    case CLOCK_REALTIME:
    case CLOCK_MONOTONIC:
    case CLOCK_BOOTTIME:
    if (put_old_timespec32(&rtn_tp, tp))
    return -EFAULT;
    return 0;
    default:
    return -EINVAL;
    }
    }
    SYSCALL_DEFINE4(clock_nanosleep_time32, clockid_t, which_clock, int, flags,
    struct old_timespec32 __user *, rqtp,
    struct old_timespec32 __user *, rmtp)
    {
    struct timespec64 t;
    ktime_t texp;
    switch (which_clock) {
    case CLOCK_REALTIME:
    case CLOCK_MONOTONIC:
    case CLOCK_BOOTTIME:
    break;
    default:
    return -EINVAL;
    }
    if (get_old_timespec32(&t, rqtp))
    return -EFAULT;
    if (!timespec64_valid(&t))
    return -EINVAL;
    if (flags & TIMER_ABSTIME)
    rmtp = core::ptr::null_mut();
    current.restart_block.fn = do_no_restart_syscall;
    current.restart_block.nanosleep.type = rmtp ? TT_COMPAT : TT_NONE;
    current.restart_block.nanosleep.compat_rmtp = rmtp;
    texp = timespec64_to_ktime(t);
    if (flags & TIMER_ABSTIME)
    texp = timens_ktime_to_host(which_clock, texp);
    return hrtimer_nanosleep(texp, flags & TIMER_ABSTIME ?
    HRTIMER_MODE_ABS : HRTIMER_MODE_REL,
    which_clock);
    }
