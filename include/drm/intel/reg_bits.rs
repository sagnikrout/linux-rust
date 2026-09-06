//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/reg_bits.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2026 Intel Corporation

//
// Wrappers over the generic fixed width BIT_U*() and GENMASK_U*()
// implementations, for compatibility reasons with previous implementation.
//

//
// Local integer constant expression version of is_power_of_2().
//

//
// REG_FIELD_PREP8() - Prepare a u8 bitfield value
// @__mask: shifted mask defining the field's length and position
// @__val: value to put in the field
//
// Local copy of FIELD_PREP() to generate an integer constant expression, force
// u8 and for consistency with REG_FIELD_GET8(), REG_BIT8() and REG_GENMASK8().
//
// @return: @__val masked and shifted into the field defined by @__mask.
//

//
// REG_FIELD_PREP16() - Prepare a u16 bitfield value
// @__mask: shifted mask defining the field's length and position
// @__val: value to put in the field
//
// Local copy of FIELD_PREP16() to generate an integer constant
// expression, force u8 and for consistency with
// REG_FIELD_GET16(), REG_BIT16() and REG_GENMASK16().
//
// @return: @__val masked and shifted into the field defined by @__mask.
//

//
// REG_FIELD_PREP() - Prepare a u32 bitfield value
// @__mask: shifted mask defining the field's length and position
// @__val: value to put in the field
//
// Local copy of FIELD_PREP() to generate an integer constant expression, force
// u32 and for consistency with REG_FIELD_GET(), REG_BIT() and REG_GENMASK().
//
// @return: @__val masked and shifted into the field defined by @__mask.
//

//
// REG_FIELD_GET8() - Extract a u8 bitfield value
// @__mask: shifted mask defining the field's length and position
// @__val: value to extract the bitfield value from
//
// Local wrapper for FIELD_GET() to force u8 and for consistency with
// REG_FIELD_PREP(), REG_BIT() and REG_GENMASK().
//
// @return: Masked and shifted value of the field defined by @__mask in @__val.
//

//
// REG_FIELD_GET() - Extract a u32 bitfield value
// @__mask: shifted mask defining the field's length and position
// @__val: value to extract the bitfield value from
//
// Local wrapper for FIELD_GET() to force u32 and for consistency with
// REG_FIELD_PREP(), REG_BIT() and REG_GENMASK().
//
// @return: Masked and shifted value of the field defined by @__mask in @__val.
//

//
// REG_FIELD_GET64() - Extract a u64 bitfield value
// @__mask: shifted mask defining the field's length and position
// @__val: value to extract the bitfield value from
//
// Local wrapper for FIELD_GET() to force u64 and for consistency with
// REG_GENMASK64().
//
// @return: Masked and shifted value of the field defined by @__mask in @__val.
//

//
// REG_FIELD_MAX() - produce the maximum value representable by a field
// @__mask: shifted mask defining the field's length and position
//
// Local wrapper for FIELD_MAX() to return the maximum bit value that can
// be held in the field specified by @_mask, cast to u32 for consistency
// with other macros.
//

