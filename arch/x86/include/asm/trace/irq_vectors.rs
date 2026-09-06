//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/trace/irq_vectors.h
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
// local_timer - called when entering/exiting a local timer interrupt
// vector handler
//
// spurious_apic - called when entering/exiting a spurious apic vector handler
//
// error_apic - called when entering/exiting an error apic vector handler
//
// x86_platform_ipi - called when entering/exiting a x86 platform ipi interrupt
// vector handler
//

//
// irq_work - called when entering/exiting a irq work interrupt
// vector handler
//
// We must dis-allow sampling irq_work_exit() because perf event sampling
// itself can cause irq_work, which would lead to an infinite loop;
//
// 1) irq_work_exit happens
// 2) generates perf sample
// 3) generates irq_work
// 4) goto 1
//

//
// The ifdef is required because that tracepoint macro hell emits tracepoint
// code in files which include this header even if the tracepoint is not
// enabled. Brilliant stuff that.
//

//
// reschedule - called when entering/exiting a reschedule vector handler
//
// call_function - called when entering/exiting a call function interrupt
// vector handler
//
// call_function_single - called when entering/exiting a call function
// single interrupt vector handler
//

//
// threshold_apic - called when entering/exiting a threshold apic interrupt
// vector handler
//

//
// deferred_error_apic - called when entering/exiting a deferred apic interrupt
// vector handler
//

//
// thermal_apic - called when entering/exiting a thermal apic interrupt
// vector handler
//

// This part must be outside protection
