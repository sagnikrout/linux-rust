//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_bswap.c
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
    __description("BSWAP, 16")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0x23ff) -> __success __success_unpriv {
    __success __success_unpriv __retval(0x23ff)
#[no_mangle]
pub unsafe extern "C" fn bswap_16() -> __naked void {
    __naked void bswap_16(void)
    {
    asm volatile ("					\
    r0 = 0xff23;					\
    r0 = bswap16 r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("BSWAP, 32")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0x23ff0000) -> __success __success_unpriv {
    __success __success_unpriv __retval(0x23ff0000)
#[no_mangle]
pub unsafe extern "C" fn bswap_32() -> __naked void {
    __naked void bswap_32(void)
    {
    asm volatile ("					\
    r0 = 0xff23;					\
    r0 = bswap32 r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("BSWAP, 64")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0x34ff12ff) -> __success __success_unpriv {
    __success __success_unpriv __retval(0x34ff12ff)
#[no_mangle]
pub unsafe extern "C" fn bswap_64() -> __naked void {
    __naked void bswap_64(void)
    {
    asm volatile ("					\
    r0 = %[u64_val] ll;					\
    r0 = bswap64 r0;				\
    exit;						\
    "	:
    : [u64_val]"i"(0xff12ff34ff56ff78ull)
    : __clobber_all);
    }

    SEC("socket") \
    __success __log_level(2) \
    __msg("r0 &= {{.*}}; R0=scalar({{.*}},var_off=(0x0; " #in_value "))") \
    __msg("r0 = " op " r0 {{.*}}; R0=scalar({{.*}},var_off=(0x0; " #out_value "))") \
    __naked void name(void) \
    { \
    asm volatile (				\
    "call %[bpf_get_prandom_u32];"		\
    "r0 &= " #in_value ";"			\
    "r0 =  " op " r0;"			\
    "r2 =  " #out_value " ll;"		\
    "if r0 > r2 goto trap_%=;"		\
    "r0 = 0;"				\
    "exit;"					\
    "trap_%=:"					\
    "r1 = 42;"				\
    "r0 = *(u64 *)(r1 + 0);"		\
    "exit;"					\
    :						\
    : __imm(bpf_get_prandom_u32)			\
    : __clobber_all);				\
    }
    BSWAP_RANGE_TEST(bswap16_range, "bswap16", 0x3f00, 0x3f)
    BSWAP_RANGE_TEST(bswap32_range, "bswap32", 0x3f00, 0x3f0000)
    BSWAP_RANGE_TEST(bswap64_range, "bswap64", 0x3f00, 0x3f000000000000)

    BSWAP_RANGE_TEST(be16_range, "be16", 0x3f00, 0x3f)
    BSWAP_RANGE_TEST(be32_range, "be32", 0x3f00, 0x3f0000)
    BSWAP_RANGE_TEST(be64_range, "be64", 0x3f00, 0x3f000000000000)
    BSWAP_RANGE_TEST(le16_range, "le16", 0x3f00, 0x3f00)
    BSWAP_RANGE_TEST(le32_range, "le32", 0x3f00, 0x3f00)
    BSWAP_RANGE_TEST(le64_range, "le64", 0x3f00, 0x3f00)

    BSWAP_RANGE_TEST(be16_range, "be16", 0x3f00, 0x3f00)
    BSWAP_RANGE_TEST(be32_range, "be32", 0x3f00, 0x3f00)
    BSWAP_RANGE_TEST(be64_range, "be64", 0x3f00, 0x3f00)
    BSWAP_RANGE_TEST(le16_range, "le16", 0x3f00, 0x3f)
    BSWAP_RANGE_TEST(le32_range, "le32", 0x3f00, 0x3f0000)
    BSWAP_RANGE_TEST(le64_range, "le64", 0x3f00, 0x3f000000000000)

    SEC("socket")
    __description("BSWAP, reset reg id")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "math between fp pointer and register with unbounded min value is not) -> __failure {
    __failure __msg("math between fp pointer and register with unbounded min value is not allowed")
#[no_mangle]
pub unsafe extern "C" fn bswap_reset_reg_id() -> __naked void {
    __naked void bswap_reset_reg_id(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
    r1 = r0;					\
    r0 = be16 r0;					\
    if r0 != 1 goto l0_%=;				\
    r2 = r10;					\
    r2 += -512;					\
    r2 += r1;					\
// (u8 *)(r2 + 0) = 0;				\
    l0_%=:							\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns)
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
