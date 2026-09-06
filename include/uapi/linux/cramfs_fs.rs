//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cramfs_fs.h
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
// Width of various bitfields in struct cramfs_inode.
// Primarily used to generate warnings in mkcramfs.
//
pub const CRAMFS_MODE_WIDTH: c_int = 16;
pub const CRAMFS_UID_WIDTH: c_int = 16;
pub const CRAMFS_SIZE_WIDTH: c_int = 24;
pub const CRAMFS_GID_WIDTH: c_int = 8;
pub const CRAMFS_NAMELEN_WIDTH: c_int = 6;
pub const CRAMFS_OFFSET_WIDTH: c_int = 26;
//
// Since inode.namelen is a unsigned 6-bit number, the maximum cramfs
// path length is 63 << 2 = 252.
//

//
// Reasonably terse representation of the inode data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cramfs_inode {
    pub uid:CRAMFS_UID_WIDTH: __u32 mode:CRAMFS_MODE_WIDTH,,
// SIZE for device files is i_rdev
    pub gid:CRAMFS_GID_WIDTH: __u32 size:CRAMFS_SIZE_WIDTH,,
// NAMELEN is the length of the file name, divided by 4 and
// OFFSET: For symlinks and non-empty regular files, this
    pub pointers: compressed form (starting with an array of block,
    pub offset:CRAMFS_OFFSET_WIDTH: __u32 namelen:CRAMFS_NAMELEN_WIDTH,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cramfs_info {
    pub crc: __u32,
    pub edition: __u32,
    pub blocks: __u32,
    pub files: __u32,
}

//
// Superblock information at the beginning of the FS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cramfs_super {
    pub /: *mut *mut __u32 magic; / 0x28cd3d45 - random number,
    pub /: *mut *mut __u32 size; / length in bytes,
    pub /: *mut *mut __u32 flags; / feature flags,
    pub /: *mut *mut __u32 future; / reserved for future use,
    pub /: *mut *mut __u8 signature[16]; / "Compressed ROMFS",
    pub /: *mut *mut cramfs_info fsid; / unique filesystem info,
    pub /: *mut *mut __u8 name[16]; / user-defined name,
    pub /: *mut *mut cramfs_inode root; / root inode data,
}

//
// Feature flags
//
// 0x00000000 - 0x000000ff: features that work for all past kernels
// 0x00000100 - 0xffffffff: features that don't work for past kernels
//
pub const CRAMFS_FLAG_FSID_VERSION_2: c_uint = 0x00000001	/* fsid version #2 */;
pub const CRAMFS_FLAG_SORTED_DIRS: c_uint = 0x00000002	/* sorted dirs */;
pub const CRAMFS_FLAG_HOLES: c_uint = 0x00000100	/* support for holes */;
pub const CRAMFS_FLAG_WRONG_SIGNATURE: c_uint = 0x00000200	/* reserved */;
pub const CRAMFS_FLAG_SHIFTED_ROOT_OFFSET: c_uint = 0x00000400	/* shifted root fs */;
pub const CRAMFS_FLAG_EXT_BLOCK_POINTERS: c_uint = 0x00000800	/* block pointer extensions */;
//
// Valid values in super.flags.  Currently we refuse to mount
// if (flags & ~CRAMFS_SUPPORTED_FLAGS).  Maybe that should be
// changed to test super.future instead.
//

//
// Block pointer flags
//
// The maximum block offset that needs to be represented is roughly:
//
// (1 << CRAMFS_OFFSET_WIDTH) * 4 +
// (1 << CRAMFS_SIZE_WIDTH) / PAGE_SIZE * (4 + PAGE_SIZE)
// = 0x11004000
//
// That leaves room for 3 flag bits in the block pointer table.
//

//
// Direct blocks are at least 4-byte aligned.
// Pointers to direct blocks are shifted down by 2 bits.
//
pub const CRAMFS_BLK_DIRECT_PTR_SHIFT: c_int = 2;
