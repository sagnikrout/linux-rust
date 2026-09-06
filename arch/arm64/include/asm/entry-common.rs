//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/entry-common.h
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

//
// DAIF.DA are cleared at the start of IRQ/FIQ handling, and when GIC
// priority masking is used the GIC irqchip driver will clear DAIF.IF
// in gic_unmask_pnmis() for normal IRQs. If anything is set in
// DAIF we must have handled an NMI, so skip preemption.
//
// Preempting a task from an IRQ means we leave copies of PSTATE
// on the stack. cpufeature's enable calls may modify PSTATE, but
// resuming one of these preempted tasks would undo those changes.
//
// Only allow a task to be preempted once cpufeatures have been
// enabled.
//

