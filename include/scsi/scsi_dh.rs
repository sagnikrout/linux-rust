//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_dh.h
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
// Header file for SCSI device handler infrastructure.
//
// Modified version of patches posted by Mike Christie <michaelc@cs.wisc.edu>
//
// Copyright IBM Corporation, 2007
// Authors:
// Chandra Seetharaman <sekharan@us.ibm.com>
// Mike Anderson <andmike@linux.vnet.ibm.com>
//

//
// device errors
//
// transport errors
//
// driver and generic errors
//
extern "C" {
    pub fn void(: *mut *mut activate_complete)(void, _arg: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_device_handler {
// Used by the infrastructure
    pub /: *mut *mut list_head list; / list of scsi_device_handlers,
// Filled by the hardware handler
    pub module: *mut module,
    pub name: *const c_char,
    pub ): *mut scsi_sense_hdr,
    pub ): *mut *mut int (attach)(struct scsi_device,
    pub ): *mut *mut void (detach)(struct scsi_device,
    pub ): *mut *mut *mut int (activate)(struct scsi_device , activate_complete, void,
    pub ): *mut *mut *mut blk_status_t (prep_fn)(struct scsi_device , struct request,
    pub ): *const *const *const int (set_params)(struct scsi_device , char,
    pub ): *mut *mut void (rescan)(struct scsi_device,
}

extern "C" {
    pub fn scsi_dh_activate(: *mut request_queue, _arg: activate_complete, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn scsi_dh_attach(: *mut request_queue, : *const c_char) -> c_int;
}
extern "C" {
    pub fn scsi_dh_set_params(: *mut request_queue, : *const c_char) -> c_int;
}

