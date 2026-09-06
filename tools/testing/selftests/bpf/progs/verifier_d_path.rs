//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_d_path.c
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
// Converted from tools/testing/selftests/bpf/verifier/d_path.c

    SEC("fentry/dentry_open")
    __description("d_path accept")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn d_path_accept() -> __naked void {
    __naked void d_path_accept(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 + 0);				\
    r2 = r10;					\
    r2 += -8;					\
    r6 = 0;						\
// (u64*)(r2 + 0) = r6;				\
    r3 = 8 ll;					\
    call %[bpf_d_path];				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_d_path)
    : __clobber_all);
    }
    SEC("fentry/d_path")
    __description("d_path reject")
#[no_mangle]
pub unsafe extern "C" fn __msg(probe": "helper call is not allowed in) -> __failure {
    __failure __msg("helper call is not allowed in probe")
#[no_mangle]
pub unsafe extern "C" fn d_path_reject() -> __naked void {
    __naked void d_path_reject(void)
    {
    asm volatile ("					\
    r1 = *(u64 *)(r1 + 0);				\
    r2 = r10;					\
    r2 += -8;					\
    r6 = 0;						\
// (u64*)(r2 + 0) = r6;				\
    r3 = 8 ll;					\
    call %[bpf_d_path];				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_d_path)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
