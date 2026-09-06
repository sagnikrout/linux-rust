//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/altera-stapl/altera-exprt.h
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
// altera-exprt.h
//
// altera FPGA driver
//
// Copyright (C) Altera Corporation 1998-2001
// Copyright (C) 2010 NetUP Inc.
// Copyright (C) 2010 Igor M. Liplianin <liplianin@netup.ru>
//
extern "C" {
    pub fn altera_shrink(in: *mut u8, in_length: u32, out: *mut u8, out_length: u32, version: i32) -> u32;
}
extern "C" {
    pub fn netup_jtag_io_lpt(device: *mut c_void, tms: c_int, tdi: c_int, read_tdo: c_int) -> c_int;
}
