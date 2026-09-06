//! Automatically rewritten from C to Rust
//! Source: rust/helpers/time.c
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
pub unsafe extern "C" fn rust_helper_fsleep(usecs: c_ulong) -> __rust_helper void {
    __rust_helper void rust_helper_fsleep(unsigned long usecs)
    {
    fsleep(usecs);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_ktime_get_real() -> __rust_helper ktime_t {
    __rust_helper ktime_t rust_helper_ktime_get_real(void)
    {
    return ktime_get_real();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_ktime_get_boottime() -> __rust_helper ktime_t {
    __rust_helper ktime_t rust_helper_ktime_get_boottime(void)
    {
    return ktime_get_boottime();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_ktime_get_clocktai() -> __rust_helper ktime_t {
    __rust_helper ktime_t rust_helper_ktime_get_clocktai(void)
    {
    return ktime_get_clocktai();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_ktime_to_us(kt: ktime_t) -> __rust_helper s64 {
    __rust_helper s64 rust_helper_ktime_to_us(const ktime_t kt)
    {
    return ktime_to_us(kt);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_ktime_to_ms(kt: ktime_t) -> __rust_helper s64 {
    __rust_helper s64 rust_helper_ktime_to_ms(const ktime_t kt)
    {
    return ktime_to_ms(kt);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_udelay(usec: c_ulong) -> __rust_helper void {
    __rust_helper void rust_helper_udelay(unsigned long usec)
    {
    udelay(usec);
    }
