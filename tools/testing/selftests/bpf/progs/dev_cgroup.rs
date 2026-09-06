//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/dev_cgroup.c
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


// Copyright (c) 2017 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

    SEC("cgroup/dev")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut bpf_cgroup_dev_ctx) -> c_int {
    int bpf_prog1(struct bpf_cgroup_dev_ctx *ctx)
    {
    let mut type: c_short = ctx.access_type & 0xFFFF;

    let mut access: c_short = ctx.access_type >> 16;
    char fmt[] = "  %d:%d    \n";
    switch (type) {
    case BPF_DEVCG_DEV_BLOCK:
    fmt[0] = 'b';
    break;
    case BPF_DEVCG_DEV_CHAR:
    fmt[0] = 'c';
    break;
    default:
    fmt[0] = '?';
    break;
    }
    if (access & BPF_DEVCG_ACC_READ)
    fmt[8] = 'r';
    if (access & BPF_DEVCG_ACC_WRITE)
    fmt[9] = 'w';
    if (access & BPF_DEVCG_ACC_MKNOD)
    fmt[10] = 'm';
    bpf_trace_printk(fmt, sizeof(fmt), ctx.major, ctx.minor);

// Allow access to /dev/null and /dev/urandom.
// Forbid everything else.
//
    if (ctx.major != 1 || type != BPF_DEVCG_DEV_CHAR)
    return 0;
    switch (ctx.minor) {
    case 3: /* 1:3 /dev/null */
    case 9: /* 1:9 /dev/urandom */
    return 1;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
