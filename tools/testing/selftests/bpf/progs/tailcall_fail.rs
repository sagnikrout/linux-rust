//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tailcall_fail.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    extern void bpf_rcu_read_lock(void) __ksym;
    extern void bpf_rcu_read_unlock(void) __ksym;

    private(A) struct bpf_spin_lock lock;
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 3);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "function calls are not allowed while holding a) -> __failure {
    __failure __msg("function calls are not allowed while holding a lock")
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_spin_lock(ctx: *mut __sk_buff) -> c_int {
    int reject_tail_call_spin_lock(struct __sk_buff *ctx)
    {
    bpf_spin_lock(&lock);
    bpf_tail_call_static(ctx, &jmp_table, 0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "tail_call cannot be used inside bpf_rcu_read_lock-ed) -> __failure {
    __failure __msg("tail_call cannot be used inside bpf_rcu_read_lock-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_rcu_lock(ctx: *mut __sk_buff) -> c_int {
    int reject_tail_call_rcu_lock(struct __sk_buff *ctx)
    {
    bpf_rcu_read_lock();
    bpf_tail_call_static(ctx, &jmp_table, 0);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "tail_call cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("tail_call cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_preempt_lock(ctx: *mut __sk_buff) -> c_int {
    int reject_tail_call_preempt_lock(struct __sk_buff *ctx)
    {
    bpf_guard_preempt();
    bpf_tail_call_static(ctx, &jmp_table, 0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(leak": "tail_call would lead to reference) -> __failure {
    __failure __msg("tail_call would lead to reference leak")
#[no_mangle]
pub unsafe extern "C" fn reject_tail_call_ref(ctx: *mut __sk_buff) -> c_int {
    int reject_tail_call_ref(struct __sk_buff *ctx)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub bpf_obj_new(typeof(*p)): *mut p =,
    pub 0): bpf_tail_call_static(ctx, &jmp_table,,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
