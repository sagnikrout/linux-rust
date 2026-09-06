//! Automatically rewritten from C Header to Rust Module
//! Source: fs/exfat/exfat_fs.h
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
// Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
//

pub const EXFAT_ROOT_INO: c_int = 1;
//
// exfat error flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exfat_error_mode {
    EXFAT_ERRORS_CONT,	/* ignore error and continue */
    EXFAT_ERRORS_PANIC,	/* panic on error */
    EXFAT_ERRORS_RO,	/* remount r/o on error */
}

//
// exfat nls lossy flag
//
pub const EXFAT_HASH_BITS: c_int = 8;

//
// Type Definitions
//
pub const ES_2_ENTRIES: c_int = 2;
pub const ES_ALL_ENTRIES: c_int = 0;
pub const ES_IDX_FILE: c_int = 0;
pub const ES_IDX_STREAM: c_int = 1;
pub const ES_IDX_FIRST_FILENAME: c_int = 2;

pub const DIR_DELETED: c_uint = 0xFFFFFFF7;
// type values
pub const TYPE_UNUSED: c_uint = 0x0000;
pub const TYPE_DELETED: c_uint = 0x0001;
pub const TYPE_INVALID: c_uint = 0x0002;
pub const TYPE_CRITICAL_PRI: c_uint = 0x0100;
pub const TYPE_BITMAP: c_uint = 0x0101;
pub const TYPE_UPCASE: c_uint = 0x0102;
pub const TYPE_VOLUME: c_uint = 0x0103;
pub const TYPE_DIR: c_uint = 0x0104;
pub const TYPE_FILE: c_uint = 0x011F;
pub const TYPE_CRITICAL_SEC: c_uint = 0x0200;
pub const TYPE_STREAM: c_uint = 0x0201;
pub const TYPE_EXTEND: c_uint = 0x0202;
pub const TYPE_ACL: c_uint = 0x0203;
pub const TYPE_BENIGN_PRI: c_uint = 0x0400;
pub const TYPE_GUID: c_uint = 0x0401;
pub const TYPE_PADDING: c_uint = 0x0402;
pub const TYPE_ACLTAB: c_uint = 0x0403;
pub const TYPE_BENIGN_SEC: c_uint = 0x0800;
pub const TYPE_VENDOR_EXT: c_uint = 0x0801;
pub const TYPE_VENDOR_ALLOC: c_uint = 0x0802;

pub const EXFAT_MIN_SUBDIR: c_int = 2;

//
// helpers for fat entry.
//

//
// helpers for bitmap.
//

// 19 entries = 1 file entry + 1 stream entry + 17 filename entries

//
// 19 entries x 32 bytes/entry = 608 bytes.
// The 608 bytes are in 3 sectors at most (even 512 Byte sector).
//

// Superblock flags
pub const EXFAT_FLAGS_SHUTDOWN: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_dentry_namebuf {
    pub lfn: *mut c_char,
    pub /: *mut *mut int lfnbuf_len; / usually MAX_UNINAME_BUF_SIZE,
}

// unicode name structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_uni_name {
// +3 for null and for converting
    pub 3]: unsigned short name[MAX_NAME_LENGTH +,
    pub name_hash: u16,
    pub name_len: c_uchar,
}

// directory structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_chain {
    pub dir: c_uint,
    pub size: c_uint,
    pub flags: c_uchar,
}

// first empty entry hint information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_hint_femp {
// entry index of a directory
    pub eidx: c_int,
// count of continuous empty entry
    pub count: c_int,
// the cluster that first empty slot exists in
    pub cur: exfat_chain,
}

// hint structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_hint {
    pub clu: c_uint,
    pub /: *mut *mut unsigned int off; / cluster offset,
    pub /: *mut *mut int eidx; / entry index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_entry_set_cache {
    pub sb: *mut super_block,
    pub start_off: c_uint,
    pub num_bh: c_int,
    pub __bh: [*mut buffer_head; DIR_CACHE_SIZE],
    pub bh: *mut buffer_head,
    pub num_entries: c_uint,
    pub modified: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_dir_entry {
// the cluster where file dentry is located
    pub dir: exfat_chain,
// the index of file dentry in ->dir
    pub entry: c_int,
    pub type: c_uint,
    pub start_clu: c_uint,
    pub flags: c_uchar,
    pub attr: c_ushort,
    pub size: loff_t,
    pub valid_size: loff_t,
    pub num_subdirs: c_uint,
    pub atime: timespec64,
    pub mtime: timespec64,
    pub crtime: timespec64,
    pub namebuf: exfat_dentry_namebuf,
}

//
// exfat mount in-memory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_mount_options {
    pub fs_uid: kuid_t,
    pub fs_gid: kgid_t,
    pub fs_fmask: c_ushort,
    pub fs_dmask: c_ushort,
// permission for setting the [am]time
    pub allow_utime: c_ushort,
// charset for filename input/display
    pub iocharset: *mut c_char,
// on error: continue, panic, remount-ro
    pub errors: exfat_error_mode,
    pub /: *mut *mut keep_last_dots:1; / Keep trailing periods in paths,
    pub /: *mut *mut int time_offset; / Offset of timestamps from UTC (in minutes),
// Support creating zero-size directory, default: false
    pub zero_size_dir: bool,
}

