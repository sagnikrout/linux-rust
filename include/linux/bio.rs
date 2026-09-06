//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bio.h
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
// Copyright (C) 2001 Jens Axboe <axboe@suse.de>
//

// struct bio, bio_vec and BIO_* flags are defined in blk_types.h

extern "C" {
    pub fn min(_arg: nr_segs, _arg: BIO_MAX_VECS) -> return;
}

//
// Return the data direction, READ or WRITE.
//

//
// Check whether this bio carries any data or not. A NULL bio is allowed.
//
extern "C" {
    pub fn page_address(bio_offset(bio: bio_page(bio)) +) -> return;
}
//
// drivers should _never_ use the all version - the bio may have been split
// before it got to the driver and the driver won't own all of it
//

// TODO: It is reasonable to complete bio with error here.
// @bytes should be less or equal to bvec[i->bi_idx].bv_len
extern "C" {
    pub fn __bio_advance(: *mut bio, bytes: unsigned);
}
//
// bio_advance - increment/complete a bio by some number of bytes
// @bio:	bio to advance
// @nbytes:	number of bytes to complete
//
// This updates bi_sector, bi_size and bi_idx; if the number of bytes to
// complete doesn't align with a bvec boundary, then bv_len and bv_offset will
// be updated on the last bvec as well.
//
// @bio will then represent the remaining, uncompleted portion of the io.
//

// iterate over multi-page bvec

//
// Iterate over all multi-page bvecs. Drivers shouldn't use this version for the
// same reasons as bio_for_each_segment_all().
//

//
// We special case discard/write same/write zeroes, because they
// interpret bi_size differently:
//
// get a reference to a bio, so it won't disappear. the intended use is
// something like:
//
// bio_get(bio);
// submit_bio(rw, bio);
// if (bio->bi_flags ...)
// do_something
// bio_put(bio);
//
// without the bio_get(), it could potentially complete I/O before submit_bio
// returns. and then bio would be freed memory when if (bio->bi_flags ...)
// runs
//
extern "C" {
    pub fn page_folio(_arg: bio_first_page_all(bio)) -> return;
}
//
// struct folio_iter - State for iterating all folios in a bio.
// @folio: The current folio we're iterating.  NULL after the last folio.
// @offset: The byte offset within the current folio.
// @length: The number of bytes in this iteration (will not cross folio
// boundary).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio_iter {
    pub folio: *mut folio,
    pub offset: usize,
    pub length: usize,
// private: for use by the iterator
    pub _next: *mut folio,
    pub _seg_count: usize,
    pub _i: c_int,
}

extern "C" {
    pub fn folio_page_idx(_arg: fi->folio, _arg: bvec->bv_page) -> *mut PAGE_SIZE;
}
//
// bio_for_each_folio_all - Iterate over each folio in a bio.
// @fi: struct folio_iter which is updated for each folio.
// @bio: struct bio to iterate over.
//

