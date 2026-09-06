//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware.h
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

pub const FW_ACTION_NOUEVENT: c_int = 0;
pub const FW_ACTION_UEVENT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
// firmware loader private fields
    pub priv: *mut c_void,
}

//
// enum fw_upload_err - firmware upload error codes
// @FW_UPLOAD_ERR_NONE: returned to indicate success
// @FW_UPLOAD_ERR_HW_ERROR: error signalled by hardware, see kernel log
// @FW_UPLOAD_ERR_TIMEOUT: SW timed out on handshake with HW/firmware
// @FW_UPLOAD_ERR_CANCELED: upload was cancelled by the user
// @FW_UPLOAD_ERR_BUSY: there is an upload operation already in progress
// @FW_UPLOAD_ERR_INVALID_SIZE: invalid firmware image size
// @FW_UPLOAD_ERR_RW_ERROR: read or write to HW failed, see kernel log
// @FW_UPLOAD_ERR_WEAROUT: FLASH device is approaching wear-out, wait & retry
// @FW_UPLOAD_ERR_FW_INVALID: invalid firmware file
// @FW_UPLOAD_ERR_MAX: Maximum error code marker
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_upload_err {
    FW_UPLOAD_ERR_NONE,
    FW_UPLOAD_ERR_HW_ERROR,
    FW_UPLOAD_ERR_TIMEOUT,
    FW_UPLOAD_ERR_CANCELED,
    FW_UPLOAD_ERR_BUSY,
    FW_UPLOAD_ERR_INVALID_SIZE,
    FW_UPLOAD_ERR_RW_ERROR,
    FW_UPLOAD_ERR_WEAROUT,
    FW_UPLOAD_ERR_FW_INVALID,
    FW_UPLOAD_ERR_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_upload {
    pub /: *mut *mut *mut void dd_handle; / reference to parent driver,
    pub /: *mut *mut *mut void priv; / firmware loader private fields,
}

//
// struct fw_upload_ops - device specific operations to support firmware upload
// @prepare:		  Required: Prepare secure update
// @write:		  Required: The write() op receives the remaining
// size to be written and must return the actual
// size written or a negative error code. The write()
// op will be called repeatedly until all data is
// written.
// @poll_complete:	  Required: Check for the completion of the
// HW authentication/programming process.
// @cancel:		  Required: Request cancellation of update. This op
// is called from the context of a different kernel
// thread, so race conditions need to be considered.
// @cleanup:		  Optional: Complements the prepare()
// function and is called at the completion
// of the update, on success or failure, if the
// prepare function succeeded.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_upload_ops {
    pub size): *const *const u8 data, u32,
    pub written): *mut u32 size, u32,
    pub fw_upload): *mut *mut fw_upload_err (poll_complete)(struct fw_upload,
    pub fw_upload): *mut *mut void (cancel)(struct fw_upload,
    pub fw_upload): *mut *mut void (cleanup)(struct fw_upload,
}

//
// Built-in firmware functionality is only available if FW_LOADER=y, but not
// FW_LOADER=m
//

extern "C" {
    pub fn firmware_request_builtin(fw: *mut firmware, name: *const c_char) -> bool;
}

extern "C" {
    pub fn release_firmware(fw: *const firmware);
}

extern "C" {
    pub fn firmware_upload_unregister(fw_upload: *mut fw_upload);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn firmware_request_cache(device: *mut device, name: *const c_char) -> c_int;
}
