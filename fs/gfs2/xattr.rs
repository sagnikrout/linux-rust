//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/xattr.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_ea_request {
    pub er_name: *const c_char,
    pub er_data: *mut c_char,
    pub er_name_len: c_uint,
    pub er_data_len: c_uint,
    pub /: *mut *mut unsigned int er_type; / GFS2_EATYPE_...,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_ea_location {
    pub el_bh: *mut buffer_head,
    pub el_ea: *mut gfs2_ea_header,
    pub el_prev: *mut gfs2_ea_header,
}

extern "C" {
    pub fn gfs2_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn gfs2_ea_dealloc(ip: *mut gfs2_inode, initialized: bool) -> c_int;
}
// Exported to acl.c
extern "C" {
    pub fn gfs2_xattr_acl_get(ip: *mut gfs2_inode, name: *const c_char, data: *mut c_char) -> c_int;
}
