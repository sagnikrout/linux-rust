//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kgdb.h
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


//
// The PowerPC (32/64) specific defines / externs for KGDB.  Based on
// the previous 32bit and 64bit specific files, which had the following
// copyrights:
//
// PPC64 Mods (C) 2005 Frank Rowand (frowand@mvista.com)
// PPC Mods (C) 2004 Tom Rini (trini@mvista.com)
// PPC Mods (C) 2003 John Whitney (john.whitney@timesys.com)
// PPC Mods (C) 1998 Michael Tesch (tesch@cs.wisc.edu)
//
// Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
// Author: Tom Rini <trini@kernel.crashing.org>
//
// 2006 (c) MontaVista Software, Inc. This file is licensed under
// the terms of the GNU General Public License version 2. This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

pub const BREAK_INSTR_SIZE: c_int = 4;

pub const BREAK_INSTR: c_uint = 0x7d821008	/* twge r2, r2 */;
pub const CACHE_FLUSH_IS_SAFE: c_int = 1;
pub const DBG_MAX_REG_NUM: c_int = 70;
// The number bytes of registers we have to save depends on a few
// things.  For 64bit we default to not including vector registers and
// vector state registers.

//
// 64 bit (8 byte) registers:
// 32 gpr, 32 fpr, nip, msr, link, ctr
// 32 bit (4 byte) registers:
// ccr, xer, fpscr
//

pub const NUMCRITREGBYTES: c_int = 184;

// On non-E500 family PPC32 we determine the size by picking the last
// register we need, but on E500 we skip sections so we list what we
// need to store, and add it up.

// 32 GPRs (8 bytes), nip, msr, ccr, link, ctr, xer, acc (8 bytes), spefscr

// CR/LR, R1, R2, R13-R31 inclusive.

