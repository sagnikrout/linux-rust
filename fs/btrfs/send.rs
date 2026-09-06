//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/send.h
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
// Copyright (C) 2012 Alexander Block.  All rights reserved.
// Copyright (C) 2012 STRATO.  All rights reserved.
//

// Conditional support for the upcoming protocol version.

pub const BTRFS_SEND_STREAM_VERSION: c_int = 3;

pub const BTRFS_SEND_STREAM_VERSION: c_int = 2;

//
// In send stream v1, no command is larger than 64K. In send stream v2, no
// limit should be assumed, the buffer size is set to be a header with
// compressed extent size.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_tlv_type {
    BTRFS_TLV_U8,
    BTRFS_TLV_U16,
    BTRFS_TLV_U32,
    BTRFS_TLV_U64,
    BTRFS_TLV_BINARY,
    BTRFS_TLV_STRING,
    BTRFS_TLV_UUID,
    BTRFS_TLV_TIMESPEC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_stream_header {
    pub magic: [c_char; sizeof(BTRFS_SEND_STREAM_MAGIC)],
    pub version: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_cmd_header {
// len excluding the header
    pub len: __le32,
    pub cmd: __le16,
// crc including the header with zero crc field
    pub crc: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_tlv_header {
    pub tlv_type: __le16,
// len excluding the header
    pub tlv_len: __le16,
// C attribute field omitted
// commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_send_cmd {
    BTRFS_SEND_C_UNSPEC		= 0,

// Version 1
    BTRFS_SEND_C_SUBVOL		= 1,
    BTRFS_SEND_C_SNAPSHOT		= 2,

    BTRFS_SEND_C_MKFILE		= 3,
    BTRFS_SEND_C_MKDIR		= 4,
    BTRFS_SEND_C_MKNOD		= 5,
    BTRFS_SEND_C_MKFIFO		= 6,
    BTRFS_SEND_C_MKSOCK		= 7,
    BTRFS_SEND_C_SYMLINK		= 8,

    BTRFS_SEND_C_RENAME		= 9,
    BTRFS_SEND_C_LINK		= 10,
    BTRFS_SEND_C_UNLINK		= 11,
    BTRFS_SEND_C_RMDIR		= 12,

    BTRFS_SEND_C_SET_XATTR		= 13,
    BTRFS_SEND_C_REMOVE_XATTR	= 14,

    BTRFS_SEND_C_WRITE		= 15,
    BTRFS_SEND_C_CLONE		= 16,

    BTRFS_SEND_C_TRUNCATE		= 17,
    BTRFS_SEND_C_CHMOD		= 18,
    BTRFS_SEND_C_CHOWN		= 19,
    BTRFS_SEND_C_UTIMES		= 20,

    BTRFS_SEND_C_END		= 21,
    BTRFS_SEND_C_UPDATE_EXTENT	= 22,
    BTRFS_SEND_C_MAX_V1		= 22,

// Version 2
    BTRFS_SEND_C_FALLOCATE		= 23,
    BTRFS_SEND_C_FILEATTR		= 24,
    BTRFS_SEND_C_ENCODED_WRITE	= 25,
    BTRFS_SEND_C_MAX_V2		= 25,

// Version 3
    BTRFS_SEND_C_ENABLE_VERITY	= 26,
    BTRFS_SEND_C_MAX_V3		= 26,
// End
    BTRFS_SEND_C_MAX		= 26,
}

// attributes in send stream
// Version 1
//
// As of send stream v2, this attribute is special: it must be the last
// attribute in a command, its header contains only the type, and its
// length is implicitly the remaining length of the command.
//
// Version 2
//
// File attributes from the FS_*_FL namespace (i_flags, xflags),
// translated to BTRFS_INODE_* bits (BTRFS_INODE_FLAG_MASK) and stored
// in btrfs_inode_item::flags (represented by btrfs_inode::flags and
// btrfs_inode::ro_flags).
//
// COMPRESSION and ENCRYPTION default to NONE (0) if omitted from
// BTRFS_SEND_C_ENCODED_WRITE.
//
// Version 3
}

extern "C" {
    pub fn btrfs_ioctl_send(send_root: *mut btrfs_root, arg: *const btrfs_ioctl_send_args) -> c_long;
}
