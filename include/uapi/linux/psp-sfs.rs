//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/psp-sfs.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Userspace interface for AMD Seamless Firmware Servicing (SFS)
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//
// Author: Ashish Kalra <ashish.kalra@amd.com>
//

//
// SFS: AMD Seamless Firmware Support (SFS) interface
//
pub const PAYLOAD_NAME_SIZE: c_int = 64;
pub const TEE_EXT_CMD_BUFFER_SIZE: c_int = 4096;
//
// struct sfs_user_get_fw_versions - get current level of base firmware (output).
// @blob:                  current level of base firmware for ASP and patch levels (input/output).
// @sfs_status:            32-bit SFS status value (output).
// @sfs_extended_status:   32-bit SFS extended status value (output).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfs_user_get_fw_versions {
    pub blob: [__u8; TEE_EXT_CMD_BUFFER_SIZE],
    pub sfs_status: __u32,
    pub sfs_extended_status: __u32,
    pub __packed: },
//
// struct sfs_user_update_package - update SFS package (input).
// @payload_name:          name of SFS package to load, verify and execute (input).
// @sfs_status:            32-bit SFS status value (output).
// @sfs_extended_status:   32-bit SFS extended status value (output).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfs_user_update_package {
    pub payload_name: [c_char; PAYLOAD_NAME_SIZE],
    pub sfs_status: __u32,
    pub sfs_extended_status: __u32,
    pub __packed: },
//
// Seamless Firmware Support (SFS) IOC
//
// possible return codes for all SFS IOCTLs:
// 0:          success
// -EINVAL:    invalid input
// -E2BIG:     excess data passed
// -EFAULT:    failed to copy to/from userspace
// -EBUSY:     mailbox in recovery or in use
// -ENODEV:    driver not bound with PSP device
// -EACCES:    request isn't authorized
// -EINVAL:    invalid parameter
// -ETIMEDOUT: request timed out
// -EAGAIN:    invalid request for state machine
// -ENOENT:    not implemented
// -ENFILE:    overflow
// -EPERM:     invalid signature
// -EIO:       PSP I/O error
//

//
// SFSIOCFWVERS - returns blob containing FW versions
// ASP provides the current level of Base Firmware for the ASP
// and the other microprocessors as well as current patch
// level(s).
//

//
// SFSIOCUPDATEPKG - updates package/payload
// ASP loads, verifies and executes the SFS package.
// By default, the SFS package/payload is loaded from
// /lib/firmware/amd, but alternative firmware loading
// path can be specified using kernel parameter
// firmware_class.path or the firmware loading path
// can be customized using sysfs file:
// /sys/module/firmware_class/parameters/path.
//

