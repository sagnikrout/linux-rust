//! Automatically rewritten from C to Rust
//! Source: fs/ramfs/file-mmu.c
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


// file-mmu.c: ramfs MMU-based file operations
//
// Resizable simple ram filesystem for Linux.
//
// Copyright (C) 2000 Linus Torvalds.
// 2000 Transmeta Corp.
//
// Usage limits added by David Gibson, Linuxcare Australia.
// This file is released under the GPL.
//
// NOTE! This filesystem is probably most useful
// not as a real filesystem, but as an example of
// how virtual filesystems can be written.
//
// It doesn't get much simpler than this. Consider
// that this file implements the full semantics of
// a POSIX-compliant read-write filesystem.
//
// Note in particular how the filesystem does not
// need to implement any data structures of its own
// to keep track of the virtual data: using the VFS
// caches is sufficient.
//

    static unsigned long ramfs_mmu_get_unmapped_area(struct file *file,
    unsigned long addr, unsigned long len, unsigned long pgoff,
    unsigned long flags)
    {
    return mm_get_unmapped_area(file, addr, len, pgoff, flags);
    }
    const struct file_operations ramfs_file_operations = {
    .read_iter	= generic_file_read_iter,
    .write_iter	= generic_file_write_iter,
    .mmap_prepare	= generic_file_mmap_prepare,
    .fsync		= noop_fsync,
    .splice_read	= filemap_splice_read,
    .splice_write	= iter_file_splice_write,
    .llseek		= generic_file_llseek,
    .get_unmapped_area	= ramfs_mmu_get_unmapped_area,
    };
    const struct inode_operations ramfs_file_inode_operations = {
    .setattr	= simple_setattr,
    .getattr	= simple_getattr,
    };
