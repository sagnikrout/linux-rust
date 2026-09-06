//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bitfield.h
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
// field_{get,prep} are variants that take a non-const mask.
//
// Example:
//
// #include <linux/bitfield.h>
// #include <linux/bits.h>
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
// FIELD_MODIFY(REG_FIELD_C, &reg, c);
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
// FIELD_PREP_CONST() - prepare a constant bitfield element
// @_mask: shifted mask defining the field's length and position
// @_val:  value to put in the field
//
// FIELD_PREP_CONST() masks and shifts up the value.  The result should
// be combined with other fields of the bitfield using logical OR.
//
// Unlike FIELD_PREP() this is a constant expression and can therefore
// be used in initializers. Error checking is less comfortable for this
// version, and non-constant masks cannot be used.
//

// mask must be non-zero */				\
// check if value fits */				\
// check if mask is contiguous */			\
// and create the value */				\
//
// FIELD_GET() - extract a bitfield element
// @_mask: shifted mask defining the field's length and position
// @_reg:  value of entire bitfield
//
// FIELD_GET() extracts the field specified by @_mask from the
// bitfield passed in as @_reg by masking and shifting it down.
//

//
// FIELD_GET_SIGNED() - extract a signed bitfield element
// @mask: shifted mask defining the field's length and position
// @reg:  value of entire bitfield
//
// Returns the sign-extended field specified by @_mask from the
// bitfield passed in as @reg by masking and shifting it down.
//

//
// FIELD_MODIFY() - modify a bitfield element
// @_mask: shifted mask defining the field's length and position
// @_reg_p: pointer to the memory that should be updated
// @_val: value to store in the bitfield
//
// FIELD_MODIFY() modifies the set of bits in @_reg_p specified by @_mask,
// by replacing them with the bitfield value passed in as @_val.
//

// (_reg_p) &= ~(_mask);							\
// (_reg_p) |= (((typeof(_mask))(_val) << __bf_shf(_mask)) & (_mask));	\

// p = (*p & ~to(field)) | type##_encode_bits(val, field);	\

//
// field_prep() - prepare a bitfield element
// @mask: shifted mask defining the field's length and position, must be
// non-zero
// @val:  value to put in the field
//
// Return: field value masked and shifted to its final destination
//
// field_prep() masks and shifts up the value.  The result should be
// combined with other fields of the bitfield using logical OR.
// Unlike FIELD_PREP(), @mask is not limited to a compile-time constant.
// Typical usage patterns are a value stored in a table, or calculated by
// shifting a constant by a variable number of bits.
// If you want to ensure that @mask is a compile-time constant, please use
// FIELD_PREP() directly instead.
//

//
// field_get() - extract a bitfield element
// @mask: shifted mask defining the field's length and position, must be
// non-zero
// @reg:  value of entire bitfield
//
// Return: extracted field value
//
// field_get() extracts the field specified by @mask from the
// bitfield passed in as @reg by masking and shifting it down.
// Unlike FIELD_GET(), @mask is not limited to a compile-time constant.
// Typical usage patterns are a value stored in a table, or calculated by
// shifting a constant by a variable number of bits.
// If you want to ensure that @mask is a compile-time constant, please use
// FIELD_GET() directly instead.
//

