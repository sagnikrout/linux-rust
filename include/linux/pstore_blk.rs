//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pstore_blk.h
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
// struct pstore_device_info - back-end pstore/blk driver structure.
//
// @flags:	Refer to macro starting with PSTORE_FLAGS defined in
// linux/pstore.h. It means what front-ends this device support.
// Zero means all backends for compatible.
// @zone:	The struct pstore_zone_info details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstore_device_info {
    pub flags: c_uint,
    pub zone: pstore_zone_info,
}

extern "C" {
    pub fn register_pstore_device(dev: *mut pstore_device_info) -> c_int;
}
extern "C" {
    pub fn unregister_pstore_device(dev: *mut pstore_device_info);
}
//
// struct pstore_blk_config - the pstore_blk backend configuration
//
// @device:		Name of the desired block device
// @max_reason:		Maximum kmsg dump reason to store to block device
// @kmsg_size:		Total size of for kmsg dumps
// @pmsg_size:		Total size of the pmsg storage area
// @console_size:	Total size of the console storage area
// @ftrace_size:	Total size for ftrace logging data (for all CPUs)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstore_blk_config {
    pub device: [c_char; 80],
    pub max_reason: kmsg_dump_reason,
    pub kmsg_size: c_ulong,
    pub pmsg_size: c_ulong,
    pub console_size: c_ulong,
    pub ftrace_size: c_ulong,
}

//
// pstore_blk_get_config - get a copy of the pstore_blk backend configuration
//
// @info:	The sturct pstore_blk_config to be filled in
//
// Failure returns negative error code, and success returns 0.
//
extern "C" {
    pub fn pstore_blk_get_config(info: *mut pstore_blk_config) -> c_int;
}
