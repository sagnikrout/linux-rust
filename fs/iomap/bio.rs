//! Automatically rewritten from C to Rust
//! Source: fs/iomap/bio.c
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
// Copyright (C) 2010 Red Hat, Inc.
// Copyright (C) 2016-2023 Christoph Hellwig.
//

    static DEFINE_SPINLOCK(failed_read_lock);
    let mut failed_read_list: static struct bio_list = BIO_EMPTY_LIST;
#[no_mangle]
unsafe extern "C" fn __iomap_read_end_io(bio: *mut bio, error: c_int) -> u32 {
    static u32 __iomap_read_end_io(struct bio *bio, int error)
    {
    struct folio_iter fi;
    let mut folio_count: u32 = 0;
    bio_for_each_folio_all(fi, bio) {
    iomap_finish_folio_read(fi.folio, fi.offset, fi.length, error);
    folio_count++;
    }
    if (bio_integrity(bio))
    fs_bio_integrity_free(bio);
    bio_put(bio);
    return folio_count;
    }
    static void
    iomap_fail_reads(
    struct work_struct	*work)
    {
    struct bio		*bio;
    let mut tmp: bio_list = BIO_EMPTY_LIST;
    unsigned long		flags;
    spin_lock_irqsave(&failed_read_lock, flags);
    bio_list_merge_init(&tmp, &failed_read_list);
    spin_unlock_irqrestore(&failed_read_lock, flags);
    while ((bio = bio_list_pop(&tmp)) != core::ptr::null_mut()) {
    __iomap_read_end_io(bio, blk_status_to_errno(bio.bi_status));
    cond_resched();
    }
    }
    static DECLARE_WORK(failed_read_work, iomap_fail_reads);
#[no_mangle]
unsafe extern "C" fn iomap_fail_buffered_read(bio: *mut bio) {
    static void iomap_fail_buffered_read(struct bio *bio)
    {
    unsigned long flags;
//
// Bounce I/O errors to a workqueue to avoid nested i_lock acquisitions
// in the fserror code.  The caller no longer owns the bio reference
// after the spinlock drops.
//
    spin_lock_irqsave(&failed_read_lock, flags);
    if (bio_list_empty(&failed_read_list))
    WARN_ON_ONCE(!schedule_work(&failed_read_work));
    bio_list_add(&failed_read_list, bio);
    spin_unlock_irqrestore(&failed_read_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn iomap_read_end_io(bio: *mut bio) {
    static void iomap_read_end_io(struct bio *bio)
    {
    if (bio.bi_status) {
    iomap_fail_buffered_read(bio);
    return;
    }
    __iomap_read_end_io(bio, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn iomap_finish_ioend_buffered_read(ioend: *mut iomap_ioend) -> u32 {
    u32 iomap_finish_ioend_buffered_read(struct iomap_ioend *ioend)
    {
    return __iomap_read_end_io(&ioend.io_bio, ioend.io_error);
    }
    void iomap_bio_submit_read_endio(const struct iomap_iter *iter,
    struct iomap_read_folio_ctx *ctx, bio_end_io_t end_io)
    {
    struct bio *bio = ctx.read_ctx;
    bio.bi_end_io = end_io;
    if (iter.iomap.flags & IOMAP_F_INTEGRITY)
    fs_bio_integrity_alloc(bio);
    submit_bio(bio);
    ctx.read_ctx = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(iomap_bio_submit_read_endio);
    static void iomap_bio_submit_read(const struct iomap_iter *iter,
    struct iomap_read_folio_ctx *ctx)
    {
    return iomap_bio_submit_read_endio(iter, ctx, iomap_read_end_io);
    }
    static struct bio_set *iomap_read_bio_set(struct iomap_read_folio_ctx *ctx)
    {
    if (ctx.ops && ctx.ops.bio_set)
    return ctx.ops.bio_set;
    return &fs_bio_set;
    }
    static void iomap_read_alloc_bio(const struct iomap_iter *iter,
    struct iomap_read_folio_ctx *ctx, size_t plen)
    {
    const struct iomap *iomap = &iter.iomap;
    let mut nr_vecs: c_uint = DIV_ROUND_UP(iomap_length(iter), PAGE_SIZE);
    struct bio_set *bio_set = iomap_read_bio_set(ctx);
    struct folio *folio = ctx.cur_folio;
    let mut gfp: gfp_t = mapping_gfp_constraint(folio.mapping, GFP_KERNEL);
    let mut orig_gfp: gfp_t = gfp;
    struct bio *bio;
// Submit the existing range if there was one.
    if (ctx.read_ctx)
    ctx.ops.submit_read(iter, ctx);
// Same as readahead_gfp_mask:
    if (ctx.rac)
    gfp |= __GFP_NORETRY | __GFP_NOWARN;
//
// If the bio_alloc fails, try it again for a single page to avoid
// having to deal with partial page reads.  This emulates what
// do_mpage_read_folio does.
//
    bio = bio_alloc_bioset(iomap.bdev, bio_max_segs(nr_vecs), REQ_OP_READ,
    gfp, bio_set);
    if (!bio)
    bio = bio_alloc_bioset(iomap.bdev, 1, REQ_OP_READ, orig_gfp,
    bio_set);
    if (ctx.rac)
    bio.bi_opf |= REQ_RAHEAD;
    bio.bi_iter.bi_sector = iomap_sector(iomap, iter.pos);
    bio_add_folio_nofail(bio, folio, plen,
    offset_in_folio(folio, iter.pos));
    ctx.read_ctx = bio;
    ctx.read_ctx_file_offset = iter.pos;
    }
    int iomap_bio_read_folio_range(const struct iomap_iter *iter,
    struct iomap_read_folio_ctx *ctx, size_t plen)
    {
    struct folio *folio = ctx.cur_folio;
    struct bio *bio = ctx.read_ctx;
    if (!bio ||
    bio_end_sector(bio) != iomap_sector(&iter.iomap, iter.pos) ||
    bio.bi_iter.bi_size > iomap_max_bio_size(&iter.iomap) - plen ||
    !bio_add_folio(bio, folio, plen, offset_in_folio(folio, iter.pos)))
    iomap_read_alloc_bio(iter, ctx, plen);
    return 0;
    }
    EXPORT_SYMBOL_GPL(iomap_bio_read_folio_range);
    const struct iomap_read_ops iomap_bio_read_ops = {
    .read_folio_range	= iomap_bio_read_folio_range,
    .submit_read		= iomap_bio_submit_read,
    };
    EXPORT_SYMBOL_GPL(iomap_bio_read_ops);
    int iomap_bio_read_folio_range_sync(const struct iomap_iter *iter,
    struct folio *folio, loff_t pos, size_t len)
    {
    const struct iomap *srcmap = iomap_iter_srcmap(iter);
    let mut sector: sector_t = iomap_sector(srcmap, pos);
    struct bio_vec bvec;
    struct bio bio;
    int error;
    bio_init(&bio, srcmap.bdev, &bvec, 1, REQ_OP_READ);
    bio.bi_iter.bi_sector = sector;
    bio_add_folio_nofail(&bio, folio, len, offset_in_folio(folio, pos));
    if (srcmap.flags & IOMAP_F_INTEGRITY)
    fs_bio_integrity_alloc(&bio);
    error = submit_bio_wait(&bio);
    if (bio_integrity(&bio)) {
    if (!error)
    error = fs_bio_integrity_verify(&bio, sector, len);
    fs_bio_integrity_free(&bio);
    }
    bio_uninit(&bio);
    return error;
    }
