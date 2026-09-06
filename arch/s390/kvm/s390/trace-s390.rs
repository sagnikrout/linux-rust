//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kvm/s390/trace-s390.h
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
// The TRACE_SYSTEM_VAR defaults to TRACE_SYSTEM, but must be a
// legitimate C variable. It is not exported to user space.
//

//
// Trace point for the creation of the kvm instance.
//
// Trace points for creation and destruction of vpcus.
//
// Trace point for start and stop of vpcus.
//
// Trace points for injection of interrupts, either per machine or
// per vcpu.
//

//
// Trace point for the actual delivery of interrupts.
//
// Trace point for resets that may be requested from userspace.
//
// Trace point for a vcpu's stop requests.
//
// Trace point for enabling channel I/O instruction support.
//
// Trace point for enabling and disabling interlocking-and-broadcasting
// suppression.
//
// Trace point for modifying ais mode for a given isc.
//
// Trace point for suppressed adapter I/O interrupt.
//
// Trace point for gmap notifier calls.
//

// This part must be outside protection
