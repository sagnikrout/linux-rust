//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/rtc_user.c
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
// Copyright (C) 2020 Intel Corporation
// Author: Johannes Berg <johannes@sipsolutions.net>
//

    static int uml_rtc_irq_fds[2];
#[no_mangle]
pub unsafe extern "C" fn uml_rtc_send_timetravel_alarm() {
    void uml_rtc_send_timetravel_alarm(void)
    {
    let mut c: c_ulonglong = 1;
    CATCH_EINTR(write(uml_rtc_irq_fds[1], &c, sizeof(c)));
    }
#[no_mangle]
pub unsafe extern "C" fn uml_rtc_start(timetravel: bool) -> c_int {
    int uml_rtc_start(bool timetravel)
    {
    int err;
    if (timetravel) {
    err = os_pipe(uml_rtc_irq_fds, 1, 1);
    if (err)
    goto fail;
    } else {
    uml_rtc_irq_fds[0] = timerfd_create(CLOCK_REALTIME, TFD_CLOEXEC);
    if (uml_rtc_irq_fds[0] < 0) {
    err = -errno;
    goto fail;
    }
// apparently timerfd won't send SIGIO, use workaround
    sigio_broken();
    err = add_sigio_fd(uml_rtc_irq_fds[0]);
    if (err < 0) {
    close(uml_rtc_irq_fds[0]);
    goto fail;
    }
    }
    return uml_rtc_irq_fds[0];
    fail:
    uml_rtc_stop(timetravel);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn uml_rtc_enable_alarm(delta_seconds: c_ulonglong) -> c_int {
    int uml_rtc_enable_alarm(unsigned long long delta_seconds)
    {
    struct itimerspec it = {
    .it_value = {
    .tv_sec = delta_seconds,
    },
    };
    if (timerfd_settime(uml_rtc_irq_fds[0], 0, &it, core::ptr::null_mut()))
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uml_rtc_disable_alarm() {
    void uml_rtc_disable_alarm(void)
    {
    uml_rtc_enable_alarm(0);
    }
#[no_mangle]
pub unsafe extern "C" fn uml_rtc_stop(timetravel: bool) {
    void uml_rtc_stop(bool timetravel)
    {
    if (timetravel)
    os_close_file(uml_rtc_irq_fds[1]);
    else
    ignore_sigio_fd(uml_rtc_irq_fds[0]);
    os_close_file(uml_rtc_irq_fds[0]);
    }
