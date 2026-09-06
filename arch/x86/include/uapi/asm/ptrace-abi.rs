//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/ptrace-abi.h
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

pub const EBX: c_int = 0;
pub const ECX: c_int = 1;
pub const EDX: c_int = 2;
pub const ESI: c_int = 3;
pub const EDI: c_int = 4;
pub const EBP: c_int = 5;
pub const EAX: c_int = 6;
pub const DS: c_int = 7;
pub const ES: c_int = 8;
pub const FS: c_int = 9;
pub const GS: c_int = 10;
pub const ORIG_EAX: c_int = 11;
pub const EIP: c_int = 12;
pub const CS: c_int = 13;
pub const EFL: c_int = 14;
pub const UESP: c_int = 15;
pub const SS: c_int = 16;
pub const FRAME_SIZE: c_int = 17;

//
// C ABI says these regs are callee-preserved. They aren't saved on kernel entry
// unless syscall needs a complete, fully filled "struct pt_regs".
//
pub const R15: c_int = 0;
pub const R14: c_int = 8;
pub const R13: c_int = 16;
pub const R12: c_int = 24;
pub const RBP: c_int = 32;
pub const RBX: c_int = 40;
// These regs are callee-clobbered. Always saved on kernel entry.
pub const R11: c_int = 48;
pub const R10: c_int = 56;
pub const R9: c_int = 64;
pub const R8: c_int = 72;
pub const RAX: c_int = 80;
pub const RCX: c_int = 88;
pub const RDX: c_int = 96;
pub const RSI: c_int = 104;
pub const RDI: c_int = 112;
//
// On syscall entry, this is syscall#. On CPU exception, this is error code.
// On hw interrupt, it's IRQ number:
//
pub const ORIG_RAX: c_int = 120;
// Return frame for iretq
pub const RIP: c_int = 128;
pub const CS: c_int = 136;
pub const EFLAGS: c_int = 144;
pub const RSP: c_int = 152;
pub const SS: c_int = 160;

// top of stack page
pub const FRAME_SIZE: c_int = 168;

// Arbitrarily choose the same ptrace numbers as used by the Sparc code.
pub const PTRACE_GETREGS: c_int = 12;
pub const PTRACE_SETREGS: c_int = 13;
pub const PTRACE_GETFPREGS: c_int = 14;
pub const PTRACE_SETFPREGS: c_int = 15;
pub const PTRACE_GETFPXREGS: c_int = 18;
pub const PTRACE_SETFPXREGS: c_int = 19;
pub const PTRACE_OLDSETOPTIONS: c_int = 21;
// only useful for access 32bit programs / kernels
pub const PTRACE_GET_THREAD_AREA: c_int = 25;
pub const PTRACE_SET_THREAD_AREA: c_int = 26;

pub const PTRACE_SYSEMU: c_int = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: c_int = 32;

