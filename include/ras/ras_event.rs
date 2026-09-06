//! Automatically rewritten from C Header to Rust Module
//! Source: include/ras/ras_event.h
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
// MCE Extended Error Log trace event
//
// These events are generated when hardware detects a corrected or
// uncorrected event.
//
// memory trace event

//
// Hardware Events Report
//
// Those events are generated when hardware detected a corrected or
// uncorrected event, and are meant to replace the current API to report
// errors defined on both EDAC and MCE subsystems.
//
// FIXME: Add events for handling memory errors originated from the
// MCE subsystem.
//
// Hardware-independent Memory Controller specific events
//
// Default error mechanisms for Memory Controller errors (CE and UE)
//
// ARM Processor Events Report
//
// This event is generated when hardware detects an ARM processor error
// has occurred. UEFI 2.6 spec section N.2.4.4.
//

//
// Non-Standard Section Report
//
// This event is generated when hardware detected a hardware
// error event, which may be of non-standard section as defined
// in UEFI spec appendix "Common Platform Error Record", or may
// be of sections for which TRACE_EVENT is not defined.
//

//
// PCIe AER Trace event
//
// These events are generated when hardware detects a corrected or
// uncorrected event on a PCIe device. The event report has
// the following structure:
//
// char * dev_name -	The name of the slot where the device resides
// ([domain:]bus:device.function).
// u32 status -		Either the correctable or uncorrectable register
// indicating what error or errors have been seen
// u8 severity -	error severity 0:NONFATAL 1:FATAL 2:CORRECTED
//

// This part must be outside protection
