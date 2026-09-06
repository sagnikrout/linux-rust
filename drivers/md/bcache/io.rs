//! Automatically rewritten from C to Rust
//! Source: drivers/md/bcache/io.c
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
// Some low level IO code, and hacks for various block layer limitations
//
// Copyright 2010, 2011 Kent Overstreet <kent.overstreet@gmail.com>
// Copyright 2012 Google, Inc.
//

// Bios with headers
#[no_mangle]
pub unsafe extern "C" fn bch_bbio_free(bio: *mut bio, c: *mut cache_set) {
    void bch_bbio_free(struct bio *bio, struct cache_set *c)
    {
    struct bbio *b = container_of(bio, struct bbio, bio);
    mempool_free(b, &c.bio_meta);
    }
    struct bio *bch_bbio_alloc(struct cache_set *c)
    {
    struct bbio *b = mempool_alloc(&c.bio_meta, GFP_NOIO);
    struct bio *bio = &b.bio;
    bio_init_inline(bio, core::ptr::null_mut(), meta_bucket_pages(&c.cache.sb), 0);
    return bio;
    }
#[no_mangle]
pub unsafe extern "C" fn __bch_submit_bbio(bio: *mut bio, c: *mut cache_set) {
    void __bch_submit_bbio(struct bio *bio, struct cache_set *c)
    {
    struct bbio *b = container_of(bio, struct bbio, bio);
    bio.bi_iter.bi_sector	= PTR_OFFSET(&b.key, 0);
    bio_set_dev(bio, c.cache.bdev);
    b.submit_time_us = local_clock_us();
    closure_bio_submit(c, bio, bio.bi_private);
    }
    void bch_submit_bbio(struct bio *bio, struct cache_set *c,
    struct bkey *k, unsigned int ptr)
    {
    struct bbio *b = container_of(bio, struct bbio, bio);
    bch_bkey_copy_single_ptr(&b.key, k, ptr);
    __bch_submit_bbio(bio, c);
    }
// IO errors
#[no_mangle]
pub unsafe extern "C" fn bch_count_backing_io_errors(dc: *mut cached_dev, bio: *mut bio) {
    void bch_count_backing_io_errors(struct cached_dev *dc, struct bio *bio)
    {
    unsigned int errors;
    WARN_ONCE(!dc, "core::ptr::null_mut() pointer of struct cached_dev");
//
// Read-ahead requests on a degrading and recovering md raid
// (e.g. raid6) device might be failured immediately by md
// raid code, which is not a real hardware media failure. So
// we shouldn't count failed REQ_RAHEAD bio to dc->io_errors.
//
    if (bio.bi_opf & REQ_RAHEAD) {
    pr_warn_ratelimited("%pg: Read-ahead I/O failed on backing device, ignore\n",
    dc.bdev);
    return;
    }
    errors = atomic_add_return(1, &dc.io_errors);
    if (errors < dc.error_limit)
    pr_err("%pg: IO error on backing device, unrecoverable\n",
    dc.bdev);
    else
    bch_cached_dev_error(dc);
    }
    void bch_count_io_errors(struct cache *ca,
    blk_status_t error,
    int is_read,
    const char *m)
    {
//
// The halflife of an error is:
// log2(1/2)/log2(127/128) * refresh ~= 88 * refresh
//
    if (ca.set.error_decay) {
    let mut count: c_uint = atomic_inc_return(&ca.io_count);
    while (count > ca.set.error_decay) {
    unsigned int errors;
    let mut old: c_uint = count;
    let mut new: c_uint = count - ca.set.error_decay;
//
// First we subtract refresh from count; each time we
// successfully do so, we rescale the errors once:
//
    count = atomic_cmpxchg(&ca.io_count, old, new);
    if (count == old) {
    count = new;
    errors = atomic_read(&ca.io_errors);
    do {
    old = errors;
    new = ((uint64_t) errors * 127) / 128;
    errors = atomic_cmpxchg(&ca.io_errors,
    old, new);
    } while (old != errors);
    }
    }
    }
    if (error) {
    unsigned int errors = atomic_add_return(1 << IO_ERROR_SHIFT,
    &ca.io_errors);
    errors >>= IO_ERROR_SHIFT;
    if (errors < ca.set.error_limit)
    pr_err("%pg: IO error on %s%s\n",
    ca.bdev, m,
    is_read ? ", recovering." : ".");
    else
    bch_cache_set_error(ca.set,
    "%pg: too many IO errors %s\n",
    ca.bdev, m);
    }
    }
    void bch_bbio_count_io_errors(struct cache_set *c, struct bio *bio,
    blk_status_t error, const char *m)
    {
    struct bbio *b = container_of(bio, struct bbio, bio);
    struct cache *ca = c.cache;
    let mut is_read: c_int = (bio_data_dir(bio) == READ ? 1 : 0);
    unsigned int threshold = op_is_write(bio_op(bio))
    ? c.congested_write_threshold_us
    : c.congested_read_threshold_us;
    if (threshold) {
    let mut t: c_uint = local_clock_us();
    let mut us: c_int = t - b.submit_time_us;
    let mut congested: c_int = atomic_read(&c.congested);
    if (us > (int) threshold) {
    let mut ms: c_int = us / 1024;
    c.congested_last_us = t;
    ms = min(ms, CONGESTED_MAX + congested);
    atomic_sub(ms, &c.congested);
    } else if (congested < 0)
    atomic_inc(&c.congested);
    }
    bch_count_io_errors(ca, error, is_read, m);
    }
    void bch_bbio_endio(struct cache_set *c, struct bio *bio,
    blk_status_t error, const char *m)
    {
    struct closure *cl = bio.bi_private;
    bch_bbio_count_io_errors(c, bio, error, m);
    bio_put(bio);
    closure_put(cl);
    }
