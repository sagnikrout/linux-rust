//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qnx6_fs.h
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
// Name                 : qnx6_fs.h
// Author               : Kai Bankett
// Function             : qnx6 global filesystem definitions
// History              : 17-01-2012 created
//

pub const QNX6_ROOT_INO: c_int = 1;
// for di_status
pub const QNX6_FILE_DIRECTORY: c_uint = 0x01;
pub const QNX6_FILE_DELETED: c_uint = 0x02;
pub const QNX6_FILE_NORMAL: c_uint = 0x03;
pub const QNX6_SUPERBLOCK_SIZE: c_uint = 0x200	/* superblock always is 512 bytes */;
pub const QNX6_SUPERBLOCK_AREA: c_uint = 0x1000	/* area reserved for superblock */;
pub const QNX6_BOOTBLOCK_SIZE: c_uint = 0x2000	/* heading bootblock area */;
pub const QNX6_DIR_ENTRY_SIZE: c_uint = 0x20	/* dir entry size of 32 bytes */;
pub const QNX6_INODE_SIZE: c_uint = 0x80	/* each inode is 128 bytes */;

// for filenames
pub const QNX6_SHORT_NAME_MAX: c_int = 27;
pub const QNX6_LONG_NAME_MAX: c_int = 510;
// list of mount options
pub const QNX6_MOUNT_MMI_FS: c_uint = 0x010000 /* mount as Audi MMI 3G fs */;
//
// This is the original qnx6 inode layout on disk.
// Each inode is 128 byte long.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_inode_entry {
    pub di_size: __fs64,
    pub di_uid: __fs32,
    pub di_gid: __fs32,
    pub di_ftime: __fs32,
    pub di_mtime: __fs32,
    pub di_atime: __fs32,
    pub di_ctime: __fs32,
    pub di_mode: __fs16,
    pub di_ext_mode: __fs16,
    pub di_block_ptr: [__fs32; QNX6_NO_DIRECT_POINTERS],
    pub di_filelevels: __u8,
    pub di_status: __u8,
    pub di_unknown2: [__u8; 2],
    pub di_zero2: [__fs32; 6],
}

//
// Each directory entry is maximum 32 bytes long.
// If more characters or special characters required it is stored
// in the longfilenames structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_dir_entry {
    pub de_inode: __fs32,
    pub de_size: __u8,
    pub de_fname: [c_char; QNX6_SHORT_NAME_MAX],
}

//
// Longfilename direntries have a different structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_long_dir_entry {
    pub de_inode: __fs32,
    pub de_size: __u8,
    pub de_unknown: [__u8; 3],
    pub de_long_inode: __fs32,
    pub de_checksum: __fs32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_long_filename {
    pub lf_size: __fs16,
    pub lf_fname: [__u8; QNX6_LONG_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_root_node {
    pub size: __fs64,
    pub ptr: [__fs32; QNX6_NO_DIRECT_POINTERS],
    pub levels: __u8,
    pub mode: __u8,
    pub spare: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_super_block {
    pub sb_magic: __fs32,
    pub sb_checksum: __fs32,
    pub sb_serial: __fs64,
    pub /: *mut *mut __fs32 sb_ctime; / time the fs was created,
    pub /: *mut *mut __fs32 sb_atime; / last access time,
    pub sb_flags: __fs32,
    pub /: *mut *mut __fs16 sb_version1; / filesystem version information,
    pub /: *mut *mut __fs16 sb_version2; / filesystem version information,
    pub sb_volumeid: [__u8; 16],
    pub sb_blocksize: __fs32,
    pub sb_num_inodes: __fs32,
    pub sb_free_inodes: __fs32,
    pub sb_num_blocks: __fs32,
    pub sb_free_blocks: __fs32,
    pub sb_allocgroup: __fs32,
    pub Inode: qnx6_root_node,
    pub Bitmap: qnx6_root_node,
    pub Longfile: qnx6_root_node,
    pub Unknown: qnx6_root_node,
}

// Audi MMI 3G superblock layout is different to plain qnx6
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_mmi_super_block {
    pub sb_magic: __fs32,
    pub sb_checksum: __fs32,
    pub sb_serial: __fs64,
    pub sb_spare0: [__u8; 12],
    pub sb_id: [__u8; 12],
    pub sb_blocksize: __fs32,
    pub sb_num_inodes: __fs32,
    pub sb_free_inodes: __fs32,
    pub sb_num_blocks: __fs32,
    pub sb_free_blocks: __fs32,
    pub sb_spare1: [__u8; 4],
    pub Inode: qnx6_root_node,
    pub Bitmap: qnx6_root_node,
    pub Longfile: qnx6_root_node,
    pub Unknown: qnx6_root_node,
}
