//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/bitfield.h
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
// Copyright (C) 2014 Felix Fietkau <nbd@nbd.name>
// Copyright (C) 2004 - 2009 Ivo van Doorn <IvDoorn@gmail.com>
//

//
// Bitfield access macros
//
// FIELD_{GET,PREP} macros take as first parameter shifted mask
// from which they extract the base mask and shift amount.
// Mask must be a compilation time constant.
//
// Example:
//
// #define REG_FIELD_A  GENMASK(6, 0)
// #define REG_FIELD_B  BIT(7)
// #define REG_FIELD_C  GENMASK(15, 8)
// #define REG_FIELD_D  GENMASK(31, 16)
//
// Get:
// a = FIELD_GET(REG_FIELD_A, reg);
// b = FIELD_GET(REG_FIELD_B, reg);
//
// Set:
// reg = FIELD_PREP(REG_FIELD_A, 1) |
// FIELD_PREP(REG_FIELD_B, 0) |
// FIELD_PREP(REG_FIELD_C, c) |
// FIELD_PREP(REG_FIELD_D, 0x40);
//
// Modify:
// reg &= ~REG_FIELD_C;
// reg |= FIELD_PREP(REG_FIELD_C, c);
//

//
// FIELD_MAX() - produce the maximum value representable by a field
// @_mask: shifted mask defining the field's length and position
//
// FIELD_MAX() returns the maximum value that can be held in the field
// specified by @_mask.
//

//
// FIELD_FIT() - check if value fits in the field
// @_mask: shifted mask defining the field's length and position
// @_val:  value to test against the field
//
// Return: true if @_val can fit inside @_mask, false if @_val is too big.
//

//
// FIELD_PREP() - prepare a bitfield element
// @_mask: shifted mask defining the field's length and position
// @_val:  value to put in the field
//
// FIELD_PREP() masks and shifts up the value.  The result should
// be combined with other fields of the bitfield using logical OR.
//

//
// FIELD_GET() - extract a bitfield element
// @_mask: shifted mask defining the field's length and position
// @_reg:  value of entire bitfield
//
// FIELD_GET() extracts the field specified by @_mask from the
// bitfield passed in as @_reg by masking and shifting it down.
//

// p = (*p & ~to(field)) | type##_encode_bits(val, field);	\

