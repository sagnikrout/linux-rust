//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ima.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2008 IBM Corporation
// Author: Mimi Zohar <zohar@us.ibm.com>
//

extern "C" {
    pub fn ima_get_current_hash_algo() -> hash_algo;
}
extern "C" {
    pub fn ima_file_hash(file: *mut file, buf: *mut c_char, buf_size: usize) -> c_int;
}
extern "C" {
    pub fn ima_inode_hash(inode: *mut inode, buf: *mut c_char, buf_size: usize) -> c_int;
}
extern "C" {
    pub fn ima_kexec_cmdline(kernel_fd: c_int, buf: *const c_void, size: c_int);
}

extern "C" {
    pub fn ima_appraise_parse_cmdline();
}

extern "C" {
    pub fn ima_add_kexec_buffer(image: *mut kimage);
}
extern "C" {
    pub fn ima_kexec_post_load(image: *mut kimage);
}

extern "C" {
    pub fn ima_free_kexec_buffer() -> int __init;
}
extern "C" {
    pub fn ima_get_kexec_buffer(addr: *mut c_void, size: *mut usize) -> int __init;
}
extern "C" {
    pub fn ima_validate_range(phys: phys_addr_t, size: usize) -> c_int;
}

extern "C" {
    pub fn is_ima_appraise_enabled() -> bool;
}

extern "C" {
    pub fn ima_appraise_signature(func: kernel_read_file_id) -> bool;
}

