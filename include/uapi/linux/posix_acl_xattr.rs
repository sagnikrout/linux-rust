//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/posix_acl_xattr.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// Copyright (C) 2002 Andreas Gruenbacher <a.gruenbacher@computer.org>
// Copyright (C) 2016 Red Hat, Inc.
//
// This file is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//

// Supported ACL a_version fields
pub const POSIX_ACL_XATTR_VERSION: c_uint = 0x0002;
// An undefined entry e_id value

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_acl_xattr_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_acl_xattr_header {
    pub a_version: __le32,
}
