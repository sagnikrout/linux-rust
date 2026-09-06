//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acconfig.h
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
// Name: acconfig.h - Global configuration constants
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Configuration options
//
// ACPI_DEBUG_OUTPUT    - This switch enables all the debug facilities of the
// ACPI subsystem.  This includes the DEBUG_PRINT output
// statements.  When disabled, all DEBUG_PRINT
// statements are compiled out.
//
// ACPI_APPLICATION     - Use this switch if the subsystem is going to be run
// at the application level.
//
// OS name, used for the _OS object.  The _OS object is essentially obsolete,
// but there is a large base of ASL/AML code in existing machines that check
// for the string below.  The use of this string usually guarantees that
// the ASL will execute down the most tested code path.  Also, there is some
// code that will not execute the _OSI method unless _OS matches the string
// below.  Therefore, change this string at your own risk.
//

// Maximum objects in the various object caches

//
// Should the subsystem abort the loading of an ACPI table if the
// table checksum is incorrect?
//

//
// Generate a version of ACPICA that only supports "reduced hardware"
// platforms (as defined in ACPI 5.0). Set to TRUE to generate a specialized
// version of ACPICA that ONLY supports the ACPI 5.0 "reduced hardware"
// model. In other words, no ACPI hardware is supported.
//
// If TRUE, this means no support for the following:
// PM Event and Control registers
// SCI interrupt (and handler)
// Fixed Events
// General Purpose Events (GPEs)
// Global Lock
// ACPI PM timer
//

//
// Subsystem Constants
//
// Version of ACPI supported
pub const ACPI_CA_SUPPORT_LEVEL: c_int = 5;
// Maximum count for a semaphore object
pub const ACPI_MAX_SEMAPHORE_COUNT: c_int = 256;
// Maximum object reference count (detects object deletion issues)
pub const ACPI_MAX_REFERENCE_COUNT: c_uint = 0x4000;
// Default page size for use in mapping memory for operation regions

// owner_id tracking. 128 entries allows for 4095 owner_ids
pub const ACPI_NUM_OWNERID_MASKS: c_int = 128;
// Size of the root table array is increased by this increment
pub const ACPI_ROOT_TABLE_SIZE_INCREMENT: c_int = 4;
// Maximum sleep allowed via Sleep() operator

// Address Range lists are per-space_id (Memory and I/O only)
pub const ACPI_ADDRESS_RANGE_MAX: c_int = 2;
// Maximum time (default 30s) of While() loops before abort
pub const ACPI_MAX_LOOP_TIMEOUT: c_int = 30;
//
// ACPI Specification constants (Do not change unless the specification changes)
//
// Method info (in WALK_STATE), containing local variables and arguments
pub const ACPI_METHOD_NUM_LOCALS: c_int = 8;
pub const ACPI_METHOD_MAX_LOCAL: c_int = 7;
pub const ACPI_METHOD_NUM_ARGS: c_int = 7;
pub const ACPI_METHOD_MAX_ARG: c_int = 6;
//
// Operand Stack (in WALK_STATE), Must be large enough to contain METHOD_MAX_ARG
//
pub const ACPI_OBJ_NUM_OPERANDS: c_int = 8;
pub const ACPI_OBJ_MAX_OPERAND: c_int = 7;
// Number of elements in the Result Stack frame, can be an arbitrary value
pub const ACPI_RESULTS_FRAME_OBJ_NUM: c_int = 8;
//
// Maximal number of elements the Result Stack can contain,
// it may be an arbitrary value not exceeding the types of
// result_size and result_count (now u8).
//
pub const ACPI_RESULTS_OBJ_NUM_MAX: c_int = 255;
// Constants used in searching for the RSDP in low memory
pub const ACPI_EBDA_PTR_LOCATION: c_uint = 0x0000040E	/* Physical Address */;
pub const ACPI_EBDA_PTR_LENGTH: c_int = 2;
pub const ACPI_EBDA_WINDOW_SIZE: c_int = 1024;
pub const ACPI_HI_RSDP_WINDOW_BASE: c_uint = 0x000E0000	/* Physical Address */;
pub const ACPI_HI_RSDP_WINDOW_SIZE: c_uint = 0x00020000;
pub const ACPI_RSDP_SCAN_STEP: c_int = 16;
// Operation regions
pub const ACPI_USER_REGION_BEGIN: c_uint = 0x80;
// Maximum space_ids for Operation Regions
pub const ACPI_MAX_ADDRESS_SPACE: c_int = 255;
pub const ACPI_NUM_DEFAULT_SPACES: c_int = 4;
// Array sizes.  Used for range checking also
pub const ACPI_MAX_MATCH_OPCODE: c_int = 5;
// RSDP checksums
pub const ACPI_RSDP_CHECKSUM_LENGTH: c_int = 20;
pub const ACPI_RSDP_XCHECKSUM_LENGTH: c_int = 36;
//
// SMBus, GSBus and IPMI buffer sizes. All have a 2-byte header,
// containing both Status and Length.
//

pub const ACPI_SMBUS_DATA_SIZE: c_int = 32;

pub const ACPI_IPMI_DATA_SIZE: c_int = 64;

pub const ACPI_MAX_GSBUS_DATA_SIZE: c_int = 255;

pub const ACPI_PRM_INPUT_BUFFER_SIZE: c_int = 26;
pub const ACPI_FFH_INPUT_BUFFER_SIZE: c_int = 256;
// _sx_d and _sx_w control methods
pub const ACPI_NUM_sx_d_METHODS: c_int = 4;
pub const ACPI_NUM_sx_w_METHODS: c_int = 5;
//
// Miscellaneous constants
//
// UUID constants

// Positions for required hyphens (dashes) in UUID strings
pub const UUID_HYPHEN1_OFFSET: c_int = 8;
pub const UUID_HYPHEN2_OFFSET: c_int = 13;
pub const UUID_HYPHEN3_OFFSET: c_int = 18;
pub const UUID_HYPHEN4_OFFSET: c_int = 23;
//
// ACPI AML Debugger
//

pub const ACPI_DB_LINE_BUFFER_SIZE: c_int = 512;

