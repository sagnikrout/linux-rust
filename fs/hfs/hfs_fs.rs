//! Automatically rewritten from C Header to Rust Module
//! Source: fs/hfs/hfs_fs.h
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


//
// linux/fs/hfs/hfs_fs.h
//
// Copyright (C) 1995-1997  Paul H. Hargrove
// (C) 2003 Ardis Technologies <roman@ardistech.com>
// This file may be distributed under the terms of the GNU General Public License.
//

//
// struct hfs_inode_info
//
// The HFS-specific part of a Linux (struct inode)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_inode_info {
    pub opencnt: core::sync::atomic::AtomicI32,
    pub flags: c_uint,
// to deal with localtime ugliness
    pub tz_secondswest: c_int,
    pub cat_key: hfs_cat_key,
    pub rsrc_inode: *mut inode,
    pub extents_lock: mutex,
    pub clump_blocks: u16 alloc_blocks,,
    pub fs_blocks: sector_t,
// Allocation extents from catlog record or volume header
    pub first_extents: hfs_extent_rec,
    pub first_blocks: u16,
    pub cached_extents: hfs_extent_rec,
    pub cached_blocks: u16 cached_start,,
    pub phys_size: loff_t,
    pub vfs_inode: inode,
}

pub const HFS_FLG_RSRC: c_uint = 0x0001;
pub const HFS_FLG_EXT_DIRTY: c_uint = 0x0002;
pub const HFS_FLG_EXT_NEW: c_uint = 0x0004;

//
// struct hfs_sb_info
//
// The HFS-specific part of a Linux (struct super_block)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_sb_info {
    pub /: *mut *mut mutex mdb_lock; / MDB operations lock,
    pub hfs_buffer: *mut *mut *mut buffer_head mdb_bh; / The,
    pub MDB: *mut *mut unsigned int mdb_offset; / byte offset of the,
    pub /: *mut *mut *mut hfs_mdb mdb; / in-memory copy of the MDB,
    pub holding: *mut *mut *mut buffer_head alt_mdb_bh; / The hfs_buffer,
    pub alternate: *mut *mut unsigned int alt_mdb_offset; / byte offset of the,
    pub the: *mut *mut *mut hfs_mdb alt_mdb; / in-memory copy of,
    pub the: *mut *mut *mut __be32 bitmap; / The page holding,
    pub about: *mut *mut *mut hfs_btree ext_tree; / Information,
    pub about: *mut *mut *mut hfs_btree cat_tree; / Information,
    pub of: *mut *mut atomic64_t file_count; / The number,
    pub of: *mut *mut atomic64_t folder_count; / The number,
    pub available: *mut *mut atomic64_t next_id; / The next,
    pub allocation: *mut *mut u32 clumpablks; / The number of,
    pub 512-byte: *mut *mut u32 fs_start; / The first,
    pub part_start: u32,
    pub of: *mut *mut u16 root_files; / The number,
    pub of: *mut *mut u16 root_dirs; / The number,
    pub of: *mut *mut u16 fs_ablocks; / The number,
    pub unused: *mut *mut u16 free_ablocks; / the number of,
    pub an: *mut *mut u32 alloc_blksz; / The size of,
    pub when: *mut *mut int s_quiet; / Silent failure,
    pub /: *mut *mut __be32 s_type; / Type for new files,
    pub /: *mut *mut __be32 s_creator; / Creator for new files,
    pub the: *mut *mut umode_t s_file_umask; / The umask applied to,
    pub the: *mut *mut umode_t s_dir_umask; / The umask applied to,
    pub /: *mut *mut kuid_t s_uid; / The uid of all files,
    pub /: *mut *mut kgid_t s_gid; / The gid of all files,
    pub part: int session,,
    pub nls_disk: *mut *mut nls_table nls_io,,
    pub bitmap_lock: mutex,
    pub flags: c_ulong,
    pub blockoffset: u16,
    pub fs_div: c_int,
    pub sb: *mut super_block,
    pub /: *mut *mut int work_queued; / non-zero delayed work is queued,
    pub /: *mut *mut delayed_work mdb_work; / MDB flush delayed work,
    pub /: *mut *mut spinlock_t work_lock; / protects mdb_work and work_queued,
}

pub const HFS_FLG_BITMAP_DIRTY: c_int = 0;
pub const HFS_FLG_MDB_DIRTY: c_int = 1;
pub const HFS_FLG_ALT_MDB_DIRTY: c_int = 2;
// bitmap.c
extern "C" {
    pub fn hfs_vbm_search_free(sb: *mut super_block, goal: u32, num_bits: *mut u32) -> u32;
}
extern "C" {
    pub fn hfs_clear_vbm_bits(sb: *mut super_block, start: u16, count: u16) -> c_int;
}
// catalog.c
extern "C" {
    pub fn hfs_cat_keycmp(key1: *const btree_key, key2: *const btree_key) -> c_int;
}
extern "C" {
    pub fn hfs_cat_delete(cnid: u32, dir: *mut inode, str: *const qstr) -> c_int;
}
//
// Validate the CNID of a catalog record.
//
// dir.c
// extent.c
extern "C" {
    pub fn hfs_ext_keycmp(key1: *const btree_key, key2: *const btree_key) -> c_int;
}
extern "C" {
    pub fn hfs_ext_find_block(ext: *mut hfs_extent, off: u16) -> u16;
}
extern "C" {
    pub fn hfs_ext_write_extent(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn hfs_extend_file(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn hfs_file_truncate(inode: *mut inode);
}
// inode.c
extern "C" {
    pub fn hfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn hfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn hfs_evict_inode(inode: *mut inode);
}
extern "C" {
    pub fn hfs_delete_inode(inode: *mut inode);
}
// attr.c
// mdb.c
extern "C" {
    pub fn is_hfs_cnid_counts_valid(sb: *mut super_block) -> bool;
}
extern "C" {
    pub fn hfs_mdb_get(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn hfs_mdb_commit(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn hfs_mdb_close(sb: *mut super_block);
}
extern "C" {
    pub fn hfs_mdb_put(sb: *mut super_block);
}
// part_tbl.c
// string.c
extern "C" {
    pub fn hfs_hash_dentry(dentry: *const dentry, this: *mut qstr) -> c_int;
}
// trans.c
// super.c
extern "C" {
    pub fn hfs_mark_mdb_dirty(sb: *mut super_block);
}
//
// There are two time systems.  Both are based on seconds since
// a particular time/date.
// Unix:	signed little-endian since 00:00 GMT, Jan. 1, 1970
// mac:	unsigned big-endian since 00:00 GMT, Jan. 1, 1904
//
// HFS implementations are highly inconsistent, this one matches the
// traditional behavior of 64-bit Linux, giving the most useful
// time range between 1970 and 2106, by treating any on-disk timestamp
// under HFS_UTC_OFFSET (Jan 1 1970) as a time between 2040 and 2106.
//

extern "C" {
    pub fn cpu_to_be32(HFS_UTC_OFFSET: lower_32_bits(ut) +) -> return;
}

