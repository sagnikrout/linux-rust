//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/siginfo.h
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

pub const SI_MAX_SIZE: c_int = 128;
//
// The default "si_band" type is "long", as specified by POSIX.
// However, some architectures want to override this to "int"
// for historical compatibility reasons, so we allow that.
//

//
// Be careful when extending this union.  On 32bit siginfo_t is 32bit
// aligned.  Which means that a 64bit field or any other field that
// would increase the alignment of siginfo_t will break the ABI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sifields {
// kill()
    pub /: *mut *mut __kernel_pid_t _pid; / sender's pid,
    pub /: *mut *mut __kernel_uid32_t _uid; / sender's uid,
    pub _kill: },
// POSIX.1b timers
    pub /: *mut *mut __kernel_timer_t _tid; / timer id,
    pub /: *mut *mut int _overrun; / overrun count,
    pub /: *mut *mut sigval_t _sigval; / same as below,
    pub /: *mut *mut int _sys_private; / Not used by the kernel. Historic leftover. Always 0.,
    pub _timer: },
// POSIX.1b signals
    pub /: *mut *mut __kernel_pid_t _pid; / sender's pid,
    pub /: *mut *mut __kernel_uid32_t _uid; / sender's uid,
    pub _sigval: sigval_t,
    pub _rt: },
// SIGCHLD
    pub /: *mut *mut __kernel_pid_t _pid; / which child,
    pub /: *mut *mut __kernel_uid32_t _uid; / sender's uid,
    pub /: *mut *mut int _status; / exit code,
    pub _utime: __ARCH_SI_CLOCK_T,
    pub _stime: __ARCH_SI_CLOCK_T,
    pub _sigchld: },
// SIGILL, SIGFPE, SIGSEGV, SIGBUS, SIGTRAP, SIGEMT
    pub /: *mut *mut *mut void __user _addr; / faulting insn/memory ref.,

// used on alpha and sparc
    pub /: *mut *mut int _trapno; / TRAP # which caused the signal,
//
// used when si_code=BUS_MCEERR_AR or
// used when si_code=BUS_MCEERR_AO
//
    pub /: *mut *mut short _addr_lsb; / LSB of the reported address,
// used when si_code=SEGV_BNDERR
    pub _dummy_bnd: [c_char; __ADDR_BND_PKEY_PAD],
    pub _lower: *mut void __user,
    pub _upper: *mut void __user,
    pub _addr_bnd: },
// used when si_code=SEGV_PKUERR
    pub _dummy_pkey: [c_char; __ADDR_BND_PKEY_PAD],
    pub _pkey: __u32,
    pub _addr_pkey: },
// used when si_code=TRAP_PERF
    pub _data: c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
    pub _perf: },
}

// SIGPOLL
// SIGSYS

//
// How these fields are to be accessed.
//

//
// si_code values
// Digital reserves positive values for kernel-generated signals.
//

pub const SI_KERNEL: c_uint = 0x80		/* sent by the kernel from somewhere */;

//
// SIGILL si_codes
//

pub const NSIGILL: c_int = 11;
//
// SIGFPE si_codes
//

pub const NSIGFPE: c_int = 15;
//
// SIGSEGV si_codes
//

pub const NSIGSEGV: c_int = 10;
//
// SIGBUS si_codes
//

// hardware memory error consumed on a machine check: action required
pub const BUS_MCEERR_AR: c_int = 4;
// hardware memory error detected in process but not consumed: action optional
pub const BUS_MCEERR_AO: c_int = 5;
pub const NSIGBUS: c_int = 5;
//
// SIGTRAP si_codes
//

pub const NSIGTRAP: c_int = 6;
//
// There is an additional set of SIGTRAP si_codes used by ptrace
// that are of the form: ((PTRACE_EVENT_XXX << 8) | SIGTRAP)
//
// Flags for si_perf_flags if SIGTRAP si_code is TRAP_PERF.
//

//
// SIGCHLD si_codes
//

pub const NSIGCHLD: c_int = 6;
//
// SIGPOLL (or any other signal without signal specific si_codes) si_codes
//

pub const NSIGPOLL: c_int = 6;
//
// SIGSYS si_codes
//

pub const NSIGSYS: c_int = 2;
//
// SIGEMT si_codes
//

pub const NSIGEMT: c_int = 1;
//
// sigevent definitions
//
// It seems likely that SIGEV_THREAD will have to be handled from
// userspace, libpthread transmuting it to SIGEV_SIGNAL, which the
// thread manager then catches and does the appropriate nonsense.
// However, everything is written out here so as to not get lost.
//

//
// This works because the alignment is ok on all current architectures
// but we leave open this being overridden in the future
//

pub const SIGEV_MAX_SIZE: c_int = 64;

