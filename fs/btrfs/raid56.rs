//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/raid56.h
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
// Copyright (C) 2012 Fusion-io  All rights reserved.
// Copyright (C) 2012 Intel Corp. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_rbio_ops {
    BTRFS_RBIO_WRITE,
    BTRFS_RBIO_READ_REBUILD,
    BTRFS_RBIO_PARITY_SCRUB,
}

//
// Overview of btrfs_raid_bio.
//
// One btrfs_raid_bio represents a full stripe of RAID56, including both data
// and P/Q stripes. For now, each data and P/Q stripe is of a fixed length (64K).
//
// One btrfs_raid_bio can have one or more bios from higher layer, covering
// part or all of the data stripes.
//
// [PAGES FROM HIGHER LAYER BIOS]
// Higher layer bios are in the btrfs_raid_bio::bio_list.
//
// Pages from the bio_list are represented like the following:
//
// bio_list:	     |<- Bio 1 ->|             |<- Bio 2 ->|  ...
// bio_paddrs:	    [0]   [1]   [2]    [3]    [4]    [5]      ...
//
// If there is a bio covering a sector (one btrfs fs block), the corresponding
// pointer in btrfs_raid_bio::bio_paddrs[] will point to the physical address
// (with the offset inside the page) of the corresponding bio.
//
// If there is no bio covering a sector, then btrfs_raid_bio::bio_paddrs[i] will
// be INVALID_PADDR.
//
// The length of each entry in bio_paddrs[] is a step (aka, min(sectorsize, PAGE_SIZE)).
//
// [PAGES FOR INTERNAL USAGES]
// Pages not covered by any bio or belonging to P/Q stripes are stored in
// btrfs_raid_bio::stripe_pages[] and stripe_paddrs[], like the following:
//
// stripe_pages:       |<- Page 0 ->|<- Page 1 ->|  ...
// stripe_paddrs:     [0]    [1]   [2]    [3]   [4] ...
//
// stripe_pages[] array stores all the pages covering the full stripe, including
// data and P/Q pages.
// stripe_pages[0] is the first page of the first data stripe.
// stripe_pages[BTRFS_STRIPE_LEN / PAGE_SIZE] is the first page of the second
// data stripe.
//
// Some pointers inside stripe_pages[] can be NULL, e.g. for a full stripe write
// (the bio covers all data stripes) there is no need to allocate pages for
// data stripes (can grab from bio_paddrs[]).
//
// If the corresponding page of stripe_paddrs[i] is not allocated, the value of
// stripe_paddrs[i] will be INVALID_PADDR.
//
// The length of each entry in stripe_paddrs[] is a step.
//
// [LOCATING A SECTOR]
// To locate a sector for IO, we need the following info:
//
// - stripe_nr
// Starts from 0 (representing the first data stripe), ends at
// @nr_data (RAID5, P stripe) or @nr_data + 1 (RAID6, Q stripe).
//
// - sector_nr
// Starts from 0 (representing the first sector of the stripe), ends
// at BTRFS_STRIPE_LEN / sectorsize - 1.
//
// - step_nr
// A step is min(sector_size, PAGE_SIZE).
//
// Starts from 0 (representing the first step of the sector), ends
// at @sector_nsteps - 1.
//
// For most call sites they do not need to bother this parameter.
// It is for bs > ps support and only for vertical stripe related works.
// (e.g. RMW/recover)
//
// - from which array
// Whether grabbing from stripe_paddrs[] (aka, internal pages) or from the
// bio_paddrs[] (aka, from the higher layer bios).
//
// For IO, a physical address is returned, so that we can extract the page and
// the offset inside the page for IO.
// A special value INVALID_PADDR represents when the physical address is invalid,
// normally meaning there is no page allocated for the specified sector.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_raid_bio {
    pub bioc: *mut btrfs_io_context,
//
// While we're doing RMW on a stripe we put it into a hash table so we
// can lock the stripe and merge more rbios into it.
//
    pub hash_list: list_head,
// LRU list for the stripe cache
    pub stripe_cache: list_head,
// For scheduling work in the helper threads
    pub work: work_struct,
