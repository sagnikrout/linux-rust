//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/get_func_args_test.c
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

    char _license[] SEC("license") = "GPL";
    let mut test1_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1) -> c_int {
    int BPF_PROG(test1)
    {
    let mut cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0, z = 0, ret = 0;
    __s64 err;
    test1_result = cnt == 1;
// valid arguments
    err = bpf_get_func_arg(ctx, 0, &a);
// We need to cast access to traced function argument values with
// proper type cast, because trampoline uses type specific instruction
// to save it, like for 'int a' with 32-bit mov like:
//
// mov %edi,-0x8(%rbp)
//
// so the upper 4 bytes are not zeroed.
//
    test1_result &= err == 0 && ((int) a == 1);
// not valid argument
    err = bpf_get_func_arg(ctx, 1, &z);
    test1_result &= err == -EINVAL;
// return value fails in fentry
    err = bpf_get_func_ret(ctx, &ret);
    test1_result &= err == -EOPNOTSUPP;
    return 0;
    }
    let mut test2_result: __u64 = 0;
    SEC("fexit/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test2) -> c_int {
    int BPF_PROG(test2)
    {
    let mut cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0, b = 0, z = 0, ret = 0;
    __s64 err;
    test2_result = cnt == 2;
// valid arguments
    err = bpf_get_func_arg(ctx, 0, &a);
    test2_result &= err == 0 && (int) a == 2;
    err = bpf_get_func_arg(ctx, 1, &b);
    test2_result &= err == 0 && b == 3;
// not valid argument
    err = bpf_get_func_arg(ctx, 2, &z);
    test2_result &= err == -EINVAL;
// return value
    err = bpf_get_func_ret(ctx, &ret);
    test2_result &= err == 0 && ret == 5;
    return 0;
    }
    let mut test3_result: __u64 = 0;
    SEC("fmod_ret/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fmod_ret_test, _a: c_int, _b: *mut c_int, _ret: c_int) -> c_int {
    int BPF_PROG(fmod_ret_test, int _a, int *_b, int _ret)
    {
    let mut cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0, b = 0, z = 0, ret = 0;
    __s64 err;
    test3_result = cnt == 2;
// valid arguments
    err = bpf_get_func_arg(ctx, 0, &a);
    test3_result &= err == 0 && ((int) a == 1);
    err = bpf_get_func_arg(ctx, 1, &b);
    test3_result &= err == 0 && ((int *) b == _b);
// not valid argument
    err = bpf_get_func_arg(ctx, 2, &z);
    test3_result &= err == -EINVAL;
// return value
    err = bpf_get_func_ret(ctx, &ret);
    test3_result &= err == 0 && ret == 0;
// change return value, it's checked in fexit_test program
    return 1234;
    }
    let mut test4_result: __u64 = 0;
    SEC("fexit/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_test, _a: c_int, _b: *mut c_int, _ret: c_int) -> c_int {
    int BPF_PROG(fexit_test, int _a, int *_b, int _ret)
    {
    let mut cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0, b = 0, z = 0, ret = 0;
    __s64 err;
    test4_result = cnt == 2;
// valid arguments
    err = bpf_get_func_arg(ctx, 0, &a);
    test4_result &= err == 0 && ((int) a == 1);
    err = bpf_get_func_arg(ctx, 1, &b);
    test4_result &= err == 0 && ((int *) b == _b);
// not valid argument
    err = bpf_get_func_arg(ctx, 2, &z);
    test4_result &= err == -EINVAL;
// return value
    err = bpf_get_func_ret(ctx, &ret);
    test4_result &= err == 0 && ret == 1234;
    return 0;
    }
    let mut test5_result: __u64 = 0;
    SEC("tp_btf/bpf_testmod_fentry_test1_tp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tp_test1) -> c_int {
    int BPF_PROG(tp_test1)
    {
    let mut cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0, z = 0;
    __s64 err;
    test5_result = cnt == 1;
    err = bpf_get_func_arg(ctx, 0, &a);
    test5_result &= err == 0 && ((int) a == 1);
// not valid argument
    err = bpf_get_func_arg(ctx, 1, &z);
    test5_result &= err == -EINVAL;
    return 0;
    }
    let mut test6_result: __u64 = 0;
    SEC("tp_btf/bpf_testmod_fentry_test2_tp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tp_test2) -> c_int {
    int BPF_PROG(tp_test2)
    {
    let mut cnt: __u64 = bpf_get_func_arg_cnt(ctx);
    let mut a: __u64 = 0, b = 0, z = 0;
    __s64 err;
    test6_result = cnt == 2;
// valid arguments
    err = bpf_get_func_arg(ctx, 0, &a);
    test6_result &= err == 0 && (int) a == 2;
    err = bpf_get_func_arg(ctx, 1, &b);
    test6_result &= err == 0 && b == 3;
// not valid argument
    err = bpf_get_func_arg(ctx, 2, &z);
    test6_result &= err == -EINVAL;
    return 0;
    }
