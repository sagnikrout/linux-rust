//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/syscall.c
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

    long compat_arm_syscall(struct pt_regs *regs, int scno);
    long sys_ni_syscall(void);
#[no_mangle]
unsafe extern "C" fn do_ni_syscall(regs: *mut pt_regs, scno: c_int) -> c_long {
    static long do_ni_syscall(struct pt_regs *regs, int scno)
    {
    if (is_compat_task()) {
    let mut ret: c_long = compat_arm_syscall(regs, scno);
    if (ret != -ENOSYS)
    return ret;
    }
    return sys_ni_syscall();
    }
#[no_mangle]
unsafe extern "C" fn __invoke_syscall(regs: *mut pt_regs, syscall_fn: syscall_fn_t) -> c_long {
    static long __invoke_syscall(struct pt_regs *regs, syscall_fn_t syscall_fn)
    {
    return syscall_fn(regs);
    }
    static void invoke_syscall(struct pt_regs *regs, unsigned int scno,
    unsigned int sc_nr,
    const syscall_fn_t syscall_table[])
    {
    long ret;
    add_random_kstack_offset();
    if (likely(scno < sc_nr)) {
    syscall_fn_t syscall_fn;
    syscall_fn = syscall_table[array_index_nospec(scno, sc_nr)];
    ret = __invoke_syscall(regs, syscall_fn);
    } else {
    ret = do_ni_syscall(regs, scno);
    }
    syscall_set_return_value(current, regs, 0, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn has_syscall_work(flags: c_ulong) -> bool {
    static inline bool has_syscall_work(unsigned long flags)
    {
    return unlikely(flags & _TIF_SYSCALL_WORK);
    }
    static void el0_svc_common(struct pt_regs *regs, int scno, int sc_nr,
    const syscall_fn_t syscall_table[])
    {
    let mut flags: c_ulong = read_thread_flags();
    regs.orig_x0 = regs.regs[0];
    regs.syscallno = scno;
//
// BTI note:
// The architecture does not guarantee that SPSR.BTYPE is zero
// on taking an SVC, so we could return to userspace with a
// non-zero BTYPE after the syscall.
//
// This shouldn't matter except when userspace is explicitly
// doing something stupid, such as setting PROT_BTI on a page
// that lacks conforming BTI/PACIxSP instructions, falling
// through from one executable page to another with differing
// PROT_BTI, or messing with BTYPE via ptrace: in such cases,
// userspace should not be surprised if a SIGILL occurs on
// syscall return.
//
// So, don't touch regs->pstate & PSR_BTYPE_MASK here.
// (Similarly for HVC and SMC elsewhere.)
//
    if (unlikely(flags & _TIF_MTE_ASYNC_FAULT)) {
//
// Process the asynchronous tag check fault before the actual
// syscall. do_notify_resume() will send a signal to userspace
// before the syscall is restarted.
//
    syscall_set_return_value(current, regs, -ERESTARTNOINTR, 0);
    return;
    }
    if (has_syscall_work(flags)) {
//
// The de-facto standard way to skip a system call using ptrace
// is to set the system call to -1 (NO_SYSCALL) and set x0 to a
// suitable error code for consumption by userspace. However,
// this cannot be distinguished from a user-issued syscall(-1)
// and so we must set x0 to -ENOSYS here in case the tracer doesn't
// issue the skip and we fall into trace_exit with x0 preserved.
//
// This is slightly odd because it also means that if a tracer
// sets the system call number to -1 but does not initialise x0,
// then x0 will be preserved for all system calls apart from a
// user-issued syscall(-1). However, requesting a skip and not
// setting the return value is unlikely to do anything sensible
// anyway.
//
    if (scno == NO_SYSCALL)
    syscall_set_return_value(current, regs, -ENOSYS, 0);
    scno = syscall_trace_enter(regs);
    if (scno == NO_SYSCALL)
    goto trace_exit;
    }
    invoke_syscall(regs, scno, sc_nr, syscall_table);
//
// The tracing status may have changed under our feet, so we have to
// check again. However, if we were tracing entry, then we always trace
// exit regardless, as the old entry assembly did.
//
    if (!has_syscall_work(flags) && !IS_ENABLED(CONFIG_DEBUG_RSEQ)) {
    flags = read_thread_flags();
    if (!has_syscall_work(flags) && !(flags & _TIF_SINGLESTEP))
    return;
    }
    trace_exit:
    syscall_trace_exit(regs);
    }
#[no_mangle]
pub unsafe extern "C" fn do_el0_svc(regs: *mut pt_regs) {
    void do_el0_svc(struct pt_regs *regs)
    {
    el0_svc_common(regs, regs.regs[8], __NR_syscalls, sys_call_table);
    }

#[no_mangle]
pub unsafe extern "C" fn do_el0_svc_compat(regs: *mut pt_regs) {
    void do_el0_svc_compat(struct pt_regs *regs)
    {
    el0_svc_common(regs, regs.regs[7], __NR_compat32_syscalls,
    compat_sys_call_table);
    }
