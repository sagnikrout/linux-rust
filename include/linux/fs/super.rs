//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs/super.h
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
// These are internal functions, please use sb_start_{write,pagefault,intwrite}
// instead.
//
extern "C" {
    pub fn percpu_down_read_trylock(1: sb->s_writers.rw_sem + level -) -> return;
}

//
// __sb_write_started - check if sb freeze level is held
// @sb: the super we write to
// @level: the freeze level
//
// * > 0 - sb freeze level is held
// *   0 - sb freeze level is not held
// * < 0 - !CONFIG_LOCKDEP/LOCK_STATE_UNKNOWN
//
extern "C" {
    pub fn lockdep_is_held_type(1: sb->s_writers.rw_sem + level -, _arg: 1) -> return;
}
//
// sb_write_started - check if SB_FREEZE_WRITE is held
// @sb: the super we write to
//
// May be false positive with !CONFIG_LOCKDEP/LOCK_STATE_UNKNOWN.
//
extern "C" {
    pub fn __sb_write_started(_arg: sb, _arg: SB_FREEZE_WRITE) -> return;
}
//
// sb_write_not_started - check if SB_FREEZE_WRITE is not held
// @sb: the super we write to
//
// May be false positive with !CONFIG_LOCKDEP/LOCK_STATE_UNKNOWN.
//
// sb_end_write - drop write access to a superblock
// @sb: the super we wrote to
//
// Decrement number of writers to the filesystem. Wake up possible waiters
// wanting to freeze the filesystem.
//
// sb_end_pagefault - drop write access to a superblock from a page fault
// @sb: the super we wrote to
//
// Decrement number of processes handling write page fault to the filesystem.
// Wake up possible waiters wanting to freeze the filesystem.
//
// sb_end_intwrite - drop write access to a superblock for internal fs purposes
// @sb: the super we wrote to
//
// Decrement fs-internal number of writers to the filesystem.  Wake up possible
// waiters wanting to freeze the filesystem.
//
// sb_start_write - get write access to a superblock
// @sb: the super we write to
//
// When a process wants to write data or metadata to a file system (i.e. dirty
// a page or an inode), it should embed the operation in a sb_start_write() -
// sb_end_write() pair to get exclusion against file system freezing. This
// function increments number of writers preventing freezing. If the file
// system is already frozen, the function waits until the file system is
// thawed.
//
// Since freeze protection behaves as a lock, users have to preserve
// ordering of freeze protection and other filesystem locks. Generally,
// freeze protection should be the outermost lock. In particular, we have:
//
// sb_start_write
// -> i_rwsem			(write path, truncate, directory ops, ...)
// -> s_umount		(freeze_super, thaw_super)
//
extern "C" {
    pub fn __sb_start_write_trylock(_arg: sb, _arg: SB_FREEZE_WRITE) -> return;
}
//
// sb_start_pagefault - get write access to a superblock from a page fault
// @sb: the super we write to
//
// When a process starts handling write page fault, it should embed the
// operation into sb_start_pagefault() - sb_end_pagefault() pair to get
// exclusion against file system freezing. This is needed since the page fault
// is going to dirty a page. This function increments number of running page
// faults preventing freezing. If the file system is already frozen, the
// function waits until the file system is thawed.
//
// Since page fault freeze protection behaves as a lock, users have to preserve
// ordering of freeze protection and other filesystem locks. It is advised to
// put sb_start_pagefault() close to mmap_lock in lock ordering. Page fault
// handling code implies lock dependency:
//
// mmap_lock
// -> sb_start_pagefault
//
// sb_start_intwrite - get write access to a superblock for internal fs purposes
// @sb: the super we write to
//
// This is the third level of protection against filesystem freezing. It is
// free for use by a filesystem. The only requirement is that it must rank
// below sb_start_pagefault.
//
// For example filesystem can call sb_start_intwrite() when starting a
// transaction which somewhat eases handling of freezing for internal sources
// of filesystem changes (internal fs threads, discarding preallocation on file
// close, etc.).
//
extern "C" {
    pub fn __sb_start_write_trylock(_arg: sb, _arg: SB_FREEZE_FS) -> return;
}

// Compare if two super blocks have the same encoding and flags

extern "C" {
    pub fn sb_set_blocksize(sb: *mut super_block, size: c_int) -> c_int;
}
extern "C" {
    pub fn sb_min_blocksize(sb: *mut super_block, size: c_int) -> int __must_check;
}
extern "C" {
    pub fn sb_init_dio_done_wq(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn fs_bdev_unregister(bdev_file: *mut file, sb: *mut super_block);
}
extern "C" {
    pub fn fs_bdev_file_release(bdev_file: *mut file, sb: *mut super_block);
}
