//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/jffs2_fs_sb.h
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
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

pub const JFFS2_SB_FLAG_RO: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_mount_opts {
    pub override_compr: bool,
    pub compr: c_uint,
// The size of the reserved pool. The reserved pool is the JFFS2 flash
// space which may only be used by root cannot be used by the other
// users. This is implemented simply by means of not allowing the
// latter users to write to the file system if the amount if the
// available space is less then 'rp_size'.
    pub set_rp_size: bool,
    pub rp_size: c_uint,
}

// A struct for the overall file system control.  Pointers to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_sb_info {
    pub mtd: *mut mtd_info,
    pub highest_ino: u32,
    pub /: *mut *mut *mut *mut uint32_t check_ino; / NEXT inode to be checked,
    pub flags: c_uint,
    pub /: *mut *mut *mut task_gc_task; / GC task struct,
    pub /: *mut *mut completion gc_thread_start; / GC thread start completion,
    pub /: *mut *mut completion gc_thread_exit; / GC thread exit completion port,
    pub following: *mut *mut mutex alloc_sem; / Used to protect all the,
    pub CLEANMARKER: *mut *mut uint32_t cleanmarker_size; / Size of an _inline_,
    pub flash_size: u32,
    pub used_size: u32,
    pub dirty_size: u32,
    pub wasted_size: u32,
    pub free_size: u32,
    pub erasing_size: u32,
    pub bad_size: u32,
    pub sector_size: u32,
    pub unchecked_size: u32,
    pub nr_free_blocks: u32,
    pub nr_erasing_blocks: u32,
// Number of free blocks there must be before we...
    pub /: *mut *mut uint8_t resv_blocks_write; / ... allow a normal filesystem write,
    pub /: *mut *mut uint8_t resv_blocks_deletion; / ... allow a normal filesystem deletion,
    pub /: *mut *mut uint8_t resv_blocks_gctrigger; / ... wake up the GC thread,
    pub /: *mut *mut uint8_t resv_blocks_gcbad; / ... pick a block from the bad_list to GC,
    pub /: *mut *mut uint8_t resv_blocks_gcmerge; / ... merge pages when garbage collecting,
// Number of 'very dirty' blocks before we trigger immediate GC
    pub vdirty_blocks_gctrigger: u8,
    pub nospc_dirty_size: u32,
    pub nr_blocks: u32,
    pub blocks: *mut *mut *mut jffs2_eraseblock blocks; / The whole array of blocks. Used for getting,
// from the offset (blocks[ofs / sector_size])
    pub /: *mut *mut *mut jffs2_eraseblock nextblock; / The block we're currently filling,
    pub /: *mut *mut *mut jffs2_eraseblock gcblock; / The block we're currently garbage-collecting,
    pub /: *mut *mut list_head clean_list; / Blocks 100% full of clean data,
    pub /: *mut *mut list_head very_dirty_list; / Blocks with lots of dirty space,
    pub /: *mut *mut list_head dirty_list; / Blocks with some dirty space,
    pub /: *mut *mut list_head erasable_list; / Blocks which are completely dirty, and need erasing,
    pub /: *mut *mut list_head erasable_pending_wbuf_list; / Blocks which need erasing but only after the current wbuf is flushed,
    pub /: *mut *mut list_head erasing_list; / Blocks which are currently erasing,
    pub /: *mut *mut list_head erase_checking_list; / Blocks which are being checked and marked,
    pub /: *mut *mut list_head erase_pending_list; / Blocks which need erasing now,
    pub /: *mut *mut list_head erase_complete_list; / Blocks which are erased and need the clean marker written to them,
    pub /: *mut *mut list_head free_list; / Blocks which are free and ready to be used,
    pub /: *mut *mut list_head bad_list; / Bad blocks.,
    pub /: *mut *mut list_head bad_used_list; / Bad blocks with valid data in.,
    pub erasing_list: *mut *mut spinlock_t erase_completion_lock; / Protect free_list and,
    pub /: *mut *mut wait_queue_head_t erase_wait; / For waiting for erases to complete,
    pub inocache_wq: wait_queue_head_t,
    pub inocache_hashsize: c_int,
    pub inocache_list: *mut jffs2_inode_cache,
    pub inocache_lock: spinlock_t,
// Sem to allow jffs2_garbage_collect_deletion_dirent to
    pub erase_free_sem: mutex,
    pub /: *mut *mut uint32_t wbuf_pagesize; / 0 for NOR and other flashes with no wbuf,

    pub /: *mut *mut *mut unsigned char wbuf_verify; / read-back buffer for verification,

    pub /: *mut *mut *mut unsigned char wbuf; / Write-behind buffer for NAND flash,
    pub wbuf_ofs: u32,
    pub wbuf_len: u32,
    pub wbuf_inodes: *mut jffs2_inodirty,
    pub /: *mut *mut rw_semaphore wbuf_sem; / Protects the write buffer,
    pub /: *mut *mut delayed_work wbuf_dwork; / write-buffer write-out work,
    pub oobbuf: *mut c_uchar,
    pub /: *mut *mut int oobavail; / How many bytes are available for JFFS2 in OOB,

    pub /: *mut *mut *mut jffs2_summary summary; / Summary information,
    pub mount_opts: jffs2_mount_opts,

    pub highest_xid: u32,
    pub highest_xseqno: u32,
    pub xattrindex: [list_head; XATTRINDEX_HASHSIZE],
    pub xattr_unchecked: list_head,
    pub xattr_dead_list: list_head,
    pub xref_dead_list: *mut jffs2_xattr_ref,
    pub xref_temp: *mut jffs2_xattr_ref,
    pub xattr_sem: rw_semaphore,
    pub xdatum_mem_usage: u32,
    pub xdatum_mem_threshold: u32,

// OS-private pointer for getting back to master superblock info
    pub os_priv: *mut c_void,
}
