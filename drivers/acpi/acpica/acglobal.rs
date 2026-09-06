//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acglobal.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acglobal.h - Declarations for global variables
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Globals related to the incoming ACPI tables
//
// Master list of all ACPI tables that were found in the RSDT/XSDT
// DSDT information. Used to check for DSDT corruption
// These addresses are calculated from the FADT Event Block addresses

//
// Handle both ACPI 1.0 and ACPI 2.0+ Integer widths. The integer width is
// determined by the revision of the DSDT: If the DSDT revision is less than
// 2, use only the lower 32 bits of the internal 64-bit Integer.
//
// Mutual exclusion within the ACPICA subsystem
//
// Predefined mutex objects. This array contains the
// actual OS mutex handles, indexed by the local ACPI_MUTEX_HANDLEs.
// (The table maps local handles to the real OS handles)
//
// Global lock mutex is an actual AML mutex object
// Global lock semaphore works in conjunction with the actual global lock
// Global lock spinlock is used for "pending" handshake
//
// Spinlocks are used for interfaces that can be possibly called at
// interrupt level
//
// Mutex for _OSI support
// Reader/Writer lock is used for namespace walk and dynamic table unload
//
// Miscellaneous globals
//
// Object caches
// System
// Global handlers
// Owner ID support
// Initialization sequencing
// Miscellaneous
// Other miscellaneous, declared and initialized in utglobal
// Lists for tracking memory allocations (debug only)

//
// ACPI Namespace
//
pub const NUM_PREDEFINED_NAMES: c_int = 10;

//
// Interpreter/Parser globals
//
// Control method single step flag
// ASL/ASL+ converter
//
// Hardware globals
//
// Event and GPE globals
//

//
// Debug support
//
// Event counters
// Dynamic control method tracing mechanism
//
// Debugger and Disassembler
//

// Do not disassemble buffers to resource descriptors

// These buffers should all be the same size
// Statistics globals

//
// ACPICA application-specific globals
//
// ASL-to-ASL+ conversion utility (implemented within the iASL compiler)

// acpi_gbl_comment_addr_list_head, NULL);

// Print buffer

