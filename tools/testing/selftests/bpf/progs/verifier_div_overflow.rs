//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_div_overflow.c
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
// Converted from tools/testing/selftests/bpf/verifier/div_overflow.c

// Just make sure that JITs used udiv/umod as otherwise we get
// an exception from INT_MIN/-1 overflow similarly as with div
// by zero.
//
    SEC("tc")
    __description("DIV32 overflow, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn div32_overflow_check_1() -> __naked void {
    __naked void div32_overflow_check_1(void)
    {
    asm volatile ("					\
    w1 = -1;					\
    w0 = %[int_min];				\
    w0 /= w1;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("DIV32 overflow, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn div32_overflow_check_2() -> __naked void {
    __naked void div32_overflow_check_2(void)
    {
    asm volatile ("					\
    w0 = %[int_min];				\
    w0 /= -1;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("DIV64 overflow, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn div64_overflow_check_1() -> __naked void {
    __naked void div64_overflow_check_1(void)
    {
    asm volatile ("					\
    r1 = -1;					\
    r2 = %[llong_min] ll;				\
    r2 /= r1;					\
    w0 = 0;						\
    if r0 == r2 goto l0_%=;				\
    w0 = 1;						\
    l0_%=:	exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("DIV64 overflow, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn div64_overflow_check_2() -> __naked void {
    __naked void div64_overflow_check_2(void)
    {
    asm volatile ("					\
    r1 = %[llong_min] ll;				\
    r1 /= -1;					\
    w0 = 0;						\
    if r0 == r1 goto l0_%=;				\
    w0 = 1;						\
    l0_%=:	exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("MOD32 overflow, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: _INT_MIN) -> __success {
    __success __retval(_INT_MIN)
#[no_mangle]
pub unsafe extern "C" fn mod32_overflow_check_1() -> __naked void {
    __naked void mod32_overflow_check_1(void)
    {
    asm volatile ("					\
    w1 = -1;					\
    w0 = %[int_min];				\
    w0 %%= w1;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("MOD32 overflow, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: _INT_MIN) -> __success {
    __success __retval(_INT_MIN)
#[no_mangle]
pub unsafe extern "C" fn mod32_overflow_check_2() -> __naked void {
    __naked void mod32_overflow_check_2(void)
    {
    asm volatile ("					\
    w0 = %[int_min];				\
    w0 %%= -1;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("MOD64 overflow, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mod64_overflow_check_1() -> __naked void {
    __naked void mod64_overflow_check_1(void)
    {
    asm volatile ("					\
    r1 = -1;					\
    r2 = %[llong_min] ll;				\
    r3 = r2;					\
    r2 %%= r1;					\
    w0 = 0;						\
    if r3 != r2 goto l0_%=;				\
    w0 = 1;						\
    l0_%=:	exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("tc")
    __description("MOD64 overflow, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn mod64_overflow_check_2() -> __naked void {
    __naked void mod64_overflow_check_2(void)
    {
    asm volatile ("					\
    r2 = %[llong_min] ll;				\
    r3 = r2;					\
    r2 %%= -1;					\
    w0 = 0;						\
    if r3 != r2 goto l0_%=;				\
    w0 = 1;						\
    l0_%=:	exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
