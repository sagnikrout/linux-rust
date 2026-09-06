//! Automatically rewritten from C to Rust
//! Source: sound/synth/util_mem.c
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
// Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
//
// Generic memory management routines for soundcard memory allocation
//

    MODULE_AUTHOR("Takashi Iwai");
    MODULE_DESCRIPTION("Generic memory management routines for soundcard memory allocation");
    MODULE_LICENSE("GPL");

//
// create a new memory manager
//
    struct snd_util_memhdr *
    snd_util_memhdr_new(int memsize)
    {
    struct snd_util_memhdr *hdr;
    hdr = kzalloc_obj(*hdr);
    if (hdr == core::ptr::null_mut())
    return core::ptr::null_mut();
    hdr.size = memsize;
    mutex_init(&hdr.block_mutex);
    INIT_LIST_HEAD(&hdr.block);
    return hdr;
    }
//
// free a memory manager
//
#[no_mangle]
pub unsafe extern "C" fn snd_util_memhdr_free(hdr: *mut snd_util_memhdr) {
    void snd_util_memhdr_free(struct snd_util_memhdr *hdr)
    {
    struct list_head *p;
    if (!hdr)
    return;
// release all blocks
    while ((p = hdr.block.next) != &hdr.block) {
    list_del(p);
    kfree(get_memblk(p));
    }
    kfree(hdr);
    }
//
// allocate a memory block (without mutex)
//
    struct snd_util_memblk *
    __snd_util_mem_alloc(struct snd_util_memhdr *hdr, int size)
    {
    struct snd_util_memblk *blk;
    unsigned int units, prev_offset;
    struct list_head *p;
    if (snd_BUG_ON(!hdr || size <= 0))
    return core::ptr::null_mut();
// word alignment
    units = size;
    if (units & 1)
    units++;
    if (units > hdr.size)
    return core::ptr::null_mut();
// look for empty block
    prev_offset = 0;
    list_for_each(p, &hdr.block) {
    blk = get_memblk(p);
    if (blk.offset - prev_offset >= units)
    goto __found;
    prev_offset = blk.offset + blk.size;
    }
    if (hdr.size - prev_offset < units)
    return core::ptr::null_mut();
    __found:
    return __snd_util_memblk_new(hdr, units, p.prev);
    }
//
// create a new memory block with the given size
// the block is linked next to prev
//
    struct snd_util_memblk *
    __snd_util_memblk_new(struct snd_util_memhdr *hdr, unsigned int units,
    struct list_head *prev)
    {
    struct snd_util_memblk *blk;
    blk = kmalloc(sizeof(struct snd_util_memblk) + hdr.block_extra_size,
    GFP_KERNEL);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
    if (prev == &hdr.block)
    blk.offset = 0;
    else {
    struct snd_util_memblk *p = get_memblk(prev);
    blk.offset = p.offset + p.size;
    }
    blk.size = units;
    list_add(&blk.list, prev);
    hdr.nblocks++;
    hdr.used += units;
    return blk;
    }
//
// allocate a memory block (with mutex)
//
    struct snd_util_memblk *
    snd_util_mem_alloc(struct snd_util_memhdr *hdr, int size)
    {
    guard(mutex)(&hdr.block_mutex);
    return __snd_util_mem_alloc(hdr, size);
    }
//
// remove the block from linked-list and free resource
// (without mutex)
//
    void
    __snd_util_mem_free(struct snd_util_memhdr *hdr, struct snd_util_memblk *blk)
    {
    list_del(&blk.list);
    hdr.nblocks--;
    hdr.used -= blk.size;
    kfree(blk);
    }
//
// free a memory block (with mutex)
//
#[no_mangle]
pub unsafe extern "C" fn snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk) -> c_int {
    int snd_util_mem_free(struct snd_util_memhdr *hdr, struct snd_util_memblk *blk)
    {
    if (snd_BUG_ON(!hdr || !blk))
    return -EINVAL;
    guard(mutex)(&hdr.block_mutex);
    __snd_util_mem_free(hdr, blk);
    return 0;
    }
//
// return available memory size
//
#[no_mangle]
pub unsafe extern "C" fn snd_util_mem_avail(hdr: *mut snd_util_memhdr) -> c_int {
    int snd_util_mem_avail(struct snd_util_memhdr *hdr)
    {
    guard(mutex)(&hdr.block_mutex);
    return hdr.size - hdr.used;
    }
    EXPORT_SYMBOL(snd_util_memhdr_new);
    EXPORT_SYMBOL(snd_util_memhdr_free);
    EXPORT_SYMBOL(snd_util_mem_alloc);
    EXPORT_SYMBOL(snd_util_mem_free);
    EXPORT_SYMBOL(snd_util_mem_avail);
    EXPORT_SYMBOL(__snd_util_mem_alloc);
    EXPORT_SYMBOL(__snd_util_mem_free);
    EXPORT_SYMBOL(__snd_util_memblk_new);
