//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_syscall_macro.c
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
// Copyright 2022 Sony Group Corporation

    let mut arg1: c_int = 0;
    let mut arg2: c_ulong = 0;
    let mut arg3: c_ulong = 0;
    let mut arg4_cx: c_ulong = 0;
    let mut arg4: c_ulong = 0;
    let mut arg5: c_ulong = 0;
    let mut arg1_core: c_int = 0;
    let mut arg2_core: c_ulong = 0;
    let mut arg3_core: c_ulong = 0;
    let mut arg4_core_cx: c_ulong = 0;
    let mut arg4_core: c_ulong = 0;
    let mut arg5_core: c_ulong = 0;
    let mut option_syscall: c_int = 0;
    let mut arg2_syscall: c_ulong = 0;
    let mut arg3_syscall: c_ulong = 0;
    let mut arg4_syscall: c_ulong = 0;
    let mut arg5_syscall: c_ulong = 0;
    let mut filter_pid: volatile pid_t = 0;
    SEC("kprobe/" SYS_PREFIX "sys_prctl")
#[no_mangle]
pub unsafe extern "C" fn BPF_KPROBE(_arg: handle_sys_prctl) -> c_int {
    int BPF_KPROBE(handle_sys_prctl)
    {
    struct pt_regs *real_regs;
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    let mut tmp: c_ulong = 0;
    if (pid != filter_pid)
    return 0;
    real_regs = PT_REGS_SYSCALL_REGS(ctx);
// test for PT_REGS_PARM
    bpf_probe_read_kernel(&tmp, sizeof(tmp), &PT_REGS_PARM1_SYSCALL(real_regs));
    arg1 = tmp;
    bpf_probe_read_kernel(&arg2, sizeof(arg2), &PT_REGS_PARM2_SYSCALL(real_regs));
    bpf_probe_read_kernel(&arg3, sizeof(arg3), &PT_REGS_PARM3_SYSCALL(real_regs));
    bpf_probe_read_kernel(&arg4_cx, sizeof(arg4_cx), &PT_REGS_PARM4(real_regs));
    bpf_probe_read_kernel(&arg4, sizeof(arg4), &PT_REGS_PARM4_SYSCALL(real_regs));
    bpf_probe_read_kernel(&arg5, sizeof(arg5), &PT_REGS_PARM5_SYSCALL(real_regs));
// test for the CORE variant of PT_REGS_PARM
    arg1_core = PT_REGS_PARM1_CORE_SYSCALL(real_regs);
    arg2_core = PT_REGS_PARM2_CORE_SYSCALL(real_regs);
    arg3_core = PT_REGS_PARM3_CORE_SYSCALL(real_regs);
    arg4_core_cx = PT_REGS_PARM4_CORE(real_regs);
    arg4_core = PT_REGS_PARM4_CORE_SYSCALL(real_regs);
    arg5_core = PT_REGS_PARM5_CORE_SYSCALL(real_regs);
    return 0;
    }
    SEC("ksyscall/prctl")
    int BPF_KSYSCALL(prctl_enter, int option, unsigned long arg2,
    unsigned long arg3, unsigned long arg4, unsigned long arg5)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != filter_pid)
    return 0;
    option_syscall = option;
    arg2_syscall = arg2;
    arg3_syscall = arg3;
    arg4_syscall = arg4;
    arg5_syscall = arg5;
    return 0;
    }
    __u64 splice_fd_in;
    __u64 splice_off_in;
    __u64 splice_fd_out;
    __u64 splice_off_out;
    __u64 splice_len;
    __u64 splice_flags;
    SEC("ksyscall/splice")
    int BPF_KSYSCALL(splice_enter, int fd_in, loff_t *off_in, int fd_out,
    loff_t *off_out, size_t len, unsigned int flags)
    {
    let mut pid: pid_t = bpf_get_current_pid_tgid() >> 32;
    if (pid != filter_pid)
    return 0;
    splice_fd_in = fd_in;
    splice_off_in = (__u64)off_in;
    splice_fd_out = fd_out;
    splice_off_out = (__u64)off_out;
    splice_len = len;
    splice_flags = flags;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
