//! Automatically rewritten from C Header to Rust Module
//! Source: fs/squashfs/squashfs_fs_sb.h
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

// Macro flag: #define SQUASHFS_FS_SB
//
// Squashfs
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
// Phillip Lougher <phillip@squashfs.org.uk>
//
// squashfs_fs_sb.h
//

//
// Waiters for a cache entry sleep on wait_queue as exclusive waiters, so
// freeing one entry wakes one task.  See squashfs_cache_get().
//
// num_waiters is only a hint used to skip pointless wakeups: it is
// incremented before a task queues itself and decremented after it is woken,
// so it can transiently exceed the number of queued tasks.  It never
// undercounts them, which is what the wakeup paths rely on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_cache {
    pub name: *mut c_char,
    pub entries: c_int,
    pub curr_blk: c_int,
    pub next_blk: c_int,
    pub num_waiters: c_int,
    pub unused: c_int,
    pub block_size: c_int,
    pub pages: c_int,
    pub lock: spinlock_t,
    pub wait_queue: wait_queue_head_t,
    pub entry: *mut squashfs_cache_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_cache_entry {
    pub block: u64,
    pub length: c_int,
    pub refcount: c_int,
    pub next_index: u64,
    pub pending: c_int,
    pub error: c_int,
    pub num_waiters: c_int,
    pub wait_queue: wait_queue_head_t,
    pub cache: *mut squashfs_cache,
    pub data: *mut c_void,
    pub actor: *mut squashfs_page_actor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_sb_info {
    pub decompressor: *const squashfs_decompressor,
    pub devblksize: c_int,
    pub devblksize_log2: c_int,
    pub block_cache: *mut squashfs_cache,
    pub fragment_cache: *mut squashfs_cache,
    pub read_page: *mut squashfs_cache,
    pub cache_mapping: *mut address_space,
    pub next_meta_index: c_int,
    pub id_table: *mut __le64,
    pub fragment_index: *mut __le64,
    pub xattr_id_table: *mut __le64,
    pub meta_index_mutex: mutex,
    pub meta_index: *mut meta_index,
    pub stream: *mut c_void,
    pub inode_lookup_table: *mut __le64,
    pub inode_table: u64,
    pub directory_table: u64,
    pub xattr_table: u64,
    pub block_size: c_uint,
    pub block_log: c_ushort,
    pub bytes_used: c_longlong,
    pub inodes: c_uint,
    pub fragments: c_uint,
    pub xattr_ids: c_uint,
    pub ids: c_uint,
    pub panic_on_errors: bool,
    pub thread_ops: *const squashfs_decompressor_thread_ops,
    pub max_thread_num: c_int,
}
