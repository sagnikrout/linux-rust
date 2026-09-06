//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_meta_access.c
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
// Converted from tools/testing/selftests/bpf/verifier/meta_access.c

    SEC("xdp")
    __description("meta access, test1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn meta_access_test1() -> __naked void {
    __naked void meta_access_test1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test2")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R0 min value is) -> __failure {
    __failure __msg("R0 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test2() -> __naked void {
    __naked void meta_access_test2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r0 = r2;					\
    r0 -= 8;					\
    r4 = r2;					\
    r4 += 8;					\
    if r4 > r3 goto l0_%=;				\
    r0 = *(u8*)(r0 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test3")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "invalid access to) -> __failure {
    __failure __msg("invalid access to packet")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test3() -> __naked void {
    __naked void meta_access_test3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test4")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "invalid access to) -> __failure {
    __failure __msg("invalid access to packet")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test4() -> __naked void {
    __naked void meta_access_test4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r4 = *(u32*)(r1 + %[xdp_md_data]);		\
    r0 = r4;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test5")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R3) -> __failure {
    __failure __msg("R3 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test5() -> __naked void {
    __naked void meta_access_test5(void)
    {
    asm volatile ("					\
    r3 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r4 = *(u32*)(r1 + %[xdp_md_data]);		\
    r0 = r3;					\
    r0 += 8;					\
    if r0 > r4 goto l0_%=;				\
    r2 = -8;					\
    call %[bpf_xdp_adjust_meta];			\
    r0 = *(u8*)(r3 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_xdp_adjust_meta),
    __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test6")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "invalid access to) -> __failure {
    __failure __msg("invalid access to packet")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test6() -> __naked void {
    __naked void meta_access_test6(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r0 = r3;					\
    r0 += 8;					\
    r4 = r2;					\
    r4 += 8;					\
    if r4 > r0 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test7")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn meta_access_test7() -> __naked void {
    __naked void meta_access_test7(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r0 = r3;					\
    r0 += 8;					\
    r4 = r2;					\
    r4 += 8;					\
    if r4 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn meta_access_test8() -> __naked void {
    __naked void meta_access_test8(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r4 = r2;					\
    r4 += 0xFFFF;					\
    if r4 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test9")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "invalid access to) -> __failure {
    __failure __msg("invalid access to packet")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test9() -> __naked void {
    __naked void meta_access_test9(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r4 = r2;					\
    r4 += 0xFFFF;					\
    r4 += 1;					\
    if r4 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test10")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "invalid access to) -> __failure {
    __failure __msg("invalid access to packet")
#[no_mangle]
pub unsafe extern "C" fn meta_access_test10() -> __naked void {
    __naked void meta_access_test10(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r4 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r5 = 42;					\
    r6 = 24;					\
// (u64*)(r10 - 8) = r5;				\
    lock *(u64 *)(r10 - 8) += r6;			\
    r5 = *(u64*)(r10 - 8);				\
    if r5 > 100 goto l0_%=;				\
    r3 += r5;					\
    r5 = r3;					\
    r6 = r2;					\
    r6 += 8;					\
    if r6 > r5 goto l0_%=;				\
    r2 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test11")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn meta_access_test11() -> __naked void {
    __naked void meta_access_test11(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r5 = 42;					\
    r6 = 24;					\
// (u64*)(r10 - 8) = r5;				\
    lock *(u64 *)(r10 - 8) += r6;			\
    r5 = *(u64*)(r10 - 8);				\
    if r5 > 100 goto l0_%=;				\
    r2 += r5;					\
    r5 = r2;					\
    r6 = r2;					\
    r6 += 8;					\
    if r6 > r3 goto l0_%=;				\
    r5 = *(u8*)(r5 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("meta access, test12")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn meta_access_test12() -> __naked void {
    __naked void meta_access_test12(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r4 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r5 = r3;					\
    r5 += 16;					\
    if r5 > r4 goto l0_%=;				\
    r0 = *(u8*)(r3 + 0);				\
    r5 = r2;					\
    r5 += 16;					\
    if r5 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
