//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acmacros.h
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
// Name: acmacros.h - C macros for the entire subsystem.
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Extract data using a pointer. Any more than a byte and we
// get into potential alignment issues -- see the STORE macros below.
// Use with care.
//

//
// printf() format helper. This macro is a workaround for the difficulties
// with emitting 64-bit integers and 64-bit pointers with the same code
// for both 32-bit and 64-bit hosts.
//

//
// Macros for moving data around to/from buffers that are possibly unaligned.
// If the hardware supports the transfer of unaligned data, just do the store.
// Otherwise, we have to move one byte at a time.
//

//
// Macros for big-endian machines
//
// These macros reverse the bytes during the move, converting little-endian to big endian
// Big Endian      <==        Little Endian
// Hi...Lo                     Lo...Hi
// 16-bit source, 16/32/64 destination

// 32-bit source, 16/32/64 destination

// 64-bit source, 16/32/64 destination

//
// Macros for little-endian machines
//

// The hardware supports unaligned transfers, just do the little-endian move
// 16-bit source, 16/32/64 destination

// 32-bit source, 16/32/64 destination

// 64-bit source, 16/32/64 destination

//
// The hardware does not support unaligned transfers. We must move the
// data one byte at a time. These macros work whether the source or
// the destination (or both) is/are unaligned. (Little-endian move)
//
// 16-bit source, 16/32/64 destination

// 32-bit source, 16/32/64 destination

// 64-bit source, 16/32/64 destination

//
// Fast power-of-two math macros for non-optimized compilers
//

// Test for ASCII character

// Signed integers
pub const ACPI_SIGN_POSITIVE: c_int = 0;
pub const ACPI_SIGN_NEGATIVE: c_int = 1;
//
// Rounding macros (Power of two boundaries only)
//

// Note: sizeof(acpi_size) evaluates to either 4 or 8 (32- vs 64-bit mode)

// Generic (non-power-of-two) rounding

// Generic bit manipulation

// Generic (power-of-two) rounding

//
// Bitmask creation
// Bit positions start at zero.
// MASK_BITS_ABOVE creates a mask starting AT the position and above
// MASK_BITS_BELOW creates a mask starting one bit BELOW the position
// MASK_BITS_ABOVE/BELOW accepts a bit offset to create a mask
// MASK_BITS_ABOVE/BELOW_32/64 accepts a bit width to create a mask
// Note: The ACPI_INTEGER_BIT_SIZE check is used to bypass compiler
// differences with the shift operator
//

// Bitfields within ACPI registers

// Generic bitfield macros and masks

pub const ACPI_1BIT_MASK: c_uint = 0x00000001;
pub const ACPI_2BIT_MASK: c_uint = 0x00000003;
pub const ACPI_3BIT_MASK: c_uint = 0x00000007;
pub const ACPI_4BIT_MASK: c_uint = 0x0000000F;
pub const ACPI_5BIT_MASK: c_uint = 0x0000001F;
pub const ACPI_6BIT_MASK: c_uint = 0x0000003F;
pub const ACPI_7BIT_MASK: c_uint = 0x0000007F;
pub const ACPI_8BIT_MASK: c_uint = 0x000000FF;
pub const ACPI_16BIT_MASK: c_uint = 0x0000FFFF;
pub const ACPI_24BIT_MASK: c_uint = 0x00FFFFFF;
// Macros to extract flag bits from position zero

// Macros to extract flag bits from position one and above

// ACPI Pathname helpers

//
// An object of type struct acpi_namespace_node can appear in some contexts
// where a pointer to an object of type union acpi_operand_object can also
// appear. This macro is used to distinguish them.
//
// The "DescriptorType" field is the second field in both structures.
//

//
// Macros for the master AML opcode table
//

pub const ARG_TYPE_WIDTH: c_int = 5;

//
// Ascii error messages can be configured out
//

//
// Error reporting. The callers module and line number are inserted by AE_INFO,
// the plist contains a set of parens to allow variable-length lists.
// These macros are used for both the debug and non-debug versions of the code.
//

// No error messages

// Macro flag: #define ACPI_WARN_PREDEFINED(plist)
// Macro flag: #define ACPI_INFO_PREDEFINED(plist)
// Macro flag: #define ACPI_BIOS_ERROR_PREDEFINED(plist)
// Macro flag: #define ACPI_ERROR_ONLY(s)

//
// Macros used for ACPICA utilities only
//
// Generate a UUID

//
// Macros used for the ASL-/ASL+ converter utility
//

// Macro flag: #define ASL_CV_LABEL_FILENODE(a)
// Macro flag: #define ASL_CV_CAPTURE_COMMENTS_ONLY(a)
// Macro flag: #define ASL_CV_CAPTURE_COMMENTS(a)
// Macro flag: #define ASL_CV_TRANSFER_COMMENTS(a)

// Macro flag: #define ASL_CV_SWITCH_FILES(a,b)
// Macro flag: #define ASL_CV_CLEAR_OP_COMMENTS(a)
// Macro flag: #define ASL_CV_PRINT_ONE_COMMENT(a,b,c,d)
// Macro flag: #define ASL_CV_PRINT_ONE_COMMENT_LIST(a,b)
pub const ASL_CV_FILE_HAS_SWITCHED(a): c_int = 0;
// Macro flag: #define ASL_CV_INIT_FILETREE(a,b)

