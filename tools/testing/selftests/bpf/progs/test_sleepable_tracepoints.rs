//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sleepable_tracepoints.c
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

    char _license[] SEC("license") = "GPL";
    int target_pid;
    int prog_triggered;
    long err;
    char copied_byte;
#[no_mangle]
unsafe extern "C" fn copy_getcwd_arg(ubuf: *mut c_char) -> c_int {
    static int copy_getcwd_arg(char *ubuf)
    {
    err = bpf_copy_from_user(&copied_byte, sizeof(copied_byte), ubuf);
    if (err)
    return err;
    prog_triggered = 1;
    return 0;
    }
    SEC("tp_btf.s/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_sys_enter_tp_btf, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle_sys_enter_tp_btf, struct pt_regs *regs, long id)
    {
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid ||
    id != __NR_getcwd)
    return 0;
    return copy_getcwd_arg((void *)PT_REGS_PARM1_SYSCALL(regs));
    }
    SEC("raw_tp.s/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_sys_enter_raw_tp, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle_sys_enter_raw_tp, struct pt_regs *regs, long id)
    {
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid ||
    id != __NR_getcwd)
    return 0;
    return copy_getcwd_arg((void *)PT_REGS_PARM1_CORE_SYSCALL(regs));
    }
    SEC("tp.s/syscalls/sys_enter_getcwd")
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter_tp(args: *mut syscall_trace_enter) -> c_int {
    int handle_sys_enter_tp(struct syscall_trace_enter *args)
    {
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid)
    return 0;
    return copy_getcwd_arg((void *)args.args[0]);
    }
    SEC("tp.s/syscalls/sys_exit_getcwd")
#[no_mangle]
pub unsafe extern "C" fn handle_sys_exit_tp(args: *mut syscall_trace_exit) -> c_int {
    int handle_sys_exit_tp(struct syscall_trace_exit *args)
    {
    struct pt_regs *regs;
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid)
    return 0;
    regs = (struct pt_regs *)bpf_task_pt_regs(bpf_get_current_task_btf());
    return copy_getcwd_arg((void *)PT_REGS_PARM1_CORE_SYSCALL(regs));
    }
    SEC("raw_tp.s")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_raw_tp_bare, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle_raw_tp_bare, struct pt_regs *regs, long id)
    {
    return 0;
    }
    SEC("tp.s")
#[no_mangle]
pub unsafe extern "C" fn handle_tp_bare(ctx: *mut c_void) -> c_int {
    int handle_tp_bare(void *ctx)
    {
    return 0;
    }
    SEC("tracepoint.s/syscalls/sys_enter_getcwd")
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter_tp_alias(args: *mut syscall_trace_enter) -> c_int {
    int handle_sys_enter_tp_alias(struct syscall_trace_enter *args)
    {
    return 0;
    }
    SEC("raw_tracepoint.s/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_sys_enter_raw_tp_alias, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle_sys_enter_raw_tp_alias, struct pt_regs *regs, long id)
    {
    return 0;
    }
    SEC("raw_tp.s/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_test_run, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle_test_run, struct pt_regs *regs, long id)
    {
    if ((__u64)regs == 0x1234ULL && (__u64)id == 0x5678ULL)
    return (__u64)regs + (__u64)id;
    return 0;
    }
    SEC("raw_tp.s/sched_switch")
    int BPF_PROG(handle_raw_tp_non_faultable, bool preempt,
    struct task_struct *prev, struct task_struct *next)
    {
    return 0;
    }
    SEC("tp.s/sched/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn handle_tp_non_syscall(ctx: *mut c_void) -> c_int {
    int handle_tp_non_syscall(void *ctx)
    {
    return 0;
    }
