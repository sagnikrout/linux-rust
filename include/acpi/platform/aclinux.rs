//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/platform/aclinux.h
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
// Name: aclinux.h - OS specific defines, etc. for Linux
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

// ACPICA external files should not include ACPICA headers directly.

// Common (in-kernel/user-space) ACPICA configuration
// Macro flag: #define ACPI_USE_SYSTEM_CLIBRARY
// Macro flag: #define ACPI_USE_DO_WHILE_0
// Macro flag: #define ACPI_IGNORE_PACKAGE_RESOLUTION_ERRORS

// Macro flag: #define ACPI_USE_SYSTEM_INTTYPES
// Macro flag: #define ACPI_USE_GPE_POLLING
// Kernel specific ACPICA configuration

// Macro flag: #define ACPI_PCI_CONFIGURED

pub const ACPI_REDUCED_HARDWARE: c_int = 1;

// Macro flag: #define ACPI_DEBUGGER

// Macro flag: #define ACPI_MUTEX_DEBUG

// Use a specific bugging default separate from ACPICA

// External globals for __KERNEL__, stubs is needed
// Macro flag: #define ACPI_GLOBAL(t,a)
// Macro flag: #define ACPI_INIT_GLOBAL(t,a,b)
// Generating stubs for configurable ACPICA macros
// Macro flag: #define ACPI_NO_MEM_ALLOCATIONS
// Generating stubs for configurable ACPICA functions
// Macro flag: #define ACPI_NO_ERROR_MESSAGES

// External interface for __KERNEL__, stub is needed

// Host-dependent types and defines for in-kernel ACPICA

// Macro flag: #define ACPI_USE_NATIVE_MATH64

// Use native linux version of acpi_os_allocate_zeroed
// Macro flag: #define USE_NATIVE_ALLOCATE_ZEROED
// Use logical addresses for accessing GPE registers in system memory
// Macro flag: #define ACPI_GPE_USE_LOGICAL_ADDRESSES
//
// Overrides for in-kernel ACPICA
//
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_initialize
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_terminate
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_allocate
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_allocate_zeroed
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_free
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_acquire_object
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_get_thread_id
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_create_lock
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_create_raw_lock
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_delete_raw_lock
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_acquire_raw_lock
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_release_raw_lock
//
// OSL interfaces used by debugger/disassembler
//
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_readable
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_writable
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_initialize_debugger
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_terminate_debugger
//
// OSL interfaces used by utilities
//
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_redirect_output
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_get_table_by_name
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_get_table_by_index
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_get_table_by_address
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_open_directory
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_get_next_filename
// Macro flag: #define ACPI_USE_ALTERNATE_PROTOTYPE_acpi_os_close_directory

//
// Linux wants to use designated initializers for function pointer structs.
//

// Macro flag: #define ACPI_USE_STANDARD_HEADERS

// Define/disable kernel-specific declarators

// Macro flag: #define __init

// Macro flag: #define __iomem

// Host-dependent types and defines for user-space ACPICA
// Macro flag: #define ACPI_FLUSH_CPU_CACHE()

pub const ACPI_MACHINE_WIDTH: c_int = 64;

pub const ACPI_MACHINE_WIDTH: c_int = 32;

// Macro flag: #define ACPI_USE_NATIVE_DIVIDE
// Macro flag: #define ACPI_USE_NATIVE_MATH64

// Macro flag: #define __cdecl

