//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/writeback.h
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
// include/linux/writeback.h
//

//
// The global dirty threshold is normally equal to the global dirty limit,
// except when the system suddenly allocates a lot of anonymous memory and
// knocks down the global dirty threshold quickly, in which case the global
// dirty limit will follow down slowly to prevent livelocking all dirtier tasks.
//
pub const DIRTY_SCOPE: c_int = 8;
//
// fs/fs-writeback.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum writeback_sync_modes {
    WB_SYNC_NONE,	/* Don't wait on anything */
    WB_SYNC_ALL,	/* Wait on every mapping */
}

//
// A control structure which tells the writeback code what to do.  These are
// always on the stack, and hence need no locking.  They are always initialised
// in a manner such that unspecified fields are set to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct writeback_control {
// public fields that can be set and/or consumed by the caller:
    pub decrement: *mut *mut long nr_to_write; / Write this many pages, and,
    pub /: *mut *mut long pages_skipped; / Pages which were not written,
//
// For a_ops->writepages(): if start or end are non-zero then this is
// a hint that the filesystem need only write out the pages inside that
// byterange.  The byte at `end' is included in the writeout request.
//
    pub range_start: loff_t,
    pub range_end: loff_t,
    pub sync_mode: writeback_sync_modes,
    pub /: *mut *mut unsigned for_kupdate:1; / A kupdate writeback,
    pub /: *mut *mut unsigned for_background:1; / A background writeback,
    pub /: *mut *mut unsigned tagged_writepages:1; / tag-and-write to avoid livelock,
    pub /: *mut *mut unsigned range_cyclic:1; / range_start is cyclic,
    pub /: *mut *mut unsigned for_sync:1; / sync(2) WB_SYNC_ALL writeback,
    pub /: *mut *mut unsigned unpinned_netfs_wb:1; / Cleared I_PINNING_NETFS_WB,
//
// When writeback IOs are bounced through async layers, only the
// initial synchronous phase should be accounted towards inode
// cgroup ownership arbitration to avoid confusion.  Later stages
// can set the following flag to disable the accounting.
//
    pub no_cgroup_owner:1: unsigned,
// internal fields used by the ->writepages implementation:
    pub fbatch: folio_batch,
    pub index: pgoff_t,
    pub saved_err: c_int,

    pub /: *mut *mut *mut bdi_writeback wb; / wb this writeback is issued under,
    pub /: *mut *mut *mut inode inode; / inode being written out,
// foreign inode detection, see wbc_detach_inode()
    pub /: *mut *mut int wb_id; / current wb id,
    pub /: *mut *mut int wb_lcand_id; / last foreign candidate wb id,
    pub /: *mut *mut int wb_tcand_id; / this foreign candidate wb id,
    pub /: *mut *mut size_t wb_bytes; / bytes written by current wb,
    pub /: *mut *mut size_t wb_lcand_bytes; / bytes written by last candidate,
    pub /: *mut *mut size_t wb_tcand_bytes; / bytes written by this candidate,

}

//
// A wb_domain represents a domain that wb's (bdi_writeback's) belong to
// and are measured against each other in.  There always is one global
// domain, global_wb_domain, that every wb in the system is a member of.
// This allows measuring the relative bandwidth of each wb to distribute
// dirtyable memory accordingly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wb_domain {
    pub lock: spinlock_t,
//
// Scale the writeback cache size proportional to the relative
// writeout speed.
//
// We do this by keeping a floating proportion between BDIs, based
// on page writeback completions [end_page_writeback()]. Those
// devices that write out pages fastest will get the larger share,
// while the slower will get a smaller share.
//
// We use page writeout completions because we are interested in
// getting rid of dirty pages. Having them written out is the
// primary goal.
//
// We introduce a concept of time, a period over which we measure
// these events, because demand can/will vary over time. The length
// of this period itself is measured in page writeback completions.
//
    pub completions: fprop_global,
    pub /: *mut *mut timer_list period_timer; / timer for aging of completions,
    pub period_time: c_ulong,
//
// The dirtyable memory and dirty threshold could be suddenly
// knocked down by a large amount (eg. on the startup of KVM in a
// swapless system). This may throw the system into deep dirty
// exceeded state and throttle heavy/light dirtiers alike. To
// retain good responsiveness, maintain global_dirty_limit for
// tracking slowly down to the knocked down dirty threshold.
//
// Both fields are protected by ->lock.
//
    pub dirty_limit_tstamp: c_ulong,
    pub dirty_limit: c_ulong,
}

