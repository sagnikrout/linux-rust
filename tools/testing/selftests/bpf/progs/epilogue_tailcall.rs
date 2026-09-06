//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/epilogue_tailcall.c
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
#[no_mangle]
unsafe extern "C" fn subprog(args: *mut st_ops_args) -> __noinline __used int {
    static __noinline __used int subprog(struct st_ops_args *args)
    {
    args.a += 1;
    return args.a;
    }
    SEC("struct_ops/test_epilogue_subprog")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_epilogue_subprog, args: *mut st_ops_args) -> c_int {
    int BPF_PROG(test_epilogue_subprog, struct st_ops_args *args)
    {
    subprog(args);
    return args.a;
    }
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    __array(values, void (void));
    } epilogue_map SEC(".maps") = {
    .values = {
    [0] = (void *)&test_epilogue_subprog,
    }
    };
    SEC("struct_ops/test_epilogue_tailcall")
#[no_mangle]
pub unsafe extern "C" fn test_epilogue_tailcall(ctx: *mut c_ulonglong) -> c_int {
    int test_epilogue_tailcall(unsigned long long *ctx)
    {
    bpf_tail_call(ctx, &epilogue_map, 0);
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_st_ops epilogue_tailcall = {
    .test_epilogue = (void *)test_epilogue_tailcall,
    };
    SEC(".struct_ops.link")
    struct bpf_testmod_st_ops epilogue_subprog = {
    .test_epilogue = (void *)test_epilogue_subprog,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn syscall_epilogue_tailcall(args: *mut st_ops_args) -> c_int {
    int syscall_epilogue_tailcall(struct st_ops_args *args)
    {
    return bpf_kfunc_st_ops_test_epilogue(args);
    }
