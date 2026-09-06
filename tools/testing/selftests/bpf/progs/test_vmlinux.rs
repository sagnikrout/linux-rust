//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_vmlinux.c
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

pub const MY_TV_NSEC: c_int = 1337;
    let mut tp_called: bool = false;
    let mut raw_tp_called: bool = false;
    let mut tp_btf_called: bool = false;
    let mut kprobe_called: bool = false;
    let mut fentry_called: bool = false;
    SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn handle__tp(args: *mut syscall_trace_enter) -> c_int {
    int handle__tp(struct syscall_trace_enter *args)
    {
    struct __kernel_timespec *ts;
    long tv_nsec;
    if (args.nr != __NR_nanosleep)
    return 0;
    ts = (void *)args.args[0];
    if (bpf_probe_read_user(&tv_nsec, sizeof(ts.tv_nsec), &ts.tv_nsec) ||
    tv_nsec != MY_TV_NSEC)
    return 0;
    tp_called = true;
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle__raw_tp, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle__raw_tp, struct pt_regs *regs, long id)
    {
    struct __kernel_timespec *ts;
    long tv_nsec;
    if (id != __NR_nanosleep)
    return 0;
    ts = (void *)PT_REGS_PARM1_CORE_SYSCALL(regs);
    if (bpf_probe_read_user(&tv_nsec, sizeof(ts.tv_nsec), &ts.tv_nsec) ||
    tv_nsec != MY_TV_NSEC)
    return 0;
    raw_tp_called = true;
    return 0;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle__tp_btf, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handle__tp_btf, struct pt_regs *regs, long id)
    {
    struct __kernel_timespec *ts;
    long tv_nsec;
    if (id != __NR_nanosleep)
    return 0;
    ts = (void *)PT_REGS_PARM1_CORE_SYSCALL(regs);
    if (bpf_probe_read_user(&tv_nsec, sizeof(ts.tv_nsec), &ts.tv_nsec) ||
    tv_nsec != MY_TV_NSEC)
    return 0;
    tp_btf_called = true;
    return 0;
    }
    SEC("kprobe")
    int BPF_KPROBE(handle__kprobe, struct hrtimer *timer, ktime_t tim, u64 delta_ns,
    const enum hrtimer_mode mode)
    {
    if (tim == MY_TV_NSEC)
    kprobe_called = true;
    return 0;
    }
    SEC("fentry")
    int BPF_PROG(handle__fentry, struct hrtimer *timer, ktime_t tim, u64 delta_ns,
    const enum hrtimer_mode mode)
    {
    if (tim == MY_TV_NSEC)
    fentry_called = true;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
