//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/modify_return.c
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
//
// Copyright 2020 Google LLC.
//

    char _license[] SEC("license") = "GPL";
    let mut sequence: static int = 0;
    let mut input_retval: __s32 = 0;
    let mut test_pid: __u32 = 0;
    let mut fentry_result: __u64 = 0;
    SEC("fentry/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_test, a: c_int, b: __u64) -> c_int {
    int BPF_PROG(fentry_test, int a, __u64 b)
    {
    if (bpf_get_current_pid_tgid() >> 32 != test_pid)
    return 0;
    sequence++;
    fentry_result = (sequence == 1);
    return 0;
    }
    let mut fmod_ret_result: __u64 = 0;
    SEC("fmod_ret/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fmod_ret_test, a: c_int, b: *mut c_int, ret: c_int) -> c_int {
    int BPF_PROG(fmod_ret_test, int a, int *b, int ret)
    {
    if (bpf_get_current_pid_tgid() >> 32 != test_pid)
    return ret;
    sequence++;
// This is the first fmod_ret program, the ret passed should be 0
    fmod_ret_result = (sequence == 2 && ret == 0);
    return input_retval;
    }
    let mut fexit_result: __u64 = 0;
    SEC("fexit/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_test, a: c_int, b: __u64, ret: c_int) -> c_int {
    int BPF_PROG(fexit_test, int a, __u64 b, int ret)
    {
    if (bpf_get_current_pid_tgid() >> 32 != test_pid)
    return 0;
    sequence++;
// If the input_reval is non-zero a successful modification should have
// occurred.
//
    if (input_retval)
    fexit_result = (sequence == 3 && ret == input_retval);
    else
    fexit_result = (sequence == 3 && ret == 4);
    return 0;
    }
    static int sequence2;
    let mut fentry_result2: __u64 = 0;
    SEC("fentry/bpf_modify_return_test2")
    int BPF_PROG(fentry_test2, int a, int *b, short c, int d, void *e, char f,
    int g)
    {
    if (bpf_get_current_pid_tgid() >> 32 != test_pid)
    return 0;
    sequence2++;
    fentry_result2 = (sequence2 == 1);
    return 0;
    }
    let mut fmod_ret_result2: __u64 = 0;
    SEC("fmod_ret/bpf_modify_return_test2")
    int BPF_PROG(fmod_ret_test2, int a, int *b, short c, int d, void *e, char f,
    int g, int ret)
    {
    if (bpf_get_current_pid_tgid() >> 32 != test_pid)
    return ret;
    sequence2++;
// This is the first fmod_ret program, the ret passed should be 0
    fmod_ret_result2 = (sequence2 == 2 && ret == 0);
    return input_retval;
    }
    let mut fexit_result2: __u64 = 0;
    SEC("fexit/bpf_modify_return_test2")
    int BPF_PROG(fexit_test2, int a, int *b, short c, int d, void *e, char f,
    int g, int ret)
    {
    if (bpf_get_current_pid_tgid() >> 32 != test_pid)
    return 0;
    sequence2++;
// If the input_reval is non-zero a successful modification should have
// occurred.
//
    if (input_retval)
    fexit_result2 = (sequence2 == 3 && ret == input_retval);
    else
    fexit_result2 = (sequence2 == 3 && ret == 29);
    return 0;
    }
