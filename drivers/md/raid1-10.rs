//! Automatically rewritten from C to Rust
//! Source: drivers/md/raid1-10.c
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
// Maximum size of each resync request

// when we get a read error on a read-only array, we redirect to another
// device without failing the first device, or trying to over-write to
// correct the read error.  To keep track of bad blocks on a per-bio
// level, we store IO_BLOCKED in the appropriate 'bios' pointer
//

// When we successfully write to a known bad-block, we need to remove the
// bad-block marking which must be done from process context.  So we record
// the success by setting devs[n].bio to IO_MADE_GOOD
//

pub const MAX_PLUG_BIO: c_int = 32;
// for managing resync I/O pages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resync_pages {
    pub raid_bio: *mut c_void,
    pub pages: [*mut page; RESYNC_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid1_plug_cb {
    pub cb: blk_plug_cb,
    pub pending: bio_list,
    pub count: c_uint,
}

#[no_mangle]
unsafe extern "C" fn rbio_pool_free(rbio: *mut c_void, data: *mut c_void) {
    static void rbio_pool_free(void *rbio, void *data)
    {
    kfree(rbio);
    }
    static inline int resync_alloc_pages(struct resync_pages *rp,
    gfp_t gfp_flags)
    {
    int i;
    for (i = 0; i < RESYNC_PAGES; i++) {
    rp.pages[i] = alloc_page(gfp_flags);
    if (!rp.pages[i])
    goto out_free;
    }
    return 0;
    out_free:
    while (--i >= 0)
    put_page(rp.pages[i]);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn resync_free_pages(rp: *mut resync_pages) {
    static inline void resync_free_pages(struct resync_pages *rp)
    {
    int i;
    for (i = 0; i < RESYNC_PAGES; i++)
    put_page(rp.pages[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn resync_get_all_pages(rp: *mut resync_pages) {
    static inline void resync_get_all_pages(struct resync_pages *rp)
    {
    int i;
    for (i = 0; i < RESYNC_PAGES; i++)
    get_page(rp.pages[i]);
    }
    static inline struct page *resync_fetch_page(struct resync_pages *rp,
    unsigned idx)
    {
    if (WARN_ON_ONCE(idx >= RESYNC_PAGES))
    return core::ptr::null_mut();
    return rp.pages[idx];
    }
//
// 'strct resync_pages' stores actual pages used for doing the resync
// IO, and it is per-bio, so make .bi_private points to it.
//
    static inline struct resync_pages *get_resync_pages(struct bio *bio)
    {
    return bio.bi_private;
    }
// generally called after bio_reset() for reseting bvec
    static void md_bio_reset_resync_pages(struct bio *bio, struct resync_pages *rp,
    int size)
    {
    let mut idx: c_int = 0;
// initialize bvec table again
    do {
    struct page *page = resync_fetch_page(rp, idx);
    let mut len: c_int = min_t(int, size, PAGE_SIZE);
    if (WARN_ON(!bio_add_page(bio, page, len, 0))) {
    bio.bi_status = BLK_STS_RESOURCE;
    bio_endio(bio);
    return;
    }
    size -= len;
    } while (idx++ < RESYNC_PAGES && size > 0);
    }
#[no_mangle]
pub unsafe extern "C" fn raid1_submit_write(bio: *mut bio) {
    static inline void raid1_submit_write(struct bio *bio)
    {
    struct md_rdev *rdev = (void *)bio.bi_bdev;
    bio.bi_next = core::ptr::null_mut();
    bio_set_dev(bio, rdev.bdev);
    if (test_bit(Faulty, &rdev.flags))
    bio_io_error(bio);
    else if (unlikely(bio_op(bio) ==  REQ_OP_DISCARD &&
    !bdev_max_discard_sectors(bio.bi_bdev)))
// Just ignore it
    bio_endio(bio);
    else
    submit_bio_noacct(bio);
    }
    static inline bool raid1_add_bio_to_plug(struct mddev *mddev, struct bio *bio,
    blk_plug_cb_fn unplug, int copies)
    {
    struct raid1_plug_cb *plug = core::ptr::null_mut();
    struct blk_plug_cb *cb;
//
// If bitmap is not enabled, it's safe to submit the io directly, and
// this can get optimal performance.
//
    if (!md_bitmap_enabled(mddev, true)) {
    raid1_submit_write(bio);
    return true;
    }
    cb = blk_check_plugged(unplug, mddev, sizeof(*plug));
    if (!cb)
    return false;
    plug = container_of(cb, struct raid1_plug_cb, cb);
    bio_list_add(&plug.pending, bio);
    if (++plug.count / MAX_PLUG_BIO >= copies) {
    list_del(&cb.list);
    cb.callback(cb, false);
    }
    return true;
    }
//
// current->bio_list will be set under submit_bio() context, in this case bitmap
// io will be added to the list and wait for current io submission to finish,
// while current io submission must wait for bitmap io to be done. In order to
// avoid such deadlock, submit bitmap io asynchronously.
//
#[no_mangle]
pub unsafe extern "C" fn raid1_prepare_flush_writes(mddev: *mut mddev) {
    static inline void raid1_prepare_flush_writes(struct mddev *mddev)
    {
    mddev.bitmap_ops.unplug(mddev, current.bio_list == core::ptr::null_mut());
    }
//
// Used by fix_read_error() to decay the per rdev read_errors.
// We halve the read error count for every hour that has elapsed
// since the last recorded read error.
//
#[no_mangle]
pub unsafe extern "C" fn check_decay_read_errors(mddev: *mut mddev, rdev: *mut md_rdev) {
    static inline void check_decay_read_errors(struct mddev *mddev, struct md_rdev *rdev)
    {
    long cur_time_mon;
    unsigned long hours_since_last;
    let mut read_errors: c_uint = atomic_read(&rdev.read_errors);
    cur_time_mon = ktime_get_seconds();
    if (rdev.last_read_error == 0) {
// first time we've seen a read error
    rdev.last_read_error = cur_time_mon;
    return;
    }
    hours_since_last = (long)(cur_time_mon -
    rdev.last_read_error) / 3600;
    rdev.last_read_error = cur_time_mon;
//
// if hours_since_last is > the number of bits in read_errors
// just set read errors to 0. We do this to avoid
// overflowing the shift of read_errors by hours_since_last.
//
    if (hours_since_last >= 8 * sizeof(read_errors))
    atomic_set(&rdev.read_errors, 0);
    else
    atomic_set(&rdev.read_errors, read_errors >> hours_since_last);
    }
#[no_mangle]
pub unsafe extern "C" fn exceed_read_errors(mddev: *mut mddev, rdev: *mut md_rdev) -> bool {
    static inline bool exceed_read_errors(struct mddev *mddev, struct md_rdev *rdev)
    {
    let mut max_read_errors: c_int = atomic_read(&mddev.max_corr_read_errors);
    int read_errors;
    check_decay_read_errors(mddev, rdev);
    read_errors =  atomic_inc_return(&rdev.read_errors);
    if (read_errors > max_read_errors) {
    pr_notice("md/"RAID_1_10_NAME":%s: %pg: Raid device exceeded read_error threshold [cur %d:max %d]\n",
    mdname(mddev), rdev.bdev, read_errors, max_read_errors);
    pr_notice("md/"RAID_1_10_NAME":%s: %pg: Failing raid device\n",
    mdname(mddev), rdev.bdev);
    md_error(mddev, rdev);
    return true;
    }
    return false;
    }
//
// raid1_check_read_range() - check a given read range for bad blocks,
// available read length is returned;
// @rdev: the rdev to read;
// @this_sector: read position;
// @len: read length;
//
// helper function for read_balance()
//
// 1) If there are no bad blocks in the range, @len is returned;
// 2) If the range are all bad blocks, 0 is returned;
// 3) If there are partial bad blocks:
// - If the bad block range starts after @this_sector, the length of first
// good region is returned;
// - If the bad block range starts before @this_sector, 0 is returned and
// the @len is updated to the offset into the region before we get to the
// good blocks;
//
    static inline int raid1_check_read_range(struct md_rdev *rdev,
    sector_t this_sector, int *len)
    {
    sector_t first_bad;
    sector_t bad_sectors;
// no bad block overlap
    if (!is_badblock(rdev, this_sector, *len, &first_bad, &bad_sectors))
    return *len;
//
// bad block range starts offset into our range so we can return the
// number of sectors before the bad blocks start.
//
    if (first_bad > this_sector)
    return first_bad - this_sector;
// read range is fully consumed by bad blocks.
    if (this_sector + *len <= first_bad + bad_sectors)
    return 0;
//
// final case, bad block range starts before or at the start of our
// range but does not cover our entire range so we still return 0 but
// update the length with the number of sectors before we get to the
// good ones.
//
// len = first_bad + bad_sectors - this_sector;
    return 0;
    }
//
// Check if read should choose the first rdev.
//
// Balance on the whole device if no resync is going on (recovery is ok) or
// below the resync window. Otherwise, take the first readable disk.
//
    static inline bool raid1_should_read_first(struct mddev *mddev,
    sector_t this_sector, int len)
    {
    if ((mddev.resync_offset < this_sector + len))
    return true;
    if (mddev_is_clustered(mddev) &&
    mddev.cluster_ops.area_resyncing(mddev, READ, this_sector,
    this_sector + len))
    return true;
    return false;
    }
//
// bio with REQ_RAHEAD can fail at anytime, before such IO is submitted to the
// underlying disks, hence don't record badblocks or retry in this case.
//
// BLK_STS_INVAL means the bio was not valid for the underlying device. This
// is a user error, not a device failure, so retrying or recording bad blocks
// would be wrong.
//
#[no_mangle]
pub unsafe extern "C" fn raid1_should_handle_error(bio: *mut bio) -> bool {
    static inline bool raid1_should_handle_error(struct bio *bio)
    {
    return !(bio.bi_opf & REQ_RAHEAD) && bio.bi_status != BLK_STS_INVAL;
    }
