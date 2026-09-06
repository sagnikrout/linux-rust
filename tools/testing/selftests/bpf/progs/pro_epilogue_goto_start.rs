//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/pro_epilogue_goto_start.c
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
// prologue
    __xlated("0: r6 = *(u64 *)(r1 +0)")
    __xlated("1: r7 = *(u64 *)(r6 +0)")
    __xlated("2: r7 += 1000")
    __xlated("3: *(u64 *)(r6 +0) = r7")
// main prog
    __xlated("4: if r1 == 0x0 goto pc+5")
    __xlated("5: if r1 == 0x1 goto pc+2")
    __xlated("6: r1 = 1")
    __xlated("7: goto pc-3")
    __xlated("8: r1 = 0")
    __xlated("9: goto pc-6")
    __xlated("10: r0 = 0")
    __xlated("11: exit")
    SEC("struct_ops/test_prologue_goto_start")
#[no_mangle]
pub unsafe extern "C" fn test_prologue_goto_start() -> __naked int {
    __naked int test_prologue_goto_start(void)
    {
    asm volatile (
    "if r1 == 0 goto +5;"
    "if r1 == 1 goto +2;"
    "r1 = 1;"
    "goto -3;"
    "r1 = 0;"
    "goto -6;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    __success
// save __u64 *ctx to stack
    __xlated("0: *(u64 *)(r10 -8) = r1")
// main prog
    __xlated("1: if r1 == 0x0 goto pc+5")
    __xlated("2: if r1 == 0x1 goto pc+2")
    __xlated("3: r1 = 1")
    __xlated("4: goto pc-3")
    __xlated("5: r1 = 0")
    __xlated("6: goto pc-6")
    __xlated("7: r0 = 0")
// epilogue
    __xlated("8: r1 = *(u64 *)(r10 -8)")
    __xlated("9: r1 = *(u64 *)(r1 +0)")
    __xlated("10: r6 = *(u64 *)(r1 +0)")
    __xlated("11: r6 += 10000")
    __xlated("12: *(u64 *)(r1 +0) = r6")
    __xlated("13: r0 = r6")
    __xlated("14: r0 *= 2")
    __xlated("15: exit")
    SEC("struct_ops/test_epilogue_goto_start")
#[no_mangle]
pub unsafe extern "C" fn test_epilogue_goto_start() -> __naked int {
    __naked int test_epilogue_goto_start(void)
    {
    asm volatile (
    "if r1 == 0 goto +5;"
    "if r1 == 1 goto +2;"
    "r1 = 1;"
    "goto -3;"
    "r1 = 0;"
    "goto -6;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
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
    __xlated("5: if r1 == 0x0 goto pc+5")
    __xlated("6: if r1 == 0x1 goto pc+2")
    __xlated("7: r1 = 1")
    __xlated("8: goto pc-3")
    __xlated("9: r1 = 0")
    __xlated("10: goto pc-6")
    __xlated("11: r0 = 0")
// epilogue
    __xlated("12: r1 = *(u64 *)(r10 -8)")
    __xlated("13: r1 = *(u64 *)(r1 +0)")
    __xlated("14: r6 = *(u64 *)(r1 +0)")
    __xlated("15: r6 += 10000")
    __xlated("16: *(u64 *)(r1 +0) = r6")
    __xlated("17: r0 = r6")
    __xlated("18: r0 *= 2")
    __xlated("19: exit")
    SEC("struct_ops/test_pro_epilogue_goto_start")
#[no_mangle]
pub unsafe extern "C" fn test_pro_epilogue_goto_start() -> __naked int {
    __naked int test_pro_epilogue_goto_start(void)
    {
    asm volatile (
    "if r1 == 0 goto +5;"
    "if r1 == 1 goto +2;"
    "r1 = 1;"
    "goto -3;"
    "r1 = 0;"
    "goto -6;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_st_ops epilogue_goto_start = {
    .test_prologue = (void *)test_prologue_goto_start,
    .test_epilogue = (void *)test_epilogue_goto_start,
    .test_pro_epilogue = (void *)test_pro_epilogue_goto_start,
    };
    SEC("syscall")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn syscall_prologue_goto_start(ctx: *mut c_void) -> c_int {
    int syscall_prologue_goto_start(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_prologue(&args);
    }
    SEC("syscall")
    __retval(20000) /* (EPILOGUE_A [10000]) * 2 */
#[no_mangle]
pub unsafe extern "C" fn syscall_epilogue_goto_start(ctx: *mut c_void) -> c_int {
    int syscall_epilogue_goto_start(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_epilogue(&args);
    }
    SEC("syscall")
    __retval(22000) /* (PROLOGUE_A [1000] + EPILOGUE_A [10000]) * 2 */
#[no_mangle]
pub unsafe extern "C" fn syscall_pro_epilogue_goto_start(ctx: *mut c_void) -> c_int {
    int syscall_pro_epilogue_goto_start(void *ctx)
    {
    let mut args: st_ops_args = {};
    return bpf_kfunc_st_ops_test_pro_epilogue(&args);
    }
