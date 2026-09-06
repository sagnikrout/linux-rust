//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/sas_ata.h
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
// Support for SATA devices on Serial Attached SCSI (SAS) controllers
//
// Copyright (C) 2006 IBM Corporation
//
// Written by: Darrick J. Wong <djwong@us.ibm.com>, IBM Corporation
//

extern "C" {
    pub fn sas_ata_schedule_reset(dev: *mut domain_device);
}
extern "C" {
    pub fn sas_ata_device_link_abort(dev: *mut domain_device, force_reset: bool);
}
extern "C" {
    pub fn sas_execute_ata_cmd(device: *mut domain_device, fis: *mut u8, force_phy_id: c_int) -> c_int;
}
extern "C" {
    pub fn smp_ata_check_ready_type(link: *mut ata_link) -> c_int;
}

