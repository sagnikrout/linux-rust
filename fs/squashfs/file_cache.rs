//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/file_cache.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2013
// Phillip Lougher <phillip@squashfs.org.uk>
//

// Read separately compressed datablock and memcopy into page cache
#[no_mangle]
pub unsafe extern "C" fn squashfs_readpage_block(folio: *mut folio, block: u64, bsize: c_int, expected: c_int) -> c_int {
    int squashfs_readpage_block(struct folio *folio, u64 block, int bsize, int expected)
    {
    struct inode *i = folio.mapping.host;
    struct squashfs_cache_entry *buffer = squashfs_get_datablock(i.i_sb,
    block, bsize);
    let mut res: c_int = buffer.error;
    if (res)
    ERROR("Unable to read page, block %llx, size %x\n", block,
    bsize);
    else
    squashfs_copy_cache(folio, buffer, expected, 0);
    squashfs_cache_put(buffer);
    return res;
    }
