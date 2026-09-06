//! Automatically rewritten from C to Rust
//! Source: fs/hfsplus/bitmap.c
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
// linux/fs/hfsplus/bitmap.c
//
// Copyright (C) 2001
// Brad Boyer (flar@allandria.com)
// (C) 2003 Ardis Technologies <roman@ardistech.com>
//
// Handling of allocation file
//

    int hfsplus_block_allocate(struct super_block *sb, u32 size,
    u32 offset, u32 *max)
    {
    struct hfsplus_sb_info *sbi = HFSPLUS_SB(sb);
    struct page *page;
    struct address_space *mapping;
    __be32 *pptr, *curr, *end;
    u32 mask, start, len, n;
    __be32 val;
    int i;
    len = *max;
    if (!len)
    return size;
    hfs_dbg("size %u, offset %u, len %u\n", size, offset, len);
    mutex_lock(&sbi.alloc_mutex);
    mapping = sbi.alloc_file.i_mapping;
    page = read_mapping_page(mapping, offset / PAGE_CACHE_BITS, core::ptr::null_mut());
    if (IS_ERR(page)) {
    start = size;
    goto out;
    }
    pptr = kmap_local_page(page);
    curr = pptr + (offset & (PAGE_CACHE_BITS - 1)) / 32;
    i = offset % 32;
    offset &= ~(PAGE_CACHE_BITS - 1);
    if ((size ^ offset) / PAGE_CACHE_BITS)
    end = pptr + PAGE_CACHE_BITS / 32;
    else
    end = pptr + ((size + 31) & (PAGE_CACHE_BITS - 1)) / 32;
// scan the first partial u32 for zero bits
    val = *curr;
    if (~val) {
    n = be32_to_cpu(val);
    mask = (1U << 31) >> i;
    for (; i < 32; mask >>= 1, i++) {
    if (!(n & mask))
    goto found;
    }
    }
    curr++;
// scan complete u32s for the first zero bit
    while (1) {
    while (curr < end) {
    val = *curr;
    if (~val) {
    n = be32_to_cpu(val);
    mask = 1 << 31;
    for (i = 0; i < 32; mask >>= 1, i++) {
    if (!(n & mask))
    goto found;
    }
    }
    curr++;
    }
    kunmap_local(pptr);
    offset += PAGE_CACHE_BITS;
    if (offset >= size)
    break;
    page = read_mapping_page(mapping, offset / PAGE_CACHE_BITS,
    core::ptr::null_mut());
    if (IS_ERR(page)) {
    start = size;
    goto out;
    }
    curr = pptr = kmap_local_page(page);
    if ((size ^ offset) / PAGE_CACHE_BITS)
    end = pptr + PAGE_CACHE_BITS / 32;
    else
    end = pptr + ((size + 31) & (PAGE_CACHE_BITS - 1)) / 32;
    }
    hfs_dbg("bitmap full\n");
    start = size;
    goto out;
    found:
    start = offset + (curr - pptr) * 32 + i;
    if (start >= size) {
    hfs_dbg("bitmap full\n");
    goto out;
    }
// do any partial u32 at the start
    len = min(size - start, len);
    while (1) {
    n |= mask;
    if (++i >= 32)
    break;
    mask >>= 1;
    if (!--len || n & mask)
    goto done;
    }
    if (!--len)
    goto done;
// curr++ = cpu_to_be32(n);
// do full u32s
    while (1) {
    while (curr < end) {
    n = be32_to_cpu(*curr);
    if (len < 32)
    goto last;
    if (n) {
    len = 32;
    goto last;
    }
// curr++ = cpu_to_be32(0xffffffff);
    len -= 32;
    }
    set_page_dirty(page);
    kunmap_local(pptr);
    offset += PAGE_CACHE_BITS;
    page = read_mapping_page(mapping, offset / PAGE_CACHE_BITS,
    core::ptr::null_mut());
    if (IS_ERR(page)) {
    start = size;
    goto out;
    }
    pptr = kmap_local_page(page);
    curr = pptr;
    end = pptr + PAGE_CACHE_BITS / 32;
    }
    last:
// do any partial u32 at end
    mask = 1U << 31;
    for (i = 0; i < len; i++) {
    if (n & mask)
    break;
    n |= mask;
    mask >>= 1;
    }
    done:
// curr = cpu_to_be32(n);
    set_page_dirty(page);
    kunmap_local(pptr);
// max = offset + (curr - pptr) * 32 + i - start;
    sbi.free_blocks -= *max;
    hfsplus_mark_mdb_dirty(sb);
    hfs_dbg("start %u, max %u\n", start, *max);
    out:
    mutex_unlock(&sbi.alloc_mutex);
    return start;
    }
#[no_mangle]
pub unsafe extern "C" fn hfsplus_block_free(sb: *mut super_block, offset: u32, count: u32) -> c_int {
    int hfsplus_block_free(struct super_block *sb, u32 offset, u32 count)
    {
    struct hfsplus_sb_info *sbi = HFSPLUS_SB(sb);
    struct page *page;
    struct address_space *mapping;
    __be32 *pptr, *curr, *end;
    u32 mask, len, pnr;
    int i;
// is there any actual work to be done?
    if (!count)
    return 0;
    hfs_dbg("offset %u, count %u\n", offset, count);
// are all of the bits in range?
    if ((offset + count) > sbi.total_blocks)
    return -ENOENT;
    mutex_lock(&sbi.alloc_mutex);
    mapping = sbi.alloc_file.i_mapping;
    pnr = offset / PAGE_CACHE_BITS;
    page = read_mapping_page(mapping, pnr, core::ptr::null_mut());
    if (IS_ERR(page))
    goto kaboom;
    pptr = kmap_local_page(page);
    curr = pptr + (offset & (PAGE_CACHE_BITS - 1)) / 32;
    end = pptr + PAGE_CACHE_BITS / 32;
    len = count;
// do any partial u32 at the start
    i = offset % 32;
    if (i) {
    let mut j: c_int = 32 - i;
    mask = 0xffffffffU << j;
    if (j > count) {
    mask |= 0xffffffffU >> (i + count);
// curr++ &= cpu_to_be32(mask);
    goto out;
    }
// curr++ &= cpu_to_be32(mask);
    count -= j;
    }
// do full u32s
    while (1) {
    while (curr < end) {
    if (count < 32)
    goto done;
// curr++ = 0;
    count -= 32;
    }
    if (!count)
    break;
    set_page_dirty(page);
    kunmap_local(pptr);
    page = read_mapping_page(mapping, ++pnr, core::ptr::null_mut());
    if (IS_ERR(page))
    goto kaboom;
    pptr = kmap_local_page(page);
    curr = pptr;
    end = pptr + PAGE_CACHE_BITS / 32;
    }
    done:
// do any partial u32 at end
    if (count) {
    mask = 0xffffffffU >> count;
// curr &= cpu_to_be32(mask);
    }
    out:
    set_page_dirty(page);
    kunmap_local(pptr);
    sbi.free_blocks += len;
    hfsplus_mark_mdb_dirty(sb);
    mutex_unlock(&sbi.alloc_mutex);
    return 0;
    kaboom:
    pr_crit("unable to mark blocks free: error %ld\n", PTR_ERR(page));
    mutex_unlock(&sbi.alloc_mutex);
    return -EIO;
    }
