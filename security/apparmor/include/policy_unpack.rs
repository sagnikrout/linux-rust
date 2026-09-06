//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/policy_unpack.h
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
// This file contains AppArmor policy loading interface function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_load_ent {
    pub list: list_head,
    pub new: *mut aa_profile,
    pub old: *mut aa_profile,
    pub rename: *mut aa_profile,
    pub ns_name: *const c_char,
}

extern "C" {
    pub fn aa_load_ent_free(ent: *mut aa_load_ent);
}
pub const PACKED_FLAG_HAT: c_int = 1;
pub const PACKED_FLAG_DEBUG1: c_int = 2;
pub const PACKED_FLAG_DEBUG2: c_int = 4;
pub const PACKED_MODE_ENFORCE: c_int = 0;
pub const PACKED_MODE_COMPLAIN: c_int = 1;
pub const PACKED_MODE_KILL: c_int = 2;
pub const PACKED_MODE_UNCONFINED: c_int = 3;
pub const PACKED_MODE_USER: c_int = 4;
//
// The AppArmor interface treats data as a type byte followed by the
// actual data.  The interface has the notion of a named entry
// which has a name (AA_NAME typecode followed by name string) followed by
// the entries typecode and data.  Named types allow for optional
// elements and extensions to be added and tested for without breaking
// backwards compatibility.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aa_code {
    AA_U8,
    AA_U16,
    AA_U32,
    AA_U64,
    AA_NAME,		/* same as string except it is items name */
    AA_STRING,
    AA_BLOB,
    AA_STRUCT,
    AA_STRUCTEND,
    AA_LIST,
    AA_LISTEND,
    AA_ARRAY,
    AA_ARRAYEND,
}

//
// aa_ext is the read of the buffer containing the serialized profile.  The
// data is copied into a kernel buffer in apparmorfs and then handed off to
// the unpack routines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_ext {
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub /: *mut *mut *mut void pos; / pointer to current position in the buffer,
    pub version: u32,
}

// struct aa_loaddata - buffer of policy raw_data set
// @count: inode/filesystem refcount - use aa_get_i_loaddata()
// @pcount: profile refcount - use aa_get_profile_loaddata()
// @list: list the loaddata is on
// @work: used to do a delayed cleanup
// @dents: refs to dents created in aafs
// @ns: the namespace this loaddata was loaded into
// @name:
// @size: the size of the data that was loaded
// @compressed_size: the size of the data when it is compressed
// @revision: unique revision count that this data was loaded as
// @abi: the abi number the loaddata uses
// @hash: a hash of the loaddata, used to help dedup data
//
// There is no loaddata ref for being on ns->rawdata_list, so
// @ns->lock must be held when walking the list. Dentries and
// inode opens hold refs on @count; profiles hold refs on @pcount.
// When the last @pcount drops, do_ploaddata_rmfs() removes the
// fs entries and drops the associated @count ref.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_loaddata {
    pub count: aa_common_ref,
    pub pcount: kref,
    pub list: list_head,
    pub work: work_struct,
    pub dents: [*mut dentry; AAFS_LOADDATA_NDENTS],
    pub ns: *mut aa_ns,
    pub name: *mut c_char,
    pub /: *mut *mut size_t size; / the original size of the payload,
    pub /: *mut *mut size_t compressed_size; / the compressed size of the payload,
    pub /: *mut *mut long revision; / the ns policy revision this caused,
    pub abi: c_int,
    pub hash: *mut c_uchar,
// Pointer to payload. If @compressed_size > 0, then this is the
// compressed version of the payload, else it is the uncompressed
// version (with the size indicated by @size).
//
    pub data: *mut c_char,
}

//
// aa_get_i_loaddata - get a reference count from a counted data reference
// @data: reference to get a count on
//
// Returns: pointer to reference
// Requires: @data to have a valid reference count on it. It is a bug
// if the race to reap can be encountered when it is used.
//
// aa_get_profile_loaddata - get a profile reference count on loaddata
// @data: reference to get a count on
//
// Returns: pointer to reference
// Requires: @data to have a valid reference count on it.
//
// aa_get_profile_loaddata_not0 - get a profile reference count if not zero
// @data: reference to get a count on
//
// Like aa_get_profile_loaddata(), but safe to call on an entry that may
// be on a list (e.g. ns->rawdata_list) where the last pcount has already
// dropped and the deferred cleanup has not yet run.
//
// Returns: pointer to reference, or %NULL if @data is NULL or its
// profile refcount has already reached zero.
//
extern "C" {
    pub fn __aa_loaddata_update(data: *mut aa_loaddata, revision: c_long);
}
extern "C" {
    pub fn aa_rawdata_eq(l: *mut aa_loaddata, r: *mut aa_loaddata) -> bool;
}
extern "C" {
    pub fn aa_loaddata_kref(kref: *mut kref);
}
extern "C" {
    pub fn aa_ploaddata_kref(kref: *mut kref);
}

extern "C" {
    pub fn aa_inbounds(e: *mut aa_ext, size: usize) -> bool;
}
extern "C" {
    pub fn aa_unpack_u16_chunk(e: *mut aa_ext, chunk: *mut c_char) -> usize;
}
extern "C" {
    pub fn aa_unpack_X(e: *mut aa_ext, code: aa_code) -> bool;
}
extern "C" {
    pub fn aa_unpack_nameX(e: *mut aa_ext, code: aa_code, name: *const c_char) -> bool;
}
extern "C" {
    pub fn aa_unpack_u32(e: *mut aa_ext, data: *mut u32, name: *const c_char) -> bool;
}
extern "C" {
    pub fn aa_unpack_u64(e: *mut aa_ext, data: *mut u64, name: *const c_char) -> bool;
}
extern "C" {
    pub fn aa_unpack_array(e: *mut aa_ext, name: *const c_char, size: *mut u16) -> bool;
}
extern "C" {
    pub fn aa_unpack_blob(e: *mut aa_ext, blob: *mut c_char, name: *const c_char) -> usize;
}
extern "C" {
    pub fn aa_unpack_str(e: *mut aa_ext, string: *const c_char, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn aa_unpack_strdup(e: *mut aa_ext, string: *mut c_char, name: *const c_char) -> c_int;
}

