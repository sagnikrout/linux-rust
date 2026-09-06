//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/backing-dev.h
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
// include/linux/backing-dev.h
//
// low-level device information and state which is propagated up through
// to high-level code.
//

extern "C" {
    pub fn bdi_put(bdi: *mut backing_dev_info);
}
extern "C" {
    pub fn bdi_register(bdi: *mut backing_dev_info, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn bdi_set_owner(bdi: *mut backing_dev_info, owner: *mut device);
}
extern "C" {
    pub fn bdi_unregister(bdi: *mut backing_dev_info);
}
extern "C" {
    pub fn wb_start_background_writeback(wb: *mut bdi_writeback);
}
extern "C" {
    pub fn wb_workfn(work: *mut work_struct);
}
extern "C" {
    pub fn wb_wait_for_completion(done: *mut wb_completion);
}
extern "C" {
    pub fn test_bit(_arg: WB_has_dirty_io, _arg: &wb->state) -> return;
}
//
// @bdi->tot_write_bandwidth is guaranteed to be > 0 if there are
// any dirty wbs.  See wb_update_write_bandwidth().
//
extern "C" {
    pub fn atomic_long_read(_arg: &bdi->tot_write_bandwidth) -> return;
}
extern "C" {
    pub fn percpu_counter_read_positive(_arg: &wb->stat[item]) -> return;
}
extern "C" {
    pub fn percpu_counter_sum_positive(_arg: &wb->stat[item]) -> return;
}
//
// maximal error of a stat counter.
//

// BDI ratio is expressed as part per 1000000 for finer granularity.
pub const BDI_RATIO_SCALE: c_int = 10000;
extern "C" {
    pub fn bdi_get_min_bytes(bdi: *mut backing_dev_info) -> u64;
}
extern "C" {
    pub fn bdi_get_max_bytes(bdi: *mut backing_dev_info) -> u64;
}
extern "C" {
    pub fn bdi_set_min_ratio(bdi: *mut backing_dev_info, min_ratio: c_uint) -> c_int;
}
extern "C" {
    pub fn bdi_set_max_ratio(bdi: *mut backing_dev_info, max_ratio: c_uint) -> c_int;
}
extern "C" {
    pub fn bdi_set_min_ratio_no_scale(bdi: *mut backing_dev_info, min_ratio: c_uint) -> c_int;
}
extern "C" {
    pub fn bdi_set_max_ratio_no_scale(bdi: *mut backing_dev_info, max_ratio: c_uint) -> c_int;
}
extern "C" {
    pub fn bdi_set_min_bytes(bdi: *mut backing_dev_info, min_bytes: u64) -> c_int;
}
extern "C" {
    pub fn bdi_set_max_bytes(bdi: *mut backing_dev_info, max_bytes: u64) -> c_int;
}
extern "C" {
    pub fn bdi_set_strict_limit(bdi: *mut backing_dev_info, strict_limit: c_uint) -> c_int;
}
//
// Flags in backing_dev_info::capability
//
// BDI_CAP_WRITEBACK:		Supports dirty page writeback, and dirty pages
// should contribute to accounting
// BDI_CAP_STRICTLIMIT:		Keep number of dirty pages below bdi threshold
//

extern "C" {
    pub fn bdi_init(bdi: *mut backing_dev_info) -> c_int;
}
//
// writeback_in_progress - determine whether there is writeback in progress
// @wb: bdi_writeback of interest
//
// Determine whether there is writeback waiting to be handled against a
// bdi_writeback.
//
extern "C" {
    pub fn test_bit(_arg: WB_writeback_running, _arg: &wb->state) -> return;
}
// Must not be used by file systems that support cgroup writeback

extern "C" {
    pub fn wb_memcg_offline(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn wb_blkcg_offline(css: *mut cgroup_subsys_state);
}
//
// inode_cgwb_enabled - test whether cgroup writeback is enabled on an inode
// @inode: inode of interest
//
// Cgroup writeback requires support from the filesystem.  Also, both memcg and
// iocg have to be on the default hierarchy.  Test whether all conditions are
// met.
//
// Note that the test result may change dynamically on the same inode
// depending on how memcg and iocg are configured.
//
// wb_find_current - find wb for %current on a bdi
// @bdi: bdi of interest
//
// Find the wb of @bdi which matches both the memcg and blkcg of %current.
// Must be called under rcu_read_lock() which protects the returend wb.
// NULL if not found.
//
// %current's blkcg equals the effective blkcg of its memcg.  No
// need to use the relatively expensive cgroup_get_e_css().
//
// wb_get_create_current - get or create wb for %current on a bdi
// @bdi: bdi of interest
// @gfp: allocation mask
//
// Equivalent to wb_get_create() on %current's memcg.  This function is
// called from a relatively hot path and optimizes the common cases using
// wb_find_current().
//
// inode_to_wb - determine the wb of an inode
// @inode: inode of interest
//
// Returns the wb @inode is currently associated with.  The caller must be
// holding either @inode->i_lock, the i_pages lock, or the
// associated wb's list_lock.
//

//
// If wbc does not have inode attached, it means cgroup writeback was
// disabled when wbc started. Just use the default wb in that case.
//
// unlocked_inode_to_wb_begin - begin unlocked inode wb access transaction
// @inode: target inode
// @cookie: output param, to be passed to the end function
//
// The caller wants to access the wb associated with @inode but isn't
// holding inode->i_lock, the i_pages lock or wb->list_lock.  This
// function determines the wb associated with @inode and ensures that the
// association doesn't change until the transaction is finished with
// unlocked_inode_to_wb_end().
//
// The caller must call unlocked_inode_to_wb_end() with *@cookie afterwards and
// can't sleep during the transaction.  IRQs may or may not be disabled on
// return.
//
// Paired with a release fence in inode_do_switch_wbs() and
// ensures that we see the new wb if we see cleared I_WB_SWITCH.
//
// Protected by either !I_WB_SWITCH + rcu_read_lock() or the i_pages
// lock.  inode_to_wb() will bark.  Deref directly.
//
// unlocked_inode_to_wb_end - end inode wb access transaction
// @inode: target inode
// @cookie: @cookie from unlocked_inode_to_wb_begin()
//

extern "C" {
    pub fn inode_to_wb(_arg: inode) -> return;
}
extern "C" {
    pub fn inode_to_wb(_arg: inode) -> return;
}

