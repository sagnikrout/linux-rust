//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acpixf.h
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
// Name: acpixf.h - External interfaces to the ACPI subsystem
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Current ACPICA subsystem version in YYYYMMDD format
pub const ACPI_CA_VERSION: c_uint = 0x20260408;

//
// Macros used for ACPICA globals and configuration
//
// Ensure that global variables are defined and initialized only once.
//
// The use of these macros allows for a single list of globals (here)
// in order to simplify maintenance of the code.
//

//
// These macros configure the various ACPICA interfaces. They are
// useful for generating stub inline functions for features that are
// configured out of the current kernel or ACPICA application.
//

//
// Public globals and runtime configuration options
//
// Enable "slack mode" of the AML interpreter?  Default is FALSE, and the
// interpreter strictly follows the ACPI specification. Setting to TRUE
// allows the interpreter to ignore certain errors and/or bad AML constructs.
//
// Currently, these features are enabled by this flag:
//
// 1) Allow "implicit return" of last value in a control method
// 2) Allow access beyond the end of an operation region
// 3) Allow access to uninitialized locals/args (auto-init to integer 0)
// 4) Allow ANY object type to be a source operand for the Store() operator
// 5) Allow unresolved references (invalid target name) in package objects
// 6) Enable warning messages for behavior that is not ACPI spec compliant
//
// Automatically serialize all methods that create named objects? Default
// is TRUE, meaning that all non_serialized methods are scanned once at
// table load time to determine those that create named objects. Methods
// that create named objects are marked Serialized in order to prevent
// possible run-time problems if they are entered by more than one thread.
//
// Create the predefined _OSI method in the namespace? Default is TRUE
// because ACPICA is fully compatible with other ACPI implementations.
// Changing this will revert ACPICA (and machine ASL) to pre-OSI behavior.
//
// Optionally use default values for the ACPI register widths. Set this to
// TRUE to use the defaults, if an FADT contains incorrect widths/lengths.
//
// Whether or not to validate (map) an entire table to verify
// checksum/duplication in early stage before install. Set this to TRUE to
// allow early table validation before install it to the table manager.
// Note that enabling this option causes errors to happen in some OSPMs
// during early initialization stages. Default behavior is to allow such
// validation.
//
// Optionally enable output from the AML Debug Object.
//
// Optionally copy the entire DSDT to local memory (instead of simply
// mapping it.) There are some BIOSs that corrupt or replace the original
// DSDT, creating the need for this option. Default is FALSE, do not copy
// the DSDT.
//
// Optionally ignore an XSDT if present and use the RSDT instead.
// Although the ACPI specification requires that an XSDT be used instead
// of the RSDT, the XSDT has been found to be corrupt or ill-formed on
// some machines. Default behavior is to use the XSDT if present.
//
// Optionally use 32-bit FADT addresses if and when there is a conflict
// (address mismatch) between the 32-bit and 64-bit versions of the
// address. Although ACPICA adheres to the ACPI specification which
// requires the use of the corresponding 64-bit address if it is non-zero,
// some machines have been found to have a corrupted non-zero 64-bit
// address. Default is FALSE, do not favor the 32-bit addresses.
//
// Optionally use 32-bit FACS table addresses.
// It is reported that some platforms fail to resume from system suspending
// if 64-bit FACS table address is selected:
// https://bugzilla.kernel.org/show_bug.cgi?id=74021
// Default is TRUE, favor the 32-bit addresses.
//
// Optionally truncate I/O addresses to 16 bits. Provides compatibility
// with other ACPI implementations. NOTE: During ACPICA initialization,
// this value is set to TRUE if any Windows OSI strings have been
// requested by the BIOS.
//
// Disable runtime checking and repair of values returned by control methods.
// Use only if the repair is causing a problem on a particular machine.
//
// Optionally do not install any SSDTs from the RSDT/XSDT during initialization.
// This can be useful for debugging ACPI problems on some machines.
//
// Optionally enable runtime namespace override.
//
// We keep track of the latest version of Windows that has been requested by
// the BIOS. ACPI 5.0.
//
// ACPI 5.0 introduces the concept of a "reduced hardware platform", meaning
// that the ACPI hardware is no longer required. A flag in the FADT indicates
// a reduced HW machine, and that flag is duplicated here for convenience.
//
// ACPI Global Lock is mainly used for systems with SMM, so no-SMM systems
// (such as loong_arch) may not have and not use Global Lock.
//
// Maximum timeout for While() loop iterations before forced method abort.
// This mechanism is intended to prevent infinite loops during interpreter
// execution within a host kernel.
//
// Optionally ignore AE_NOT_FOUND errors from named reference package elements
// during DSDT/SSDT table loading. This reduces error "noise" in platforms
// whose firmware is carrying around a bunch of unused package objects that
// refer to non-existent named objects. However, If the AML actually tries to
// use such a package, the unresolved element(s) will be replaced with NULL
// elements.
//
// This mechanism is used to trace a specified AML method. The method is
// traced each time it is executed.
//
// Runtime configuration of debug output control masks. We want the debug
// switches statically initialized so they are already set when the debugger
// is entered.
//
// Optionally enable timer output with Debug Object output
//
// Debugger command handshake globals. Host OSes need to access these
// variables to implement their own command handshake mechanism.
//

