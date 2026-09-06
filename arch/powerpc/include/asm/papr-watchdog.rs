//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/papr-watchdog.h
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
//
// H_WATCHDOG Input
//
// R4: "flags":
//
// Bits 48-55: "operation"
//
pub const PSERIES_WDTF_OP_START: c_uint = 0x100UL		/* start timer */;
pub const PSERIES_WDTF_OP_STOP: c_uint = 0x200UL		/* stop timer */;
pub const PSERIES_WDTF_OP_QUERY: c_uint = 0x300UL		/* query timer capabilities */;
//
// Bits 56-63: "timeoutAction" (for "Start Watchdog" only)
//
pub const PSERIES_WDTF_ACTION_HARD_POWEROFF: c_uint = 0x1UL	/* poweroff */;
pub const PSERIES_WDTF_ACTION_HARD_RESTART: c_uint = 0x2UL	/* restart */;
pub const PSERIES_WDTF_ACTION_DUMP_RESTART: c_uint = 0x3UL	/* dump + restart */;
//
// R5: "watchdogNumber":
// PAPR says use -1 (all ones) to stop all watchdogs.
//

//
// H_WATCHDOG Output
//
// R3: Return code
//
// H_SUCCESS    The operation completed.
//
// H_BUSY	    The hypervisor is too busy; retry the operation.
//
// H_PARAMETER  The given "flags" are somehow invalid.  Either the
// "operation" or "timeoutAction" is invalid, or a
// reserved bit is set.
//
// H_P2         The given "watchdogNumber" is zero or exceeds the
// supported maximum value.
//
// H_P3         The given "timeoutInMs" is below the supported
// minimum value.
//
// H_NOOP       The given "watchdogNumber" is already stopped.
//
// H_HARDWARE   The operation failed for ineffable reasons.
//
// H_FUNCTION   The H_WATCHDOG hypercall is not supported by this
// hypervisor.
//
// R4:
//
// - For the "Query Watchdog Capabilities" operation, a 64-bit
// structure:
//

