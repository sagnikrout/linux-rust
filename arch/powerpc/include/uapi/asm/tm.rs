//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/tm.h
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
// Reason codes describing kernel causes for transaction aborts.  By
// convention, bit0 is copied to TEXASR[56] (IBM bit 7) which is set if
// the failure is persistent.  PAPR saves 0xff-0xe0 for the hypervisor.
//
pub const TM_CAUSE_PERSISTENT: c_uint = 0x01;
pub const TM_CAUSE_KVM_RESCHED: c_uint = 0xe0  /* From PAPR */;
pub const TM_CAUSE_KVM_FAC_UNAV: c_uint = 0xe2  /* From PAPR */;
pub const TM_CAUSE_RESCHED: c_uint = 0xde;
pub const TM_CAUSE_TLBI: c_uint = 0xdc;
pub const TM_CAUSE_FAC_UNAV: c_uint = 0xda;
pub const TM_CAUSE_SYSCALL: c_uint = 0xd8;
pub const TM_CAUSE_MISC: c_uint = 0xd6  /* future use */;
pub const TM_CAUSE_SIGNAL: c_uint = 0xd4;
pub const TM_CAUSE_ALIGNMENT: c_uint = 0xd2;
pub const TM_CAUSE_EMULATE: c_uint = 0xd0;
