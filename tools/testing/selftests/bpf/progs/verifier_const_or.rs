//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_const_or.c
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
// Converted from tools/testing/selftests/bpf/verifier/const_or.c

    SEC("tracepoint")
    __description("constant register |= constant should keep constant type")
    __success
#[no_mangle]
pub unsafe extern "C" fn constant_should_keep_constant_type() -> __naked void {
    __naked void constant_should_keep_constant_type(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -48;					\
    r2 = 34;					\
    r2 |= 13;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    exit;						\
    "	:
    : __imm(bpf_probe_read_kernel)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("constant register |= constant should not bypass stack boundary checks")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=58": "invalid write to stack R1 off=-48) -> __failure {
    __failure __msg("invalid write to stack R1 off=-48 size=58")
#[no_mangle]
pub unsafe extern "C" fn not_bypass_stack_boundary_checks_1() -> __naked void {
    __naked void not_bypass_stack_boundary_checks_1(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -48;					\
    r2 = 34;					\
    r2 |= 24;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    exit;						\
    "	:
    : __imm(bpf_probe_read_kernel)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("constant register |= constant register should keep constant type")
    __success
#[no_mangle]
pub unsafe extern "C" fn register_should_keep_constant_type() -> __naked void {
    __naked void register_should_keep_constant_type(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -48;					\
    r2 = 34;					\
    r4 = 13;					\
    r2 |= r4;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    exit;						\
    "	:
    : __imm(bpf_probe_read_kernel)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("constant register |= constant register should not bypass stack boundary checks")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=58": "invalid write to stack R1 off=-48) -> __failure {
    __failure __msg("invalid write to stack R1 off=-48 size=58")
#[no_mangle]
pub unsafe extern "C" fn not_bypass_stack_boundary_checks_2() -> __naked void {
    __naked void not_bypass_stack_boundary_checks_2(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += -48;					\
    r2 = 34;					\
    r4 = 24;					\
    r2 |= r4;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    exit;						\
    "	:
    : __imm(bpf_probe_read_kernel)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
