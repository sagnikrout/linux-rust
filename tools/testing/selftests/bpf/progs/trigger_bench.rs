//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/trigger_bench.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
pub const CPU_MASK: c_int = 255;

// matches struct counter in bench.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter {
    pub value: c_long,
    pub __attribute__((aligned(128))): },
    pub hits: [counter; MAX_CPUS],
#[no_mangle]
unsafe extern "C" fn inc_counter() -> __always_inline void {
    static __always_inline void inc_counter(void)
    {
    pub bpf_get_smp_processor_id(): int cpu =,
    pub 1): __sync_add_and_fetch(&hits[cpu & CPU_MASK].value,,
    }
    pub stacktrace: volatile int,
    pub stack_trace_t: [typedef __u64; 128],
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub __u32): __type(key,,
    pub stack_trace_t): __type(value,,
    pub SEC(".maps"): } stack_heap,
#[no_mangle]
unsafe extern "C" fn do_stacktrace(ctx: *mut c_void) -> __always_inline void {
    static __always_inline void do_stacktrace(void *ctx)
    {
    if (!stacktrace)
    pub &(__u32){0}): *mut *mut __u64 ptr = bpf_map_lookup_elem(&stack_heap,,
    if (ptr)
    pub 0): bpf_get_stack(ctx, ptr, sizeof(stack_trace_t),,
    }
#[no_mangle]
unsafe extern "C" fn handle(ctx: *mut c_void) -> __always_inline void {
    static __always_inline void handle(void *ctx)
    {
    }
    SEC("?uprobe")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_uprobe(ctx: *mut c_void) -> c_int {
    int bench_trigger_uprobe(void *ctx)
    {
    pub 0: return,
    }
    SEC("?uprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_uprobe_multi(ctx: *mut c_void) -> c_int {
    int bench_trigger_uprobe_multi(void *ctx)
    {
    pub 0: return,
    }
    pub 0: volatile int batch_iters =,
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn trigger_kernel_count(ctx: *mut c_void) -> c_int {
    int trigger_kernel_count(void *ctx)
    {
    pub i: c_int,
    pub {: for (i = 0; i < batch_iters; i++),
    }
    pub 0: return,
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn trigger_driver(ctx: *mut c_void) -> c_int {
    int trigger_driver(void *ctx)
    {
    pub i: c_int,
    pub i++): for (i = 0; i < batch_iters;,
    pub /: *mut *mut (void)bpf_get_numa_node_id(); / attach point for benchmarking,
    pub 0: return,
    }
    pub __weak: extern int bpf_modify_return_test_tp(int nonce) __ksym,
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn trigger_driver_kfunc(ctx: *mut c_void) -> c_int {
    int trigger_driver_kfunc(void *ctx)
    {
    pub i: c_int,
    pub i++): for (i = 0; i < batch_iters;,
    pub /: *mut *mut (void)bpf_modify_return_test_tp(0); / attach point for benchmarking,
    pub 0: return,
    }
    SEC("?kprobe/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_kprobe(ctx: *mut c_void) -> c_int {
    int bench_trigger_kprobe(void *ctx)
    {
    pub 0: return,
    }
    SEC("?kretprobe/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_kretprobe(ctx: *mut c_void) -> c_int {
    int bench_trigger_kretprobe(void *ctx)
    {
    pub 0: return,
    }
    SEC("?kprobe.multi/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_kprobe_multi(ctx: *mut c_void) -> c_int {
    int bench_trigger_kprobe_multi(void *ctx)
    {
    pub 0: return,
    }
    SEC("?kprobe.multi/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_kprobe_multi_empty(ctx: *mut c_void) -> c_int {
    int bench_kprobe_multi_empty(void *ctx)
    {
    pub 0: return,
    }
    SEC("?kretprobe.multi/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_kretprobe_multi(ctx: *mut c_void) -> c_int {
    int bench_trigger_kretprobe_multi(void *ctx)
    {
    pub 0: return,
    }
    SEC("?kretprobe.multi/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_kretprobe_multi_empty(ctx: *mut c_void) -> c_int {
    int bench_kretprobe_multi_empty(void *ctx)
    {
    pub 0: return,
    }
    SEC("?fentry/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_fentry(ctx: *mut c_void) -> c_int {
    int bench_trigger_fentry(void *ctx)
    {
    pub 0: return,
    }
    SEC("?fexit/bpf_get_numa_node_id")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_fexit(ctx: *mut c_void) -> c_int {
    int bench_trigger_fexit(void *ctx)
    {
    pub 0: return,
    }
    SEC("?fmod_ret/bpf_modify_return_test_tp")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_fmodret(ctx: *mut c_void) -> c_int {
    int bench_trigger_fmodret(void *ctx)
    {
    pub -22: return,
    }
    SEC("?tp/bpf_test_run/bpf_trigger_tp")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_tp(ctx: *mut c_void) -> c_int {
    int bench_trigger_tp(void *ctx)
    {
    pub 0: return,
    }
    SEC("?raw_tp/bpf_trigger_tp")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_rawtp(ctx: *mut c_void) -> c_int {
    int bench_trigger_rawtp(void *ctx)
    {
    pub 0: return,
    }
    SEC("?usdt")
#[no_mangle]
pub unsafe extern "C" fn bench_trigger_usdt(ctx: *mut c_void) -> c_int {
    int bench_trigger_usdt(void *ctx)
    {
    pub 0: return,
    }
