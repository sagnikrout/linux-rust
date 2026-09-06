//! Automatically rewritten from C to Rust
//! Source: drivers/md/bcache/movinggc.c
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
// Moving/copying garbage collector
//
// Copyright 2012 Google, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct moving_io {
    pub cl: closure,
    pub w: *mut keybuf_key,
    pub op: data_insert_op,
    pub bio: bbio,
}

#[no_mangle]
unsafe extern "C" fn moving_pred(buf: *mut keybuf, k: *mut bkey) -> bool {
    static bool moving_pred(struct keybuf *buf, struct bkey *k)
    {
    struct cache_set *c = container_of(buf, struct cache_set,
    moving_gc_keys);
    unsigned int i;
    for (i = 0; i < KEY_PTRS(k); i++)
    if (ptr_available(c, k, i) &&
    GC_MOVE(PTR_BUCKET(c, k, i)))
    return true;
    return false;
    }
// Moving GC - IO loop
#[no_mangle]
pub unsafe extern "C" fn CLOSURE_CALLBACK(_arg: moving_io_destructor) -> static {
    static CLOSURE_CALLBACK(moving_io_destructor)
    {
    closure_type(io, struct moving_io, cl);
    kfree(io);
    }
#[no_mangle]
pub unsafe extern "C" fn CLOSURE_CALLBACK(_arg: write_moving_finish) -> static {
    static CLOSURE_CALLBACK(write_moving_finish)
    {
    closure_type(io, struct moving_io, cl);
    struct bio *bio = &io.bio.bio;
    bio_free_pages(bio);
    if (io.op.replace_collision)
    trace_bcache_gc_copy_collision(&io.w.key);
    bch_keybuf_del(&io.op.c.moving_gc_keys, io.w);
    up(&io.op.c.moving_in_flight);
    closure_return_with_destructor(cl, moving_io_destructor);
    }
#[no_mangle]
unsafe extern "C" fn read_moving_endio(bio: *mut bio) {
    static void read_moving_endio(struct bio *bio)
    {
    struct bbio *b = container_of(bio, struct bbio, bio);
    struct moving_io *io = container_of(bio.bi_private,
    struct moving_io, cl);
    if (bio.bi_status)
    io.op.status = bio.bi_status;
    else if (!KEY_DIRTY(&b.key) &&
    ptr_stale(io.op.c, &b.key, 0)) {
    io.op.status = BLK_STS_IOERR;
    }
    bch_bbio_endio(io.op.c, bio, bio.bi_status, "reading data to move");
    }
#[no_mangle]
unsafe extern "C" fn moving_init(io: *mut moving_io) {
    static void moving_init(struct moving_io *io)
    {
    struct bio *bio = &io.bio.bio;
    bio_init_inline(bio, core::ptr::null_mut(),
    DIV_ROUND_UP(KEY_SIZE(&io.w.key), PAGE_SECTORS), 0);
    bio_get(bio);
    bio.bi_ioprio = IOPRIO_PRIO_VALUE(IOPRIO_CLASS_IDLE, 0);
    bio.bi_iter.bi_size	= KEY_SIZE(&io.w.key) << 9;
    bio.bi_private		= &io.cl;
    bch_bio_map(bio, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn CLOSURE_CALLBACK(_arg: write_moving) -> static {
    static CLOSURE_CALLBACK(write_moving)
    {
    closure_type(io, struct moving_io, cl);
    struct data_insert_op *op = &io.op;
    if (!op.status) {
    moving_init(io);
    io.bio.bio.bi_iter.bi_sector = KEY_START(&io.w.key);
    op.write_prio		= 1;
    op.bio			= &io.bio.bio;
    op.writeback		= KEY_DIRTY(&io.w.key);
    op.csum		= KEY_CSUM(&io.w.key);
    bkey_copy(&op.replace_key, &io.w.key);
    op.replace		= true;
    closure_call(&op.cl, bch_data_insert, core::ptr::null_mut(), cl);
    }
    continue_at(cl, write_moving_finish, op.wq);
    }
#[no_mangle]
pub unsafe extern "C" fn CLOSURE_CALLBACK(_arg: read_moving_submit) -> static {
    static CLOSURE_CALLBACK(read_moving_submit)
    {
    closure_type(io, struct moving_io, cl);
    struct bio *bio = &io.bio.bio;
    bch_submit_bbio(bio, io.op.c, &io.w.key, 0);
    continue_at(cl, write_moving, io.op.wq);
    }
#[no_mangle]
unsafe extern "C" fn read_moving(c: *mut cache_set) {
    static void read_moving(struct cache_set *c)
    {
    struct keybuf_key *w;
    struct moving_io *io;
    struct bio *bio;
    struct closure cl;
    closure_init_stack(&cl);
// XXX: if we error, background writeback could stall indefinitely
    while (!test_bit(CACHE_SET_STOPPING, &c.flags)) {
    w = bch_keybuf_next_rescan(c, &c.moving_gc_keys,
    &MAX_KEY, moving_pred);
    if (!w)
    break;
    if (ptr_stale(c, &w.key, 0)) {
    bch_keybuf_del(&c.moving_gc_keys, w);
    continue;
    }
    io = kzalloc(sizeof(*io) + sizeof(struct bio_vec) *
    DIV_ROUND_UP(KEY_SIZE(&w.key), PAGE_SECTORS),
    GFP_KERNEL);
    if (!io)
    goto err;
    w.private	= io;
    io.w		= w;
    io.op.inode	= KEY_INODE(&w.key);
    io.op.c	= c;
    io.op.wq	= c.moving_gc_wq;
    moving_init(io);
    bio = &io.bio.bio;
    bio.bi_opf = REQ_OP_READ;
    bio.bi_end_io	= read_moving_endio;
    if (bch_bio_alloc_pages(bio, GFP_KERNEL))
    goto err;
    trace_bcache_gc_copy(&w.key);
    down(&c.moving_in_flight);
    closure_call(&io.cl, read_moving_submit, core::ptr::null_mut(), &cl);
    }
    if (0) {
    err:		if (!IS_ERR_OR_NULL(w.private))
    kfree(w.private);
    bch_keybuf_del(&c.moving_gc_keys, w);
    }
    closure_sync(&cl);
    }
#[no_mangle]
unsafe extern "C" fn bucket_cmp(l: *mut bucket, r: *mut bucket) -> bool {
    static bool bucket_cmp(struct bucket *l, struct bucket *r)
    {
    return GC_SECTORS_USED(l) < GC_SECTORS_USED(r);
    }
#[no_mangle]
unsafe extern "C" fn bucket_heap_top(ca: *mut cache) -> c_uint {
    static unsigned int bucket_heap_top(struct cache *ca)
    {
    struct bucket *b;
    return (b = heap_peek(&ca.heap)) ? GC_SECTORS_USED(b) : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bch_moving_gc(c: *mut cache_set) {
    void bch_moving_gc(struct cache_set *c)
    {
    struct cache *ca = c.cache;
    struct bucket *b;
    unsigned long sectors_to_move, reserve_sectors;
    if (!c.copy_gc_enabled)
    return;
    mutex_lock(&c.bucket_lock);
    sectors_to_move = 0;
    reserve_sectors = ca.sb.bucket_size *
    fifo_used(&ca.free[RESERVE_MOVINGGC]);
    ca.heap.used = 0;
    for_each_bucket(b, ca) {
    if (GC_MARK(b) == GC_MARK_METADATA ||
    !GC_SECTORS_USED(b) ||
    GC_SECTORS_USED(b) == ca.sb.bucket_size ||
    atomic_read(&b.pin))
    continue;
    if (!heap_full(&ca.heap)) {
    sectors_to_move += GC_SECTORS_USED(b);
    heap_add(&ca.heap, b, bucket_cmp);
    } else if (bucket_cmp(b, heap_peek(&ca.heap))) {
    sectors_to_move -= bucket_heap_top(ca);
    sectors_to_move += GC_SECTORS_USED(b);
    ca.heap.data[0] = b;
    heap_sift(&ca.heap, 0, bucket_cmp);
    }
    }
    while (sectors_to_move > reserve_sectors) {
    heap_pop(&ca.heap, b, bucket_cmp);
    sectors_to_move -= GC_SECTORS_USED(b);
    }
    while (heap_pop(&ca.heap, b, bucket_cmp))
    SET_GC_MOVE(b, 1);
    mutex_unlock(&c.bucket_lock);
    c.moving_gc_keys.last_scanned = ZERO_KEY;
    read_moving(c);
    }
#[no_mangle]
pub unsafe extern "C" fn bch_moving_init_cache_set(c: *mut cache_set) {
    void bch_moving_init_cache_set(struct cache_set *c)
    {
    bch_keybuf_init(&c.moving_gc_keys);
    sema_init(&c.moving_in_flight, 64);
    }
