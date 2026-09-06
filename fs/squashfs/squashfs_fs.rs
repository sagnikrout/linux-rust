//! Automatically rewritten from C Header to Rust Module
//! Source: fs/squashfs/squashfs_fs.h
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

// Macro flag: #define SQUASHFS_FS
//
// Squashfs
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
// Phillip Lougher <phillip@squashfs.org.uk>
//
// squashfs_fs.h
//

pub const SQUASHFS_MAJOR: c_int = 4;
pub const SQUASHFS_MINOR: c_int = 0;
pub const SQUASHFS_START: c_int = 0;
// size of metadata (inode and directory) blocks
pub const SQUASHFS_METADATA_SIZE: c_int = 8192;
pub const SQUASHFS_BLOCK_OFFSET: c_int = 2;
// default size of block device I/O

pub const SQUASHFS_DEVBLK_SIZE: c_int = 4096;

pub const SQUASHFS_DEVBLK_SIZE: c_int = 1024;

pub const SQUASHFS_FILE_MAX_SIZE: c_int = 1048576;
pub const SQUASHFS_FILE_MAX_LOG: c_int = 20;
// Max length of filename (not 255)
pub const SQUASHFS_NAME_LEN: c_int = 256;
// Max value for directory header count
pub const SQUASHFS_DIR_COUNT: c_int = 256;

// Filesystem flags
pub const SQUASHFS_NOI: c_int = 0;
pub const SQUASHFS_NOD: c_int = 1;
pub const SQUASHFS_NOF: c_int = 3;
pub const SQUASHFS_NO_FRAG: c_int = 4;
pub const SQUASHFS_ALWAYS_FRAG: c_int = 5;
pub const SQUASHFS_DUPLICATE: c_int = 6;
pub const SQUASHFS_EXPORT: c_int = 7;
pub const SQUASHFS_COMP_OPT: c_int = 10;

// Inode types including extended types
pub const SQUASHFS_DIR_TYPE: c_int = 1;
pub const SQUASHFS_REG_TYPE: c_int = 2;
pub const SQUASHFS_SYMLINK_TYPE: c_int = 3;
pub const SQUASHFS_BLKDEV_TYPE: c_int = 4;
pub const SQUASHFS_CHRDEV_TYPE: c_int = 5;
pub const SQUASHFS_FIFO_TYPE: c_int = 6;
pub const SQUASHFS_SOCKET_TYPE: c_int = 7;
pub const SQUASHFS_LDIR_TYPE: c_int = 8;
pub const SQUASHFS_LREG_TYPE: c_int = 9;
pub const SQUASHFS_LSYMLINK_TYPE: c_int = 10;
pub const SQUASHFS_LBLKDEV_TYPE: c_int = 11;
pub const SQUASHFS_LCHRDEV_TYPE: c_int = 12;
pub const SQUASHFS_LFIFO_TYPE: c_int = 13;
pub const SQUASHFS_LSOCKET_TYPE: c_int = 14;
// Max type value stored in directory entry
pub const SQUASHFS_MAX_DIR_TYPE: c_int = 7;
// Xattr types
pub const SQUASHFS_XATTR_USER: c_int = 0;
pub const SQUASHFS_XATTR_TRUSTED: c_int = 1;
pub const SQUASHFS_XATTR_SECURITY: c_int = 2;
pub const SQUASHFS_XATTR_VALUE_OOL: c_int = 256;
pub const SQUASHFS_XATTR_PREFIX_MASK: c_uint = 0xff;
// Flag whether block is compressed or uncompressed, bit is set if block is
// uncompressed

//
// Inode number ops.  Inodes consist of a compressed block number, and an
// uncompressed offset within that block
//

// fragment and fragment table defines

// inode lookup table defines

// uid/gid lookup table defines

// xattr id lookup table defines

// cached data constants for filesystem
pub const SQUASHFS_CACHED_BLKS: c_int = 8;
// meta index cache

