//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/base/firmware_loader/sysfs.h
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
// struct firmware_fallback_config - firmware fallback configuration settings
//
// Helps describe and fine tune the fallback mechanism.
//
// @force_sysfs_fallback: force the sysfs fallback mechanism to be used
// as if one had enabled CONFIG_FW_LOADER_USER_HELPER_FALLBACK=y.
// Useful to help debug a CONFIG_FW_LOADER_USER_HELPER_FALLBACK=y
// functionality on a kernel where that config entry has been disabled.
// @ignore_sysfs_fallback: force to disable the sysfs fallback mechanism.
// This emulates the behaviour as if we had set the kernel
// config CONFIG_FW_LOADER_USER_HELPER=n.
// @old_timeout: for internal use
// @loading_timeout: the timeout to wait for the fallback mechanism before
// giving up, in seconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct firmware_fallback_config {
    pub force_sysfs_fallback: c_uint,
    pub ignore_sysfs_fallback: c_uint,
    pub old_timeout: c_int,
    pub loading_timeout: c_int,
}

// These getters are vetted to use int properly
// These setters are vetted to use int properly

extern "C" {
    pub fn register_sysfs_loader() -> c_int;
}
extern "C" {
    pub fn unregister_sysfs_loader();
}

extern "C" {
    pub fn register_firmware_config_sysctl() -> c_int;
}
extern "C" {
    pub fn unregister_firmware_config_sysctl();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_sysfs {
    pub nowait: bool,
    pub dev: device,
    pub fw_priv: *mut fw_priv,
    pub fw: *mut firmware,
    pub fw_upload_priv: *mut c_void,
}

extern "C" {
    pub fn __fw_load_abort(fw_priv: *mut fw_priv);
}

extern "C" {
    pub fn fw_upload_start(fw_sysfs: *mut fw_sysfs) -> c_int;
}
extern "C" {
    pub fn fw_upload_free(fw_sysfs: *mut fw_sysfs);
}
extern "C" {
    pub fn fw_upload_is_visible(kobj: *mut kobject, attr: *mut attribute, n: c_int) -> umode_t;
}

