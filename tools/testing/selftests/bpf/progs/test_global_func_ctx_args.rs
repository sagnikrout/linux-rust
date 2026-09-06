//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func_ctx_args.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    static long stack[256];
//
// KPROBE contexts
//
#[no_mangle]
pub unsafe extern "C" fn kprobe_typedef_ctx_subprog(ctx: *mut bpf_user_pt_regs_t) -> __weak int {
    __weak int kprobe_typedef_ctx_subprog(bpf_user_pt_regs_t *ctx)
    {
    return bpf_get_stack(ctx, &stack, sizeof(stack), 0);
    }
    SEC("?kprobe")
    __success
#[no_mangle]
pub unsafe extern "C" fn kprobe_typedef_ctx(ctx: *mut c_void) -> c_int {
    int kprobe_typedef_ctx(void *ctx)
    {
    return kprobe_typedef_ctx_subprog(ctx);
    }
// s390x defines:
//
// typedef user_pt_regs bpf_user_pt_regs_t;
// typedef struct { ... } user_pt_regs;
//
// And so "canonical" underlying struct type is anonymous.
// So on s390x only valid ways to have PTR_TO_CTX argument in global subprogs
// are:
// - bpf_user_pt_regs_t *ctx (typedef);
// - struct bpf_user_pt_regs_t *ctx (backwards compatible struct hack);
// - void *ctx __arg_ctx (arg:ctx tag)
//
// Other architectures also allow using underlying struct types (e.g.,
// `struct pt_regs *ctx` for x86-64)
//

#[no_mangle]
pub unsafe extern "C" fn kprobe_struct_ctx_subprog(ctx: *mut pt_regs_struct_t) -> __weak int {
    __weak int kprobe_struct_ctx_subprog(pt_regs_struct_t *ctx)
    {
    return bpf_get_stack((void *)ctx, &stack, sizeof(stack), 0);
    }
    SEC("?kprobe")
    __success
#[no_mangle]
pub unsafe extern "C" fn kprobe_resolved_ctx(ctx: *mut c_void) -> c_int {
    int kprobe_resolved_ctx(void *ctx)
    {
    return kprobe_struct_ctx_subprog(ctx);
    }

// this is current hack to make this work on old kernels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_user_pt_regs_t {
#[no_mangle]
pub unsafe extern "C" fn kprobe_workaround_ctx_subprog(ctx: *mut bpf_user_pt_regs_t) -> __weak int {
    __weak int kprobe_workaround_ctx_subprog(struct bpf_user_pt_regs_t *ctx)
    {
    pub 0): return bpf_get_stack(ctx, &stack, sizeof(stack),,
    }
    SEC("?kprobe")
    __success
#[no_mangle]
pub unsafe extern "C" fn kprobe_workaround_ctx(ctx: *mut c_void) -> c_int {
    int kprobe_workaround_ctx(void *ctx)
    {
    pub kprobe_workaround_ctx_subprog(ctx): return,
    }
//
// RAW_TRACEPOINT contexts
//
#[no_mangle]
pub unsafe extern "C" fn raw_tp_ctx_subprog(ctx: *mut bpf_raw_tracepoint_args) -> __weak int {
    __weak int raw_tp_ctx_subprog(struct bpf_raw_tracepoint_args *ctx)
    {
    pub 0): return bpf_get_stack(ctx, &stack, sizeof(stack),,
    }
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn raw_tp_ctx(ctx: *mut c_void) -> c_int {
    int raw_tp_ctx(void *ctx)
    {
    pub raw_tp_ctx_subprog(ctx): return,
    }
//
// RAW_TRACEPOINT_WRITABLE contexts
//
#[no_mangle]
pub unsafe extern "C" fn raw_tp_writable_ctx_subprog(ctx: *mut bpf_raw_tracepoint_args) -> __weak int {
    __weak int raw_tp_writable_ctx_subprog(struct bpf_raw_tracepoint_args *ctx)
    {
    pub 0): return bpf_get_stack(ctx, &stack, sizeof(stack),,
    }
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn raw_tp_writable_ctx(ctx: *mut c_void) -> c_int {
    int raw_tp_writable_ctx(void *ctx)
    {
    pub raw_tp_writable_ctx_subprog(ctx): return,
    }
//
// PERF_EVENT contexts
//
#[no_mangle]
pub unsafe extern "C" fn perf_event_ctx_subprog(ctx: *mut bpf_perf_event_data) -> __weak int {
    __weak int perf_event_ctx_subprog(struct bpf_perf_event_data *ctx)
    {
    pub 0): return bpf_get_stack(ctx, &stack, sizeof(stack),,
    }
    SEC("?perf_event")
    __success
#[no_mangle]
pub unsafe extern "C" fn perf_event_ctx(ctx: *mut c_void) -> c_int {
    int perf_event_ctx(void *ctx)
    {
    pub perf_event_ctx_subprog(ctx): return,
    }
// this global subprog can be now called from many types of entry progs, each
// with different context type
//
#[no_mangle]
pub unsafe extern "C" fn subprog_ctx_tag(__arg_ctx: *mut *mut void ctx) -> __weak int {
    __weak int subprog_ctx_tag(void *ctx __arg_ctx)
    {
    pub 0): return bpf_get_stack(ctx, stack, sizeof(stack),,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_struct {
    __weak int subprog_multi_ctx_tags(void *ctx1 __arg_ctx,
    struct my_struct *mem,
    void *ctx2 __arg_ctx)
    {
    if (!mem)
    pub 0: return,
    return bpf_get_stack(ctx1, stack, sizeof(stack), 0) +
    mem.x +
    pub 0): bpf_get_stack(ctx2, stack, sizeof(stack),,
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_raw_tp(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_raw_tp(void *ctx)
    {
    pub }: my_x = { .x = 123,
    pub ctx): return subprog_ctx_tag(ctx) + subprog_multi_ctx_tags(ctx, &x,,
    }
    SEC("?perf_event")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_perf(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_perf(void *ctx)
    {
    pub }: my_x = { .x = 123,
    pub ctx): return subprog_ctx_tag(ctx) + subprog_multi_ctx_tags(ctx, &x,,
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_kprobe(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_kprobe(void *ctx)
    {
    pub }: my_x = { .x = 123,
    pub ctx): return subprog_ctx_tag(ctx) + subprog_multi_ctx_tags(ctx, &x,,
    }
