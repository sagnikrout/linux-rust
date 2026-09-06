//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_autoload.c
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
// Copyright (c) 2020 Facebook

    let mut prog1_called: bool = false;
    let mut prog2_called: bool = false;
    let mut prog3_called: bool = false;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn prog1(ctx: *const c_void) -> c_int {
    int prog1(const void *ctx)
    {
    prog1_called = true;
    return 0;
    }
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn prog2(ctx: *const c_void) -> c_int {
    int prog2(const void *ctx)
    {
    prog2_called = true;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fake_kernel_struct {
    pub whatever: c_int,
    pub __attribute__((preserve_access_index)): },
    SEC("fentry/unexisting-kprobe-will-fail-if-loaded")
#[no_mangle]
pub unsafe extern "C" fn prog3(ctx: *const c_void) -> c_int {
    int prog3(const void *ctx)
    {
    pub )ctx: *mut *mut fake_kernel_fake = (void,
    pub 123: fake->whatever =,
    pub true: prog3_called =,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
