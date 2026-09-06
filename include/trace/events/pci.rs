//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/pci.h
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

// Enums require being exported to userspace, for user tool parsing

//
// Now redefine the EM() and EMe() macros to map the enums to the strings
// that will be printed in the output.
//

//
// Note: For generic PCI hotplug events, we pass already-resolved strings
// (port_name, slot) instead of driver-specific structures like 'struct
// controller'.  This is because different PCI hotplug drivers (pciehp, cpqphp,
// ibmphp, shpchp) define their own versions of 'struct controller' with
// different fields and helper functions. Using driver-specific structures would
// make the tracepoint interface non-generic and cause compatibility issues
// across different drivers.
//

// This part must be outside protection
