//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/dummy_st_ops_fail.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    SEC("struct_ops.s/test_2")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_dummy_ops": "attach to unsupported member test_2 of struct) -> __failure {
    __failure __msg("attach to unsupported member test_2 of struct bpf_dummy_ops")
    int BPF_PROG(test_unsupported_field_sleepable,
    struct bpf_dummy_ops_state *state, int a1, unsigned short a2,
    char a3, unsigned long a4)
    {
// Tries to mark an unsleepable field in struct bpf_dummy_ops as sleepable.
    return 0;
    }
    SEC(".struct_ops")
    struct bpf_dummy_ops dummy_1 = {
    .test_1 = core::ptr::null_mut(),
    .test_2 = (void *)test_unsupported_field_sleepable,
    .test_sleepable = (void *)core::ptr::null_mut(),
    };
