//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-io.c
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
// Copyright (C) 2003 Sistina Software
// Copyright (C) 2006 Red Hat GmbH
//
// This file is released under the GPL.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_io_client {
    pub pool: mempool_t,
    pub bios: bio_set,
}

//
// Aligning 'struct io' reduces the number of bits required to store
// its address.  Refer to store_io_and_region_in_bio() below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io {
    pub error_bits: c_ulong,
    pub unsup_bits: c_ulong,
    pub count: core::sync::atomic::AtomicI32,
    pub client: *mut dm_io_client,
    pub callback: io_notify_fn,
    pub context: *mut c_void,
    pub vma_invalidate_address: *mut c_void,
    pub vma_invalidate_size: c_ulong,
    pub __aligned(DM_IO_MAX_REGIONS): },
    pub _dm_io_cache: *mut static struct kmem_cache,
//
// Create a client with mempool and bioset.
//
    struct dm_io_client *dm_io_client_create(void)
    {
    pub client: *mut dm_io_client,
    pub dm_get_reserved_bio_based_ios(): unsigned int min_ios =,
    pub ret: c_int,
    pub kzalloc_obj(*client): *mut client =,
    if (!client)
    pub ERR_PTR(-ENOMEM): return,
    pub _dm_io_cache): ret = mempool_init_slab_pool(&client->pool, min_ios,,
    if (ret)
    pub bad: goto,
    pub BIOSET_NEED_BVECS): ret = bioset_init(&client->bios, min_ios, 0,,
    if (ret)
    pub bad: goto,
    pub client: return,
    bad:
    pub ERR_PTR(ret): return,
    }
