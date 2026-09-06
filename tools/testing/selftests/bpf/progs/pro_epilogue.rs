//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/pro_epilogue.c
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
pub unsafe extern "C" fn __kfunc_btf_root() {
    void __kfunc_btf_root(void)
    {
    bpf_kfunc_st_ops_inc10(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn subprog(args: *mut st_ops_args) -> __noinline __used int {
    static __noinline __used int subprog(struct st_ops_args *args)
    {
    args.a += 1;
    return args.a;
    }
    __success
// prologue
    __xlated("0: r6 = *(u64 *)(r1 +0)")
    __xlated("1: r7 = *(u64 *)(r6 +0)")
    __xlated("2: r7 += 1000")
    __xlated("3: *(u64 *)(r6 +0) = r7")
// main prog
    __xlated("4: r1 = *(u64 *)(r1 +0)")
    __xlated("5: r6 = r1")
    __xlated("6: call kernel-function")
    __xlated("7: r1 = r6")
    __xlated("8: call pc+1")
    __xlated("9: exit")
    SEC("struct_ops/test_prologue")
#[no_mangle]
pub unsafe extern "C" fn test_prologue() -> __naked int {
    __naked int test_prologue(void)
    {
    asm volatile (
    "r1 = *(u64 *)(r1 +0);"
    "r6 = r1;"
    "call %[bpf_kfunc_st_ops_inc10];"
    "r1 = r6;"
    "call subprog;"
    "exit;"
    :
    : __imm(bpf_kfunc_st_ops_inc10)
    : __clobber_all);
    }
    __success
// save __u64 *ctx to stack
    __xlated("0: *(u64 *)(r10 -8) = r1")
// main prog
    __xlated("1: r1 = *(u64 *)(r1 +0)")
    __xlated("2: r6 = r1")
    __xlated("3: call kernel-function")
    __xlated("4: r1 = r6")
    __xlated("5: call pc+")
// epilogue
    __xlated("6: r1 = *(u64 *)(r10 -8)")
    __xlated("7: r1 = *(u64 *)(r1 +0)")
    __xlated("8: r6 = *(u64 *)(r1 +0)")
    __xlated("9: r6 += 10000")
    __xlated("10: *(u64 *)(r1 +0) = r6")
    __xlated("11: r0 = r6")
    __xlated("12: r0 *= 2")
    __xlated("13: exit")
    SEC("struct_ops/test_epilogue")
#[no_mangle]
pub unsafe extern "C" fn test_epilogue() -> __naked int {
    __naked int test_epilogue(void)
    {
    asm volatile (
    "r1 = *(u64 *)(r1 +0);"
    "r6 = r1;"
    "call %[bpf_kfunc_st_ops_inc10];"
    "r1 = r6;"
    "call subprog;"
    "exit;"
    :
    : __imm(bpf_kfunc_st_ops_inc10)
    : __clobber_all);
    }
    __success
// prologue
    __xlated("0: r6 = *(u64 *)(r1 +0)")
    __xlated("1: r7 = *(u64 *)(r6 +0)")
    __xlated("2: r7 += 1000")
    __xlated("3: *(u64 *)(r6 +0) = r7")
// save __u64 *ctx to stack
    __xlated("4: *(u64 *)(r10 -8) = r1")
// main prog
    __xlated("5: r1 = *(u64 *)(r1 +0)")
    __xlated("6: r6 = r1")
    __xlated("7: call kernel-function")
    __xlated("8: r1 = r6")
    __xlated("9: call pc+")
// epilogue
    __xlated("10: r1 = *(u64 *)(r10 -8)")
    __xlated("11: r1 = *(u64 *)(r1 +0)")
    __xlated("12: r6 = *(u64 *)(r1 +0)")
    __xlated("13: r6 += 10000")
    __xlated("14: *(u64 *)(r1 +0) = r6")
    __xlated("15: r0 = r6")
    __xlated("16: r0 *= 2")
    __xlated("17: exit")
    SEC("struct_ops/test_pro_epilogue")
#[no_mangle]
pub unsafe extern "C" fn test_pro_epilogue() -> __naked int {
    __naked int test_pro_epilogue(void)
    {
    asm volatile (
    "r1 = *(u64 *)(r1 +0);"
    "r6 = r1;"
    "call %[bpf_kfunc_st_ops_inc10];"
    "r1 = r6;"
    "call subprog;"
    "exit;"
    :
    : __imm(bpf_kfunc_st_ops_inc10)
    : __clobber_all);
    }
    SEC("syscall")
    __retval(1011) /* PROLOGUE_A [1000] + KFUNC_INC10 + SUBPROG_A [1] */
#[no_mangle]
pub unsafe extern "C" fn syscall_prologue(ctx: *mut c_void) -> c_int {
    int syscall_prologue(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_prologue(&args);
    }
    SEC("syscall")
    __retval(20022) /* (KFUNC_INC10 + SUBPROG_A [1] + EPILOGUE_A [10000]) * 2 */
#[no_mangle]
pub unsafe extern "C" fn syscall_epilogue(ctx: *mut c_void) -> c_int {
    int syscall_epilogue(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_epilogue(&args);
    }
    SEC("syscall")
    __retval(22022) /* (PROLOGUE_A [1000] + KFUNC_INC10 + SUBPROG_A [1] + EPILOGUE_A [10000]) * 2 */
#[no_mangle]
pub unsafe extern "C" fn syscall_pro_epilogue(ctx: *mut c_void) -> c_int {
    int syscall_pro_epilogue(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_pro_epilogue(&args);
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_st_ops pro_epilogue = {
    .test_prologue = (void *)test_prologue,
    .test_epilogue = (void *)test_epilogue,
    .test_pro_epilogue = (void *)test_pro_epilogue,
    };
