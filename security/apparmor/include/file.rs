//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/file.h
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
// AppArmor security module
//
// This file contains AppArmor file mediation function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

// struct aa_file_ctx - the AppArmor context the file was opened in
// @lock: lock to update the ctx
// @label: label currently cached on the ctx
// @perms: the permission the file was opened with
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_file_ctx {
    pub lock: spinlock_t,
    pub label: *mut aa_label __rcu,
    pub allow: u32,
}

//
// The xindex is broken into 3 parts
// - index - an index into either the exec name table or the variable table
// - exec type - which determines how the executable name and index are used
// - flags - which modify how the destination name is applied
//

pub const AA_X_TYPE_MASK: c_uint = 0x0c000000;

pub const AA_X_NAME: c_uint = 0x04000000 /* use executable name px */;
pub const AA_X_TABLE: c_uint = 0x08000000 /* use a specified name ->n# */;
pub const AA_X_UNSAFE: c_uint = 0x10000000;
pub const AA_X_CHILD: c_uint = 0x20000000;
pub const AA_X_INHERIT: c_uint = 0x40000000;
pub const AA_X_UNCONFINED: c_uint = 0x80000000;
// need to make conditional which ones are being set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_cond {
    pub uid: kuid_t,
    pub mode: umode_t,
}

extern "C" {
    pub fn aa_inherit_files(cred: *const cred, files: *mut files_struct);
}
//
// aa_map_file_to_perms - map file flags to AppArmor permissions
// @file: open file to map flags to AppArmor permissions
//
// Returns: apparmor permission set for the file
//
// trunc implies write permission
