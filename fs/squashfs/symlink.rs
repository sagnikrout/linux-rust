//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/symlink.c
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
// Squashfs - a compressed read only filesystem for Linux
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
// Phillip Lougher <phillip@squashfs.org.uk>
//
// symlink.c
//
// This file implements code to handle symbolic links.
//
// The data contents of symbolic links are stored inside the symbolic
// link inode within the inode table.  This allows the normally small symbolic
// link to be compressed as part of the inode table, achieving much greater
// compression than if the symbolic link was compressed individually.
//

#[no_mangle]
unsafe extern "C" fn squashfs_symlink_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    static int squashfs_symlink_read_folio(struct file *file, struct folio *folio)
    {
    struct inode *inode = folio.mapping.host;
    struct super_block *sb = inode.i_sb;
    struct squashfs_sb_info *msblk = sb.s_fs_info;
    let mut index: c_int = folio_pos(folio);
    let mut block: u64 = squashfs_i(inode).start;
    let mut offset: c_int = squashfs_i(inode).offset;
    let mut length: c_int = min_t(int, i_size_read(inode) - index, PAGE_SIZE);
    int bytes, copied, error;
    void *pageaddr;
    struct squashfs_cache_entry *entry;
    TRACE("Entered squashfs_symlink_readpage, page index %ld, start block "
    "%llx, offset %x\n", folio.index, block, offset);
//
// Skip index bytes into symlink metadata.
//
    if (index) {
    bytes = squashfs_read_metadata(sb, core::ptr::null_mut(), &block, &offset,
    index);
    if (bytes < 0) {
    ERROR("Unable to read symlink [%llx:%x]\n",
    squashfs_i(inode).start,
    squashfs_i(inode).offset);
    error = bytes;
    goto out;
    }
    }
//
// Read length bytes from symlink metadata.  Squashfs_read_metadata
// is not used here because it can sleep and we want to use
// kmap_local to map the folio.  Instead call the underlying
// squashfs_cache_get routine.  As length bytes may overlap metadata
// blocks, we may need to call squashfs_cache_get multiple times.
//
    for (bytes = 0; bytes < length; offset = 0, bytes += copied) {
    entry = squashfs_cache_get(sb, msblk.block_cache, block, 0);
    if (entry.error) {
    ERROR("Unable to read symlink [%llx:%x]\n",
    squashfs_i(inode).start,
    squashfs_i(inode).offset);
    squashfs_cache_put(entry);
    error = entry.error;
    goto out;
    }
    pageaddr = kmap_local_folio(folio, 0);
    copied = squashfs_copy_data(pageaddr + bytes, entry, offset,
    length - bytes);
    if (copied == length - bytes)
    memset(pageaddr + length, 0, PAGE_SIZE - length);
    else
    block = entry.next_index;
    kunmap_local(pageaddr);
    squashfs_cache_put(entry);
    }
    flush_dcache_folio(folio);
    error = 0;
    out:
    folio_end_read(folio, error == 0);
    return error;
    }
    const struct address_space_operations squashfs_symlink_aops = {
    .read_folio = squashfs_symlink_read_folio
    };
    const struct inode_operations squashfs_symlink_inode_ops = {
    .get_link = page_get_link,
    .listxattr = squashfs_listxattr
    };
