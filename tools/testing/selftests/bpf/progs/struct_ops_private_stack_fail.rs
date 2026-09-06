//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_private_stack_fail.c
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
    void bpf_testmod_ops3_call_test_2(void) __ksym;
    int val_i, val_j;
#[no_mangle]
pub unsafe extern "C" fn subprog2(a: *mut c_int, b: *mut c_int) -> __noinline static int {
    __noinline static int subprog2(int *a, int *b)
    {
    return val_i + a[10] + b[20];
    }
#[no_mangle]
pub unsafe extern "C" fn subprog1(a: *mut c_int) -> __noinline static int {
    __noinline static int subprog1(int *a)
    {
// stack size 200 bytes
    int b[50] = {};
    b[20] = 2;
    return subprog2(a, b);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1) -> c_int {
    int BPF_PROG(test_1)
    {
// stack size 100 bytes
    int a[25] = {};
    a[10] = 1;
    val_i = subprog1(a);
    bpf_testmod_ops3_call_test_2();
    return 0;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_2) -> c_int {
    int BPF_PROG(test_2)
    {
// stack size 400 bytes
    int a[100] = {};
    a[10] = 3;
    val_j = subprog1(a);
    return 0;
    }
    SEC(".struct_ops")
    struct bpf_testmod_ops3 testmod_1 = {
    .test_1 = (void *)test_1,
    .test_2 = (void *)test_2,
    };
