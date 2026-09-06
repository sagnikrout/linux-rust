//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vmstat.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reclaim_stat {
    pub nr_dirty: unsigned,
    pub nr_unqueued_dirty: unsigned,
    pub nr_congested: unsigned,
    pub nr_writeback: unsigned,
    pub nr_immediate: unsigned,
    pub nr_activate: [unsigned; ANON_AND_FILE],
    pub nr_ref_keep: unsigned,
    pub nr_unmap_fail: unsigned,
    pub nr_lazyfree_fail: unsigned,
    pub nr_demoted: unsigned,
}

// Stat data for system wide items
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vm_stat_item {
    NR_DIRTY_THRESHOLD,
    NR_DIRTY_BG_THRESHOLD,
    NR_MEMMAP_PAGES,	/* page metadata allocated through buddy allocator */
    NR_MEMMAP_BOOT_PAGES,	/* page metadata allocated through boot allocator */
    NR_VM_STAT_ITEMS,
}

//
// Light weight per cpu counter implementation.
//
// Counters should only be incremented and no critical kernel component
// should rely on the counter values.
//
// Counters are handled completely inline. On many platforms the code
// generated will simply be the increment of a global address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_event_state {
    pub event: [c_ulong; NR_VM_EVENT_ITEMS],
}

//
// vm counters are allowed to be racy. Use raw_cpu_ops to avoid the
// local_irq_disable overhead.
//
extern "C" {
    pub fn all_vm_events(: *mut c_ulong);
}
extern "C" {
    pub fn vm_events_fold_cpu(cpu: c_int);
}

// Disable counters

//
// Zone and node-based page accounting with per cpu differentials.
//

extern "C" {
    pub fn atomic_long_read(_arg: &zone->vm_numa_event[item]) -> return;
}
extern "C" {
    pub fn atomic_long_read(_arg: &vm_numa_event[item]) -> return;
}

//
// Non-clamping variant of global_node_page_state() intended for callers that
// snapshot a monotonically-incremented counter and subtract two samples.
// Returns the raw wrapping value so that unsigned modular subtraction stays
// correct across a signed-long overflow (a real hazard on 32-bit) that the
// clamp in global_node_page_state() would otherwise turn into a huge spurious
// delta. Do NOT use for non-monotonic page-count reads.
//
extern "C" {
    pub fn global_node_page_state_pages(_arg: item) -> return;
}

//
// More accurate version that also considers the currently pending
// deltas. For that we need to loop over all cpus to find the current
// deltas. There is no synchronization so the result cannot be
// exactly accurate either.
//

// See __count_vm_event comment on why raw_cpu_inc is used.
extern "C" {
    pub fn sum_zone_numa_event_state(node: c_int, item: numa_stat_item) -> c_ulong;
}
extern "C" {
    pub fn fold_vm_numa_events();
}

extern "C" {
    pub fn __mod_zone_page_state(: *mut zone, item: zone_stat_item, _arg: c_long);
}
extern "C" {
    pub fn __inc_zone_page_state(: *mut page, zone_stat_item: enum);
}
extern "C" {
    pub fn __dec_zone_page_state(: *mut page, zone_stat_item: enum);
}
extern "C" {
    pub fn __mod_node_page_state(: *mut pglist_data, item: node_stat_item, _arg: c_long);
}
extern "C" {
    pub fn __inc_node_page_state(: *mut page, node_stat_item: enum);
}
extern "C" {
    pub fn __dec_node_page_state(: *mut page, node_stat_item: enum);
}
extern "C" {
    pub fn mod_zone_page_state(: *mut zone, zone_stat_item: enum, _arg: c_long);
}
extern "C" {
    pub fn inc_zone_page_state(: *mut page, zone_stat_item: enum);
}
extern "C" {
    pub fn dec_zone_page_state(: *mut page, zone_stat_item: enum);
}
extern "C" {
    pub fn mod_node_page_state(: *mut pglist_data, node_stat_item: enum, _arg: c_long);
}
extern "C" {
    pub fn inc_node_page_state(: *mut page, node_stat_item: enum);
}
extern "C" {
    pub fn dec_node_page_state(: *mut page, node_stat_item: enum);
}
extern "C" {
    pub fn __inc_zone_state(: *mut zone, zone_stat_item: enum);
}
extern "C" {
    pub fn __inc_node_state(: *mut pglist_data, node_stat_item: enum);
}
extern "C" {
    pub fn __dec_zone_state(: *mut zone, zone_stat_item: enum);
}
extern "C" {
    pub fn __dec_node_state(: *mut pglist_data, node_stat_item: enum);
}
extern "C" {
    pub fn quiet_vmstat();
}
extern "C" {
    pub fn cpu_vm_stats_fold(cpu: c_int);
}
extern "C" {
    pub fn refresh_zone_stat_thresholds();
}
extern "C" {
    pub fn drain_zonestat(zone: *mut zone, : *mut per_cpu_zonestat);
}
extern "C" {
    pub fn calculate_pressure_threshold(zone: *mut zone) -> c_int;
}
extern "C" {
    pub fn calculate_normal_threshold(zone: *mut zone) -> c_int;
}
extern "C" {
    pub fn vmstat_flush_workqueue();
}

//
// We do not maintain differentials in a single processor configuration.
// The functions directly modify the zone and global counters.
//
// Only cgroups use subpage accounting right now; at
// the global level, these items still change in
// multiples of whole pages. Store them as pages
// internally to keep the per-cpu counters compact.
//
// We only use atomic operations to update counters. So there is no need to
// disable interrupts.
//

extern "C" {
    pub fn memmap_boot_pages_add(delta: c_long);
}
extern "C" {
    pub fn memmap_pages_add(delta: c_long);
}
