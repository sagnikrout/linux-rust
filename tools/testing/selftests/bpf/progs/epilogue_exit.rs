//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/epilogue_exit.c
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
    __success
// save __u64 *ctx to stack
    __xlated("0: *(u64 *)(r10 -8) = r1")
// main prog
    __xlated("1: r1 = *(u64 *)(r1 +0)")
    __xlated("2: r2 = *(u64 *)(r1 +0)")
    __xlated("3: r3 = 0")
    __xlated("4: r4 = 1")
    __xlated("5: if r2 == 0x0 goto pc+10")
    __xlated("6: r0 = 0")
    __xlated("7: *(u64 *)(r1 +0) = r3")
// epilogue
    __xlated("8: r1 = *(u64 *)(r10 -8)")
    __xlated("9: r1 = *(u64 *)(r1 +0)")
    __xlated("10: r6 = *(u64 *)(r1 +0)")
    __xlated("11: r6 += 10000")
    __xlated("12: *(u64 *)(r1 +0) = r6")
    __xlated("13: r0 = r6")
    __xlated("14: r0 *= 2")
    __xlated("15: exit")
// 2nd part of the main prog after the first exit
    __xlated("16: *(u64 *)(r1 +0) = r4")
    __xlated("17: r0 = 1")
// Clear the r1 to ensure it does not have
// off-by-1 error and ensure it jumps back to the
// beginning of epilogue which initializes
// the r1 with the ctx ptr.
//
    __xlated("18: r1 = 0")
    __xlated("19: gotol pc-12")
    SEC("struct_ops/test_epilogue_exit")
#[no_mangle]
pub unsafe extern "C" fn test_epilogue_exit() -> __naked int {
    __naked int test_epilogue_exit(void)
    {
    asm volatile (
    "r1 = *(u64 *)(r1 +0);"
    "r2 = *(u64 *)(r1 +0);"
    "r3 = 0;"
    "r4 = 1;"
    "if r2 == 0 goto +3;"
    "r0 = 0;"
    "*(u64 *)(r1 + 0) = r3;"
    "exit;"
    "*(u64 *)(r1 + 0) = r4;"
    "r0 = 1;"
    "r1 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_st_ops epilogue_exit = {
    .test_epilogue = (void *)test_epilogue_exit,
    };
    SEC("syscall")
    __retval(20000)
#[no_mangle]
pub unsafe extern "C" fn syscall_epilogue_exit0(ctx: *mut c_void) -> c_int {
    int syscall_epilogue_exit0(void *ctx)
    {
    let mut args: st_ops_args = { .a = 1 };
    return bpf_kfunc_st_ops_test_epilogue(&args);
    }
    SEC("syscall")
    __retval(20002)
#[no_mangle]
pub unsafe extern "C" fn syscall_epilogue_exit1(ctx: *mut c_void) -> c_int {
    int syscall_epilogue_exit1(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_epilogue(&args);
    }
