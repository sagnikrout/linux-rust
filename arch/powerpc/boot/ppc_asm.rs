//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/ppc_asm.h
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
// Definitions used by various bits of low-level assembly code on PowerPC.
//
// Copyright (C) 1995-1999 Gary Thomas, Paul Mackerras, Cort Dougan.
//
// Condition Register Bit Fields
pub const cr0: c_int = 0;
pub const cr1: c_int = 1;
pub const cr2: c_int = 2;
pub const cr3: c_int = 3;
pub const cr4: c_int = 4;
pub const cr5: c_int = 5;
pub const cr6: c_int = 6;
pub const cr7: c_int = 7;
// General Purpose Registers (GPRs)
pub const r0: c_int = 0;
pub const r1: c_int = 1;
pub const r2: c_int = 2;
pub const r3: c_int = 3;
pub const r4: c_int = 4;
pub const r5: c_int = 5;
pub const r6: c_int = 6;
pub const r7: c_int = 7;
pub const r8: c_int = 8;
pub const r9: c_int = 9;
pub const r10: c_int = 10;
pub const r11: c_int = 11;
pub const r12: c_int = 12;
pub const r13: c_int = 13;
pub const r14: c_int = 14;
pub const r15: c_int = 15;
pub const r16: c_int = 16;
pub const r17: c_int = 17;
pub const r18: c_int = 18;
pub const r19: c_int = 19;
pub const r20: c_int = 20;
pub const r21: c_int = 21;
pub const r22: c_int = 22;
pub const r23: c_int = 23;
pub const r24: c_int = 24;
pub const r25: c_int = 25;
pub const r26: c_int = 26;
pub const r27: c_int = 27;
pub const r28: c_int = 28;
pub const r29: c_int = 29;
pub const r30: c_int = 30;
pub const r31: c_int = 31;
pub const SPRN_TBRL: c_int = 268;
pub const SPRN_TBRU: c_int = 269;
pub const SPRN_HSRR0: c_uint = 0x13A	/* Hypervisor Save/Restore 0 */;
pub const SPRN_HSRR1: c_uint = 0x13B	/* Hypervisor Save/Restore 1 */;
pub const MSR_LE: c_uint = 0x0000000000000001;

