//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/token_lsm.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    int my_pid;
    int reject_capable;
    int reject_cmd;
    SEC("lsm/bpf_token_capable")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: token_capable, token: *mut bpf_token, cap: c_int) -> c_int {
    int BPF_PROG(token_capable, struct bpf_token *token, int cap)
    {
    if (my_pid == 0 || my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    if (reject_capable)
    return -1;
    return 0;
    }
    SEC("lsm/bpf_token_cmd")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: token_cmd, token: *mut bpf_token, cmd: enum bpf_cmd) -> c_int {
    int BPF_PROG(token_cmd, struct bpf_token *token, enum bpf_cmd cmd)
    {
    if (my_pid == 0 || my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    if (reject_cmd)
    return -1;
    return 0;
    }
