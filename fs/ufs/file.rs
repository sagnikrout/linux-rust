//! Automatically rewritten from C to Rust
//! Source: fs/ufs/file.c
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
// linux/fs/ufs/file.c
//
// Copyright (C) 1998
// Daniel Pirkl <daniel.pirkl@email.cz>
// Charles University, Faculty of Mathematics and Physics
//
// from
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
// ext2 fs regular file handling primitives
//

//
// We have mostly NULL's here: the current defaults are ok for
// the ufs filesystem.
//
    const struct file_operations ufs_file_operations = {
    .llseek		= generic_file_llseek,
    .read_iter	= generic_file_read_iter,
    .write_iter	= generic_file_write_iter,
    .mmap_prepare	= generic_file_mmap_prepare,
    .open           = generic_file_open,
    .fsync		= simple_fsync,
    .splice_read	= filemap_splice_read,
    .splice_write	= iter_file_splice_write,
    .setlease	= generic_setlease,
    };
