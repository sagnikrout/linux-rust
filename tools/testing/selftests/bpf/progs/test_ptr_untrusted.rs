//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ptr_untrusted.c
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
// Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com>

    char tp_name[128];
    SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: lsm_run, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(lsm_run, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    switch (cmd) {
    case BPF_RAW_TRACEPOINT_OPEN:
    bpf_copy_from_user(tp_name, sizeof(tp_name) - 1,
    (void *)attr.raw_tracepoint.name);
    break;
    default:
    break;
    }
    return 0;
    }
    SEC("raw_tracepoint")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: raw_tp_run) -> c_int {
    int BPF_PROG(raw_tp_run)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