//
// EXFAT file system superblock in-memory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_sb_info {
    pub /: *mut *mut unsigned long long num_sectors; / num of sectors in volume,
    pub /: *mut *mut unsigned int num_clusters; / num of clusters in volume,
    pub /: *mut *mut unsigned int cluster_size; / cluster size in bytes,
    pub cluster_size_bits: c_uint,
    pub /: *mut *mut unsigned int sect_per_clus; / cluster size in sectors,
    pub sect_per_clus_bits: c_uint,
    pub /: *mut *mut unsigned long long FAT1_start_sector; / FAT1 start sector,
    pub /: *mut *mut unsigned long long FAT2_start_sector; / FAT2 start sector,
    pub /: *mut *mut unsigned long long data_start_sector; / data area start sector,
    pub data_start_bytes: c_ulonglong,
    pub /: *mut *mut unsigned int num_FAT_sectors; / num of FAT sectors,
    pub /: *mut *mut unsigned int root_dir; / root dir cluster,
    pub /: *mut *mut unsigned int dentries_per_clu; / num of dentries per cluster,
    pub /: *mut *mut unsigned int vol_flags; / volume flags,
    pub /: *mut *mut unsigned int vol_flags_persistent; / volume flags to retain,
    pub /: *mut *mut *mut buffer_head boot_bh; / buffer_head of BOOT sector,
    pub /: *mut *mut unsigned int map_clu; / allocation bitmap start cluster,
    pub /: *mut *mut unsigned int map_sectors; / num of allocation bitmap sectors,
    pub /: *mut *mut *mut *mut buffer_head vol_amap; / allocation bitmap,
    pub /: *mut *mut *mut unsigned short vol_utbl; / upcase table,
    pub /: *mut *mut unsigned int clu_srch_ptr; / cluster search pointer,
    pub /: *mut *mut unsigned int used_clusters; / number of used clusters,
    pub /: *mut *mut unsigned long s_exfat_flags; / Exfat superblock flags,
    pub /: *mut *mut mutex s_lock; / superblock lock,
    pub /: *mut *mut mutex bitmap_lock; / bitmap lock,
    pub options: exfat_mount_options,
    pub /: *mut *mut *mut nls_table nls_io; / Charset used for input and display,
    pub ratelimit: ratelimit_state,
    pub inode_hash_lock: spinlock_t,
    pub inode_hashtable: [hlist_head; EXFAT_HASH_SIZE],
    pub rcu: rcu_head,
}

pub const EXFAT_CACHE_VALID: c_int = 0;
//
// EXFAT file system inode in-memory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exfat_inode_info {
// the cluster where file dentry is located
    pub dir: exfat_chain,
// the index of file dentry in ->dir
    pub entry: c_int,
    pub type: c_uint,
    pub attr: c_ushort,
    pub start_clu: c_uint,
    pub flags: c_uchar,
//
// the copy of low 32bit of i_version to check
// the validation of hint_stat.
//
    pub version: c_uint,
// hint for cluster last accessed
    pub hint_bmap: exfat_hint,
// hint for entry index we try to lookup next time
    pub hint_stat: exfat_hint,
// hint for first empty entry
    pub hint_femp: exfat_hint_femp,
    pub cache_lru_lock: spinlock_t,
    pub cache_lru: list_head,
    pub nr_caches: c_int,
// for avoiding the race between alloc and free
    pub cache_valid_id: c_uint,
// on-disk position of directory entry or 0
    pub i_pos: loff_t,
    pub valid_size: loff_t,
// block-aligned size zeroed in the page cache (>= valid_size)
    pub zeroed_size: loff_t,
// hash by i_location
    pub i_hash_fat: hlist_node,
    pub vfs_inode: inode,
// File creation time
    pub i_crtime: timespec64,
}

