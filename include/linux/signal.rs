//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/signal.h
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

// for sysctl

extern "C" {
    pub fn copy_siginfo_to_user(to: *mut siginfo_t __user, from: *const kernel_siginfo_t) -> c_int;
}
extern "C" {
    pub fn copy_siginfo_from_user(to: *mut kernel_siginfo_t, from: *const siginfo_t __user) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siginfo_layout {
    SIL_KILL,
    SIL_TIMER,
    SIL_POLL,
    SIL_FAULT,
    SIL_FAULT_TRAPNO,
    SIL_FAULT_MCEERR,
    SIL_FAULT_BNDERR,
    SIL_FAULT_PKUERR,
    SIL_FAULT_PERF_EVENT,
    SIL_CHLD,
    SIL_RT,
    SIL_SYS,
}

extern "C" {
    pub fn siginfo_layout(sig: unsigned, si_code: c_int) -> siginfo_layout;
}
//
// Define some primitives to manipulate sigset_t.
//

// We don't use <linux/bitops.h> for these because there is no need to

// Some extensions for manipulating the low 32 signals in particular.

extern "C" {
    pub fn flush_sigqueue(queue: *mut sigpending);
}
// Test if 'sig' is valid signal. Use this instead of testing _NSIG directly
extern "C" {
    pub fn next_signal(pending: *mut sigpending, mask: *mut sigset_t) -> c_int;
}
extern "C" {
    pub fn sigprocmask(_arg: c_int, : *mut sigset_t, : *mut sigset_t) -> c_int;
}
extern "C" {
    pub fn set_current_blocked(: *mut sigset_t);
}
extern "C" {
    pub fn __set_current_blocked(: *const sigset_t);
}
extern "C" {
    pub fn get_signal(ksig: *mut ksignal) -> bool;
}
extern "C" {
    pub fn signal_setup_done(failed: c_int, ksig: *mut ksignal, stepping: c_int);
}
extern "C" {
    pub fn exit_signals(tsk: *mut task_struct);
}
extern "C" {
    pub fn kernel_sigaction(_arg: c_int, _arg: __sighandler_t);
}

//
// Kernel threads handle their own signals. Let the signal code
// know it'll be handled, so that they don't get converted to
// SIGKILL or just silently dropped.
//
// Kernel threads handle their own signals. Let the signal code
// know signals sent by the kernel will be handled, so that they
// don't get silently dropped.
//
extern "C" {
    pub fn unhandled_signal(tsk: *mut task_struct, sig: c_int) -> bool;
}
//
// In POSIX a signal is sent either to a specific thread (Linux task)
// or to the process as a whole (Linux thread group).  How the signal
// is sent determines whether it's to one thread or the whole group,
// which determines which signal mask(s) are involved in blocking it
// from being delivered until later.  When the signal is delivered,
// either it's caught or ignored by a user handler or it has a default
// effect that applies to the whole thread group (POSIX process).
//
// The possible effects an unblocked signal set to SIG_DFL can have are:
// ignore	- Nothing Happens
// terminate	- kill the process, i.e. all threads in the group,
// similar to exit_group.  The group leader (only) reports
// WIFSIGNALED status to its parent.
// coredump	- write a core dump file describing all threads using
// the same mm and then kill all those threads
// stop 	- stop all the threads in the group, i.e. TASK_STOPPED state
//
// SIGKILL and SIGSTOP cannot be caught, blocked, or ignored.
// Other signals when not blocked and set to SIG_DFL behaves as follows.
// The job control signals also have other special effects.
//
// +--------------------+------------------+
// |  POSIX signal      |  default action  |
// +--------------------+------------------+
// |  SIGHUP            |  terminate	|
// |  SIGINT            |	terminate	|
// |  SIGQUIT           |	coredump 	|
// |  SIGILL            |	coredump 	|
// |  SIGTRAP           |	coredump 	|
// |  SIGABRT/SIGIOT    |	coredump 	|
// |  SIGBUS            |	coredump 	|
// |  SIGFPE            |	coredump 	|
// |  SIGKILL           |	terminate(+)	|
// |  SIGUSR1           |	terminate	|
// |  SIGSEGV           |	coredump 	|
// |  SIGUSR2           |	terminate	|
// |  SIGPIPE           |	terminate	|
// |  SIGALRM           |	terminate	|
// |  SIGTERM           |	terminate	|
// |  SIGCHLD           |	ignore   	|
// |  SIGCONT           |	ignore(*)	|
// |  SIGSTOP           |	stop(*)(+)  	|
// |  SIGTSTP           |	stop(*)  	|
// |  SIGTTIN           |	stop(*)  	|
// |  SIGTTOU           |	stop(*)  	|
// |  SIGURG            |	ignore   	|
// |  SIGXCPU           |	coredump 	|
// |  SIGXFSZ           |	coredump 	|
// |  SIGVTALRM         |	terminate	|
// |  SIGPROF           |	terminate	|
// |  SIGPOLL/SIGIO     |	terminate	|
// |  SIGSYS/SIGUNUSED  |	coredump 	|
// |  SIGSTKFLT         |	terminate	|
// |  SIGWINCH          |	ignore   	|
// |  SIGPWR            |	terminate	|
// |  SIGRTMIN-SIGRTMAX |	terminate       |
// +--------------------+------------------+
// |  non-POSIX signal  |  default action  |
// +--------------------+------------------+
// |  SIGEMT            |  coredump	|
// +--------------------+------------------+
//
// (+) For SIGKILL and SIGSTOP the action is "always", not just "default".
// (*) Special job control effects:
// When SIGCONT is sent, it resumes the process (all threads in the group)
// from TASK_STOPPED state and also clears any pending/queued stop signals
// (any of those marked with "stop(*)").  This happens regardless of blocking,
// catching, or ignoring SIGCONT.  When any stop signal is sent, it clears
// any pending/queued SIGCONT signals; this happens regardless of blocking,
// catching, or ignored the stop signal, though (except for SIGSTOP) the
// default action of stopping the process may happen later or never.
//

pub const SIGEMT_MASK: c_int = 0;

extern "C" {
    pub fn signals_init();
}
extern "C" {
    pub fn restore_altstack(: *const stack_t __user) -> c_int;
}
extern "C" {
    pub fn __save_altstack(: *mut stack_t __user, long: unsigned) -> c_int;
}

extern "C" {
    pub fn sigaltstack_size_valid(ss_size: usize) -> bool;
}

extern "C" {
    pub fn render_sigset_t(: *mut seq_file, : *const c_char, : *mut sigset_t);
}

//
// Given a fault address and a signal and si_code which correspond to the
// _sigfault union member, returns the address that must appear in si_addr if
// the signal handler does not have SA_EXPOSE_TAGBITS enabled in sa_flags.
//

