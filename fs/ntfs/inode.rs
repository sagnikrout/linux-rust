//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/inode.h
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
// Defines for inode structures NTFS Linux kernel driver.
//
// Copyright (c) 2001-2007 Anton Altaparmakov
// Copyright (c) 2002 Richard Russon
// Copyright (c) 2025 LG Electronics Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ntfs_inode_mutex_lock_class {
    NTFS_INODE_MUTEX_PARENT,
    NTFS_INODE_MUTEX_NORMAL,
    NTFS_INODE_MUTEX_NORMAL_CHILD,
    NTFS_INODE_MUTEX_PARENT_2,
    NTFS_INODE_MUTEX_NORMAL_2,
    NTFS_EXTEND_MUTEX_PARENT,
    NTFS_EA_MUTEX_NORMAL
}

//
// The NTFS in-memory inode structure. It is just used as an extension to the
// fields already provided in the VFS inode.
// @size_lock: Lock serializing access to inode sizes.
// @state: NTFS specific flags describing this inode.
// @flags: Flags describing the file. (Copy from STANDARD_INFORMATION).
// @mft_no: Number of the mft record / inode.
// @seq_no: Sequence number of the mft record.
// @count: Inode reference count for book keeping.
// @vol: Pointer to the ntfs volume of this inode.
//
// If NInoAttr() is true, the below fields describe the attribute which
// this fake inode belongs to. The actual inode of this attribute is
// pointed to by base_ntfs_ino and nr_extents is always set to -1 (see
// below). For real inodes, we also set the type (AT_DATA for files and
// AT_INDEX_ALLOCATION for directories), with the name = NULL and
// name_len = 0 for files and name = I30 (global constant) and
// name_len = 4 for directories.
// @type: Attribute type of this fake inode.
// @name: Attribute name of this fake inode.
// @name_len: Attribute name length of this fake inode.
// @runlist: If state has the NI_NonResident bit set, the runlist of
// the unnamed data attribute (if a file) or of the index allocation
// attribute (directory) or of the attribute described by the fake inode
// (if NInoAttr()). If runlist.rl is NULL, the runlist has not been read
// in yet or has been unmapped. If NI_NonResident is clear, the attribute
// is resident (file and fake inode) or there is no $I30 index allocation
// attribute (small directory). In the latter case runlist.rl is always
// NULL.
// @data_size: Copy from the attribute record.
// @initialized_size: Copy from the attribute record.
// @allocated_size: Copy from the attribute record.
// @i_crtime: File Creation time.
// @mrec: MFT record
// @mrec_lock: Lock for serializing access to the mft record belonging to
// this inode.
// @folio: The folio containing the mft record of the inode.
// @folio_ofs: Offset into the folio at which the mft record begins.
// @mft_lcn: Number containing the mft record.
// @mft_lcn_count: Number of clusters per mft record.
//
// Attribute list support (only for use by the attribute lookup
// functions). Setup during read_inode for all inodes with attribute
// lists. Only valid if NI_AttrList is set in state.
// @attr_list_size: Length of attribute list value in bytes.
// @attr_list: Attribute list value itself.
//
// It is a directory, $MFT, or an index inode.
// @block_size: Size of an index block.
// @vcn_size: Size of a vcn in this index.
// @collation_rule: The collation rule for the index.
// @block_size_bits: Log2 of the above.
// @vcn_size_bits: Log2 of the above.
//
// It is a compressed/sparse file/attribute inode.
// @size: Copy of compressed_size from $DATA.
// @block_size: Size of a compression block (cb).
// @block_size_bits: Log2 of the size of a cb.
// @block_clusters: Number of clusters per cb.
// @extent_lock: Lock for accessing/modifying the below.
// @nr_extents: For a base mft record, the number of attached extent inodes
// (0 if none), for extent records and for fake inodes describing an
// attribute this is -1.
//
// This union is only used if nr_extents != 0.
// @extent_ntfs_inos: For nr_extents > 0, array of the ntfs inodes of
// the extent mft records belonging to this base inode which have been
// loaded.
// @base_ntfs_ino: For nr_extents == -1, the ntfs inode of the base mft
// record. For fake inodes, the real (base) inode to which the attribute
// belongs.
// @i_dealloc_clusters: delayed allocated clusters.
// @target: symlink buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_inode {
    pub size_lock: rwlock_t,
    pub state: c_ulong,
    pub flags: __le32,
    pub mft_no: u64,
    pub seq_no: u16,
    pub count: core::sync::atomic::AtomicI32,
    pub vol: *mut ntfs_volume,
    pub type: __le32,
    pub name: *mut __le16,
    pub name_len: u32,
    pub runlist: runlist,
    pub data_size: i64,
    pub initialized_size: i64,
    pub allocated_size: i64,
    pub i_crtime: timespec64,
    pub mrec: *mut c_void,
    pub mrec_lock: mutex,
    pub folio: *mut folio,
    pub folio_ofs: c_int,
    pub mft_lcn: [i64; 2],
    pub mft_lcn_count: c_uint,
    pub attr_list_size: u32,
    pub attr_list: *mut u8,
    pub block_size: u32,
    pub vcn_size: u32,
    pub collation_rule: __le32,
    pub block_size_bits: u8,
    pub vcn_size_bits: u8,
    pub index: },
    pub size: i64,
    pub block_size: u32,
    pub block_size_bits: u8,
    pub block_clusters: u8,
    pub compressed: },
    pub itype: },
    pub extent_lock: mutex,
    pub nr_extents: i32,
    pub extent_ntfs_inos: *mut ntfs_inode,
    pub base_ntfs_ino: *mut ntfs_inode,
    pub ext: },
    pub i_dealloc_clusters: c_uint,
    pub reparse_tag: __le32,
    pub reparse_flags: __le32,
    pub target: *mut c_char,
}

