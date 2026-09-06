//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/ptrace.c
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
//
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// Macro flag: #define CREATE_TRACE_POINTS

#[no_mangle]
pub unsafe extern "C" fn user_enable_single_step(child: *mut task_struct) {
    void user_enable_single_step(struct task_struct *child)
    {
    set_tsk_thread_flag(child, TIF_SINGLESTEP);

    SUBARCH_SET_SINGLESTEPPING(child, 1);

    }
#[no_mangle]
pub unsafe extern "C" fn user_disable_single_step(child: *mut task_struct) {
    void user_disable_single_step(struct task_struct *child)
    {
    clear_tsk_thread_flag(child, TIF_SINGLESTEP);

    SUBARCH_SET_SINGLESTEPPING(child, 0);

    }
//
// Called by kernel/ptrace.c when detaching..
//
#[no_mangle]
pub unsafe extern "C" fn ptrace_disable(child: *mut task_struct) {
    void ptrace_disable(struct task_struct *child)
    {
    user_disable_single_step(child);
    }
    long arch_ptrace(struct task_struct *child, long request,
    unsigned long addr, unsigned long data)
    {
    int i, ret;
    unsigned long __user *p = (void __user *)data;
    void __user *vp = p;
    switch (request) {
// read the word at location addr in the USER area.
    case PTRACE_PEEKUSR:
    ret = peek_user(child, addr, data);
    break;
// write the word at location addr in the USER area
    case PTRACE_POKEUSR:
    ret = poke_user(child, addr, data);
    break;
    case PTRACE_SYSEMU:
    case PTRACE_SYSEMU_SINGLESTEP:
    ret = -EIO;
    break;

    case PTRACE_GETREGS: { /* Get all gp regs from the child. */
    if (!access_ok(p, MAX_REG_OFFSET)) {
    ret = -EIO;
    break;
    }
    for ( i = 0; i < MAX_REG_OFFSET; i += sizeof(long) ) {
    __put_user(getreg(child, i), p);
    p++;
    }
    ret = 0;
    break;
    }

    case PTRACE_SETREGS: { /* Set all gp regs in the child. */
    let mut tmp: c_ulong = 0;
    if (!access_ok(p, MAX_REG_OFFSET)) {
    ret = -EIO;
    break;
    }
    for ( i = 0; i < MAX_REG_OFFSET; i += sizeof(long) ) {
    __get_user(tmp, p);
    putreg(child, i, tmp);
    p++;
    }
    ret = 0;
    break;
    }

    case PTRACE_GET_THREAD_AREA:
    ret = ptrace_get_thread_area(child, addr, vp);
    break;
    case PTRACE_SET_THREAD_AREA:
    ret = ptrace_set_thread_area(child, addr, vp);
    break;
    default:
    ret = ptrace_request(child, request, addr, data);
    if (ret == -EIO)
    ret = subarch_ptrace(child, request, addr, data);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn send_sigtrap(regs: *mut uml_pt_regs, error_code: c_int) {
    static void send_sigtrap(struct uml_pt_regs *regs, int error_code)
    {
// Send us the fake SIGTRAP
    force_sig_fault(SIGTRAP, TRAP_BRKPT,
// User-mode eip?
    UPT_IS_USER(regs) ? (void __user *) UPT_IP(regs) : core::ptr::null_mut());
    }
//
// XXX Check TIF_SINGLESTEP for singlestepping check and
// PT_PTRACED vs TIF_SYSCALL_TRACE for syscall tracing check
//
#[no_mangle]
pub unsafe extern "C" fn syscall_trace_enter(regs: *mut pt_regs) -> c_int {
    int syscall_trace_enter(struct pt_regs *regs)
    {
    audit_syscall_entry(UPT_SYSCALL_NR(&regs.regs),
    UPT_SYSCALL_ARG1(&regs.regs),
    UPT_SYSCALL_ARG2(&regs.regs),
    UPT_SYSCALL_ARG3(&regs.regs),
    UPT_SYSCALL_ARG4(&regs.regs));
    if (test_thread_flag(TIF_SYSCALL_TRACEPOINT))
    trace_sys_enter(regs, UPT_SYSCALL_NR(&regs.regs));
    if (!test_thread_flag(TIF_SYSCALL_TRACE))
    return 0;
    return !ptrace_report_syscall_permit_entry(regs);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_trace_leave(regs: *mut pt_regs) {
    void syscall_trace_leave(struct pt_regs *regs)
    {
    let mut ptraced: c_int = current.ptrace;
    audit_syscall_exit(regs);
// Fake a debug trap
    if (test_thread_flag(TIF_SINGLESTEP))
    send_sigtrap(&regs.regs, 0);
    if (test_thread_flag(TIF_SYSCALL_TRACEPOINT))
    trace_sys_exit(regs, PT_REGS_SYSCALL_RET(regs));
    if (!test_thread_flag(TIF_SYSCALL_TRACE))
    return;
    ptrace_report_syscall_exit(regs, 0);
// force do_signal() --> is_syscall()
    if (ptraced & PT_PTRACED)
    set_thread_flag(TIF_SIGPENDING);
    }
