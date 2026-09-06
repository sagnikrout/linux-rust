//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_assoc_reuse.c
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
pub const MAP_A_MAGIC: c_int = 1234;
    int test_err_a;
    int recur;
//
// test_1_a is reused. The kfunc should not be able to get the associated
// struct_ops and call test_1 recursively as it is ambiguous.
//
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1_a, args: *mut st_ops_args) -> c_int {
    int BPF_PROG(test_1_a, struct st_ops_args *args)
    {
    int ret;
    if (!recur) {
    recur++;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(args);
    if (ret != -1)
    test_err_a++;
    recur--;
    }
    return MAP_A_MAGIC;
    }
// Programs associated with st_ops_map_a
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn syscall_prog_a(ctx: *mut c_void) -> c_int {
    int syscall_prog_a(void *ctx)
    {
    let mut args: st_ops_args = {};
    int ret;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_A_MAGIC)
    test_err_a++;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_multi_st_ops st_ops_map_a = {
    .test_1 = (void *)test_1_a,
    };
// Programs associated with st_ops_map_b
    int test_err_b;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn syscall_prog_b(ctx: *mut c_void) -> c_int {
    int syscall_prog_b(void *ctx)
    {
    let mut args: st_ops_args = {};
    int ret;
    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&args);
    if (ret != MAP_A_MAGIC)
    test_err_b++;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_multi_st_ops st_ops_map_b = {
    .test_1 = (void *)test_1_a,
    };
