//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifs_swn.h
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
// Witness Service client for CIFS
//
// Copyright (c) 2020 Samuel Cabrero <scabrero@suse.de>
//

extern "C" {
    pub fn cifs_swn_register(tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn cifs_swn_unregister(tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn cifs_swn_notify(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn cifs_swn_dump(m: *mut seq_file);
}
extern "C" {
    pub fn cifs_swn_check();
}

