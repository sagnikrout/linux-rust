//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/minix_fs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// The minix filesystem constants/structures
//
// Thanks to Kees J Bot for sending me the definitions of the new
// minix filesystem (aka V2) with bigger inodes and 32-bit block
// pointers.
//
pub const MINIX_ROOT_INO: c_int = 1;
// Not the same as the bogus LINK_MAX in <linux/limits.h>. Oh well.
pub const MINIX_LINK_MAX: c_int = 250;
pub const MINIX2_LINK_MAX: c_int = 65530;
pub const MINIX_I_MAP_SLOTS: c_int = 8;
pub const MINIX_Z_MAP_SLOTS: c_int = 64;
pub const MINIX_VALID_FS: c_uint = 0x0001		/* Clean fs. */;
pub const MINIX_ERROR_FS: c_uint = 0x0002		/* fs has errors. */;

//
// This is the original minix inode layout on disk.
// Note the 8-bit gid and atime and ctime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_inode {
    pub i_mode: __u16,
    pub i_uid: __u16,
    pub i_size: __u32,
    pub i_time: __u32,
    pub i_gid: __u8,
    pub i_nlinks: __u8,
    pub i_zone: [__u16; 9],
}

//
// The new minix inode has all the time entries, as well as
// long block numbers and a third indirect block (7+1+1+1
// instead of 7+1+1). Also, some previously 8-bit values are
// now 16-bit. The inode is now 64 bytes instead of 32.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix2_inode {
    pub i_mode: __u16,
    pub i_nlinks: __u16,
    pub i_uid: __u16,
    pub i_gid: __u16,
    pub i_size: __u32,
    pub i_atime: __u32,
    pub i_mtime: __u32,
    pub i_ctime: __u32,
    pub i_zone: [__u32; 10],
}

//
// minix super-block data on disk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_super_block {
    pub s_ninodes: __u16,
    pub s_nzones: __u16,
    pub s_imap_blocks: __u16,
    pub s_zmap_blocks: __u16,
    pub s_firstdatazone: __u16,
    pub s_log_zone_size: __u16,
    pub s_max_size: __u32,
    pub s_magic: __u16,
    pub s_state: __u16,
    pub s_zones: __u32,
}

//
// V3 minix super-block data on disk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix3_super_block {
    pub s_ninodes: __u32,
    pub s_pad0: __u16,
    pub s_imap_blocks: __u16,
    pub s_zmap_blocks: __u16,
    pub s_firstdatazone: __u16,
    pub s_log_zone_size: __u16,
    pub s_pad1: __u16,
    pub s_max_size: __u32,
    pub s_zones: __u32,
    pub s_magic: __u16,
    pub s_pad2: __u16,
    pub s_blocksize: __u16,
    pub s_disk_version: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_dir_entry {
    pub inode: __u16,
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix3_dir_entry {
    pub inode: __u32,
    pub name: [c_char; ],
}
