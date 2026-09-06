//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/speed_select_if/isst_if_common.h
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
// Intel Speed Select Interface: Drivers Internal defines
// Copyright (c) 2019, Intel Corporation.
// All rights reserved.
//
// Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>
//
pub const PCI_DEVICE_ID_INTEL_RAPL_PRIO_DEVID_0: c_uint = 0x3451;
pub const PCI_DEVICE_ID_INTEL_CFG_MBOX_DEVID_0: c_uint = 0x3459;
pub const PCI_DEVICE_ID_INTEL_RAPL_PRIO_DEVID_1: c_uint = 0x3251;
pub const PCI_DEVICE_ID_INTEL_CFG_MBOX_DEVID_1: c_uint = 0x3259;
pub const MSR_OS_MAILBOX_INTERFACE: c_uint = 0xB0;
pub const MSR_OS_MAILBOX_DATA: c_uint = 0xB1;
//
// Validate maximum commands in a single request.
// This is enough to handle command to every core in one ioctl, or all
// possible message id to one CPU. Limit is also helpful for resonse time
// per IOCTL request, as PUNIT may take different times to process each
// request and may hold for long for too many commands.
//
pub const ISST_IF_CMD_LIMIT: c_int = 64;
pub const ISST_IF_API_VERSION: c_uint = 0x01;
pub const ISST_IF_DRIVER_VERSION: c_uint = 0x01;
pub const ISST_IF_DEV_MBOX: c_int = 0;
pub const ISST_IF_DEV_MMIO: c_int = 1;
pub const ISST_IF_DEV_TPMI: c_int = 2;
pub const ISST_IF_DEV_MAX: c_int = 3;
//
// struct isst_if_cmd_cb - Used to register a IOCTL handler
// @registered:	Used by the common code to store registry. Caller don't
// to touch this field
// @cmd_size:	The command size of the individual command in IOCTL
// @offset:	Offset to the first valid member in command structure.
// This will be the offset of the start of the command
// after command count field
// @api_version: API version supported for this target. 0, if none.
// @owner:	Registered module owner
// @cmd_callback: Callback function to handle IOCTL. The callback has the
// command pointer with data for command. There is a pointer
// called write_only, which when set, will not copy the
// response to user ioctl buffer. The "resume" argument
// can be used to avoid storing the command for replay
// during system resume
// @def_ioctl:	Default IOCTL handler callback, if there is no match in
// the existing list of IOCTL handled by the common handler.
//
// This structure is used to register an handler for IOCTL. To avoid
// code duplication common code handles all the IOCTL command read/write
// including handling multiple command in single IOCTL. The caller just
// need to execute a command via the registered callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_if_cmd_cb {
    pub registered: c_int,
    pub cmd_size: c_int,
    pub offset: c_int,
    pub api_version: c_int,
    pub owner: *mut module,
    pub resume): *mut *mut *mut *mut long (cmd_callback)(u8 ptr, int write_only, int,
    pub arg): *mut *mut *mut long (def_ioctl)(struct file file, unsigned int cmd, unsigned long,
}

// Internal interface functions
extern "C" {
    pub fn isst_if_cdev_register(type: c_int, cb: *mut isst_if_cmd_cb) -> c_int;
}
extern "C" {
    pub fn isst_if_cdev_unregister(type: c_int);
}
extern "C" {
    pub fn isst_if_mbox_cmd_set_req(mbox_cmd: *mut isst_if_mbox_cmd) -> bool;
}
extern "C" {
    pub fn isst_if_mbox_cmd_invalid(cmd: *mut isst_if_mbox_cmd) -> bool;
}
extern "C" {
    pub fn isst_resume_common();
}