extern "C" {
    pub fn bio_trim(bio: *mut bio, offset: sector_t, size: sector_t);
}
//
// bio_next_split - get next @sectors from a bio, splitting if necessary
// @bio:	bio to split
// @sectors:	number of sectors to split from the front of @bio
// @gfp:	gfp mask
// @bs:		bio set to allocate from
//
// Return: a bio representing the next @sectors of @bio - if the bio is smaller
// than @sectors, returns the original bio unchanged.
//
extern "C" {
    pub fn bio_split(_arg: bio, _arg: sectors, _arg: gfp, _arg: bs) -> return;
}
extern "C" {
    pub fn bioset_init(: *mut bio_set, int: unsigned, int: unsigned, flags: c_int) -> c_int;
}
extern "C" {
    pub fn bioset_exit(: *mut bio_set);
}
extern "C" {
    pub fn bio_put(: *mut bio);
}
extern "C" {
    pub fn bio_alloc_bioset(_arg: bdev, _arg: nr_vecs, _arg: opf, _arg: gfp_mask, _arg: &fs_bio_set) -> return;
}
extern "C" {
    pub fn submit_bio(bio: *mut bio);
}
//
// bio_in_atomic - check if the current context is unsafe for bio completion
//
// Return: %true in atomic contexts (e.g. hard/soft IRQ, preempt-disabled);
// %false when a bio can be safely completed in the current context.
//
extern "C" {
    pub fn __bio_complete_in_task(bio: *mut bio);
}
//
// bio_complete_in_task - ensure a bio is completed in preemptible task context
// @bio: bio to complete
//
// If called from non-task context, offload the bio completion to a worker
// thread and return %true. Else return %false and do nothing.
//
// Uses BIO_COMPLETE_IN_TASK as a sentinel: if set, the bio was already
// deferred and we are running in the worker — return %false so the
// callback proceeds instead of re-deferring.
//
extern "C" {
    pub fn bio_endio(: *mut bio);
}
//
// bio_endio_status - end I/O on a bio with a specific status
// @bio:	bio
// @status:	status to set
//
// Set @bio->bi_status to @status and call bio_endio().
//
// Calculate number of bvec segments that should be allocated to fit data
// pointed by @iter. If @iter is backed by bvec it's going to be reused
// instead of allocating a new one.
//
extern "C" {
    pub fn iov_iter_npages(_arg: iter, _arg: max_segs) -> return;
}
//
// bio_iov_bounce_nr_vecs - calculate number of bvecs for a bounce bio
// @iter:	iter to bounce from
// @op:		REQ_OP_* for the bio
//
// Calculates how many bvecs are needed for the next bio to bounce from/to
// @iter.
//
// We still need to bounce bvec iters, so don't special case them
// here unlike in bio_iov_vecs_to_alloc.
//
// For reads we need to use a vector for the bounce buffer, account
// for that here.
//
extern "C" {
    pub fn iov_iter_npages(_arg: iter, _arg: BIO_MAX_VECS) -> return;
}
extern "C" {
    pub fn bio_uninit(: *mut bio);
}
extern "C" {
    pub fn bio_reset(bio: *mut bio, bdev: *mut block_device, opf: blk_opf_t);
}
extern "C" {
    pub fn bio_reuse(bio: *mut bio, opf: blk_opf_t);
}
extern "C" {
    pub fn bio_chain(: *mut bio, : *mut bio);
}
extern "C" {
    pub fn bio_add_virt_nofail(bio: *mut bio, vaddr: *mut c_void, len: unsigned);
}
//
// bio_add_max_vecs - number of bio_vecs needed to add data to a bio
// @kaddr: kernel virtual address to add
// @len: length in bytes to add
//
// Calculate how many bio_vecs need to be allocated to add the kernel virtual
// address range in [@kaddr:@len] in the worse case.
//
extern "C" {
    pub fn DIV_ROUND_UP(len: offset_in_page(kaddr) +, _arg: PAGE_SIZE) -> return;
}
extern "C" {
    pub fn bio_add_vmalloc_chunk(bio: *mut bio, vaddr: *mut c_void, len: unsigned) -> c_uint;
}
extern "C" {
    pub fn bio_add_vmalloc(bio: *mut bio, vaddr: *mut c_void, len: c_uint) -> bool;
}
extern "C" {
    pub fn submit_bio_wait(bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn bio_iov_iter_set(bio: *mut bio, iter: *const iov_iter) -> bool;
}
extern "C" {
    pub fn __bio_release_pages(bio: *mut bio, mark_dirty: bool);
}
extern "C" {
    pub fn bio_set_pages_dirty(bio: *mut bio);
}
extern "C" {
    pub fn bio_check_pages_dirty(bio: *mut bio);
}
extern "C" {
    pub fn bio_iov_iter_unbounce(bio: *mut bio, is_error: bool, mark_dirty: bool);
}
extern "C" {
    pub fn bio_copy_data(dst: *mut bio, src: *mut bio);
}
extern "C" {
    pub fn bio_free_pages(bio: *mut bio);
}
extern "C" {
    pub fn zero_fill_bio(bio: *mut bio);
}
extern "C" {
    pub fn guard_bio_eod(bio: *mut bio);
}

extern "C" {
    pub fn bio_associate_blkg(bio: *mut bio);
}
extern "C" {
    pub fn bio_clone_blkg_association(dst: *mut bio, src: *mut bio);
}
extern "C" {
    pub fn blkcg_punt_bio_submit(bio: *mut bio);
}

//
// BIO list management for use by remapping drivers (e.g. DM or MD) and loop.
//
// A bio_list anchors a singly-linked list of bios chained through the bi_next
// member of the bio.  The bio_list also caches the last list member to allow
// fast access to the tail.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_list {
    pub head: *mut bio,
    pub tail: *mut bio,
}

//
// Increment chain count for the bio. Make sure the CHAIN flag update
// is visible before the raised count.
//
// bio_set is used to allow other portions of the IO system to
// allocate their own private memory pools for bio and iovec structures.
// These memory pools in turn all allocate from the bio_slab
// and the bvec_slabs[].
//
pub const BIO_POOL_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_set {
    pub bio_slab: *mut kmem_cache,
    pub front_pad: c_uint,
//
// per-cpu bio alloc cache
//
    pub cache: *mut bio_alloc_cache __percpu,
    pub bio_pool: mempool_t,
    pub bvec_pool: mempool_t,
    pub back_pad: c_uint,
//
// Deadlock avoidance for stacking block drivers: see comments in
// bio_alloc_bioset() for details
//
    pub rescue_lock: spinlock_t,
    pub rescue_list: bio_list,
    pub rescue_work: work_struct,
    pub rescue_workqueue: *mut workqueue_struct,
//
// Hot un-plug notifier for the per-cpu cache, if used
//
    pub cpuhp_dead: hlist_node,
}

//
// bio_is_zone_append - is this a zone append bio?
// @bio:	bio to check
//
// Check if @bio is a zone append operation.  Core block layer code and end_io
// handlers must use this instead of an open coded REQ_OP_ZONE_APPEND check
// because the block layer can rewrite REQ_OP_ZONE_APPEND to REQ_OP_WRITE if
// it is not natively supported.
//
