//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mvme147.h
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
// $Id: mvme147.h,v 1.4 1997/01/19 23:07:10 davem Exp $
//
// Header file for the MVME147 built-in SCSI controller for Linux
//
// Written and (C) 1993, Hamish Macdonald, see mvme147.c for more info
//

extern "C" {
    pub fn mvme147_detect(: *mut scsi_host_template) -> c_int;
}
extern "C" {
    pub fn mvme147_release(: *mut Scsi_Host) -> c_int;
}

pub const CMD_PER_LUN: c_int = 2;

pub const CAN_QUEUE: c_int = 16;

