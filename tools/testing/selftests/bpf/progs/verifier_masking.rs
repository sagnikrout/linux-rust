//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_masking.c
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
// Converted from tools/testing/selftests/bpf/verifier/masking.c

    SEC("socket")
    __description("masking, test out of bounds 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_1() -> __naked void {
    __naked void test_out_of_bounds_1(void)
    {
    asm volatile ("					\
    w1 = 5;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 5 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_2() -> __naked void {
    __naked void test_out_of_bounds_2(void)
    {
    asm volatile ("					\
    w1 = 1;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_3() -> __naked void {
    __naked void test_out_of_bounds_3(void)
    {
    asm volatile ("					\
    w1 = 0xffffffff;				\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xffffffff - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_4() -> __naked void {
    __naked void test_out_of_bounds_4(void)
    {
    asm volatile ("					\
    w1 = 0xffffffff;				\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_5() -> __naked void {
    __naked void test_out_of_bounds_5(void)
    {
    asm volatile ("					\
    w1 = -1;					\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_6() -> __naked void {
    __naked void test_out_of_bounds_6(void)
    {
    asm volatile ("					\
    w1 = -1;					\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xffffffff - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_7() -> __naked void {
    __naked void test_out_of_bounds_7(void)
    {
    asm volatile ("					\
    r1 = 5;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 5 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_8() -> __naked void {
    __naked void test_out_of_bounds_8(void)
    {
    asm volatile ("					\
    r1 = 1;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 9")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_9() -> __naked void {
    __naked void test_out_of_bounds_9(void)
    {
    asm volatile ("					\
    r1 = 0xffffffff;				\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xffffffff - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 10")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_10() -> __naked void {
    __naked void test_out_of_bounds_10(void)
    {
    asm volatile ("					\
    r1 = 0xffffffff;				\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 11")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_11() -> __naked void {
    __naked void test_out_of_bounds_11(void)
    {
    asm volatile ("					\
    r1 = -1;					\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test out of bounds 12")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_out_of_bounds_12() -> __naked void {
    __naked void test_out_of_bounds_12(void)
    {
    asm volatile ("					\
    r1 = -1;					\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xffffffff - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 4) -> __success __success_unpriv {
    __success __success_unpriv __retval(4)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_1() -> __naked void {
    __naked void masking_test_in_bounds_1(void)
    {
    asm volatile ("					\
    w1 = 4;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 5 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_2() -> __naked void {
    __naked void masking_test_in_bounds_2(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xffffffff - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0xfffffffe) -> __success __success_unpriv {
    __success __success_unpriv __retval(0xfffffffe)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_3() -> __naked void {
    __naked void masking_test_in_bounds_3(void)
    {
    asm volatile ("					\
    w1 = 0xfffffffe;				\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xffffffff - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0xabcde) -> __success __success_unpriv {
    __success __success_unpriv __retval(0xabcde)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_4() -> __naked void {
    __naked void masking_test_in_bounds_4(void)
    {
    asm volatile ("					\
    w1 = 0xabcde;					\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 0xabcdef - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_5() -> __naked void {
    __naked void masking_test_in_bounds_5(void)
    {
    asm volatile ("					\
    w1 = 0;						\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 1 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 46) -> __success __success_unpriv {
    __success __success_unpriv __retval(46)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_6() -> __naked void {
    __naked void masking_test_in_bounds_6(void)
    {
    asm volatile ("					\
    w1 = 46;					\
    w2 = %[__imm_0];				\
    r2 -= r1;					\
    r2 |= r1;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r1 &= r2;					\
    r0 = r1;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 47 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 46) -> __success __success_unpriv {
    __success __success_unpriv __retval(46)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_7() -> __naked void {
    __naked void masking_test_in_bounds_7(void)
    {
    asm volatile ("					\
    r3 = -46;					\
    r3 *= -1;					\
    w2 = %[__imm_0];				\
    r2 -= r3;					\
    r2 |= r3;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r3 &= r2;					\
    r0 = r3;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 47 - 1)
    : __clobber_all);
    }
    SEC("socket")
    __description("masking, test in bounds 8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn masking_test_in_bounds_8() -> __naked void {
    __naked void masking_test_in_bounds_8(void)
    {
    asm volatile ("					\
    r3 = -47;					\
    r3 *= -1;					\
    w2 = %[__imm_0];				\
    r2 -= r3;					\
    r2 |= r3;					\
    r2 = -r2;					\
    r2 s>>= 63;					\
    r3 &= r2;					\
    r0 = r3;					\
    exit;						\
    "	:
    : __imm_const(__imm_0, 47 - 1)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
