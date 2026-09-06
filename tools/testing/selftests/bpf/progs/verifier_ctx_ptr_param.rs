//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_ctx_ptr_param.c
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
//
// Verifier tests for single- and multi-level pointer parameter handling
// Copyright (c) 2026 CrowdStrike, Inc.
//

    SEC("fentry/bpf_fentry_test_ppvoid")
    __description("fentry/void**: void ** inferred as scalar")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __log_level(2)
    __msg("R1=ctx() R2=scalar()")
#[no_mangle]
pub unsafe extern "C" fn fentry_ppvoid_as_scalar() -> __naked void {
    __naked void fentry_ppvoid_as_scalar(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 0);	\
    r0 = 0;	\
    exit;	\
    " ::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test_pppvoid")
    __description("fentry/void***: void *** inferred as scalar")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __log_level(2)
    __msg("R1=ctx() R2=scalar()")
#[no_mangle]
pub unsafe extern "C" fn fentry_pppvoid_as_scalar() -> __naked void {
    __naked void fentry_pppvoid_as_scalar(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 0);	\
    r0 = 0;	\
    exit;	\
    " ::: __clobber_all);
    }
    SEC("fentry/bpf_fentry_test_ppfile")
    __description("fentry/struct file**: struct file ** inferred as scalar")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __log_level(2)
    __msg("R1=ctx() R2=scalar()")
#[no_mangle]
pub unsafe extern "C" fn fentry_ppfile_as_scalar() -> __naked void {
    __naked void fentry_ppfile_as_scalar(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 0);	\
    r0 = 0;	\
    exit;	\
    " ::: __clobber_all);
    }
    SEC("fexit/bpf_fexit_test_ret_ppfile")
    __description("fexit/return struct file**: returned struct file ** inferred as scalar")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __log_level(2)
    __msg("R1=ctx() R2=scalar()")
#[no_mangle]
pub unsafe extern "C" fn fexit_ppfile_as_scalar() -> __naked void {
    __naked void fexit_ppfile_as_scalar(void)
    {
    asm volatile ("					\
    r2 = *(u64 *)(r1 + 0);	\
    r0 = 0;	\
    exit;	\
    " ::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
