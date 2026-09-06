//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/dummy_st_ops_success.c
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
// Copyright (C) 2021. Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
    SEC("struct_ops/test_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1, state: *mut bpf_dummy_ops_state) -> c_int {
    int BPF_PROG(test_1, struct bpf_dummy_ops_state *state)
    {
    int ret;
// Check that 'state' nullable status is detected correctly.
// If 'state' argument would be assumed non-null by verifier
// the code below would be deleted as dead (which it shouldn't).
// Hide it from the compiler behind 'asm' block to avoid
// unnecessary optimizations.
//
    asm volatile (
    "if %[state] != 0 goto +2;"
    "r0 = 0xf2f3f4f5;"
    "exit;"
    ::[state]"p"(state));
    ret = state.val;
    state.val = 0x5a;
    return ret;
    }
    __u64 test_2_args[5];
    SEC("struct_ops/test_2")
    int BPF_PROG(test_2, struct bpf_dummy_ops_state *state, int a1, unsigned short a2,
    char a3, unsigned long a4)
    {
    test_2_args[0] = state.val;
    test_2_args[1] = a1;
    test_2_args[2] = a2;
    test_2_args[3] = a3;
    test_2_args[4] = a4;
    return 0;
    }
    SEC("struct_ops.s/test_sleepable")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_sleepable, state: *mut bpf_dummy_ops_state) -> c_int {
    int BPF_PROG(test_sleepable, struct bpf_dummy_ops_state *state)
    {
    return 0;
    }
    SEC(".struct_ops")
    struct bpf_dummy_ops dummy_1 = {
    .test_1 = (void *)test_1,
    .test_2 = (void *)test_2,
    .test_sleepable = (void *)test_sleepable,
    };
