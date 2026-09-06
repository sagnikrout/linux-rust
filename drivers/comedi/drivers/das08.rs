//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/das08.h
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
// das08.h
//
// Header for common DAS08 support (used by ISA/PCI/PCMCIA drivers)
//
// Copyright (C) 2003 Frank Mori Hess <fmhess@users.sourceforge.net>
//

// different ways ai data is encoded in first two registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum das08_ai_encoding {
// types of ai range table used by different boards
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum das08_lrange {
    das08_pg_none, das08_bipolar5, das08_pgh, das08_pgl, das08_pgm
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct das08_board_struct {
    pub name: *const c_char,
    pub /: *mut *mut bool is_jr; / true for 'JR' boards,
    pub ai_nbits: c_uint,
    pub ai_pg: das08_lrange,
    pub ai_encoding: das08_ai_encoding,
    pub ao_nbits: c_uint,
    pub di_nchan: c_uint,
    pub do_nchan: c_uint,
    pub i8255_offset: c_uint,
    pub i8254_offset: c_uint,
    pub /: *mut *mut unsigned int iosize; / number of ioports used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct das08_private_struct {
// bits for do/mux register on boards without separate do register
    pub do_mux_bits: c_uint,
    pub pg_gainlist: *const c_uint,
}

extern "C" {
    pub fn das08_common_attach(dev: *mut comedi_device, iobase: c_ulong) -> c_int;
}
