//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tracing_failure.c
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
    SEC("?fentry/bpf_spin_lock")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_spin_lock, lock: *mut bpf_spin_lock) -> c_int {
    int BPF_PROG(test_spin_lock, struct bpf_spin_lock *lock)
    {
    return 0;
    }
    SEC("?fentry/bpf_spin_unlock")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_spin_unlock, lock: *mut bpf_spin_lock) -> c_int {
    int BPF_PROG(test_spin_unlock, struct bpf_spin_lock *lock)
    {
    return 0;
    }
    SEC("?fentry/__rcu_read_lock")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tracing_deny) -> c_int {
    int BPF_PROG(tracing_deny)
    {
    return 0;
    }
    SEC("?fexit/do_exit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_noreturns) -> c_int {
    int BPF_PROG(fexit_noreturns)
    {
    return 0;
    }
    SEC("?fexit/bpf_testmod_test_int128_ret")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_int128_ret) -> c_int {
    int BPF_PROG(fexit_int128_ret)
    {
    return 0;
    }