//
// wb_domain_size_changed - memory available to a wb_domain has changed
// @dom: wb_domain of interest
//
// This function should be called when the amount of memory available to
// @dom has changed.  It resets @dom's dirty limit parameters to prevent
// the past values which don't match the current configuration from skewing
// dirty throttling.  Without this, when memory size of a wb_domain is
// greatly reduced, the dirty throttling logic may allow too many pages to
// be dirtied leading to consecutive unnecessary OOMs and may get stuck in
// that situation.
//
// fs/fs-writeback.c
//
extern "C" {
    pub fn writeback_inodes_sb(: *mut super_block, reason: wb_reason);
}
extern "C" {
    pub fn try_to_writeback_inodes_sb(sb: *mut super_block, reason: wb_reason);
}
extern "C" {
    pub fn sync_inodes_sb(: *mut super_block);
}
extern "C" {
    pub fn wakeup_flusher_threads(reason: wb_reason);
}
extern "C" {
    pub fn inode_wait_for_writeback(inode: *mut inode);
}
extern "C" {
    pub fn inode_io_list_del(inode: *mut inode);
}

extern "C" {
    pub fn __inode_attach_wb(inode: *mut inode, folio: *mut folio);
}
extern "C" {
    pub fn wbc_detach_inode(wbc: *mut writeback_control);
}
extern "C" {
    pub fn cgroup_writeback_umount(sb: *mut super_block);
}
extern "C" {
    pub fn cleanup_offline_cgwb(wb: *mut bdi_writeback) -> bool;
}
//
// inode_attach_wb - associate an inode with its wb
// @inode: inode of interest
// @folio: folio being dirtied (may be NULL)
//
// If @inode doesn't have its wb, associate it with the wb matching the
// memcg of @folio or, if @folio is NULL, %current.  May be called w/ or w/o
// @inode->i_lock.
//
// inode_detach_wb - disassociate an inode from its wb
// @inode: inode of interest
//
// @inode is being freed.  Detach from its wb.
//
// wbc_init_bio - writeback specific initialization of bio
// @wbc: writeback_control for the writeback in progress
// @bio: bio to be initialized
//
// @bio is a part of the writeback in progress controlled by @wbc.  Perform
// writeback specific initialization.  This is used to apply the cgroup
// writeback context.  Must be called after the bio has been associated with
// a device.
//
// pageout() path doesn't attach @wbc to the inode being written
// out.  This is intentional as we don't want the function to block
// behind a slow cgroup.  Ultimately, we want pageout() to kick off
// regular writeback instead of writing things out itself.
//
extern "C" {
    pub fn inode_switch_wbs_work_fn(work: *mut work_struct);
}

//
// mm/page-writeback.c
//
// consolidated parameters for balance_dirty_pages() and its subroutines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirty_throttle_control {

    pub dom: *mut wb_domain,
    pub /: *mut *mut *mut dirty_throttle_control gdtc; / only set in memcg dtc's,

    pub wb: *mut bdi_writeback,
    pub wb_completions: *mut fprop_local_percpu,
    pub /: *mut *mut unsigned long avail; / dirtyable,
    pub /: *mut *mut unsigned long dirty; / file_dirty + write + nfs,
    pub /: *mut *mut unsigned long thresh; / dirty threshold,
    pub /: *mut *mut unsigned long bg_thresh; / dirty background threshold,
    pub /: *mut *mut unsigned long limit; / hard dirty limit,
    pub /: *mut *mut unsigned long wb_dirty; / per-wb counterparts,
    pub wb_thresh: c_ulong,
    pub wb_bg_thresh: c_ulong,
    pub pos_ratio: c_ulong,
    pub freerun: bool,
    pub dirty_exceeded: bool,
}

extern "C" {
    pub fn node_dirty_ok(pgdat: *mut pglist_data) -> bool;
}
extern "C" {
    pub fn wb_domain_init(dom: *mut wb_domain, gfp: gfp_t) -> c_int;
}

extern "C" {
    pub fn wb_domain_exit(dom: *mut wb_domain);
}

// These are exported to sysctl.
extern "C" {
    pub fn global_dirty_limits(pbackground: *mut c_ulong, pdirty: *mut c_ulong);
}
extern "C" {
    pub fn wb_calc_thresh(wb: *mut bdi_writeback, thresh: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn cgwb_calc_thresh(wb: *mut bdi_writeback) -> c_ulong;
}
extern "C" {
    pub fn wb_update_bandwidth(wb: *mut bdi_writeback);
}
// Invoke balance dirty pages in async mode.
pub const BDP_ASYNC: c_uint = 0x0001;
extern "C" {
    pub fn balance_dirty_pages_ratelimited(mapping: *mut address_space);
}
extern "C" {
    pub fn wb_over_bg_thresh(wb: *mut bdi_writeback) -> bool;
}
extern "C" {
    pub fn do_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn writeback_set_ratelimit();
}
extern "C" {
    pub fn filemap_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
}
extern "C" {
    pub fn folio_redirty_for_writepage(: *mut writeback_control, : *mut folio) -> bool;
}
extern "C" {
    pub fn redirty_page_for_writepage(: *mut writeback_control, : *mut page) -> bool;
}
extern "C" {
    pub fn sb_mark_inode_writeback(inode: *mut inode);
}
extern "C" {
    pub fn sb_clear_inode_writeback(inode: *mut inode);
}
//
// 4MB minimal write chunk size
//

