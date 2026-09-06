//! Automatically rewritten from C Header to Rust Module
//! Source: fs/befs/befs_fs_types.h
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
// fs/befs/befs_fs_types.h
//
// Copyright (C) 2001 Will Dyson (will@cs.earlham.edu)
//
// from linux/include/linux/befs_fs.h
//
// Copyright (C) 1999 Makoto Kato (m_kato@ga2.so-net.ne.jp)
//

//
// Max name lengths of BFS
//
pub const BEFS_NAME_LEN: c_int = 255;
pub const BEFS_SYMLINK_LEN: c_int = 144;
pub const BEFS_NUM_DIRECT_BLOCKS: c_int = 12;
pub const B_OS_NAME_LENGTH: c_int = 32;
// The datastream blocks mapped by the double-indirect
// block are always 4 fs blocks long.
// This eliminates the need for linear searches among
// the potentially huge number of indirect blocks
//
// Err. Should that be 4 fs blocks or 4k???
// It matters on large blocksize volumes
//
pub const BEFS_DBLINDIR_BRUN_LEN: c_int = 4;
//
// Flags of superblock
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum super_flags {
    BEFS_BYTESEX_BE,
    BEFS_BYTESEX_LE,
    BEFS_CLEAN = 0x434c454e,
    BEFS_DIRTY = 0x44495254,
    BEFS_SUPER_MAGIC1 = 0x42465331,	/* BFS1 */
    BEFS_SUPER_MAGIC2 = 0xdd121031,
    BEFS_SUPER_MAGIC3 = 0x15b6830e,
}

pub const BEFS_BYTEORDER_NATIVE: c_uint = 0x42494745;

//
// Flags of inode
//
pub const BEFS_INODE_MAGIC1: c_uint = 0x3bbe0ad9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inode_flags {
    BEFS_INODE_IN_USE = 0x00000001,
    BEFS_ATTR_INODE = 0x00000004,
    BEFS_INODE_LOGGED = 0x00000008,
    BEFS_INODE_DELETED = 0x00000010,
    BEFS_LONG_SYMLINK = 0x00000040,
    BEFS_PERMANENT_FLAG = 0x0000ffff,
    BEFS_INODE_NO_CREATE = 0x00010000,
    BEFS_INODE_WAS_WRITTEN = 0x00020000,
    BEFS_NO_TRANSACTION = 0x00040000,
}

//
// On-Disk datastructures of BeFS
//
pub type fs64 = u64 ;
pub type fs32 = u32 ;
pub type fs16 = u16 ;
pub type befs_off_t = u64;
pub type befs_time_t = fs64;
// Block runs
pub type befs_disk_inode_addr = befs_disk_block_run;
pub type befs_inode_addr = befs_block_run;
//
// The Superblock Structure
//
// Note: the indirect and dbl_indir block_runs may
// be longer than one block!
//
// Attribute
// Inode structure
//
// B+tree superblock
//
pub const BEFS_BTREE_MAGIC: c_uint = 0x69f6c2e8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btree_types {
    BTREE_STRING_TYPE = 0,
    BTREE_INT32_TYPE = 1,
    BTREE_UINT32_TYPE = 2,
    BTREE_INT64_TYPE = 3,
    BTREE_UINT64_TYPE = 4,
    BTREE_FLOAT_TYPE = 5,
    BTREE_DOUBLE_TYPE = 6
}

//
// Header structure of each btree node
//
