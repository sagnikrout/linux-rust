//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/get_func_args_fsession_test.c
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
    SEC("fsession/bpf_fentry_test1")
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
    test1_result &= err == 0 && ((int) a == 1);
// not valid argument
    err = bpf_get_func_arg(ctx, 1, &z);
    test1_result &= err == -EINVAL;
    if (bpf_session_is_return(ctx)) {
    err = bpf_get_func_ret(ctx, &ret);
    test1_result &= err == 0 && ret == 2;
    } else {
    err = bpf_get_func_ret(ctx, &ret);
    test1_result &= err == 0 && ret == 0;
    }
    return 0;
    }
