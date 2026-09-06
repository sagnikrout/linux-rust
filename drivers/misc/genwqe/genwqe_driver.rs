//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/genwqe/genwqe_driver.h
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
// IBM Accelerator Family 'GenWQE'
//
// (C) Copyright IBM Corp. 2013
//
// Author: Frank Haverkamp <haver@linux.vnet.ibm.com>
// Author: Joerg-Stephan Vogt <jsvogt@de.ibm.com>
// Author: Michael Jung <mijung@gmx.net>
// Author: Michael Ruettger <michael@ibmra.de>
//

//
// Static minor number assignement, until we decide/implement
// something dynamic.
//

//
// genwqe_requ_alloc() - Allocate a new DDCB execution request
//
// This data structure contains the user visiable fields of the DDCB
// to be executed.
//
// Return: ptr to genwqe_ddcb_cmd data structure
//
// ddcb_requ_free() - Free DDCB execution request.
// @req:       ptr to genwqe_ddcb_cmd data structure.
//
extern "C" {
    pub fn ddcb_requ_free(req: *mut genwqe_ddcb_cmd);
}
extern "C" {
    pub fn genwqe_crc32(buff: *mut u8, len: usize, init: u32) -> u32;
}
