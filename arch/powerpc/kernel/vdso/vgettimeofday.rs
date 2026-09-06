//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/vdso/vgettimeofday.c
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
// Powerpc userspace implementations of gettimeofday() and similar.
//

    int __c_kernel_clock_gettime(clockid_t clock, struct __kernel_timespec *ts,
    const struct vdso_time_data *vd)
    {
    return __cvdso_clock_gettime_data(vd, clock, ts);
    }
    int __c_kernel_clock_getres(clockid_t clock_id, struct __kernel_timespec *res,
    const struct vdso_time_data *vd)
    {
    return __cvdso_clock_getres_data(vd, clock_id, res);
    }

    int __c_kernel_clock_gettime(clockid_t clock, struct old_timespec32 *ts,
    const struct vdso_time_data *vd)
    {
    return __cvdso_clock_gettime32_data(vd, clock, ts);
    }
    int __c_kernel_clock_getres(clockid_t clock_id, struct old_timespec32 *res,
    const struct vdso_time_data *vd)
    {
    return __cvdso_clock_getres_time32_data(vd, clock_id, res);
    }

    int __c_kernel_clock_gettime64(clockid_t clock, struct __kernel_timespec *ts,
    const struct vdso_time_data *vd)
    {
    return __cvdso_clock_gettime_data(vd, clock, ts);
    }
    int __c_kernel_clock_getres_time64(clockid_t clock_id, struct __kernel_timespec *res,
    const struct vdso_time_data *vd)
    {
    return __cvdso_clock_getres_data(vd, clock_id, res);
    }

    int __c_kernel_gettimeofday(struct __kernel_old_timeval *tv, struct timezone *tz,
    const struct vdso_time_data *vd)
    {
    return __cvdso_gettimeofday_data(vd, tv, tz);
    }
#[no_mangle]
pub unsafe extern "C" fn __c_kernel_time(time: *mut __kernel_old_time_t, vd: *const vdso_time_data) -> __kernel_old_time_t {
    __kernel_old_time_t __c_kernel_time(__kernel_old_time_t *time, const struct vdso_time_data *vd)
    {
    return __cvdso_time_data(vd, time);
    }
