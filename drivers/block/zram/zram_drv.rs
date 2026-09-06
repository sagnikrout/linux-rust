//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/zram/zram_drv.h
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


//
// Compressed RAM block device
//
// Copyright (C) 2008, 2009, 2010  Nitin Gupta
// 2012, 2013 Minchan Kim
//
// This code is released using a dual license strategy: BSD/GPL
// You can choose the licence that better fits your requirements.
//
// Released under the terms of 3-clause BSD License
// Released under the terms of GNU General Public License Version 2.0
//

pub const ZRAM_LOGICAL_BLOCK_SHIFT: c_int = 12;

//
// ZRAM is mainly used for memory efficiency so we want to keep memory
// footprint small and thus squeeze size and zram pageflags into a flags
// member. The lower ZRAM_FLAG_SHIFT bits is for object size (excluding
// header), which cannot be larger than PAGE_SIZE (requiring PAGE_SHIFT
// bits), the higher bits are for zram_pageflags.
//
// We use BUILD_BUG_ON() to make sure that zram pageflags don't overflow.
//

// Only 2 bits are allowed for comp priority index
pub const ZRAM_COMP_PRIORITY_MASK: c_uint = 0x3;
// Flags for zram pages (table[page_no].flags)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zram_pageflags {
    ZRAM_SAME = ZRAM_FLAG_SHIFT,	/* Page consists the same element */
    ZRAM_ENTRY_LOCK, /* entry access lock bit */
    ZRAM_WB,	/* page is stored on backing_device */
    ZRAM_PP_SLOT,	/* Selected for post-processing */
    ZRAM_HUGE,	/* Incompressible page */
    ZRAM_IDLE,	/* not accessed page since last idle marking */
    ZRAM_INCOMPRESSIBLE, /* none of the algorithms could compress it */

    ZRAM_COMP_PRIORITY_BIT1, /* First bit of comp priority index */
    ZRAM_COMP_PRIORITY_BIT2, /* Second bit of comp priority index */

    __NR_ZRAM_PAGEFLAGS,
}

//
// The slot lock is a bit-wait lock on the whole __lock word, while
// flags and ac_time alias that word as two u32s.  The lock bit must
// land in the slot that ZRAM_ENTRY_LOCK reserves in attr.flags; on
// 64-bit big-endian the flags word maps to the upper half of __lock,
// so the bit position has to be shifted up.
//

//
// Allocated for each disk page.  We use bit-lock (ZRAM_ENTRY_LOCK bit
// of flags) to save memory.  There can be plenty of entries and standard
// locking primitives (e.g. mutex) will significantly increase sizeof()
// of each entry and hence of the meta table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zram_table_entry {
    pub handle: c_ulong,
    pub __lock: c_ulong,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attr {
    pub flags: u32,

    pub ac_time: u32,

    pub attr: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zram_stats {
    pub /: *mut *mut atomic64_t compr_data_size; / compressed size of pages stored,
    pub /: *mut *mut atomic64_t failed_reads; / can happen when memory is too low,
    pub /: *mut *mut atomic64_t failed_writes; / can happen when memory is too low,
    pub /: *mut *mut atomic64_t notify_free; / no. of swap slot free notifications,
    pub /: *mut *mut atomic64_t same_pages; / no. of same element filled pages,
    pub /: *mut *mut atomic64_t huge_pages; / no. of huge pages,
    pub /: *mut *mut atomic64_t huge_pages_since; / no. of huge pages since zram set up,
    pub /: *mut *mut atomic64_t pages_stored; / no. of pages currently stored,
    pub /: *mut *mut atomic_long_t max_used_pages; / no. of maximum pages stored,
    pub /: *mut *mut atomic64_t miss_free; / no. of missed free,

    pub /: *mut *mut atomic64_t bd_count; / no. of pages in backing device,
    pub /: *mut *mut atomic64_t bd_reads; / no. of reads from backing device,
    pub /: *mut *mut atomic64_t bd_writes; / no. of writes from backing device,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zram {
    pub table: *mut zram_table_entry,
    pub table_lock_map: lockdep_map,
    pub table_lock_key: lock_class_key,
    pub mem_pool: *mut zs_pool,
    pub comps: [*mut zcomp; ZRAM_MAX_COMPS],
    pub params: [zcomp_params; ZRAM_MAX_COMPS],
    pub disk: *mut gendisk,
// Locks the device either in exclusive or in shared mode
    pub dev_lock: rw_semaphore,
//
// the number of pages zram can consume for storing compressed data
//
    pub limit_pages: c_ulong,
    pub stats: zram_stats,
//
// This is the limit on amount of *uncompressed* worth of data
// we can store in a disk.
//
    pub /: *mut *mut u64 disksize; / bytes,
    pub comp_algs: [*const c_char; ZRAM_MAX_COMPS],
//
// zram is claimed so open request will be failed
//
    pub /: *mut *mut bool claim; / Protected by disk->open_mutex,

    pub backing_dev: *mut file,
    pub wb_limit_enable: bool,
    pub compressed_wb: bool,
    pub wb_batch_size: u32,
    pub bd_wb_limit: u64,
    pub bdev: *mut block_device,
    pub bitmap: *mut c_ulong,
    pub nr_pages: c_ulong,

    pub debugfs_dir: *mut dentry,

}
