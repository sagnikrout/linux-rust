//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/urandom_read_lib1.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
pub const _SDT_HAS_SEMAPHORES: c_int = 1;

pub const SHARED: c_int = 1;

    unsigned short urandlib_read_with_sema_semaphore SEC(".probes");
#[no_mangle]
pub unsafe extern "C" fn urandlib_read_with_sema(iter_num: c_int, iter_cnt: c_int, read_sz: c_int) {
    void urandlib_read_with_sema(int iter_num, int iter_cnt, int read_sz)
    {
    STAP_PROBE3(urandlib, read_with_sema, iter_num, iter_cnt, read_sz);
    }
    COMPAT_VERSION(urandlib_api_v1, urandlib_api, LIBURANDOM_READ_1.0.0)
#[no_mangle]
pub unsafe extern "C" fn urandlib_api_v1() -> c_int {
    int urandlib_api_v1(void)
    {
    return 1;
    }
    DEFAULT_VERSION(urandlib_api_v2, urandlib_api, LIBURANDOM_READ_2.0.0)
#[no_mangle]
pub unsafe extern "C" fn urandlib_api_v2() -> c_int {
    int urandlib_api_v2(void)
    {
    return 2;
    }
    COMPAT_VERSION(urandlib_api_sameoffset, urandlib_api_sameoffset, LIBURANDOM_READ_1.0.0)
    DEFAULT_VERSION(urandlib_api_sameoffset, urandlib_api_sameoffset, LIBURANDOM_READ_2.0.0)
#[no_mangle]
pub unsafe extern "C" fn urandlib_api_sameoffset() -> c_int {
    int urandlib_api_sameoffset(void)
    {
    return 3;
    }
