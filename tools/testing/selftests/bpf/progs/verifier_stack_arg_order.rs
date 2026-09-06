//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_stack_arg_order.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    defined(__BPF_FEATURE_STACK_ARGUMENT)
    __noinline __used __naked
#[no_mangle]
unsafe extern "C" fn subprog_bad_order_6args(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int) -> c_int {
    static int subprog_bad_order_6args(int a, int b, int c, int d, int e, int f)
    {
    asm volatile (
    "*(u64 *)(r11 - 8) = r1;"
    "r0 = *(u64 *)(r11 + 8);"
    "exit;"
    ::: __clobber_all
    );
    }
    SEC("tc")
    __description("stack_arg: r11 load after r11 store")
    __failure
    __msg("r11 load must be before any r11 store or call insn")
    __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[no_mangle]
pub unsafe extern "C" fn stack_arg_load_after_store() -> __naked void {
    __naked void stack_arg_load_after_store(void)
    {
    asm volatile (
    "r1 = 1;"
    "r2 = 2;"
    "r3 = 3;"
    "r4 = 4;"
    "r5 = 5;"
    "*(u64 *)(r11 - 8) = 6;"
    "call subprog_bad_order_6args;"
    "exit;"
    ::: __clobber_all
    );
    }
    __noinline __used __naked
    static int subprog_call_before_load_6args(int a, int b, int c, int d, int e,
    int f)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "r0 = *(u64 *)(r11 + 8);"
    "exit;"
    :: __imm(bpf_get_prandom_u32)
    : __clobber_all
    );
    }
    SEC("tc")
    __description("stack_arg: r11 load after a call")
    __failure
    __msg("r11 load must be before any r11 store or call insn")
    __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[no_mangle]
pub unsafe extern "C" fn stack_arg_load_after_call() -> __naked void {
    __naked void stack_arg_load_after_call(void)
    {
    asm volatile (
    "r1 = 1;"
    "r2 = 2;"
    "r3 = 3;"
    "r4 = 4;"
    "r5 = 5;"
    "*(u64 *)(r11 - 8) = 6;"
    "call subprog_call_before_load_6args;"
    "exit;"
    ::: __clobber_all
    );
    }
    __noinline __used __naked
    static int subprog_pruning_call_before_load_6args(int a, int b, int c, int d,
    int e, int f)
    {
    asm volatile (
    "if r1 s> 0 goto l0_%=;"
    "goto l1_%=;"
    "l0_%=:"
    "call %[bpf_get_prandom_u32];"
    "l1_%=:"
    "r0 = *(u64 *)(r11 + 8);"
    "exit;"
    :: __imm(bpf_get_prandom_u32)
    : __clobber_all
    );
    }
    SEC("tc")
    __description("stack_arg: pruning keeps r11 load ordering")
    __failure
    __flag(BPF_F_TEST_STATE_FREQ)
    __msg("r11 load must be before any r11 store or call insn")
    __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[no_mangle]
pub unsafe extern "C" fn stack_arg_pruning_load_after_call() -> __naked void {
    __naked void stack_arg_pruning_load_after_call(void)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "r1 = r0;"
    "r2 = 2;"
    "r3 = 3;"
    "r4 = 4;"
    "r5 = 5;"
    "*(u64 *)(r11 - 8) = 6;"
    "call subprog_pruning_call_before_load_6args;"
    "exit;"
    :: __imm(bpf_get_prandom_u32)
    : __clobber_all
    );
    }
//
// "bad_ptr": the first arg is 'long *', which is not a recognized pointer
// type for static subprogs (not ctx, dynptr, or tagged).  btf_prepare_func_args()
// sets arg_cnt = 7 / stack_arg_cnt = 2, then fails with -EINVAL.  The subprog
// is marked unreliable but the call still proceeds for static subprogs.
//
    __noinline __used __naked
#[no_mangle]
unsafe extern "C" fn subprog_bad_ptr_7args(a: *mut c_long, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int, g: c_int) {
    static void subprog_bad_ptr_7args(long *a, int b, int c, int d, int e, int f, int g)
    {
    asm volatile (
    "r0 = *(u64 *)(r11 + 8);"
    "r1 = *(u64 *)(r11 + 16);"
    "exit;"
    ::: __clobber_all
    );
    }
    SEC("tc")
    __description("stack_arg: read without caller write")
    __failure
    __msg("callee expects 7 args, stack arg1 is not initialized")
    __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[no_mangle]
pub unsafe extern "C" fn stack_arg_read_without_write_1() -> __naked void {
    __naked void stack_arg_read_without_write_1(void)
    {
    asm volatile (
    "r1 = 0;"
    "r2 = 0;"
    "r3 = 0;"
    "r4 = 0;"
    "r5 = 0;"
    "call subprog_bad_ptr_7args;"
    "exit;"
    ::: __clobber_all
    );
    }
    SEC("tc")
    __description("stack_arg: read with not-initialized caller write")
    __failure
    __msg("R0 !read_ok")
    __btf_func_path("btf__verifier_stack_arg_order.bpf.o")
#[no_mangle]
pub unsafe extern "C" fn stack_arg_read_without_write_2() -> __naked void {
    __naked void stack_arg_read_without_write_2(void)
    {
    asm volatile (
    "r1 = 0;"
    "r2 = 0;"
    "r3 = 0;"
    "r4 = 0;"
    "r5 = 0;"
    "*(u64 *)(r11 - 8) = 0;"
    "*(u64 *)(r11 - 16) = 0;"
    "call subprog_bad_ptr_7args;"
    "call subprog_bad_ptr_7args;"
    "exit;"
    ::: __clobber_all
    );
    }

    SEC("socket")
    __description("stack_arg order is not supported by compiler or jit, use a dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
