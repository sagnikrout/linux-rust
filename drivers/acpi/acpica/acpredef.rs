//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acpredef.h
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
// Name: acpredef - Information table for ACPI predefined methods and objects
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Return Package types
//
// 1) PTYPE1 packages do not contain subpackages.
//
// ACPI_PTYPE1_FIXED: Fixed-length length, 1 or 2 object types:
// object type
// count
// object type
// count
//
// ACPI_PTYPE1_VAR: Variable-length length. Zero-length package is allowed:
// object type (Int/Buf/Ref)
//
// ACPI_PTYPE1_OPTION: Package has some required and some optional elements
// (Used for _PRW)
//
// 2) PTYPE2 packages contain a Variable-length number of subpackages. Each
// of the different types describe the contents of each of the subpackages.
//
// ACPI_PTYPE2: Each subpackage contains 1 or 2 object types. Zero-length
// parent package is allowed:
// object type
// count
// object type
// count
// (Used for _ALR,_MLS,_PSS,_TRT,_TSS)
//
// ACPI_PTYPE2_COUNT: Each subpackage has a count as first element.
// Zero-length parent package is allowed:
// object type
// (Used for _CSD,_PSD,_TSD)
//
// ACPI_PTYPE2_PKG_COUNT: Count of subpackages at start, 1 or 2 object types:
// object type
// count
// object type
// count
// (Used for _CST)
//
// ACPI_PTYPE2_FIXED: Each subpackage is of Fixed-length. Zero-length
// parent package is allowed.
// (Used for _PRT)
//
// ACPI_PTYPE2_MIN: Each subpackage has a Variable-length but minimum length.
// Zero-length parent package is allowed:
// (Used for _HPX)
//
// ACPI_PTYPE2_REV_FIXED: Revision at start, each subpackage is Fixed-length
// (Used for _ART, _FPS)
//
// ACPI_PTYPE2_FIX_VAR: Each subpackage consists of some fixed-length elements
// followed by an optional element. Zero-length parent package is allowed.
// object type
// count
// object type
// count = 0 (optional)
// (Used for _DLM)
//
// ACPI_PTYPE2_VAR_VAR: Variable number of subpackages, each of either a
// constant or variable length. The subpackages are preceded by a
// constant number of objects.
// (Used for _LPI, _RDI)
//
// ACPI_PTYPE2_UUID_PAIR: Each subpackage is preceded by a UUID Buffer. The UUID
// defines the format of the package. Zero-length parent package is
// allowed.
// (Used for _DSD)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_return_package_types {
    ACPI_PTYPE1_FIXED = 1,
    ACPI_PTYPE1_VAR = 2,
    ACPI_PTYPE1_OPTION = 3,
    ACPI_PTYPE2 = 4,
    ACPI_PTYPE2_COUNT = 5,
    ACPI_PTYPE2_PKG_COUNT = 6,
    ACPI_PTYPE2_FIXED = 7,
    ACPI_PTYPE2_MIN = 8,
    ACPI_PTYPE2_REV_FIXED = 9,
    ACPI_PTYPE2_FIX_VAR = 10,
    ACPI_PTYPE2_VAR_VAR = 11,
    ACPI_PTYPE2_UUID_PAIR = 12,
    ACPI_PTYPE_CUSTOM = 13
}

// Support macros for users of the predefined info table
pub const METHOD_PREDEF_ARGS_MAX: c_int = 5;
pub const METHOD_ARG_BIT_WIDTH: c_int = 3;
pub const METHOD_ARG_MASK: c_uint = 0x0007;
pub const ARG_COUNT_IS_MINIMUM: c_uint = 0x8000;

// Macros used to build the predefined info table
pub const METHOD_0ARGS: c_int = 0;

pub const METHOD_NO_RETURN_VALUE: c_int = 0;

// Support macros for the resource descriptor info table
pub const WIDTH_1: c_uint = 0x0001;
pub const WIDTH_2: c_uint = 0x0002;
pub const WIDTH_3: c_uint = 0x0004;
pub const WIDTH_8: c_uint = 0x0008;
pub const WIDTH_16: c_uint = 0x0010;
pub const WIDTH_32: c_uint = 0x0020;
pub const WIDTH_64: c_uint = 0x0040;
pub const VARIABLE_DATA: c_uint = 0x0080;
pub const NUM_RESOURCE_WIDTHS: c_int = 8;

//
// Predefined method/object information table.
//
// These are the names that can actually be evaluated via acpi_evaluate_object.
// Not present in this table are the following:
//
// 1) Predefined/Reserved names that are not usually evaluated via
// acpi_evaluate_object:
// _Lxx and _Exx GPE methods
// _Qxx EC methods
// _T_x compiler temporary variables
// _Wxx wake events
//
// 2) Predefined names that never actually exist within the AML code:
// Predefined resource descriptor field names
//
// 3) Predefined names that are implemented within ACPICA:
// _OSI
//
// The main entries in the table each contain the following items:
//
// name                 - The ACPI reserved name
// argument_list        - Contains (in 16 bits), the number of required
// arguments to the method (3 bits), and a 3-bit type
// field for each argument (up to 4 arguments). The
// METHOD_?ARGS macros generate the correct packed data.
// expected_btypes      - Allowed type(s) for the return value.
// 0 means that no return value is expected.
//
// For methods that return packages, the next entry in the table contains
// information about the expected structure of the package. This information
// is saved here (rather than in a separate table) in order to minimize the
// overall size of the stored data.
//
// Note: The additional braces are intended to promote portability.
//
// Note2: Table is used by the kernel-resident subsystem, the iASL compiler,
// and the acpi_help utility.
//
// TBD: _PRT - currently ignore reversed entries. Attempt to fix in nsrepair.
// Possibly fixing package elements like _BIF, etc.
//
// For _HPX, a single package is returned, containing a variable-length number
// of subpackages. Each subpackage contains a PCI record setting.
// There are several different type of record settings, of different
// lengths, but all elements of all settings are Integers.
//
// For _PRT, many BIOSs reverse the 3rd and 4th Package elements (Source
// and source_index). This bug is so prevalent that there is code in the
// ACPICA Resource Manager to detect this and switch them back. For now,
// do not allow and issue a warning. To allow this and eliminate the
// warning, add the ACPI_RTYPE_REFERENCE type to the 4th element (index 3)
// in the statement below.
//
// For _S0_ through _S5_, the ACPI spec defines a return Package
// containing 1 Integer, but most DSDTs have it wrong - 2,3, or 4 integers.
// Allow this by making the objects "Variable-length length", but all elements
// must be Integers.
//
// Acpi 1.0 defined _WAK with no return value. Later, it was changed to return a package
// _WDG/_WED are MS extensions defined by "Windows Instrumentation"

//
// Predefined names for use in Resource Descriptors. These names do not
// appear in the global Predefined Name table (since these names never
// appear in actual AML byte code, only in the original ASL)
//
// Note: Used by iASL compiler and acpi_help utility only.
//

