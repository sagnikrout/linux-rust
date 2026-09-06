//! Automatically rewritten from C Header to Rust Module
//! Source: fs/hfsplus/hfsplus_fs.h
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
// linux/include/linux/hfsplus_fs.h
//
// Copyright (C) 1999
// Brad Boyer (flar@pants.nu)
// (C) 2003 Ardis Technologies <roman@ardistech.com>
//

// Runtime config options
pub const HFSPLUS_DEF_CR_TYPE: c_uint = 0x3F3F3F3F  /* '????' */;
pub const HFSPLUS_TYPE_DATA: c_uint = 0x00;
pub const HFSPLUS_TYPE_RSRC: c_uint = 0xFF;
pub const NODE_HASH_SIZE: c_int = 256;
// B-tree mutex nested subclasses
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfsplus_btree_mutex_classes {
    CATALOG_BTREE_MUTEX,
    EXTENTS_BTREE_MUTEX,
    ATTR_BTREE_MUTEX,
}

// An HFS+ BTree held in memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_btree {
    pub sb: *mut super_block,
    pub inode: *mut inode,
    pub keycmp: btree_keycmp,
    pub cnid: u32,
    pub root: u32,
    pub leaf_count: u32,
    pub leaf_head: u32,
    pub leaf_tail: u32,
    pub node_count: u32,
    pub free_nodes: u32,
    pub attributes: u32,
    pub node_size: c_uint,
    pub node_size_shift: c_uint,
    pub max_key_len: c_uint,
    pub depth: c_uint,
    pub tree_lock: mutex,
    pub pages_per_bnode: c_uint,
    pub hash_lock: spinlock_t,
    pub node_hash: [*mut hfs_bnode; NODE_HASH_SIZE],
    pub node_hash_cnt: c_int,
}

// An HFS+ BTree node in memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_bnode {
    pub tree: *mut hfs_btree,
    pub prev: u32,
    pub this: u32,
    pub next: u32,
    pub parent: u32,
    pub num_recs: u16,
    pub type: u8,
    pub height: u8,
    pub next_hash: *mut hfs_bnode,
    pub flags: c_ulong,
    pub lock_wq: wait_queue_head_t,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub page_offset: c_uint,
    pub page: [*mut page; ],
}

pub const HFS_BNODE_LOCK: c_int = 0;
pub const HFS_BNODE_ERROR: c_int = 1;
pub const HFS_BNODE_NEW: c_int = 2;
pub const HFS_BNODE_DIRTY: c_int = 3;
pub const HFS_BNODE_DELETED: c_int = 4;
//
// Attributes file states
//
pub const HFSPLUS_EMPTY_ATTR_TREE: c_int = 0;
pub const HFSPLUS_CREATING_ATTR_TREE: c_int = 1;
pub const HFSPLUS_VALID_ATTR_TREE: c_int = 2;
pub const HFSPLUS_FAILED_ATTR_TREE: c_int = 3;
//
// HFS+ superblock info (built from Volume Header on disk)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_sb_info {
    pub s_vhdr_buf: *mut c_void,
    pub s_vhdr: *mut hfsplus_vh,
    pub s_backup_vhdr_buf: *mut c_void,
    pub s_backup_vhdr: *mut hfsplus_vh,
    pub ext_tree: *mut hfs_btree,
    pub cat_tree: *mut hfs_btree,
    pub attr_tree: *mut hfs_btree,
    pub attr_tree_state: core::sync::atomic::AtomicI32,
    pub alloc_file: *mut inode,
    pub hidden_dir: *mut inode,
    pub nls: *mut nls_table,
// Runtime variables
    pub blockoffset: u32,
    pub min_io_size: u32,
    pub part_start: sector_t,
    pub sect_count: sector_t,
    pub fs_shift: c_int,
// immutable data from the volume header
    pub alloc_blksz: u32,
    pub alloc_blksz_shift: c_int,
    pub total_blocks: u32,
    pub rsrc_clump_blocks: u32 data_clump_blocks,,
// mutable data from the volume header, protected by alloc_mutex
    pub free_blocks: u32,
    pub alloc_mutex: mutex,
// mutable data from the volume header, protected by vh_mutex
    pub next_cnid: u32,
    pub file_count: u32,
    pub folder_count: u32,
    pub vh_mutex: mutex,
