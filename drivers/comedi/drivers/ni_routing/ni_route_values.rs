//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_routing/ni_route_values.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// comedi/drivers/ni_routing/ni_route_values.h
// Route information for NI boards.
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

//
// This file includes the tables that are a list of all the values of various
// signals routes available on NI hardware.  In many cases, one does not
// explicitly make these routes, rather one might indicate that something is
// used as the source of one particular trigger or another (using
// *_src=TRIG_EXT).
//
// This file is meant to be included by comedi/drivers/ni_routes.c
//

// Marks a register value as valid, implemented, and tested.

// Marks a register value as implemented but needing testing.

// Marks a register value as not implemented.
pub const U(x): c_uint = 0x0;
pub type register_type = u8;

// Marks a register value as implemented but needing testing.

// Marks a register value as not implemented.

// Tests whether a register is marked as valid/implemented/tested

// Tests whether a register is implemented but not tested

// Tests whether a register is not implemented

// need more space to store extra marks
pub type register_type = u16;

// Mask out the marking bit(s).

//
// Gi_SRC(x,1) implements Gi_Src_SubSelect = 1
//
// This appears to only really be a valid MUX for m-series devices.
//

//
// struct family_route_values - Register values for all routes for a particular
// family.
// @family: lower-case string representation of a specific series or family of
// devices from National Instruments where each member of this family
// shares the same register values for the various signal MUXes.  It
// should be noted that not all devices of any family have access to
// all routes defined.
// @register_values: Table of all register values for various signal MUXes on
// National Instruments devices.  The first index of this table is the
// signal destination (i.e. identification of the signal MUX).  The
// second index of this table is the signal source (i.e. input of the
// signal MUX).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct family_route_values {
    pub family: *const c_char,
    pub register_values: [register_type; NI_NUM_NAMES][NI_NUM_NAMES],
}
