//! Automatically rewritten from C Header to Rust Module
//! Source: fs/zonefs/zonefs.h
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
// Simple zone file system for zoned block devices.
//
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//

//
// Maximum length of file names: this only needs to be large enough to fit
// the zone group directory names and a decimal zone number for file names.
// 16 characters is plenty.
//
pub const ZONEFS_NAME_MAX: c_int = 16;
//
// Zone types: ZONEFS_ZTYPE_SEQ is used for all sequential zone types
// defined in linux/blkzoned.h, that is, BLK_ZONE_TYPE_SEQWRITE_REQ and
// BLK_ZONE_TYPE_SEQWRITE_PREF.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zonefs_ztype {
    ZONEFS_ZTYPE_CNV,
    ZONEFS_ZTYPE_SEQ,
    ZONEFS_ZTYPE_MAX,
}

//
// In-memory per-file inode zone data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonefs_zone {
// Zone state flags
    pub z_flags: c_uint,
// Zone start sector (512B unit)
    pub z_sector: sector_t,
// Zone size (bytes)
    pub z_size: loff_t,
// Zone capacity (file maximum size, bytes)
    pub z_capacity: loff_t,
// Write pointer offset in the zone (sequential zones only, bytes)
    pub z_wpoffset: loff_t,
// Saved inode uid, gid and access rights
    pub z_mode: umode_t,
    pub z_uid: kuid_t,
    pub z_gid: kgid_t,
}

//
// In memory zone group information: all zones of a group are exposed
// as files, one file per zone.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonefs_zone_group {
    pub g_inode: *mut inode,
    pub g_nr_zones: c_uint,
    pub g_zones: *mut zonefs_zone,
}

//
// In-memory inode data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonefs_inode_info {
    pub i_vnode: inode,
//
// To serialise fully against both syscall and mmap based IO and
// sequential file truncation, two locks are used. For serializing
// zonefs_seq_file_truncate() against zonefs_iomap_begin(), that is,
// file truncate operations against block mapping, i_truncate_mutex is
// used. i_truncate_mutex also protects against concurrent accesses
// and changes to the inode private data, and in particular changes to
// a sequential file size on completion of direct IO writes.
// Serialization of mmap read IOs with truncate and syscall IO
// operations is done with invalidate_lock in addition to
// i_truncate_mutex.  Only zonefs_seq_file_truncate() takes both lock
// (invalidate_lock first, i_truncate_mutex second).
//
    pub i_truncate_mutex: mutex,
// guarded by i_truncate_mutex
    pub i_wr_refcnt: c_uint,
}

extern "C" {
    pub fn container_of(_arg: inode, zonefs_inode_info: struct, _arg: i_vnode) -> return;
}
extern "C" {
    pub fn zonefs_zone_is_cnv(_arg: zonefs_inode_zone(inode)) -> return;
}
extern "C" {
    pub fn zonefs_zone_is_seq(_arg: zonefs_inode_zone(inode)) -> return;
}
//
// On-disk super block (block 0).
//
pub const ZONEFS_LABEL_LEN: c_int = 64;
pub const ZONEFS_UUID_SIZE: c_int = 16;
pub const ZONEFS_SUPER_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonefs_super {
// Magic number
    pub s_magic: __le32,
// Checksum
    pub s_crc: __le32,
// Volume label
    pub s_label: [c_char; ZONEFS_LABEL_LEN],
// 128-bit uuid
    pub s_uuid: [__u8; ZONEFS_UUID_SIZE],
// Features
    pub s_features: __le64,
// UID/GID to use for files
    pub s_uid: __le32,
    pub s_gid: __le32,
// File permissions
    pub s_perm: __le32,
// Padding to ZONEFS_SUPER_SIZE bytes
    pub s_reserved: [__u8; 3988],
    pub __packed: },
//
// Feature flags: specified in the s_features field of the on-disk super
// block struct zonefs_super and in-memory in the s_feartures field of
// struct zonefs_sb_info.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zonefs_features {
//
// Aggregate contiguous conventional zones into a single file.
//
    ZONEFS_F_AGGRCNV = 1ULL << 0,
//
// Use super block specified UID for files instead of default 0.
//
    ZONEFS_F_UID = 1ULL << 1,
//
// Use super block specified GID for files instead of default 0.
//
    ZONEFS_F_GID = 1ULL << 2,
//
// Use super block specified file permissions instead of default 640.
//
    ZONEFS_F_PERM = 1ULL << 3,
}

//
// Mount options for zone write pointer error handling.
//

//
// In-memory Super block information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonefs_sb_info {
    pub s_mount_opts: c_ulong,
    pub s_lock: spinlock_t,
    pub s_features: c_ulonglong,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub s_perm: umode_t,
    pub s_uuid: uuid_t,
    pub s_zone_sectors_shift: c_uint,
    pub s_zgroup: [zonefs_zone_group; ZONEFS_ZTYPE_MAX],
    pub s_blocks: loff_t,
    pub s_used_blocks: loff_t,
    pub s_max_wro_seq_files: c_uint,
    pub s_wro_seq_files: core::sync::atomic::AtomicI32,
    pub s_max_active_seq_files: c_uint,
    pub s_active_seq_files: core::sync::atomic::AtomicI32,
    pub s_sysfs_registered: bool,
    pub s_kobj: kobject,
    pub s_kobj_unregister: completion,
}

// In super.c
extern "C" {
    pub fn zonefs_inode_account_active(inode: *mut inode);
}
extern "C" {
    pub fn zonefs_inode_zone_mgmt(inode: *mut inode, op: req_op) -> c_int;
}
extern "C" {
    pub fn zonefs_i_size_write(inode: *mut inode, isize: loff_t);
}
extern "C" {
    pub fn zonefs_update_stats(inode: *mut inode, new_isize: loff_t);
}
extern "C" {
    pub fn __zonefs_io_error(inode: *mut inode, write: bool);
}
// In super.c
// In file.c
extern "C" {
    pub fn zonefs_file_truncate(inode: *mut inode, isize: loff_t) -> c_int;
}
// In sysfs.c
extern "C" {
    pub fn zonefs_sysfs_register(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn zonefs_sysfs_unregister(sb: *mut super_block);
}
extern "C" {
    pub fn zonefs_sysfs_init() -> c_int;
}
extern "C" {
    pub fn zonefs_sysfs_exit();
}
