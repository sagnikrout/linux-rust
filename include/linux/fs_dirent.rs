//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs_dirent.h
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
// This is a header for the common implementation of dirent
// to fs on-disk file type conversion.  Although the fs on-disk
// bits are specific to every file system, in practice, many
// file systems use the exact same on-disk format to describe
// the lower 3 file type bits that represent the 7 POSIX file
// types.
//
// It is important to note that the definitions in this
// header MUST NOT change. This would break both the
// userspace ABI and the on-disk format of filesystems
// using this code.
//
// All those file systems can use this generic code for the
// conversions.
//
// struct dirent file types
// exposed to user via getdents(2), readdir(3)
//
// These match bits 12..15 of stat.st_mode
// (ie "(i_mode >> 12) & 15").
//
pub const S_DT_SHIFT: c_int = 12;

// these are defined by POSIX and also present in glibc's dirent.h
pub const DT_UNKNOWN: c_int = 0;
pub const DT_FIFO: c_int = 1;
pub const DT_CHR: c_int = 2;
pub const DT_DIR: c_int = 4;
pub const DT_BLK: c_int = 6;
pub const DT_REG: c_int = 8;
pub const DT_LNK: c_int = 10;
pub const DT_SOCK: c_int = 12;
pub const DT_WHT: c_int = 14;

//
// fs on-disk file types.
// Only the low 3 bits are used for the POSIX file types.
// Other bits are reserved for fs private use.
// These definitions are shared and used by multiple filesystems,
// and MUST NOT change under any circumstances.
//
// Note that no fs currently stores the whiteout type on-disk,
// so whiteout dirents are exposed to user as DT_CHR.
//
pub const FT_UNKNOWN: c_int = 0;
pub const FT_REG_FILE: c_int = 1;
pub const FT_DIR: c_int = 2;
pub const FT_CHRDEV: c_int = 3;
pub const FT_BLKDEV: c_int = 4;
pub const FT_FIFO: c_int = 5;
pub const FT_SOCK: c_int = 6;
pub const FT_SYMLINK: c_int = 7;
pub const FT_MAX: c_int = 8;
//
// declarations for helper functions, accompanying implementation
// is in fs/fs_dirent.c
//
extern "C" {
    pub fn fs_ftype_to_dtype(filetype: c_uint) -> c_uchar;
}
extern "C" {
    pub fn fs_umode_to_ftype(mode: umode_t) -> c_uchar;
}
extern "C" {
    pub fn fs_umode_to_dtype(mode: umode_t) -> c_uchar;
}
