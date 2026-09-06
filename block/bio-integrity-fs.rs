//! Automatically rewritten from C to Rust
//! Source: block/bio-integrity-fs.c
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
// Copyright (c) 2025 Christoph Hellwig.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_bio_integrity_buf {
    pub bip: bio_integrity_payload,
    pub bvec: bio_vec,
}

    static struct kmem_cache *fs_bio_integrity_cache;
    static mempool_t fs_bio_integrity_pool;
#[no_mangle]
pub unsafe extern "C" fn fs_bio_integrity_alloc(bio: *mut bio) -> c_uint {
    unsigned int fs_bio_integrity_alloc(struct bio *bio)
    {
    struct fs_bio_integrity_buf *iib;
    unsigned int action;
    action = bio_integrity_action(bio);
    if (!action)
    return 0;
    iib = mempool_alloc(&fs_bio_integrity_pool, GFP_NOFS);
    bio_integrity_init(bio, &iib.bip, &iib.bvec, 1);
    bio_integrity_alloc_buf(bio, GFP_NOFS, action & BI_ACT_ZERO);
    if (action & BI_ACT_CHECK)
    bio_integrity_setup_default(bio);
    return action;
    }
#[no_mangle]
pub unsafe extern "C" fn fs_bio_integrity_free(bio: *mut bio) {
    void fs_bio_integrity_free(struct bio *bio)
    {
    struct bio_integrity_payload *bip = bio_integrity(bio);
    bio_integrity_free_buf(bip);
    mempool_free(container_of(bip, struct fs_bio_integrity_buf, bip),
    &fs_bio_integrity_pool);
    bio.bi_integrity = core::ptr::null_mut();
    bio.bi_opf &= ~REQ_INTEGRITY;
    }
#[no_mangle]
pub unsafe extern "C" fn fs_bio_integrity_generate(bio: *mut bio) {
    void fs_bio_integrity_generate(struct bio *bio)
    {
    if (fs_bio_integrity_alloc(bio) &&
    (bio_integrity(bio).bip_flags & BIP_CHECK_FLAGS))
    bio_integrity_generate(bio);
    }
    EXPORT_SYMBOL_GPL(fs_bio_integrity_generate);
#[no_mangle]
pub unsafe extern "C" fn fs_bio_integrity_verify(bio: *mut bio, sector: sector_t, size: c_uint) -> c_int {
    int fs_bio_integrity_verify(struct bio *bio, sector_t sector, unsigned int size)
    {
    struct blk_integrity *bi = blk_get_integrity(bio.bi_bdev.bd_disk);
    struct bio_integrity_payload *bip = bio_integrity(bio);
    struct bvec_iter data_iter = {
    .bi_sector	= sector,
    .bi_size	= size,
    };
    if (!bip || !(bip.bip_flags & BIP_CHECK_FLAGS))
    return 0;
//
// Reinitialize bip->bip_iter.
//
// This is for use in the submitter after the driver is done with the
// bio.  Requires the submitter to remember the sector and the size.
//
    memset(&bip.bip_iter, 0, sizeof(bip.bip_iter));
    bip.bip_iter.bi_sector = sector;
    bip.bip_iter.bi_size = bio_integrity_bytes(bi, size >> SECTOR_SHIFT);
    return blk_status_to_errno(bio_integrity_verify(bio, &data_iter));
    }
#[no_mangle]
unsafe extern "C" fn fs_bio_integrity_init() -> int __init {
    static int __init fs_bio_integrity_init(void)
    {
    fs_bio_integrity_cache = kmem_cache_create("fs_bio_integrity",
    sizeof(struct fs_bio_integrity_buf), 0,
    SLAB_HWCACHE_ALIGN | SLAB_PANIC, core::ptr::null_mut());
    if (mempool_init_slab_pool(&fs_bio_integrity_pool, BIO_POOL_SIZE,
    fs_bio_integrity_cache))
    panic("fs_bio_integrity: can't create pool\n");
    return 0;
    }
    fs_initcall(fs_bio_integrity_init);
