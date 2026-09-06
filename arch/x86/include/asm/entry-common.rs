//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/entry-common.h
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


// SPDX-License-Identifier: GPL-2.0-only

// Check that the stack and regs on entry from user mode are sane.
//
// Make sure that the entry code gave us a sensible EFLAGS
// register.  Native because we want to check the actual CPU
// state, not the interrupt state as imagined by Xen.
//
// For !SMAP hardware we patch out CLAC on entry.
//
// We think we came from user mode. Make sure pt_regs agrees.
//
// All entries from user mode (except #DF) should be on the
// normal thread stack and should have user pt_regs in the
// correct location.
//

//
// Compat syscalls set TS_COMPAT.  Make sure we clear it before
// returning to user mode.  We need to clear it *after* signal
// handling, because syscall restart has a fixup for compat
// syscalls.  The fixup is exercised by the ptrace_syscall_32
// selftest.
//
// We also need to clear TS_REGS_POKED_I386: the 32-bit tracer
// special case only applies after poking regs and before the
// very next return to user mode.
//

// Avoid unnecessary reads of 'x86_ibpb_exit_to_user'

extern "C" {
    pub fn x86_entry_from_kvm(entry_type: c_uint, vector: c_uint);
}
