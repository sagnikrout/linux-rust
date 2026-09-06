//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acoutput.h
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
// Name: acoutput.h -- debug output
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Debug levels and component IDs. These are used to control the
// granularity of the output of the ACPI_DEBUG_PRINT macro -- on a
// per-component basis and a per-exception-type basis.
//
// Component IDs are used in the global "DebugLayer"
pub const ACPI_UTILITIES: c_uint = 0x00000001;
pub const ACPI_HARDWARE: c_uint = 0x00000002;
pub const ACPI_EVENTS: c_uint = 0x00000004;
pub const ACPI_TABLES: c_uint = 0x00000008;
pub const ACPI_NAMESPACE: c_uint = 0x00000010;
pub const ACPI_PARSER: c_uint = 0x00000020;
pub const ACPI_DISPATCHER: c_uint = 0x00000040;
pub const ACPI_EXECUTER: c_uint = 0x00000080;
pub const ACPI_RESOURCES: c_uint = 0x00000100;
pub const ACPI_CA_DEBUGGER: c_uint = 0x00000200;
pub const ACPI_OS_SERVICES: c_uint = 0x00000400;
pub const ACPI_CA_DISASSEMBLER: c_uint = 0x00000800;
// Component IDs for ACPI tools and utilities
pub const ACPI_COMPILER: c_uint = 0x00001000;
pub const ACPI_TOOLS: c_uint = 0x00002000;
pub const ACPI_EXAMPLE: c_uint = 0x00004000;
pub const ACPI_DRIVER: c_uint = 0x00008000;
pub const DT_COMPILER: c_uint = 0x00010000;
pub const ASL_PREPROCESSOR: c_uint = 0x00020000;
pub const ACPI_ALL_COMPONENTS: c_uint = 0x0001FFFF;

// Component IDs reserved for ACPI drivers
pub const ACPI_ALL_DRIVERS: c_uint = 0xFFFF0000;
//
// Raw debug output levels, do not use these in the ACPI_DEBUG_PRINT macros
//
pub const ACPI_LV_INIT: c_uint = 0x00000001;
pub const ACPI_LV_DEBUG_OBJECT: c_uint = 0x00000002;
pub const ACPI_LV_INFO: c_uint = 0x00000004;
pub const ACPI_LV_REPAIR: c_uint = 0x00000008;
pub const ACPI_LV_TRACE_POINT: c_uint = 0x00000010;
pub const ACPI_LV_ALL_EXCEPTIONS: c_uint = 0x0000001F;
// Trace verbosity level 1 [Standard Trace Level]
pub const ACPI_LV_INIT_NAMES: c_uint = 0x00000020;
pub const ACPI_LV_PARSE: c_uint = 0x00000040;
pub const ACPI_LV_LOAD: c_uint = 0x00000080;
pub const ACPI_LV_DISPATCH: c_uint = 0x00000100;
pub const ACPI_LV_EXEC: c_uint = 0x00000200;
pub const ACPI_LV_NAMES: c_uint = 0x00000400;
pub const ACPI_LV_OPREGION: c_uint = 0x00000800;
pub const ACPI_LV_BFIELD: c_uint = 0x00001000;
pub const ACPI_LV_TABLES: c_uint = 0x00002000;
pub const ACPI_LV_VALUES: c_uint = 0x00004000;
pub const ACPI_LV_OBJECTS: c_uint = 0x00008000;
pub const ACPI_LV_RESOURCES: c_uint = 0x00010000;
pub const ACPI_LV_USER_REQUESTS: c_uint = 0x00020000;
pub const ACPI_LV_PACKAGE: c_uint = 0x00040000;
pub const ACPI_LV_EVALUATION: c_uint = 0x00080000;
pub const ACPI_LV_VERBOSITY1: c_uint = 0x000FFF40 | ACPI_LV_ALL_EXCEPTIONS;
// Trace verbosity level 2 [Function tracing and memory allocation]
pub const ACPI_LV_ALLOCATIONS: c_uint = 0x00100000;
pub const ACPI_LV_FUNCTIONS: c_uint = 0x00200000;
pub const ACPI_LV_OPTIMIZATIONS: c_uint = 0x00400000;
pub const ACPI_LV_PARSE_TREES: c_uint = 0x00800000;
pub const ACPI_LV_VERBOSITY2: c_uint = 0x00F00000 | ACPI_LV_VERBOSITY1;

// Trace verbosity level 3 [Threading, I/O, and Interrupts]
pub const ACPI_LV_MUTEX: c_uint = 0x01000000;
pub const ACPI_LV_THREADS: c_uint = 0x02000000;
pub const ACPI_LV_IO: c_uint = 0x04000000;
pub const ACPI_LV_INTERRUPTS: c_uint = 0x08000000;
pub const ACPI_LV_VERBOSITY3: c_uint = 0x0F000000 | ACPI_LV_VERBOSITY2;
// Exceptionally verbose output -- also used in the global "DebugLevel"
pub const ACPI_LV_AML_DISASSEMBLE: c_uint = 0x10000000;
pub const ACPI_LV_VERBOSE_INFO: c_uint = 0x20000000;
pub const ACPI_LV_FULL_TABLES: c_uint = 0x40000000;
pub const ACPI_LV_EVENTS: c_uint = 0x80000000;
pub const ACPI_LV_VERBOSE: c_uint = 0xF0000000;
//
// Debug level macros that are used in the DEBUG_PRINT macros
//

