//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/romfs_fs.h
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

// The basic structures of the romfs filesystem

pub const ROMFS_MAGIC: c_uint = 0x7275;
pub const ROMFS_MAXFN: c_int = 128;

// On-disk "super block"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct romfs_super_block {
    pub word0: __be32,
    pub word1: __be32,
    pub size: __be32,
    pub checksum: __be32,
    pub /: *mut *mut char name[]; / volume name,
}

// On disk inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct romfs_inode {
    pub /: *mut *mut __be32 next; / low 4 bits see ROMFH_,
    pub spec: __be32,
    pub size: __be32,
    pub checksum: __be32,
    pub name: [c_char; ],
}

pub const ROMFH_TYPE: c_int = 7;
pub const ROMFH_HRD: c_int = 0;
pub const ROMFH_DIR: c_int = 1;
pub const ROMFH_REG: c_int = 2;
pub const ROMFH_SYM: c_int = 3;
pub const ROMFH_BLK: c_int = 4;
pub const ROMFH_CHR: c_int = 5;
pub const ROMFH_SCK: c_int = 6;
pub const ROMFH_FIF: c_int = 7;
pub const ROMFH_EXEC: c_int = 8;
// Alignment
pub const ROMFH_SIZE: c_int = 16;

