//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rslib.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Generic Reed Solomon encoder / decoder library
//
// Copyright (C) 2004 Thomas Gleixner (tglx@kernel.org)
//
// RS code lifted from reed solomon library written by Phil Karn
// Copyright 2002 Phil Karn, KA9Q
//

//
// struct rs_codec - rs codec data
//
// @mm:		Bits per symbol
// @nn:		Symbols per block (= (1<<mm)-1)
// @alpha_to:	log lookup table
// @index_of:	Antilog lookup table
// @genpoly:	Generator polynomial
// @nroots:	Number of generator roots = number of parity symbols
// @fcr:	First consecutive root, index form
// @prim:	Primitive element, index form
// @iprim:	prim-th root of 1, index form
// @gfpoly:	The primitive generator polynominal
// @gffunc:	Function to generate the field, if non-canonical representation
// @users:	Users of this structure
// @list:	List entry for the rs codec list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rs_codec {
    pub mm: c_int,
    pub nn: c_int,
    pub alpha_to: *mut u16,
    pub index_of: *mut u16,
    pub genpoly: *mut u16,
    pub nroots: c_int,
    pub fcr: c_int,
    pub prim: c_int,
    pub iprim: c_int,
    pub gfpoly: c_int,
    pub (*gffunc)(int): *mut c_int,
    pub users: c_int,
    pub list: list_head,
}

//
// struct rs_control - rs control structure per instance
// @codec:	The codec used for this instance
// @buffers:	Internal scratch buffers used in calls to decode_rs()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rs_control {
    pub codec: *mut rs_codec,
    pub buffers: [u16; ],
}

// General purpose RS codec, 8-bit data width, symbol width 1-15 bit

// General purpose RS codec, 16-bit data width, symbol width 1-15 bit

//
// init_rs - Create a RS control struct and initialize it
// @symsize:	the symbol size (number of bits)
// @gfpoly:	the extended Galois field generator polynomial coefficients,
// with the 0th coefficient in the low order bit. The polynomial
// must be primitive;
// @fcr:	the first consecutive root of the rs code generator polynomial
// in index form
// @prim:	primitive element to generate polynomial roots
// @nroots:	RS code generator polynomial degree (number of roots)
//
// Allocations use GFP_KERNEL.
//
extern "C" {
    pub fn init_rs_gfp(_arg: symsize, _arg: gfpoly, _arg: fcr, _arg: prim, _arg: nroots, _arg: GFP_KERNEL) -> return;
}
// Release a rs control structure
extern "C" {
    pub fn free_rs(rs: *mut rs_control);
}
// modulo replacement for galois field arithmetics
//
// @rs:	Pointer to the RS codec
// @x:		the value to reduce
//
// where
// rs->mm = number of bits per symbol
// rs->nn = (2^rs->mm) - 1
//
// Simple arithmetic modulo would return a wrong result for values
// >= 3 * rs->nn
//
