//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_uninit.c
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
// Converted from tools/testing/selftests/bpf/verifier/uninit.c

    SEC("socket")
    __description("read uninitialized register")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R2) -> __failure {
    __failure __msg("R2 !read_ok")
    __msg("R2 has never been initialized on this path")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn read_uninitialized_register() -> __naked void {
    __naked void read_uninitialized_register(void)
    {
    asm volatile ("					\
    r0 = r2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("read invalid register")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R15 is) -> __failure {
    __failure __msg("R15 is invalid")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn read_invalid_register() -> __naked void {
    __naked void read_invalid_register(void)
    {
    asm volatile ("					\
    .8byte %[mov64_reg];				\
    exit;						\
    "	:
    : __imm_insn(mov64_reg, BPF_MOV64_REG(BPF_REG_0, -1))
    : __clobber_all);
    }
    SEC("socket")
    __description("program doesn't init R0 before exit")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R0) -> __failure {
    __failure __msg("R0 !read_ok")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn t_init_r0_before_exit() -> __naked void {
    __naked void t_init_r0_before_exit(void)
    {
    asm volatile ("					\
    r2 = r1;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("program doesn't init R0 before exit in all branches")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R0) -> __failure {
    __failure __msg("R0 !read_ok")
    __msg_unpriv("R1 pointer comparison")
#[no_mangle]
pub unsafe extern "C" fn before_exit_in_all_branches() -> __naked void {
    __naked void before_exit_in_all_branches(void)
    {
    asm volatile ("					\
    if r1 >= 0 goto l0_%=;				\
    r0 = 1;						\
    r0 += 2;					\
    l0_%=:	exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
