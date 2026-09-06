//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_cfg.c
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
// Converted from tools/testing/selftests/bpf/verifier/cfg.c

    SEC("socket")
    __description("unreachable")
#[no_mangle]
pub unsafe extern "C" fn __msg(_arg: "unreachable") -> __failure {
    __failure __msg("unreachable")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn unreachable() -> __naked void {
    __naked void unreachable(void)
    {
    asm volatile ("					\
    exit;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unreachable2")
#[no_mangle]
pub unsafe extern "C" fn __msg(_arg: "unreachable") -> __failure {
    __failure __msg("unreachable")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn unreachable2() -> __naked void {
    __naked void unreachable2(void)
    {
    asm volatile ("					\
    goto l0_%=;					\
    goto l0_%=;					\
    l0_%=:	exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("out of range jump")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "jump out of) -> __failure {
    __failure __msg("jump out of range")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn out_of_range_jump() -> __naked void {
    __naked void out_of_range_jump(void)
    {
    asm volatile ("					\
    goto l0_%=;					\
    exit;						\
    l0_%=:							\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("out of range jump2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "jump out of) -> __failure {
    __failure __msg("jump out of range")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn out_of_range_jump2() -> __naked void {
    __naked void out_of_range_jump2(void)
    {
    asm volatile ("					\
    goto -2;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("loop (back-edge)")
#[no_mangle]
pub unsafe extern "C" fn __msg(1": "unreachable insn) -> __failure {
    __failure __msg("unreachable insn 1")
    __msg_unpriv("back-edge")
#[no_mangle]
pub unsafe extern "C" fn loop_back_edge() -> __naked void {
    __naked void loop_back_edge(void)
    {
    asm volatile ("					\
    l0_%=:	goto l0_%=;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("loop2 (back-edge)")
#[no_mangle]
pub unsafe extern "C" fn __msg(4": "unreachable insn) -> __failure {
    __failure __msg("unreachable insn 4")
    __msg_unpriv("back-edge")
#[no_mangle]
pub unsafe extern "C" fn loop2_back_edge() -> __naked void {
    __naked void loop2_back_edge(void)
    {
    asm volatile ("					\
    l0_%=:	r1 = r0;					\
    r2 = r0;					\
    r3 = r0;					\
    goto l0_%=;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("conditional loop")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "infinite loop) -> __failure {
    __failure __msg("infinite loop detected")
    __msg_unpriv("back-edge")
#[no_mangle]
pub unsafe extern "C" fn conditional_loop() -> __naked void {
    __naked void conditional_loop(void)
    {
    asm volatile ("					\
    r0 = r1;					\
    l0_%=:	r2 = r0;					\
    r3 = r0;					\
    if r1 == 0 goto l0_%=;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("conditional loop (2)")
    __success
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(11": "back-edge from insn 10 to) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("back-edge from insn 10 to 11")
#[no_mangle]
pub unsafe extern "C" fn conditional_loop2() -> __naked void {
    __naked void conditional_loop2(void)
    {
    asm volatile ("					\
    r9 = 2 ll;					\
    r3 = 0x20 ll;					\
    r4 = 0x35 ll;					\
    r8 = r4;					\
    goto l1_%=;					\
    l0_%=:	r9 -= r3;					\
    r9 -= r4;					\
    r9 -= r8;					\
    l1_%=:	r8 += r4;					\
    if r8 < 0x64 goto l0_%=;			\
    r0 = r9;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unconditional loop after conditional jump")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "infinite loop) -> __failure {
    __failure __msg("infinite loop detected")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(2": "back-edge from insn 3 to) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("back-edge from insn 3 to 2")
#[no_mangle]
pub unsafe extern "C" fn uncond_loop_after_cond_jmp() -> __naked void {
    __naked void uncond_loop_after_cond_jmp(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 > 0 goto l1_%=;				\
    l0_%=:	r0 = 1;						\
    goto l0_%=;					\
    l1_%=:	exit;						\
    "	::: __clobber_all);
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn never_ending_subprog() -> c_ulong {
    static unsigned long never_ending_subprog()
    {
    asm volatile ("					\
    r0 = r1;					\
    goto -1;					\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unconditional loop after conditional jump")
// infinite loop is detected *after* check_cfg()
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "infinite loop) -> __failure {
    __failure __msg("infinite loop detected")
#[no_mangle]
pub unsafe extern "C" fn uncond_loop_in_subprog_after_cond_jmp() -> __naked void {
    __naked void uncond_loop_in_subprog_after_cond_jmp(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 > 0 goto l1_%=;				\
    l0_%=:	r0 += 1;					\
    call never_ending_subprog;			\
    l1_%=:	exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
