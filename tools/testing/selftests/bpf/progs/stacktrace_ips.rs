//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stacktrace_ips.c
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
// Copyright (c) 2018 Facebook

pub const PERF_MAX_STACK_DEPTH: c_int = 127;

    typedef __u64 stack_trace_t[PERF_MAX_STACK_DEPTH];
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(max_entries, 16384);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stackmap SEC(".maps");
    extern bool CONFIG_UNWINDER_ORC __kconfig __weak;
//
// This function is here to have CONFIG_UNWINDER_ORC
// used and added to object BTF.
//
#[no_mangle]
pub unsafe extern "C" fn unused() -> c_int {
    int unused(void)
    {
    return CONFIG_UNWINDER_ORC ? 0 : 1;
    }
    __u32 stack_key;
    SEC("kprobe")
#[no_mangle]
pub unsafe extern "C" fn kprobe_test(ctx: *mut pt_regs) -> c_int {
    int kprobe_test(struct pt_regs *ctx)
    {
    stack_key = bpf_get_stackid(ctx, &stackmap, 0);
    return 0;
    }
    SEC("kprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn kprobe_multi_test(ctx: *mut pt_regs) -> c_int {
    int kprobe_multi_test(struct pt_regs *ctx)
    {
    stack_key = bpf_get_stackid(ctx, &stackmap, 0);
    return 0;
    }
    SEC("raw_tp/bpf_testmod_test_read")
#[no_mangle]
pub unsafe extern "C" fn rawtp_test(ctx: *mut c_void) -> c_int {
    int rawtp_test(void *ctx)
    {
// Skip ebpf program entry in the stack.
    stack_key = bpf_get_stackid(ctx, &stackmap, 0);
    return 0;
    }
    SEC("fentry/bpf_testmod_stacktrace_test")
#[no_mangle]
pub unsafe extern "C" fn fentry_test(ctx: *mut pt_regs) -> c_int {
    int fentry_test(struct pt_regs *ctx)
    {
//
// Skip 2 bpf_program/trampoline stack entries:
// - bpf_prog_bd1f7a949f55fb03_fentry_test
// - bpf_trampoline_182536277701
//
    stack_key = bpf_get_stackid(ctx, &stackmap, 2);
    return 0;
    }
    SEC("fexit/bpf_testmod_stacktrace_test")
#[no_mangle]
pub unsafe extern "C" fn fexit_test(ctx: *mut pt_regs) -> c_int {
    int fexit_test(struct pt_regs *ctx)
    {
// Skip 2 bpf_program/trampoline stack entries, check fentry_test.
    stack_key = bpf_get_stackid(ctx, &stackmap, 2);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
