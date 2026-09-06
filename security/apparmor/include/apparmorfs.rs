//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/apparmorfs.h
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
// This file contains AppArmor filesystem definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aa_sfs_type {
    AA_SFS_TYPE_BOOLEAN,
    AA_SFS_TYPE_STRING,
    AA_SFS_TYPE_U64,
    AA_SFS_TYPE_FOPS,
    AA_SFS_TYPE_DIR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_sfs_entry {
    pub name: *const c_char,
    pub dentry: *mut dentry,
    pub mode: umode_t,
    pub v_type: aa_sfs_type,
    pub boolean: bool,
    pub string: *mut c_char,
    pub u64: c_ulong,
    pub files: *mut aa_sfs_entry,
    pub v: },
    pub file_ops: *const file_operations,
}

extern "C" {
    pub fn aa_destroy_aafs() -> void __init;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aafs_ns_type {
    AAFS_NS_DIR,
    AAFS_NS_PROFS,
    AAFS_NS_NS,
    AAFS_NS_RAW_DATA,
    AAFS_NS_LOAD,
    AAFS_NS_REPLACE,
    AAFS_NS_REMOVE,
    AAFS_NS_REVISION,
    AAFS_NS_COUNT,
    AAFS_NS_MAX_COUNT,
    AAFS_NS_SIZE,
    AAFS_NS_MAX_SIZE,
    AAFS_NS_OWNER,
    AAFS_NS_SIZEOF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aafs_prof_type {
    AAFS_PROF_DIR,
    AAFS_PROF_PROFS,
    AAFS_PROF_NAME,
    AAFS_PROF_MODE,
    AAFS_PROF_ATTACH,
    AAFS_PROF_HASH,
    AAFS_PROF_RAW_DATA,
    AAFS_PROF_RAW_HASH,
    AAFS_PROF_RAW_ABI,
    AAFS_PROF_SIZEOF,
}

extern "C" {
    pub fn aa_create_aafs() -> c_int;
}
extern "C" {
    pub fn __aa_bump_ns_revision(ns: *mut aa_ns);
}
extern "C" {
    pub fn __aafs_profile_rmdir(profile: *mut aa_profile);
}
extern "C" {
    pub fn __aafs_profile_mkdir(profile: *mut aa_profile, parent: *mut dentry) -> c_int;
}
extern "C" {
    pub fn __aafs_ns_rmdir(ns: *mut aa_ns);
}

extern "C" {
    pub fn __aa_fs_remove_rawdata(rawdata: *mut aa_loaddata);
}
extern "C" {
    pub fn __aa_fs_create_rawdata(ns: *mut aa_ns, rawdata: *mut aa_loaddata) -> c_int;
}
extern "C" {
    pub fn __aa_remove_rawdata_symlink_dents(profile: *mut aa_profile);
}
extern "C" {
    pub fn __aa_create_rawdata_symlink_dents(profile: *mut aa_profile) -> c_int;
}

// empty stub