pub const SQUASHFS_META_ENTRIES: c_int = 127;
pub const SQUASHFS_META_SLOTS: c_int = 8;
pub const SQUASHFS_SCAN_INDEXES: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meta_entry {
    pub data_block: u64,
    pub index_block: c_uint,
    pub offset: c_ushort,
    pub pad: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meta_index {
    pub inode_number: c_uint,
    pub offset: c_uint,
    pub entries: c_ushort,
    pub skip: c_ushort,
    pub locked: c_ushort,
    pub pad: c_ushort,
    pub meta_entry: [meta_entry; SQUASHFS_META_ENTRIES],
}

//
// definitions for structures on disk
//
pub const ZLIB_COMPRESSION: c_int = 1;
pub const LZMA_COMPRESSION: c_int = 2;
pub const LZO_COMPRESSION: c_int = 3;
pub const XZ_COMPRESSION: c_int = 4;
pub const LZ4_COMPRESSION: c_int = 5;
pub const ZSTD_COMPRESSION: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_super_block {
    pub s_magic: __le32,
    pub inodes: __le32,
    pub mkfs_time: __le32,
    pub block_size: __le32,
    pub fragments: __le32,
    pub compression: __le16,
    pub block_log: __le16,
    pub flags: __le16,
    pub no_ids: __le16,
    pub s_major: __le16,
    pub s_minor: __le16,
    pub root_inode: __le64,
    pub bytes_used: __le64,
    pub id_table_start: __le64,
    pub xattr_id_table_start: __le64,
    pub inode_table_start: __le64,
    pub directory_table_start: __le64,
    pub fragment_table_start: __le64,
    pub lookup_table_start: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_dir_index {
    pub index: __le32,
    pub start_block: __le32,
    pub size: __le32,
    pub name: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_base_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_ipc_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub nlink: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_lipc_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub nlink: __le32,
    pub xattr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_dev_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub nlink: __le32,
    pub rdev: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_ldev_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub nlink: __le32,
    pub rdev: __le32,
    pub xattr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_symlink_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub nlink: __le32,
    pub symlink_size: __le32,
    pub symlink: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_reg_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub start_block: __le32,
    pub fragment: __le32,
    pub offset: __le32,
    pub file_size: __le32,
    pub block_list: [__le16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_lreg_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub start_block: __le64,
    pub file_size: __le64,
    pub sparse: __le64,
    pub nlink: __le32,
    pub fragment: __le32,
    pub offset: __le32,
    pub xattr: __le32,
    pub block_list: [__le16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_dir_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub start_block: __le32,
    pub nlink: __le32,
    pub file_size: __le16,
    pub offset: __le16,
    pub parent_inode: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_ldir_inode {
    pub inode_type: __le16,
    pub mode: __le16,
    pub uid: __le16,
    pub guid: __le16,
    pub mtime: __le32,
    pub inode_number: __le32,
    pub nlink: __le32,
    pub file_size: __le32,
    pub start_block: __le32,
    pub parent_inode: __le32,
    pub i_count: __le16,
    pub offset: __le16,
    pub xattr: __le32,
    pub index: [squashfs_dir_index; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union squashfs_inode {
    pub base: squashfs_base_inode,
    pub dev: squashfs_dev_inode,
    pub ldev: squashfs_ldev_inode,
    pub symlink: squashfs_symlink_inode,
    pub reg: squashfs_reg_inode,
    pub lreg: squashfs_lreg_inode,
    pub dir: squashfs_dir_inode,
    pub ldir: squashfs_ldir_inode,
    pub ipc: squashfs_ipc_inode,
    pub lipc: squashfs_lipc_inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_dir_entry {
    pub offset: __le16,
    pub inode_number: __le16,
    pub type: __le16,
    pub size: __le16,
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_dir_header {
    pub count: __le32,
    pub start_block: __le32,
    pub inode_number: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_fragment_entry {
    pub start_block: __le64,
    pub size: __le32,
    pub unused: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_xattr_entry {
    pub type: __le16,
    pub size: __le16,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_xattr_val {
    pub vsize: __le32,
    pub value: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_xattr_id {
    pub xattr: __le64,
    pub count: __le32,
    pub size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_xattr_id_table {
    pub xattr_table_start: __le64,
    pub xattr_ids: __le32,
    pub unused: __le32,
}
