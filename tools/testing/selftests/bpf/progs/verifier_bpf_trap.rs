//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_bpf_trap.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    SEC("socket")
    __description("__builtin_trap with simple c code")
#[no_mangle]
pub unsafe extern "C" fn __msg(variable?": "unexpected __bpf_trap() due to uninitialized) -> __failure {
    __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
#[no_mangle]
pub unsafe extern "C" fn bpf_builtin_trap_with_simple_c() {
    void bpf_builtin_trap_with_simple_c(void)
    {
    __builtin_trap();
    }

    SEC("socket")
    __description("__bpf_trap with simple c code")
#[no_mangle]
pub unsafe extern "C" fn __msg(variable?": "unexpected __bpf_trap() due to uninitialized) -> __failure {
    __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
#[no_mangle]
pub unsafe extern "C" fn bpf_trap_with_simple_c() {
    void bpf_trap_with_simple_c(void)
    {
    __bpf_trap();
    }
    SEC("socket")
    __description("__bpf_trap as the second-from-last insn")
#[no_mangle]
pub unsafe extern "C" fn __msg(variable?": "unexpected __bpf_trap() due to uninitialized) -> __failure {
    __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
#[no_mangle]
pub unsafe extern "C" fn bpf_trap_at_func_end() -> __naked void {
    __naked void bpf_trap_at_func_end(void)
    {
    asm volatile (
    "r0 = 0;"
    "call %[__bpf_trap];"
    "exit;"
    :
    : __imm(__bpf_trap)
    : __clobber_all);
    }
    SEC("socket")
    __description("dead code __bpf_trap in the middle of code")
    __success
#[no_mangle]
pub unsafe extern "C" fn dead_bpf_trap_in_middle() -> __naked void {
    __naked void dead_bpf_trap_in_middle(void)
    {
    asm volatile (
    "r0 = 0;"
    "if r0 == 0 goto +1;"
    "call %[__bpf_trap];"
    "r0 = 2;"
    "exit;"
    :
    : __imm(__bpf_trap)
    : __clobber_all);
    }
    SEC("socket")
    __description("reachable __bpf_trap in the middle of code")
#[no_mangle]
pub unsafe extern "C" fn __msg(variable?": "unexpected __bpf_trap() due to uninitialized) -> __failure {
    __failure __msg("unexpected __bpf_trap() due to uninitialized variable?")
#[no_mangle]
pub unsafe extern "C" fn live_bpf_trap_in_middle() -> __naked void {
    __naked void live_bpf_trap_in_middle(void)
    {
    asm volatile (
    "r0 = 0;"
    "if r0 == 1 goto +1;"
    "call %[__bpf_trap];"
    "r0 = 2;"
    "exit;"
    :
    : __imm(__bpf_trap)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
