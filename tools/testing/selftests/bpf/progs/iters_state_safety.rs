//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_state_safety.c
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
// Copyright (c) 2022 Facebook

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
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("fp-8=iter_num(id=1,state=active,depth=0)")
#[no_mangle]
pub unsafe extern "C" fn create_and_destroy(ctx: *mut c_void) -> c_int {
    int create_and_destroy(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
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
pub unsafe extern "C" fn __msg(id=1": "Unreleased reference) -> __failure {
    __failure __msg("Unreleased reference id=1")
#[no_mangle]
pub unsafe extern "C" fn create_and_forget_to_destroy_fail(ctx: *mut c_void) -> c_int {
    int create_and_forget_to_destroy_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_num as) -> __failure {
    __failure __msg("expected an initialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn destroy_without_creating_fail(ctx: *mut c_void) -> c_int {
    int destroy_without_creating_fail(void *ctx)
    {
// init with zeros to stop verifier complaining about uninit stack
    struct bpf_iter_num iter;
    asm volatile (
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
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_num as) -> __failure {
    __failure __msg("expected an initialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn compromise_iter_w_direct_write_fail(ctx: *mut c_void) -> c_int {
    int compromise_iter_w_direct_write_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// directly write over first half of iter state
    "*(u64 *)(%[iter] + 0) = r0;"
// (attempt to) destroy iterator
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
pub unsafe extern "C" fn __msg(id=1": "Unreleased reference) -> __failure {
    __failure __msg("Unreleased reference id=1")
#[no_mangle]
pub unsafe extern "C" fn compromise_iter_w_direct_write_and_skip_destroy_fail(ctx: *mut c_void) -> c_int {
    int compromise_iter_w_direct_write_and_skip_destroy_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// directly write over first half of iter state
    "*(u64 *)(%[iter] + 0) = r0;"
// don't destroy iter, leaking ref, which should fail
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_num as) -> __failure {
    __failure __msg("expected an initialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn compromise_iter_w_helper_write_fail(ctx: *mut c_void) -> c_int {
    int compromise_iter_w_helper_write_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// overwrite 8th byte with bpf_probe_read_kernel()
    "r1 = %[iter];"
    "r1 += 7;"
    "r2 = 1;"
    "r3 = 0;" /* core::ptr::null_mut() */
    "call %[bpf_probe_read_kernel];"
// (attempt to) destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
    :
    : __imm_ptr(iter), ITER_HELPERS, __imm(bpf_probe_read_kernel)
    : __clobber_common
    );
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn subprog_with_iter() -> __noinline void {
    static __noinline void subprog_with_iter(void)
    {
    struct bpf_iter_num iter;
    bpf_iter_num_new(&iter, 0, 1);
    return;
    }
    SEC("?raw_tp")
    __failure
// ensure there was a call to subprog, which might happen without __noinline
    __msg("returning from callee:")
    __msg("Unreleased reference id=1")
#[no_mangle]
pub unsafe extern "C" fn leak_iter_from_subprog_fail(ctx: *mut c_void) -> c_int {
    int leak_iter_from_subprog_fail(void *ctx)
    {
    subprog_with_iter();
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("fp-8=iter_num(id=1,state=active,depth=0)")
#[no_mangle]
pub unsafe extern "C" fn valid_stack_reuse(ctx: *mut c_void) -> c_int {
    int valid_stack_reuse(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
// now reuse same stack slots
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
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
pub unsafe extern "C" fn __msg(R1": "expected uninitialized iter_num as) -> __failure {
    __failure __msg("expected uninitialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn double_create_fail(ctx: *mut c_void) -> c_int {
    int double_create_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// (attempt to) create iterator again
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
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
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_num as) -> __failure {
    __failure __msg("expected an initialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn double_destroy_fail(ctx: *mut c_void) -> c_int {
    int double_destroy_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
// (attempt to) destroy iterator again
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
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_num as) -> __failure {
    __failure __msg("expected an initialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn next_without_new_fail(ctx: *mut c_void) -> c_int {
    int next_without_new_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// don't create iterator and try to iterate
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
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
pub unsafe extern "C" fn __msg(R1": "expected an initialized iter_num as) -> __failure {
    __failure __msg("expected an initialized iter_num as R1")
#[no_mangle]
pub unsafe extern "C" fn next_after_destroy_fail(ctx: *mut c_void) -> c_int {
    int next_after_destroy_fail(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// create iterator
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// destroy iterator
    "r1 = %[iter];"
    "call %[bpf_iter_num_destroy];"
// don't create iterator and try to iterate
    "r1 = %[iter];"
    "call %[bpf_iter_num_next];"
    :
    : __imm_ptr(iter), ITER_HELPERS
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid read from) -> __failure {
    __failure __msg("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn read_from_iter_slot_fail() -> int __naked {
    int __naked read_from_iter_slot_fail(void)
    {
    asm volatile (
// r6 points to struct bpf_iter_num on the stack
    "r6 = r10;"
    "r6 += -24;"
// create iterator
    "r1 = r6;"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// attempt to leak bpf_iter_num state
    "r7 = *(u64 *)(r6 + 0);"
    "r8 = *(u64 *)(r6 + 8);"
// destroy iterator
    "r1 = r6;"
    "call %[bpf_iter_num_destroy];"
// leak bpf_iter_num state
    "r0 = r7;"
    "if r7 > r8 goto +1;"
    "r0 = r8;"
    "exit;"
    :
    : ITER_HELPERS
    : __clobber_common, "r6", "r7", "r8"
    );
    }
    int zero;
    SEC("?raw_tp")
    __failure
    __flag(BPF_F_TEST_STATE_FREQ)
    __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn stacksafe_should_not_conflate_stack_spill_and_iter(ctx: *mut c_void) -> c_int {
    int stacksafe_should_not_conflate_stack_spill_and_iter(void *ctx)
    {
    struct bpf_iter_num iter;
    asm volatile (
// Create a fork in logic, with general setup as follows:
// - fallthrough (first) path is valid;
// - branch (second) path is invalid.
// Then depending on what we do in fallthrough vs branch path,
// we try to detect bugs in func_states_equal(), regsafe(),
// refsafe(), stack_safe(), and similar by tricking verifier
// into believing that branch state is a valid subset of
// a fallthrough state. Verifier should reject overall
// validation, unless there is a bug somewhere in verifier
// logic.
//
    "call %[bpf_get_prandom_u32];"
    "r6 = r0;"
    "call %[bpf_get_prandom_u32];"
    "r7 = r0;"
    "if r6 > r7 goto bad;" /* fork */
// spill r6 into stack slot of bpf_iter_num var
    "*(u64 *)(%[iter] + 0) = r6;"
    "goto skip_bad;"
    "bad:"
// create iterator in the same stack slot
    "r1 = %[iter];"
    "r2 = 0;"
    "r3 = 1000;"
    "call %[bpf_iter_num_new];"
// but then forget about it and overwrite it back to r6 spill
    "*(u64 *)(%[iter] + 0) = r6;"
    "skip_bad:"
    "goto +0;" /* force checkpoint */
// corrupt stack slots, if they are really dynptr
    "*(u64 *)(%[iter] + 0) = r6;"
    :
    : __imm_ptr(iter),
    __imm_addr(zero),
    __imm(bpf_get_prandom_u32),
    __imm(bpf_dynptr_from_mem),
    ITER_HELPERS
    : __clobber_common, "r6", "r7"
    );
    return 0;
    }