//
// bio_list and bio_list_lock are used to add more bios into the stripe
// in hopes of avoiding the full RMW
//
    pub bio_list: bio_list,
    pub bio_list_lock: spinlock_t,
//
// Also protected by the bio_list_lock, the plug list is used by the
// plugging code to collect partial bios while plugged.  The stripe
// locking code also uses it to hand off the stripe lock to the next
// pending IO.
//
    pub plug_list: list_head,
// Flags that tell us if it is safe to merge with this bio.
    pub flags: c_ulong,
//
// Set if we're doing a parity rebuild for a read from higher up, which
// is handled differently from a parity rebuild as part of RMW.
//
    pub operation: btrfs_rbio_ops,
// How many pages there are for the full stripe including P/Q
    pub nr_pages: u16,
// How many sectors there are for the full stripe including P/Q
    pub nr_sectors: u16,
// Number of data stripes (no p/q)
    pub nr_data: u8,
// Number of all stripes (including P/Q)
    pub real_stripes: u8,
// How many pages there are for each stripe
    pub stripe_npages: u8,
// How many sectors there are for each stripe
    pub stripe_nsectors: u8,
//
// How many steps there are for one sector.
//
// For bs > ps cases, it's sectorsize / PAGE_SIZE.
// For bs <= ps cases, it's always 1.
//
    pub sector_nsteps: u8,
// Stripe number that we're scrubbing
    pub scrubp: u8,
//
// Size of all the bios in the bio_list.  This helps us decide if the
// rbio maps to a full stripe or not.
//
    pub bio_list_bytes: c_int,
    pub refs: refcount_t,
    pub stripes_pending: core::sync::atomic::AtomicI32,
    pub io_wait: wait_queue_head_t,
// Bitmap to record which horizontal stripe has data
    pub dbitmap: c_ulong,
// Allocated with stripe_nsectors-many bits for finish_*() calls
    pub finish_pbitmap: c_ulong,
//
// These are two arrays of pointers.  We allocate the rbio big enough
// to hold them both and setup their locations when the rbio is
// allocated.
//
// Pointers to pages that we allocated for reading/writing stripes
// directly from the disk (including P/Q).
//
    pub stripe_pages: *mut page,
// Pointers to the sectors in the bio_list, for faster lookup
    pub bio_paddrs: *mut phys_addr_t,
// Pointers to the sectors in the stripe_pages[].
    pub stripe_paddrs: *mut phys_addr_t,
// Each set bit means the corresponding sector in stripe_sectors[] is uptodate.
    pub stripe_uptodate_bitmap: *mut c_ulong,
// Allocated with real_stripes-many pointers for finish_*() calls
    pub finish_pointers: *mut c_void,
//
// The bitmap recording where IO errors happened.
// Each bit is corresponding to one sector in either bio_sectors[] or
// stripe_sectors[] array.
//
    pub error_bitmap: *mut c_ulong,
//
// Checksum buffer if the rbio is for data.  The buffer should cover
// all data sectors (excluding P/Q sectors).
//
    pub csum_buf: *mut u8,
//
// Each bit represents if the corresponding sector has data csum found.
// Should only cover data sectors (excluding P/Q sectors).
//
    pub csum_bitmap: *mut c_ulong,
}

//
// For trace event usage only. Records useful debug info for each bio submitted
// by RAID56 to each physical device.
//
// No matter signed or not, (-1) is always the one indicating we can not grab
// the proper stripe number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid56_bio_trace_info {
    pub devid: u64,
// The offset inside the stripe. (<= STRIPE_LEN)
    pub offset: u32,
//
// Stripe number.
// 0 is the first data stripe, and nr_data for P stripe,
// nr_data + 1 for Q stripe.
// >= real_stripes for
//
    pub stripe_nr: u8,
}

extern "C" {
    pub fn raid56_parity_write(bio: *mut bio, bioc: *mut btrfs_io_context);
}
extern "C" {
    pub fn raid56_parity_submit_scrub_rbio(rbio: *mut btrfs_raid_bio);
}
extern "C" {
    pub fn btrfs_alloc_stripe_hash_table(info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_free_stripe_hash_table(info: *mut btrfs_fs_info);
}
