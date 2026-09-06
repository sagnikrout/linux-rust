//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/bitfield.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
//
// Efx bitfield access
//
// Efx NICs make extensive use of bitfields up to 128 bits
// wide.  Since there is no native 128-bit datatype on most systems,
// and since 64-bit datatypes are inefficient on 32-bit systems and
// vice versa, we wrap accesses in a way that uses the most efficient
// datatype.
//
// The NICs are PCI devices and therefore little-endian.  Since most
// of the quantities that we deal with are DMAed to/from host memory,
// we define our datatypes (efx_oword_t, efx_qword_t and
// efx_dword_t) to be little-endian.
//
// Lowest bit numbers and widths
pub const EFX_DUMMY_FIELD_LBN: c_int = 0;
pub const EFX_DUMMY_FIELD_WIDTH: c_int = 0;
pub const EFX_BYTE_0_LBN: c_int = 0;
pub const EFX_BYTE_0_WIDTH: c_int = 8;
pub const EFX_WORD_0_LBN: c_int = 0;
pub const EFX_WORD_0_WIDTH: c_int = 16;
pub const EFX_WORD_1_LBN: c_int = 16;
pub const EFX_WORD_1_WIDTH: c_int = 16;
pub const EFX_DWORD_0_LBN: c_int = 0;
pub const EFX_DWORD_0_WIDTH: c_int = 32;
pub const EFX_DWORD_1_LBN: c_int = 32;
pub const EFX_DWORD_1_WIDTH: c_int = 32;
pub const EFX_DWORD_2_LBN: c_int = 64;
pub const EFX_DWORD_2_WIDTH: c_int = 32;
pub const EFX_DWORD_3_LBN: c_int = 96;
pub const EFX_DWORD_3_WIDTH: c_int = 32;
pub const EFX_QWORD_0_LBN: c_int = 0;
pub const EFX_QWORD_0_WIDTH: c_int = 64;
// Specified attribute (e.g. LBN) of the specified field

// Low bit number of the specified field

// Bit width of the specified field

// High bit number of the specified field

// Mask equal in width to the specified field.
//
// For example, a field with width 5 would have a mask of 0x1f.
//
// The maximum width mask that can be generated is 64 bits.
//

// Mask equal in width to the specified field.
//
// For example, a field with width 5 would have a mask of 0x1f.
//
// The maximum width mask that can be generated is 32 bits.  Use
// EFX_MASK64 for higher width fields.
//

// A doubleword (i.e. 4 byte) datatype - little-endian in HW
// A quadword (i.e. 8 byte) datatype - little-endian in HW
// An octword (eight-word, i.e. 16 byte) datatype - little-endian in HW
// Format string and value expanders for printk

//
// Extract bit field portion [low,high) from the native-endian element
// which contains bits [min,max).
//
// For example, suppose "element" represents the high 32 bits of a
// 64-bit value, and we wish to extract the bits belonging to the bit
// field occupying bits 28-45 of this 64-bit value.
//
// Then EFX_EXTRACT ( element, 32, 63, 28, 45 ) would give
//
// ( element ) << 4
//
// The result will contain the relevant bits filled in in the range
// [0,high-low), with garbage in bits [high-low+1,...).
//

//
// Extract bit field portion [low,high) from the 64-bit little-endian
// element which contains bits [min,max)
//

//
// Extract bit field portion [low,high) from the 32-bit little-endian
// element which contains bits [min,max)
//

//
// Construct bit field portion
//
// Creates the portion of the bit field [low,high) that lies within
// the range [min,max).
//

//
// Construct bit field portion
//
// Creates the portion of the named bit field that lies within the
// range [min,max).
//

//
// Construct bit field
//
// Creates the portion of the named bit fields that lie within the
// range [min,max).
//

// Populate an octword field with various numbers of arguments

// Populate a quadword field with various numbers of arguments

// Populate a dword field with various numbers of arguments

//
// Modify a named field within an already-populated structure.  Used
// for read-modify-write operations.
//

// Used to avoid compiler warnings about shift range exceeding width
// of the data types when dma_addr_t is only 32 bits wide.
//

// Static initialiser

