//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_looping.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";

    __imm(bpf_iter_num_new),				\
    __imm(bpf_iter_num_next),				\
    __imm(bpf_iter_num_destroy)
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn force_clang_to_emit_btf_for_externs(ctx: *mut c_void) -> c_int {
    int force_clang_to_emit_btf_for_externs(void *ctx)
    {
// we need this as a workaround to enforce compiler emitting BTF
// information for bpf_iter_num_{new,next,destroy}() kfuncs,
// as, apparently, it doesn't emit it for symbols only referenced from
// assembly (or cleanup attribute, for that matter, as well)
//
    bpf_repeat(0);
    return 0;
    }
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn consume_first_item_only(ctx: *mut c_void) -> c_int {
    int consume_first_item_only(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// consume first item
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
    "if r0 == 0 goto +1;"
    "r0 = *(u32 *)(r0 + 0);"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn missing_null_check_fail(ctx: *mut c_void) -> c_int {
    int missing_null_check_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// consume first element
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
// FAIL: deref with no NULL check
    "r1 = *(u32 *)(r0 + 0);"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
    __failure
    __msg("invalid access to memory, mem_size=4 off=0 size=8")
    __msg("R0 min value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn wrong_sized_read_fail(ctx: *mut c_void) -> c_int {
    int wrong_sized_read_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// consume first element
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
    "if r0 == 0 goto +1;"
// FAIL: deref more than available 4 bytes
    "r0 = *(u64 *)(r0 + 0);"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn simplest_loop(ctx: *mut c_void) -> c_int {
    int simplest_loop(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
    "r6 = 0;" /* init sum */
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 10;"
    "call %[bpf_iter_num_new];"
    "1:"
// consume next item
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
    "if r0 == 0 goto 2f;"
    "r0 = *(u32 *)(r0 + 0);"
    "r6 += r0;" /* accumulate sum */
    "goto 1b;"
    "2:"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common, "r6"
    );
    return 0;
    }
    __used
#[no_mangle]
unsafe extern "C" fn iterator_with_diff_stack_depth(x: c_int) {
    static void iterator_with_diff_stack_depth(int x)
    {
    struct bpf_iter_num iter;
    asm volatile (
    "if r1 == 42 goto 0f;"
    "*(u64 *)(r10 - 128) = 0;"
    "0:"
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 10;"
    "call %[bpf_iter_num_new];"
    "1:"
// consume next item
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
    "if r0 == 0 goto 2f;"
    "goto 1b;"
    "2:"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common, "r6"
    );
    }
    SEC("socket")
    __success
#[no_mangle]
pub unsafe extern "C" fn widening_stack_size_bug(ctx: *mut c_void) -> __naked int {
    __naked int widening_stack_size_bug(void *ctx)
    {
//
// Depending on iterator_with_diff_stack_depth() parameter value,
// subprogram stack depth is either 8 or 128 bytes. Arrange values so
// that it is 128 on a first call and 8 on a second. This triggered a
// bug in verifier's widen_imprecise_scalars() logic.
//
    asm volatile (
    "r6 = 0;"
    "r1 = 0;"
    "1:"
    "call iterator_with_diff_stack_depth;"
    "r1 = 42;"
    "r6 += 1;"
    "if r6 < 2 goto 1b;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
