//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_bpf_cookie.c
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
// Copyright (c) 2021 Facebook

    int my_tid;
    __u64 kprobe_res;
    __u64 kprobe_multi_res;
    __u64 kretprobe_res;
    __u64 uprobe_res;
    __u64 uretprobe_res;
    __u64 tp_res;
    __u64 pe_res;
    __u64 raw_tp_res;
    __u64 tp_btf_res;
    __u64 fentry_res;
    __u64 fexit_res;
    __u64 fmod_ret_res;
    __u64 lsm_res;
#[no_mangle]
unsafe extern "C" fn update(ctx: *mut c_void, res: *mut __u64) {
    static void update(void *ctx, __u64 *res)
    {
    if (my_tid != (u32)bpf_get_current_pid_tgid())
    return;
// res |= bpf_get_attach_cookie(ctx);
    }
    SEC("kprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_kprobe(ctx: *mut pt_regs) -> c_int {
    int handle_kprobe(struct pt_regs *ctx)
    {
    update(ctx, &kprobe_res);
    return 0;
    }
    SEC("kretprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_kretprobe(ctx: *mut pt_regs) -> c_int {
    int handle_kretprobe(struct pt_regs *ctx)
    {
    update(ctx, &kretprobe_res);
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe(struct pt_regs *ctx)
    {
    update(ctx, &uprobe_res);
    return 0;
    }
    SEC("uretprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe(ctx: *mut pt_regs) -> c_int {
    int handle_uretprobe(struct pt_regs *ctx)
    {
    update(ctx, &uretprobe_res);
    return 0;
    }
// bpf_prog_array, used by kernel internally to keep track of attached BPF
// programs to a given BPF hook (e.g., for tracepoints) doesn't allow the same
// BPF program to be attached multiple times. So have three identical copies
// ready to attach to the same tracepoint.
//
    SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn handle_tp1(ctx: *mut pt_regs) -> c_int {
    int handle_tp1(struct pt_regs *ctx)
    {
    update(ctx, &tp_res);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn handle_tp2(ctx: *mut pt_regs) -> c_int {
    int handle_tp2(struct pt_regs *ctx)
    {
    update(ctx, &tp_res);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn handle_tp3(ctx: *mut c_void) -> c_int {
    int handle_tp3(void *ctx)
    {
    update(ctx, &tp_res);
    return 1;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn handle_pe(ctx: *mut pt_regs) -> c_int {
    int handle_pe(struct pt_regs *ctx)
    {
    update(ctx, &pe_res);
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_raw_tp(ctx: *mut c_void) -> c_int {
    int handle_raw_tp(void *ctx)
    {
    update(ctx, &raw_tp_res);
    return 0;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_tp_btf(ctx: *mut c_void) -> c_int {
    int handle_tp_btf(void *ctx)
    {
    update(ctx, &tp_btf_res);
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_test1, a: c_int) -> c_int {
    int BPF_PROG(fentry_test1, int a)
    {
    update(ctx, &fentry_res);
    return 0;
    }
    SEC("fexit/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_test1, a: c_int, ret: c_int) -> c_int {
    int BPF_PROG(fexit_test1, int a, int ret)
    {
    update(ctx, &fexit_res);
    return 0;
    }
    SEC("fmod_ret/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fmod_ret_test, _a: c_int, _b: *mut c_int, _ret: c_int) -> c_int {
    int BPF_PROG(fmod_ret_test, int _a, int *_b, int _ret)
    {
    update(ctx, &fmod_ret_res);
    return 1234;
    }
    SEC("lsm/file_mprotect")
    int BPF_PROG(test_int_hook, struct vm_area_struct *vma,
    unsigned long reqprot, unsigned long prot, int ret)
    {
    if (my_tid != (u32)bpf_get_current_pid_tgid())
    return ret;
    update(ctx, &lsm_res);
    return -EPERM;
    }
    char _license[] SEC("license") = "GPL";
