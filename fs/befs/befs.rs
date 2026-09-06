//! Automatically rewritten from C Header to Rust Module
//! Source: fs/befs/befs.h
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
// befs.h
//
// Copyright (C) 2001-2002 Will Dyson <will_dyson@pobox.com>
// Copyright (C) 1999 Makoto Kato (m_kato@ga2.so-net.ne.jp)
//

// used in debug.c

pub type befs_blocknr_t = u64;
//
// BeFS in memory structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct befs_mount_options {
    pub gid: kgid_t,
    pub uid: kuid_t,
    pub use_gid: c_int,
    pub use_uid: c_int,
    pub debug: c_int,
    pub iocharset: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct befs_sb_info {
    pub magic1: u32,
    pub block_size: u32,
    pub block_shift: u32,
    pub byte_order: c_int,
    pub num_blocks: befs_off_t,
    pub used_blocks: befs_off_t,
    pub inode_size: u32,
    pub magic2: u32,
// Allocation group information
    pub blocks_per_ag: u32,
    pub ag_shift: u32,
    pub num_ags: u32,
// State of the superblock
    pub flags: u32,
// Journal log entry
    pub log_blocks: befs_block_run,
    pub log_start: befs_off_t,
    pub log_end: befs_off_t,
    pub root_dir: befs_inode_addr,
    pub indices: befs_inode_addr,
    pub magic3: u32,
    pub mount_opts: befs_mount_options,
    pub nls: *mut nls_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct befs_inode_info {
    pub i_flags: u32,
    pub i_type: u32,
    pub i_inode_num: befs_inode_addr,
    pub i_parent: befs_inode_addr,
    pub i_attribute: befs_inode_addr,
    pub ds: befs_data_stream,
    pub symlink: [c_char; BEFS_SYMLINK_LEN],
    pub i_data: },
    pub vfs_inode: inode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum befs_err {
    BEFS_OK,
    BEFS_ERR,
    BEFS_BAD_INODE,
    BEFS_BT_END,
    BEFS_BT_EMPTY,
    BEFS_BT_MATCH,
    BEFS_BT_OVERFLOW,
    BEFS_BT_NOT_FOUND
}

//
// debug.c
extern "C" {
    pub fn befs_error(sb: *const super_block, fmt: *const c_char, ...);
}
extern "C" {
    pub fn befs_warning(sb: *const super_block, fmt: *const c_char, ...);
}
extern "C" {
    pub fn befs_debug(sb: *const super_block, fmt: *const c_char, ...);
}
extern "C" {
    pub fn befs_dump_super_block(sb: *const super_block, : *mut befs_super_block);
}
extern "C" {
    pub fn befs_dump_inode(sb: *const super_block, : *mut befs_inode);
}
extern "C" {
    pub fn befs_dump_index_entry(sb: *const super_block, : *mut befs_disk_btree_super);
}
extern "C" {
    pub fn befs_dump_index_node(sb: *const super_block, : *mut befs_btree_nodehead);
}
//
// Gets a pointer to the private portion of the super_block
// structure from the public part
//
extern "C" {
    pub fn container_of(_arg: inode, befs_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn BEFS_SB(sizeof(befs_disk_inode_addr: sb)->block_size /) -> return;
}

