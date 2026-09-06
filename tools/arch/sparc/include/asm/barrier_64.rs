//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/sparc/include/asm/barrier_64.h
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
// Copied from the kernel sources to tools/:
//
// These are here in an effort to more fully work around Spitfire Errata
// #51.  Essentially, if a memory barrier occurs soon after a mispredicted
// branch, the chip can stop executing instructions until a trap occurs.
// Therefore, if interrupts are disabled, the chip can hang forever.
//
// It used to be believed that the memory barrier had to be right in the
// delay slot, but a case has been traced recently wherein the memory barrier
// was one instruction after the branch delay slot and the chip still hung.
// The offending sequence was the following in sym_wakeup_done() of the
// sym53c8xx_2 driver:
//
// call	sym_ccb_from_dsa, 0
// movge	%icc, 0, %l0
// brz,pn	%o0, .LL1303
// mov	%o0, %l2
// membar	#LoadLoad
//
// The branch has to be mispredicted for the bug to occur.  Therefore, we put
// the memory barrier explicitly into a "branch always, predicted taken"
// delay slot to avoid the problem case.
//

// The kernel always executes in TSO memory model these days,
// and furthermore most sparc64 chips implement more stringent
// memory ordering than required by the specifications.
//

