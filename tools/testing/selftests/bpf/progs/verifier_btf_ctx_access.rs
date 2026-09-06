//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_btf_ctx_access.c
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
// Converted from tools/testing/selftests/bpf/verifier/btf_ctx_access.c

    SEC("fentry/bpf_modify_return_test")
    __description("btf_ctx_access accept")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn btf_ctx_access_accept() -> __naked void {
    __naked void btf_ctx_access_accept(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 8);		/* load 2nd argument value (int pointer) */\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test9")
    __description("btf_ctx_access u32 pointer accept")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ctx_access_u32_pointer_accept() -> __naked void {
    __naked void ctx_access_u32_pointer_accept(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 0);		/* load 1nd argument value (u32 pointer) */\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test9")
    __description("btf_ctx_access u32 pointer reject u32")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "size 4 must be) -> __failure {
    __failure __msg("size 4 must be 8")
#[no_mangle]
pub unsafe extern "C" fn ctx_access_u32_pointer_reject_32() -> __naked void {
    __naked void ctx_access_u32_pointer_reject_32(void)
    {
    asm volatile ("					\
    r2 = *(u32 *)(r1 + 0);		/* load 1st argument with narrow load */\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test9")
    __description("btf_ctx_access u32 pointer reject u16")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "size 2 must be) -> __failure {
    __failure __msg("size 2 must be 8")
#[no_mangle]
pub unsafe extern "C" fn ctx_access_u32_pointer_reject_16() -> __naked void {
    __naked void ctx_access_u32_pointer_reject_16(void)
    {
    asm volatile ("					\
    r2 = *(u16 *)(r1 + 0);		/* load 1st argument with narrow load */\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test9")
    __description("btf_ctx_access u32 pointer reject u8")
#[no_mangle]
pub unsafe extern "C" fn __msg(8": "size 1 must be) -> __failure {
    __failure __msg("size 1 must be 8")
#[no_mangle]
pub unsafe extern "C" fn ctx_access_u32_pointer_reject_8() -> __naked void {
    __naked void ctx_access_u32_pointer_reject_8(void)
    {
    asm volatile ("					\
    r2 = *(u8 *)(r1 + 0);		/* load 1st argument with narrow load */\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test10")
    __description("btf_ctx_access const void pointer accept")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn ctx_access_const_void_pointer_accept() -> __naked void {
    __naked void ctx_access_const_void_pointer_accept(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 0);		/* load 1st argument value (const void pointer) */\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
