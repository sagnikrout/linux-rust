//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_driver.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_driver {
    pub gendrv: device_driver,
    pub ): *mut *mut int (probe)(struct scsi_device,
    pub ): *mut *mut void (remove)(struct scsi_device,
    pub ): *mut *mut void (shutdown)(struct scsi_device,
    pub ): *mut *mut int (resume)(struct device,
    pub ): *mut *mut void (rescan)(struct device,
    pub ): *mut *mut blk_status_t (init_command)(struct scsi_cmnd,
    pub ): *mut *mut void (uninit_command)(struct scsi_cmnd,
    pub ): *mut *mut int (done)(struct scsi_cmnd,
    pub int): *mut *mut *mut int (eh_action)(struct scsi_cmnd ,,
    pub ): *mut *mut void (eh_reset)(struct scsi_cmnd,
}

extern "C" {
    pub fn __scsi_register_driver(: *mut scsi_driver, : *mut module) -> c_int;
}

extern "C" {
    pub fn scsi_register_interface(: *mut class_interface) -> c_int;
}

// make sure not to use it with passthrough commands
extern "C" {
    pub fn to_scsi_driver(_arg: cmd->device->sdev_gendev.driver) -> return;
}