//
// Exception level -- used in the global "DebugLevel"
//
// Note: For errors, use the ACPI_ERROR or ACPI_EXCEPTION interfaces.
// For warnings, use ACPI_WARNING.
//

// Trace level -- also used in the global "DebugLevel"

// Defaults for debug_level, debug and normal

//
// Global trace flags
//

// Defaults for trace debugging level/layer

pub const ACPI_TRACE_LAYER_ALL: c_uint = 0x000001FF;

//
// The module name is used primarily for error and debug messages.
// The __FILE__ macro is not very useful for this, because it
// usually includes the entire pathname to the module making the
// debug output difficult to read.
//

//
// For the no-debug and no-error-msg cases, we must at least define
// a null module name.
//
// Macro flag: #define ACPI_MODULE_NAME(name)

//
// Ascii error messages can be configured out
//

//
// Error reporting. Callers module and line number are inserted by AE_INFO,
// the plist contains a set of parens to allow variable-length lists.
// These macros are used for both the debug and non-debug versions of the code.
//

// No error messages
// Macro flag: #define ACPI_INFO(plist)
// Macro flag: #define ACPI_WARNING(plist)
// Macro flag: #define ACPI_WARNING_ONCE(plist)
// Macro flag: #define ACPI_EXCEPTION(plist)
// Macro flag: #define ACPI_ERROR(plist)
// Macro flag: #define ACPI_ERROR_ONCE(plist)
// Macro flag: #define ACPI_BIOS_WARNING(plist)
// Macro flag: #define ACPI_BIOS_EXCEPTION(plist)
// Macro flag: #define ACPI_BIOS_ERROR(plist)
// Macro flag: #define ACPI_DEBUG_OBJECT(obj,l,i)

//
// Debug macros that are conditionally compiled
//

//
// If ACPI_GET_FUNCTION_NAME was not defined in the compiler-dependent header,
// define it now. This is the case where there the compiler does not support
// a __func__ macro or equivalent.
//

//
// The Name parameter should be the procedure name as a non-quoted string.
// The function name is also used by the function exit macros below.
// Note: (const char) is used to be compatible with the debug interfaces
// and macros such as __func__.
//

// Compiler supports __func__ (or equivalent) -- Ignore this macro
// Macro flag: #define ACPI_FUNCTION_NAME(name)

//
// Common parameters used for debug output functions:
// line number, function name, module(file) name, component ID
//

// Check if debug output is currently dynamically enabled

//
// Master debug print macros
// Print message if and only if:
// 1) Debug print for the current component is enabled
// 2) Debug error level or trace level for the print statement is enabled
//
// November 2012: Moved the runtime check for whether to actually emit the
// debug message outside of the print function itself. This improves overall
// performance at a relatively small code cost. Implementation involves the
// use of variadic macros supported by C99.
//
// Note: the ACPI_DO_WHILE0 macro is used to prevent some compilers from
// complaining about these constructs. On other compilers the do...while
// adds some extra code, so this feature is optional.
//

// DEBUG_PRINT functions

// Helper macros for DEBUG_PRINT

//
// Function entry tracing
//
// The name of the function is emitted as a local variable that is
// intended to be used by both the entry trace and the exit trace.
//
// Helper macro

// The actual entry trace macros

//
// Function exit tracing
//
// These macros include a return statement. This is usually considered
// bad form, but having a separate exit macro before the actual return
// is very ugly and difficult to maintain.
//
// One of the FUNCTION_TRACE macros above must be used in conjunction
// with these macros so that "_AcpiFunctionName" is defined.
//
// There are two versions of most of the return macros. The default version is
// safer, since it avoids side-effects by guaranteeing that the argument will
// not be evaluated twice.
//
// A less-safe version of the macros is provided for optional use if the
// compiler uses excessive CPU stack (for example, this may happen in the
// debug case if code optimization is disabled.)
//
// Exit trace helper macro

// The actual exit macros

// Conditional execution

// Various object display routines for debug

//
// This is the non-debug case -- make everything go away,
// leaving no executable debug code!
//
// Macro flag: #define ACPI_DEBUG_PRINT(pl)
// Macro flag: #define ACPI_DEBUG_PRINT_RAW(pl)
// Macro flag: #define ACPI_DEBUG_EXEC(a)
// Macro flag: #define ACPI_DEBUG_ONLY_MEMBERS(a)
// Macro flag: #define ACPI_FUNCTION_NAME(a)
// Macro flag: #define ACPI_FUNCTION_TRACE(a)

// Macro flag: #define ACPI_FUNCTION_ENTRY()
// Macro flag: #define ACPI_DUMP_STACK_ENTRY(a)

// Return macros must have a return statement at the minimum

