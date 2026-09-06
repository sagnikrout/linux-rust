//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_leak_ptr.c
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
// Converted from tools/testing/selftests/bpf/verifier/leak_ptr.c

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("socket")
    __description("leak pointer into ctx 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC stores into R1 ctx is not) -> __failure {
    __failure __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(mem": "R2 leaks addr into) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R2 leaks addr into mem")
#[no_mangle]
pub unsafe extern "C" fn leak_pointer_into_ctx_1() -> __naked void {
    __naked void leak_pointer_into_ctx_1(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    r2 = %[map_hash_8b] ll;				\
    lock *(u64 *)(r1 + %[__sk_buff_cb_0]) += r2;	\
    exit;						\
    "	:
    : __imm_addr(map_hash_8b),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("leak pointer into ctx 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC stores into R1 ctx is not) -> __failure {
    __failure __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(mem": "R10 leaks addr into) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R10 leaks addr into mem")
#[no_mangle]
pub unsafe extern "C" fn leak_pointer_into_ctx_2() -> __naked void {
    __naked void leak_pointer_into_ctx_2(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r1 + %[__sk_buff_cb_0]) = r0;		\
    lock *(u64 *)(r1 + %[__sk_buff_cb_0]) += r10;	\
    exit;						\
    "	:
    : __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("leak pointer into ctx 3")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(ctx": "R2 leaks addr into) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R2 leaks addr into ctx")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn leak_pointer_into_ctx_3() -> __naked void {
    __naked void leak_pointer_into_ctx_3(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    r2 = %[map_hash_8b] ll;				\
// (u64*)(r1 + %[__sk_buff_cb_0]) = r2;		\
    exit;						\
    "	:
    : __imm_addr(map_hash_8b),
    __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0]))
    : __clobber_all);
    }
    SEC("socket")
    __description("leak pointer into map val")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(mem": "R6 leaks addr into) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R6 leaks addr into mem")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn leak_pointer_into_map_val() -> __naked void {
    __naked void leak_pointer_into_map_val(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r3 = 0;						\
// (u64*)(r0 + 0) = r3;				\
    lock *(u64 *)(r0 + 0) += r6;			\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
