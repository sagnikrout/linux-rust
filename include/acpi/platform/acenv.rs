//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/platform/acenv.h
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
// Name: acenv.h - Host and compiler configuration
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Environment configuration. The purpose of this file is to interface ACPICA
// to the local environment. This includes compiler-specific, OS-specific,
// and machine-specific configuration.
//
// Types for ACPI_MUTEX_TYPE
pub const ACPI_BINARY_SEMAPHORE: c_int = 0;
pub const ACPI_OSL_MUTEX: c_int = 1;
// Types for DEBUGGER_THREADING
pub const DEBUGGER_SINGLE_THREADED: c_int = 0;
pub const DEBUGGER_MULTI_THREADED: c_int = 1;
//
// Configuration for ACPI tools and utilities
//
// Common application configuration. All single threaded except for acpi_exec.

// Macro flag: #define ACPI_APPLICATION
// Macro flag: #define ACPI_SINGLE_THREADED
// Macro flag: #define USE_NATIVE_ALLOCATE_ZEROED

// iASL configuration

// Macro flag: #define ACPI_DEBUG_OUTPUT
// Macro flag: #define ACPI_CONSTANT_EVAL_ONLY
// Macro flag: #define ACPI_LARGE_NAMESPACE_NODE
// Macro flag: #define ACPI_DATA_TABLE_DISASSEMBLY
// Macro flag: #define ACPI_32BIT_PHYSICAL_ADDRESS
pub const ACPI_DISASSEMBLER: c_int = 1;

// acpi_exec configuration. Multithreaded with full AML debugger

// Macro flag: #define ACPI_APPLICATION
// Macro flag: #define ACPI_FULL_DEBUG
// Macro flag: #define ACPI_MUTEX_DEBUG
// Macro flag: #define ACPI_DBG_TRACK_ALLOCATIONS

// acpi_help configuration. Error messages disabled.

// Macro flag: #define ACPI_NO_ERROR_MESSAGES

// acpi_names configuration. Debug output enabled.

// Macro flag: #define ACPI_DEBUG_OUTPUT

// acpi_exec/acpi_names/Example configuration. Native RSDP used.

// Macro flag: #define ACPI_USE_NATIVE_RSDP_POINTER

// acpi_dump configuration. Native mapping used if provided by the host

// Macro flag: #define ACPI_USE_NATIVE_MEMORY_MAPPING

// acpi_names/Example configuration. Hardware disabled

pub const ACPI_REDUCED_HARDWARE: c_int = 1;

// Linkable ACPICA library. Two versions, one with full debug.

// Macro flag: #define ACPI_USE_LOCAL_CACHE
pub const ACPI_DEBUGGER: c_int = 1;
pub const ACPI_DISASSEMBLER: c_int = 1;

// Macro flag: #define ACPI_DEBUG_OUTPUT

// Common for all ACPICA applications

// Macro flag: #define ACPI_USE_LOCAL_CACHE

// Common debug/disassembler support

// Macro flag: #define ACPI_DEBUG_OUTPUT
pub const ACPI_DEBUGGER: c_int = 1;
pub const ACPI_DISASSEMBLER: c_int = 1;

//
// acpisrc CR\LF support
// Unix file line endings do not include the carriage return.
// If the acpisrc utility is being built using a microsoft compiler, it means
// that it will be running on a windows machine which means that the output is
// expected to have CR/LF newlines. If the acpisrc utility is built with
// anything else, it will likely run on a system with LF newlines. This flag
// tells the acpisrc utility that newlines will be in the LF format.
//
pub const ACPI_SRC_OS_LF_ONLY: c_int = 0;
// ! [Begin] no source code translation
//
// Host configuration files. The compiler configuration files are included
// first.
//

//
// EFI applications can be built with -nostdlib, in this case, it must be
// included after including all other host environmental definitions, in
// order to override the definitions.
//

// Unknown environment

// ! [End] no source code translation !
//
// Setup defaults for the required symbols that were not defined in one of
// the host/compiler files above.
//
// 64-bit data types

// Type of mutex supported by host. Default is binary semaphores.

// Global Lock acquire/release

// NULL/invalid value to use for destroyed or not-yet-created semaphores.

// Flush CPU cache - used when going to sleep. Wbinvd or similar.

// Macro flag: #define ACPI_FLUSH_CPU_CACHE()

// "inline" keywords - configurable since inline is not standardized

// Macro flag: #define ACPI_INLINE

// Use ordered initialization if compiler doesn't support designated.

//
// Configurable calling conventions:
//
// ACPI_SYSTEM_XFACE        - Interfaces to host OS (handlers, threads)
// ACPI_EXTERNAL_XFACE      - External ACPI interfaces
// ACPI_INTERNAL_XFACE      - Internal ACPI interfaces
// ACPI_INTERNAL_VAR_XFACE  - Internal variable-parameter list interfaces
//

// Macro flag: #define ACPI_SYSTEM_XFACE

// Macro flag: #define ACPI_EXTERNAL_XFACE

// Macro flag: #define ACPI_INTERNAL_XFACE

// Macro flag: #define ACPI_INTERNAL_VAR_XFACE

//
// Debugger threading model
// Use single threaded if the entire subsystem is contained in an application
// Use multiple threaded when the subsystem is running in the kernel.
//
// By default the model is single threaded if ACPI_APPLICATION is set,
// multi-threaded if ACPI_APPLICATION is not set.
//

//
// C library configuration
//
// ACPI_USE_SYSTEM_CLIBRARY - Define this if linking to an actual C library.
// Otherwise, local versions of string/memory functions will be used.
// ACPI_USE_STANDARD_HEADERS - Define this if linking to a C library and
// the standard header files may be used. Defining this implies that
// ACPI_USE_SYSTEM_CLIBRARY has been defined.
//
// The ACPICA subsystem only uses low level C library functions that do not
// call operating system services and may therefore be inlined in the code.
//
// It may be necessary to tailor these include files to the target
// generation environment.
//
// Use the standard C library headers. We want to keep these to a minimum.

// Use the standard headers from the standard locations

// Macro flag: #define ACPI_INIT_FUNCTION

