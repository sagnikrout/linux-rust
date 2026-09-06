//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/attrib.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Defines for attribute handling in NTFS Linux kernel driver.
//
// Copyright (c) 2001-2005 Anton Altaparmakov
// Copyright (c) 2002 Richard Russon
// Copyright (c) 2025 LG Electronics Co., Ltd.
//

//
// ntfs_attr_search_ctx - used in attribute search functions
// @mrec: buffer containing mft record to search
// @mapped_mrec: true if @mrec was mapped by the search functions
// @attr: attribute record in @mrec where to begin/continue search
// @is_first: if true ntfs_attr_lookup() begins search with @attr, else after
// @ntfs_ino: Inode owning this attribute search
// @al_entry: Current attribute list entry
// @base_ntfs_ino: Base inode
// @mapped_base_mrec: true if @base_mrec was mapped by the search
// @base_attr: Base attribute record pointer
//
// Structure must be initialized to zero before the first call to one of the
// attribute search functions. Initialize @mrec to point to the mft record to
// search, and @attr to point to the first attribute within @mrec (not necessary
// if calling the _first() functions), and set @is_first to 'true' (not necessary
// if calling the _first() functions).
//
// If @is_first is 'true', the search begins with @attr. If @is_first is 'false',
// the search begins after @attr. This is so that, after the first call to one
// of the search attribute functions, we can call the function again, without
// any modification of the search context, to automagically get the next
// matching attribute.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_attr_search_ctx {
    pub mrec: *mut mft_record,
    pub mapped_mrec: bool,
    pub attr: *mut attr_record,
    pub is_first: bool,
    pub ntfs_ino: *mut ntfs_inode,
    pub al_entry: *mut attr_list_entry,
    pub base_ntfs_ino: *mut ntfs_inode,
    pub base_mrec: *mut mft_record,
    pub mapped_base_mrec: bool,
    pub base_attr: *mut attr_record,
}

extern "C" {
    pub fn ntfs_map_runlist(ni: *mut ntfs_inode, vcn: i64) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_map_whole_runlist(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_list_is_valid(al_start: *const u8, size: i64) -> bool;
}
extern "C" {
    pub fn le64_to_cpu(_arg: a->data.non_resident.data_size) -> return;
}
extern "C" {
    pub fn ntfs_attr_reinit_search_ctx(ctx: *mut ntfs_attr_search_ctx);
}
extern "C" {
    pub fn ntfs_attr_put_search_ctx(ctx: *mut ntfs_attr_search_ctx);
}
extern "C" {
    pub fn ntfs_attr_record_resize(m: *mut mft_record, a: *mut attr_record, new_size: u32) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_make_non_resident(ni: *mut ntfs_inode, data_size: u32) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_set_initialized_size(ni: *mut ntfs_inode, new_size: loff_t) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_close(n: *mut ntfs_inode);
}
extern "C" {
    pub fn ntfs_attr_fallocate(ni: *mut ntfs_inode, start: loff_t, byte_len: loff_t, keep_size: bool) -> c_int;
}
extern "C" {
    pub fn ntfs_non_resident_attr_insert_range(ni: *mut ntfs_inode, start_vcn: i64, len: i64) -> c_int;
}
extern "C" {
    pub fn ntfs_non_resident_attr_collapse_range(ni: *mut ntfs_inode, start_vcn: i64, len: i64) -> c_int;
}
extern "C" {
    pub fn ntfs_non_resident_attr_punch_hole(ni: *mut ntfs_inode, start_vcn: i64, len: i64) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_expand(ni: *mut ntfs_inode, newsize: i64, prealloc_size: i64) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_truncate_i(ni: *mut ntfs_inode, newsize: i64, holes: c_uint) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_truncate(ni: *mut ntfs_inode, newsize: i64) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_rm(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_record_rm(ctx: *mut ntfs_attr_search_ctx) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_record_move_to(ctx: *mut ntfs_attr_search_ctx, ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_record_move_away(ctx: *mut ntfs_attr_search_ctx, extra: c_int) -> c_int;
}
extern "C" {
    pub fn ntfs_attr_name_free(name: *mut c_uchar);
}
extern "C" {
    pub fn ntfs_attr_update_mapping_pairs(ni: *mut ntfs_inode, from_vcn: i64) -> c_int;
}
//
// ntfs_attrs_walk - syntactic sugar for walking all attributes in an inode
// @ctx:	initialised attribute search context
//
// Syntactic sugar for walking attributes in an inode.
//
// Return 0 on success and -1 on error with errno set to the error code from
// ntfs_attr_lookup().
//
// Example: When you want to enumerate all attributes in an open ntfs inode
// @ni, you can simply do:
//
// int err;
// struct ntfs_attr_search_ctx *ctx = ntfs_attr_get_search_ctx(ni, NULL);
// if (!ctx)
// // Error code is in errno. Handle this case.
// while (!(err = ntfs_attrs_walk(ctx))) {
// struct attr_record *attr = ctx->attr;
// // attr now contains the next attribute. Do whatever you want
// // with it and then just continue with the while loop.
// }
// if (err && errno != ENOENT)
// // Ooops. An error occurred! You should handle this case.
// // Now finished with all attributes in the inode.
//
