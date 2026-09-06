//! Automatically rewritten from C Header to Rust Module
//! Source: fs/adfs/adfs.h
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

// Internal data structures for ADFS
pub const ADFS_FREE_FRAG: c_int = 0;
pub const ADFS_BAD_FRAG: c_int = 1;
pub const ADFS_ROOT_FRAG: c_int = 2;

// RISC OS 12-bit filetype is stored in load_address[19:8]

//
// adfs file system inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_inode_info {
    pub mmu_private: loff_t,
    pub /: *mut *mut __u32 parent_id; / parent indirect disc address,
    pub /: *mut *mut __u32 indaddr; / object indirect disc address,
    pub /: *mut *mut __u32 loadaddr; / RISC OS load address,
    pub /: *mut *mut __u32 execaddr; / RISC OS exec address,
    pub /: *mut *mut unsigned int attr; / RISC OS permissions,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn container_of(_arg: inode, adfs_inode_info: struct, _arg: vfs_inode) -> return;
}
//
// Forward-declare this
//
// ADFS file system superblock data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_sb_info {
    pub /: *mut *mut *mut adfs_discmap s_map; / bh list containing map,
    pub /: *const *const *const adfs_dir_ops s_dir; / directory operations,
}

//
// Directory handling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_dir {
    pub sb: *mut super_block,
    pub nr_buffers: c_int,
    pub bh: [*mut buffer_head; 4],
    pub bhs: *mut buffer_head,
    pub pos: c_uint,
    pub parent_id: __u32,
    pub dirhead: *mut adfs_dirheader,
    pub bighead: *mut adfs_bigdirheader,
}

//
// This is the overall maximum name length
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct object_info {
    pub /: *mut *mut __u32 parent_id; / parent object id,
    pub /: *mut *mut __u32 indaddr; / indirect disc addr,
    pub /: *mut *mut __u32 loadaddr; / load address,
    pub /: *mut *mut __u32 execaddr; / execution address,
    pub /: *mut *mut __u32 size; / size,
    pub /: *mut *mut __u8 attr; / RISC OS attributes,
    pub /: *mut *mut unsigned int name_len; / name length,
    pub /: *mut *mut char name[ADFS_MAX_NAME_LEN];/ file name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_dir_ops {
    pub dir): *mut unsigned int size, struct adfs_dir,
    pub ctx): *mut *mut *mut int (iterate)(struct adfs_dir dir, struct dir_context,
    pub fpos): *mut *mut *mut int (setpos)(struct adfs_dir dir, unsigned int,
    pub obj): *mut *mut *mut int (getnext)(struct adfs_dir dir, struct object_info,
    pub obj): *mut *mut *mut int (update)(struct adfs_dir dir, struct object_info,
    pub obj): *mut *mut *mut int (create)(struct adfs_dir dir, struct object_info,
    pub obj): *mut *mut *mut int (remove)(struct adfs_dir dir, struct object_info,
    pub dir): *mut *mut int (commit)(struct adfs_dir,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_discmap {
    pub dm_bh: *mut buffer_head,
    pub dm_startblk: __u32,
    pub dm_startbit: c_uint,
    pub dm_endbit: c_uint,
}

// Inode stuff
extern "C" {
    pub fn adfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
// map.c
extern "C" {
    pub fn adfs_map_lookup(sb: *mut super_block, frag_id: u32, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn adfs_map_statfs(sb: *mut super_block, buf: *mut kstatfs);
}
extern "C" {
    pub fn adfs_free_map(sb: *mut super_block);
}
// Misc

extern "C" {
    pub fn adfs_msg(sb: *mut super_block, pfx: *const c_char, fmt: *const c_char, ...);
}
// super.c
//
// Inodes and file operations
//
// dir_*.c
extern "C" {
    pub fn adfs_dir_relse(dir: *mut adfs_dir);
}
extern "C" {
    pub fn adfs_object_fixup(dir: *mut adfs_dir, obj: *mut object_info);
}
// file.c
//
// Calculate the address of a block in an object given the block offset
// and the object identity.
//
// The root directory ID should always be looked up in the map [3.4]
//
extern "C" {
    pub fn adfs_map_lookup(_arg: sb, 8: indaddr >>, _arg: block) -> return;
}
// Return the disc record from the map
