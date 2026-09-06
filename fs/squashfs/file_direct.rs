//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/file_direct.c
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

// Read separately compressed datablock directly into page cache
    int squashfs_readpage_block(struct folio *folio, u64 block, int bsize,
    int expected)
    {
    struct page *target_page = &folio.page;
    struct inode *inode = folio.mapping.host;
    struct squashfs_sb_info *msblk = inode.i_sb.s_fs_info;
    let mut file_end: loff_t = (i_size_read(inode) - 1) >> PAGE_SHIFT;
    let mut mask: c_int = (1 << (msblk.block_log - PAGE_SHIFT)) - 1;
    let mut start_index: loff_t = folio.index & ~mask;
    let mut end_index: loff_t = start_index | mask;
    loff_t index;
    int i, pages, bytes, res = -ENOMEM;
    struct page **page, *last_page;
    struct squashfs_page_actor *actor;
    void *pageaddr;
    if (end_index > file_end)
    end_index = file_end;
    pages = end_index - start_index + 1;
    page = kmalloc_array(pages, sizeof(void *), GFP_KERNEL);
    if (page == core::ptr::null_mut())
    return res;
// Try to grab all the pages covered by the Squashfs block
    for (i = 0, index = start_index; index <= end_index; index++) {
    page[i] = (index == folio.index) ? target_page :
    grab_cache_page_nowait(folio.mapping, index);
    if (page[i] == core::ptr::null_mut())
    continue;
    if (PageUptodate(page[i])) {
    unlock_page(page[i]);
    put_page(page[i]);
    continue;
    }
    i++;
    }
    pages = i;
//
// Create a "page actor" which will kmap and kunmap the
// page cache pages appropriately within the decompressor
//
    actor = squashfs_page_actor_init_special(msblk, page, pages, expected,
    start_index << PAGE_SHIFT);
    if (actor == core::ptr::null_mut())
    goto out;
// Decompress directly into the page cache buffers
    res = squashfs_read_data(inode.i_sb, block, bsize, core::ptr::null_mut(), actor);
    last_page = squashfs_page_actor_free(actor);
    if (res < 0)
    goto mark_errored;
    if (res != expected || IS_ERR(last_page)) {
    res = -EIO;
    goto mark_errored;
    }
// Last page (if present) may have trailing bytes not filled
    bytes = res % PAGE_SIZE;
    if (end_index == file_end && last_page && bytes) {
    pageaddr = kmap_local_page(last_page);
    memset(pageaddr + bytes, 0, PAGE_SIZE - bytes);
    kunmap_local(pageaddr);
    }
// Mark pages as uptodate, unlock and release
    for (i = 0; i < pages; i++) {
    flush_dcache_page(page[i]);
    SetPageUptodate(page[i]);
    unlock_page(page[i]);
    if (page[i] != target_page)
    put_page(page[i]);
    }
    kfree(page);
    return 0;
    mark_errored:
// Decompression failed.  Target_page is
// dealt with by the caller
//
    for (i = 0; i < pages; i++) {
    if (page[i] == core::ptr::null_mut() || page[i] == target_page)
    continue;
    flush_dcache_page(page[i]);
    unlock_page(page[i]);
    put_page(page[i]);
    }
    out:
    kfree(page);
    return res;
    }
