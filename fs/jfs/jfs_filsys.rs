//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_filsys.h
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
//
// Copyright (C) International Business Machines Corp., 2000-2003
//
// jfs_filsys.h
//
// file system (implementation-dependent) constants
//
// refer to <limits.h> for system wide implementation-dependent constants
//
// file system option (superblock flag)
//
// directory option
pub const JFS_UNICODE: c_uint = 0x00000001	/* unicode name */;
// mount time flags for error handling
pub const JFS_ERR_REMOUNT_RO: c_uint = 0x00000002	/* remount read-only */;
pub const JFS_ERR_CONTINUE: c_uint = 0x00000004	/* continue */;
pub const JFS_ERR_PANIC: c_uint = 0x00000008	/* panic */;

// Quota support
pub const JFS_USRQUOTA: c_uint = 0x00000010;
pub const JFS_GRPQUOTA: c_uint = 0x00000020;
// mount time flag to disable journaling to disk
pub const JFS_NOINTEGRITY: c_uint = 0x00000040;
// mount time flag to enable TRIM to ssd disks
pub const JFS_DISCARD: c_uint = 0x00000080;
// commit option
pub const JFS_COMMIT: c_uint = 0x00000f00	/* commit option mask */;
pub const JFS_GROUPCOMMIT: c_uint = 0x00000100	/* group (of 1) commit */;
pub const JFS_LAZYCOMMIT: c_uint = 0x00000200	/* lazy commit */;
pub const JFS_TMPFS: c_uint = 0x00000400	/* temporary file system -;
// do not log/commit:
// Never implemented
//
// log logical volume option
pub const JFS_INLINELOG: c_uint = 0x00000800	/* inline log within file system */;
pub const JFS_INLINEMOVE: c_uint = 0x00001000	/* inline log being moved */;
// Secondary aggregate inode table
pub const JFS_BAD_SAIT: c_uint = 0x00010000	/* current secondary ait is bad */;
// sparse regular file support
pub const JFS_SPARSE: c_uint = 0x00020000	/* sparse regular file */;
// DASD Limits		F226941
pub const JFS_DASD_ENABLED: c_uint = 0x00040000	/* DASD limits enabled */;
pub const JFS_DASD_PRIME: c_uint = 0x00080000	/* Prime DASD usage on boot */;
// big endian flag
pub const JFS_SWAP_BYTES: c_uint = 0x00100000	/* running on big endian computer */;
// Directory index
pub const JFS_DIR_INDEX: c_uint = 0x00200000	/* Persistent index for */;
// platform options
pub const JFS_LINUX: c_uint = 0x10000000	/* Linux support */;
pub const JFS_DFS: c_uint = 0x20000000	/* DCE DFS LFS support */;
// Never implemented
pub const JFS_OS2: c_uint = 0x40000000	/* OS/2 support */;
// case-insensitive name/directory support
pub const JFS_AIX: c_uint = 0x80000000	/* AIX support */;
//
// buffer cache configuration
//
// page size

// buffer page size

//
// fs fundamental size
//
// PSIZE >= file system block size >= PBSIZE >= DISIZE
//

pub const XTPAGE_SIZE: c_int = 4096;
pub const log2_PAGESIZE: c_int = 12;
pub const IAG_SIZE: c_int = 4096;
pub const IAG_EXTENT_SIZE: c_int = 4096;

pub const IAGFREELIST_LWM: c_int = 64;

pub const MINBLOCKSIZE: c_int = 512;
pub const L2MINBLOCKSIZE: c_int = 9;
pub const MAXBLOCKSIZE: c_int = 4096;
pub const L2MAXBLOCKSIZE: c_int = 12;

pub const JFS_LINK_MAX: c_uint = 0xffffffff;
// Minimum number of bytes supported for a JFS partition

//
// file system block size -> physical block size
//

// size in byte -> last page number

// size in byte -> last file system block number

//
// fixed physical block address (physical block size = 512 byte)
//
// NOTE: since we can't guarantee a physical block size of 512 bytes the use of
// these macros should be removed and the byte offset macros used instead.
//

// 1st extent of aggregate inode table
//

//
// SIZE_OF_SUPER defines the total amount of space reserved on disk for the
// superblock.  This is not the same as the superblock structure, since all of
// this space is not currently being used.
//

//
// SIZE_OF_AG_TABLE defines the amount of space reserved to hold the AG table
//

//
// SIZE_OF_MAP_PAGE defines the amount of disk space reserved for each page of
// the inode allocation map (to hold iag)
//

//
// fixed byte offset address
//
pub const SUPER1_OFF: c_uint = 0x8000	/* primary superblock */;

//
// Control page of aggregate inode map
// followed by 1st extent of map
//

//
// 1st extent of aggregate inode table
//

//
// secondary superblock
//

//
// block allocation map
//
// The following macro is used to indicate the number of reserved disk blocks at
// the front of an aggregate, in terms of physical blocks.  This value is
// currently defined to be 32K.  This turns out to be the same as the primary
// superblock's address, since it directly follows the reserved blocks.
//

//
// The following macro is used to indicate the number of reserved bytes at the
// front of an aggregate.  This value is currently defined to be 32K.  This
// turns out to be the same as the primary superblock's byte offset, since it
// directly follows the reserved blocks.
//

//
// The following macro defines the byte offset for the first inode extent in
// the aggregate inode table.  This allows us to find the self inode to find the
// rest of the table.  Currently this value is 44K.
//

//
// fixed reserved inode number
//
// aggregate inode

// fileset inode map inode
//
// per fileset inode

// or directory or link...
//

// an inode.  (To fsck this is also the first
// inode in part 2 of the agg inode table.)
//
// directory configuration
//
pub const JFS_NAME_MAX: c_int = 255;

//
// file system state (superblock state)
//
pub const FM_CLEAN: c_uint = 0x00000000	/* file system is unmounted and clean */;
pub const FM_MOUNT: c_uint = 0x00000001	/* file system is mounted cleanly */;
pub const FM_DIRTY: c_uint = 0x00000002	/* file system was not unmounted and clean;
// when mounted or
// commit failure occurred while being mounted:
// fsck() must be run to repair
//
pub const FM_LOGREDO: c_uint = 0x00000004	/* log based recovery (logredo()) failed:;
// fsck() must be run to repair
//
pub const FM_EXTENDFS: c_uint = 0x00000008	/* file system extendfs() in progress */;
pub const FM_STATE_MAX: c_uint = 0x0000000f	/* max value of s_state */;
