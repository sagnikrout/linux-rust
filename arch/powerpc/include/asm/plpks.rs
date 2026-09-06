//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/plpks.h
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
// Copyright (C) 2022 IBM Corporation
// Author: Nayna Jain <nayna@linux.ibm.com>
//
// Platform keystore for pseries LPAR(PLPKS).
//

// Object policy flags from supported_policies

// Signature algorithm flags from signed_update_algorithms

// Object label OS metadata flags
pub const PLPKS_VAR_LINUX: c_uint = 0x02;
pub const PLPKS_VAR_COMMON: c_uint = 0x04;
// Flags for which consumer owns an object is owned by
pub const PLPKS_FW_OWNER: c_uint = 0x1;
pub const PLPKS_BOOTLOADER_OWNER: c_uint = 0x2;
pub const PLPKS_OS_OWNER: c_uint = 0x3;
// Flags for label metadata fields
pub const PLPKS_LABEL_VERSION: c_int = 0;
pub const PLPKS_MAX_LABEL_ATTR_SIZE: c_int = 16;
pub const PLPKS_MAX_NAME_SIZE: c_int = 239;
pub const PLPKS_MAX_DATA_SIZE: c_int = 4000;
// Timeouts for PLPKS operations

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plpks_var {
    pub component: *mut c_char,
    pub name: *mut u8,
    pub data: *mut u8,
    pub policy: u32,
    pub namelen: u16,
    pub datalen: u16,
    pub os: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plpks_var_name {
    pub name: *mut u8,
    pub namelen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plpks_var_name_list {
    pub varcount: u32,
    pub varlist: [plpks_var_name; ],
}

extern "C" {
    pub fn plpks_signed_update_var(var: *mut plpks_var, flags: u64) -> c_int;
}
extern "C" {
    pub fn plpks_write_var(var: plpks_var) -> c_int;
}
extern "C" {
    pub fn plpks_read_os_var(var: *mut plpks_var) -> c_int;
}
extern "C" {
    pub fn plpks_read_fw_var(var: *mut plpks_var) -> c_int;
}
extern "C" {
    pub fn plpks_read_bootloader_var(var: *mut plpks_var) -> c_int;
}
extern "C" {
    pub fn plpks_is_available() -> bool;
}
extern "C" {
    pub fn plpks_get_version() -> u8;
}
extern "C" {
    pub fn plpks_get_objoverhead() -> u16;
}
extern "C" {
    pub fn plpks_get_maxpwsize() -> u16;
}
extern "C" {
    pub fn plpks_get_maxobjectsize() -> u16;
}
extern "C" {
    pub fn plpks_get_maxobjectlabelsize() -> u16;
}
extern "C" {
    pub fn plpks_get_totalsize() -> u32;
}
extern "C" {
    pub fn plpks_get_usedspace() -> u32;
}
extern "C" {
    pub fn plpks_get_supportedpolicies() -> u32;
}
extern "C" {
    pub fn plpks_get_maxlargeobjectsize() -> u32;
}
extern "C" {
    pub fn plpks_get_signedupdatealgorithms() -> u64;
}
extern "C" {
    pub fn plpks_get_wrappingfeatures() -> u64;
}
extern "C" {
    pub fn plpks_get_passwordlen() -> u16;
}
extern "C" {
    pub fn plpks_early_init_devtree();
}
extern "C" {
    pub fn plpks_populate_fdt(fdt: *mut c_void) -> c_int;
}
extern "C" {
    pub fn plpks_config_create_softlink(from: *mut kobject) -> c_int;
}
extern "C" {
    pub fn plpks_wrapping_is_supported() -> bool;
}
extern "C" {
    pub fn plpks_gen_wrapping_key() -> c_int;
}