// Config options
    pub creator: u32,
    pub type: u32,
    pub umask: umode_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub session: int part,,
    pub flags: c_ulong,
    pub /: *mut *mut int work_queued; / non-zero delayed work is queued,
    pub /: *mut *mut delayed_work sync_work; / FS sync delayed work,
    pub /: *mut *mut spinlock_t work_lock; / protects sync_work and work_queued,
    pub rcu: rcu_head,
}

pub const HFSPLUS_SB_WRITEBACKUP: c_int = 0;
pub const HFSPLUS_SB_NODECOMPOSE: c_int = 1;
pub const HFSPLUS_SB_FORCE: c_int = 2;
pub const HFSPLUS_SB_HFSX: c_int = 3;
pub const HFSPLUS_SB_CASEFOLD: c_int = 4;
pub const HFSPLUS_SB_NOBARRIER: c_int = 5;
pub const HFSPLUS_SB_UID: c_int = 6;
pub const HFSPLUS_SB_GID: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_inode_info {
    pub opencnt: core::sync::atomic::AtomicI32,
//
// Extent allocation information, protected by extents_lock.
//
    pub first_blocks: u32,
    pub clump_blocks: u32,
    pub alloc_blocks: u32,
    pub cached_start: u32,
    pub cached_blocks: u32,
    pub first_extents: hfsplus_extent_rec,
    pub cached_extents: hfsplus_extent_rec,
    pub extent_state: c_uint,
    pub extents_lock: mutex,
//
// Immutable data.
//
    pub rsrc_inode: *mut inode,
    pub create_date: __be32,
//
// Protected by sbi->vh_mutex.
//
    pub linkid: u32,
//
// Accessed using atomic bitops.
//
    pub flags: c_ulong,
//
// Protected by i_mutex.
//
    pub fs_blocks: sector_t,
    pub /: *mut *mut u8 userflags; / BSD user file flags,
    pub /: *mut *mut u32 subfolders; / Subfolder count (HFSX only),
    pub phys_size: loff_t,
    pub vfs_inode: inode,
}

pub const HFSPLUS_EXT_DIRTY: c_uint = 0x0001;
pub const HFSPLUS_EXT_NEW: c_uint = 0x0002;

extern "C" {
    pub fn container_of(_arg: inode, hfsplus_inode_info: struct, _arg: vfs_inode) -> return;
}

//
// Mark an inode dirty, and also mark the btree in which the
// specific type of metadata is stored.
// For data or metadata that gets written back by into the catalog btree
// by hfsplus_write_inode a plain mark_inode_dirty call is enough.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_find_data {
// filled by caller
    pub search_key: *mut hfsplus_btree_key,
    pub key: *mut hfsplus_btree_key,
// filled by find
    pub tree: *mut hfs_btree,
    pub bnode: *mut hfs_bnode,
// filled by findrec
    pub record: c_int,
    pub keylength: int keyoffset,,
    pub entrylength: int entryoffset,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_readdir_data {
    pub pos: loff_t,
    pub key: hfsplus_cat_key,
}

//
// Find minimum acceptible I/O size for an hfsplus sb.
//

//
// hfs+-specific ioctl for making the filesystem bootable
//

