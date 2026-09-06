//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/time.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// time function definitions for NOLIBC
// Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

//
// int clock_getres(clockid_t clockid, struct timespec *res);
// int clock_gettime(clockid_t clockid, struct timespec *tp);
// int clock_settime(clockid_t clockid, const struct timespec *tp);
// int clock_nanosleep(clockid_t clockid, int flags, const struct timespec *rqtp,
// struct timespec *rmtp)
//

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_clock_getres_time64, _arg: clockid, _arg: res) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_clock_getres, _arg: clockid, _arg: res) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_clock_getres(clockid, _arg: res)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_clock_gettime64, _arg: clockid, _arg: tp) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_clock_gettime, _arg: clockid, _arg: tp) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_clock_gettime(clockid, _arg: tp)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_clock_settime64, _arg: clockid, _arg: tp) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_clock_settime, _arg: clockid, _arg: tp) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_clock_settime(clockid, _arg: tp)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_clock_nanosleep_time64, _arg: clockid, _arg: flags, _arg: rqtp, _arg: rmtp) -> return;
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_clock_nanosleep, _arg: clockid, _arg: flags, _arg: rqtp, _arg: rmtp) -> return;
}

// Directly return a positive error number
extern "C" {
    pub fn __sysret(_arg: _sys_clock_nanosleep(CLOCK_REALTIME, _arg: 0, _arg: rqtp, _arg: rmtp)) -> return;
}
// note, cannot fail here
// tptr = tv.tv_sec;
//
// int timer_create(clockid_t clockid, struct sigevent *evp, timer_t *timerid);
// int timer_gettime(timer_t timerid, struct itimerspec *curr_value);
// int timer_settime(timer_t timerid, int flags, const struct itimerspec *new_value, struct itimerspec *old_value);
//
extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_timer_create, _arg: clockid, _arg: evp, _arg: timerid) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_timer_create(clockid, _arg: evp, _arg: timerid)) -> return;
}
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_timer_delete, _arg: timerid) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_timer_delete(timerid)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_timer_gettime64, _arg: timerid, _arg: curr_value) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_timer_gettime, _arg: timerid, _arg: curr_value) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_timer_gettime(timerid, _arg: curr_value)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_timer_settime64, _arg: timerid, _arg: flags, _arg: new_value, _arg: old_value) -> return;
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_timer_settime, _arg: timerid, _arg: flags, _arg: new_value, _arg: old_value) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_timer_settime(timerid, _arg: flags, _arg: new_value, _arg: old_value)) -> return;
}