//
// Defined bits for the state field in the ntfs_inode structure.
// (f) = files only, (d) = directories only, (a) = attributes/fake inodes only
//
// NI_Dirty			Mft record needs to be written to disk.
// NI_AttrListDirty		Mft record contains an attribute list.
// NI_AttrList			Mft record contains an attribute list.
// NI_AttrListNonResident	Attribute list is non-resident. Implies
// NI_AttrList is set.
// NI_Attr			1: Fake inode for attribute i/o.
// 0: Real inode or extent inode.
// NI_MstProtected		Attribute is protected by MST fixups.
// NI_NonResident		Unnamed data attr is non-resident (f)
// Attribute is non-resident (a).
// NI_IndexAllocPresent		$I30 index alloc attr is present (d).
// NI_Compressed		Unnamed data attr is compressed (f).
// Create compressed files by default (d).
// Attribute is compressed (a).
// NI_Encrypted			Unnamed data attr is encrypted (f).
// Create encrypted files by default (d).
// Attribute is encrypted (a).
// NI_Sparse			Unnamed data attr is sparse (f).
// Create sparse files by default (d).
// Attribute is sparse (a).
// NI_SparseDisabled		May not create sparse regions.
// NI_FullyMapped		Runlist is fully mapped.
// NI_FileNameDirty		FILE_NAME attributes need to be updated.
// NI_BeingDeleted		ntfs inode is being delated.
// NI_BeingCreated		ntfs inode is being created.
// NI_HasEA			ntfs inode has EA attribute.
// NI_RunlistDirty		runlist need to be updated.
//
// NOTE: We should be adding dirty mft records to a list somewhere and they
// should be independent of the (ntfs/vfs) inode structure so that an inode can
// be removed but the record can be left dirty for syncing later.
//
// Macro tricks to expand the NInoFoo(), NInoSetFoo(), and NInoClearFoo()
// functions.
//

//
// As above for NInoTestSetFoo() and NInoTestClearFoo().
//

// Emit the ntfs inode bitops functions.
//
// The full structure containing a ntfs_inode and a vfs struct inode. Used for
// all real and fake inodes but not for extent inodes which lack the vfs struct
// inode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct big_ntfs_inode {
    pub ntfs_inode: ntfs_inode,
    pub /: *mut *mut inode vfs_inode; / The vfs inode structure.,
}

//
// NTFS_I - return the ntfs inode given a vfs inode
// @inode:	VFS inode
//
// NTFS_I() returns the ntfs inode associated with the VFS @inode.
//
// ntfs_attr - ntfs in memory attribute structure
//
// This structure exists only to provide a small structure for the
// ntfs_{attr_}iget()/ntfs_test_inode()/ntfs_init_locked_inode() mechanism.
//
// NOTE: Elements are ordered by size to make the structure as compact as
// possible on all architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_attr {
    pub mft_no: u64,
    pub name: *mut __le16,
    pub name_len: u32,
    pub type: __le32,
    pub state: c_ulong,
}

extern "C" {
    pub fn ntfs_test_inode(vi: *mut inode, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ntfs_free_big_inode(inode: *mut inode);
}
extern "C" {
    pub fn ntfs_drop_big_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ntfs_evict_big_inode(vi: *mut inode);
}
extern "C" {
    pub fn __ntfs_init_inode(sb: *mut super_block, ni: *mut ntfs_inode);
}
extern "C" {
    pub fn ntfs_clear_extent_inode(ni: *mut ntfs_inode);
}
extern "C" {
    pub fn ntfs_read_inode_mount(vi: *mut inode) -> c_int;
}
extern "C" {
    pub fn ntfs_show_options(sf: *mut seq_file, root: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ntfs_truncate_vfs(vi: *mut inode, new_size: loff_t, i_size: loff_t) -> c_int;
}
extern "C" {
    pub fn ntfs_get_block_mft_record(mft_ni: *mut ntfs_inode, ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn __ntfs_write_inode(vi: *mut inode, sync: c_int) -> c_int;
}
extern "C" {
    pub fn ntfs_inode_attach_all_extents(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_inode_add_attrlist(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_inode_free_empty_extents(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_destroy_ext_inode(ni: *mut ntfs_inode);
}
extern "C" {
    pub fn ntfs_inode_free_space(ni: *mut ntfs_inode, size: c_int) -> c_int;
}
extern "C" {
    pub fn ntfs_inode_attr_pread(vi: *mut inode, pos: i64, count: i64, buf: *mut u8) -> i64;
}
extern "C" {
    pub fn ntfs_inode_close(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_inode_sync_filename(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_set_vfs_operations(inode: *mut inode, mode: mode_t, dev: dev_t);
}