//
// Functions in any *.c used in other files
//
// attributes.c
extern "C" {
    pub fn hfsplus_create_attr_tree_cache() -> int __init;
}
extern "C" {
    pub fn hfsplus_destroy_attr_tree_cache();
}
extern "C" {
    pub fn hfsplus_destroy_attr_entry(entry: *mut hfsplus_attr_entry);
}
extern "C" {
    pub fn hfsplus_attr_exists(inode: *mut inode, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn hfsplus_delete_attr(inode: *mut inode, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn hfsplus_delete_all_attrs(dir: *mut inode, cnid: u32) -> c_int;
}
// bitmap.c
extern "C" {
    pub fn hfsplus_block_free(sb: *mut super_block, offset: u32, count: u32) -> c_int;
}
// btree.c
extern "C" {
    pub fn hfs_btree_close(tree: *mut hfs_btree);
}
extern "C" {
    pub fn hfs_btree_write(tree: *mut hfs_btree) -> c_int;
}
extern "C" {
    pub fn hfs_bmap_reserve(tree: *mut hfs_btree, rsvd_nodes: u32) -> c_int;
}
extern "C" {
    pub fn hfs_bmap_free(node: *mut hfs_bnode);
}
// bnode.c
extern "C" {
    pub fn hfs_bnode_read(node: *mut hfs_bnode, buf: *mut c_void, off: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_read_u16(node: *mut hfs_bnode, off: u32) -> u16;
}
extern "C" {
    pub fn hfs_bnode_read_u8(node: *mut hfs_bnode, off: u32) -> u8;
}
extern "C" {
    pub fn hfs_bnode_read_key(node: *mut hfs_bnode, key: *mut c_void, off: u32);
}
extern "C" {
    pub fn hfs_bnode_write(node: *mut hfs_bnode, buf: *mut c_void, off: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_write_u16(node: *mut hfs_bnode, off: u32, data: u16);
}
extern "C" {
    pub fn hfs_bnode_clear(node: *mut hfs_bnode, off: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_move(node: *mut hfs_bnode, dst: u32, src: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_dump(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_unlink(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_unhash(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_free(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_get(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_put(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_need_zeroout(tree: *mut hfs_btree) -> bool;
}
// brec.c
extern "C" {
    pub fn hfs_brec_lenoff(node: *mut hfs_bnode, rec: u16, off: *mut u16) -> u16;
}
extern "C" {
    pub fn hfs_brec_keylen(node: *mut hfs_bnode, rec: u16) -> u16;
}
extern "C" {
    pub fn hfs_brec_insert(fd: *mut hfs_find_data, entry: *mut c_void, entry_len: u32) -> c_int;
}
extern "C" {
    pub fn hfs_brec_remove(fd: *mut hfs_find_data) -> c_int;
}
// bfind.c
extern "C" {
    pub fn hfs_find_init(tree: *mut hfs_btree, fd: *mut hfs_find_data) -> c_int;
}
extern "C" {
    pub fn hfs_find_exit(fd: *mut hfs_find_data);
}
extern "C" {
    pub fn hfs_brec_find(fd: *mut hfs_find_data, do_key_compare: search_strategy_t) -> c_int;
}
extern "C" {
    pub fn hfs_brec_read(fd: *mut hfs_find_data, rec: *mut c_void, rec_len: u32) -> c_int;
}
extern "C" {
    pub fn hfs_brec_goto(fd: *mut hfs_find_data, cnt: c_int) -> c_int;
}
// catalog.c
extern "C" {
    pub fn hfsplus_cat_set_perms(inode: *mut inode, perms: *mut hfsplus_perm);
}
extern "C" {
    pub fn hfsplus_delete_cat(cnid: u32, dir: *mut inode, str: *const qstr) -> c_int;
}
// dir.c
// extents.c
extern "C" {
    pub fn hfsplus_ext_write_extent(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn hfsplus_file_extend(inode: *mut inode, zeroout: bool) -> c_int;
}
extern "C" {
    pub fn hfsplus_file_truncate(inode: *mut inode);
}
// inode.c
extern "C" {
    pub fn hfsplus_delete_inode(inode: *mut inode);
}
extern "C" {
    pub fn hfsplus_cat_read_inode(inode: *mut inode, fd: *mut hfs_find_data) -> c_int;
}
extern "C" {
    pub fn hfsplus_cat_write_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn hfsplus_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
// ioctl.c
extern "C" {
    pub fn hfsplus_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// options.c
extern "C" {
    pub fn hfsplus_fill_defaults(opts: *mut hfsplus_sb_info);
}
extern "C" {
    pub fn hfsplus_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
}
extern "C" {
    pub fn hfsplus_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int;
}
// part_tbl.c
// super.c
extern "C" {
    pub fn hfsplus_mark_mdb_dirty(sb: *mut super_block);
}
extern "C" {
    pub fn hfsplus_prepare_volume_header_for_commit(vhdr: *mut hfsplus_vh);
}
extern "C" {
    pub fn hfsplus_commit_superblock(sb: *mut super_block) -> c_int;
}
// tables.c
// unicode.c
extern "C" {
    pub fn hfsplus_hash_dentry(dentry: *const dentry, str: *mut qstr) -> c_int;
}
// wrapper.c
extern "C" {
    pub fn hfsplus_read_wrapper(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn hfsplus_brec_read_cat(fd: *mut hfs_find_data, entry: *mut hfsplus_cat_entry) -> c_int;
}
//
// time helpers: convert between 1904-base and 1970-base timestamps
//
// HFS+ implementations are highly inconsistent, this one matches the
// traditional behavior of 64-bit Linux, giving the most useful
// time range between 1970 and 2106, by treating any on-disk timestamp
// under HFSPLUS_UTC_OFFSET (Jan 1 1970) as a time between 2040 and 2106.
//

extern "C" {
    pub fn cpu_to_be32(HFSPLUS_UTC_OFFSET: lower_32_bits(ut) +) -> return;
}
// compatibility

