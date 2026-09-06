//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/soc/renesas,r8a78000-mfis.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Constants for the second mbox-cell of the Renesas MFIS IP core. To be treated
// as bit flags which can be ORed.
//
// MFIS HW design before r8a78001 requires a channel to be marked as either
// TX or RX.
//

//
// MFIS variants before r8a78001 work with pairs of IICR and EICR registers.
// Usually, it is specified in the datasheets which of the two a specific core
// should use. Then, it does not need extra description in DT. For plain MFIS
// of r8a78000, this is selectable, though. According to the system design and
// the firmware in use, these channels need to be marked. This is not needed
// with other versions of the MFIS, not even with MFIS-SCP of r8a78000.
//

