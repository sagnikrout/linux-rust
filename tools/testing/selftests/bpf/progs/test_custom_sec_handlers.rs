//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_custom_sec_handlers.c
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
// Copyright (c) 2022 Facebook

    const volatile int my_pid;
    bool abc1_called;
    bool abc2_called;
    bool custom1_called;
    bool custom2_called;
    bool kprobe1_called;
    bool xyz_called;
    SEC("abc")
#[no_mangle]
pub unsafe extern "C" fn abc1(ctx: *mut c_void) -> c_int {
    int abc1(void *ctx)
    {
    abc1_called = true;
    return 0;
    }
    SEC("abc/whatever")
#[no_mangle]
pub unsafe extern "C" fn abc2(ctx: *mut c_void) -> c_int {
    int abc2(void *ctx)
    {
    abc2_called = true;
    return 0;
    }
    SEC("custom")
#[no_mangle]
pub unsafe extern "C" fn custom1(ctx: *mut c_void) -> c_int {
    int custom1(void *ctx)
    {
    custom1_called = true;
    return 0;
    }
    SEC("custom/something")
#[no_mangle]
pub unsafe extern "C" fn custom2(ctx: *mut c_void) -> c_int {
    int custom2(void *ctx)
    {
    custom2_called = true;
    return 0;
    }
    SEC("kprobe")
#[no_mangle]
pub unsafe extern "C" fn kprobe1(ctx: *mut c_void) -> c_int {
    int kprobe1(void *ctx)
    {
    kprobe1_called = true;
    return 0;
    }
    SEC("xyz/blah")
#[no_mangle]
pub unsafe extern "C" fn xyz(ctx: *mut c_void) -> c_int {
    int xyz(void *ctx)
    {
    int whatever;
// use sleepable helper, custom handler should set sleepable flag
    bpf_copy_from_user(&whatever, sizeof(whatever), core::ptr::null_mut());
    xyz_called = true;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
