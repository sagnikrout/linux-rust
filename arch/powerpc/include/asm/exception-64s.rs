//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/exception-64s.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Extracted from head_64.S
//
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Rewritten by Cort Dougan (cort@cs.nmt.edu) for PReP
// Copyright (C) 1996 Cort Dougan <cort@cs.nmt.edu>
// Adapted for Power Macintosh by Paul Mackerras.
// Low-level exception handlers and MMU support
// rewritten by Paul Mackerras.
// Copyright (C) 1996 Paul Mackerras.
//
// Adapted for 64bit PowerPC by Dave Engebretsen, Peter Bergner, and
// Mike Corrigan {engebret|bergner|mikejc}@us.ibm.com
//
// This file contains the low-level support and setup for the
// PowerPC-64 platform, including trap and interrupt dispatch.
//
// The following macros define the code that appears as
// the prologue to each of the exception handlers.  They
// are split into two parts to allow a single kernel binary
// to be used for pSeries and iSeries.
//
// We make as much of the exception code common between native
// exception handlers (including pSeries LPAR) and iSeries LPAR
// implementations as possible.
//

// PACA save area size in u64 units (exgen, exmc, etc)
pub const EX_SIZE: c_int = 10;
// PACA save area offsets
pub const EX_R9: c_int = 0;
pub const EX_R10: c_int = 8;
pub const EX_R11: c_int = 16;
pub const EX_R12: c_int = 24;
pub const EX_R13: c_int = 32;
pub const EX_DAR: c_int = 40;
pub const EX_DSISR: c_int = 48;
pub const EX_CCR: c_int = 52;
pub const EX_CFAR: c_int = 56;
pub const EX_PPR: c_int = 64;
pub const EX_CTR: c_int = 72;
//
// maximum recursive depth of MCE exceptions
//
pub const MAX_MCE_DEPTH: c_int = 4;

//
// r10 must be free to use, r13 must be paca
//

//
// r10, ctr must be free to use, r13 must be paca
//

//
// Macros for annotating the expected destination of (h)rfid
//
// The nop instructions allow us to insert one or more instructions to flush the
// L1-D cache when returning to userspace or a guest.
//
// powerpc relies on return from interrupt/syscall being context synchronising
// (which hrfid, rfid, and rfscv are) to support ARCH_HAS_MEMBARRIER_SYNC_CORE
// without additional synchronisation instructions.
//
// soft-masked interrupt replay does not include a context-synchronising rfid,
// but those always return to kernel, the sync is only required when returning
// to user.
//

// Prototype for function defined in exceptions-64s.S
extern "C" {
    pub fn do_uaccess_flush();
}

