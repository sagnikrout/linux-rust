//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_sdiv.c
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

    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
    defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) || \
    defined(__TARGET_ARCH_loongarch)) && \
    __clang_major__ >= 18
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_1() -> __naked void {
    __naked void sdiv32_non_zero_imm_1(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w0 s/= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_2() -> __naked void {
    __naked void sdiv32_non_zero_imm_2(void)
    {
    asm volatile ("					\
    w0 = 41;					\
    w0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 20) -> __success __success_unpriv {
    __success __success_unpriv __retval(20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_3() -> __naked void {
    __naked void sdiv32_non_zero_imm_3(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_4() -> __naked void {
    __naked void sdiv32_non_zero_imm_4(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w0 s/= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_5() -> __naked void {
    __naked void sdiv32_non_zero_imm_5(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 21) -> __success __success_unpriv {
    __success __success_unpriv __retval(21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_6() -> __naked void {
    __naked void sdiv32_non_zero_imm_6(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 21) -> __success __success_unpriv {
    __success __success_unpriv __retval(21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_7() -> __naked void {
    __naked void sdiv32_non_zero_imm_7(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w0 s/= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero imm divisor, check 8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 20) -> __success __success_unpriv {
    __success __success_unpriv __retval(20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_imm_8() -> __naked void {
    __naked void sdiv32_non_zero_imm_8(void)
    {
    asm volatile ("					\
    w0 = 41;					\
    w0 s/= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_1() -> __naked void {
    __naked void sdiv32_non_zero_reg_1(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w1 = 2;						\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_2() -> __naked void {
    __naked void sdiv32_non_zero_reg_2(void)
    {
    asm volatile ("					\
    w0 = 41;					\
    w1 = -2;					\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 20) -> __success __success_unpriv {
    __success __success_unpriv __retval(20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_3() -> __naked void {
    __naked void sdiv32_non_zero_reg_3(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w1 = -2;					\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_4() -> __naked void {
    __naked void sdiv32_non_zero_reg_4(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w1 = 2;						\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_5() -> __naked void {
    __naked void sdiv32_non_zero_reg_5(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = -2;					\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 21) -> __success __success_unpriv {
    __success __success_unpriv __retval(21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_6() -> __naked void {
    __naked void sdiv32_non_zero_reg_6(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w1 = -2;					\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 21) -> __success __success_unpriv {
    __success __success_unpriv __retval(21)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_7() -> __naked void {
    __naked void sdiv32_non_zero_reg_7(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 2;						\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, non-zero reg divisor, check 8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 20) -> __success __success_unpriv {
    __success __success_unpriv __retval(20)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_non_zero_reg_8() -> __naked void {
    __naked void sdiv32_non_zero_reg_8(void)
    {
    asm volatile ("					\
    w0 = 41;					\
    w1 = 2;						\
    w0 s/= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero imm divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_imm_1() -> __naked void {
    __naked void sdiv64_non_zero_imm_1(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r0 s/= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero imm divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_imm_2() -> __naked void {
    __naked void sdiv64_non_zero_imm_2(void)
    {
    asm volatile ("					\
    r0 = 41;					\
    r0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero imm divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 20) -> __success __success_unpriv {
    __success __success_unpriv __retval(20)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_imm_3() -> __naked void {
    __naked void sdiv64_non_zero_imm_3(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero imm divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_imm_4() -> __naked void {
    __naked void sdiv64_non_zero_imm_4(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r0 s/= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero imm divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_imm_5() -> __naked void {
    __naked void sdiv64_non_zero_imm_5(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero imm divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 21) -> __success __success_unpriv {
    __success __success_unpriv __retval(21)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_imm_6() -> __naked void {
    __naked void sdiv64_non_zero_imm_6(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r0 s/= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero reg divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_reg_1() -> __naked void {
    __naked void sdiv64_non_zero_reg_1(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r1 = 2;						\
    r0 s/= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero reg divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -20) -> __success __success_unpriv {
    __success __success_unpriv __retval(-20)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_reg_2() -> __naked void {
    __naked void sdiv64_non_zero_reg_2(void)
    {
    asm volatile ("					\
    r0 = 41;					\
    r1 = -2;					\
    r0 s/= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero reg divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 20) -> __success __success_unpriv {
    __success __success_unpriv __retval(20)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_reg_3() -> __naked void {
    __naked void sdiv64_non_zero_reg_3(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r1 = -2;					\
    r0 s/= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero reg divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_reg_4() -> __naked void {
    __naked void sdiv64_non_zero_reg_4(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r1 = 2;						\
    r0 s/= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero reg divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -21) -> __success __success_unpriv {
    __success __success_unpriv __retval(-21)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_reg_5() -> __naked void {
    __naked void sdiv64_non_zero_reg_5(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r1 = -2;					\
    r0 s/= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, non-zero reg divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 21) -> __success __success_unpriv {
    __success __success_unpriv __retval(21)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_non_zero_reg_6() -> __naked void {
    __naked void sdiv64_non_zero_reg_6(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r1 = -2;					\
    r0 s/= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero imm divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_imm_1() -> __naked void {
    __naked void smod32_non_zero_imm_1(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w0 s%%= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero imm divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_imm_2() -> __naked void {
    __naked void smod32_non_zero_imm_2(void)
    {
    asm volatile ("					\
    w0 = 41;					\
    w0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero imm divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_imm_3() -> __naked void {
    __naked void smod32_non_zero_imm_3(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero imm divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_imm_4() -> __naked void {
    __naked void smod32_non_zero_imm_4(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w0 s%%= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero imm divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_imm_5() -> __naked void {
    __naked void smod32_non_zero_imm_5(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero imm divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_imm_6() -> __naked void {
    __naked void smod32_non_zero_imm_6(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero reg divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_reg_1() -> __naked void {
    __naked void smod32_non_zero_reg_1(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w1 = 2;						\
    w0 s%%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero reg divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_reg_2() -> __naked void {
    __naked void smod32_non_zero_reg_2(void)
    {
    asm volatile ("					\
    w0 = 41;					\
    w1 = -2;					\
    w0 s%%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero reg divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_reg_3() -> __naked void {
    __naked void smod32_non_zero_reg_3(void)
    {
    asm volatile ("					\
    w0 = -41;					\
    w1 = -2;					\
    w0 s%%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero reg divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_reg_4() -> __naked void {
    __naked void smod32_non_zero_reg_4(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w1 = 2;						\
    w0 s%%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero reg divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_reg_5() -> __naked void {
    __naked void smod32_non_zero_reg_5(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = -2;					\
    w0 s%%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, non-zero reg divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_non_zero_reg_6() -> __naked void {
    __naked void smod32_non_zero_reg_6(void)
    {
    asm volatile ("					\
    w0 = -42;					\
    w1 = -2;					\
    w0 s%%= w1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_1() -> __naked void {
    __naked void smod64_non_zero_imm_1(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r0 s%%= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_2() -> __naked void {
    __naked void smod64_non_zero_imm_2(void)
    {
    asm volatile ("					\
    r0 = 41;					\
    r0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_3() -> __naked void {
    __naked void smod64_non_zero_imm_3(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_4() -> __naked void {
    __naked void smod64_non_zero_imm_4(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r0 s%%= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -0) -> __success __success_unpriv {
    __success __success_unpriv __retval(-0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_5() -> __naked void {
    __naked void smod64_non_zero_imm_5(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_6() -> __naked void {
    __naked void smod64_non_zero_imm_6(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r0 s%%= -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_7() -> __naked void {
    __naked void smod64_non_zero_imm_7(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r0 s%%= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero imm divisor, check 8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_imm_8() -> __naked void {
    __naked void smod64_non_zero_imm_8(void)
    {
    asm volatile ("					\
    r0 = 41;					\
    r0 s%%= 2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_1() -> __naked void {
    __naked void smod64_non_zero_reg_1(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r1 = 2;						\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_2() -> __naked void {
    __naked void smod64_non_zero_reg_2(void)
    {
    asm volatile ("					\
    r0 = 41;					\
    r1 = -2;					\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 3")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_3() -> __naked void {
    __naked void smod64_non_zero_reg_3(void)
    {
    asm volatile ("					\
    r0 = -41;					\
    r1 = -2;					\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 4")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_4() -> __naked void {
    __naked void smod64_non_zero_reg_4(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r1 = 2;						\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 5")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_5() -> __naked void {
    __naked void smod64_non_zero_reg_5(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r1 = -2;					\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 6")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_6() -> __naked void {
    __naked void smod64_non_zero_reg_6(void)
    {
    asm volatile ("					\
    r0 = -42;					\
    r1 = -2;					\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_7() -> __naked void {
    __naked void smod64_non_zero_reg_7(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r1 = 2;						\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, non-zero reg divisor, check 8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn smod64_non_zero_reg_8() -> __naked void {
    __naked void smod64_non_zero_reg_8(void)
    {
    asm volatile ("					\
    r0 = 41;					\
    r1 = 2;						\
    r0 s%%= r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, zero divisor")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_zero_divisor() -> __naked void {
    __naked void sdiv32_zero_divisor(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 0;						\
    w2 = -1;					\
    w2 s/= w1;					\
    w0 = w2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, zero divisor")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn sdiv64_zero_divisor() -> __naked void {
    __naked void sdiv64_zero_divisor(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r1 = 0;						\
    r2 = -1;					\
    r2 s/= r1;					\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, zero divisor")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod32_zero_divisor() -> __naked void {
    __naked void smod32_zero_divisor(void)
    {
    asm volatile ("					\
    w0 = 42;					\
    w1 = 0;						\
    w2 = -1;					\
    w2 s%%= w1;					\
    w0 = w2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, zero divisor")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn smod64_zero_divisor() -> __naked void {
    __naked void smod64_zero_divisor(void)
    {
    asm volatile ("					\
    r0 = 42;					\
    r1 = 0;						\
    r2 = -1;					\
    r2 s%%= r1;					\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, overflow r/r, LLONG_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
    __arch_x86_64
    __xlated("0: r2 = 0x8000000000000000")
    __xlated("2: r3 = -1")
    __xlated("3: r4 = r2")
    __xlated("4: r12 = r3")
    __xlated("5: r12 += 1")
    __xlated("6: if r12 > 0x1 goto pc+4")
    __xlated("7: if r12 == 0x0 goto pc+1")
    __xlated("8: r2 = 0")
    __xlated("9: r2 = -r2")
    __xlated("10: goto pc+1")
    __xlated("11: r2 s/= r3")
    __xlated("12: r0 = 0")
    __xlated("13: if r2 != r4 goto pc+1")
    __xlated("14: r0 = 1")
    __xlated("15: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv64_overflow_rr() -> __naked void {
    __naked void sdiv64_overflow_rr(void)
    {
    asm volatile ("					\
    r2 = %[llong_min] ll;				\
    r3 = -1;					\
    r4 = r2;					\
    r2 s/= r3;					\
    r0 = 0;						\
    if r2 != r4 goto +1;				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, r/r, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -5) -> __success {
    __success __retval(-5)
    __arch_x86_64
    __xlated("0: r2 = 5")
    __xlated("1: r3 = -1")
    __xlated("2: r12 = r3")
    __xlated("3: r12 += 1")
    __xlated("4: if r12 > 0x1 goto pc+4")
    __xlated("5: if r12 == 0x0 goto pc+1")
    __xlated("6: r2 = 0")
    __xlated("7: r2 = -r2")
    __xlated("8: goto pc+1")
    __xlated("9: r2 s/= r3")
    __xlated("10: r0 = r2")
    __xlated("11: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv64_rr_divisor_neg_1() -> __naked void {
    __naked void sdiv64_rr_divisor_neg_1(void)
    {
    asm volatile ("					\
    r2 = 5;						\
    r3 = -1;					\
    r2 s/= r3;					\
    r0 = r2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, overflow r/i, LLONG_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
    __arch_x86_64
    __xlated("0: r2 = 0x8000000000000000")
    __xlated("2: r4 = r2")
    __xlated("3: r2 = -r2")
    __xlated("4: r0 = 0")
    __xlated("5: if r2 != r4 goto pc+1")
    __xlated("6: r0 = 1")
    __xlated("7: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv64_overflow_ri() -> __naked void {
    __naked void sdiv64_overflow_ri(void)
    {
    asm volatile ("					\
    r2 = %[llong_min] ll;				\
    r4 = r2;					\
    r2 s/= -1;					\
    r0 = 0;						\
    if r2 != r4 goto +1;				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV64, r/i, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -5) -> __success {
    __success __retval(-5)
    __arch_x86_64
    __xlated("0: r2 = 5")
    __xlated("1: r4 = r2")
    __xlated("2: r2 = -r2")
    __xlated("3: r0 = r2")
    __xlated("4: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv64_ri_divisor_neg_1() -> __naked void {
    __naked void sdiv64_ri_divisor_neg_1(void)
    {
    asm volatile ("					\
    r2 = 5;						\
    r4 = r2;					\
    r2 s/= -1;					\
    r0 = r2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, overflow r/r, INT_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
    __arch_x86_64
    __xlated("0: w2 = -2147483648")
    __xlated("1: w3 = -1")
    __xlated("2: w4 = w2")
    __xlated("3: r12 = r3")
    __xlated("4: w12 += 1")
    __xlated("5: if w12 > 0x1 goto pc+4")
    __xlated("6: if w12 == 0x0 goto pc+1")
    __xlated("7: w2 = 0")
    __xlated("8: w2 = -w2")
    __xlated("9: goto pc+1")
    __xlated("10: w2 s/= w3")
    __xlated("11: r0 = 0")
    __xlated("12: if w2 != w4 goto pc+1")
    __xlated("13: r0 = 1")
    __xlated("14: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv32_overflow_rr() -> __naked void {
    __naked void sdiv32_overflow_rr(void)
    {
    asm volatile ("					\
    w2 = %[int_min];				\
    w3 = -1;					\
    w4 = w2;					\
    w2 s/= w3;					\
    r0 = 0;						\
    if w2 != w4 goto +1;				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, r/r, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 5) -> __success {
    __success __retval(5)
    __arch_x86_64
    __xlated("0: w2 = -5")
    __xlated("1: w3 = -1")
    __xlated("2: w4 = w2")
    __xlated("3: r12 = r3")
    __xlated("4: w12 += 1")
    __xlated("5: if w12 > 0x1 goto pc+4")
    __xlated("6: if w12 == 0x0 goto pc+1")
    __xlated("7: w2 = 0")
    __xlated("8: w2 = -w2")
    __xlated("9: goto pc+1")
    __xlated("10: w2 s/= w3")
    __xlated("11: w0 = w2")
    __xlated("12: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv32_rr_divisor_neg_1() -> __naked void {
    __naked void sdiv32_rr_divisor_neg_1(void)
    {
    asm volatile ("					\
    w2 = -5;					\
    w3 = -1;					\
    w4 = w2;					\
    w2 s/= w3;					\
    w0 = w2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, overflow r/i, INT_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
    __arch_x86_64
    __xlated("0: w2 = -2147483648")
    __xlated("1: w4 = w2")
    __xlated("2: w2 = -w2")
    __xlated("3: r0 = 0")
    __xlated("4: if w2 != w4 goto pc+1")
    __xlated("5: r0 = 1")
    __xlated("6: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv32_overflow_ri() -> __naked void {
    __naked void sdiv32_overflow_ri(void)
    {
    asm volatile ("					\
    w2 = %[int_min];				\
    w4 = w2;					\
    w2 s/= -1;					\
    r0 = 0;						\
    if w2 != w4 goto +1;				\
    r0 = 1;						\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, r/i, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -5) -> __success {
    __success __retval(-5)
    __arch_x86_64
    __xlated("0: w2 = 5")
    __xlated("1: w4 = w2")
    __xlated("2: w2 = -w2")
    __xlated("3: w0 = w2")
    __xlated("4: exit")
#[no_mangle]
pub unsafe extern "C" fn sdiv32_ri_divisor_neg_1() -> __naked void {
    __naked void sdiv32_ri_divisor_neg_1(void)
    {
    asm volatile ("					\
    w2 = 5;						\
    w4 = w2;					\
    w2 s/= -1;					\
    w0 = w2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, overflow r/r, LLONG_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: r2 = 0x8000000000000000")
    __xlated("2: r3 = -1")
    __xlated("3: r4 = r2")
    __xlated("4: r12 = r3")
    __xlated("5: r12 += 1")
    __xlated("6: if r12 > 0x1 goto pc+3")
    __xlated("7: if r12 == 0x1 goto pc+3")
    __xlated("8: w2 = 0")
    __xlated("9: goto pc+1")
    __xlated("10: r2 s%= r3")
    __xlated("11: r0 = r2")
    __xlated("12: exit")
#[no_mangle]
pub unsafe extern "C" fn smod64_overflow_rr() -> __naked void {
    __naked void smod64_overflow_rr(void)
    {
    asm volatile ("					\
    r2 = %[llong_min] ll;				\
    r3 = -1;					\
    r4 = r2;					\
    r2 s%%= r3;					\
    r0 = r2;					\
    exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, r/r, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: r2 = 5")
    __xlated("1: r3 = -1")
    __xlated("2: r4 = r2")
    __xlated("3: r12 = r3")
    __xlated("4: r12 += 1")
    __xlated("5: if r12 > 0x1 goto pc+3")
    __xlated("6: if r12 == 0x1 goto pc+3")
    __xlated("7: w2 = 0")
    __xlated("8: goto pc+1")
    __xlated("9: r2 s%= r3")
    __xlated("10: r0 = r2")
    __xlated("11: exit")
#[no_mangle]
pub unsafe extern "C" fn smod64_rr_divisor_neg_1() -> __naked void {
    __naked void smod64_rr_divisor_neg_1(void)
    {
    asm volatile ("					\
    r2 = 5;						\
    r3 = -1;					\
    r4 = r2;					\
    r2 s%%= r3;					\
    r0 = r2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, overflow r/i, LLONG_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: r2 = 0x8000000000000000")
    __xlated("2: r4 = r2")
    __xlated("3: w2 = 0")
    __xlated("4: r0 = r2")
    __xlated("5: exit")
#[no_mangle]
pub unsafe extern "C" fn smod64_overflow_ri() -> __naked void {
    __naked void smod64_overflow_ri(void)
    {
    asm volatile ("					\
    r2 = %[llong_min] ll;				\
    r4 = r2;					\
    r2 s%%= -1;					\
    r0 = r2;					\
    exit;						\
    "	:
    : __imm_const(llong_min, LLONG_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD64, r/i, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: r2 = 5")
    __xlated("1: r4 = r2")
    __xlated("2: w2 = 0")
    __xlated("3: r0 = r2")
    __xlated("4: exit")
#[no_mangle]
pub unsafe extern "C" fn smod64_ri_divisor_neg_1() -> __naked void {
    __naked void smod64_ri_divisor_neg_1(void)
    {
    asm volatile ("					\
    r2 = 5;						\
    r4 = r2;					\
    r2 s%%= -1;					\
    r0 = r2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, overflow r/r, INT_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: w2 = -2147483648")
    __xlated("1: w3 = -1")
    __xlated("2: w4 = w2")
    __xlated("3: r12 = r3")
    __xlated("4: w12 += 1")
    __xlated("5: if w12 > 0x1 goto pc+3")
    __xlated("6: if w12 == 0x1 goto pc+4")
    __xlated("7: w2 = 0")
    __xlated("8: goto pc+1")
    __xlated("9: w2 s%= w3")
    __xlated("10: goto pc+1")
    __xlated("11: w2 = w2")
    __xlated("12: r0 = r2")
    __xlated("13: exit")
#[no_mangle]
pub unsafe extern "C" fn smod32_overflow_rr() -> __naked void {
    __naked void smod32_overflow_rr(void)
    {
    asm volatile ("					\
    w2 = %[int_min];				\
    w3 = -1;					\
    w4 = w2;					\
    w2 s%%= w3;					\
    r0 = r2;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, r/r, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: w2 = -5")
    __xlated("1: w3 = -1")
    __xlated("2: w4 = w2")
    __xlated("3: r12 = r3")
    __xlated("4: w12 += 1")
    __xlated("5: if w12 > 0x1 goto pc+3")
    __xlated("6: if w12 == 0x1 goto pc+4")
    __xlated("7: w2 = 0")
    __xlated("8: goto pc+1")
    __xlated("9: w2 s%= w3")
    __xlated("10: goto pc+1")
    __xlated("11: w2 = w2")
    __xlated("12: r0 = r2")
    __xlated("13: exit")
#[no_mangle]
pub unsafe extern "C" fn smod32_rr_divisor_neg_1() -> __naked void {
    __naked void smod32_rr_divisor_neg_1(void)
    {
    asm volatile ("					\
    w2 = -5;				\
    w3 = -1;					\
    w4 = w2;					\
    w2 s%%= w3;					\
    r0 = r2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, overflow r/i, INT_MIN/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: w2 = -2147483648")
    __xlated("1: w4 = w2")
    __xlated("2: w2 = 0")
    __xlated("3: r0 = r2")
    __xlated("4: exit")
#[no_mangle]
pub unsafe extern "C" fn smod32_overflow_ri() -> __naked void {
    __naked void smod32_overflow_ri(void)
    {
    asm volatile ("					\
    w2 = %[int_min];				\
    w4 = w2;					\
    w2 s%%= -1;					\
    r0 = r2;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, r/i, small_val/-1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __xlated("0: w2 = 5")
    __xlated("1: w4 = w2")
    __xlated("2: w2 = 0")
    __xlated("3: w0 = w2")
    __xlated("4: exit")
#[no_mangle]
pub unsafe extern "C" fn smod32_ri_divisor_neg_1() -> __naked void {
    __naked void smod32_ri_divisor_neg_1(void)
    {
    asm volatile ("					\
    w2 = 5;						\
    w4 = w2;					\
    w2 s%%= -1;					\
    w0 = w2;					\
    exit;						\
    "	:
    :
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, INT_MIN divided by 2, imm")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1073741824) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1073741824)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_int_min_div_2_imm() -> __naked void {
    __naked void sdiv32_int_min_div_2_imm(void)
    {
    asm volatile ("					\
    w0 = %[int_min];				\
    w0 s/= 2;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SDIV32, INT_MIN divided by 2, reg")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1073741824) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1073741824)
#[no_mangle]
pub unsafe extern "C" fn sdiv32_int_min_div_2_reg() -> __naked void {
    __naked void sdiv32_int_min_div_2_reg(void)
    {
    asm volatile ("					\
    w0 = %[int_min];				\
    w1 = 2;						\
    w0 s/= w1;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, INT_MIN modulo 2, imm")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_int_min_mod_2_imm() -> __naked void {
    __naked void smod32_int_min_mod_2_imm(void)
    {
    asm volatile ("					\
    w0 = %[int_min];				\
    w0 s%%= 2;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }
    SEC("socket")
    __description("SMOD32, INT_MIN modulo -2, imm")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn smod32_int_min_mod_neg2_imm() -> __naked void {
    __naked void smod32_int_min_mod_neg2_imm(void)
    {
    asm volatile ("					\
    w0 = %[int_min];				\
    w0 s%%= -2;					\
    exit;						\
    "	:
    : __imm_const(int_min, INT_MIN)
    : __clobber_all);
    }

    SEC("socket")
    __description("cpuv4 is not supported by compiler or jit, use a dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
