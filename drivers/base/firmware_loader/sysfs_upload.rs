//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/base/firmware_loader/sysfs_upload.h
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
// enum fw_upload_prog - firmware upload progress codes
// @FW_UPLOAD_PROG_IDLE: there is no firmware upload in progress
// @FW_UPLOAD_PROG_RECEIVING: worker thread is receiving firmware data
// @FW_UPLOAD_PROG_PREPARING: target device is preparing for firmware upload
// @FW_UPLOAD_PROG_TRANSFERRING: data is being copied to the device
// @FW_UPLOAD_PROG_PROGRAMMING: device is performing the firmware update
// @FW_UPLOAD_PROG_MAX: Maximum progress code marker
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_upload_prog {
    FW_UPLOAD_PROG_IDLE,
    FW_UPLOAD_PROG_RECEIVING,
    FW_UPLOAD_PROG_PREPARING,
    FW_UPLOAD_PROG_TRANSFERRING,
    FW_UPLOAD_PROG_PROGRAMMING,
    FW_UPLOAD_PROG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_upload_priv {
    pub fw_upload: *mut fw_upload,
    pub module: *mut module,
    pub name: *const c_char,
    pub ops: *const fw_upload_ops,
    pub /: *mut *mut mutex lock; / protect data structure contents,
    pub work: work_struct,
    pub /: *const *const *const u8 data; / pointer to update data,
    pub /: *mut *mut u32 remaining_size; / size remaining to transfer,
    pub progress: fw_upload_prog,
    pub /: *mut *mut fw_upload_prog err_progress; / progress at time of failure,
    pub /: *mut *mut fw_upload_err err_code; / security manager error code,
}
