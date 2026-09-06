//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hw_bitfield.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2025, Collabora Ltd.
//

//
// FIELD_PREP_WM16() - prepare a bitfield element with a mask in the upper half
// @_mask: shifted mask defining the field's length and position
// @_val:  value to put in the field
//
// FIELD_PREP_WM16() masks and shifts up the value, as well as bitwise ORs the
// result with the mask shifted up by 16.
//
// This is useful for a common design of hardware registers where the upper
// 16-bit half of a 32-bit register is used as a write-enable mask. In such a
// register, a bit in the lower half is only updated if the corresponding bit
// in the upper half is high.
//

//
// FIELD_PREP_WM16_CONST() - prepare a constant bitfield element with a mask in
// the upper half
// @_mask: shifted mask defining the field's length and position
// @_val:  value to put in the field
//
// FIELD_PREP_WM16_CONST() masks and shifts up the value, as well as bitwise ORs
// the result with the mask shifted up by 16.
//
// This is useful for a common design of hardware registers where the upper
// 16-bit half of a 32-bit register is used as a write-enable mask. In such a
// register, a bit in the lower half is only updated if the corresponding bit
// in the upper half is high.
//
// Unlike FIELD_PREP_WM16(), this is a constant expression and can therefore
// be used in initializers. Error checking is less comfortable for this
// version.
//

