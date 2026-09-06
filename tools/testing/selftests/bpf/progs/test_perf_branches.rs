//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_perf_branches.c
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
// Copyright (c) 2019 Facebook

    let mut valid: c_int = 0;
    let mut run_cnt: c_int = 0;
    let mut required_size_out: c_int = 0;
    let mut written_stack_out: c_int = 0;
    let mut written_global_out: c_int = 0;
    struct {
    __u64 _a;
    __u64 _b;
    __u64 _c;
    } fpbe[30] = {0};
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn perf_branches(ctx: *mut c_void) -> c_int {
    int perf_branches(void *ctx)
    {
    __u64 entries[4 * 3] = {0};
    int required_size, written_stack, written_global;
    ++run_cnt;
// write to stack
    written_stack = bpf_read_branch_records(ctx, entries, sizeof(entries), 0);
// ignore spurious events
    if (!written_stack)
    return 1;
// get required size
    required_size = bpf_read_branch_records(ctx, core::ptr::null_mut(), 0,
    BPF_F_GET_BRANCH_RECORDS_SIZE);
    written_global = bpf_read_branch_records(ctx, fpbe, sizeof(fpbe), 0);
// ignore spurious events
    if (!written_global)
    return 1;
    required_size_out = required_size;
    written_stack_out = written_stack;
    written_global_out = written_global;
    valid = 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
