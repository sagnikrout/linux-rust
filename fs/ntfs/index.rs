//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/index.h
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
// Defines for NTFS kernel index handling.
//
// Copyright (c) 2004 Anton Altaparmakov
//

pub const MAX_PARENT_VCN: c_int = 32;
//
// @idx_ni:	index inode containing the @entry described by this context
// @name:	Unicode name of the indexed attribute
// (usually $I30 for directories)
// @name_len:	length of @name in Unicode characters
// @entry:	index entry (points into @ir or @ia)
// @cr:		creation time of the entry (for sorting/validation)
// @data:	index entry data (points into @entry)
// @data_len:	length in bytes of @data
// @is_in_root:	'true' if @entry is in @ir and 'false' if it is in @ia
// @ir:		index root if @is_in_root and NULL otherwise
// @actx:	attribute search context if @is_in_root and NULL otherwise
// @ib:		index block header (valid when @is_in_root is 'false')
// @ia_ni:	index allocation inode (extent inode) for @ia
// @parent_pos:	array of parent entry positions in the B-tree nodes
// @parent_vcn:	VCNs of parent index blocks in the B-tree traversal
// @pindex:	current depth (number of parent nodes) in the traversal
// (maximum is MAX_PARENT_VCN)
// @ib_dirty:	true if the current index block (@ia/@ib) was modified
// @block_size:	size of index blocks in bytes (from $INDEX_ROOT or $Boot)
// @vcn_size_bits: log2(cluster size)
// @sync_write:	true if synchronous writeback is requested for this context
//
// @idx_ni is the index inode this context belongs to.
//
// @entry is the index entry described by this context.  @data and @data_len
// are the index entry data and its length in bytes, respectively.  @data
// simply points into @entry.  This is probably what the user is interested in.
//
// If @is_in_root is 'true', @entry is in the index root attribute @ir described
// by the attribute search context @actx and the base inode @base_ni.  @ia and
// @page are NULL in this case.
//
// If @is_in_root is 'false', @entry is in the index allocation attribute and @ia
// and @page point to the index allocation block and the mapped, locked page it
// is in, respectively.  @ir, @actx and @base_ni are NULL in this case.
//
// To obtain a context call ntfs_index_ctx_get().
//
// We use this context to allow ntfs_index_lookup() to return the found index
// @entry and its @data without having to allocate a buffer and copy the @entry
// and/or its @data into it.
//
// When finished with the @entry and its @data, call ntfs_index_ctx_put() to
// free the context and other associated resources.
//
// If the index entry was modified, ntfs_index_entry_mark_dirty()
// or ntfs_index_entry_write() before the call to ntfs_index_ctx_put() to
// ensure that the changes are written to disk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_index_context {
    pub idx_ni: *mut ntfs_inode,
    pub name: *mut __le16,
    pub name_len: u32,
    pub entry: *mut index_entry,
    pub cr: __le32,
    pub data: *mut c_void,
    pub data_len: u16,
    pub is_in_root: bool,
    pub ir: *mut index_root,
    pub actx: *mut ntfs_attr_search_ctx,
    pub ib: *mut index_block,
    pub ia_ni: *mut ntfs_inode,
    pub parent_pos: [c_int; MAX_PARENT_VCN],
    pub parent_vcn: [i64; MAX_PARENT_VCN],
    pub pindex: c_int,
    pub ib_dirty: bool,
    pub block_size: u32,
    pub vcn_size_bits: u8,
    pub sync_write: bool,
}

extern "C" {
    pub fn ntfs_index_ctx_put(ictx: *mut ntfs_index_context);
}
extern "C" {
    pub fn ntfs_index_entry_mark_dirty(ictx: *mut ntfs_index_context);
}
extern "C" {
    pub fn ntfs_index_add_filename(ni: *mut ntfs_inode, fn: *mut file_name_attr, mref: u64) -> c_int;
}
extern "C" {
    pub fn ntfs_index_remove(ni: *mut ntfs_inode, key: *const c_void, keylen: u32) -> c_int;
}
extern "C" {
    pub fn ntfs_index_rm(icx: *mut ntfs_index_context) -> c_int;
}
extern "C" {
    pub fn ntfs_index_ctx_reinit(icx: *mut ntfs_index_context);
}
extern "C" {
    pub fn ntfs_ie_add(icx: *mut ntfs_index_context, ie: *mut index_entry) -> c_int;
}
extern "C" {
    pub fn ntfs_icx_ib_sync_write(icx: *mut ntfs_index_context) -> c_int;
}
