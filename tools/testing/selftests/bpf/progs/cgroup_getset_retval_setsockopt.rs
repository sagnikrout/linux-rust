//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_getset_retval_setsockopt.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2021 Google LLC.
//

    let mut invocations: __u32 = 0;
    let mut assertion_error: __u32 = 0;
    let mut retval_value: __u32 = 0;
    let mut page_size: __s32 = 0;
    SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn get_retval(ctx: *mut bpf_sockopt) -> c_int {
    int get_retval(struct bpf_sockopt *ctx)
    {
    retval_value = bpf_get_retval();
    __sync_fetch_and_add(&invocations, 1);
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 1;
    }
    SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn set_eunatch(ctx: *mut bpf_sockopt) -> c_int {
    int set_eunatch(struct bpf_sockopt *ctx)
    {
    __sync_fetch_and_add(&invocations, 1);
    if (bpf_set_retval(-EUNATCH))
    assertion_error = 1;
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 0;
    }
    SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn set_eisconn(ctx: *mut bpf_sockopt) -> c_int {
    int set_eisconn(struct bpf_sockopt *ctx)
    {
    __sync_fetch_and_add(&invocations, 1);
    if (bpf_set_retval(-EISCONN))
    assertion_error = 1;
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 0;
    }
    SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn legacy_eperm(ctx: *mut bpf_sockopt) -> c_int {
    int legacy_eperm(struct bpf_sockopt *ctx)
    {
    __sync_fetch_and_add(&invocations, 1);
// optval larger than PAGE_SIZE use kernel's buffer.
    if (ctx.optlen > page_size)
    ctx.optlen = 0;
    return 0;
    }
