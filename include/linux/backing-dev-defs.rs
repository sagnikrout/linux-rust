//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/backing-dev-defs.h
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
// Bits in bdi_writeback.state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wb_state {
    WB_registered,		/* bdi_register() was done */
    WB_writeback_running,	/* Writeback is in progress */
    WB_has_dirty_io,	/* Dirty inodes on ->b_{dirty|io|more_io} */
    WB_start_all,		/* nr_pages == 0 (all) work pending */
    WB_start_dontcache,	/* dontcache writeback pending */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wb_stat_item {
    WB_RECLAIMABLE,
    WB_WRITEBACK,
    WB_DIRTIED,
    WB_WRITTEN,
    WB_DONTCACHE_DIRTY,
    NR_WB_STAT_ITEMS
}

//
// why some writeback work was initiated
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wb_reason {
    WB_REASON_BACKGROUND,
    WB_REASON_VMSCAN,
    WB_REASON_SYNC,
    WB_REASON_PERIODIC,
    WB_REASON_FS_FREE_SPACE,
//
// There is no bdi forker thread any more and works are done
// by emergency worker, however, this is TPs userland visible
// and we'll be exposing exactly the same information,
// so it has a mismatch name.
//
    WB_REASON_FORKER_THREAD,
    WB_REASON_FOREIGN_FLUSH,
    WB_REASON_DONTCACHE,

    WB_REASON_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wb_completion {
    pub cnt: core::sync::atomic::AtomicI32,
    pub waitq: *mut wait_queue_head_t,
    pub /: *mut *mut unsigned long progress_stamp; / The jiffies when slow progress is detected,
    pub /: *mut *mut unsigned long wait_start; / The jiffies when waiting for the writeback work to finish,
}

//
// If one wants to wait for one or more wb_writeback_works, each work's
// ->done should be set to a wb_completion defined using the following
// macro.  Once all work items are issued with wb_queue_work(), the caller
// can wait for the completion of all using wb_wait_for_completion().  Work
// items which are waited upon aren't freed automatically on completion.
//

//
// Each wb (bdi_writeback) can perform writeback operations, is measured
// and throttled, independently.  Without cgroup writeback, each bdi
// (bdi_writeback) is served by its embedded bdi->wb.
//
// On the default hierarchy, blkcg implicitly enables memcg.  This allows
// using memcg's page ownership for attributing writeback IOs, and every
// memcg - blkcg combination can be served by its own wb by assigning a
// dedicated wb to each memcg, which enables isolation across different
// cgroups and propagation of IO back pressure down from the IO layer upto
// the tasks which are generating the dirty pages to be written back.
//
// A cgroup wb is indexed on its bdi by the ID of the associated memcg,
// refcounted with the number of inodes attached to it, and pins the memcg
// and the corresponding blkcg.  As the corresponding blkcg for a memcg may
// change as blkcg is disabled and enabled higher up in the hierarchy, a wb
// is tested for blkcg after lookup and removed from index on mismatch so
// that a new wb for the combination can be created.
//
// Each bdi_writeback that is not embedded into the backing_dev_info must hold
// a reference to the parent backing_dev_info.  See cgwb_create() for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdi_writeback {
    pub /: *mut *mut *mut backing_dev_info bdi; / our parent bdi,
    pub /: *mut *mut unsigned long state; / Always use atomic bitops on this,
    pub /: *mut *mut unsigned long last_old_flush; / last old data flush,
    pub /: *mut *mut list_head b_dirty; / dirty inodes,
    pub /: *mut *mut list_head b_io; / parked for writeback,
    pub /: *mut *mut list_head b_more_io; / parked for more writeback,
    pub /: *mut *mut list_head b_dirty_time; / time stamps are dirty,
    pub /: *mut *mut *mut spinlock_t list_lock; / protects the b_ lists,
    pub /: *mut *mut atomic_t writeback_inodes; / number of inodes under writeback,
    pub stat: [percpu_counter; NR_WB_STAT_ITEMS],
    pub /: *mut *mut unsigned long bw_time_stamp; / last time write bw is updated,
    pub dirtied_stamp: c_ulong,
    pub /: *mut *mut unsigned long written_stamp; / pages written at bw_time_stamp,
    pub /: *mut *mut unsigned long write_bandwidth; / the estimated write bandwidth,
    pub /: *mut *mut unsigned long avg_write_bandwidth; / further smoothed write bw, > 0,
//
// The base dirty throttle rate, re-calculated on every 200ms.
// All the bdi tasks' dirty rate will be curbed under it.
// @dirty_ratelimit tracks the estimated @balanced_dirty_ratelimit
// in small steps and is much more smooth/stable than the latter.
//
    pub dirty_ratelimit: c_ulong,
    pub balanced_dirty_ratelimit: c_ulong,
    pub completions: fprop_local_percpu,
    pub dirty_exceeded: c_int,
    pub start_all_reason: wb_reason,
    pub /: *mut *mut spinlock_t work_lock; / protects work_list & dwork scheduling,
    pub work_list: list_head,
    pub /: *mut *mut delayed_work dwork; / work item used for writeback,
    pub /: *mut *mut delayed_work bw_dwork; / work item used for bandwidth estimate,
    pub /: *mut *mut list_head bdi_node; / anchored at bdi->wb_list,

    pub /: *mut *mut percpu_ref refcnt; / used only for !root wb's,
    pub memcg_completions: fprop_local_percpu,
    pub /: *mut *mut *mut cgroup_subsys_state memcg_css; / the associated memcg,
    pub /: *mut *mut *mut cgroup_subsys_state blkcg_css; / and blkcg,
    pub /: *mut *mut list_head memcg_node; / anchored at memcg->cgwb_list,
    pub /: *mut *mut list_head blkcg_node; / anchored at blkcg->cgwb_list,
    pub /: *mut *mut list_head b_attached; / attached inodes, protected by list_lock,
    pub /: *mut *mut list_head offline_node; / anchored at offline_cgwbs,
    pub switching: *mut *mut work_switch_work; / work used to perform inode,
// to this wb
    pub for: *mut *mut llist_head switch_wbs_ctxs; / queued contexts,
// writeback switching
    pub release_work: work_struct,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_dev_info {
    pub id: u64,
    pub /: *mut *mut rb_node rb_node; / keyed by ->id,
    pub bdi_list: list_head,
// max readahead in PAGE_SIZE units
    pub ra_pages: unsigned long __data_racy,
    pub /: *mut *mut unsigned long io_pages; / max allowed IO size,
    pub /: *mut *mut kref refcnt; / Reference counter for the structure,
    pub /: *mut *mut unsigned int capabilities; / Device capabilities,
    pub min_ratio: c_uint,
    pub max_prop_frac: unsigned int max_ratio,,
//
// Sum of avg_write_bw of wbs with dirty inodes.  > 0 if there are
// any dirty wbs, which is depended upon by bdi_has_dirty().
//
    pub tot_write_bandwidth: atomic_long_t,
//
// Jiffies when last process was dirty throttled on this bdi. Used by
// blk-wbt.
//
    pub last_bdp_sleep: c_ulong,
    pub /: *mut *mut bdi_writeback wb; / the root writeback info for this bdi,
    pub /: *mut *mut list_head wb_list; / list of all wbs,

    pub /: *mut *mut radix_tree_root cgwb_tree; / radix tree of active cgroup wbs,
    pub /: *mut *mut mutex cgwb_release_mutex; / protect shutdown of wb structs,
    pub /: *mut *mut rw_semaphore wb_switch_rwsem; / no cgwb switch while syncing,

    pub wb_waitq: wait_queue_head_t,
    pub dev: *mut device,
    pub dev_name: [c_char; 64],
    pub owner: *mut device,

    pub debug_dir: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wb_lock_cookie {
    pub locked: bool,
    pub flags: c_ulong,
}

//
// wb_tryget - try to increment a wb's refcount
// @wb: bdi_writeback to get
//
extern "C" {
    pub fn percpu_ref_tryget(_arg: &wb->refcnt) -> return;
}
//
// wb_get - increment a wb's refcount
// @wb: bdi_writeback to get
//
// wb_put_many - decrement a wb's refcount
// @wb: bdi_writeback to put
// @nr: number of references to put
//
// A driver bug might cause a file to be removed before bdi was
// initialized.
//
// wb_put - decrement a wb's refcount
// @wb: bdi_writeback to put
//
// wb_dying - is a wb dying?
// @wb: bdi_writeback of interest
//
// Returns whether @wb is unlinked and being drained.
//
extern "C" {
    pub fn percpu_ref_is_dying(_arg: &wb->refcnt) -> return;
}

