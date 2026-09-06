//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pstore_zone.h
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

extern "C" {
    pub fn ssize_t(: *mut *mut pstore_zone_read_op)(char, _arg: usize, _arg: loff_t) -> typedef;
}
extern "C" {
    pub fn ssize_t(: *const *const pstore_zone_write_op)(char, _arg: usize, _arg: loff_t) -> typedef;
}
extern "C" {
    pub fn ssize_t(_arg: *mut pstore_zone_erase_op)(size_t, _arg: loff_t) -> typedef;
}
//
// struct pstore_zone_info - pstore/zone back-end driver structure
//
// @owner:	Module which is responsible for this back-end driver.
// @name:	Name of the back-end driver.
// @total_size: The total size in bytes pstore/zone can use. It must be greater
// than 4096 and be multiple of 4096.
// @kmsg_size:	The size of oops/panic zone. Zero means disabled, otherwise,
// it must be multiple of SECTOR_SIZE(512 Bytes).
// @max_reason: Maximum kmsg dump reason to store.
// @pmsg_size:	The size of pmsg zone which is the same as @kmsg_size.
// @console_size:The size of console zone which is the same as @kmsg_size.
// @ftrace_size:The size of ftrace zone which is the same as @kmsg_size.
// @read:	The general read operation. Both of the function parameters
// @size and @offset are relative value to storage.
// On success, the number of bytes should be returned, others
// mean error.
// @write:	The same as @read, but the following error number:
// -EBUSY means try to write again later.
// -ENOMSG means to try next zone.
// @erase:	The general erase operation for device with special removing
// job. Both of the function parameters @size and @offset are
// relative value to storage.
// Return 0 on success and others on failure.
// @panic_write:The write operation only used for panic case. It's optional
// if you do not care panic log. The parameters are relative
// value to storage.
// On success, the number of bytes should be returned, others
// excluding -ENOMSG mean error. -ENOMSG means to try next zone.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstore_zone_info {
    pub owner: *mut module,
    pub name: *const c_char,
    pub total_size: c_ulong,
    pub kmsg_size: c_ulong,
    pub max_reason: c_int,
    pub pmsg_size: c_ulong,
    pub console_size: c_ulong,
    pub ftrace_size: c_ulong,
    pub read: pstore_zone_read_op,
    pub write: pstore_zone_write_op,
    pub erase: pstore_zone_erase_op,
    pub panic_write: pstore_zone_write_op,
}

extern "C" {
    pub fn register_pstore_zone(info: *mut pstore_zone_info) -> c_int;
}
extern "C" {
    pub fn unregister_pstore_zone(info: *mut pstore_zone_info);
}
