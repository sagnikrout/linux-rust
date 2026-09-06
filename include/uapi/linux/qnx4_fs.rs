//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/qnx4_fs.h
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
// Name                         : qnx4_fs.h
// Author                       : Richard Frowijn
// Function                     : qnx4 global filesystem definitions
// History                      : 23-03-1998 created
//

pub const QNX4_ROOT_INO: c_int = 1;
pub const QNX4_MAX_XTNTS_PER_XBLK: c_int = 60;
// for di_status
pub const QNX4_FILE_USED: c_uint = 0x01;
pub const QNX4_FILE_MODIFIED: c_uint = 0x02;
pub const QNX4_FILE_BUSY: c_uint = 0x04;
pub const QNX4_FILE_LINK: c_uint = 0x08;
pub const QNX4_FILE_INODE: c_uint = 0x10;
pub const QNX4_FILE_FSYSCLEAN: c_uint = 0x20;
pub const QNX4_I_MAP_SLOTS: c_int = 8;
pub const QNX4_Z_MAP_SLOTS: c_int = 64;
pub const QNX4_VALID_FS: c_uint = 0x0001	/* Clean fs. */;
pub const QNX4_ERROR_FS: c_uint = 0x0002	/* fs has errors. */;
pub const QNX4_BLOCK_SIZE: c_uint = 0x200	/* blocksize of 512 bytes */;

pub const QNX4_DIR_ENTRY_SIZE: c_uint = 0x040	/* dir entry size of 64 bytes */;

pub const QNX4_XBLK_ENTRY_SIZE: c_uint = 0x200	/* xblk entry size */;
pub const QNX4_INODES_PER_BLOCK: c_uint = 0x08	/* 512 / 64 */;
// for filenames
pub const QNX4_SHORT_NAME_MAX: c_int = 16;
pub const QNX4_NAME_MAX: c_int = 48;
//
// This is the original qnx4 inode layout on disk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx4_inode_entry {
    pub di_fname: [c_char; QNX4_SHORT_NAME_MAX],
    pub di_size: qnx4_off_t,
    pub di_first_xtnt: qnx4_xtnt_t,
    pub di_xblk: __le32,
    pub di_ftime: __le32,
    pub di_mtime: __le32,
    pub di_atime: __le32,
    pub di_ctime: __le32,
    pub di_num_xtnts: qnx4_nxtnt_t,
    pub di_mode: qnx4_mode_t,
    pub di_uid: qnx4_muid_t,
    pub di_gid: qnx4_mgid_t,
    pub di_nlink: qnx4_nlink_t,
    pub di_zero: [__u8; 4],
    pub di_type: qnx4_ftype_t,
    pub di_status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx4_link_info {
    pub dl_fname: [c_char; QNX4_NAME_MAX],
    pub dl_inode_blk: __le32,
    pub dl_inode_ndx: __u8,
    pub dl_spare: [__u8; 10],
    pub dl_status: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx4_xblk {
    pub xblk_next_xblk: __le32,
    pub xblk_prev_xblk: __le32,
    pub xblk_num_xtnts: __u8,
    pub xblk_spare: [__u8; 3],
    pub xblk_num_blocks: __le32,
    pub xblk_xtnts: [qnx4_xtnt_t; QNX4_MAX_XTNTS_PER_XBLK],
    pub xblk_signature: [c_char; 8],
    pub xblk_first_xtnt: qnx4_xtnt_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx4_super_block {
    pub RootDir: qnx4_inode_entry,
    pub Inode: qnx4_inode_entry,
    pub Boot: qnx4_inode_entry,
    pub AltBoot: qnx4_inode_entry,
}
