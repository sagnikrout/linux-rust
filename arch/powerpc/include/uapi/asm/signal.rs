//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/signal.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const _NSIG: c_int = 64;

pub const _NSIG_BPW: c_int = 64;

pub const _NSIG_BPW: c_int = 32;

pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGTRAP: c_int = 5;
pub const SIGABRT: c_int = 6;
pub const SIGIOT: c_int = 6;
pub const SIGBUS: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGUSR1: c_int = 10;
pub const SIGSEGV: c_int = 11;
pub const SIGUSR2: c_int = 12;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;
pub const SIGSTKFLT: c_int = 16;
pub const SIGCHLD: c_int = 17;
pub const SIGCONT: c_int = 18;
pub const SIGSTOP: c_int = 19;
pub const SIGTSTP: c_int = 20;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 23;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGIO: c_int = 29;

//
pub const SIGLOST: c_int = 29;
//
pub const SIGPWR: c_int = 30;
pub const SIGSYS: c_int = 31;
pub const SIGUNUSED: c_int = 31;
// These should not be considered constants from userland.
pub const SIGRTMIN: c_int = 32;

pub const SA_RESTORER: c_uint = 0x04000000U;

pub const MINSIGSTKSZ: c_int = 8192;
pub const SIGSTKSZ: c_int = 32768;

pub const MINSIGSTKSZ: c_int = 2048;
pub const SIGSTKSZ: c_int = 8192;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_mask: old_sigset_t,
    pub sa_flags: c_ulong,
    pub sa_restorer: __sigrestore_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub /: *mut *mut sigset_t sa_mask; / mask last for extensibility,
}

//
// These are parameters to dbg_sigreturn syscall.  They enable or
// disable certain debugging things that can be done from signal
// handlers.  The dbg_sigreturn syscall *must* be called from a
// SA_SIGINFO signal so the ucontext can be passed to it.  It takes an
// array of struct sig_dbg_op, which has the debug operations to
// perform before returning from the signal.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sig_dbg_op {
    pub dbg_type: c_int,
    pub dbg_value: c_ulong,
}

// Enable or disable single-stepping.  The value sets the state.
pub const SIG_DBG_SINGLE_STEPPING: c_int = 1;
// Enable or disable branch tracing.  The value sets the state.
pub const SIG_DBG_BRANCH_TRACING: c_int = 2;

