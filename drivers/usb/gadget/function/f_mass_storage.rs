//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/f_mass_storage.h
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
pub struct fsg_module_parameters {
    pub file: [*mut c_char; FSG_MAX_LUNS],
    pub ro: [bool; FSG_MAX_LUNS],
    pub removable: [bool; FSG_MAX_LUNS],
    pub cdrom: [bool; FSG_MAX_LUNS],
    pub nofua: [bool; FSG_MAX_LUNS],
    pub cdrom_count: unsigned int file_count, ro_count, removable_count,,
    pub nofua_count: c_uint,
    pub /: *mut *mut unsigned int luns; / nluns,
    pub /: *mut *mut bool stall; / can_stall,
}

// FSF callback functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsg_lun_opts {
    pub group: config_group,
    pub lun: *mut fsg_lun,
    pub lun_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsg_opts {
    pub common: *mut fsg_common,
    pub func_inst: usb_function_instance,
    pub lun0: fsg_lun_opts,
    pub default_groups: [*mut config_group; 2],
    pub /: *mut *mut bool no_configfs; / for legacy gadgets,
//
// Read/write access to configfs attributes is handled by configfs.
//
// This is to protect the data from concurrent access by read/write
// and create symlink/remove symlink.
//
    pub lock: mutex,
    pub refcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsg_lun_config {
    pub filename: *const c_char,
    pub ro: c_char,
    pub removable: c_char,
    pub cdrom: c_char,
    pub nofua: c_char,
    pub inquiry_string: [c_char; INQUIRY_STRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsg_config {
    pub nluns: unsigned,
    pub luns: [fsg_lun_config; FSG_MAX_LUNS],
// Callback functions.
    pub ops: *const fsg_operations,
// Gadget's private data.
    pub private_data: *mut c_void,
    pub /: *const *const *const char vendor_name; / 8 characters or less,
    pub /: *const *const *const char product_name; / 16 characters or less,
    pub can_stall: c_char,
    pub fsg_num_buffers: c_uint,
}

extern "C" {
    pub fn container_of(_arg: fi, fsg_opts: struct, _arg: func_inst) -> return;
}
extern "C" {
    pub fn fsg_common_set_sysfs(common: *mut fsg_common, sysfs: bool);
}
extern "C" {
    pub fn fsg_common_set_num_buffers(common: *mut fsg_common, n: c_uint) -> c_int;
}
extern "C" {
    pub fn fsg_common_free_buffers(common: *mut fsg_common);
}
extern "C" {
    pub fn fsg_common_remove_lun(lun: *mut fsg_lun);
}
extern "C" {
    pub fn fsg_common_remove_luns(common: *mut fsg_common);
}
extern "C" {
    pub fn fsg_common_create_luns(common: *mut fsg_common, cfg: *mut fsg_config) -> c_int;
}
