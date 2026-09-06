//! Automatically rewritten from C Header to Rust Module
//! Source: fs/minix/minix.h
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

pub const MINIX_V1: c_uint = 0x0001		/* original minix fs */;
pub const MINIX_V2: c_uint = 0x0002		/* minix V2 fs */;
pub const MINIX_V3: c_uint = 0x0003		/* minix V3 fs */;
//
// minix fs inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_inode_info {
    pub i1_data: [__u16; 16],
    pub i2_data: [__u32; 16],
    pub u: },
    pub i_metadata_bhs: mapping_metadata_bhs,
    pub vfs_inode: inode,
}

//
// minix super-block data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_sb_info {
    pub s_ninodes: c_ulong,
    pub s_nzones: c_ulong,
    pub s_imap_blocks: c_ulong,
    pub s_zmap_blocks: c_ulong,
    pub s_firstdatazone: c_ulong,
    pub s_log_zone_size: c_ulong,
    pub s_dirsize: c_int,
    pub s_namelen: c_int,
    pub s_imap: *mut *mut *mut buffer_head,
    pub s_zmap: *mut *mut *mut buffer_head,
    pub s_sbh: *mut *mut buffer_head,
    pub s_ms: *mut *mut minix_super_block,
    pub s_mount_state: c_ushort,
    pub s_version: c_ushort,
}

extern "C" {
    pub fn minix_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn minix_count_free_inodes(sb: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn minix_new_block(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn minix_free_block(inode: *mut inode, block: c_ulong);
}
extern "C" {
    pub fn minix_count_free_blocks(sb: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn minix_prepare_chunk(folio: *mut folio, pos: loff_t, len: unsigned) -> c_int;
}
extern "C" {
    pub fn V1_minix_truncate(: *mut inode);
}
extern "C" {
    pub fn V2_minix_truncate(: *mut inode);
}
extern "C" {
    pub fn minix_truncate(: *mut inode);
}
extern "C" {
    pub fn minix_set_inode(: *mut inode, _arg: dev_t);
}
extern "C" {
    pub fn V1_minix_get_block(: *mut inode, _arg: c_long, : *mut buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn V2_minix_get_block(: *mut inode, _arg: c_long, : *mut buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn V1_minix_blocks(_arg: loff_t, : *mut super_block) -> unsigned;
}
extern "C" {
    pub fn V2_minix_blocks(_arg: loff_t, : *mut super_block) -> unsigned;
}
extern "C" {
    pub fn minix_add_link(dentry*: *mut struct, inode*: *mut struct) -> c_int;
}
extern "C" {
    pub fn minix_delete_entry(: *mut minix_dir_entry, : *mut folio) -> c_int;
}
extern "C" {
    pub fn minix_make_empty(inode*: *mut struct, inode*: *mut struct) -> c_int;
}
extern "C" {
    pub fn minix_empty_dir(inode*: *mut struct) -> c_int;
}
extern "C" {
    pub fn minix_inode_by_name(dentry*: *mut struct) -> ino_t;
}
extern "C" {
    pub fn container_of(_arg: inode, minix_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn DIV_ROUND_UP_POW2(_arg: bits, 8: *mut *mut blocksize) -> return;
}

//
// big-endian 32 or 64 bit indexed bitmaps on big-endian system or
// little-endian bitmaps on little-endian system
//

//
// big-endian 16bit indexed bitmaps
//

//
// little-endian bitmaps
//