extern "C" {
    pub fn container_of(_arg: inode, exfat_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn test_bit(_arg: EXFAT_FLAGS_SHUTDOWN, _arg: &EXFAT_SB(sb)->s_exfat_flags) -> return;
}
//
// If ->i_mode can't hold 0222 (i.e. ATTR_RO), we use ->i_attrs to
// save ATTR_RO instead of ->i_mode.
//
// If it's directory and !sbi->options.rodir, ATTR_RO isn't read-only
// bit, it's just used as flag for app.
//
// Convert attribute bits and a mask to the UNIX mode.
// Return the FAT attribute byte for this inode
//
// helpers for cluster size to byte conversion.
//
// helpers for block size to byte conversion.
//
// helpers for block size to dentry size conversion.
//
// helpers for cluster size to dentry size conversion.
//
// super.c
extern "C" {
    pub fn exfat_set_volume_dirty(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn exfat_clear_volume_dirty(sb: *mut super_block) -> c_int;
}
// fatent.c

extern "C" {
    pub fn exfat_free_cluster(inode: *mut inode, p_chain: *mut exfat_chain) -> c_int;
}
extern "C" {
    pub fn exfat_zeroed_cluster(dir: *mut inode, clu: c_uint) -> c_int;
}
// balloc.c
extern "C" {
    pub fn exfat_load_bitmap(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn exfat_free_bitmap(sbi: *mut exfat_sb_info);
}
extern "C" {
    pub fn exfat_set_bitmap(sb: *mut super_block, clu: c_uint, sync: bool) -> c_int;
}
extern "C" {
    pub fn exfat_clear_bitmap(sb: *mut super_block, clu: c_uint, sync: bool) -> c_int;
}
extern "C" {
    pub fn exfat_test_bitmap(sb: *mut super_block, clu: c_uint) -> bool;
}
extern "C" {
    pub fn exfat_find_free_bitmap(sb: *mut super_block, clu: c_uint) -> c_uint;
}
extern "C" {
    pub fn exfat_count_used_clusters(sb: *mut super_block, ret_count: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn exfat_trim_fs(inode: *mut inode, range: *mut fstrim_range) -> c_int;
}
// file.c
extern "C" {
    pub fn __exfat_truncate(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn exfat_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn exfat_file_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn exfat_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn exfat_force_shutdown(sb: *mut super_block, flags: u32) -> c_int;
}
// namei.c
// cache.c
extern "C" {
    pub fn exfat_cache_init() -> c_int;
}
extern "C" {
    pub fn exfat_cache_shutdown();
}
extern "C" {
    pub fn exfat_cache_inval_inode(inode: *mut inode);
}
// dir.c
extern "C" {
    pub fn exfat_get_entry_type(p_entry: *mut exfat_dentry) -> c_uint;
}
extern "C" {
    pub fn exfat_update_dir_chksum(es: *mut exfat_entry_set_cache);
}
extern "C" {
    pub fn exfat_calc_num_entries(p_uniname: *mut exfat_uni_name) -> c_int;
}
extern "C" {
    pub fn exfat_alloc_new_dir(inode: *mut inode, clu: *mut exfat_chain) -> c_int;
}

extern "C" {
    pub fn exfat_put_dentry_set(es: *mut exfat_entry_set_cache, sync: c_int) -> c_int;
}
extern "C" {
    pub fn exfat_count_dir_entries(sb: *mut super_block, p_dir: *mut exfat_chain) -> c_int;
}
// inode.c
extern "C" {
    pub fn exfat_sync_inode(inode: *mut inode);
}
extern "C" {
    pub fn exfat_hash_inode(inode: *mut inode, i_pos: loff_t);
}
extern "C" {
    pub fn exfat_unhash_inode(inode: *mut inode);
}
extern "C" {
    pub fn __exfat_write_inode(inode: *mut inode, sync: c_int) -> c_int;
}
extern "C" {
    pub fn exfat_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn exfat_evict_inode(inode: *mut inode);
}
// exfat/nls.c
extern "C" {
    pub fn exfat_toupper(sb: *mut super_block, a: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn exfat_create_upcase_table(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn exfat_free_upcase_table(sbi: *mut exfat_sb_info);
}
// exfat/misc.c

// expand to pr_*() with prefix

extern "C" {
    pub fn exfat_truncate_atime(ts: *mut timespec64);
}
extern "C" {
    pub fn exfat_truncate_inode_atime(inode: *mut inode);
}
extern "C" {
    pub fn exfat_calc_chksum16(data: *mut c_void, len: c_int, chksum: u16, type: c_int) -> u16;
}
extern "C" {
    pub fn exfat_calc_chksum32(data: *mut c_void, len: c_int, chksum: u32, type: c_int) -> u32;
}
extern "C" {
    pub fn exfat_update_bh(bh: *mut buffer_head, sync: c_int) -> c_int;
}
extern "C" {
    pub fn exfat_update_bhs(bhs: *mut buffer_head, nr_bhs: c_int, sync: c_int) -> c_int;
}
extern "C" {
    pub fn exfat_chain_dup(dup: *mut exfat_chain, ec: *mut exfat_chain);
}