#[no_mangle]
pub unsafe extern "C" fn dm_io_client_destroy(client: *mut dm_io_client) {
    void dm_io_client_destroy(struct dm_io_client *client)
    {
    }
//
// -------------------------------------------------------------------
// We need to keep track of which region a bio is doing io for.
// To avoid a memory allocation to store just 5 or 6 bits, we
// ensure the 'struct io' pointer is aligned so enough low bits are
// always zero and then combine it with the region number directly in
// bi_private.
// -------------------------------------------------------------------
//
    static void store_io_and_region_in_bio(struct bio *bio, struct io *io,
    unsigned int region)
    {
    if (unlikely(!IS_ALIGNED((unsigned long)io, DM_IO_MAX_REGIONS))) {
    pub io): DMCRIT("Unaligned struct io pointer %p",,
    }
    pub region): *mut *mut bio->bi_private = (void )((unsigned long)io |,
    }
    static void retrieve_io_and_region_from_bio(struct bio *bio, struct io **io,
    unsigned int *region)
    {
    pub long)bio->bi_private: unsigned long val = (unsigned,
// io = (void *)(val & -(unsigned long)DM_IO_MAX_REGIONS);
// region = val & (DM_IO_MAX_REGIONS - 1);
    }
//
// --------------------------------------------------------------
// We need an io object to keep track of the number of bios that
// have been dispatched for a particular io.
// --------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn complete_io(io: *mut io) {
    static void complete_io(struct io *io)
    {
    pub io->error_bits: unsigned long error_bits =,
    pub io->unsup_bits: unsigned long unsup_bits =,
    pub io->callback: io_notify_fn fn =,
    pub io->context: *mut *mut void context =,
    if (io.vma_invalidate_size)
    invalidate_kernel_vmap_range(io.vma_invalidate_address,
    pub &io->client->pool): mempool_free(io,,
    pub context): fn(error_bits, unsup_bits,,
    }
#[no_mangle]
unsafe extern "C" fn dec_count(io: *mut io, region: c_uint, error: blk_status_t) {
    static void dec_count(struct io *io, unsigned int region, blk_status_t error)
    {
    if (unlikely(error)) {
    if (error == BLK_STS_NOTSUPP || error == BLK_STS_INVAL)
    pub &io->unsup_bits): set_bit(region,,
    else
    pub &io->error_bits): set_bit(region,,
    }
    if (atomic_dec_and_test(&io.count))
    }
#[no_mangle]
unsafe extern "C" fn endio(bio: *mut bio) {
    static void endio(struct bio *bio)
    {
    pub io: *mut io,
    pub region: c_uint,
    pub error: blk_status_t,
    if (bio.bi_status && bio_data_dir(bio) == READ)
//
// The bio destructor in bio_put() may use the io object.
//
    pub &region): retrieve_io_and_region_from_bio(bio, &io,,
    pub bio->bi_status: error =,
    pub error): dec_count(io, region,,
    }
//
// --------------------------------------------------------------
// These little objects provide an abstraction for getting a new
// destination page for io.
// --------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpages {
    void (*get_page)(struct dpages *dp,
    pub offset): *mut *mut *mut *mut page p, unsigned long len, unsigned int,
    pub dp): *mut *mut void (next_page)(struct dpages,
    pub context_u: c_uint,
    pub context_ptr: *mut c_void,
    pub orig_bio: *mut bio,
    pub vma_invalidate_address: *mut c_void,
    pub vma_invalidate_size: c_ulong,
}

//
// Functions for getting the pages from a list.
//
    static void list_get_page(struct dpages *dp,
    struct page **p, unsigned long *len, unsigned int *offset)
    {
    let mut o: c_uint = dp.context_u;
    struct page_list *pl = dp.context_ptr;
// p = pl->page;
// len = PAGE_SIZE - o;
// offset = o;
    }
#[no_mangle]
unsafe extern "C" fn list_next_page(dp: *mut dpages) {
    static void list_next_page(struct dpages *dp)
    {
    struct page_list *pl = dp.context_ptr;
    dp.context_ptr = pl.next;
    dp.context_u = 0;
    }
#[no_mangle]
unsafe extern "C" fn list_dp_init(dp: *mut dpages, pl: *mut page_list, offset: c_uint) {
    static void list_dp_init(struct dpages *dp, struct page_list *pl, unsigned int offset)
    {
    dp.get_page = list_get_page;
    dp.next_page = list_next_page;
    dp.context_u = offset;
    dp.context_ptr = pl;
    }
//
// Functions for getting the pages from a VMA.
//
    static void vm_get_page(struct dpages *dp,
    struct page **p, unsigned long *len, unsigned int *offset)
    {
// p = vmalloc_to_page(dp->context_ptr);
// offset = dp->context_u;
// len = PAGE_SIZE - dp->context_u;
    }
#[no_mangle]
unsafe extern "C" fn vm_next_page(dp: *mut dpages) {
    static void vm_next_page(struct dpages *dp)
    {
    dp.context_ptr += PAGE_SIZE - dp.context_u;
    dp.context_u = 0;
    }
#[no_mangle]
unsafe extern "C" fn vm_dp_init(dp: *mut dpages, data: *mut c_void) {
    static void vm_dp_init(struct dpages *dp, void *data)
    {
    dp.get_page = vm_get_page;
    dp.next_page = vm_next_page;
    dp.context_u = offset_in_page(data);
    dp.context_ptr = data;
    }
//
// Functions for getting the pages from kernel memory.
//
    static void km_get_page(struct dpages *dp, struct page **p, unsigned long *len,
    unsigned int *offset)
    {
// p = virt_to_page(dp->context_ptr);
// offset = dp->context_u;
// len = PAGE_SIZE - dp->context_u;
    }
#[no_mangle]
unsafe extern "C" fn km_next_page(dp: *mut dpages) {
    static void km_next_page(struct dpages *dp)
    {
    dp.context_ptr += PAGE_SIZE - dp.context_u;
    dp.context_u = 0;
    }
#[no_mangle]
unsafe extern "C" fn km_dp_init(dp: *mut dpages, data: *mut c_void) {
    static void km_dp_init(struct dpages *dp, void *data)
    {
    dp.get_page = km_get_page;
    dp.next_page = km_next_page;
    dp.context_u = offset_in_page(data);
    dp.context_ptr = data;
    }
//
// ---------------------------------------------------------------
// IO routines that accept a list of pages.
// ---------------------------------------------------------------
//
    static void do_region(const blk_opf_t opf, unsigned int region,
    struct dm_io_region *where, struct dpages *dp,
    struct io *io, unsigned short ioprio)
    {
    struct bio *bio;
    struct page *page;
    unsigned long len;
    unsigned int offset;
    unsigned int num_bvecs;
    let mut remaining: sector_t = where.count;
    struct request_queue *q = bdev_get_queue(where.bdev);
    sector_t num_sectors;
    unsigned int special_cmd_max_sectors;
    let mut op: enum req_op = opf & REQ_OP_MASK;
//
// Reject unsupported discard and write same requests.
//
    if (op == REQ_OP_DISCARD)
    special_cmd_max_sectors = bdev_max_discard_sectors(where.bdev);
#[no_mangle]
pub unsafe extern "C" fn if(REQ_OP_WRITE_ZEROES: op ==) -> else {
    else if (op == REQ_OP_WRITE_ZEROES)
    special_cmd_max_sectors = q.limits.max_write_zeroes_sectors;
    if ((op == REQ_OP_DISCARD || op == REQ_OP_WRITE_ZEROES) &&
    special_cmd_max_sectors == 0) {
    atomic_inc(&io.count);
    dec_count(io, region, BLK_STS_NOTSUPP);
    return;
    }
    if (dp.orig_bio) {
    bio = bio_alloc_clone(where.bdev, dp.orig_bio, GFP_NOIO,
    &io.client.bios);
    bio.bi_iter.bi_sector = where.sector;
    bio.bi_iter.bi_size = where.count << SECTOR_SHIFT;
    bio.bi_opf = opf;
    bio.bi_end_io = endio;
    bio.bi_ioprio = ioprio;
    store_io_and_region_in_bio(bio, io, region);
    atomic_inc(&io.count);
    submit_bio(bio);
    return;
    }
//
// where->count may be zero if op holds a flush and we need to
// send a zero-sized flush.
//
    do {
//
// Allocate a suitably sized-bio.
//
    switch (op) {
    case REQ_OP_DISCARD:
    case REQ_OP_WRITE_ZEROES:
    num_bvecs = 0;
    break;
    default:
    num_bvecs = bio_max_segs(dm_sector_div_up(remaining,
    (PAGE_SIZE >> SECTOR_SHIFT)) + 1);
    }
    bio = bio_alloc_bioset(where.bdev, num_bvecs, opf, GFP_NOIO,
    &io.client.bios);
    bio.bi_iter.bi_sector = where.sector + (where.count - remaining);
    bio.bi_end_io = endio;
    bio.bi_ioprio = ioprio;
    store_io_and_region_in_bio(bio, io, region);
    if (op == REQ_OP_DISCARD || op == REQ_OP_WRITE_ZEROES) {
    num_sectors = min_t(sector_t, special_cmd_max_sectors, remaining);
    bio.bi_iter.bi_size = num_sectors << SECTOR_SHIFT;
    remaining -= num_sectors;
    } else {
    while (remaining) {
//
// Try and add as many pages as possible.
//
    dp.get_page(dp, &page, &len, &offset);
    len = min(len, to_bytes(remaining));
    if (!bio_add_page(bio, page, len, offset))
    break;
    offset = 0;
    remaining -= to_sector(len);
    dp.next_page(dp);
    }
    }
    atomic_inc(&io.count);
    submit_bio(bio);
    WARN_ON_ONCE(opf & REQ_ATOMIC && remaining);
    } while (remaining);
    }
    static void dispatch_io(blk_opf_t opf, unsigned int num_regions,
    struct dm_io_region *where, struct dpages *dp,
    struct io *io, unsigned short ioprio)
    {
    int i;
    let mut old_pages: dpages = *dp;
    BUG_ON(num_regions > DM_IO_MAX_REGIONS);
//
// For multiple regions we need to be careful to rewind
// the dp object for each call to do_region.
//
    for (i = 0; i < num_regions; i++) {
// dp = old_pages;
    if (where[i].count || (opf & REQ_PREFLUSH))
    do_region(opf, i, where + i, dp, io, ioprio);
    }
//
// Drop the extra reference that we were holding to avoid
// the io being completed too early.
//
    dec_count(io, 0, 0);
    }
    static void async_io(struct dm_io_client *client, unsigned int num_regions,
    struct dm_io_region *where, blk_opf_t opf,
    struct dpages *dp, io_notify_fn fn, void *context,
    unsigned short ioprio)
    {
    struct io *io;
    io = mempool_alloc(&client.pool, GFP_NOIO);
    io.error_bits = 0;
    io.unsup_bits = 0;
    atomic_set(&io.count, 1); /* see dispatch_io() */
    io.client = client;
    io.callback = fn;
    io.context = context;
    io.vma_invalidate_address = dp.vma_invalidate_address;
    io.vma_invalidate_size = dp.vma_invalidate_size;
    dispatch_io(opf, num_regions, where, dp, io, ioprio);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_io {
    pub error_bits: c_ulong,
    pub unsup_bits: c_ulong,
    pub wait: completion,
}

#[no_mangle]
unsafe extern "C" fn sync_io_complete(error: c_ulong, unsup: c_ulong, context: *mut c_void) {
    static void sync_io_complete(unsigned long error, unsigned long unsup, void *context)
    {
    struct sync_io *sio = context;
    sio.error_bits = error;
    sio.unsup_bits = unsup;
    complete(&sio.wait);
    }
    static int sync_io(struct dm_io_client *client, unsigned int num_regions,
    struct dm_io_region *where, blk_opf_t opf, struct dpages *dp,
    unsigned long *error_bits, unsigned long *unsup_bits,
    unsigned short ioprio)
    {
    struct sync_io sio;
    init_completion(&sio.wait);
    async_io(client, num_regions, where, opf | REQ_SYNC, dp,
    sync_io_complete, &sio, ioprio);
    wait_for_completion_io(&sio.wait);
    if (error_bits)
// error_bits = sio.error_bits;
    if (unsup_bits)
// unsup_bits = sio.unsup_bits;
    return sio.error_bits ? -EIO : sio.unsup_bits ? -EOPNOTSUPP : 0;
    }
    static int dp_init(struct dm_io_request *io_req, struct dpages *dp,
    unsigned long size)
    {
// Set up dpages based on memory type
    dp.vma_invalidate_address = core::ptr::null_mut();
    dp.vma_invalidate_size = 0;
    dp.orig_bio = core::ptr::null_mut();
    switch (io_req.mem.type) {
    case DM_IO_PAGE_LIST:
    list_dp_init(dp, io_req.mem.ptr.pl, io_req.mem.offset);
    break;
    case DM_IO_BIO:
//
// The destination bios clone this bio's biovec directly, so
// there are no per-page accessors to set up here.
//
    dp.orig_bio = io_req.mem.ptr.bio;
    break;
    case DM_IO_VMA:
    flush_kernel_vmap_range(io_req.mem.ptr.vma, size);
    if ((io_req.bi_opf & REQ_OP_MASK) == REQ_OP_READ) {
    dp.vma_invalidate_address = io_req.mem.ptr.vma;
    dp.vma_invalidate_size = size;
    }
    vm_dp_init(dp, io_req.mem.ptr.vma);
    break;
    case DM_IO_KMEM:
    km_dp_init(dp, io_req.mem.ptr.addr);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    int dm_io(struct dm_io_request *io_req, unsigned int num_regions,
    struct dm_io_region *where, unsigned long *sync_error_bits,
    unsigned long *sync_unsup_bits, unsigned short ioprio)
    {
    int r;
    struct dpages dp;
    if (num_regions > 1 && !op_is_write(io_req.bi_opf)) {
    WARN_ON(1);
    return -EIO;
    }
    r = dp_init(io_req, &dp, (unsigned long)where.count << SECTOR_SHIFT);
    if (r)
    return r;
    if (!io_req.notify.fn)
    return sync_io(io_req.client, num_regions, where,
    io_req.bi_opf, &dp, sync_error_bits,
    sync_unsup_bits, ioprio);
    async_io(io_req.client, num_regions, where, io_req.bi_opf, &dp,
    io_req.notify.fn, io_req.notify.context, ioprio);
    return 0;
    }
    EXPORT_SYMBOL(dm_io);
#[no_mangle]
pub unsafe extern "C" fn dm_io_init() -> int __init {
    int __init dm_io_init(void)
    {
    _dm_io_cache = KMEM_CACHE(io, 0);
    if (!_dm_io_cache)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dm_io_exit() {
    void dm_io_exit(void)
    {
    kmem_cache_destroy(_dm_io_cache);
    _dm_io_cache = core::ptr::null_mut();
    }
