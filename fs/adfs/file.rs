//! Automatically rewritten from C to Rust
//! Source: fs/adfs/file.c
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
// linux/fs/adfs/file.c
//
// Copyright (C) 1997-1999 Russell King
// from:
//
// linux/fs/ext2/file.c
//
// Copyright (C) 1992, 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//
// from
//
// linux/fs/minix/file.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// adfs regular file handling primitives
//

    const struct file_operations adfs_file_operations = {
    .llseek		= generic_file_llseek,
    .read_iter	= generic_file_read_iter,
    .mmap_prepare	= generic_file_mmap_prepare,
    .fsync		= simple_fsync,
    .write_iter	= generic_file_write_iter,
    .splice_read	= filemap_splice_read,
    };
    const struct inode_operations adfs_file_inode_operations = {
    .setattr	= adfs_setattr,
    };
