//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/xilinx_hwicap/buffer_icap.h
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


//
// Author: Xilinx, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// XILINX IS PROVIDING THIS DESIGN, CODE, OR INFORMATION "AS IS"
// AS A COURTESY TO YOU, SOLELY FOR USE IN DEVELOPING PROGRAMS AND
// SOLUTIONS FOR XILINX DEVICES.  BY PROVIDING THIS DESIGN, CODE,
// OR INFORMATION AS ONE POSSIBLE IMPLEMENTATION OF THIS FEATURE,
// APPLICATION OR STANDARD, XILINX IS MAKING NO REPRESENTATION
// THAT THIS IMPLEMENTATION IS FREE FROM ANY CLAIMS OF INFRINGEMENT,
// AND YOU ARE RESPONSIBLE FOR OBTAINING ANY RIGHTS YOU MAY REQUIRE
// FOR YOUR IMPLEMENTATION.  XILINX EXPRESSLY DISCLAIMS ANY
// WARRANTY WHATSOEVER WITH RESPECT TO THE ADEQUACY OF THE
// IMPLEMENTATION, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OR
// REPRESENTATIONS THAT THIS IMPLEMENTATION IS FREE FROM CLAIMS OF
// INFRINGEMENT, IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE.
//
// (c) Copyright 2003-2008 Xilinx Inc.
// All rights reserved.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//

// Loads a partial bitstream from system memory.
extern "C" {
    pub fn buffer_icap_get_status(drvdata: *mut hwicap_drvdata) -> u32;
}
extern "C" {
    pub fn buffer_icap_reset(drvdata: *mut hwicap_drvdata);
}