//
// Other miscellaneous globals
//
// ACPICA public interface configuration.
//
// Interfaces that are configured out of the ACPICA build are replaced
// by inlined stubs by default.
//
// Hardware-reduced prototypes (default: Not hardware reduced).
//
// All ACPICA hardware-related interfaces that use these macros will be
// configured out of the ACPICA build if the ACPI_REDUCED_HARDWARE flag
// is set to TRUE.
//
// Note: This static build option for reduced hardware is intended to
// reduce ACPICA code size if desired or necessary. However, even if this
// option is not specified, the runtime behavior of ACPICA is dependent
// on the actual FADT reduced hardware flag (HW_REDUCED_ACPI). If set,
// the flag will enable similar behavior -- ACPICA will not attempt
// to access any ACPI-relate hardware (SCI, GPEs, Fixed Events, etc.)
//

//
// Error message prototypes (default: error messages enabled).
//
// All interfaces related to error and warning messages
// will be configured out of the ACPICA build if the
// ACPI_NO_ERROR_MESSAGE flag is defined.
//

//
// Debugging output prototypes (default: no debug output).
//
// All interfaces related to debug output messages
// will be configured out of the ACPICA build unless the
// ACPI_DEBUG_OUTPUT flag is defined.
//

//
// Application prototypes
//
// All interfaces used by application will be configured
// out of the ACPICA build unless the ACPI_APPLICATION
// flag is defined.
//

//
// Debugger prototypes
//
// All interfaces used by debugger will be configured
// out of the ACPICA build unless the ACPI_DEBUGGER
// flag is defined.
//

//
// ACPICA public interface prototypes
//
// Initialization
//
// initial_storage,
//
// Miscellaneous global interfaces
//
// ret_buffer))
// acpi_format_exception(acpi_status exception))
// return_buffer))
//
// ACPI table load/unload interfaces
//
// ACPI table manipulation interfaces
//
// rsdp_address))
// out_table_header))
// out_table))
//
// Namespace and name interfaces
//
// Object manipulation and enumeration
//
// parameter_objects,
// return_object_buffer))
// external_params,
// return_buffer,
// return_buffer))
//
// Handler interfaces
//
// context))
//
// Global Lock interfaces
//
// Interfaces to AML mutex objects
//
// Fixed Event interfaces
//
// event_status))
//
// General Purpose Event (GPE) Interfaces
//
// event_status))
// gpe_block_address,
//
// Resource interfaces
//
// uuid,
// ret_buffer))
// buffer,
// in_buffer))
// ret_buffer))
// resource,
// out))
// resource_ptr))
//
// Hardware (ACPI device) interfaces
//
// Sleep/Wake interfaces
//
// ACPI Timer interfaces
//
// Error/Warning output
//
// Debug output
//
extern "C" {
    pub fn acpi_initialize_debugger() -> acpi_status;
}
extern "C" {
    pub fn acpi_terminate_debugger();
}
//
// Divergences
//
extern "C" {
    pub fn acpi_set_debugger_thread_id(thread_id: acpi_thread_id);
}
