//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_bounds_deduction.c
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
// Converted from tools/testing/selftests/bpf/verifier/bounds_deduction.c

    SEC("socket")
    __description("check deducing bounds from const, 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar": "R0 tried to subtract pointer from) -> __failure {
    __failure __msg("R0 tried to subtract pointer from scalar")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_1() -> __naked void {
    __naked void deducing_bounds_from_const_1(void)
    {
    asm volatile ("					\
    r0 = 1;						\
    if r0 s>= 1 goto l0_%=;				\
    l0_%=:	r0 -= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 2")
    __success __failure_unpriv
    __msg_unpriv("R1 has pointer with unsupported alu operation")
    __retval(1)
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_2() -> __naked void {
    __naked void deducing_bounds_from_const_2(void)
    {
    asm volatile ("					\
    r0 = 1;						\
    if r0 s>= 1 goto l0_%=;				\
    exit;						\
    l0_%=:	if r0 s<= 1 goto l1_%=;				\
    exit;						\
    l1_%=:	r1 -= r0;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar": "R0 tried to subtract pointer from) -> __failure {
    __failure __msg("R0 tried to subtract pointer from scalar")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_3() -> __naked void {
    __naked void deducing_bounds_from_const_3(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 s<= 0 goto l0_%=;				\
    l0_%=:	r0 -= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 4")
    __success __failure_unpriv
    __msg_unpriv("R6 has pointer with unsupported alu operation")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_4() -> __naked void {
    __naked void deducing_bounds_from_const_4(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r0 = 0;						\
    if r0 s<= 0 goto l0_%=;				\
    exit;						\
    l0_%=:	if r0 s>= 0 goto l1_%=;				\
    exit;						\
    l1_%=:	r6 -= r0;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 5")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar": "R0 tried to subtract pointer from) -> __failure {
    __failure __msg("R0 tried to subtract pointer from scalar")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_5() -> __naked void {
    __naked void deducing_bounds_from_const_5(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 s>= 1 goto l0_%=;				\
    r0 -= r1;					\
    l0_%=:	exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 6")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar": "R0 tried to subtract pointer from) -> __failure {
    __failure __msg("R0 tried to subtract pointer from scalar")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_6() -> __naked void {
    __naked void deducing_bounds_from_const_6(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 s>= 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r0 -= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 7")
#[no_mangle]
pub unsafe extern "C" fn __msg(ptr": "dereference of modified ctx) -> __failure {
    __failure __msg("dereference of modified ctx ptr")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_7() -> __naked void {
    __naked void deducing_bounds_from_const_7(void)
    {
    asm volatile ("					\
    r0 = %[__imm_0];				\
    if r0 s>= 0 goto l0_%=;				\
    l0_%=:	r1 -= r0;					\
    r0 = *(u32*)(r1 + %[__sk_buff_mark]);		\
    exit;						\
    "	:
    : __imm_const(__imm_0, ~0),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 8")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "negative offset ctx ptr R1 off=-1) -> __failure {
    __failure __msg("negative offset ctx ptr R1 off=-1 disallowed")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_8() -> __naked void {
    __naked void deducing_bounds_from_const_8(void)
    {
    asm volatile ("					\
    r0 = %[__imm_0];				\
    if r0 s>= 0 goto l0_%=;				\
    r1 += r0;					\
    l0_%=:	r0 = *(u32*)(r1 + %[__sk_buff_mark]);		\
    exit;						\
    "	:
    : __imm_const(__imm_0, ~0),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 9")
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar": "R0 tried to subtract pointer from) -> __failure {
    __failure __msg("R0 tried to subtract pointer from scalar")
    __msg_unpriv("R1 has pointer with unsupported alu operation")
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_9() -> __naked void {
    __naked void deducing_bounds_from_const_9(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 s>= 0 goto l0_%=;				\
    l0_%=:	r0 -= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("check deducing bounds from const, 10")
    __failure
    __msg("math between ctx pointer and register with unbounded min value is not allowed")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn deducing_bounds_from_const_10() -> __naked void {
    __naked void deducing_bounds_from_const_10(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r0 = 0;						\
    if r0 s<= 0 goto l0_%=;				\
    l0_%=: /* Marks r0 as unknown. */			\
    call %[bpf_get_prandom_u32];			\
    r0 -= r6;					\
    exit;						\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
