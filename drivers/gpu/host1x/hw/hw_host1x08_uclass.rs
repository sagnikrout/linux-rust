//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/hw/hw_host1x08_uclass.h
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
// Copyright (c) 2018 NVIDIA Corporation.
//
// Function naming determines intended use:
//
// <x>_r(void) : Returns the offset for register <x>.
//
// <x>_w(void) : Returns the word offset for word (4 byte) element <x>.
//
// <x>_<y>_s(void) : Returns size of field <y> of register <x> in bits.
//
// <x>_<y>_f(u32 v) : Returns a value based on 'v' which has been shifted
// and masked to place it at field <y> of register <x>.  This value
// can be |'d with others to produce a full register value for
// register <x>.
//
// <x>_<y>_m(void) : Returns a mask for field <y> of register <x>.  This
// value can be ~'d and then &'d to clear the value of field <y> for
// register <x>.
//
// <x>_<y>_<z>_f(void) : Returns the constant value <z> after being shifted
// to place it at field <y> of register <x>.  This value can be |'d
// with others to produce a full register value for <x>.
//
// <x>_<y>_v(u32 r) : Returns the value of field <y> from a full register
// <x> value 'r' after being shifted to place its LSB at bit 0.
// This value is suitable for direct comparison with other unshifted
// values appropriate for use in field <y> of register <x>.
//
// <x>_<y>_<z>_v(void) : Returns the constant value for <z> defined for
// field <y> of register <x>.  This value is suitable for direct
// comparison with unshifted values appropriate for use in field <y>
// of register <x>.
//

