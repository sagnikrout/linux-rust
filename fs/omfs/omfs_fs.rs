//! Automatically rewritten from C Header to Rust Module
//! Source: fs/omfs/omfs_fs.h
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
// OMFS On-disk structures
pub const OMFS_MAGIC: c_uint = 0xC2993D87;
pub const OMFS_IMAGIC: c_uint = 0xD2;

pub const OMFS_NAMELEN: c_int = 256;
pub const OMFS_DIR_START: c_uint = 0x1b8;
pub const OMFS_EXTENT_START: c_uint = 0x1d0;
pub const OMFS_EXTENT_CONT: c_uint = 0x40;
pub const OMFS_XOR_COUNT: c_int = 19;
pub const OMFS_MAX_BLOCK_SIZE: c_int = 8192;
pub const OMFS_MAX_CLUSTER_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_super_block {
    pub s_fill1: [c_char; 256],
    pub /: *mut *mut __be64 s_root_block; / block number of omfs_root_block,
    pub /: *mut *mut __be64 s_num_blocks; / total number of FS blocks,
    pub /: *mut *mut __be32 s_magic; / OMFS_MAGIC,
    pub /: *mut *mut __be32 s_blocksize; / size of a block,
    pub /: *mut *mut __be32 s_mirrors; / # of mirrors of system blocks,
    pub /: *mut *mut __be32 s_sys_blocksize; / size of non-data blocks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_header {
    pub /: *mut *mut __be64 h_self; / FS block where this is located,
    pub /: *mut *mut __be32 h_body_size; / size of useful data after header,
    pub /: *mut *mut __be16 h_crc; / crc-ccitt of body_size bytes,
    pub h_fill1: [c_char; 2],
    pub /: *mut *mut u8 h_version; / version, always 1,
    pub /: *mut *mut char h_type; / OMFS_INODE_X,
    pub /: *mut *mut u8 h_magic; / OMFS_IMAGIC,
    pub /: *mut *mut u8 h_check_xor; / XOR of header bytes before this,
    pub h_fill2: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_root_block {
    pub /: *mut *mut omfs_header r_head; / header,
    pub r_fill1: __be64,
    pub /: *mut *mut __be64 r_num_blocks; / total number of FS blocks,
    pub /: *mut *mut __be64 r_root_dir; / block # of root directory,
    pub /: *mut *mut __be64 r_bitmap; / block # of free space bitmap,
    pub /: *mut *mut __be32 r_blocksize; / size of a block,
    pub /: *mut *mut __be32 r_clustersize; / size allocated for data blocks,
    pub /: *mut *mut __be64 r_mirrors; / # of mirrors of system blocks,
    pub /: *mut *mut char r_name[OMFS_NAMELEN]; / partition label,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_inode {
    pub /: *mut *mut omfs_header i_head; / header,
    pub /: *mut *mut __be64 i_parent; / parent containing this inode,
    pub /: *mut *mut __be64 i_sibling; / next inode in hash bucket,
    pub /: *mut *mut __be64 i_ctime; / ctime, in milliseconds,
    pub i_fill1: [c_char; 35],
    pub /: *mut *mut char i_type; / OMFS_[DIR,FILE],
    pub i_fill2: __be32,
    pub i_fill3: [c_char; 64],
    pub /: *mut *mut char i_name[OMFS_NAMELEN]; / filename,
    pub /: *mut *mut __be64 i_size; / size of file, in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_extent_entry {
    pub /: *mut *mut __be64 e_cluster; / start location of a set of blocks,
    pub /: *mut *mut __be64 e_blocks; / number of blocks after e_cluster,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_extent {
    pub /: *mut *mut __be64 e_next; / next extent table location,
    pub /: *mut *mut __be32 e_extent_count; / total # extents in this table,
    pub e_fill: __be32,
    pub /: *mut *mut omfs_extent_entry e_entry[]; / start of extent entries,
}
