//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/signal.c
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

    EXPORT_SYMBOL(block_signals);
    EXPORT_SYMBOL(unblock_signals);
#[no_mangle]
pub unsafe extern "C" fn block_signals_trace() {
    void block_signals_trace(void)
    {
    block_signals();
    if (current_thread_info())
    trace_hardirqs_off();
    }
#[no_mangle]
pub unsafe extern "C" fn unblock_signals_trace() {
    void unblock_signals_trace(void)
    {
    if (current_thread_info())
    trace_hardirqs_on();
    unblock_signals();
    }
#[no_mangle]
pub unsafe extern "C" fn um_trace_signals_on() {
    void um_trace_signals_on(void)
    {
    if (current_thread_info())
    trace_hardirqs_on();
    }
#[no_mangle]
pub unsafe extern "C" fn um_trace_signals_off() {
    void um_trace_signals_off(void)
    {
    if (current_thread_info())
    trace_hardirqs_off();
    }
//
// OK, we're invoking a handler
//
#[no_mangle]
unsafe extern "C" fn handle_signal(ksig: *mut ksignal, regs: *mut pt_regs) {
    static void handle_signal(struct ksignal *ksig, struct pt_regs *regs)
    {
    sigset_t *oldset = sigmask_to_save();
    let mut singlestep: c_int = 0;
    unsigned long sp;
    int err;
    if (test_thread_flag(TIF_SINGLESTEP) && (current.ptrace & PT_PTRACED))
    singlestep = 1;
// Did we come from a system call?
    if (PT_REGS_SYSCALL_NR(regs) >= 0) {
// If so, check system call restarting..
    switch (PT_REGS_SYSCALL_RET(regs)) {
    case -ERESTART_RESTARTBLOCK:
    case -ERESTARTNOHAND:
    PT_REGS_SYSCALL_RET(regs) = -EINTR;
    break;
    case -ERESTARTSYS:
    if (!(ksig.ka.sa.sa_flags & SA_RESTART)) {
    PT_REGS_SYSCALL_RET(regs) = -EINTR;
    break;
    }
    fallthrough;
    case -ERESTARTNOINTR:
    PT_REGS_RESTART_SYSCALL(regs);
    PT_REGS_ORIG_SYSCALL(regs) = PT_REGS_SYSCALL_NR(regs);
    break;
    }
    }
    sp = PT_REGS_SP(regs);
    if ((ksig.ka.sa.sa_flags & SA_ONSTACK) && (sas_ss_flags(sp) == 0))
    sp = current.sas_ss_sp + current.sas_ss_size;

    if (!(ksig.ka.sa.sa_flags & SA_SIGINFO))
    err = setup_signal_stack_sc(sp, ksig, regs, oldset);
    else

    err = setup_signal_stack_si(sp, ksig, regs, oldset);
    signal_setup_done(err, ksig, singlestep);
    }
#[no_mangle]
pub unsafe extern "C" fn do_signal(regs: *mut pt_regs) {
    void do_signal(struct pt_regs *regs)
    {
    struct ksignal ksig;
    let mut handled_sig: c_int = 0;
    while (get_signal(&ksig)) {
    handled_sig = 1;
// Whee!  Actually deliver the signal.
    handle_signal(&ksig, regs);
    }
// Did we come from a system call?
    if (!handled_sig && (PT_REGS_SYSCALL_NR(regs) >= 0)) {
// Restart the system call - no handlers present
    switch (PT_REGS_SYSCALL_RET(regs)) {
    case -ERESTARTNOHAND:
    case -ERESTARTSYS:
    case -ERESTARTNOINTR:
    PT_REGS_ORIG_SYSCALL(regs) = PT_REGS_SYSCALL_NR(regs);
    PT_REGS_RESTART_SYSCALL(regs);
    break;
    case -ERESTART_RESTARTBLOCK:
    PT_REGS_ORIG_SYSCALL(regs) = __NR_restart_syscall;
    PT_REGS_RESTART_SYSCALL(regs);
    break;
    }
    }
//
// if there's no signal to deliver, we just put the saved sigmask
// back
//
    if (!handled_sig)
    restore_saved_sigmask();
    }
