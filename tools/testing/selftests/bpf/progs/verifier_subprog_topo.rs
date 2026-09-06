//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_subprog_topo.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

// linear chain main -> A -> B
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn linear_b() -> c_ulong {
    static unsigned long linear_b(void)
    {
    asm volatile (
    "r0 = 42;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn linear_a() -> c_ulong {
    static unsigned long linear_a(void)
    {
    asm volatile (
    "call linear_b;"
    "exit;"
    );
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = linear_b")
    __msg("topo_order[1] = linear_a")
    __msg("topo_order[2] = topo_linear")
#[no_mangle]
pub unsafe extern "C" fn topo_linear() -> __naked int {
    __naked int topo_linear(void)
    {
    asm volatile (
    "call linear_a;"
    "exit;"
    );
    }
// diamond main -> A, main -> B, A -> C, B -> C
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn diamond_c() -> c_ulong {
    static unsigned long diamond_c(void)
    {
    asm volatile (
    "r0 = 1;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn diamond_b() -> c_ulong {
    static unsigned long diamond_b(void)
    {
    asm volatile (
    "call diamond_c;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn diamond_a() -> c_ulong {
    static unsigned long diamond_a(void)
    {
    asm volatile (
    "call diamond_c;"
    "exit;"
    );
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = diamond_c")
    __msg("topo_order[3] = topo_diamond")
#[no_mangle]
pub unsafe extern "C" fn topo_diamond() -> __naked int {
    __naked int topo_diamond(void)
    {
    asm volatile (
    "call diamond_a;"
    "call diamond_b;"
    "exit;"
    );
    }
// main -> global_a (global) -> static_leaf (static, leaf)
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn static_leaf() -> c_ulong {
    static unsigned long static_leaf(void)
    {
    asm volatile (
    "r0 = 7;"
    "exit;"
    );
    }
    __noinline __used
#[no_mangle]
pub unsafe extern "C" fn global_a(x: c_int) -> c_int {
    int global_a(int x)
    {
    return static_leaf();
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = static_leaf")
    __msg("topo_order[1] = global_a")
    __msg("topo_order[2] = topo_mixed")
#[no_mangle]
pub unsafe extern "C" fn topo_mixed() -> __naked int {
    __naked int topo_mixed(void)
    {
    asm volatile (
    "r1 = 0;"
    "call global_a;"
    "exit;"
    );
    }
//
// shared static callee from global and main:
// main -> shared_leaf (static)
// main -> global_b (global) -> shared_leaf (static)
//
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn shared_leaf() -> c_ulong {
    static unsigned long shared_leaf(void)
    {
    asm volatile (
    "r0 = 99;"
    "exit;"
    );
    }
    __noinline __used
#[no_mangle]
pub unsafe extern "C" fn global_b(x: c_int) -> c_int {
    int global_b(int x)
    {
    return shared_leaf();
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = shared_leaf")
    __msg("topo_order[1] = global_b")
    __msg("topo_order[2] = topo_shared")
#[no_mangle]
pub unsafe extern "C" fn topo_shared() -> __naked int {
    __naked int topo_shared(void)
    {
    asm volatile (
    "call shared_leaf;"
    "r1 = 0;"
    "call global_b;"
    "exit;"
    );
    }
// duplicate calls to the same subprog
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn dup_leaf() -> c_ulong {
    static unsigned long dup_leaf(void)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    );
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = dup_leaf")
    __msg("topo_order[1] = topo_dup_calls")
#[no_mangle]
pub unsafe extern "C" fn topo_dup_calls() -> __naked int {
    __naked int topo_dup_calls(void)
    {
    asm volatile (
    "call dup_leaf;"
    "call dup_leaf;"
    "exit;"
    );
    }
// main calls bpf_loop() with loop_cb as the callback
#[no_mangle]
unsafe extern "C" fn loop_cb(idx: c_int, ctx: *mut c_void) -> c_int {
    static int loop_cb(int idx, void *ctx)
    {
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = loop_cb")
    __msg("topo_order[1] = topo_loop_cb")
#[no_mangle]
pub unsafe extern "C" fn topo_loop_cb() -> c_int {
    int topo_loop_cb(void)
    {
    bpf_loop(1, loop_cb, core::ptr::null_mut(), 0);
    return 0;
    }
//
// bpf_loop callback calling another subprog
// main -> bpf_loop(callback=loop_cb2) -> loop_cb2 -> loop_cb2_leaf
//
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn loop_cb2_leaf() -> c_ulong {
    static unsigned long loop_cb2_leaf(void)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    );
    }
#[no_mangle]
unsafe extern "C" fn loop_cb2(idx: c_int, ctx: *mut c_void) -> c_int {
    static int loop_cb2(int idx, void *ctx)
    {
    return loop_cb2_leaf();
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = loop_cb2_leaf")
    __msg("topo_order[1] = loop_cb2")
    __msg("topo_order[2] = topo_loop_cb_chain")
#[no_mangle]
pub unsafe extern "C" fn topo_loop_cb_chain() -> c_int {
    int topo_loop_cb_chain(void)
    {
    bpf_loop(1, loop_cb2, core::ptr::null_mut(), 0);
    return 0;
    }
// no calls (single subprog)
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("topo_order[0] = topo_no_calls")
#[no_mangle]
pub unsafe extern "C" fn topo_no_calls() -> __naked int {
    __naked int topo_no_calls(void)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    );
    }
    char _license[] SEC("license") = "GPL";
