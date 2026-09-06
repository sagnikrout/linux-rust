//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/signal.c
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
// Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2004 PathScale, Inc
// Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

    void (*sig_info[NSIG])(int, struct siginfo *, struct uml_pt_regs *, void *mc) = {
    [SIGTRAP]	= relay_signal,
    [SIGFPE]	= relay_signal,
    [SIGILL]	= relay_signal,
    [SIGWINCH]	= winch,
    [SIGBUS]	= relay_signal,
    [SIGSEGV]	= segv_handler,
    [SIGIO]		= sigio_handler,
    [SIGCHLD]	= sigchld_handler,
    };
#[no_mangle]
unsafe extern "C" fn sig_handler_common(sig: c_int, si: *mut siginfo, mc: *mut mcontext_t) {
    static void sig_handler_common(int sig, struct siginfo *si, mcontext_t *mc)
    {
    struct uml_pt_regs r;
    r.is_user = 0;
    if (sig == SIGSEGV) {
// For segfaults, we want the data from the sigcontext.
    get_regs_from_mc(&r, mc);
    GET_FAULTINFO_FROM_MC(r.faultinfo, mc);
    }
// enable signals if sig isn't IRQ signal
    if ((sig != SIGIO) && (sig != SIGWINCH) && (sig != SIGCHLD))
    unblock_signals_trace();
    (*sig_info[sig])(sig, si, &r, mc);
    }
//
// These are the asynchronous signals.  SIGPROF is excluded because we want to
// be able to profile all of UML, not just the non-critical sections.  If
// profiling is not thread-safe, then that is not my problem.  We can disable
// profiling when SMP is enabled in that case.
//
pub const SIGIO_BIT: c_int = 0;

pub const SIGALRM_BIT: c_int = 1;

pub const SIGCHLD_BIT: c_int = 2;

    __thread int signals_enabled;

    static int signals_blocked, signals_blocked_pending;

    static __thread unsigned int signals_pending;
    static __thread unsigned int signals_active;
#[no_mangle]
unsafe extern "C" fn sig_handler(sig: c_int, si: *mut siginfo, mc: *mut mcontext_t) {
    static void sig_handler(int sig, struct siginfo *si, mcontext_t *mc)
    {
    let mut enabled: c_int = signals_enabled;

    if ((signals_blocked ||
    __atomic_load_n(&signals_blocked_pending, __ATOMIC_SEQ_CST)) &&
    (sig == SIGIO)) {
// increment so unblock will do another round
    __atomic_add_fetch(&signals_blocked_pending, 1,
    __ATOMIC_SEQ_CST);
    return;
    }

    if (!enabled && (sig == SIGIO)) {
//
// In TT_MODE_EXTERNAL, need to still call time-travel
// handlers. This will mark signals_pending by itself
// (only if necessary.)
// Note we won't get here if signals are hard-blocked
// (which is handled above), in that case the hard-
// unblock will handle things.
//
    if (time_travel_mode == TT_MODE_EXTERNAL)
    sigio_run_timetravel_handlers();
    else
    signals_pending |= SIGIO_MASK;
    return;
    }
    if (!enabled && (sig == SIGCHLD)) {
    signals_pending |= SIGCHLD_MASK;
    return;
    }
    block_signals_trace();
    sig_handler_common(sig, si, mc);
    um_set_signals_trace(enabled);
    }
#[no_mangle]
unsafe extern "C" fn timer_real_alarm_handler(mc: *mut mcontext_t) {
    static void timer_real_alarm_handler(mcontext_t *mc)
    {
    struct uml_pt_regs regs;
    if (mc != core::ptr::null_mut())
    get_regs_from_mc(&regs, mc);
    else
    memset(&regs, 0, sizeof(regs));
    timer_handler(SIGALRM, core::ptr::null_mut(), &regs);
    }
#[no_mangle]
unsafe extern "C" fn timer_alarm_handler(sig: c_int, unused_si: *mut siginfo, mc: *mut mcontext_t) {
    static void timer_alarm_handler(int sig, struct siginfo *unused_si, mcontext_t *mc)
    {
    int enabled;
    enabled = signals_enabled;
    if (!signals_enabled) {
    signals_pending |= SIGALRM_MASK;
    return;
    }
    block_signals_trace();
    signals_active |= SIGALRM_MASK;
    timer_real_alarm_handler(mc);
    signals_active &= ~SIGALRM_MASK;
    um_set_signals_trace(enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn deliver_alarm() {
    timer_alarm_handler(SIGALRM, core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn timer_set_signal_handler() {
    void timer_set_signal_handler(void)
    {
    set_handler(SIGALRM);
    }
#[no_mangle]
pub unsafe extern "C" fn timer_alarm_pending() -> c_int {
    int timer_alarm_pending(void)
    {
    return !!(signals_pending & SIGALRM_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn set_sigstack(sig_stack: *mut c_void, size: c_int) {
    void set_sigstack(void *sig_stack, int size)
    {
    stack_t stack = {
    .ss_flags = 0,
    .ss_sp = sig_stack,
    .ss_size = size
    };
    if (sigaltstack(&stack, core::ptr::null_mut()) != 0)
    panic("enabling signal stack failed, errno = %d\n", errno);
    }
#[no_mangle]
unsafe extern "C" fn sigusr1_handler(sig: c_int, unused_si: *mut siginfo, mc: *mut mcontext_t) {
    static void sigusr1_handler(int sig, struct siginfo *unused_si, mcontext_t *mc)
    {
    uml_pm_wake();
    }
#[no_mangle]
pub unsafe extern "C" fn register_pm_wake_signal() {
    void register_pm_wake_signal(void)
    {
    set_handler(SIGUSR1);
    }
    static void (*handlers[_NSIG])(int sig, struct siginfo *si, mcontext_t *mc) = {
    [SIGSEGV] = sig_handler,
    [SIGBUS] = sig_handler,
    [SIGILL] = sig_handler,
    [SIGFPE] = sig_handler,
    [SIGTRAP] = sig_handler,
    [SIGIO] = sig_handler,
    [SIGWINCH] = sig_handler,
// SIGCHLD is only actually registered in seccomp mode.
    [SIGCHLD] = sig_handler,
    [SIGALRM] = timer_alarm_handler,
    [SIGUSR1] = sigusr1_handler,
    };
#[no_mangle]
unsafe extern "C" fn hard_handler(sig: c_int, si: *mut siginfo_t, p: *mut c_void) {
    static void hard_handler(int sig, siginfo_t *si, void *p)
    {
    ucontext_t *uc = p;
    mcontext_t *mc = &uc.uc_mcontext;
    let mut save_errno: c_int = errno;
    (*handlers[sig])(sig, (struct siginfo *)si, mc);
    errno = save_errno;
    }
#[no_mangle]
pub unsafe extern "C" fn set_handler(sig: c_int) {
    void set_handler(int sig)
    {
    struct sigaction action;
    let mut flags: c_int = SA_SIGINFO | SA_ONSTACK;
    sigset_t sig_mask;
    action.sa_sigaction = hard_handler;
// block irq ones
    sigemptyset(&action.sa_mask);
    sigaddset(&action.sa_mask, SIGIO);
    sigaddset(&action.sa_mask, SIGWINCH);
    sigaddset(&action.sa_mask, SIGALRM);
    if (using_seccomp)
    sigaddset(&action.sa_mask, SIGCHLD);
    if (sig == SIGSEGV)
    flags |= SA_NODEFER;
    if (sigismember(&action.sa_mask, sig))
    flags |= SA_RESTART; /* if it's an irq signal */
    action.sa_flags = flags;
    action.sa_restorer = core::ptr::null_mut();
    if (sigaction(sig, &action, core::ptr::null_mut()) < 0)
    panic("sigaction failed - errno = %d\n", errno);
    sigemptyset(&sig_mask);
    sigaddset(&sig_mask, sig);
    if (sigprocmask(SIG_UNBLOCK, &sig_mask, core::ptr::null_mut()) < 0)
    panic("sigprocmask failed - errno = %d\n", errno);
    }
#[no_mangle]
pub unsafe extern "C" fn send_sigio_to_self() {
    void send_sigio_to_self(void)
    {
    kill(os_getpid(), SIGIO);
    }
#[no_mangle]
pub unsafe extern "C" fn change_sig(signal: c_int, on: c_int) -> c_int {
    int change_sig(int signal, int on)
    {
    sigset_t sigset;
    sigemptyset(&sigset);
    sigaddset(&sigset, signal);
    if (sigprocmask(on ? SIG_UNBLOCK : SIG_BLOCK, &sigset, core::ptr::null_mut()) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __block_signals() {
    static inline void __block_signals(void)
    {
    if (!signals_enabled)
    return;
    os_local_ipi_disable();
    barrier();
    signals_enabled = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __unblock_signals() {
    static inline void __unblock_signals(void)
    {
    if (signals_enabled)
    return;
    signals_enabled = 1;
    barrier();
    os_local_ipi_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn block_signals() {
    void block_signals(void)
    {
    __block_signals();
//
// This must return with signals disabled, so this barrier
// ensures that writes are flushed out before the return.
// This might matter if gcc figures out how to inline this and
// decides to shuffle this code into the caller.
//
    barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn unblock_signals() {
    void unblock_signals(void)
    {
    int save_pending;
    if (signals_enabled == 1)
    return;
    __unblock_signals();

    deliver_time_travel_irqs();

//
// We loop because the IRQ handler returns with interrupts off.  So,
// interrupts may have arrived and we need to re-enable them and
// recheck signals_pending.
//
    while (1) {
//
// Save and reset save_pending after enabling signals.  This
// way, signals_pending won't be changed while we're reading it.
//
// Setting signals_enabled and reading signals_pending must
// happen in this order, so have the barrier here.
//
    barrier();
    save_pending = signals_pending;
    if (save_pending == 0)
    return;
    signals_pending = 0;
//
// We have pending interrupts, so disable signals, as the
// handlers expect them off when they are called.  They will
// be enabled again above. We need to trace this, as we're
// expected to be enabling interrupts already, but any more
// tracing that happens inside the handlers we call for the
// pending signals will mess up the tracing state.
//
    __block_signals();
    um_trace_signals_off();
//
// Deal with SIGIO first because the alarm handler might
// schedule, leaving the pending SIGIO stranded until we come
// back here.
//
// SIGIO's handler doesn't use siginfo or mcontext,
// so they can be NULL.
//
    if (save_pending & SIGIO_MASK)
    sig_handler_common(SIGIO, core::ptr::null_mut(), core::ptr::null_mut());
    if (save_pending & SIGCHLD_MASK) {
    let mut regs: uml_pt_regs = {};
    sigchld_handler(SIGCHLD, core::ptr::null_mut(), &regs, core::ptr::null_mut());
    }
// Do not reenter the handler
    if ((save_pending & SIGALRM_MASK) && (!(signals_active & SIGALRM_MASK)))
    timer_real_alarm_handler(core::ptr::null_mut());
// Rerun the loop only if there is still pending SIGIO and not in TIMER handler
    if (!(signals_pending & SIGIO_MASK) && (signals_active & SIGALRM_MASK))
    return;
// Re-enable signals and trace that we're doing so.
    um_trace_signals_on();
    __unblock_signals();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn um_get_signals() -> c_int {
    int um_get_signals(void)
    {
    return signals_enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn um_set_signals(enable: c_int) -> c_int {
    int um_set_signals(int enable)
    {
    int ret;
    if (signals_enabled == enable)
    return enable;
    ret = signals_enabled;
    if (enable)
    unblock_signals();
    else block_signals();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn um_set_signals_trace(enable: c_int) -> c_int {
    int um_set_signals_trace(int enable)
    {
    int ret;
    if (signals_enabled == enable)
    return enable;
    ret = signals_enabled;
    if (enable)
    unblock_signals_trace();
    else
    block_signals_trace();
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn mark_sigio_pending() {
    void mark_sigio_pending(void)
    {
//
// It would seem that this should be atomic so
// it isn't a read-modify-write with a signal
// that could happen in the middle, losing the
// value set by the signal.
//
// However, this function is only called when in
// time-travel=ext simulation mode, in which case
// the only signal ever pending is SIGIO, which
// is blocked while this can be called, and the
// timer signal (SIGALRM) cannot happen.
//
    signals_pending |= SIGIO_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn block_signals_hard() {
    void block_signals_hard(void)
    {
    signals_blocked++;
    barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn unblock_signals_hard() {
    void unblock_signals_hard(void)
    {
    static bool unblocking;
    if (!signals_blocked)
    panic("unblocking signals while not blocked");
    if (--signals_blocked)
    return;
//
// Must be set to 0 before we check pending so the
// SIGIO handler will run as normal unless we're still
// going to process signals_blocked_pending.
//
    barrier();
//
// Note that block_signals_hard()/unblock_signals_hard() can be called
// within the unblock_signals()/sigio_run_timetravel_handlers() below.
// This would still be prone to race conditions since it's actually a
// call _within_ e.g. vu_req_read_message(), where we observed this
// issue, which loops. Thus, if the inner call handles the recorded
// pending signals, we can get out of the inner call with the real
// signal hander no longer blocked, and still have a race. Thus don't
// handle unblocking in the inner call, if it happens, but only in
// the outermost call - 'unblocking' serves as an ownership for the
// signals_blocked_pending decrement.
//
    if (unblocking)
    return;
    unblocking = true;
    while (__atomic_load_n(&signals_blocked_pending, __ATOMIC_SEQ_CST)) {
    if (signals_enabled) {
// signals are enabled so we can touch this
    signals_pending |= SIGIO_MASK;
//
// this is a bit inefficient, but that's
// not really important
//
    block_signals();
    unblock_signals();
    } else {
//
// we need to run time-travel handlers even
// if not enabled
//
    sigio_run_timetravel_handlers();
    }
//
// The decrement of signals_blocked_pending must be atomic so
// that the signal handler will either happen before or after
// the decrement, not during a read-modify-write:
// - If it happens before, it can increment it and we'll
// decrement it and do another round in the loop.
// - If it happens after it'll see 0 for both signals_blocked
// and signals_blocked_pending and thus run the handler as
// usual (subject to signals_enabled, but that's unrelated.)
//
// Note that a call to unblock_signals_hard() within the calls
// to unblock_signals() or sigio_run_timetravel_handlers() above
// will do nothing due to the 'unblocking' state, so this cannot
// underflow as the only one decrementing will be the outermost
// one.
//
    if (__atomic_sub_fetch(&signals_blocked_pending, 1,
    __ATOMIC_SEQ_CST) < 0)
    panic("signals_blocked_pending underflow");
    }
    unblocking = false;
    }
