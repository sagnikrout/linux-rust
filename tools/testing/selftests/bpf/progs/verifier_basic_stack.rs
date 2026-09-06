//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_basic_stack.c
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
// Converted from tools/testing/selftests/bpf/verifier/basic_stack.c

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("socket")
    __description("stack out of bounds")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "invalid write to) -> __failure {
    __failure __msg("invalid write to stack")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn stack_out_of_bounds() -> __naked void {
    __naked void stack_out_of_bounds(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 + 8) = r1;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("uninitialized stack1")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
    __msg("subprog 0 (uninitialized_stack1) main {{.*}} stack 8")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(stack": "invalid read from) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn uninitialized_stack1() -> __naked void {
    __naked void uninitialized_stack1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("uninitialized stack2")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
    __msg("subprog 0 (uninitialized_stack2) main insns_self {{[0-9]+}} insns_total {{[0-9]+}} stack 8")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(stack": "invalid read from) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn uninitialized_stack2() -> __naked void {
    __naked void uninitialized_stack2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r0 = *(u64*)(r2 - 8);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("invalid fp arithmetic")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R1 subtraction from stack) -> __failure {
    __failure __msg("R1 subtraction from stack pointer")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_fp_arithmetic() -> __naked void {
    __naked void invalid_fp_arithmetic(void)
    {
// If this gets ever changed, make sure JITs can deal with it.
    asm volatile ("					\
    r0 = 0;						\
    r1 = r10;					\
    r1 -= 8;					\
// (u64*)(r1 + 0) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("non-invalid fp arithmetic")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn non_invalid_fp_arithmetic() -> __naked void {
    __naked void non_invalid_fp_arithmetic(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r10 - 8) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("misaligned read from stack")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "misaligned stack) -> __failure {
    __failure __msg("misaligned stack access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn misaligned_read_from_stack() -> __naked void {
    __naked void misaligned_read_from_stack(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r0 = *(u64*)(r2 - 4);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("stack pointer arithmetic preserves frame number")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "R7 invalid mem access) -> __failure {
    __failure __msg("R7 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn stack_ptr_arith_preserves_frameno() -> __naked void {
    __naked void stack_ptr_arith_preserves_frameno(void)
    {
    asm volatile ("\
    r3 = 0;						\
// (u64 *)(r10 - 8) = r3;			\
    r1 = %[map_hash_8b] ll;			\
    r2 = r10;					\
    r2 += -8;					\
    call %[bpf_map_lookup_elem];		\
    if r0 != 0 goto +2;			\
    r0 = 0;						\
    exit;						\
    r1 = r0;					\
    r2 = 0;						\
    r3 = 0;						\
    call stack_ptr_arith_preserves_frameno_subprog;\
    r0 = 0;						\
    exit;						\
    ":
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
#[no_mangle]
unsafe extern "C" fn stack_ptr_arith_preserves_frameno_subprog() -> __used __naked void {
    static __used __naked void stack_ptr_arith_preserves_frameno_subprog(void)
    {
    asm volatile ("\
// (u64 *)(r10 - 8) = r1;			\
    r6 = -8;					\
    r6 += r10;					\
// (u64 *)(r6 + 0) = r2;			\
    r7 = *(u64 *)(r10 - 8);			\
// (u64 *)(r7 + 0) = r3;			\
    r0 = 0;						\
    exit;						\
    "::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
