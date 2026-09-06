//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_div0.c
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
// Converted from tools/testing/selftests/bpf/verifier/div0.c

    SEC("socket")
    __description("DIV32 by 0, zero check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn by_0_zero_check_1_1() -> __naked void {
    __naked void by_0_zero_check_1_1(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 0;						\
    w2 = 1;						\
    w2 /= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("DIV32 by 0, zero check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn by_0_zero_check_2_1() -> __naked void {
    __naked void by_0_zero_check_2_1(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    r1 = 0xffffffff00000000LL ll;			\
    w2 = 1;						\
    w2 /= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("DIV64 by 0, zero check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn div64_by_0_zero_check() -> __naked void {
    __naked void div64_by_0_zero_check(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 0;						\
    w2 = 1;						\
    r2 /= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOD32 by 0, zero check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn by_0_zero_check_1_2() -> __naked void {
    __naked void by_0_zero_check_1_2(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 0;						\
    w2 = 1;						\
    w2 %%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOD32 by 0, zero check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn by_0_zero_check_2_2() -> __naked void {
    __naked void by_0_zero_check_2_2(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    r1 = 0xffffffff00000000LL ll;			\
    w2 = 1;						\
    w2 %%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("MOD64 by 0, zero check")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success __success_unpriv {
    __success __success_unpriv __retval(42)
#[no_mangle]
pub unsafe extern "C" fn mod64_by_0_zero_check() -> __naked void {
    __naked void mod64_by_0_zero_check(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 0;						\
    w2 = 1;						\
    r2 %%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("DIV32 by 0, zero check ok, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 8) -> __success {
    __success __retval(8)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_ok_cls_1() -> __naked void {
    __naked void _0_zero_check_ok_cls_1(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 2;						\
    w2 = 16;					\
    w2 /= w1;					\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("DIV32 by 0, zero check 1, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_1_cls_1() -> __naked void {
    __naked void _0_zero_check_1_cls_1(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w0 = 1;						\
    w0 /= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("DIV32 by 0, zero check 2, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_2_cls_1() -> __naked void {
    __naked void _0_zero_check_2_cls_1(void)
    {
    asm volatile ("					\
    r1 = 0xffffffff00000000LL ll;			\
    w0 = 1;						\
    w0 /= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("DIV64 by 0, zero check, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn by_0_zero_check_cls() -> __naked void {
    __naked void by_0_zero_check_cls(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w0 = 1;						\
    r0 /= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("MOD32 by 0, zero check ok, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 2) -> __success {
    __success __retval(2)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_ok_cls_2() -> __naked void {
    __naked void _0_zero_check_ok_cls_2(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 3;						\
    w2 = 5;						\
    w2 %%= w1;					\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("MOD32 by 0, zero check 1, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_1_cls_2() -> __naked void {
    __naked void _0_zero_check_1_cls_2(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w0 = 1;						\
    w0 %%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("MOD32 by 0, zero check 2, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_2_cls_2() -> __naked void {
    __naked void _0_zero_check_2_cls_2(void)
    {
    asm volatile ("					\
    r1 = 0xffffffff00000000LL ll;			\
    w0 = 1;						\
    w0 %%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("MOD64 by 0, zero check 1, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 2) -> __success {
    __success __retval(2)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_1_cls_3() -> __naked void {
    __naked void _0_zero_check_1_cls_3(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w0 = 2;						\
    r0 %%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("MOD64 by 0, zero check 2, cls")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success {
    __success __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn _0_zero_check_2_cls_3() -> __naked void {
    __naked void _0_zero_check_2_cls_3(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w0 = -1;					\
    r0 %%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
