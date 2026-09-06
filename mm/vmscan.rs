//! Automatically rewritten from C to Rust
//! Source: mm/vmscan.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// Swap reorganised 29.12.95, Stephen Tweedie.
// kswapd added: 7.1.96  sct
// Removed kswapd_ctl limits, and swap out as many pages as needed
// to bring the system back to freepages.high: 2.4.97, Rik van Riel.
// Zone aware kswapd started 02/00, Kanoj Sarcar (kanoj@sgi.com).
// Multiqueue VM started 5.8.00, Rik van Riel.
//

// Macro flag: #define CREATE_TRACE_POINTS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_control {
// How many pages shrink_list() should reclaim
    pub nr_to_reclaim: c_ulong,
//
// Nodemask of nodes allowed by the caller. If NULL, all nodes
// are scanned.
//
    pub nodemask: *const nodemask_t,
//
// The memory cgroup that hit its limit and as a result is the
// primary target of this reclaim invocation.
//
    pub target_mem_cgroup: *mut mem_cgroup,
//
// Scan pressure balancing between anon and file LRUs
//
    pub anon_cost: c_ulong,
    pub file_cost: c_ulong,
// Swappiness value for proactive reclaim. Always use sc_swappiness()!
    pub proactive_swappiness: *mut c_int,
// Can active folios be deactivated as part of reclaim?
pub const DEACTIVATE_ANON: c_int = 1;
pub const DEACTIVATE_FILE: c_int = 2;
    pub may_deactivate:2: c_uint,
    pub force_deactivate:1: c_uint,
    pub skipped_deactivate:1: c_uint,
// zone_reclaim_mode, boost reclaim
    pub may_writepage:1: c_uint,
// zone_reclaim_mode
    pub may_unmap:1: c_uint,
// zone_reclaim_mode, boost reclaim, cgroup restrictions
    pub may_swap:1: c_uint,
// Not allow cache_trim_mode to be turned on as part of reclaim?
    pub no_cache_trim_mode:1: c_uint,
// Has cache_trim_mode failed at least once?
    pub cache_trim_mode_failed:1: c_uint,
// Proactive reclaim invoked by userspace
    pub proactive:1: c_uint,
//
// Cgroup memory below memory.low is protected as long as we
// don't threaten to OOM. If any cgroup is reclaimed at
// reduced force or passed over entirely due to its memory.low
// setting (memcg_low_skipped), and nothing is reclaimed as a
// result, then go back for one more cycle that reclaims the protected
// memory (memcg_low_reclaim) to avert OOM.
//
    pub memcg_low_reclaim:1: c_uint,
    pub memcg_low_skipped:1: c_uint,
// Shared cgroup tree walk failed, rescan the whole tree
    pub memcg_full_walk:1: c_uint,
    pub hibernation_mode:1: c_uint,
// One of the zones is ready for compaction
    pub compaction_ready:1: c_uint,
// There is easily reclaimable cold cache in the current node
    pub cache_trim_mode:1: c_uint,
// The file folios on the current node are dangerously low
    pub file_is_tiny:1: c_uint,
// Always discard instead of demoting to lower tier memory
    pub no_demotion:1: c_uint,
// Allocation order
    pub order: i8,
// Scan (total_size >> priority) pages at once
    pub priority: i8,
// The highest zone to isolate folios for reclaim from
    pub reclaim_idx: i8,
// This context's GFP mask
    pub gfp_mask: gfp_t,
// Incremented by the number of inactive pages that were scanned
    pub nr_scanned: c_ulong,
Number of pages freed so far during a call to shrink_zones()
    pub nr_reclaimed: c_ulong,
    struct {
    pub dirty: c_uint,
    pub congested: c_uint,
    pub writeback: c_uint,
    pub immediate: c_uint,
    pub taken: c_uint,
    pub nr: },
// for recording the reclaimed slab by now
    pub reclaim_state: reclaim_state,
}

    do {								
    if ((_folio).lru.prev != _base) {			
pub static mut prev: *mut c_void = core::ptr::null_mut();				
    
    prev = lru_to_folio(&(_folio.lru));		
    prefetchw(&prev._field);			
    }							
    } while (0)

//
// From 0 .. MAX_SWAPPINESS.  Higher means more swappy.
//
pub static mut vm_swappiness: c_int = 60;
#[no_mangle]
unsafe extern "C" fn sc_swappiness(sc: *mut scan_control, memcg: *mut mem_cgroup) -> c_int {
    if (sc.proactive && sc.proactive_swappiness) {
    return *sc.proactive_swappiness;
    }
    return mem_cgroup_swappiness(memcg);
    }

// Returns true for reclaim through cgroup limits or cgroup interfaces.
#[no_mangle]
unsafe extern "C" fn cgroup_reclaim(sc: *mut scan_control) -> bool {
    return sc.target_mem_cgroup;
    }
//
// Returns true for reclaim on the root cgroup. This is true for direct
// allocator reclaim and reclaim through cgroup interfaces on the root cgroup.
//
#[no_mangle]
unsafe extern "C" fn root_reclaim(sc: *mut scan_control) -> bool {
    return !sc.target_mem_cgroup || mem_cgroup_is_root(sc.target_mem_cgroup);
    }
//
// writeback_throttling_sane - is the usual dirty throttling mechanism available?
// @sc: scan_control in question
//
// The normal page dirty throttling mechanism in balance_dirty_pages() is
// completely broken with the legacy memcg and direct stalling in
// shrink_folio_list() is used for throttling instead, which lacks all the
// niceties such as fairness, adaptive pausing, bandwidth proportional
// allocation and configurability.
//
// This function tests whether the vmscan currently in progress can assume
// that the normal dirty throttling mechanism is operational.
//
#[no_mangle]
unsafe extern "C" fn writeback_throttling_sane(sc: *mut scan_control) -> bool {
    if (!cgroup_reclaim(sc)) {
    return true;
    }

    if (cgroup_subsys_on_dfl(memory_cgrp_subsys)) {
    return true;
    }

    return false;
    }

#[no_mangle]
unsafe extern "C" fn cgroup_reclaim(sc: *mut scan_control) -> bool {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn root_reclaim(sc: *mut scan_control) -> bool {
    return true;
    }
#[no_mangle]
unsafe extern "C" fn writeback_throttling_sane(sc: *mut scan_control) -> bool {
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn is_exec_file_folio(folio: *mut folio, vma_flags: *mut vma_flags_t) -> bool {
    return vma_flags_test(vma_flags, VMA_EXEC_BIT) && folio_is_file_lru(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn set_task_reclaim_state(task: *mut task_struct, rs: *mut reclaim_state) {
// Check for an overwrite
    WARN_ON_ONCE!(rs && task.reclaim_state);
// Check for the nulling of an already-nulled member
    WARN_ON_ONCE!(!rs && !task.reclaim_state);
    task.reclaim_state = rs;
    }
//
// flush_reclaim_state(): add pages reclaimed outside of LRU-based reclaim to
// scan_control->nr_reclaimed.
//
#[no_mangle]
unsafe extern "C" fn flush_reclaim_state(sc: *mut scan_control) {
//
// Currently, reclaim_state->reclaimed includes three types of pages
// freed outside of vmscan:
// (1) Slab pages.
// (2) Clean file pages from pruned inodes (on highmem systems).
// (3) XFS freed buffer pages.
//
// For all of these cases, we cannot universally link the pages to a
// single memcg. For example, a memcg-aware shrinker can free one object
// charged to the target memcg, causing an entire page to be freed.
// If we count the entire page as reclaimed from the memcg, we end up
// overestimating the reclaimed amount (potentially under-reclaiming).
//
// Only count such pages for global reclaim to prevent under-reclaiming
// from the target memcg; preventing unnecessary retries during memcg
// charging and false positives from proactive reclaim.
//
// For uncommon cases where the freed pages were actually mostly
// charged to the target memcg, we end up underestimating the reclaimed
// amount. This should be fine. The freed pages will be uncharged
// anyway, even if they are not counted here properly, and we will be
// able to make forward progress in charging (which is usually in a
// retry loop).
//
// We can go one step further, and report the uncharged objcg pages in
// memcg reclaim, to make reporting more accurate and reduce
// underestimation, but it's probably not worth the complexity for now.
//
    if (current.reclaim_state && root_reclaim(sc)) {
    sc.nr_reclaimed += current.reclaim_state.reclaimed;
    current.reclaim_state.reclaimed = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn can_demote(nid: c_int, sc: *mut scan_control, memcg: *mut mem_cgroup) -> bool {
    let mut pgdat = NODE_DATA(nid);
    let mut allowed_mask;
    if (!pgdat || !numa_demotion_enabled) {
    return false;
    }
    if (sc && sc.no_demotion) {
    return false;
    }
    node_get_allowed_targets(pgdat, &allowed_mask);
    if (nodes_empty(allowed_mask)) {
    return false;
    }
// Filter out nodes that are not in cgroup's mems_allowed.
    mem_cgroup_node_filter_allowed(memcg, &allowed_mask);
    return !nodes_empty(allowed_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn can_reclaim_anon_pages(memcg: *mut mem_cgroup, nid: c_int, sc: *mut scan_control) -> bool {
    if (memcg == core::ptr::null_mut()) {
//
// For non-memcg reclaim, is there
// space in any swap device?
//
    if (get_nr_swap_pages() > 0) {
    return true;
    }
    } else {
// Is the memcg below its swap limit?
    if (mem_cgroup_get_nr_swap_pages(memcg) > 0) {
    return true;
    }
    }
//
// The page can not be swapped.
//
// Can it be reclaimed from this node via demotion?
//
    return can_demote(nid, sc, memcg);
    }
//
// This misses isolated folios which are not accounted for to save counters.
// As the data only determines if reclaim or compaction continues, it is
// not expected that isolated folios will be a dominating factor.
//
#[no_mangle]
pub unsafe extern "C" fn zone_reclaimable_pages(zone: *mut zone) -> c_ulong {
    let mut nr = 0;
    nr = zone_page_state_snapshot(zone, NR_ZONE_INACTIVE_FILE) +
    zone_page_state_snapshot(zone, NR_ZONE_ACTIVE_FILE);
    if (can_reclaim_anon_pages(core::ptr::null_mut(), zone_to_nid(zone), core::ptr::null_mut())) {
    nr += zone_page_state_snapshot(zone, NR_ZONE_INACTIVE_ANON) +
    zone_page_state_snapshot(zone, NR_ZONE_ACTIVE_ANON);
    }
    return nr;
    }
//
// lruvec_lru_size -  Returns the number of pages on the given LRU list.
// @lruvec: lru vector
// @lru: lru to use
// @zone_idx: zones to consider (use MAX_NR_ZONES - 1 for the whole LRU list)
//
#[no_mangle]
pub unsafe extern "C" fn lruvec_lru_size(lruvec: *mut lruvec, lru: lru_list, zone_idx: c_int) -> c_ulong {
pub static mut size: c_ulong = 0;
    let mut zid = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_managed_zone_pgdat(zone, lruvec_pgdat(lruvec), zid, zone_idx) {
    if (!mem_cgroup_disabled()) {
    size += mem_cgroup_get_zone_lru_size(lruvec, lru, zid);
    }
    else {
    size += zone_page_state(zone, NR_ZONE_LRU_BASE + lru);
    }
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn drop_slab_node(nid: c_int) -> c_ulong {
pub static mut freed: c_ulong = 0;
    let mut memcg = core::ptr::null_mut();
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    freed += shrink_slab(GFP_KERNEL, nid, memcg, 0);
    } while ((memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut())) != core::ptr::null_mut());
    return freed;
    }
#[no_mangle]
pub unsafe extern "C" fn drop_slab() {
    let mut nid = 0;
pub static mut shift: c_int = 0;
    let mut freed = 0;
    do {
    freed = 0;
    for_each_online_node(nid) {
    if (fatal_signal_pending(current)) {
    return;
    }
    freed += drop_slab_node(nid);
    }
    } while ((freed >> shift++) > 1);
    }

    do {								
    BUILD_BUG_ON!(PGSTEAL_##type - PGSTEAL_KSWAPD !=		
    PGDEMOTE_##type - PGDEMOTE_KSWAPD);	
    BUILD_BUG_ON!(PGSTEAL_##type - PGSTEAL_KSWAPD !=		
    PGSCAN_##type - PGSCAN_KSWAPD);		
    } while (0)
#[no_mangle]
unsafe extern "C" fn reclaimer_offset(sc: *mut scan_control) -> c_int {
    CHECK_RECLAIMER_OFFSET(DIRECT);
    CHECK_RECLAIMER_OFFSET(KHUGEPAGED);
    CHECK_RECLAIMER_OFFSET(PROACTIVE);
    if (current_is_kswapd()) {
    return 0;
    }
    if (current_is_khugepaged()) {
    return PGSTEAL_KHUGEPAGED - PGSTEAL_KSWAPD;
    }
    if (sc.proactive) {
    return PGSTEAL_PROACTIVE - PGSTEAL_KSWAPD;
    }
    return PGSTEAL_DIRECT - PGSTEAL_KSWAPD;
    }
//
// We detected a synchronous write error writing a folio out.  Probably
// -ENOSPC.  We need to propagate that into the address_space for a subsequent
// fsync(), msync() or close().
//
// The tricky part is that after writepage we cannot touch the mapping: nothing
// prevents it from being freed up.  But we have a ref on the folio and once
// that folio is locked, the mapping is pinned.
//
// We're allowed to run sleeping folio_lock() here because we know the caller has
// __GFP_FS.
//
#[no_mangle]
pub unsafe extern "C" fn handle_write_error(mapping: *mut address_space, folio: *mut folio, error: c_int) {
    folio_lock(folio);
    if (folio_mapping(folio) == mapping) {
    mapping_set_error(mapping, error);
    }
    folio_unlock(folio);
    }
#[no_mangle]
unsafe extern "C" fn skip_throttle_noprogress(pgdat: *mut pg_data_t) -> bool {
pub static mut reclaimable: c_int = 0;
    let mut i = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
//
// If kswapd is disabled, reschedule if necessary but do not
// throttle as the system is likely near OOM.
//
    if (kswapd_test_hopeless(pgdat)) {
    return true;
    }
//
// If there are a lot of dirty/writeback folios then do not
// throttle as throttling will occur when the folios cycle
// towards the end of the LRU if still under writeback.
//
    for_each_managed_zone_pgdat(zone, pgdat, i, MAX_NR_ZONES - 1) {
    reclaimable += zone_reclaimable_pages(zone);
    write_pending += zone_page_state_snapshot(zone,
    NR_ZONE_WRITE_PENDING);
    }
    if (2 * write_pending <= reclaimable) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn reclaim_throttle(pgdat: *mut pg_data_t, reason: vmscan_throttle_state) {
    let mut wqh = &pgdat.reclaim_wait[reason];
    let mut timeout = 0;
    let mut ret = 0;
pub static mut wait: usize = 0;
//
// Do not throttle user workers, kthreads other than kswapd or
// workqueues. They may be required for reclaim to make
// forward progress (e.g. journalling workqueues or kthreads).
//
    if (!current_is_kswapd() &&
    current.flags & (PF_USER_WORKER|PF_KTHREAD)) {
    cond_resched();
    return;
    }
//
// These figures are pulled out of thin air.
// VMSCAN_THROTTLE_ISOLATED is a transient condition based on too many
// parallel reclaimers which is a short-lived event so the timeout is
// short. Failing to make progress or waiting on writeback are
// potentially long-lived events so use a longer timeout. This is shaky
// logic as a failure to make progress could be due to anything from
// writeback to a slow device to excessive referenced folios at the tail
// of the inactive LRU.
//
    match (reason) {
    VMSCAN_THROTTLE_WRITEBACK => {
    timeout = HZ/10;
    if (atomic_inc_return(&pgdat.nr_writeback_throttled) == 1) {
    WRITE_ONCE(pgdat.nr_reclaim_start,
    node_page_state(pgdat, NR_THROTTLED_WRITTEN));
    }
    // break;
    }
    VMSCAN_THROTTLE_CONGESTED => {
    fallthrough;
    }
    VMSCAN_THROTTLE_NOPROGRESS => {
    if (skip_throttle_noprogress(pgdat)) {
    cond_resched();
    return;
    }
    timeout = 1;
    // break;
    }
    VMSCAN_THROTTLE_ISOLATED => {
    timeout = HZ/50;
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    timeout = HZ;
    // break;
    }
    }
    prepare_to_wait(wqh, &wait, TASK_UNINTERRUPTIBLE);
    ret = schedule_timeout(timeout);
    finish_wait(wqh, &wait);
    if (reason == VMSCAN_THROTTLE_WRITEBACK) {
    atomic_dec(&pgdat.nr_writeback_throttled);
    }
    trace_mm_vmscan_throttled(pgdat.node_id, jiffies_to_usecs(timeout),
    jiffies_to_usecs(timeout - ret),
    reason);
    }
//
// Account for folios written if tasks are throttled waiting on dirty
// folios to clean. If enough folios have been cleaned since throttling
// started then wakeup the throttled tasks.
//
#[no_mangle]
pub unsafe extern "C" fn __acct_reclaim_writeback(pgdat: *mut pg_data_t, folio: *mut folio, nr_throttled: c_int) {
    let mut nr_written = 0;
    node_stat_add_folio(folio, NR_THROTTLED_WRITTEN);
//
// This is an inaccurate read as the per-cpu deltas may not
// be synchronised. However, given that the system is
// writeback throttled, it is not worth taking the penalty
// of getting an accurate count. At worst, the throttle
// timeout guarantees forward progress.
//
    nr_written = node_page_state(pgdat, NR_THROTTLED_WRITTEN) -
    READ_ONCE(pgdat.nr_reclaim_start);
    if (nr_written > SWAP_CLUSTER_MAX * nr_throttled) {
    wake_up(&pgdat.reclaim_wait[VMSCAN_THROTTLE_WRITEBACK]);
    }
    }
// possible outcome of pageout()
    typedef enum {
// failed to write folio out, folio is locked
    PAGE_KEEP,
// move folio to the active list, folio is locked
    PAGE_ACTIVATE,
// folio has been sent to the disk successfully, folio is unlocked
    PAGE_SUCCESS,
// folio is clean and locked
    PAGE_CLEAN,
    } pageout_t;
//
// pageout is called by shrink_folio_list() for each dirty folio.
//
    static pageout_t pageout(swap_io_ctx *ctx, address_space *mapping, folio *folio, list_head *folio_list)
    {
    let mut res = 0;
//
// We no longer attempt to writeback filesystem folios here, other
// than tmpfs/shmem.  That's taken care of in page-writeback.
// If we find a dirty filesystem folio at the end of the LRU list,
// typically that means the filesystem is saturating the storage
// with contiguous writes and telling it to write a folio here
// would only make the situation worse by injecting an element
// of random access.
//
// If the folio is swapcache, write it back even if that would
// block, for some throttling. This happens by accident, because
// swap_backing_dev_info is bust: it doesn't reflect the
// congestion state of the swapdevs.  Easy to fix, if needed.
//
// A freeable shmem or swapcache folio is referenced only by the
// caller that isolated the folio and the page cache.
//
    if (folio_ref_count(folio) != 1 + folio_nr_pages(folio) || !mapping) {
    return PAGE_KEEP;
    }
    if (!shmem_mapping(mapping) && !folio_test_anon(folio)) {
    return PAGE_ACTIVATE;
    }
    if (!folio_clear_dirty_for_io(folio)) {
    return PAGE_CLEAN;
    }
    folio_set_reclaim(folio);
//
// The large shmem folio can be split if CONFIG_THP_SWAP is not enabled
// or we failed to allocate contiguous swap entries, in which case
// the split out folios get added back to folio_list.
//
    if (shmem_mapping(mapping)) {
    res = shmem_writeout(ctx, folio, folio_list);
    }
    else {
    res = swap_writeout(ctx, folio);
    }
    if (res < 0) {
    handle_write_error(mapping, folio, res);
    }
    if (res == AOP_WRITEPAGE_ACTIVATE) {
    folio_clear_reclaim(folio);
    return PAGE_ACTIVATE;
    }
// synchronous write?
    if (!folio_test_writeback(folio)) {
    folio_clear_reclaim(folio);
    }
    trace_mm_vmscan_write_folio(folio);
    lruvec_stat_mod_folio(folio, NR_VMSCAN_WRITE, folio_nr_pages(folio));
    return PAGE_SUCCESS;
    }
//
// Same as remove_mapping, but if the folio is removed from the mapping, it
// gets returned with a refcount of 0.
//
#[no_mangle]
pub unsafe extern "C" fn __remove_mapping(mapping: *mut address_space, folio: *mut folio, reclaimed: bool, target_memcg: *mut mem_cgroup) -> c_int {
    let mut refcount = 0;
    let mut shadow = core::ptr::null_mut();
pub static mut ci: *mut c_void = core::ptr::null_mut();
    BUG_ON!(!folio_test_locked(folio));
    BUG_ON!(mapping != folio_mapping(folio));
    if (folio_test_swapcache(folio)) {
    ci = swap_cluster_get_and_lock_irq(folio);
    } else {
    spin_lock(&mapping.host.i_lock);
    xa_lock_irq(&mapping.i_pages);
    }
//
// The non racy check for a busy folio.
//
// Must be careful with the order of the tests. When someone has
// a ref to the folio, it may be possible that they dirty it then
// drop the reference. So if the dirty flag is tested before the
// refcount here, then the following race may occur:
//
// get_user_pages(&page);
// [user mapping goes away]
// write_to(page);
// !folio_test_dirty(folio)    [good]
// folio_set_dirty(folio);
// folio_put(folio);
// !refcount(folio)   [good, discard it]
//
// [oops, our write_to data is lost]
//
// Reversing the order of the tests ensures such a situation cannot
// escape unnoticed. The smp_rmb is needed to ensure the folio->flags
// load is not satisfied before that of folio->_refcount.
//
// Note that if the dirty flag is always set via folio_mark_dirty,
// and thus under the i_pages lock, then this ordering is not required.
//
    refcount = 1 + folio_nr_pages(folio);
    if (!folio_ref_freeze(folio, refcount)) {
// goto;
    }
// note: atomic_cmpxchg in folio_ref_freeze provides the smp_rmb
    if (unlikely(folio_test_dirty(folio))) {
    folio_ref_unfreeze(folio, refcount);
// goto;
    }
    if (folio_test_swapcache(folio)) {
pub static mut swap: swp_entry_t = 0;
    if (reclaimed && !mapping_exiting(mapping)) {
    shadow = workingset_eviction(folio, target_memcg);
    }
    __memcg1_swapout(folio, ci);
    __swap_cache_del_folio(ci, folio, swap, shadow);
    swap_cluster_unlock_irq(ci);
    } else {
    void (*free_folio);
    free_folio = mapping.a_ops.free_folio;
//
// Remember a shadow entry for reclaimed file cache in
// order to detect refaults, thus thrashing, later on.
//
// But don't store shadows in an address space that is
// already exiting.  This is not just an optimization,
// inode reclaim needs to empty out the radix tree or
// the nodes are lost.  Don't plant shadows behind its
// back.
//
// We also don't store shadows for DAX mappings because the
// only page cache folios found in these are zero pages
// covering holes, and because we don't want to mix DAX
// exceptional entries and shadow exceptional entries in the
// same address_space.
//
    if (reclaimed && folio_is_file_lru(folio) &&
    !mapping_exiting(mapping) && !dax_mapping(mapping)) {
    shadow = workingset_eviction(folio, target_memcg);
    }
    __filemap_remove_folio(folio, shadow);
    xa_unlock_irq(&mapping.i_pages);
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    spin_unlock(&mapping.host.i_lock);
    if (free_folio) {
    free_folio(folio);
    }
    }
    return 1;
// label;
    if (folio_test_swapcache(folio)) {
    swap_cluster_unlock_irq(ci);
    } else {
    xa_unlock_irq(&mapping.i_pages);
    spin_unlock(&mapping.host.i_lock);
    }
    return 0;
    }
//
// remove_mapping() - Attempt to remove a folio from its mapping.
// @mapping: The address space.
// @folio: The folio to remove.
//
// If the folio is dirty, under writeback or if someone else has a ref
// on it, removal will fail.
// Return: The number of pages removed from the mapping.  0 if the folio
// could not be removed.
// Context: The caller should have a single refcount on the folio and
// hold its lock.
//
#[no_mangle]
pub unsafe extern "C" fn remove_mapping(mapping: *mut address_space, folio: *mut folio) -> c_long {
    if (__remove_mapping(mapping, folio, false, core::ptr::null_mut())) {
//
// Unfreezing the refcount with 1 effectively
// drops the pagecache ref for us without requiring another
// atomic operation.
//
    folio_ref_unfreeze(folio, 1);
    return folio_nr_pages(folio);
    }
    return 0;
    }
//
// folio_putback_lru - Put previously isolated folio onto appropriate LRU list.
// @folio: Folio to be returned to an LRU list.
//
// Add previously isolated @folio to appropriate LRU list.
// The folio may still be unevictable for other reasons.
//
// Context: lru_lock must not be held, interrupts must be enabled.
//
#[no_mangle]
pub unsafe extern "C" fn folio_putback_lru(folio: *mut folio) {
    folio_add_lru(folio);
    folio_put(folio);		/* drop ref from isolate */
    }
    enum folio_references {
    FOLIOREF_RECLAIM,
    FOLIOREF_KEEP,
    FOLIOREF_ACTIVATE,
    };

//
// Only used on a mapped folio in the eviction (rmap walk) path, where promotion
// needs to be done by taking the folio off the LRU list and then adding it back
// with PG_active set. In contrast, the aging (page table walk) path uses
// folio_update_gen().
//
#[no_mangle]
unsafe extern "C" fn lru_gen_set_refs(folio: *mut folio, vma_flags: *const vma_flags_t) -> bool {
// see the comment on LRU_REFS_FLAGS
    if (!folio_test_referenced(folio) && !folio_test_workingset(folio)) {
// Activate file-backed executable folios after first usage.
    if (is_exec_file_folio(folio, vma_flags)) {
    set_mask_bits(&folio.flags.f, LRU_REFS_FLAGS, BIT(PG_workingset));
    return true;
    }
    set_mask_bits(&folio.flags.f, LRU_REFS_MASK, BIT(PG_referenced));
    return false;
    }
// Promote on second access
    if (folio_lru_refs(folio) > 1) {
    set_mask_bits(&folio.flags.f, LRU_REFS_FLAGS, BIT(PG_workingset));
    }
    else {
    folio_mark_accessed(folio);
    }
    return true;
    }

#[no_mangle]
unsafe extern "C" fn lru_gen_set_refs(folio: *mut folio, vma_flags: *const vma_flags_t) -> bool {
    return false;
    }

    static enum folio_references folio_check_references(folio *folio, scan_control *sc)
    {
    let mut referenced_ptes = 0;
    let mut referenced_folio = 0;
    let mut vma_flags;
    referenced_ptes = folio_referenced(folio, 1, sc.target_mem_cgroup,
    &vma_flags);
//
// The supposedly reclaimable folio was found to be in a VM_LOCKED vma.
// Let the folio, now marked Mlocked, be moved to the unevictable list.
//
    if (vma_flags_test(&vma_flags, VMA_LOCKED_BIT)) {
    return FOLIOREF_ACTIVATE;
    }
//
// There are two cases to consider.
// 1) Rmap lock contention: rotate.
// 2) Skip the non-shared swapbacked folio mapped solely by
// the exiting or OOM-reaped process.
//
    if (referenced_ptes == -1) {
    return FOLIOREF_KEEP;
    }
    if (lru_gen_enabled() && !lru_gen_switching()) {
    if (!referenced_ptes) {
    return FOLIOREF_RECLAIM;
    }
    return lru_gen_set_refs(folio, &vma_flags) ? FOLIOREF_ACTIVATE : FOLIOREF_KEEP;
    }
    referenced_folio = folio_test_clear_referenced(folio);
    if (referenced_ptes) {
//
// All mapped folios start out with page table
// references from the instantiating fault, so we need
// to look twice if a mapped file/anon folio is used more
// than once.
//
// Mark it and spare it for another trip around the
// inactive list.  Another page table reference will
// lead to its activation.
//
// Note: the mark is set for activated folios as well
// so that recently deactivated but used folios are
// quickly recovered.
//
    folio_set_referenced(folio);
    if (referenced_folio || referenced_ptes > 1) {
    return FOLIOREF_ACTIVATE;
    }
//
// Activate file-backed executable folios after first usage.
//
    if (is_exec_file_folio(folio, &vma_flags)) {
    return FOLIOREF_ACTIVATE;
    }
    return FOLIOREF_KEEP;
    }
    return FOLIOREF_RECLAIM;
    }
// Check if a folio is dirty or under writeback
#[no_mangle]
pub unsafe extern "C" fn folio_check_dirty_writeback(folio: *mut folio, dirty: *mut bool, writeback: *mut bool) {
pub static mut mapping: *mut c_void = core::ptr::null_mut();
//
// Anonymous folios are not handled by flushers and must be written
// from reclaim context. Do not stall reclaim based on them.
// MADV_FREE anonymous folios are put into inactive file list too.
// They could be mistakenly treated as file lru. So further anon
// test is needed.
//
    if (!folio_is_file_lru(folio) || folio_test_lazyfree(folio)) {
// dirty = false;
// writeback = false;
    return;
    }
// By default assume that the folio flags are accurate
// dirty = folio_test_dirty(folio);
// writeback = folio_test_writeback(folio);
// Verify dirty/writeback state if the filesystem supports it
    if (!folio_test_private(folio)) {
    return;
    }
    mapping = folio_mapping(folio);
    if (mapping && mapping.a_ops.is_dirty_writeback) {
    mapping.a_ops.is_dirty_writeback(folio, dirty, writeback);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_demote_folio(src: *mut folio, private: c_ulong) -> *mut c_void {
    struct migration_target_control *mtc, target_nid_mtc;
pub static mut dst: *mut c_void = core::ptr::null_mut();
    mtc = private;
//
// make sure we allocate from the target node first also trying to
// demote or reclaim pages from the target node via kswapd if we are
// low on free memory on target node. If we don't do this and if
// we have free memory on the slower(lower) memtier, we would start
// allocating pages from slower(lower) memory tiers without even forcing
// a demotion of cold pages from the target memtier. This can result
// in the kernel placing hot pages in slower(lower) memory tiers.
//
    target_nid_mtc = *mtc;
    target_nid_mtc.nmask = core::ptr::null_mut();
    target_nid_mtc.gfp_mask |= __GFP_THISNODE;
    dst = alloc_migration_target(src, (unsigned long)&target_nid_mtc);
    if (dst) {
    return dst;
    }
    return alloc_migration_target(src, (unsigned long)mtc);
    }
//
// Take folios on @demote_folios and attempt to demote them to another node.
// Folios which are not demoted are left on @demote_folios.
//
#[no_mangle]
pub unsafe extern "C" fn demote_folio_list(demote_folios: *mut list_head, pgdat: *mut pglist_data, memcg: *mut mem_cgroup) -> c_uint {
    let mut target_nid = 0;
    let mut nr_succeeded = 0;
    let mut allowed_mask;
pub static mut migration_target_control: usize = 0;
    if (list_empty(demote_folios)) {
    return 0;
    }
    node_get_allowed_targets(pgdat, &allowed_mask);
    mem_cgroup_node_filter_allowed(memcg, &allowed_mask);
    if (nodes_empty(allowed_mask)) {
    return 0;
    }
    target_nid = next_demotion_node(pgdat.node_id, &allowed_mask);
    if (target_nid == NUMA_NO_NODE) {
// No lower-tier nodes or nodes were hot-unplugged.
    return 0;
    }
    mtc.nid = target_nid;
// Demotion ignores all cpuset and mempolicy settings
    migrate_pages(demote_folios, alloc_demote_folio, core::ptr::null_mut(),
    (unsigned long)&mtc, MIGRATE_ASYNC, MR_DEMOTION,
    &nr_succeeded);
    return nr_succeeded;
    }
#[no_mangle]
unsafe extern "C" fn may_enter_fs(folio: *mut folio, gfp_mask: gfp_t) -> bool {
    if (gfp_mask & __GFP_FS) {
    return true;
    }
//
// We can "enter_fs" for swap-cache with only __GFP_IO unless backed by
// a swapfile that requires GFP_NOFS I/O.
//
    if (folio_test_swapcache(folio) && (gfp_mask & __GFP_IO) &&
    !(__swap_entry_to_info(folio.swap).ops.flags &
    SWAP_OPS_F_REQUIRE_NOFS)) {
    return true;
    }
    return false;
    }
//
// shrink_folio_list() returns the number of reclaimed pages
//
#[no_mangle]
pub unsafe extern "C" fn shrink_folio_list(folio_list: *mut list_head, pgdat: *mut pglist_data, sc: *mut scan_control, stat: *mut reclaim_stat, ignore_references: bool, memcg: *mut mem_cgroup) -> c_uint {
pub static mut free_folios: usize = 0;
pub static mut ret_folios: usize = 0;
pub static mut demote_folios: usize = 0;
pub static mut nr_reclaimed: c_uint = 0;
pub static mut pgactivate: c_uint = 0;
    let mut do_demote_pass = 0;
pub static mut ctx: swap_io_ctx = 0;
    folio_batch_init(&free_folios);
    memset(stat, 0, sizeof!(*stat));
    cond_resched();
    do_demote_pass = can_demote(pgdat.node_id, sc, memcg);
// label;
    while (!list_empty(folio_list)) {
pub static mut mapping: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut references: folio_references = 0;
    let mut dirty = 0;
    let mut writeback = 0;
    let mut nr_pages = 0;
    cond_resched();
    folio = lru_to_folio(folio_list);
    list_del(&folio.lru);
    if (!folio_trylock(folio)) {
// goto;
    }
    if (folio_contain_hwpoisoned_page(folio)) {
//
// unmap_poisoned_folio() can't handle large
// folio, just skip it. memory_failure() will
// handle it if the UCE is triggered again.
//
    if (folio_test_large(folio)) {
// goto;
    }
    unmap_poisoned_folio(folio, folio_pfn(folio), false);
    folio_unlock(folio);
    folio_put(folio);
    continue;
    }
    VM_BUG_ON_FOLIO(folio_test_active(folio), folio);
    nr_pages = folio_nr_pages(folio);
// Account the number of base pages
    sc.nr_scanned += nr_pages;
    if (unlikely(!folio_evictable(folio))) {
// goto;
    }
    if (!sc.may_unmap && folio_mapped(folio)) {
// goto;
    }
//
// The number of dirty pages determines if a node is marked
// reclaim_congested. kswapd will stall and start writing
// folios if the tail of the LRU is all dirty unqueued folios.
//
    folio_check_dirty_writeback(folio, &dirty, &writeback);
    if (dirty || writeback) {
    stat.nr_dirty += nr_pages;
    }
    if (dirty && !writeback) {
    stat.nr_unqueued_dirty += nr_pages;
    }
//
// Treat this folio as congested if folios are cycling
// through the LRU so quickly that the folios marked
// for immediate reclaim are making it to the end of
// the LRU a second time.
//
    if (writeback && folio_test_reclaim(folio)) {
    stat.nr_congested += nr_pages;
    }
//
// If a folio at the tail of the LRU is under writeback, there
// are three cases to consider.
//
// 1) If reclaim is encountering an excessive number
// of folios under writeback and this folio has both
// the writeback and reclaim flags set, then it
// indicates that folios are being queued for I/O but
// are being recycled through the LRU before the I/O
// can complete. Waiting on the folio itself risks an
// indefinite stall if it is impossible to writeback
// the folio due to I/O error or disconnected storage
// so instead note that the LRU is being scanned too
// quickly and the caller can stall after the folio
// list has been processed.
//
// 2) Global or new memcg reclaim encounters a folio that is
// not marked for immediate reclaim, or the caller does not
// have __GFP_FS (or __GFP_IO if it's simply going to swap,
// not to fs), or the folio belongs to a mapping where
// waiting on writeback during reclaim may lead to a deadlock.
// In this case mark the folio for immediate reclaim and
// continue scanning.
//
// Require may_enter_fs() because we would wait on fs, which
// may not have submitted I/O yet. And the loop driver might
// enter reclaim, and deadlock if it waits on a folio for
// which it is needed to do the write (loop masks off
// __GFP_IO|__GFP_FS for this reason); but more thought
// would probably show more reasons.
//
// 3) Legacy memcg encounters a folio that already has the
// reclaim flag set. memcg does not have any dirty folio
// throttling so we could easily OOM just because too many
// folios are in writeback and there is nothing else to
// reclaim. Wait for the writeback to complete.
//
// In cases 1) and 2) we activate the folios to get them out of
// the way while we continue scanning for clean folios on the
// inactive list and refilling from the active list. The
// observation here is that waiting for disk writes is more
// expensive than potentially causing reloads down the line.
// Since they're marked for immediate reclaim, they won't put
// memory pressure on the cache working set any longer than it
// takes to write them to disk.
//
    if (folio_test_writeback(folio)) {
    mapping = folio_mapping(folio);
// Case 1 above
    if (current_is_kswapd() &&
    folio_test_reclaim(folio) &&
    test_bit(PGDAT_WRITEBACK, &pgdat.flags)) {
    stat.nr_immediate += nr_pages;
// goto;
// Case 2 above
    } else if (writeback_throttling_sane(sc) ||
    !folio_test_reclaim(folio) ||
    !may_enter_fs(folio, sc.gfp_mask) ||
    (mapping &&
    mapping_writeback_may_deadlock_on_reclaim(mapping))) {
//
// This is slightly racy -
// folio_end_writeback() might have
// just cleared the reclaim flag, then
// setting the reclaim flag here ends up
// interpreted as the readahead flag - but
// that does not matter enough to care.
// What we do want is for this folio to
// have the reclaim flag set next time
// memcg reclaim reaches the tests above,
// so it will then wait for writeback to
// avoid OOM; and it's also appropriate
// in global reclaim.
//
    folio_set_reclaim(folio);
    stat.nr_writeback += nr_pages;
// goto;
// Case 3 above
    } else {
    folio_unlock(folio);
    folio_wait_writeback(folio);
// then go back and try same folio again
    list_add_tail(&folio.lru, folio_list);
    continue;
    }
    }
    if (!ignore_references) {
    references = folio_check_references(folio, sc);
    }
    match (references) {
    FOLIOREF_ACTIVATE => {
// goto;
    }
    FOLIOREF_KEEP => {
    stat.nr_ref_keep += nr_pages;
// goto;
    }
    FOLIOREF_RECLAIM => {
    ; /* try to reclaim the folio below */
    }
    }
//
// Before reclaiming the folio, try to relocate
// its contents to another node.
//
    if (do_demote_pass &&
    (thp_migration_supported() || !folio_test_large(folio))) {
    list_add(&folio.lru, &demote_folios);
    folio_unlock(folio);
    continue;
    }
//
// Anonymous process memory has backing store?
// Try to allocate it some swap space here.
// Lazyfree folio could be freed directly
//
    if (folio_test_anon(folio) && folio_test_swapbacked(folio) &&
    !folio_test_swapcache(folio)) {
    if (!(sc.gfp_mask & __GFP_IO)) {
// goto;
    }
    if (folio_maybe_dma_pinned(folio)) {
// goto;
    }
    if (folio_test_large(folio)) {
// cannot split folio, skip it
    if (folio_expected_ref_count(folio) !=
    folio_ref_count(folio) - 1) {
// goto;
    }
//
// Split partially mapped folios right away.
// We can free the unmapped pages without IO.
//
    if (data_race(!list_empty(&folio._deferred_list) &&
    folio_test_partially_mapped(folio)) &&
    split_folio_to_list(folio, folio_list)) {
// goto;
    }
    }
    if (folio_alloc_swap(folio)) {
pub static mut order: int __maybe_unused = 0;
    if (!folio_test_large(folio)) {
// goto;
    }
// Fallback to swap normal pages
    if (split_folio_to_list(folio, folio_list)) {
// goto;
    }

    if (nr_pages >= HPAGE_PMD_NR) {
    count_memcg_folio_events(folio,
    THP_SWPOUT_FALLBACK, 1);
    count_vm_event(THP_SWPOUT_FALLBACK);
    }

    count_mthp_stat(order, MTHP_STAT_SWPOUT_FALLBACK);
    if (folio_alloc_swap(folio)) {
// goto;
    }
    }
//
// Normally the folio will be dirtied in unmap because
// its pte should be dirty. A special case is MADV_FREE
// page. The page's pte could have dirty bit cleared but
// the folio's SwapBacked flag is still set because
// clearing the dirty bit and SwapBacked flag has no
// lock protected. For such folio, unmap will not set
// dirty bit for it, so folio reclaim will not write the
// folio out. This can cause data corruption when the
// folio is swapped in later. Always setting the dirty
// flag for the folio solves the problem.
//
    folio_mark_dirty(folio);
    }
//
// If the folio was split above, the tail pages will make
// their own pass through this function and be accounted
// then.
//
    if ((nr_pages > 1) && !folio_test_large(folio)) {
    sc.nr_scanned -= (nr_pages - 1);
    nr_pages = 1;
    }
//
// The folio is mapped into the page tables of one or more
// processes. Try to unmap it here.
//
    if (folio_mapped(folio)) {
pub static mut flags: ttu_flags = 0;
pub static mut was_swapbacked: bool = false;
    if (folio_test_pmd_mappable(folio)) {
    flags |= TTU_SPLIT_HUGE_PMD;
    }
//
// Without TTU_SYNC, try_to_unmap will only begin to
// hold PTL from the first present PTE within a large
// folio. Some initial PTEs might be skipped due to
// races with parallel PTE writes in which PTEs can be
// cleared temporarily before being written new present
// values. This will lead to a large folio is still
// mapped while some subpages have been partially
// unmapped after try_to_unmap; TTU_SYNC helps
// try_to_unmap acquire PTL from the first PTE,
// eliminating the influence of temporary PTE values.
//
    if (folio_test_large(folio)) {
    flags |= TTU_SYNC;
    }
    try_to_unmap(folio, flags);
    if (folio_mapped(folio)) {
    stat.nr_unmap_fail += nr_pages;
    if (!was_swapbacked &&
    folio_test_swapbacked(folio)) {
    stat.nr_lazyfree_fail += nr_pages;
    }
// goto;
    }
    }
//
// Folio is unmapped now so it cannot be newly pinned anymore.
// No point in trying to reclaim folio if it is pinned.
// Furthermore we don't want to reclaim underlying fs metadata
// if the folio is pinned and thus potentially modified by the
// pinning process as that may upset the filesystem.
//
    if (folio_maybe_dma_pinned(folio)) {
// goto;
    }
    mapping = folio_mapping(folio);
    if (folio_test_dirty(folio)) {
    if (folio_is_file_lru(folio)) {
//
// Immediately reclaim when written back.
// Similar in principle to folio_deactivate()
// except we already have the folio isolated
// and know it's dirty
//
    node_stat_mod_folio(folio, NR_VMSCAN_IMMEDIATE,
    nr_pages);
    if (!folio_test_reclaim(folio)) {
    folio_set_reclaim(folio);
    }
// goto;
    }
    if (!may_enter_fs(folio, sc.gfp_mask)) {
// goto;
    }
    if (!sc.may_writepage) {
// goto;
    }
//
// Folio is dirty. Flush the TLB if a writable entry
// potentially exists to avoid CPU writes after I/O
// starts and then write it out here.
//
    try_to_unmap_flush_dirty();
    switch (pageout(&ctx, mapping, folio, folio_list)) {
    case PAGE_KEEP:
// goto;
    case PAGE_ACTIVATE:
//
// If shmem folio is split when writeback to swap,
// the tail pages will make their own pass through
// this function and be accounted then.
//
    if (nr_pages > 1 && !folio_test_large(folio)) {
    sc.nr_scanned -= (nr_pages - 1);
    nr_pages = 1;
    }
// goto;
    case PAGE_SUCCESS:
    if (nr_pages > 1 && !folio_test_large(folio)) {
    sc.nr_scanned -= (nr_pages - 1);
    nr_pages = 1;
    }
    if (folio_test_writeback(folio)) {
// goto;
    }
    if (folio_test_dirty(folio)) {
// goto;
    }
//
// A synchronous write - probably a ramdisk.  Go
// ahead and try to reclaim the folio.
//
    if (!folio_trylock(folio)) {
// goto;
    }
    if (folio_test_dirty(folio) ||
    folio_test_writeback(folio)) {
// goto;
    }
    mapping = folio_mapping(folio);
    fallthrough;
    case PAGE_CLEAN:
    ; /* try to free the folio below */
    }
    }
//
// If the folio has buffers, try to free the buffer
// mappings associated with this folio. If we succeed
// we try to free the folio as well.
//
// We do this even if the folio is dirty.
// filemap_release_folio() does not perform I/O, but it
// is possible for a folio to have the dirty flag set,
// but it is actually clean (all its buffers are clean).
// This happens if the buffers were written out directly,
// with bh_submit(). ext3 will do this, as well as
// the blockdev mapping.  filemap_release_folio() will
// discover that cleanness and will drop the buffers
// and mark the folio clean - it can be freed.
//
// Rarely, folios can have buffers and no ->mapping.
// These are the folios which were not successfully
// invalidated in truncate_cleanup_folio().  We try to
// drop those buffers here and if that worked, and the
// folio is no longer mapped into process address space
// (refcount == 1) it can be freed.  Otherwise, leave
// the folio on the LRU so it is swappable.
//
    if (folio_needs_release(folio)) {
    if (!filemap_release_folio(folio, sc.gfp_mask)) {
// goto;
    }
    if (!mapping && folio_ref_count(folio) == 1) {
    folio_unlock(folio);
    if (folio_put_testzero(folio)) {
// goto;
    }
    else {
//
// rare race with speculative reference.
// the speculative reference will free
// this folio shortly, so we may
// increment nr_reclaimed here (and
// leave it off the LRU).
//
    nr_reclaimed += nr_pages;
    continue;
    }
    }
    }
    if (folio_test_lazyfree(folio)) {
// follow __remove_mapping for reference
    if (!folio_ref_freeze(folio, 1)) {
// goto;
    }
//
// The folio has only one reference left, which is
// from the isolation. After the caller puts the
// folio back on the lru and drops the reference, the
// folio will be freed anyway. It doesn't matter
// which lru it goes on. So we don't bother checking
// the dirty flag here.
//
    count_vm_events(PGLAZYFREED, nr_pages);
    count_memcg_folio_events(folio, PGLAZYFREED, nr_pages);
    } else if (!mapping || !__remove_mapping(mapping, folio, true,
    sc.target_mem_cgroup)) {
// goto;
    }
    folio_unlock(folio);
// label;
//
// Folio may get swapped out as a whole, need to account
// all pages in it.
//
    nr_reclaimed += nr_pages;
    folio_unqueue_deferred_split(folio);
    if (folio_batch_add(&free_folios, folio) == 0) {
    mem_cgroup_uncharge_folios(&free_folios);
    try_to_unmap_flush();
    free_unref_folios(&free_folios);
    }
    continue;
// label;
//
// The tail pages that are failed to add into swap cache
// reach here.  Fixup nr_scanned and nr_pages.
//
    if (nr_pages > 1) {
    sc.nr_scanned -= (nr_pages - 1);
    nr_pages = 1;
    }
// label;
// Not a candidate for swapping, so reclaim swap space.
    if (folio_test_swapcache(folio) &&
    (mem_cgroup_swap_full(folio) || folio_test_mlocked(folio))) {
    folio_free_swap(folio);
    }
    VM_BUG_ON_FOLIO(folio_test_active(folio), folio);
    if (!folio_test_mlocked(folio)) {
pub static mut type: c_int = 0;
    folio_set_active(folio);
    stat.nr_activate[type] += nr_pages;
    count_memcg_folio_events(folio, PGACTIVATE, nr_pages);
    }
// label;
    folio_unlock(folio);
// label;
    list_add(&folio.lru, &ret_folios);
    VM_BUG_ON_FOLIO(folio_test_lru(folio) ||
    folio_test_unevictable(folio), folio);
    }
// 'folio_list' is always empty here
// Migrate folios selected for demotion
    nr_demoted = demote_folio_list(&demote_folios, pgdat, memcg);
    nr_reclaimed += nr_demoted;
    stat.nr_demoted += nr_demoted;
// Folios that could not be demoted are still in @demote_folios
    if (!list_empty(&demote_folios)) {
// Folios which weren't demoted go back on @folio_list
    list_splice_init(&demote_folios, folio_list);
//
// goto retry to reclaim the undemoted folios in folio_list if
// desired.
//
// Reclaiming directly from top tier nodes is not often desired
// due to it breaking the LRU ordering: in general memory
// should be reclaimed from lower tier nodes and demoted from
// top tier nodes.
//
// However, disabling reclaim from top tier nodes entirely
// would cause ooms in edge scenarios where lower tier memory
// is unreclaimable for whatever reason, eg memory being
// mlocked or too hot to reclaim. We can disable reclaim
// from top tier nodes in proactive reclaim though as that is
// not real memory pressure.
//
    if (!sc.proactive) {
    do_demote_pass = false;
// goto;
    }
    }
    pgactivate = stat.nr_activate[0] + stat.nr_activate[1];
    mem_cgroup_uncharge_folios(&free_folios);
    try_to_unmap_flush();
    free_unref_folios(&free_folios);
    list_splice(&ret_folios, folio_list);
    count_vm_events(PGACTIVATE, pgactivate);
    swap_write_submit(&ctx);
    return nr_reclaimed;
    }
#[no_mangle]
pub unsafe extern "C" fn reclaim_clean_pages_from_list(zone: *mut zone, folio_list: *mut list_head) -> c_uint {
pub static mut scan_control: usize = 0;
pub static mut stat: usize = 0;
    let mut nr_reclaimed = 0;
    let mut folio = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut clean_folios: usize = 0;
    let mut noreclaim_flag = 0;
    list_for_each_entry_safe(folio, next, folio_list, lru) {
// TODO: these pages should not even appear in this list.
    if (page_has_movable_ops(&folio.page)) {
    continue;
    }
    if (!folio_test_hugetlb(folio) && folio_is_file_lru(folio) &&
    !folio_test_dirty(folio) && !folio_test_unevictable(folio)) {
    folio_clear_active(folio);
    list_move(&folio.lru, &clean_folios);
    }
    }
//
// We should be safe here since we are only dealing with file pages and
// we are not kswapd and therefore cannot write dirty file pages. But
// call memalloc_noreclaim_save() anyway, just in case these conditions
// change in the future.
//
    noreclaim_flag = memalloc_noreclaim_save();
    nr_reclaimed = shrink_folio_list(&clean_folios, zone.zone_pgdat, &sc,
    &stat, true, core::ptr::null_mut());
    memalloc_noreclaim_restore(noreclaim_flag);
    list_splice(&clean_folios, folio_list);
    mod_node_page_state(zone.zone_pgdat, NR_ISOLATED_FILE,
    -(long)nr_reclaimed);
//
// Since lazyfree pages are isolated from file LRU from the beginning,
// they will rotate back to anonymous LRU in the end if it failed to
// discard so isolated count will be mismatched.
// Compensate the isolated count for both LRU lists.
//
    mod_node_page_state(zone.zone_pgdat, NR_ISOLATED_ANON,
    stat.nr_lazyfree_fail);
    mod_node_page_state(zone.zone_pgdat, NR_ISOLATED_FILE,
    -(long)stat.nr_lazyfree_fail);
    return nr_reclaimed;
    }
//
// Update LRU sizes after isolating pages. The LRU size updates must
// be complete before mem_cgroup_update_lru_size due to a sanity check.
//
    static __always_inline void update_lru_sizes(lruvec *lruvec,
    enum lru_list lru, unsigned long *nr_zone_taken)
    {
    let mut zid = 0;
    while (zid < MAX_NR_ZONES) {
    if (!nr_zone_taken[zid]) {
    continue;
    }
    update_lru_size(lruvec, lru, zid, -nr_zone_taken[zid]);
    }
    }
//
// Isolating page from the lruvec to fill in @dst list by nr_to_scan times.
//
// lruvec->lru_lock is heavily contended.  Some of the functions that
// shrink the lists perform better by taking out a batch of pages
// and working on them outside the LRU lock.
//
// For pagecache intensive workloads, this function is the hottest
// spot in the kernel (apart from copy_*_user functions).
//
// Lru_lock must be held before calling this function.
//
// @nr_to_scan:	The number of eligible pages to look through on the list.
// @lruvec:	The LRU vector to pull pages from.
// @dst:	The temp list to put pages on to.
// @nr_scanned:	The number of pages that were scanned.
// @sc:		The scan_control struct for this reclaim session
// @lru:	LRU list id for isolating
//
// returns how many pages were moved onto *@dst.
//
#[no_mangle]
pub unsafe extern "C" fn isolate_lru_folios(nr_to_scan: c_ulong, lruvec: *mut lruvec, dst: *mut list_head, nr_scanned: *mut c_ulong, sc: *mut scan_control, lru: lru_list) -> c_ulong {
    let mut src = &lruvec.lists[lru];
pub static mut nr_taken: c_ulong = 0;
    unsigned long nr_zone_taken[MAX_NR_ZONES] = { 0 };
    unsigned long nr_skipped[MAX_NR_ZONES] = { 0, };
pub static mut skipped: c_ulong = 0;
    let mut nr_pages = 0;
pub static mut max_nr_skipped: c_ulong = 0;
pub static mut folios_skipped: usize = 0;
    while (scan < nr_to_scan && !list_empty(src)) {
    let mut move_to = src;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    folio = lru_to_folio(src);
    prefetchw_prev_lru_folio(folio, src, flags);
    nr_pages = folio_nr_pages(folio);
    total_scan += nr_pages;
// Using max_nr_skipped to prevent hard LOCKUP
    if (max_nr_skipped < SWAP_CLUSTER_MAX_SKIPPED &&
    (folio_zonenum(folio) > sc.reclaim_idx)) {
    nr_skipped[folio_zonenum(folio)] += nr_pages;
    move_to = &folios_skipped;
    max_nr_skipped += 1;
// goto;
    }
//
// Do not count skipped folios because that makes the function
// return with no isolated folios if the LRU mostly contains
// ineligible folios.  This causes the VM to not reclaim any
// folios, triggering a premature OOM.
// Account all pages in a folio.
//
    scan += nr_pages;
    if (!folio_test_lru(folio)) {
// goto;
    }
    if (!sc.may_unmap && folio_mapped(folio)) {
// goto;
    }
//
// Be careful not to clear the lru flag until after we're
// sure the folio is not being freed elsewhere -- the
// folio release code relies on it.
//
    if (unlikely(!folio_try_get(folio))) {
// goto;
    }
    if (!folio_test_clear_lru(folio)) {
// Another thread is already isolating this folio
    folio_put(folio);
// goto;
    }
    nr_taken += nr_pages;
    nr_zone_taken[folio_zonenum(folio)] += nr_pages;
    move_to = dst;
// label;
    list_move(&folio.lru, move_to);
    }
//
// Splice any skipped folios to the start of the LRU list. Note that
// this disrupts the LRU order when reclaiming for lower zones but
// we cannot splice to the tail. If we did then the SWAP_CLUSTER_MAX
// scanning would soon rescan the same folios to skip and waste lots
// of cpu cycles.
//
    if (!list_empty(&folios_skipped)) {
    let mut zid = 0;
    list_splice(&folios_skipped, src);
    while (zid < MAX_NR_ZONES) {
    if (!nr_skipped[zid]) {
    continue;
    }
    __count_zid_vm_events(PGSCAN_SKIP, zid, nr_skipped[zid]);
    skipped += nr_skipped[zid];
    }
    }
// nr_scanned = total_scan;
    trace_mm_vmscan_lru_isolate(sc.reclaim_idx, sc.order, nr_to_scan,
    total_scan, skipped, nr_taken, lru);
    update_lru_sizes(lruvec, lru, nr_zone_taken);
    return nr_taken;
    }
//
// folio_isolate_lru() - Try to isolate a folio from its LRU list.
// @folio: Folio to isolate from its LRU list.
//
// Isolate a @folio from an LRU list and adjust the vmstat statistic
// corresponding to whatever LRU list the folio was on.
//
// The folio will have its LRU flag cleared.  If it was found on the
// active list, it will have the Active flag set.  If it was found on the
// unevictable list, it will have the Unevictable flag set.  These flags
// may need to be cleared by the caller before letting the page go.
//
// Context:
//
// (1) Must be called with an elevated refcount on the folio. This is a
// fundamental difference from isolate_lru_folios() (which is called
// without a stable reference).
// (2) The lru_lock must not be held.
// (3) Interrupts must be enabled.
//
// Return: true if the folio was removed from an LRU list.
// false if the folio was not on an LRU list.
//
#[no_mangle]
pub unsafe extern "C" fn folio_isolate_lru(folio: *mut folio) -> bool {
pub static mut ret: bool = false;
    VM_BUG_ON_FOLIO(!folio_ref_count(folio), folio);
    if (folio_test_clear_lru(folio)) {
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    folio_get(folio);
    lruvec = folio_lruvec_lock_irq(folio);
    lruvec_del_folio(lruvec, folio);
    lruvec_unlock_irq(lruvec);
    ret = true;
    }
    return ret;
    }
//
// A direct reclaimer may isolate SWAP_CLUSTER_MAX pages from the LRU list and
// then get rescheduled. When there are massive number of tasks doing page
// allocation, such sleeping direct reclaimers may keep piling up on each CPU,
// the LRU list will go small and be scanned faster than necessary, leading to
// unnecessary swapping, thrashing and OOM.
//
#[no_mangle]
pub unsafe extern "C" fn too_many_isolated(pgdat: *mut pglist_data, file: c_int, sc: *mut scan_control) -> bool {
    unsigned long inactive, isolated;
    let mut too_many = 0;
    if (current_is_kswapd()) {
    return false;
    }
    if (!writeback_throttling_sane(sc)) {
    return false;
    }
    if (file) {
    inactive = node_page_state(pgdat, NR_INACTIVE_FILE);
    isolated = node_page_state(pgdat, NR_ISOLATED_FILE);
    } else {
    inactive = node_page_state(pgdat, NR_INACTIVE_ANON);
    isolated = node_page_state(pgdat, NR_ISOLATED_ANON);
    }
//
// GFP_NOIO/GFP_NOFS callers are allowed to isolate more pages, so they
// won't get blocked by normal direct-reclaimers, forming a circular
// deadlock.
//
    if (gfp_has_io_fs(sc.gfp_mask)) {
    inactive >>= 3;
    }
    too_many = isolated > inactive;
// Wake up tasks throttled due to too_many_isolated.
    if (!too_many) {
    wake_throttle_isolated(pgdat);
    }
    return too_many;
    }
//
// move_folios_to_lru() moves folios from private @list to appropriate LRU list.
//
// Returns the number of pages moved to the appropriate lruvec.
//
// Note: The caller must not hold any lruvec lock.
//
#[no_mangle]
unsafe extern "C" fn move_folios_to_lru(list: *mut list_head) -> c_uint {
    int nr_pages, nr_moved = 0;
    let mut lruvec = core::ptr::null_mut();
pub static mut free_folios: usize = 0;
    folio_batch_init(&free_folios);
    while (!list_empty(list)) {
    let mut folio = lru_to_folio(list);
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
    list_del(&folio.lru);
    if (unlikely(!folio_evictable(folio))) {
    lruvec_unlock_irq(lruvec);
    folio_putback_lru(folio);
    lruvec = core::ptr::null_mut();
    continue;
    }
//
// The folio_set_lru needs to be kept here for list integrity.
// Otherwise:
// #0 move_folios_to_lru             #1 release_pages
// if (!folio_put_testzero())
// if (folio_put_testzero())
// !lru //skip lru_lock
// folio_set_lru()
// list_add(&folio->lru,)
//
    folio_set_lru(folio);
    if (unlikely(folio_put_testzero(folio))) {
    __folio_clear_lru_flags(folio);
    folio_unqueue_deferred_split(folio);
    if (folio_batch_add(&free_folios, folio) == 0) {
    lruvec_unlock_irq(lruvec);
    mem_cgroup_uncharge_folios(&free_folios);
    free_unref_folios(&free_folios);
    lruvec = core::ptr::null_mut();
    }
    continue;
    }
    lruvec_add_folio(lruvec, folio);
    nr_pages = folio_nr_pages(folio);
    nr_moved += nr_pages;
    if (folio_test_active(folio)) {
    workingset_age_nonresident(lruvec, nr_pages);
    }
    }
    if (lruvec) {
    lruvec_unlock_irq(lruvec);
    }
    if (free_folios.nr) {
    mem_cgroup_uncharge_folios(&free_folios);
    free_unref_folios(&free_folios);
    }
    return nr_moved;
    }
//
// If a kernel thread (such as nfsd for loop-back mounts) services a backing
// device by writing to the page cache it sets PF_LOCAL_THROTTLE. In this case
// we should not throttle.  Otherwise it is safe to do so.
//
#[no_mangle]
unsafe extern "C" fn current_may_throttle() -> c_int {
    return !(current.flags & PF_LOCAL_THROTTLE);
    }
#[no_mangle]
pub unsafe extern "C" fn handle_reclaim_writeback(nr_taken: c_ulong, pgdat: *mut pglist_data, sc: *mut scan_control, stat: *mut reclaim_stat) {
//
// If dirty folios are scanned that are not queued for IO, it
// implies that flushers are not doing their job. This can
// happen when memory pressure pushes dirty folios to the end of
// the LRU before the dirty limits are breached and the dirty
// data has expired. It can also happen when the proportion of
// dirty folios grows not through writes but through memory
// pressure reclaiming all the clean cache. And in some cases,
// the flushers simply cannot keep up with the allocation
// rate. Nudge the flusher threads in case they are asleep.
//
    if (stat.nr_unqueued_dirty == nr_taken) {
    wakeup_flusher_threads(WB_REASON_VMSCAN);
//
// For cgroupv1 dirty throttling is achieved by waking up
// the kernel flusher here and later waiting on folios
// which are in writeback to finish (see shrink_folio_list()).
//
// Flusher may not be able to issue writeback quickly
// enough for cgroupv1 writeback throttling to work
// on a large system.
//
    if (!writeback_throttling_sane(sc)) {
    reclaim_throttle(pgdat, VMSCAN_THROTTLE_WRITEBACK);
    }
    }
    sc.nr.dirty += stat.nr_dirty;
    sc.nr.congested += stat.nr_congested;
    sc.nr.writeback += stat.nr_writeback;
    sc.nr.immediate += stat.nr_immediate;
    sc.nr.taken += nr_taken;
    }
//
// shrink_inactive_list() is a helper for shrink_node().  It returns the number
// of reclaimed pages
//
#[no_mangle]
pub unsafe extern "C" fn shrink_inactive_list(nr_to_scan: c_ulong, lruvec: *mut lruvec, sc: *mut scan_control, lru: lru_list) -> c_ulong {
pub static mut folio_list: usize = 0;
    let mut nr_scanned = 0;
pub static mut nr_reclaimed: c_uint = 0;
    let mut nr_taken = 0;
pub static mut stat: usize = 0;
pub static mut file: bool = false;
    enum node_stat_item item;
    let mut pgdat = lruvec_pgdat(lruvec);
pub static mut stalled: bool = false;
    while (unlikely(too_many_isolated(pgdat, file, sc))) {
    if (stalled) {
    return 0;
    }
// wait a bit for the reclaimer.
    stalled = true;
    reclaim_throttle(pgdat, VMSCAN_THROTTLE_ISOLATED);
// We are about to die and free our memory. Return now.
    if (fatal_signal_pending(current)) {
    return SWAP_CLUSTER_MAX;
    }
    }
    lru_add_drain();
    lruvec_lock_irq(lruvec);
    nr_taken = isolate_lru_folios(nr_to_scan, lruvec, &folio_list,
    &nr_scanned, sc, lru);
    __mod_node_page_state(pgdat, NR_ISOLATED_ANON + file, nr_taken);
    item = PGSCAN_KSWAPD + reclaimer_offset(sc);
    mod_lruvec_state(lruvec, item, nr_scanned);
    mod_lruvec_state(lruvec, PGSCAN_ANON + file, nr_scanned);
    lruvec_unlock_irq(lruvec);
    if (nr_taken == 0) {
    return 0;
    }
    nr_reclaimed = shrink_folio_list(&folio_list, pgdat, sc, &stat, false,
    lruvec_memcg(lruvec));
    move_folios_to_lru(&folio_list);
    mod_lruvec_state(lruvec, PGDEMOTE_KSWAPD + reclaimer_offset(sc),
    stat.nr_demoted);
    mod_node_page_state(pgdat, NR_ISOLATED_ANON + file, -nr_taken);
    item = PGSTEAL_KSWAPD + reclaimer_offset(sc);
    mod_lruvec_state(lruvec, item, nr_reclaimed);
    mod_lruvec_state(lruvec, PGSTEAL_ANON + file, nr_reclaimed);
    if (nr_scanned > nr_reclaimed) {
    mod_lruvec_state(lruvec, PGROTATE_ANON + file,
    nr_scanned - nr_reclaimed);
    }
    handle_reclaim_writeback(nr_taken, pgdat, sc, &stat);
    trace_mm_vmscan_lru_shrink_inactive(pgdat.node_id,
    nr_scanned, nr_reclaimed, &stat, sc.priority, file);
    return nr_reclaimed;
    }
//
// shrink_active_list() moves folios from the active LRU to the inactive LRU.
//
// We move them the other way if the folio is referenced by one or more
// processes.
//
// If the folios are mostly unmapped, the processing is fast and it is
// appropriate to hold lru_lock across the whole operation.  But if
// the folios are mapped, the processing is slow (folio_referenced()), so
// we should drop lru_lock around each folio.  It's impossible to balance
// this, so instead we remove the folios from the LRU while processing them.
// It is safe to rely on the active flag against the non-LRU folios in here
// because nobody will play with that bit on a non-LRU folio.
//
// The downside is that we have to touch folio->_refcount against each folio.
// But we had to alter folio->flags anyway.
//
#[no_mangle]
pub unsafe extern "C" fn shrink_active_list(nr_to_scan: c_ulong, lruvec: *mut lruvec, sc: *mut scan_control, lru: lru_list) {
    let mut nr_taken = 0;
    let mut nr_scanned = 0;
    let mut vma_flags;
pub static mut l_hold: usize = 0;	/* The folios which were snipped off */
pub static mut l_active: usize = 0;
pub static mut l_inactive: usize = 0;
    let mut nr_deactivate = 0;
    let mut nr_activate = 0;
pub static mut nr_rotated: unsigned = 0;
pub static mut file: bool = false;
    let mut pgdat = lruvec_pgdat(lruvec);
    lru_add_drain();
    lruvec_lock_irq(lruvec);
    nr_taken = isolate_lru_folios(nr_to_scan, lruvec, &l_hold,
    &nr_scanned, sc, lru);
    __mod_node_page_state(pgdat, NR_ISOLATED_ANON + file, nr_taken);
    mod_lruvec_state(lruvec, PGREFILL, nr_scanned);
    lruvec_unlock_irq(lruvec);
    while (!list_empty(&l_hold)) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    cond_resched();
    folio = lru_to_folio(&l_hold);
    list_del(&folio.lru);
    if (unlikely(!folio_evictable(folio))) {
    folio_putback_lru(folio);
    continue;
    }
    if (unlikely(buffer_heads_over_limit)) {
    if (folio_needs_release(folio) &&
    folio_trylock(folio)) {
    filemap_release_folio(folio, 0);
    folio_unlock(folio);
    }
    }
// Referenced or rmap lock contention: rotate
    if (folio_referenced(folio, 0, sc.target_mem_cgroup,
    &vma_flags) != 0) {
//
// Identify referenced, file-backed active folios and
// give them one more trip around the active list. So
// that executable code get better chances to stay in
// memory under moderate memory pressure.  Anon folios
// are not likely to be evicted by use-once streaming
// IO, plus JVM can create lots of anon VM_EXEC folios,
// so we ignore them here.
//
    if (is_exec_file_folio(folio, &vma_flags)) {
    nr_rotated += folio_nr_pages(folio);
    list_add(&folio.lru, &l_active);
    continue;
    }
    }
    folio_clear_active(folio);	/* we are de-activating */
    folio_set_workingset(folio);
    list_add(&folio.lru, &l_inactive);
    }
//
// Move folios back to the lru list.
//
    nr_activate = move_folios_to_lru(&l_active);
    nr_deactivate = move_folios_to_lru(&l_inactive);
    count_vm_events(PGDEACTIVATE, nr_deactivate);
    count_memcg_events(lruvec_memcg(lruvec), PGDEACTIVATE, nr_deactivate);
    mod_node_page_state(pgdat, NR_ISOLATED_ANON + file, -nr_taken);
    if (nr_rotated) {
    mod_lruvec_state(lruvec, PGROTATE_ANON + file, nr_rotated);
    }
    trace_mm_vmscan_lru_shrink_active(pgdat.node_id, nr_taken, nr_activate,
    nr_deactivate, nr_rotated, sc.priority, file);
    }
#[no_mangle]
pub unsafe extern "C" fn reclaim_folio_list(folio_list: *mut list_head, pgdat: *mut pglist_data) -> c_uint {
pub static mut stat: usize = 0;
    let mut nr_reclaimed = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut scan_control: usize = 0;
    nr_reclaimed = shrink_folio_list(folio_list, pgdat, &sc, &stat, true, core::ptr::null_mut());
    while (!list_empty(folio_list)) {
    folio = lru_to_folio(folio_list);
    list_del(&folio.lru);
    folio_putback_lru(folio);
    }
    trace_mm_vmscan_reclaim_pages(pgdat.node_id, sc.nr_scanned, nr_reclaimed, &stat);
    return nr_reclaimed;
    }
#[no_mangle]
pub unsafe extern "C" fn reclaim_pages(folio_list: *mut list_head) -> c_ulong {
    let mut nid = 0;
pub static mut nr_reclaimed: c_uint = 0;
pub static mut node_folio_list: usize = 0;
    let mut noreclaim_flag = 0;
    if (list_empty(folio_list)) {
    return nr_reclaimed;
    }
    noreclaim_flag = memalloc_noreclaim_save();
    nid = folio_nid(lru_to_folio(folio_list));
    do {
    let mut folio = lru_to_folio(folio_list);
    if (nid == folio_nid(folio)) {
    folio_clear_active(folio);
    list_move(&folio.lru, &node_folio_list);
    continue;
    }
    nr_reclaimed += reclaim_folio_list(&node_folio_list, NODE_DATA(nid));
    nid = folio_nid(lru_to_folio(folio_list));
    } while (!list_empty(folio_list));
    nr_reclaimed += reclaim_folio_list(&node_folio_list, NODE_DATA(nid));
    memalloc_noreclaim_restore(noreclaim_flag);
    return nr_reclaimed;
    }
#[no_mangle]
pub unsafe extern "C" fn shrink_list(lru: lru_list, nr_to_scan: c_ulong, lruvec: *mut lruvec, sc: *mut scan_control) -> c_ulong {
    if (is_active_lru(lru)) {
    if (sc.may_deactivate & (1 << is_file_lru(lru))) {
    shrink_active_list(nr_to_scan, lruvec, sc, lru);
    }
    else {
    sc.skipped_deactivate = 1;
    }
    return 0;
    }
    return shrink_inactive_list(nr_to_scan, lruvec, sc, lru);
    }
//
// The inactive anon list should be small enough that the VM never has
// to do too much work.
//
// The inactive file list should be small enough to leave most memory
// to the established workingset on the scan-resistant active list,
// but large enough to avoid thrashing the aggregate readahead window.
//
// Both inactive lists should also be large enough that each inactive
// folio has a chance to be referenced again before it is reclaimed.
//
// If that fails and refaulting is observed, the inactive list grows.
//
// The inactive_ratio is the target ratio of ACTIVE to INACTIVE folios
// on this LRU, maintained by the pageout code. An inactive_ratio
// of 3 means 3:1 or 25% of the folios are kept on the inactive list.
//
// total     target    max
// memory    ratio     inactive
// -------------------------------------
// 10MB       1         5MB
// 100MB       1        50MB
// 1GB       3       250MB
// 10GB      10       0.9GB
// 100GB      31         3GB
// 1TB     101        10GB
// 10TB     320        32GB
//
#[no_mangle]
unsafe extern "C" fn inactive_is_low(lruvec: *mut lruvec, inactive_lru: lru_list) -> bool {
pub static mut active_lru: lru_list = 0;
    unsigned long inactive, active;
    let mut inactive_ratio = 0;
    let mut gb = 0;
    inactive = lruvec_page_state(lruvec, NR_LRU_BASE + inactive_lru);
    active = lruvec_page_state(lruvec, NR_LRU_BASE + active_lru);
    gb = (inactive + active) >> (30 - PAGE_SHIFT);
    if (gb) {
    inactive_ratio = int_sqrt(10 * gb);
    }
    else {
    inactive_ratio = 1;
    }
    return inactive * inactive_ratio < active;
    }
    enum scan_balance {
    SCAN_EQUAL,
    SCAN_FRACT,
    SCAN_ANON,
    SCAN_FILE,
    };
#[no_mangle]
unsafe extern "C" fn prepare_scan_control(pgdat: *mut pg_data_t, sc: *mut scan_control) {
    let mut anon_cost = core::ptr::null_mut();
    let mut file_cost = core::ptr::null_mut();
pub static mut target_lruvec: *mut c_void = core::ptr::null_mut();
    let mut lrusize = 0;
    let mut file = 0;
    if (lru_gen_enabled() && !lru_gen_switching()) {
    return;
    }
    target_lruvec = mem_cgroup_lruvec(sc.target_mem_cgroup, pgdat);
//
// Flush the memory cgroup stats in rate-limited way as we don't need
// most accurate stats here. We may switch to regular stats flushing
// in the future once it is cheap enough.
//
    mem_cgroup_flush_stats_ratelimited(sc.target_mem_cgroup);
//
// Determine the scan balance between anon and file LRUs.
//
// The cost model is based on rotations, refaults and
// reclaim-driven writes (anon only) on each side.
//
// These event counters are monotonic, so each reclaim cycle
// the delta since the last scan is extracted and incorporated
// into a decaying average. This ensures currency, as workloads
// change over time, and avoids overflow in the calculations.
//
// Use lruvec_page_state_monotonic() so unsigned subtraction
// yields the correct delta across a signed-long wraparound of
// the underlying counter (a real hazard on 32-bit that the
// clamp in lruvec_page_state() would otherwise turn into a huge
// spurious delta).
//
    spin_lock(&target_lruvec.cost_lock);
    while (f <= 1) {
    let mut cost = &target_lruvec.cost[f];
    unsigned long rotated, io, nr_rotated, nr_io;
    rotated = lruvec_page_state_monotonic(target_lruvec,
    PGROTATE_ANON + f);
    io = lruvec_page_state_monotonic(target_lruvec,
    WORKINGSET_RESTORE_BASE + f);
    if (f == WORKINGSET_ANON) {
    io += lruvec_page_state_monotonic(target_lruvec,
    NR_VMSCAN_WRITE);
    }
    nr_rotated = rotated - cost.last_rotated;
    nr_io = io - cost.last_io;
//
// Reflect the relative cost of incurring IO and spending
// CPU time on rotations. This doesn't attempt to make a
// precise comparison, it just says: if reloads are about
// comparable between the LRU lists, or rotations are
// overwhelmingly different between them, adjust scan
// balance for CPU work.
//
    cost.count += nr_io * SWAP_CLUSTER_MAX + nr_rotated;
    cost.last_rotated = rotated;
    cost.last_io = io;
    }
    anon_cost = &target_lruvec.cost[WORKINGSET_ANON];
    file_cost = &target_lruvec.cost[WORKINGSET_FILE];
    lrusize = lruvec_page_state(target_lruvec, NR_INACTIVE_ANON) +
    lruvec_page_state(target_lruvec, NR_ACTIVE_ANON) +
    lruvec_page_state(target_lruvec, NR_INACTIVE_FILE) +
    lruvec_page_state(target_lruvec, NR_ACTIVE_FILE);
    while (anon_cost.count + file_cost.count > lrusize / 4) {
    anon_cost.count /= 2;
    file_cost.count /= 2;
    }
    sc.anon_cost = anon_cost.count;
    sc.file_cost = file_cost.count;
    spin_unlock(&target_lruvec.cost_lock);
//
// Target desirable inactive:active list ratios for the anon
// and file LRU lists.
//
    if (!sc.force_deactivate) {
    let mut refaults = 0;
//
// When refaults are being observed, it means a new
// workingset is being established. Deactivate to get
// rid of any stale active pages quickly.
//
    refaults = lruvec_page_state(target_lruvec,
    WORKINGSET_ACTIVATE_ANON);
    if (refaults != target_lruvec.refaults[WORKINGSET_ANON] ||
    inactive_is_low(target_lruvec, LRU_INACTIVE_ANON)) {
    sc.may_deactivate |= DEACTIVATE_ANON;
    }
    else {
    sc.may_deactivate &= ~DEACTIVATE_ANON;
    }
    refaults = lruvec_page_state(target_lruvec,
    WORKINGSET_ACTIVATE_FILE);
    if (refaults != target_lruvec.refaults[WORKINGSET_FILE] ||
    inactive_is_low(target_lruvec, LRU_INACTIVE_FILE)) {
    sc.may_deactivate |= DEACTIVATE_FILE;
    }
    else {
    sc.may_deactivate &= ~DEACTIVATE_FILE;
    }
    } else {
    sc.may_deactivate = DEACTIVATE_ANON | DEACTIVATE_FILE;
    }
//
// If we have plenty of inactive file pages that aren't
// thrashing, try to reclaim those first before touching
// anonymous pages.
//
    file = lruvec_page_state(target_lruvec, NR_INACTIVE_FILE);
    if (file >> sc.priority && !(sc.may_deactivate & DEACTIVATE_FILE) &&
    !sc.no_cache_trim_mode) {
    sc.cache_trim_mode = 1;
    }
    else {
    sc.cache_trim_mode = 0;
    }
//
// Prevent the reclaimer from falling into the cache trap: as
// cache pages start out inactive, every cache fault will tip
// the scan balance towards the file LRU.  And as the file LRU
// shrinks, so does the window for rotation from references.
// This means we have a runaway feedback loop where a tiny
// thrashing file LRU becomes infinitely more attractive than
// anon pages.  Try to detect this based on file LRU size.
//
    if (!cgroup_reclaim(sc)) {
pub static mut total_high_wmark: c_ulong = 0;
    unsigned long free, anon;
    let mut z = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    free = sum_zone_node_page_state(pgdat.node_id, NR_FREE_PAGES);
    file = node_page_state(pgdat, NR_ACTIVE_FILE) +
    node_page_state(pgdat, NR_INACTIVE_FILE);
    for_each_managed_zone_pgdat(zone, pgdat, z, MAX_NR_ZONES - 1) {
    total_high_wmark += high_wmark_pages(zone);
    }
//
// Consider anon: if that's low too, this isn't a
// runaway file reclaim problem, but rather just
// extreme pressure. Reclaim as per usual then.
//
    anon = node_page_state(pgdat, NR_INACTIVE_ANON);
    sc.file_is_tiny =
    file + free <= total_high_wmark &&
    !(sc.may_deactivate & DEACTIVATE_ANON) &&
    anon >> sc.priority;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn calculate_pressure_balance(sc: *mut scan_control, swappiness: c_int, fraction: *mut u64, denominator: *mut u64) {
    unsigned long anon_cost, file_cost, total_cost;
    unsigned long ap, fp;
//
// Calculate the pressure balance between anon and file pages.
//
// The amount of pressure we put on each LRU is inversely
// proportional to the cost of reclaiming each list, as
// determined by the share of pages that are refaulting, times
// the relative IO cost of bringing back a swapped out
// anonymous page vs reloading a filesystem page (swappiness).
//
// Although we limit that influence to ensure no list gets
// left behind completely: at least a third of the pressure is
// applied, before swappiness.
//
// With swappiness at 100, anon and file have equal IO cost.
//
    total_cost = sc.anon_cost + sc.file_cost;
    anon_cost = total_cost + sc.anon_cost;
    file_cost = total_cost + sc.file_cost;
    total_cost = anon_cost + file_cost;
    ap = swappiness * (total_cost + 1);
    ap /= anon_cost + 1;
    fp = (MAX_SWAPPINESS - swappiness) * (total_cost + 1);
    fp /= file_cost + 1;
    fraction[WORKINGSET_ANON] = ap;
    fraction[WORKINGSET_FILE] = fp;
// denominator = ap + fp;
    }
#[no_mangle]
pub unsafe extern "C" fn apply_proportional_protection(memcg: *mut mem_cgroup, sc: *mut scan_control, scan: c_ulong) -> c_ulong {
    unsigned long min, low, usage;
    mem_cgroup_protection(sc.target_mem_cgroup, memcg, &min, &low, &usage);
    if (min || low) {
//
// Scale a cgroup's reclaim pressure by proportioning
// its current usage to its memory.low or memory.min
// setting.
//
// This is important, as otherwise scanning aggression
// becomes extremely binary -- from nothing as we
// approach the memory protection threshold, to totally
// nominal as we exceed it.  This results in requiring
// setting extremely liberal protection thresholds. It
// also means we simply get no protection at all if we
// set it too low, which is not ideal.
//
// If there is any protection in place, we reduce scan
// pressure by how much of the total memory used is
// within protection thresholds.
//
// There is one special case: in the first reclaim pass,
// we skip over all groups that are within their low
// protection. If that fails to reclaim enough pages to
// satisfy the reclaim goal, we come back and override
// the best-effort low protection. However, we still
// ideally want to honor how well-behaved groups are in
// that case instead of simply punishing them all
// equally. As such, we reclaim them based on how much
// memory they are using, reducing the scan pressure
// again by how much of the total memory used is under
// hard protection.
//
    let mut protection = 0;
// memory.low scaling, make sure we retry before OOM
    if (!sc.memcg_low_reclaim && low > min) {
    protection = low;
    sc.memcg_low_skipped = 1;
    } else {
    protection = min;
    }
// Avoid TOCTOU with earlier protection check
    usage = max(usage, protection);
    scan -= scan * protection / (usage + 1);
//
// Minimally target SWAP_CLUSTER_MAX pages to keep
// reclaim moving forwards, avoiding decrementing
// sc->priority further than desirable.
//
    scan = max(scan, SWAP_CLUSTER_MAX);
    }
    return scan;
    }
//
// Determine how aggressively the anon and file LRU lists should be
// scanned.
//
// nr[0] = anon inactive folios to scan; nr[1] = anon active folios to scan
// nr[2] = file inactive folios to scan; nr[3] = file active folios to scan
//
#[no_mangle]
pub unsafe extern "C" fn get_scan_count(lruvec: *mut lruvec, sc: *mut scan_control, nr: *mut c_ulong) {
    let mut pgdat = lruvec_pgdat(lruvec);
    let mut memcg = lruvec_memcg(lruvec);
pub static mut swappiness: c_int = 0;
    u64 fraction[ANON_AND_FILE];
    u64 denominator = 0;	/* gcc */
    enum scan_balance scan_balance;
    enum lru_list lru;
//
// Proactive reclaim initiated by userspace for anonymous memory only.
// SWAPPINESS_ANON_ONLY is set only on the proactive reclaim path, so
// warn if it shows up elsewhere. When anon cannot be reclaimed (e.g.
// no swap), bail out instead of falling back to evicting file pages,
// which would violate the anon-only semantics.
//
    if (swappiness == SWAPPINESS_ANON_ONLY) {
    WARN_ON_ONCE!(!sc.proactive);
    if (!can_reclaim_anon_pages(memcg, pgdat.node_id, sc)) {
    memset(nr, 0, sizeof!(*nr) * NR_LRU_LISTS);
    return;
    }
    scan_balance = SCAN_ANON;
// goto;
    }
// If we have no swap space, do not bother scanning anon folios.
    if (!sc.may_swap || !can_reclaim_anon_pages(memcg, pgdat.node_id, sc)) {
    scan_balance = SCAN_FILE;
// goto;
    }
//
// Global reclaim will swap to prevent OOM even with no
// swappiness, but memcg users want to use this knob to
// disable swapping for individual groups completely when
// using the memory controller's swap limit feature would be
// too expensive.
//
    if (cgroup_reclaim(sc) && !swappiness) {
    scan_balance = SCAN_FILE;
// goto;
    }
//
// Do not apply any pressure balancing cleverness when the
// system is close to OOM, scan both anon and file equally
// (unless the swappiness setting disagrees with swapping).
//
    if (!sc.priority && swappiness) {
    scan_balance = SCAN_EQUAL;
// goto;
    }
//
// If the system is almost out of file pages, force-scan anon.
//
    if (sc.file_is_tiny) {
    scan_balance = SCAN_ANON;
// goto;
    }
//
// If there is enough inactive page cache, we do not reclaim
// anything from the anonymous working right now to make sure
// a streaming file access pattern doesn't cause swapping.
//
    if (sc.cache_trim_mode) {
    scan_balance = SCAN_FILE;
// goto;
    }
    scan_balance = SCAN_FRACT;
    calculate_pressure_balance(sc, swappiness, fraction, &denominator);
// label;
    for_each_evictable_lru(lru) {
pub static mut file: bool = false;
    let mut lruvec_size = 0;
    let mut scan = 0;
    lruvec_size = lruvec_lru_size(lruvec, lru, sc.reclaim_idx);
    scan = apply_proportional_protection(memcg, sc, lruvec_size);
    scan >>= sc.priority;
//
// If the cgroup's already been deleted, make sure to
// scrape out the remaining cache.
//
    if (!scan && !mem_cgroup_online(memcg)) {
    scan = min(lruvec_size, SWAP_CLUSTER_MAX);
    }
    match (scan_balance) {
    SCAN_EQUAL => {
// Scan lists relative to size
    // break;
    }
    SCAN_FRACT => {
//
// Scan types proportional to swappiness and
// their relative recent reclaim efficiency.
// Make sure we don't miss the last page on
// the offlined memory cgroups because of a
// round-off error.
//
    scan = mem_cgroup_online(memcg) ?
    div64_u64(scan * fraction[file], denominator) :
    DIV64_U64_ROUND_UP(scan * fraction[file],
    denominator);
    // break;
    }
    SCAN_FILE => {
    }
    SCAN_ANON => {
// Scan one type exclusively
    if ((scan_balance == SCAN_FILE) != file) {
    scan = 0;
    }
    // break;
    }
    _ => {
// Look ma, no brain
    BUG();
    }
    }
    nr[lru] = scan;
    }
    }
//
// Anonymous LRU management is a waste if there is
// ultimately no way to reclaim the memory.
//
#[no_mangle]
pub unsafe extern "C" fn can_age_anon_pages(lruvec: *mut lruvec, sc: *mut scan_control) -> bool {
// Aging the anon LRU is valuable if swap is present:
    if (total_swap_pages > 0) {
    return true;
    }
// Also valuable if anon pages can be demoted:
    return can_demote(lruvec_pgdat(lruvec).node_id, sc,
    lruvec_memcg(lruvec));
    }

pub static mut lru_switch: usize = 0;

pub static mut lru_gen_caps: usize = 0;

pub static mut lru_gen_caps: usize = 0;

#[no_mangle]
unsafe extern "C" fn should_walk_mmu() -> bool {
    return arch_has_hw_pte_young() && get_cap(LRU_GEN_MM_WALK);
    }
#[no_mangle]
unsafe extern "C" fn should_clear_pmd_young() -> bool {
    return arch_has_hw_nonleaf_pmd_young() && get_cap(LRU_GEN_NONLEAF_YOUNG);
    }
//
// shorthand helpers
//

    let mut max_seq = READ_ONCE((lruvec).lrugen.max_seq)

    unsigned long min_seq[ANON_AND_FILE] = {			
    READ_ONCE((lruvec).lrugen.min_seq[LRU_GEN_ANON]),	
    READ_ONCE((lruvec).lrugen.min_seq[LRU_GEN_FILE]),	
    }
// Get the min/max evictable type based on swappiness

    min((min_seq)[min_type(swappiness)], (min_seq)[max_type(swappiness)])

    for ((gen) = 0; (gen) < MAX_NR_GENS; (gen)++)			 {
    for ((type) = 0; (type) < ANON_AND_FILE; (type)++)	
    }
    for ((zone) = 0; (zone) < MAX_NR_ZONES; (zone)++) {

    for ((type) = min_type(swappiness); (type) <= max_type(swappiness); (type)++)
    }

#[no_mangle]
pub unsafe extern "C" fn get_lruvec(memcg: *mut mem_cgroup, nid: c_int) -> *mut c_void {
    let mut pgdat = NODE_DATA(nid);

    if (memcg) {
    let mut lruvec = &memcg.nodeinfo[nid].lruvec;
// see the comment in mem_cgroup_lruvec()
    if (!lruvec.pgdat) {
    lruvec.pgdat = pgdat;
    }
    return lruvec;
    }

    VM_WARN_ON_ONCE(!mem_cgroup_disabled());
    return &pgdat.__lruvec;
    }
#[no_mangle]
unsafe extern "C" fn get_swappiness(lruvec: *mut lruvec, sc: *mut scan_control) -> c_int {
    let mut memcg = lruvec_memcg(lruvec);
    let mut pgdat = lruvec_pgdat(lruvec);
pub static mut swappiness: c_int = 0;
    if (swappiness == SWAPPINESS_ANON_ONLY) {
    return swappiness;
    }
    if (!sc.may_swap) {
    return 0;
    }
    if (!can_demote(pgdat.node_id, sc, memcg) &&
    mem_cgroup_get_nr_swap_pages(memcg) < MIN_LRU_BATCH) {
    return 0;
    }
    return swappiness;
    }
#[no_mangle]
unsafe extern "C" fn get_nr_gens(lruvec: *mut lruvec, type: c_int) -> c_int {
    return lruvec.lrugen.max_seq - lruvec.lrugen.min_seq[type] + 1;
    }
#[no_mangle]
unsafe extern "C" fn seq_is_valid(lruvec: *mut lruvec) -> bool __maybe_unused {
    let mut type = 0;
    while (type < ANON_AND_FILE) {
pub static mut n: c_int = 0;
    if (n < MIN_NR_GENS || n > MAX_NR_GENS) {
    return false;
    }
    }
    return true;
    }
//
// Bloom filters
//
// Bloom filters with m=1<<15, k=2 and the false positive rates of ~1/5 when
// n=10,000 and ~1/2 when n=20,000, where, conventionally, m is the number of
// bits in a bitmap, k is the number of hash functions and n is the number of
// inserted items.
//
// Page table walkers use one of the two filters to reduce their search space.
// To get rid of non-leaf entries that no longer have enough leaf entries, the
// aging uses the double-buffering technique to flip to the other filter each
// time it produces a new generation. For non-leaf entries that have enough
// leaf entries, the aging carries them over to the next generation in
// walk_pmd_range(); the eviction also report them when walking the rmap
// in lru_gen_look_around().
//
// For future optimizations:
// 1. It's not necessary to keep both filters all the time. The spare one can be
// freed after the RCU grace period and reallocated if needed again.
// 2. And when reallocating, it's worth scaling its size according to the number
// of inserted entries in the other filter, to reduce the memory overhead on
// small systems and false positives on large systems.
// 3. Jenkins' hash function is an alternative to Knuth's.
//
pub const BLOOM_FILTER_SHIFT: c_int = 15;
#[no_mangle]
pub unsafe extern "C" fn filter_gen_from_seq(seq: c_ulong) -> c_int {
    return seq % NR_BLOOM_FILTERS;
    }
#[no_mangle]
unsafe extern "C" fn get_item_key(item: *mut c_void, key: *mut c_int) {
pub static mut hash: u32 = 0;
    BUILD_BUG_ON!(BLOOM_FILTER_SHIFT * 2 > BITS_PER_TYPE(u32));
    key[0] = hash & (BIT(BLOOM_FILTER_SHIFT) - 1);
    key[1] = hash >> BLOOM_FILTER_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter(mm_state: *mut lru_gen_mm_state, seq: c_ulong, item: *mut c_void) -> bool {
    int key[2];
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut gen: c_int = 0;
    filter = READ_ONCE(mm_state.filters[gen]);
    if (!filter) {
    return true;
    }
    get_item_key(item, key);
    return test_bit(key[0], filter) && test_bit(key[1], filter);
    }
#[no_mangle]
pub unsafe extern "C" fn update_bloom_filter(mm_state: *mut lru_gen_mm_state, seq: c_ulong, item: *mut c_void) {
    int key[2];
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut gen: c_int = 0;
    filter = READ_ONCE(mm_state.filters[gen]);
    if (!filter) {
    return;
    }
    get_item_key(item, key);
    if (!test_bit(key[0], filter)) {
    set_bit(key[0], filter);
    }
    if (!test_bit(key[1], filter)) {
    set_bit(key[1], filter);
    }
    }
#[no_mangle]
unsafe extern "C" fn reset_bloom_filter(mm_state: *mut lru_gen_mm_state, seq: c_ulong) {
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut gen: c_int = 0;
    filter = mm_state.filters[gen];
    if (filter) {
    bitmap_clear(filter, 0, BIT(BLOOM_FILTER_SHIFT));
    return;
    }
    filter = bitmap_zalloc(BIT(BLOOM_FILTER_SHIFT),
    __GFP_HIGH | __GFP_NOMEMALLOC | __GFP_NOWARN);
    WRITE_ONCE(mm_state.filters[gen], filter);
    }
//
// mm_struct list
//

#[no_mangle]
pub unsafe extern "C" fn get_mm_list(memcg: *mut mem_cgroup) -> *mut c_void {
pub static mut lru_gen_mm_list: usize = 0;

    if (memcg) {
    return &memcg.mm_list;
    }

    VM_WARN_ON_ONCE(!mem_cgroup_disabled());
    return &mm_list;
    }
#[no_mangle]
pub unsafe extern "C" fn get_mm_state(lruvec: *mut lruvec) -> *mut c_void {
    return &lruvec.mm_state;
    }
#[no_mangle]
pub unsafe extern "C" fn get_next_mm(walk: *mut lru_gen_mm_walk) -> *mut c_void {
    let mut key = 0;
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut pgdat = lruvec_pgdat(walk.lruvec);
    let mut mm_state = get_mm_state(walk.lruvec);
    mm = list_entry(mm_state.head, mm_struct, lru_gen.list);
    key = pgdat.node_id % BITS_PER_TYPE(mm.lru_gen.bitmap);
    if (!walk.force_scan && !test_bit(key, &mm.lru_gen.bitmap)) {
    return core::ptr::null_mut();
    }
    clear_bit(key, &mm.lru_gen.bitmap);
    mmgrab(mm);
    return mm;
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_add_mm(mm: *mut mm_struct) {
    let mut nid = 0;
    let mut memcg = get_mem_cgroup_from_mm(mm);
    let mut mm_list = get_mm_list(memcg);
    VM_WARN_ON_ONCE(!list_empty(&mm.lru_gen.list));

    VM_WARN_ON_ONCE(mm.lru_gen.memcg);
    mm.lru_gen.memcg = memcg;

    spin_lock(&mm_list.lock);
    for_each_node_state(nid, N_MEMORY) {
    let mut lruvec = get_lruvec(memcg, nid);
    let mut mm_state = get_mm_state(lruvec);
// the first addition since the last iteration
    if (mm_state.tail == &mm_list.fifo) {
    mm_state.tail = &mm.lru_gen.list;
    }
    }
    list_add_tail(&mm.lru_gen.list, &mm_list.fifo);
    spin_unlock(&mm_list.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_del_mm(mm: *mut mm_struct) {
    let mut nid = 0;
pub static mut mm_list: *mut c_void = core::ptr::null_mut();
    let mut memcg = core::ptr::null_mut();
    if (list_empty(&mm.lru_gen.list)) {
    return;
    }

    memcg = mm.lru_gen.memcg;

    mm_list = get_mm_list(memcg);
    spin_lock(&mm_list.lock);
    for_each_node(nid) {
    let mut lruvec = get_lruvec(memcg, nid);
    let mut mm_state = get_mm_state(lruvec);
// where the current iteration continues after
    if (mm_state.head == &mm.lru_gen.list) {
    mm_state.head = mm_state.head.prev;
    }
// where the last iteration ended before
    if (mm_state.tail == &mm.lru_gen.list) {
    mm_state.tail = mm_state.tail.next;
    }
    }
    list_del_init(&mm.lru_gen.list);
    spin_unlock(&mm_list.lock);

    mem_cgroup_put(mm.lru_gen.memcg);
    mm.lru_gen.memcg = core::ptr::null_mut();

    }

#[no_mangle]
pub unsafe extern "C" fn lru_gen_migrate_mm(mm: *mut mm_struct) {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut task = rcu_dereference_protected(mm.owner, true);
    VM_WARN_ON_ONCE(task.mm != mm);
    lockdep_assert_held(&task.alloc_lock);
// for mm_update_next_owner()
    if (mem_cgroup_disabled()) {
    return;
    }
// migration can happen before addition
    if (!mm.lru_gen.memcg) {
    return;
    }
    rcu_read_lock();
    memcg = mem_cgroup_from_task(task);
    rcu_read_unlock();
    if (memcg == mm.lru_gen.memcg) {
    return;
    }
    VM_WARN_ON_ONCE(list_empty(&mm.lru_gen.list));
    lru_gen_del_mm(mm);
    lru_gen_add_mm(mm);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: get_mm_list
pub unsafe extern "C" fn get_mm_list_dup(memcg: *mut mem_cgroup) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: get_mm_state
pub unsafe extern "C" fn get_mm_state_dup(lruvec: *mut lruvec) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: get_next_mm
pub unsafe extern "C" fn get_next_mm_dup(walk: *mut lru_gen_mm_walk) -> *mut c_void {
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn reset_mm_stats(walk: *mut lru_gen_mm_walk, last: bool) {
    let mut i = 0;
    let mut hist = 0;
    let mut lruvec = walk.lruvec;
    let mut mm_state = get_mm_state(lruvec);
    lockdep_assert_held(&get_mm_list(lruvec_memcg(lruvec)).lock);
    hist = lru_hist_from_seq(walk.seq);
    while (i < NR_MM_STATS) {
    WRITE_ONCE(mm_state.stats[hist][i],
    mm_state.stats[hist][i] + walk.mm_stats[i]);
    walk.mm_stats[i] = 0;
    }
    if (NR_HIST_GENS > 1 && last) {
    hist = lru_hist_from_seq(walk.seq + 1);
    for (i = 0; i < NR_MM_STATS; i++) {
    WRITE_ONCE(mm_state.stats[hist][i], 0);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn iterate_mm_list(walk: *mut lru_gen_mm_walk, iter: *mut mm_struct) -> bool {
pub static mut first: bool = false;
pub static mut last: bool = false;
    let mut mm = core::ptr::null_mut();
    let mut lruvec = walk.lruvec;
    let mut memcg = lruvec_memcg(lruvec);
    let mut mm_list = get_mm_list(memcg);
    let mut mm_state = get_mm_state(lruvec);
//
// mm_state->seq is incremented after each iteration of mm_list. There
// are three interesting cases for this page table walker:
// 1. It tries to start a new iteration with a stale max_seq: there is
// nothing left to do.
// 2. It started the next iteration: it needs to reset the Bloom filter
// so that a fresh set of PTE tables can be recorded.
// 3. It ended the current iteration: it needs to reset the mm stats
// counters and tell its caller to increment max_seq.
//
    spin_lock(&mm_list.lock);
    VM_WARN_ON_ONCE(mm_state.seq + 1 < walk.seq);
    if (walk.seq <= mm_state.seq) {
// goto;
    }
    if (!mm_state.head) {
    mm_state.head = &mm_list.fifo;
    }
    if (mm_state.head == &mm_list.fifo) {
    first = true;
    }
    do {
    mm_state.head = mm_state.head.next;
    if (mm_state.head == &mm_list.fifo) {
    WRITE_ONCE(mm_state.seq, mm_state.seq + 1);
    last = true;
    break;
    }
// force scan for those added after the last iteration
    if (!mm_state.tail || mm_state.tail == mm_state.head) {
    mm_state.tail = mm_state.head.next;
    walk.force_scan = true;
    }
    } while (!(mm = get_next_mm(walk)));
// label;
    if (*iter || last) {
    reset_mm_stats(walk, last);
    }
    spin_unlock(&mm_list.lock);
    if (mm && first) {
    reset_bloom_filter(mm_state, walk.seq + 1);
    }
    if (*iter) {
    mmdrop(*iter);
    }
// iter = mm;
    return last;
    }
#[no_mangle]
unsafe extern "C" fn iterate_mm_list_nowalk(lruvec: *mut lruvec, seq: c_ulong) -> bool {
pub static mut success: bool = false;
    let mut memcg = lruvec_memcg(lruvec);
    let mut mm_list = get_mm_list(memcg);
    let mut mm_state = get_mm_state(lruvec);
    spin_lock(&mm_list.lock);
    VM_WARN_ON_ONCE(mm_state.seq + 1 < seq);
    if (seq > mm_state.seq) {
    mm_state.head = core::ptr::null_mut();
    mm_state.tail = core::ptr::null_mut();
    WRITE_ONCE(mm_state.seq, mm_state.seq + 1);
    success = true;
    }
    spin_unlock(&mm_list.lock);
    return success;
    }
//
// PID controller
//
// A feedback loop based on Proportional-Integral-Derivative (PID) controller.
//
// The P term is refaulted/(evicted+protected) from a tier in the generation
// currently being evicted; the I term is the exponential moving average of the
// P term over the generations previously evicted, using the smoothing factor
// 1/2; the D term isn't supported.
//
// The setpoint (SP) is always the first tier of one type; the process variable
// (PV) is either any tier of the other type or any other tier of the same
// type.
//
// The error is the difference between the SP and the PV; the correction is to
// turn off protection when SP>PV or turn on protection when SP<PV.
//
// For future optimizations:
// 1. The D term may discount the other two terms over time so that long-lived
// generations can resist stale information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_pos {
    pub refaulted: c_ulong,
    pub total: c_ulong,
    pub gain: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn read_ctrl_pos(lruvec: *mut lruvec, type: c_int, tier: c_int, gain: c_int, pos: *mut ctrl_pos) {
    let mut i = 0;
    let mut lrugen = &lruvec.lrugen;
pub static mut hist: c_int = 0;
    pos.gain = gain;
    pos.refaulted = pos.total = 0;
    while (i <= min(tier, MAX_NR_TIERS - 1)) {
    pos.refaulted += lrugen.avg_refaulted[type][i] +
    atomic_long_read(&lrugen.refaulted[hist][type][i]);
    pos.total += lrugen.avg_total[type][i] +
    lrugen.protected[hist][type][i] +
    atomic_long_read(&lrugen.evicted[hist][type][i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn reset_ctrl_pos(lruvec: *mut lruvec, type: c_int, carryover: bool) {
    let mut hist = 0;
    let mut tier = 0;
    let mut lrugen = &lruvec.lrugen;
pub static mut clear: bool = false;
pub static mut seq: c_ulong = 0;
    lockdep_assert_held(&lruvec.lru_lock);
    if (!carryover && !clear) {
    return;
    }
    hist = lru_hist_from_seq(seq);
    while (tier < MAX_NR_TIERS) {
    if (carryover) {
    let mut sum = 0;
    sum = lrugen.avg_refaulted[type][tier] +
    atomic_long_read(&lrugen.refaulted[hist][type][tier]);
    WRITE_ONCE(lrugen.avg_refaulted[type][tier], sum / 2);
    sum = lrugen.avg_total[type][tier] +
    lrugen.protected[hist][type][tier] +
    atomic_long_read(&lrugen.evicted[hist][type][tier]);
    WRITE_ONCE(lrugen.avg_total[type][tier], sum / 2);
    }
    if (clear) {
    atomic_long_set(&lrugen.refaulted[hist][type][tier], 0);
    atomic_long_set(&lrugen.evicted[hist][type][tier], 0);
    WRITE_ONCE(lrugen.protected[hist][type][tier], 0);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn positive_ctrl_err(sp: *mut ctrl_pos, pv: *mut ctrl_pos) -> bool {
//
// Return true if the PV has a limited number of refaults or a lower
// refaulted/total than the SP.
//
    return pv.refaulted < MIN_LRU_BATCH ||
    pv.refaulted * (sp.total + MIN_LRU_BATCH) * sp.gain <=
    (sp.refaulted + 1) * pv.total * pv.gain;
    }
//
// the aging
//
// promote pages accessed through page tables
#[no_mangle]
unsafe extern "C" fn folio_update_gen(folio: *mut folio, gen: c_int, vma_flags: *const vma_flags_t) -> c_int {
    unsigned long new_flags, old_flags = READ_ONCE(folio.flags.f);
    VM_WARN_ON_ONCE(gen >= MAX_NR_GENS);
//
// See the comment on LRU_REFS_FLAGS, and activate file-backed
// executable folios after first usage to avoid typical IO
// thrashing from reclaiming.
//
    if (!folio_test_referenced(folio) && !folio_test_workingset(folio) &&
    !is_exec_file_folio(folio, vma_flags)) {
    set_mask_bits(&folio.flags.f, LRU_REFS_MASK, BIT(PG_referenced));
    return -1;
    }
    do {
// lru_gen_del_folio() has isolated this page?
    if (!(old_flags & LRU_GEN_MASK)) {
    return -1;
    }
    new_flags = old_flags & ~(LRU_GEN_MASK | LRU_REFS_FLAGS);
    new_flags |= ((gen + 1UL) << LRU_GEN_PGOFF) | BIT(PG_workingset);
    } while (!try_cmpxchg(&folio.flags.f, &old_flags, new_flags));
    return ((old_flags & LRU_GEN_MASK) >> LRU_GEN_PGOFF) - 1;
    }
// protect pages accessed multiple times through file descriptors
#[no_mangle]
unsafe extern "C" fn folio_inc_gen(lruvec: *mut lruvec, folio: *mut folio) -> c_int {
pub static mut type: c_int = 0;
    let mut lrugen = &lruvec.lrugen;
    int new_gen, old_gen = lru_gen_from_seq(lrugen.min_seq[type]);
    unsigned long new_flags, old_flags = READ_ONCE(folio.flags.f);
    VM_WARN_ON_ONCE_FOLIO(!(old_flags & LRU_GEN_MASK), folio);
    do {
    new_gen = ((old_flags & LRU_GEN_MASK) >> LRU_GEN_PGOFF) - 1;
// folio_update_gen() has promoted this page?
    if (new_gen >= 0 && new_gen != old_gen) {
    return new_gen;
    }
    new_gen = (old_gen + 1) % MAX_NR_GENS;
    new_flags = old_flags & ~(LRU_GEN_MASK | LRU_REFS_FLAGS);
    new_flags |= (new_gen + 1UL) << LRU_GEN_PGOFF;
    } while (!try_cmpxchg(&folio.flags.f, &old_flags, new_flags));
    lru_gen_update_size(lruvec, folio, old_gen, new_gen);
    return new_gen;
    }
#[no_mangle]
pub unsafe extern "C" fn update_batch_size(walk: *mut lru_gen_mm_walk, folio: *mut folio, old_gen: c_int, new_gen: c_int) {
pub static mut type: c_int = 0;
pub static mut zone: c_int = 0;
pub static mut delta: c_int = 0;
    VM_WARN_ON_ONCE(old_gen >= MAX_NR_GENS);
    VM_WARN_ON_ONCE(new_gen >= MAX_NR_GENS);
    walk.batched += 1;
    walk.nr_pages[old_gen][type][zone] -= delta;
    walk.nr_pages[new_gen][type][zone] += delta;
    }
#[no_mangle]
unsafe extern "C" fn reset_batch_size(walk: *mut lru_gen_mm_walk) {
    let mut gen = 0;
    let mut type = 0;
    let mut zone = 0;
    let mut lruvec = lruvec_live_lock_irq(walk.lruvec);
    let mut lrugen = &lruvec.lrugen;
    walk.batched = 0;
    for_each_gen_type_zone(gen, type, zone) {
pub static mut lru: lru_list = 0;
pub static mut delta: c_int = 0;
    if (!delta) {
    continue;
    }
    walk.nr_pages[gen][type][zone] = 0;
    WRITE_ONCE(lrugen.nr_pages[gen][type][zone],
    lrugen.nr_pages[gen][type][zone] + delta);
    if (lru_gen_is_active(lruvec, gen)) {
    lru += LRU_ACTIVE;
    }
    __update_lru_size(lruvec, lru, zone, delta);
    }
    lruvec_unlock_irq(lruvec);
    }
#[no_mangle]
unsafe extern "C" fn should_skip_vma(start: c_ulong, end: c_ulong, args: *mut mm_walk) -> c_int {
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    let mut vma = args.vma;
    let mut walk = args.private;
    if (!vma_is_accessible(vma)) {
    return true;
    }
    if (is_vm_hugetlb_page(vma)) {
    return true;
    }
    if (!vma_has_recency(vma)) {
    return true;
    }
    if (vma.vm_flags & (VM_LOCKED | VM_SPECIAL)) {
    return true;
    }
    if (vma == get_gate_vma(vma.vm_mm)) {
    return true;
    }
    if (vma_is_anonymous(vma)) {
    return !walk.swappiness;
    }
    if (WARN_ON_ONCE!(!vma.vm_file || !vma.vm_file.f_mapping)) {
    return true;
    }
    mapping = vma.vm_file.f_mapping;
    if (mapping_unevictable(mapping)) {
    return true;
    }
    if (shmem_mapping(mapping)) {
    return !walk.swappiness;
    }
    if (walk.swappiness > MAX_SWAPPINESS) {
    return true;
    }
// to exclude special mappings like dax, etc.
    return !mapping.a_ops.read_folio;
    }
//
// Some userspace memory allocators map many single-page VMAs. Instead of
// returning back to the PGD table for each of such VMAs, finish an entire PMD
// table to reduce zigzags and improve cache performance.
//
#[no_mangle]
pub unsafe extern "C" fn get_next_vma(mask: c_ulong, size: c_ulong, args: *mut mm_walk, vm_start: *mut c_ulong, vm_end: *mut c_ulong) -> bool {
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
    VMA_ITERATOR(vmi, args.mm, start);
    VM_WARN_ON_ONCE(mask & size);
    VM_WARN_ON_ONCE((start & mask) != (*vm_start & mask));
    for_each_vma(vmi, args.vma) {
    if (end && end <= args.vma.vm_start) {
    return false;
    }
    if (should_skip_vma(args.vma.vm_start, args.vma.vm_end, args)) {
    continue;
    }
// vm_start = max(start, args->vma->vm_start);
// vm_end = min(end - 1, args->vma->vm_end - 1) + 1;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn get_pte_pfn(pte: pte_t, vma: *mut vm_area_struct, addr: c_ulong, pgdat: *mut pglist_data) -> c_ulong {
pub static mut pfn: c_ulong = 0;
    VM_WARN_ON_ONCE(addr < vma.vm_start || addr >= vma.vm_end);
    if (!pte_present(pte) || is_zero_pfn(pfn)) {
    return -1;
    }
    if (WARN_ON_ONCE!(pte_special(pte))) {
    return -1;
    }
    if (!pte_young(pte) && !mm_has_notifiers(vma.vm_mm)) {
    return -1;
    }
    if (WARN_ON_ONCE!(!pfn_valid(pfn))) {
    return -1;
    }
    if (pfn < pgdat.node_start_pfn || pfn >= pgdat_end_pfn(pgdat)) {
    return -1;
    }
    return pfn;
    }
#[no_mangle]
pub unsafe extern "C" fn get_pmd_pfn(pmd: pmd_t, vma: *mut vm_area_struct, addr: c_ulong, pgdat: *mut pglist_data) -> c_ulong {
pub static mut pfn: c_ulong = 0;
    VM_WARN_ON_ONCE(addr < vma.vm_start || addr >= vma.vm_end);
    if (!pmd_present(pmd) || is_huge_zero_pmd(pmd)) {
    return -1;
    }
    if (!pmd_young(pmd) && !mm_has_notifiers(vma.vm_mm)) {
    return -1;
    }
    if (WARN_ON_ONCE!(!pfn_valid(pfn))) {
    return -1;
    }
    if (pfn < pgdat.node_start_pfn || pfn >= pgdat_end_pfn(pgdat)) {
    return -1;
    }
    return pfn;
    }
#[no_mangle]
pub unsafe extern "C" fn get_pfn_folio(pfn: c_ulong, memcg: *mut mem_cgroup, pgdat: *mut pglist_data) -> *mut c_void {
    let mut folio = pfn_folio(pfn);
    if (folio_lru_gen(folio) < 0) {
    return core::ptr::null_mut();
    }
    if (folio_nid(folio) != pgdat.node_id) {
    return core::ptr::null_mut();
    }
    rcu_read_lock();
    if (folio_memcg(folio) != memcg) {
    folio = core::ptr::null_mut();
    }
    rcu_read_unlock();
    return folio;
    }
#[no_mangle]
unsafe extern "C" fn suitable_to_scan(total: c_int, young: c_int) -> bool {
pub static mut n: c_int = 0;
// suitable if the average number of young PTEs per cacheline is >=1
    return young * n >= total;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_update_folio(walk: *mut lru_gen_mm_walk, vma: *mut vm_area_struct, folio: *mut folio, new_gen: c_int, dirty: bool) {
    let mut old_gen = 0;
    if (!folio) {
    return;
    }
    if (dirty && !folio_test_dirty(folio) &&
    !(folio_test_anon(folio) && folio_test_swapbacked(folio) &&
    !folio_test_swapcache(folio))) {
    folio_mark_dirty(folio);
    }
    if (walk) {
    old_gen = folio_update_gen(folio, new_gen, &vma.flags);
    if (old_gen >= 0 && old_gen != new_gen) {
    update_batch_size(walk, folio, old_gen, new_gen);
    }
    } else if (lru_gen_set_refs(folio, &vma.flags)) {
    old_gen = folio_lru_gen(folio);
    if (old_gen >= 0 && old_gen != new_gen) {
    folio_activate(folio);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pte_range(pmd: *mut pmd_t, start: c_ulong, end: c_ulong, args: *mut mm_walk) -> bool {
    let mut i = 0;
    let mut dirty = 0;
pub static mut pte: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
pub static mut total: c_int = 0;
pub static mut young: c_int = 0;
    let mut last = core::ptr::null_mut();
    let mut walk = args.private;
    let mut memcg = lruvec_memcg(walk.lruvec);
    let mut pgdat = lruvec_pgdat(walk.lruvec);
pub static mut walk.lruvec: usize = 0;
pub static mut gen: c_int = 0;
    let mut nr = 0;
    let mut pmdval;
    pte = pte_offset_map_rw_nolock(args.mm, pmd, start & PMD_MASK, &pmdval, &ptl);
    if (!pte) {
    return false;
    }
    if (!spin_trylock(ptl)) {
    pte_unmap(pte);
    return true;
    }
    if (unlikely(!pmd_same(pmdval, pmdp_get_lockless(pmd)))) {
    pte_unmap_unlock(pte, ptl);
    return false;
    }
    lazy_mmu_mode_enable();
// label;
    while (addr != end) {
    let mut pfn = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut cur_pte = pte + i;
pub static mut ptent: pte_t = 0;
    nr = 1;
    total += 1;
    walk.mm_stats[MM_LEAF_TOTAL]++;
    pfn = get_pte_pfn(ptent, args.vma, addr, pgdat);
    if (pfn == -1) {
    continue;
    }
    folio = get_pfn_folio(pfn, memcg, pgdat);
    if (!folio) {
    continue;
    }
    if (folio_test_large(folio)) {
pub static mut max_nr: c_uint = 0;
    nr = folio_pte_batch_flags(folio, core::ptr::null_mut(), cur_pte, &ptent,
    max_nr, FPB_MERGE_YOUNG_DIRTY);
    total += nr - 1;
    walk.mm_stats[MM_LEAF_TOTAL] += nr - 1;
    }
    if (!test_and_clear_young_ptes_notify(args.vma, addr, cur_pte, nr)) {
    continue;
    }
    if (last != folio) {
    walk_update_folio(walk, args.vma, last, gen, dirty);
    last = folio;
    dirty = false;
    }
    if (pte_dirty(ptent)) {
    dirty = true;
    }
    young += nr;
    walk.mm_stats[MM_LEAF_YOUNG] += nr;
    }
    walk_update_folio(walk, args.vma, last, gen, dirty);
    last = core::ptr::null_mut();
    if (i < PTRS_PER_PTE && get_next_vma(PMD_MASK, PAGE_SIZE, args, &start, &end)) {
// goto;
    }
    lazy_mmu_mode_disable();
    pte_unmap_unlock(pte, ptl);
    return suitable_to_scan(total, young);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pmd_range_locked(pud: *mut pud_t, addr: c_ulong, vma: *mut vm_area_struct, args: *mut mm_walk, bitmap: *mut c_ulong, first: *mut c_ulong) {
    let mut i = 0;
    let mut dirty = 0;
pub static mut pmd: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut last = core::ptr::null_mut();
    let mut walk = args.private;
    let mut memcg = lruvec_memcg(walk.lruvec);
    let mut pgdat = lruvec_pgdat(walk.lruvec);
pub static mut walk.lruvec: usize = 0;
pub static mut gen: c_int = 0;
    VM_WARN_ON_ONCE(pud_leaf(*pud));
// try to batch at most 1+MIN_LRU_BATCH+1 entries
    if (*first == -1) {
// first = addr;
    bitmap_zero(bitmap, MIN_LRU_BATCH);
    return;
    }
    i = addr == -1 ? 0 : pmd_index(addr) - pmd_index(*first);
    if (i && i <= MIN_LRU_BATCH) {
    __set_bit(i - 1, bitmap);
    return;
    }
    pmd = pmd_offset(pud, *first);
    ptl = pmd_lockptr(args.mm, pmd);
    if (!spin_trylock(ptl)) {
// goto;
    }
    lazy_mmu_mode_enable();
    do {
    let mut pfn = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
// don't round down the first address
    addr = i ? (*first & PMD_MASK) + i * PMD_SIZE : *first;
    if (!pmd_present(pmd[i])) {
// goto;
    }
    if (!pmd_trans_huge(pmd[i])) {
    if (!walk.force_scan && should_clear_pmd_young() &&
    !mm_has_notifiers(args.mm)) {
    pmdp_test_and_clear_young(vma, addr, pmd + i);
    }
// goto;
    }
    pfn = get_pmd_pfn(pmd[i], vma, addr, pgdat);
    if (pfn == -1) {
// goto;
    }
    folio = get_pfn_folio(pfn, memcg, pgdat);
    if (!folio) {
// goto;
    }
    if (!pmdp_test_and_clear_young_notify(vma, addr, pmd + i)) {
// goto;
    }
    if (last != folio) {
    walk_update_folio(walk, vma, last, gen, dirty);
    last = folio;
    dirty = false;
    }
    if (pmd_dirty(pmd[i])) {
    dirty = true;
    }
    walk.mm_stats[MM_LEAF_YOUNG]++;
// label;
    i = i > MIN_LRU_BATCH ? 0 : find_next_bit(bitmap, MIN_LRU_BATCH, i) + 1;
    } while (i <= MIN_LRU_BATCH);
    walk_update_folio(walk, vma, last, gen, dirty);
    lazy_mmu_mode_disable();
    spin_unlock(ptl);
// label;
// first = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pmd_range(pud: *mut pud_t, start: c_ulong, end: c_ulong, args: *mut mm_walk) {
    let mut i = 0;
pub static mut pmd: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
    let mut addr = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut bitmap: usize = 0;
pub static mut first: c_ulong = 0;
    let mut walk = args.private;
    let mut mm_state = get_mm_state(walk.lruvec);
    VM_WARN_ON_ONCE(pud_leaf(*pud));
//
// Finish an entire PMD in two passes: the first only reaches to PTE
// tables to avoid taking the PMD lock; the second, if necessary, takes
// the PMD lock to clear the accessed bit in PMD entries.
//
    pmd = pmd_offset(pud, start & PUD_MASK);
// label;
// walk_pte_range() may call get_next_vma()
    vma = args.vma;
    while (addr != end) {
pub static mut val: pmd_t = 0;
    next = pmd_addr_end(addr, end);
    if (!pmd_present(val) || is_huge_zero_pmd(val)) {
    walk.mm_stats[MM_LEAF_TOTAL]++;
    continue;
    }
    if (pmd_trans_huge(val)) {
    let mut pgdat = lruvec_pgdat(walk.lruvec);
pub static mut pfn: c_ulong = 0;
    walk.mm_stats[MM_LEAF_TOTAL]++;
    if (pfn != -1) {
    walk_pmd_range_locked(pud, addr, vma, args, bitmap, &first);
    }
    continue;
    }
    if (!walk.force_scan && should_clear_pmd_young() &&
    !mm_has_notifiers(args.mm)) {
    if (!pmd_young(val)) {
    continue;
    }
    walk_pmd_range_locked(pud, addr, vma, args, bitmap, &first);
    }
    if (!walk.force_scan && !test_bloom_filter(mm_state, walk.seq, pmd + i)) {
    continue;
    }
    walk.mm_stats[MM_NONLEAF_FOUND]++;
    if (!walk_pte_range(&val, addr, next, args)) {
    continue;
    }
    walk.mm_stats[MM_NONLEAF_ADDED]++;
// carry over to the next generation
    update_bloom_filter(mm_state, walk.seq + 1, pmd + i);
    }
    walk_pmd_range_locked(pud, -1, vma, args, bitmap, &first);
    if (i < PTRS_PER_PMD && get_next_vma(PUD_MASK, PMD_SIZE, args, &start, &end)) {
// goto;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pud_range(p4d: *mut p4d_t, start: c_ulong, end: c_ulong, args: *mut mm_walk) -> c_int {
    let mut i = 0;
pub static mut pud: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    let mut next = 0;
    let mut walk = args.private;
    VM_WARN_ON_ONCE(p4d_leaf(*p4d));
    pud = pud_offset(p4d, start & P4D_MASK);
// label;
    while (addr != end) {
pub static mut val: pud_t = 0;
    next = pud_addr_end(addr, end);
    if (!pud_present(val) || WARN_ON_ONCE!(pud_leaf(val))) {
    continue;
    }
    walk_pmd_range(&val, addr, next, args);
    if (need_resched() || walk.batched >= MAX_LRU_BATCH) {
    end = (addr | ~PUD_MASK) + 1;
// goto;
    }
    }
    if (i < PTRS_PER_PUD && get_next_vma(P4D_MASK, PUD_SIZE, args, &start, &end)) {
// goto;
    }
    end = round_up(end, P4D_SIZE);
// label;
    if (!end || !args.vma) {
    return 1;
    }
    walk.next_addr = max(end, args.vma.vm_start);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn walk_mm(mm: *mut mm_struct, walk: *mut lru_gen_mm_walk) {
pub static mut mm_walk_ops: usize = 0;
    let mut err = 0;
    let mut lruvec = walk.lruvec;
    walk.next_addr = FIRST_USER_ADDRESS;
    do {
pub static mut lruvec: usize = 0;
    err = -EBUSY;
// another thread might have called inc_max_seq()
    if (walk.seq != max_seq) {
    break;
    }
// the caller might be holding the lock for write
    if (mmap_read_trylock(mm)) {
    err = walk_page_range(mm, walk.next_addr, ULONG_MAX, &mm_walk_ops, walk);
    mmap_read_unlock(mm);
    }
    if (walk.batched) {
    reset_batch_size(walk);
    }
    cond_resched();
    } while (err == -EAGAIN);
    }
#[no_mangle]
pub unsafe extern "C" fn set_mm_walk(pgdat: *mut pglist_data, force_alloc: bool) -> *mut c_void {
    let mut walk = current.reclaim_state.mm_walk;
    if (pgdat && current_is_kswapd()) {
    VM_WARN_ON_ONCE(walk);
    walk = &pgdat.mm_walk;
    } else if (!walk && force_alloc) {
    VM_WARN_ON_ONCE(current_is_kswapd());
    walk = kzalloc_obj(*walk,
    __GFP_HIGH | __GFP_NOMEMALLOC | __GFP_NOWARN);
    }
    current.reclaim_state.mm_walk = walk;
    return walk;
    }
#[no_mangle]
unsafe extern "C" fn clear_mm_walk() {
    let mut walk = current.reclaim_state.mm_walk;
    VM_WARN_ON_ONCE(walk && memchr_inv(walk.nr_pages, 0, sizeof!(walk.nr_pages)));
    VM_WARN_ON_ONCE(walk && memchr_inv(walk.mm_stats, 0, sizeof!(walk.mm_stats)));
    current.reclaim_state.mm_walk = core::ptr::null_mut();
    if (!current_is_kswapd()) {
    kfree(walk);
    }
    }
#[no_mangle]
unsafe extern "C" fn inc_min_seq(lruvec: *mut lruvec, type: c_int, swappiness: c_int) -> bool {
    let mut zone = 0;
pub static mut remaining: c_int = 0;
    let mut lrugen = &lruvec.lrugen;
pub static mut hist: c_int = 0;
    int new_gen, old_gen = lru_gen_from_seq(lrugen.min_seq[type]);
// For file type, skip the check if swappiness is anon only
    if (type && (swappiness == SWAPPINESS_ANON_ONLY)) {
// goto;
    }
// For anon type, skip the check if swappiness is zero (file only)
    if (!type && !swappiness) {
// goto;
    }
// prevent cold/hot inversion if the type is evictable
    while (zone < MAX_NR_ZONES) {
    let mut head = &lrugen.folios[old_gen][type][zone];
    while (!list_empty(head)) {
    let mut folio = lru_to_folio(head);
pub static mut refs: c_int = 0;
pub static mut workingset: bool = false;
    VM_WARN_ON_ONCE_FOLIO(folio_test_unevictable(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_active(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_is_file_lru(folio) != type, folio);
    VM_WARN_ON_ONCE_FOLIO(folio_zonenum(folio) != zone, folio);
    new_gen = folio_inc_gen(lruvec, folio);
    list_move_tail(&folio.lru, &lrugen.folios[new_gen][type][zone]);
// don't count the workingset being lazily promoted
    if (refs + workingset != BIT(LRU_REFS_WIDTH) + 1) {
pub static mut tier: c_int = 0;
pub static mut delta: c_int = 0;
    WRITE_ONCE(lrugen.protected[hist][type][tier],
    lrugen.protected[hist][type][tier] + delta);
    }
    if (!--remaining) {
    return false;
    }
    }
    }
// label;
    reset_ctrl_pos(lruvec, type, true);
    WRITE_ONCE(lrugen.min_seq[type], lrugen.min_seq[type] + 1);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn try_to_inc_min_seq(lruvec: *mut lruvec, swappiness: c_int) {
    let mut gen = 0;
    let mut type = 0;
    let mut zone = 0;
pub static mut seq_inc_flag: bool = false;
    let mut lrugen = &lruvec.lrugen;
pub static mut lruvec: usize = 0;
    VM_WARN_ON_ONCE(!seq_is_valid(lruvec));
// find the oldest populated generation
    for_each_evictable_type(type, swappiness) {
    while (min_seq[type] + MIN_NR_GENS <= lrugen.max_seq) {
    gen = lru_gen_from_seq(min_seq[type]);
    while (zone < MAX_NR_ZONES) {
    if (!list_empty(&lrugen.folios[gen][type][zone])) {
// goto;
    }
    }
    min_seq[type]++;
    seq_inc_flag = true;
    }
// label;
    ;
    }
//
// If min_seq[type] of both anonymous and file is not increased,
// return here to avoid unnecessary checking overhead later.
//
    if (!seq_inc_flag) {
    return;
    }
// see the comment on lru_gen_folio
    if (swappiness && swappiness <= MAX_SWAPPINESS) {
pub static mut seq: c_ulong = 0;
    if (min_seq[LRU_GEN_ANON] > seq && min_seq[LRU_GEN_FILE] < seq) {
    min_seq[LRU_GEN_ANON] = seq;
    }

    else if (min_seq[LRU_GEN_FILE] > seq && min_seq[LRU_GEN_ANON] < seq) {
    min_seq[LRU_GEN_FILE] = seq;
    }
    }
    for_each_evictable_type(type, swappiness) {
    if (min_seq[type] <= lrugen.min_seq[type]) {
    continue;
    }
    reset_ctrl_pos(lruvec, type, true);
    WRITE_ONCE(lrugen.min_seq[type], min_seq[type]);
    }
    }
#[no_mangle]
unsafe extern "C" fn inc_max_seq(lruvec: *mut lruvec, seq: c_ulong, swappiness: c_int) -> bool {
    let mut success = 0;
    let mut prev = 0;
    let mut next = 0;
    let mut type = 0;
    let mut zone = 0;
    let mut lrugen = &lruvec.lrugen;
// label;
    if (seq < READ_ONCE(lrugen.max_seq)) {
    return false;
    }
    lruvec_lock_irq(lruvec);
    VM_WARN_ON_ONCE(!seq_is_valid(lruvec));
    success = seq == lrugen.max_seq;
    if (!success) {
// goto;
    }
    while (type < ANON_AND_FILE) {
    if (get_nr_gens(lruvec, type) != MAX_NR_GENS) {
    continue;
    }
    if (inc_min_seq(lruvec, type, swappiness)) {
    continue;
    }
    lruvec_unlock_irq(lruvec);
    cond_resched();
// goto;
    }
//
// Update the active/inactive LRU sizes for compatibility. Both sides of
// the current max_seq need to be covered, since max_seq+1 can overlap
// with min_seq[LRU_GEN_ANON] if swapping is constrained. And if they do
// overlap, cold/hot inversion happens.
//
    prev = lru_gen_from_seq(lrugen.max_seq - 1);
    next = lru_gen_from_seq(lrugen.max_seq + 1);
    while (type < ANON_AND_FILE) {
    while (zone < MAX_NR_ZONES) {
pub static mut lru: lru_list = 0;
    let mut delta = lrugen.nr_pages[prev][type][zone] -
    lrugen.nr_pages[next][type][zone];
    if (!delta) {
    continue;
    }
    __update_lru_size(lruvec, lru, zone, delta);
    __update_lru_size(lruvec, lru + LRU_ACTIVE, zone, -delta);
    }
    }
    for (type = 0; type < ANON_AND_FILE; type++) {
    reset_ctrl_pos(lruvec, type, false);
    }
    WRITE_ONCE(lrugen.timestamps[next], jiffies);
// make sure preceding modifications appear
    smp_store_release(&lrugen.max_seq, lrugen.max_seq + 1);
// label;
    lruvec_unlock_irq(lruvec);
    return success;
    }
#[no_mangle]
pub unsafe extern "C" fn try_to_inc_max_seq(lruvec: *mut lruvec, seq: c_ulong, swappiness: c_int, force_scan: bool) -> bool {
    let mut success = 0;
pub static mut walk: *mut c_void = core::ptr::null_mut();
    let mut mm = core::ptr::null_mut();
    let mut lrugen = &lruvec.lrugen;
    let mut mm_state = get_mm_state(lruvec);
    VM_WARN_ON_ONCE(seq > READ_ONCE(lrugen.max_seq));
    if (!mm_state) {
    return inc_max_seq(lruvec, seq, swappiness);
    }
// see the comment in iterate_mm_list()
    if (seq <= READ_ONCE(mm_state.seq)) {
    return false;
    }
//
// If the hardware doesn't automatically set the accessed bit, fallback
// to lru_gen_look_around(), which only clears the accessed bit in a
// handful of PTEs. Spreading the work out over a period of time usually
// is less efficient, but it avoids bursty page faults.
//
    if (!should_walk_mmu()) {
    success = iterate_mm_list_nowalk(lruvec, seq);
// goto;
    }
    walk = set_mm_walk(core::ptr::null_mut(), true);
    if (!walk) {
    success = iterate_mm_list_nowalk(lruvec, seq);
// goto;
    }
    walk.lruvec = lruvec;
    walk.seq = seq;
    walk.swappiness = swappiness;
    walk.force_scan = force_scan;
    do {
    success = iterate_mm_list(walk, &mm);
    if (mm) {
    walk_mm(mm, walk);
    }
    } while (mm);
// label;
    if (success) {
    success = inc_max_seq(lruvec, seq, swappiness);
    WARN_ON_ONCE!(!success);
    }
    return success;
    }
//
// working set protection
//
#[no_mangle]
unsafe extern "C" fn set_initial_priority(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let mut priority = 0;
    let mut reclaimable = 0;
    if (sc.priority != DEF_PRIORITY || sc.nr_to_reclaim < MIN_LRU_BATCH) {
    return;
    }
//
// Determine the initial priority based on
// (total >> priority) * reclaimed_to_scanned_ratio = nr_to_reclaim,
// where reclaimed_to_scanned_ratio = inactive / total.
//
    reclaimable = node_page_state(pgdat, NR_INACTIVE_FILE);
    if (can_reclaim_anon_pages(core::ptr::null_mut(), pgdat.node_id, sc)) {
    reclaimable += node_page_state(pgdat, NR_INACTIVE_ANON);
    }
// round down reclaimable and round up sc->nr_to_reclaim
    priority = fls_long(reclaimable) - 1 - fls_long(sc.nr_to_reclaim - 1);
//
// The estimation is based on LRU pages only, so cap it to prevent
// overshoots of shrinker objects by large margins.
//
    sc.priority = clamp(priority, DEF_PRIORITY / 2, DEF_PRIORITY);
    }
#[no_mangle]
unsafe extern "C" fn lruvec_evictable_size(lruvec: *mut lruvec, swappiness: c_int) -> c_ulong {
    let mut gen = 0;
    let mut type = 0;
    let mut zone = 0;
    unsigned long seq, total = 0;
    let mut lrugen = &lruvec.lrugen;
pub static mut lruvec: usize = 0;
pub static mut lruvec: usize = 0;
    for_each_evictable_type(type, swappiness) {
    while (seq <= max_seq) {
    gen = lru_gen_from_seq(seq);
    for (zone = 0; zone < MAX_NR_ZONES; zone++) {
    total += max(READ_ONCE(lrugen.nr_pages[gen][type][zone]), 0L);
    }
    }
    }
    return total;
    }
#[no_mangle]
unsafe extern "C" fn lruvec_is_sizable(lruvec: *mut lruvec, sc: *mut scan_control) -> bool {
    let mut total = 0;
pub static mut swappiness: c_int = 0;
    let mut memcg = lruvec_memcg(lruvec);
    total = lruvec_evictable_size(lruvec, swappiness);
// whether the size is big enough to be helpful
    return mem_cgroup_online(memcg) ? (total >> sc.priority) : total;
    }
#[no_mangle]
pub unsafe extern "C" fn lruvec_is_reclaimable(lruvec: *mut lruvec, sc: *mut scan_control, min_ttl: c_ulong) -> bool {
    let mut gen = 0;
    let mut birth = 0;
pub static mut swappiness: c_int = 0;
    let mut memcg = lruvec_memcg(lruvec);
pub static mut lruvec: usize = 0;
    if (mem_cgroup_below_min(core::ptr::null_mut(), memcg)) {
    return false;
    }
    if (!lruvec_is_sizable(lruvec, sc)) {
    return false;
    }
    gen = lru_gen_from_seq(evictable_min_seq(min_seq, swappiness));
    birth = READ_ONCE(lruvec.lrugen.timestamps[gen]);
    return time_is_before_jiffies(birth + min_ttl);
    }
// to protect the working set of the last N jiffies
    static unsigned long lru_gen_min_ttl ;
#[no_mangle]
unsafe extern "C" fn lru_gen_age_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut min_ttl: c_ulong = 0;
pub static mut reclaimable: bool = false;
    VM_WARN_ON_ONCE(!current_is_kswapd());
    set_initial_priority(pgdat, sc);
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    let mut lruvec = mem_cgroup_lruvec(memcg, pgdat);
    mem_cgroup_calculate_protection(core::ptr::null_mut(), memcg);
    if (!reclaimable) {
    reclaimable = lruvec_is_reclaimable(lruvec, sc, min_ttl);
    }
    } while ((memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut())));
//
// The main goal is to OOM kill if every generation from all memcgs is
// younger than min_ttl. However, another possibility is all memcgs are
// either too small or below min.
//
    if (!reclaimable && mutex_trylock(&oom_lock)) {
pub static mut oom_control: usize = 0;
    out_of_memory(&oc);
    mutex_unlock(&oom_lock);
    }
    }
//
// rmap/PT walk feedback
//
// This function exploits spatial locality when shrink_folio_list() walks the
// rmap. It scans the adjacent PTEs of a young PTE and promotes hot pages. If
// the scan was done cacheline efficiently, it adds the PMD entry pointing to
// the PTE table to the Bloom filter. This forms a feedback loop between the
// eviction and the aging.
//
#[no_mangle]
pub unsafe extern "C" fn lru_gen_look_around(pvmw: *mut page_vma_mapped_walk, nr: c_uint) -> bool {
    let mut i = 0;
    let mut dirty = 0;
    let mut start = 0;
    let mut end = 0;
pub static mut walk: *mut c_void = core::ptr::null_mut();
    let mut last = core::ptr::null_mut();
pub static mut young: c_int = 0;
    let mut pte = pvmw.pte;
pub static mut addr: c_ulong = 0;
    let mut vma = pvmw.vma;
    let mut folio = pfn_folio(pvmw.pfn);
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut pgdat = folio_pgdat(folio);
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
pub static mut mm_state: *mut c_void = core::ptr::null_mut();
    let mut max_seq = 0;
    let mut gen = 0;
    lockdep_assert_held(pvmw.ptl);
    VM_WARN_ON_ONCE_FOLIO(folio_test_lru(folio), folio);
    if (!test_and_clear_young_ptes_notify(vma, addr, pte, nr)) {
    return false;
    }
    if (spin_is_contended(pvmw.ptl)) {
    return true;
    }
// exclude special VMAs containing anon pages from COW
    if (vma.vm_flags & VM_SPECIAL) {
    return true;
    }
// avoid taking the LRU lock under the PTL when possible
    walk = current.reclaim_state ? current.reclaim_state.mm_walk : core::ptr::null_mut();
    start = max(addr & PMD_MASK, vma.vm_start);
    end = min(addr | ~PMD_MASK, vma.vm_end - 1) + 1;
    if (end - start == PAGE_SIZE) {
    return true;
    }
    if (end - start > MIN_LRU_BATCH * PAGE_SIZE) {
    if (addr - start < MIN_LRU_BATCH * PAGE_SIZE / 2) {
    end = start + MIN_LRU_BATCH * PAGE_SIZE;
    }

    else if (end - addr < MIN_LRU_BATCH * PAGE_SIZE / 2) {
    start = end - MIN_LRU_BATCH * PAGE_SIZE;
    }
    else {
    start = addr - MIN_LRU_BATCH * PAGE_SIZE / 2;
    end = addr + MIN_LRU_BATCH * PAGE_SIZE / 2;
    }
    }
    memcg = get_mem_cgroup_from_folio(folio);
    lruvec = mem_cgroup_lruvec(memcg, pgdat);
    max_seq = READ_ONCE((lruvec).lrugen.max_seq);
    gen = lru_gen_from_seq(max_seq);
    mm_state = get_mm_state(lruvec);
    lazy_mmu_mode_enable();
    pte -= (addr - start) / PAGE_SIZE;
    while (addr != end) {
    let mut pfn = 0;
pub static mut ptent: pte_t = 0;
    nr = 1;
    pfn = get_pte_pfn(ptent, vma, addr, pgdat);
    if (pfn == -1) {
    continue;
    }
    folio = get_pfn_folio(pfn, memcg, pgdat);
    if (!folio) {
    continue;
    }
    if (folio_test_large(folio)) {
pub static mut max_nr: c_uint = 0;
    nr = folio_pte_batch_flags(folio, core::ptr::null_mut(), pte, &ptent,
    max_nr, FPB_MERGE_YOUNG_DIRTY);
    }
    if (!test_and_clear_young_ptes_notify(vma, addr, pte, nr)) {
    continue;
    }
    if (last != folio) {
    walk_update_folio(walk, vma, last, gen, dirty);
    last = folio;
    dirty = false;
    }
    if (pte_dirty(ptent)) {
    dirty = true;
    }
    young += nr;
    }
    walk_update_folio(walk, vma, last, gen, dirty);
    lazy_mmu_mode_disable();
// feedback from rmap walkers to page table walkers
    if (mm_state && suitable_to_scan(i, young)) {
    update_bloom_filter(mm_state, max_seq, pvmw.pmd);
    }
    mem_cgroup_put(memcg);
    return true;
    }
//
// memcg LRU
//
// see the comment on MEMCG_NR_GENS
    enum {
    MEMCG_LRU_NOP,
    MEMCG_LRU_HEAD,
    MEMCG_LRU_TAIL,
    MEMCG_LRU_OLD,
    MEMCG_LRU_YOUNG,
    };
#[no_mangle]
unsafe extern "C" fn lru_gen_rotate_memcg(lruvec: *mut lruvec, op: c_int) {
    let mut seg = 0;
    let mut old = 0;
    let mut new = 0;
    let mut flags = 0;
pub static mut bin: c_int = 0;
    let mut pgdat = lruvec_pgdat(lruvec);
    spin_lock_irqsave(&pgdat.memcg_lru.lock, flags);
    VM_WARN_ON_ONCE(hlist_nulls_unhashed(&lruvec.lrugen.list));
    seg = 0;
    new = old = lruvec.lrugen.gen;
// see the comment on MEMCG_NR_GENS
    if (op == MEMCG_LRU_HEAD) {
    seg = MEMCG_LRU_HEAD;
    }

    else if (op == MEMCG_LRU_TAIL) {
    seg = MEMCG_LRU_TAIL;
    }

    else if (op == MEMCG_LRU_OLD) {
    new = get_memcg_gen(pgdat.memcg_lru.seq);
    }

    else if (op == MEMCG_LRU_YOUNG) {
    new = get_memcg_gen(pgdat.memcg_lru.seq + 1);
    }
    else {
    VM_WARN_ON_ONCE(true);
    }
    WRITE_ONCE(lruvec.lrugen.seg, seg);
    WRITE_ONCE(lruvec.lrugen.gen, new);
    hlist_nulls_del_rcu(&lruvec.lrugen.list);
    if (op == MEMCG_LRU_HEAD || op == MEMCG_LRU_OLD) {
    hlist_nulls_add_head_rcu(&lruvec.lrugen.list, &pgdat.memcg_lru.fifo[new][bin]);
    }
    else {
    hlist_nulls_add_tail_rcu(&lruvec.lrugen.list, &pgdat.memcg_lru.fifo[new][bin]);
    }
    pgdat.memcg_lru.nr_memcgs[old]--;
    pgdat.memcg_lru.nr_memcgs[new]++;
    if (!pgdat.memcg_lru.nr_memcgs[old] && old == get_memcg_gen(pgdat.memcg_lru.seq)) {
    WRITE_ONCE(pgdat.memcg_lru.seq, pgdat.memcg_lru.seq + 1);
    }
    spin_unlock_irqrestore(&pgdat.memcg_lru.lock, flags);
    }

#[no_mangle]
pub unsafe extern "C" fn lru_gen_online_memcg(memcg: *mut mem_cgroup) {
    let mut gen = 0;
    let mut nid = 0;
pub static mut bin: c_int = 0;
    for_each_node(nid) {
    let mut pgdat = NODE_DATA(nid);
    let mut lruvec = get_lruvec(memcg, nid);
    spin_lock_irq(&pgdat.memcg_lru.lock);
    VM_WARN_ON_ONCE(!hlist_nulls_unhashed(&lruvec.lrugen.list));
    gen = get_memcg_gen(pgdat.memcg_lru.seq);
    lruvec.lrugen.gen = gen;
    hlist_nulls_add_tail_rcu(&lruvec.lrugen.list, &pgdat.memcg_lru.fifo[gen][bin]);
    pgdat.memcg_lru.nr_memcgs[gen]++;
    spin_unlock_irq(&pgdat.memcg_lru.lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_offline_memcg(memcg: *mut mem_cgroup) {
    let mut nid = 0;
    for_each_node(nid) {
    let mut lruvec = get_lruvec(memcg, nid);
    lru_gen_rotate_memcg(lruvec, MEMCG_LRU_OLD);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_release_memcg(memcg: *mut mem_cgroup) {
    let mut gen = 0;
    let mut nid = 0;
    for_each_node(nid) {
    let mut pgdat = NODE_DATA(nid);
    let mut lruvec = get_lruvec(memcg, nid);
    spin_lock_irq(&pgdat.memcg_lru.lock);
    if (hlist_nulls_unhashed(&lruvec.lrugen.list)) {
// goto;
    }
    gen = lruvec.lrugen.gen;
    hlist_nulls_del_init_rcu(&lruvec.lrugen.list);
    pgdat.memcg_lru.nr_memcgs[gen]--;
    if (!pgdat.memcg_lru.nr_memcgs[gen] && gen == get_memcg_gen(pgdat.memcg_lru.seq)) {
    WRITE_ONCE(pgdat.memcg_lru.seq, pgdat.memcg_lru.seq + 1);
    }
// label;
    spin_unlock_irq(&pgdat.memcg_lru.lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_soft_reclaim(memcg: *mut mem_cgroup, nid: c_int) {
    let mut lruvec = get_lruvec(memcg, nid);
// see the comment on MEMCG_NR_GENS
    if (READ_ONCE(lruvec.lrugen.seg) != MEMCG_LRU_HEAD) {
    lru_gen_rotate_memcg(lruvec, MEMCG_LRU_HEAD);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn recheck_lru_gen_max_memcg(memcg: *mut mem_cgroup, nid: c_int) -> bool {
    let mut lruvec = get_lruvec(memcg, nid);
    let mut type = 0;
    while (type < ANON_AND_FILE) {
    if (get_nr_gens(lruvec, type) != MAX_NR_GENS) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn try_to_inc_max_seq_nowalk(memcg: *mut mem_cgroup, lruvec: *mut lruvec) {
    let mut mm_list = get_mm_list(memcg);
    let mut mm_state = get_mm_state(lruvec);
pub static mut swappiness: c_int = 0;
pub static mut lruvec: usize = 0;
pub static mut success: bool = false;
//
// We are not iterating the mm_list here, updating mm_state->seq is just
// to make mm walkers work properly.
//
    if (mm_state) {
    spin_lock(&mm_list.lock);
    VM_WARN_ON_ONCE(mm_state.seq + 1 < max_seq);
    if (max_seq > mm_state.seq) {
    WRITE_ONCE(mm_state.seq, mm_state.seq + 1);
    success = true;
    }
    spin_unlock(&mm_list.lock);
    } else {
    success = true;
    }
    if (success) {
    inc_max_seq(lruvec, max_seq, swappiness);
    }
    }
//
// We need to ensure that the folios of child memcg can be reparented to the
// same gen of the parent memcg, so the gens of the parent memcg needed be
// incremented to the MAX_NR_GENS before reparenting.
//
#[no_mangle]
pub unsafe extern "C" fn max_lru_gen_memcg(memcg: *mut mem_cgroup, nid: c_int) {
    let mut lruvec = get_lruvec(memcg, nid);
    let mut type = 0;
    while (type < ANON_AND_FILE) {
    while (get_nr_gens(lruvec, type) < MAX_NR_GENS) {
    try_to_inc_max_seq_nowalk(memcg, lruvec);
    cond_resched();
    }
    }
    }
//
// Compared to traditional LRU, MGLRU faces the following challenges:
//
// 1. Each lruvec has between MIN_NR_GENS and MAX_NR_GENS generations, the
// number of generations of the parent and child memcg may be different,
// so we cannot simply transfer MGLRU folios in the child memcg to the
// parent memcg as we did for traditional LRU folios.
// 2. The generation information is stored in folio->flags, but we cannot
// traverse these folios while holding the lru lock, otherwise it may
// cause softlockup.
// 3. In walk_update_folio(), the gen of folio and corresponding lru size
// may be updated, but the folio is not immediately moved to the
// corresponding lru list. Therefore, there may be folios of different
// generations on an LRU list.
// 4. In lru_gen_del_folio(), the generation to which the folio belongs is
// found based on the generation information in folio->flags, and the
// corresponding LRU size will be updated. Therefore, we need to update
// the lru size correctly during reparenting, otherwise the lru size may
// be updated incorrectly in lru_gen_del_folio().
//
// Finally, we choose a compromise method, which is to splice the lru list in
// the child memcg to the lru list of the same generation in the parent memcg
// during reparenting.
//
// The same generation has different meanings in the parent and child memcg,
// so this compromise method will cause the LRU inversion problem. But as the
// system runs, this problem will be fixed automatically.
//
#[no_mangle]
pub unsafe extern "C" fn __lru_gen_reparent_memcg(child_lruvec: *mut lruvec, parent_lruvec: *mut lruvec, zone: c_int, type: c_int) {
    let mut child_lrugen = core::ptr::null_mut();
    let mut parent_lrugen = core::ptr::null_mut();
pub static mut lru: lru_list = 0;
    let mut i = 0;
    child_lrugen = &child_lruvec.lrugen;
    parent_lrugen = &parent_lruvec.lrugen;
    while (i < get_nr_gens(child_lruvec, type)) {
pub static mut gen: c_int = 0;
pub static mut nr_pages: c_long = 0;
pub static mut child_lru_active: c_int = 0;
pub static mut parent_lru_active: c_int = 0;
// Assuming that child pages are colder than parent pages
    list_splice_tail_init(&child_lrugen.folios[gen][type][zone],
    &parent_lrugen.folios[gen][type][zone]);
    WRITE_ONCE(child_lrugen.nr_pages[gen][type][zone], 0);
    WRITE_ONCE(parent_lrugen.nr_pages[gen][type][zone],
    parent_lrugen.nr_pages[gen][type][zone] + nr_pages);
    if (lru_gen_is_active(child_lruvec, gen) != lru_gen_is_active(parent_lruvec, gen)) {
    __update_lru_size(child_lruvec, lru + child_lru_active, zone, -nr_pages);
    __update_lru_size(parent_lruvec, lru + parent_lru_active, zone, nr_pages);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_reparent_memcg(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, nid: c_int) {
    let mut child_lruvec = core::ptr::null_mut();
    let mut parent_lruvec = core::ptr::null_mut();
    let mut type = 0;
    let mut zid = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    enum lru_list lru;
    child_lruvec = get_lruvec(memcg, nid);
    parent_lruvec = get_lruvec(parent, nid);
    for_each_managed_zone_pgdat(zone, NODE_DATA(nid), zid, MAX_NR_ZONES - 1) {
    for (type = 0; type < ANON_AND_FILE; type++)
    }
    __lru_gen_reparent_memcg(child_lruvec, parent_lruvec, zid, type);
    for_each_lru(lru) {
    for_each_managed_zone_pgdat(zone, NODE_DATA(nid), zid, MAX_NR_ZONES - 1) {
pub static mut size: c_ulong = 0;
    if (!size) {
    continue;
    }
// Move the accounting, do not duplicate it.
    mem_cgroup_update_lru_size(parent_lruvec, lru, zid, size);
    mem_cgroup_update_lru_size(child_lruvec, lru, zid, -(long)size);
    }
    }
    }

//
// the eviction
//
#[no_mangle]
pub unsafe extern "C" fn sort_folio(lruvec: *mut lruvec, folio: *mut folio, sc: *mut scan_control, tier_idx: c_int) -> bool {
pub static mut gen: c_int = 0;
pub static mut type: c_int = 0;
pub static mut zone: c_int = 0;
pub static mut delta: c_int = 0;
pub static mut refs: c_int = 0;
pub static mut workingset: bool = false;
pub static mut tier: c_int = 0;
    let mut lrugen = &lruvec.lrugen;
    VM_WARN_ON_ONCE_FOLIO(gen >= MAX_NR_GENS, folio);
// unevictable: let it through and the generic path will cull it
    if (!folio_evictable(folio)) {
    return false;
    }
// promoted
    if (gen != lru_gen_from_seq(lrugen.min_seq[type])) {
    list_move(&folio.lru, &lrugen.folios[gen][type][zone]);
    return true;
    }
// protected
    if (tier > tier_idx || refs + workingset == BIT(LRU_REFS_WIDTH) + 1) {
    gen = folio_inc_gen(lruvec, folio);
    list_move(&folio.lru, &lrugen.folios[gen][type][zone]);
// don't count the workingset being lazily promoted
    if (refs + workingset != BIT(LRU_REFS_WIDTH) + 1) {
pub static mut hist: c_int = 0;
    WRITE_ONCE(lrugen.protected[hist][type][tier],
    lrugen.protected[hist][type][tier] + delta);
    }
    return true;
    }
// ineligible
    if (zone > sc.reclaim_idx) {
    gen = folio_inc_gen(lruvec, folio);
    list_move_tail(&folio.lru, &lrugen.folios[gen][type][zone]);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn isolate_folio(lruvec: *mut lruvec, folio: *mut folio, sc: *mut scan_control) -> bool {
    let mut success = 0;
// raced with release_pages()
    if (!folio_try_get(folio)) {
    return false;
    }
// raced with another isolation
    if (!folio_test_clear_lru(folio)) {
    folio_put(folio);
    return false;
    }
// see the comment on LRU_REFS_FLAGS
    if (!folio_test_referenced(folio)) {
    set_mask_bits(&folio.flags.f, LRU_REFS_MASK, 0);
    }
    success = lru_gen_del_folio(lruvec, folio, true);
    VM_WARN_ON_ONCE_FOLIO(!success, folio);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn scan_folios(nr_to_scan: c_ulong, lruvec: *mut lruvec, sc: *mut scan_control, type: c_int, tier: c_int, list: *mut list_head, isolatedp: *mut c_int) -> c_int {
    let mut i = 0;
    let mut gen = 0;
    enum node_stat_item item;
pub static mut sorted: c_int = 0;
pub static mut scanned: c_int = 0;
pub static mut isolated: c_int = 0;
pub static mut skipped: c_int = 0;
pub static mut remaining: c_ulong = 0;
    let mut lrugen = &lruvec.lrugen;
    VM_WARN_ON_ONCE(nr_to_scan > MAX_LRU_BATCH);
    VM_WARN_ON_ONCE(!list_empty(list));
    if (get_nr_gens(lruvec, type) == MIN_NR_GENS) {
    return 0;
    }
    gen = lru_gen_from_seq(lrugen.min_seq[type]);
    while (i > 0) {
pub static mut moved: usize = 0;
pub static mut skipped_zone: c_int = 0;
pub static mut zone: c_int = 0;
    let mut head = &lrugen.folios[gen][type][zone];
    while (!list_empty(head)) {
    let mut folio = lru_to_folio(head);
pub static mut delta: c_int = 0;
    VM_WARN_ON_ONCE_FOLIO(folio_test_unevictable(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_active(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_is_file_lru(folio) != type, folio);
    VM_WARN_ON_ONCE_FOLIO(folio_zonenum(folio) != zone, folio);
    scanned += delta;
    if (sort_folio(lruvec, folio, sc, tier)) {
    sorted += delta;
    }
if true {
    list_add(&folio.lru, list);
    isolated += delta;
    } else {
    list_move(&folio.lru, &moved);
    skipped_zone += delta;
    }
    if (!--remaining || max(isolated, skipped_zone) >= MIN_LRU_BATCH) {
    break;
    }
    }
    if (skipped_zone) {
    list_splice(&moved, head);
    __count_zid_vm_events(PGSCAN_SKIP, zone, skipped_zone);
    skipped += skipped_zone;
    }
    if (!remaining || isolated >= MIN_LRU_BATCH) {
    break;
    }
    }
    item = PGSCAN_KSWAPD + reclaimer_offset(sc);
    mod_lruvec_state(lruvec, item, isolated);
    mod_lruvec_state(lruvec, PGREFILL, sorted);
    mod_lruvec_state(lruvec, PGSCAN_ANON + type, isolated);
    trace_mm_vmscan_lru_isolate(sc.reclaim_idx, sc.order, nr_to_scan,
    scanned, skipped, isolated,
    type ? LRU_INACTIVE_FILE : LRU_INACTIVE_ANON);
// isolatedp = isolated;
    return scanned;
    }
#[no_mangle]
unsafe extern "C" fn get_tier_idx(lruvec: *mut lruvec, type: c_int) -> c_int {
    let mut tier = 0;
    struct ctrl_pos sp, pv = {};
//
// To leave a margin for fluctuations, use a larger gain factor (2:3).
// This value is chosen because any other tier would have at least twice
// as many refaults as the first tier.
//
    read_ctrl_pos(lruvec, type, 0, 2, &sp);
    while (tier < MAX_NR_TIERS) {
    read_ctrl_pos(lruvec, type, tier, 3, &pv);
    if (!positive_ctrl_err(&sp, &pv)) {
    break;
    }
    }
    return tier - 1;
    }
#[no_mangle]
unsafe extern "C" fn get_type_to_scan(lruvec: *mut lruvec, swappiness: c_int) -> c_int {
    struct ctrl_pos sp, pv = {};
    if (swappiness <= MIN_SWAPPINESS + 1) {
    return LRU_GEN_FILE;
    }
    if (swappiness >= MAX_SWAPPINESS) {
    return LRU_GEN_ANON;
    }
//
// Compare the sum of all tiers of anon with that of file to determine
// which type to scan.
//
    read_ctrl_pos(lruvec, LRU_GEN_ANON, MAX_NR_TIERS, swappiness, &sp);
    read_ctrl_pos(lruvec, LRU_GEN_FILE, MAX_NR_TIERS, MAX_SWAPPINESS - swappiness, &pv);
    return positive_ctrl_err(&sp, &pv);
    }
#[no_mangle]
pub unsafe extern "C" fn isolate_folios(nr_to_scan: c_ulong, lruvec: *mut lruvec, sc: *mut scan_control, swappiness: c_int, list: *mut list_head, isolated: *mut c_int, isolate_type: *mut c_int, isolate_scanned: *mut c_int) -> c_int {
    let mut i = 0;
pub static mut total_scanned: c_int = 0;
pub static mut type: c_int = 0;
    for_each_evictable_type(i, swappiness) {
    let mut scanned = 0;
pub static mut tier: c_int = 0;
    scanned = scan_folios(nr_to_scan, lruvec, sc,
    type, tier, list, isolated);
    total_scanned += scanned;
    if (*isolated) {
// isolate_type = type;
// isolate_scanned = scanned;
    break;
    }
//
// If scanned > 0 and isolated == 0, avoid falling back to the
// other type, as this type remains sufficient. Falling back
// too readily can disrupt the positive_ctrl_err() bias.
//
    if (!scanned) {
    type = !type;
    }
    }
    return total_scanned;
    }
#[no_mangle]
pub unsafe extern "C" fn evict_folios(nr_to_scan: c_ulong, lruvec: *mut lruvec, sc: *mut scan_control, swappiness: c_int) -> c_int {
pub static mut list: usize = 0;
pub static mut clean: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
    enum node_stat_item item;
pub static mut stat: usize = 0;
pub static mut walk: *mut c_void = core::ptr::null_mut();
    let mut scanned = 0;
    let mut reclaimed = 0;
pub static mut isolated: c_int = 0;
pub static mut total_reclaimed: c_ulong = 0;
pub static mut skip_retry: bool = false;
    let mut memcg = lruvec_memcg(lruvec);
    let mut pgdat = lruvec_pgdat(lruvec);
    lruvec_lock_irq(lruvec);
// In case folio deletion left empty old gens, flush them
    try_to_inc_min_seq(lruvec, swappiness);
    scanned = isolate_folios(nr_to_scan, lruvec, sc, swappiness,
    &list, &isolated, &type, &type_scanned);
    nr_isolated = isolated;
// Scanning may have emptied the oldest gen, flush it
    if (scanned) {
    try_to_inc_min_seq(lruvec, swappiness);
    }
    lruvec_unlock_irq(lruvec);
    if (list_empty(&list)) {
    return scanned;
    }
// label;
    reclaimed = shrink_folio_list(&list, pgdat, sc, &stat, false, memcg);
    sc.nr_reclaimed += reclaimed;
    total_reclaimed += reclaimed;
// Retry pass is only meant for clean folios without new isolation
    if (isolated) {
    handle_reclaim_writeback(isolated, pgdat, sc, &stat);
    }
    trace_mm_vmscan_lru_shrink_inactive(pgdat.node_id,
    type_scanned, reclaimed, &stat, sc.priority,
    type ? LRU_INACTIVE_FILE : LRU_INACTIVE_ANON);
    list_for_each_entry_safe_reverse(folio, next, &list, lru) {
pub static mut lruvec: usize = 0;
// move_folios_to_lru() culls unevictable folios via folio_putback_lru()
    if (!folio_evictable(folio)) {
    continue;
    }
// retry folios that may have missed folio_rotate_reclaimable()
    if (!skip_retry && !folio_test_active(folio) && !folio_mapped(folio) &&
    !folio_test_dirty(folio) && !folio_test_writeback(folio)) {
    list_move(&folio.lru, &clean);
    continue;
    }
// don't add rejected folios to the oldest generation
    if (lru_gen_folio_seq(lruvec, folio, false) == min_seq[type]) {
    set_mask_bits(&folio.flags.f, LRU_REFS_FLAGS, BIT(PG_active));
    }
    }
    move_folios_to_lru(&list);
    walk = current.reclaim_state.mm_walk;
    if (walk && walk.batched) {
    walk.lruvec = lruvec;
    reset_batch_size(walk);
    }
    mod_lruvec_state(lruvec, PGDEMOTE_KSWAPD + reclaimer_offset(sc),
    stat.nr_demoted);
    item = PGSTEAL_KSWAPD + reclaimer_offset(sc);
    mod_lruvec_state(lruvec, item, reclaimed);
    mod_lruvec_state(lruvec, PGSTEAL_ANON + type, reclaimed);
    list_splice_init(&clean, &list);
    if (!list_empty(&list)) {
    skip_retry = true;
    isolated = 0;
// goto;
    }
    if (nr_isolated > total_reclaimed) {
    mod_lruvec_state(lruvec, PGROTATE_ANON + type,
    nr_isolated - total_reclaimed);
    }
    return scanned;
    }
#[no_mangle]
pub unsafe extern "C" fn should_run_aging(lruvec: *mut lruvec, max_seq: c_ulong, sc: *mut scan_control, swappiness: c_int) -> bool {
pub static mut lruvec: usize = 0;
// have to run aging, since eviction is not possible anymore
    if (evictable_min_seq(min_seq, swappiness) + MIN_NR_GENS > max_seq) {
    return true;
    }
// try to avoid aging, do gentle reclaim at the default priority
    if (sc.priority == DEF_PRIORITY) {
    return false;
    }
// better to run aging even though eviction is still possible
    return evictable_min_seq(min_seq, swappiness) + MIN_NR_GENS == max_seq;
    }
#[no_mangle]
pub unsafe extern "C" fn get_nr_to_scan(lruvec: *mut lruvec, sc: *mut scan_control, memcg: *mut mem_cgroup, swappiness: c_int) -> c_long {
    unsigned long nr_to_scan, evictable;
    let mut pgdat = lruvec_pgdat(lruvec);
//
// Proactive reclaim initiated by userspace for anonymous memory only.
// SWAPPINESS_ANON_ONLY is set only on the proactive reclaim path, so
// warn if it shows up elsewhere. When anon cannot be reclaimed (e.g.
// no swap), return 0 to skip the scan entirely, avoiding useless scan
// work when there is nothing eligible to reclaim.
//
    if (swappiness == SWAPPINESS_ANON_ONLY) {
    WARN_ON_ONCE!(!sc.proactive);
    if (!can_reclaim_anon_pages(memcg, pgdat.node_id, sc)) {
    return 0;
    }
    }
    evictable = lruvec_evictable_size(lruvec, swappiness);
// try to scrape all its memory if this memcg was deleted
    if (!mem_cgroup_online(memcg)) {
    return evictable;
    }
    nr_to_scan = apply_proportional_protection(memcg, sc, evictable);
    nr_to_scan >>= sc.priority;
    return nr_to_scan;
    }
#[no_mangle]
unsafe extern "C" fn should_abort_scan(lruvec: *mut lruvec, sc: *mut scan_control) -> bool {
    let mut i = 0;
    enum zone_watermarks mark;
    if (unlikely(sc.proactive && signal_pending(current))) {
    return true;
    }
    if (sc.nr_reclaimed >= max(sc.nr_to_reclaim, compact_gap(sc.order))) {
    return true;
    }
// check the order to exclude compaction-induced reclaim
    if (!current_is_kswapd() || sc.order) {
    return false;
    }
    mark = sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING ?
    WMARK_PROMO : WMARK_HIGH;
    while (i <= sc.reclaim_idx) {
    let mut zone = lruvec_pgdat(lruvec).node_zones + i;
pub static mut size: c_ulong = 0;
    if (managed_zone(zone) && !zone_watermark_ok(zone, 0, size, sc.reclaim_idx, 0)) {
    return false;
    }
    }
// kswapd should abort if all eligible zones are safe
    return true;
    }
//
// For future optimizations:
// 1. Defer try_to_inc_max_seq() to workqueues to reduce latency for memcg
// reclaim.
//
#[no_mangle]
unsafe extern "C" fn try_to_shrink_lruvec(lruvec: *mut lruvec, sc: *mut scan_control) -> bool {
pub static mut need_rotate: bool = false;
    let mut nr_batch = 0;
    let mut nr_to_scan = 0;
pub static mut swappiness: c_int = 0;
    let mut memcg = lruvec_memcg(lruvec);
    nr_to_scan = get_nr_to_scan(lruvec, sc, memcg, swappiness);
    while (nr_to_scan > 0) {
    let mut delta = 0;
pub static mut lruvec: usize = 0;
    if (mem_cgroup_below_min(sc.target_mem_cgroup, memcg)) {
    need_rotate = true;
    break;
    }
    if (should_run_aging(lruvec, max_seq, sc, swappiness)) {
    if (try_to_inc_max_seq(lruvec, max_seq, swappiness, false)) {
    need_rotate = true;
    }
    should_age = true;
    }
    nr_batch = min(nr_to_scan, MIN_LRU_BATCH);
    delta = evict_folios(nr_batch, lruvec, sc, swappiness);
    if (!delta) {
    break;
    }
    if (should_abort_scan(lruvec, sc)) {
    break;
    }
//
// Root reclaim needs rotation when low on cold folio for better
// fairness. Cgroup reclaim gets fairness from the iterator.
//
    if (root_reclaim(sc) && should_age) {
    break;
    }
    nr_to_scan -= delta;
    cond_resched();
    }
    return need_rotate;
    }
#[no_mangle]
unsafe extern "C" fn shrink_one(lruvec: *mut lruvec, sc: *mut scan_control) -> c_int {
    let mut need_rotate = 0;
pub static mut scanned: c_ulong = 0;
pub static mut reclaimed: c_ulong = 0;
    let mut memcg = lruvec_memcg(lruvec);
    let mut pgdat = lruvec_pgdat(lruvec);
// lru_gen_age_node() called mem_cgroup_calculate_protection()
    if (mem_cgroup_below_min(core::ptr::null_mut(), memcg)) {
    return MEMCG_LRU_YOUNG;
    }
    if (mem_cgroup_below_low(core::ptr::null_mut(), memcg)) {
// see the comment on MEMCG_NR_GENS
    if (READ_ONCE(lruvec.lrugen.seg) != MEMCG_LRU_TAIL) {
    return MEMCG_LRU_TAIL;
    }
    memcg_memory_event(memcg, MEMCG_LOW);
    }
    need_rotate = try_to_shrink_lruvec(lruvec, sc);
    shrink_slab(sc.gfp_mask, pgdat.node_id, memcg, sc.priority);
    if (!sc.proactive) {
    vmpressure(sc.gfp_mask, sc.order, memcg, false,
    sc.nr_scanned - scanned, sc.nr_reclaimed - reclaimed);
    }
    flush_reclaim_state(sc);
    if (need_rotate && mem_cgroup_online(memcg)) {
    return MEMCG_LRU_YOUNG;
    }
    if (!need_rotate && lruvec_is_sizable(lruvec, sc)) {
    return 0;
    }
// one retry if offlined or too small
    return READ_ONCE(lruvec.lrugen.seg) != MEMCG_LRU_TAIL ?
    MEMCG_LRU_TAIL : MEMCG_LRU_YOUNG;
    }
#[no_mangle]
unsafe extern "C" fn shrink_many(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let mut op = 0;
    let mut gen = 0;
    let mut bin = 0;
    let mut first_bin = 0;
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
pub static mut lrugen: *mut c_void = core::ptr::null_mut();
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut pos: *mut c_void = core::ptr::null_mut();
    gen = get_memcg_gen(READ_ONCE(pgdat.memcg_lru.seq));
    bin = first_bin = get_random_u32_below(MEMCG_NR_BINS);
// label;
    op = 0;
    memcg = core::ptr::null_mut();
    rcu_read_lock();
    hlist_nulls_for_each_entry_rcu(lrugen, pos, &pgdat.memcg_lru.fifo[gen][bin], list) {
    if (op) {
    lru_gen_rotate_memcg(lruvec, op);
    op = 0;
    }
    mem_cgroup_put(memcg);
    memcg = core::ptr::null_mut();
    if (gen != READ_ONCE(lrugen.gen)) {
    continue;
    }
    lruvec = container_of!(lrugen, lruvec, lrugen);
    memcg = lruvec_memcg(lruvec);
    if (!mem_cgroup_tryget(memcg)) {
    lru_gen_release_memcg(memcg);
    memcg = core::ptr::null_mut();
    continue;
    }
    rcu_read_unlock();
    op = shrink_one(lruvec, sc);
    rcu_read_lock();
    if (should_abort_scan(lruvec, sc)) {
    break;
    }
    }
    rcu_read_unlock();
    if (op) {
    lru_gen_rotate_memcg(lruvec, op);
    }
    mem_cgroup_put(memcg);
    if (!is_a_nulls(pos)) {
    return;
    }
// restart if raced with lru_gen_rotate_memcg()
    if (gen != get_nulls_value(pos)) {
// goto;
    }
// try the rest of the bins of the current generation
    bin = get_memcg_bin(bin + 1);
    if (bin != first_bin) {
// goto;
    }
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_shrink_lruvec(lruvec: *mut lruvec, sc: *mut scan_control) {
pub static mut plug: usize = 0;
    VM_WARN_ON_ONCE(root_reclaim(sc));
    VM_WARN_ON_ONCE(!sc.may_writepage || !sc.may_unmap);
    lru_add_drain();
    blk_start_plug(&plug);
    set_mm_walk(core::ptr::null_mut(), sc.proactive);
    if (try_to_shrink_lruvec(lruvec, sc)) {
    lru_gen_rotate_memcg(lruvec, MEMCG_LRU_YOUNG);
    }
    clear_mm_walk();
    blk_finish_plug(&plug);
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_shrink_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
pub static mut plug: usize = 0;
pub static mut reclaimed: c_ulong = 0;
    VM_WARN_ON_ONCE(!root_reclaim(sc));
//
// Unmapped clean folios are already prioritized. Scanning for more of
// them is likely futile and can cause high reclaim latency when there
// is a large number of memcgs.
//
    if (!sc.may_writepage || !sc.may_unmap) {
// goto;
    }
    lru_add_drain();
    blk_start_plug(&plug);
    set_mm_walk(pgdat, sc.proactive);
    set_initial_priority(pgdat, sc);
    if (current_is_kswapd()) {
    sc.nr_reclaimed = 0;
    }
    if (mem_cgroup_disabled()) {
    shrink_one(&pgdat.__lruvec, sc);
    }
    else {
    shrink_many(pgdat, sc);
    }
    if (current_is_kswapd()) {
    sc.nr_reclaimed += reclaimed;
    }
    clear_mm_walk();
    blk_finish_plug(&plug);
// label;
    if (sc.nr_reclaimed > reclaimed) {
    kswapd_try_clear_hopeless(pgdat, sc.order, sc.reclaim_idx);
    }
    }
//
// state change
//
#[no_mangle]
unsafe extern "C" fn state_is_valid(lruvec: *mut lruvec) -> bool __maybe_unused {
    let mut lrugen = &lruvec.lrugen;
    if (lrugen.enabled) {
    enum lru_list lru;
    for_each_evictable_lru(lru) {
    if (!list_empty(&lruvec.lists[lru])) {
    return false;
    }
    }
    } else {
    let mut gen = 0;
    let mut type = 0;
    let mut zone = 0;
    for_each_gen_type_zone(gen, type, zone) {
    if (!list_empty(&lrugen.folios[gen][type][zone])) {
    return false;
    }
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn fill_evictable(lruvec: *mut lruvec) -> bool {
    enum lru_list lru;
pub static mut remaining: c_int = 0;
    for_each_evictable_lru(lru) {
pub static mut type: c_int = 0;
pub static mut active: bool = false;
    let mut head = &lruvec.lists[lru];
    while (!list_empty(head)) {
    let mut success = 0;
    let mut folio = lru_to_folio(head);
    VM_WARN_ON_ONCE_FOLIO(folio_test_unevictable(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_active(folio) != active, folio);
    VM_WARN_ON_ONCE_FOLIO(folio_is_file_lru(folio) != type, folio);
    VM_WARN_ON_ONCE_FOLIO(folio_lru_gen(folio) != -1, folio);
    lruvec_del_folio(lruvec, folio);
    success = lru_gen_add_folio(lruvec, folio, false);
    VM_WARN_ON_ONCE(!success);
    if (!--remaining) {
    return false;
    }
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn drain_evictable(lruvec: *mut lruvec) -> bool {
    let mut gen = 0;
    let mut type = 0;
    let mut zone = 0;
pub static mut remaining: c_int = 0;
    for_each_gen_type_zone(gen, type, zone) {
    let mut head = &lruvec.lrugen.folios[gen][type][zone];
    while (!list_empty(head)) {
    let mut success = 0;
    let mut folio = lru_to_folio(head);
    VM_WARN_ON_ONCE_FOLIO(folio_test_unevictable(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_active(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_is_file_lru(folio) != type, folio);
    VM_WARN_ON_ONCE_FOLIO(folio_zonenum(folio) != zone, folio);
    success = lru_gen_del_folio(lruvec, folio, false);
    VM_WARN_ON_ONCE(!success);
    lruvec_add_folio(lruvec, folio);
    if (!--remaining) {
    return false;
    }
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_change_state(enabled: bool) {
pub static mut state_mutex: usize = 0;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    cgroup_lock();
    cpus_read_lock();
    get_online_mems();
    mutex_lock(&state_mutex);
    if (enabled == lru_gen_enabled()) {
// goto;
    }
    static_branch_enable_cpuslocked(&lru_switch);
    if (enabled) {
    static_branch_enable_cpuslocked(&lru_gen_caps[LRU_GEN_CORE]);
    }
    else {
    static_branch_disable_cpuslocked(&lru_gen_caps[LRU_GEN_CORE]);
    }
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    let mut nid = 0;
    for_each_node(nid) {
    let mut lruvec = get_lruvec(memcg, nid);
    lruvec_lock_irq(lruvec);
    VM_WARN_ON_ONCE(!seq_is_valid(lruvec));
    VM_WARN_ON_ONCE(!state_is_valid(lruvec));
    lruvec.lrugen.enabled = enabled;
    while (!(enabled ? fill_evictable(lruvec) : drain_evictable(lruvec))) {
    lruvec_unlock_irq(lruvec);
    cond_resched();
    lruvec_lock_irq(lruvec);
    }
    lruvec_unlock_irq(lruvec);
    }
    cond_resched();
    } while ((memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut())));
    static_branch_disable_cpuslocked(&lru_switch);
// label;
    mutex_unlock(&state_mutex);
    put_online_mems();
    cpus_read_unlock();
    cgroup_unlock();
    }
//
// sysfs interface
//
#[no_mangle]
unsafe extern "C" fn min_ttl_ms_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", jiffies_to_msecs(READ_ONCE(lru_gen_min_ttl)));
    }
// see Documentation/admin-guide/mm/multigen_lru.rst for details
#[no_mangle]
pub unsafe extern "C" fn min_ttl_ms_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, len: size_t) -> ssize_t {
    let mut msecs = 0;
    if (kstrtouint(buf, 0, &msecs)) {
    return -EINVAL;
    }
    WRITE_ONCE(lru_gen_min_ttl, msecs_to_jiffies(msecs));
    return len;
    }
pub static mut lru_gen_min_ttl_attr: kobj_attribute = 0;
#[no_mangle]
unsafe extern "C" fn enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
pub static mut caps: c_uint = 0;
    if (get_cap(LRU_GEN_CORE)) {
    caps |= BIT(LRU_GEN_CORE);
    }
    if (should_walk_mmu()) {
    caps |= BIT(LRU_GEN_MM_WALK);
    }
    if (should_clear_pmd_young()) {
    caps |= BIT(LRU_GEN_NONLEAF_YOUNG);
    }
    return sysfs_emit(buf, "0x%04x\n", caps);
    }
// see Documentation/admin-guide/mm/multigen_lru.rst for details
#[no_mangle]
pub unsafe extern "C" fn enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, len: size_t) -> ssize_t {
    let mut i = 0;
    let mut caps = 0;
    if (tolower(*buf) == 'n') {
    caps = 0;
    }

    else if (tolower(*buf) == 'y') {
    caps = -1;
    }

    else if (kstrtouint(buf, 0, &caps)) {
    return -EINVAL;
    }
    while (i < NR_LRU_GEN_CAPS) {
pub static mut enabled: bool = false;
    if (i == LRU_GEN_CORE) {
    lru_gen_change_state(enabled);
    }

    else if (enabled) {
    static_branch_enable(&lru_gen_caps[i]);
    }
    else {
    static_branch_disable(&lru_gen_caps[i]);
    }
    }
    return len;
    }
pub static mut lru_gen_enabled_attr: kobj_attribute = 0;
    static struct attribute *lru_gen_attrs[] = {
    &lru_gen_min_ttl_attr.attr,
    &lru_gen_enabled_attr.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
//
// debugfs interface
//
#[no_mangle]
pub unsafe extern "C" fn lru_gen_seq_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut nr_to_skip: loff_t = 0;
    m.private = kvmalloc(PATH_MAX, GFP_KERNEL);
    if (!m.private) {
    return ERR_PTR(-ENOMEM);
    }
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    let mut nid = 0;
    for_each_node_state(nid, N_MEMORY) {
    if (!nr_to_skip--) {
    return get_lruvec(memcg, nid);
    }
    }
    } while ((memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut())));
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_seq_stop(m: *mut seq_file, v: *mut c_void) {
    if (!IS_ERR_OR_NULL(v)) {
    mem_cgroup_iter_break(core::ptr::null_mut(), lruvec_memcg(v));
    }
    kvfree(m.private);
    m.private = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_seq_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
pub static mut nid: c_int = 0;
    let mut memcg = lruvec_memcg(v);
    ++*pos;
    nid = next_memory_node(nid);
    if (nid == MAX_NUMNODES) {
    memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut());
    if (!memcg) {
    return core::ptr::null_mut();
    }
    nid = first_memory_node;
    }
    return get_lruvec(memcg, nid);
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_seq_show_full(m: *mut seq_file, lruvec: *mut lruvec, max_seq: c_ulong, min_seq: *mut c_ulong, seq: c_ulong) {
    let mut i = 0;
    let mut type = 0;
    let mut tier = 0;
pub static mut hist: c_int = 0;
    let mut lrugen = &lruvec.lrugen;
    let mut mm_state = get_mm_state(lruvec);
    while (tier < MAX_NR_TIERS) {
    seq_printf(m, "            %10d", tier);
    while (type < ANON_AND_FILE) {
    let mut s = "xxx";
    unsigned long n[3] = {};
    if (seq == max_seq) {
    s = "RTx";
    n[0] = READ_ONCE(lrugen.avg_refaulted[type][tier]);
    n[1] = READ_ONCE(lrugen.avg_total[type][tier]);
    } else if (seq == min_seq[type] || NR_HIST_GENS > 1) {
    s = "rep";
    n[0] = atomic_long_read(&lrugen.refaulted[hist][type][tier]);
    n[1] = atomic_long_read(&lrugen.evicted[hist][type][tier]);
    n[2] = READ_ONCE(lrugen.protected[hist][type][tier]);
    }
    for (i = 0; i < 3; i++) {
    seq_printf(m, " %10lu%c", n[i], s[i]);
    }
    }
    seq_putc(m, '\n');
    }
    if (!mm_state) {
    return;
    }
    seq_puts(m, "                      ");
    while (i < NR_MM_STATS) {
    let mut s = "xxxx";
pub static mut n: c_ulong = 0;
    if (seq == max_seq && NR_HIST_GENS == 1) {
    s = "TYFA";
    n = READ_ONCE(mm_state.stats[hist][i]);
    } else if (seq != max_seq && NR_HIST_GENS > 1) {
    s = "tyfa";
    n = READ_ONCE(mm_state.stats[hist][i]);
    }
    seq_printf(m, " %10lu%c", n, s[i]);
    }
    seq_putc(m, '\n');
    }
// see Documentation/admin-guide/mm/multigen_lru.rst for details
#[no_mangle]
unsafe extern "C" fn lru_gen_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut seq = 0;
pub static mut full: bool = false;
    let mut lruvec = v;
    let mut lrugen = &lruvec.lrugen;
pub static mut nid: c_int = 0;
    let mut memcg = lruvec_memcg(lruvec);
pub static mut lruvec: usize = 0;
pub static mut lruvec: usize = 0;
    if (nid == first_memory_node) {
    let mut path = memcg ? m.private : "";

    if (memcg) {
    cgroup_path(memcg.css.cgroup, m.private, PATH_MAX);
    }

    seq_printf(m, "memcg %llu %s\n", mem_cgroup_id(memcg), path);
    }
    seq_printf(m, " node %5d\n", nid);
    if (!full) {
    seq = evictable_min_seq(min_seq, MAX_SWAPPINESS / 2);
    }

    else if (max_seq >= MAX_NR_GENS) {
    seq = max_seq - MAX_NR_GENS + 1;
    }
    else {
    seq = 0;
    }
    while (seq <= max_seq) {
    let mut type = 0;
    let mut zone = 0;
pub static mut gen: c_int = 0;
pub static mut birth: c_ulong = 0;
    seq_printf(m, " %10lu %10u", seq, jiffies_to_msecs(jiffies - birth));
    while (type < ANON_AND_FILE) {
pub static mut size: c_ulong = 0;
pub static mut mark: c_char = 0;
    for (zone = 0; zone < MAX_NR_ZONES; zone++) {
    size += max(READ_ONCE(lrugen.nr_pages[gen][type][zone]), 0L);
    }
    seq_printf(m, " %10lu%c", size, mark);
    }
    seq_putc(m, '\n');
    if (full) {
    lru_gen_seq_show_full(m, lruvec, max_seq, min_seq, seq);
    }
    }
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn run_aging(lruvec: *mut lruvec, seq: c_ulong, swappiness: c_int, force_scan: bool) -> c_int {
pub static mut lruvec: usize = 0;
    if (seq > max_seq) {
    return -EINVAL;
    }
    return try_to_inc_max_seq(lruvec, max_seq, swappiness, force_scan) ? 0 : -EEXIST;
    }
#[no_mangle]
pub unsafe extern "C" fn run_eviction(lruvec: *mut lruvec, seq: c_ulong, sc: *mut scan_control, swappiness: c_int, nr_to_reclaim: c_ulong) -> c_int {
    let mut nr_batch = 0;
pub static mut lruvec: usize = 0;
    if (seq + MIN_NR_GENS > max_seq) {
    return -EINVAL;
    }
    sc.nr_reclaimed = 0;
    while (!signal_pending(current)) {
pub static mut lruvec: usize = 0;
    if (seq < evictable_min_seq(min_seq, swappiness)) {
    return 0;
    }
    if (sc.nr_reclaimed >= nr_to_reclaim) {
    return 0;
    }
    nr_batch = min(nr_to_reclaim - sc.nr_reclaimed, MAX_LRU_BATCH);
    if (!evict_folios(nr_batch, lruvec, sc, swappiness)) {
    return 0;
    }
    cond_resched();
    }
    return -EINTR;
    }
#[no_mangle]
pub unsafe extern "C" fn run_cmd(cmd: c_char, memcg_id: u64, nid: c_int, seq: c_ulong, sc: *mut scan_control, swappiness: c_int, opt: c_ulong) -> c_int {
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    let mut memcg = core::ptr::null_mut();
    if (nid < 0 || nid >= MAX_NUMNODES || !node_state(nid, N_MEMORY)) {
    return -EINVAL;
    }
    if (!mem_cgroup_disabled()) {
    memcg = mem_cgroup_get_from_id(memcg_id);
    if (!memcg) {
    return -EINVAL;
    }
    }
    if (memcg_id != mem_cgroup_id(memcg)) {
// goto;
    }
    sc.target_mem_cgroup = memcg;
    lruvec = get_lruvec(memcg, nid);
    if (swappiness < MIN_SWAPPINESS) {
    swappiness = get_swappiness(lruvec, sc);
    }

    else if (swappiness > SWAPPINESS_ANON_ONLY) {
// goto;
    }
    match (cmd) {
    '+' => {
    err = run_aging(lruvec, seq, swappiness, opt);
    // break;
    }
    '-' => {
    err = run_eviction(lruvec, seq, sc, swappiness, opt);
    // break;
    }
    }
// label;
    mem_cgroup_put(memcg);
    return err;
    }
// see Documentation/admin-guide/mm/multigen_lru.rst for details
#[no_mangle]
pub unsafe extern "C" fn lru_gen_seq_write(file: *mut file, src: *mut c_char, len: size_t, pos: *mut loff_t) -> ssize_t {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut cur = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut flags = 0;
pub static mut plug: usize = 0;
pub static mut err: c_int = 0;
pub static mut scan_control: usize = 0;
    buf = kvmalloc(len + 1, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    if (copy_from_user(buf, src, len)) {
    kvfree(buf);
    return -EFAULT;
    }
    set_task_reclaim_state(current, &sc.reclaim_state);
    flags = memalloc_noreclaim_save();
    blk_start_plug(&plug);
    if (!set_mm_walk(core::ptr::null_mut(), true)) {
    err = -ENOMEM;
// goto;
    }
    next = buf;
    next[len] = '\0';
    while ((cur = strsep(&next, ",;\n"))) {
    let mut n = 0;
    let mut end = 0;
    char cmd, swap_string[5];
    let mut memcg_id = 0;
    let mut nid = 0;
    let mut seq = 0;
    let mut swappiness = 0;
pub static mut opt: c_ulong = 0;
    cur = skip_spaces(cur);
    if (!*cur) {
    continue;
    }
    n = sscanf(cur, "%c %llu %u %lu %n %4s %n %lu %n", &cmd, &memcg_id, &nid,
    &seq, &end, swap_string, &end, &opt, &end);
    if (n < 4 || cur[end]) {
    err = -EINVAL;
    break;
    }
    if (n == 4) {
    swappiness = -1;
    } else if (!strcmp("max", swap_string)) {
// set by userspace for anonymous memory only
    swappiness = SWAPPINESS_ANON_ONLY;
    } else {
    err = kstrtouint(swap_string, 0, &swappiness);
    if (err) {
    break;
    }
    }
    err = run_cmd(cmd, memcg_id, nid, seq, &sc, swappiness, opt);
    if (err) {
    break;
    }
    }
// label;
    clear_mm_walk();
    blk_finish_plug(&plug);
    memalloc_noreclaim_restore(flags);
    set_task_reclaim_state(current, core::ptr::null_mut());
    kvfree(buf);
    return err ? : len;
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_seq_open(inode: *mut inode, file: *mut file) -> c_int {
    return seq_open(file, &lru_gen_seq_ops);
    }
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
//
// initialization
//
#[no_mangle]
pub unsafe extern "C" fn lru_gen_init_pgdat(pgdat: *mut pglist_data) {
    let mut i = 0;
    let mut j = 0;
    spin_lock_init(&pgdat.memcg_lru.lock);
    while (i < MEMCG_NR_GENS) {
    for (j = 0; j < MEMCG_NR_BINS; j++) {
    INIT_HLIST_NULLS_HEAD(&pgdat.memcg_lru.fifo[i][j], i);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_init_lruvec(lruvec: *mut lruvec) {
    let mut i = 0;
    let mut gen = 0;
    let mut type = 0;
    let mut zone = 0;
    let mut lrugen = &lruvec.lrugen;
    let mut mm_state = get_mm_state(lruvec);
    lrugen.max_seq = MIN_NR_GENS + 1;
    lrugen.enabled = lru_gen_enabled();
    for (i = 0; i <= MIN_NR_GENS + 1; i++) {
    lrugen.timestamps[i] = jiffies;
    }
    for_each_gen_type_zone(gen, type, zone) {
    INIT_LIST_HEAD(&lrugen.folios[gen][type][zone]);
    }
    if (mm_state) {
    mm_state.seq = MIN_NR_GENS;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn lru_gen_init_memcg(memcg: *mut mem_cgroup) {
    let mut mm_list = get_mm_list(memcg);
    if (!mm_list) {
    return;
    }
    INIT_LIST_HEAD(&mm_list.fifo);
    spin_lock_init(&mm_list.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn lru_gen_exit_memcg(memcg: *mut mem_cgroup) {
    let mut i = 0;
    let mut nid = 0;
    let mut mm_list = get_mm_list(memcg);
    VM_WARN_ON_ONCE(mm_list && !list_empty(&mm_list.fifo));
    for_each_node(nid) {
    let mut lruvec = get_lruvec(memcg, nid);
    let mut mm_state = get_mm_state(lruvec);
    VM_WARN_ON_ONCE(memchr_inv(lruvec.lrugen.nr_pages, 0,
    sizeof!(lruvec.lrugen.nr_pages)));
    lruvec.lrugen.list.next = LIST_POISON1;
    if (!mm_state) {
    continue;
    }
    while (i < NR_BLOOM_FILTERS) {
    bitmap_free(mm_state.filters[i]);
    mm_state.filters[i] = core::ptr::null_mut();
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn init_lru_gen() -> c_int {
    BUILD_BUG_ON!(MIN_NR_GENS + 1 >= MAX_NR_GENS);
    BUILD_BUG_ON!(BIT(LRU_GEN_WIDTH) <= MAX_NR_GENS);
    if (sysfs_create_group(mm_kobj, &lru_gen_attr_group)) {
    pr_err!("lru_gen: failed to create sysfs group\n");
    }
    debugfs_create_file_aux_num("lru_gen", 0644, core::ptr::null_mut(), core::ptr::null_mut(), false,
    &lru_gen_rw_fops);
    debugfs_create_file_aux_num("lru_gen_full", 0444, core::ptr::null_mut(), core::ptr::null_mut(), true,
    &lru_gen_ro_fops);
    return 0;
    };
    late_initcall!(init_lru_gen);

#[no_mangle]
unsafe extern "C" fn lru_gen_age_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
    BUILD_BUG();
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_shrink_lruvec(lruvec: *mut lruvec, sc: *mut scan_control) {
    BUILD_BUG();
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_shrink_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
    BUILD_BUG();
    }

#[no_mangle]
unsafe extern "C" fn shrink_lruvec(lruvec: *mut lruvec, sc: *mut scan_control) {
    unsigned long nr[NR_LRU_LISTS];
    unsigned long targets[NR_LRU_LISTS];
    let mut nr_to_scan = 0;
    enum lru_list lru;
pub static mut nr_reclaimed: c_ulong = 0;
pub static mut nr_to_reclaim: c_ulong = 0;
    let mut proportional_reclaim = 0;
pub static mut plug: usize = 0;
    if ((lru_gen_enabled() || lru_gen_switching()) && !root_reclaim(sc)) {
    lru_gen_shrink_lruvec(lruvec, sc);
    if (!lru_gen_switching()) {
    return;
    }
    }
    get_scan_count(lruvec, sc, nr);
// Record the original scan target for proportional adjustments later
    memcpy(targets, nr, sizeof!(nr));
//
// Global reclaiming within direct reclaim at DEF_PRIORITY is a normal
// event that can occur when there is little memory pressure e.g.
// multiple streaming readers/writers. Hence, we do not abort scanning
// when the requested number of pages are reclaimed when scanning at
// DEF_PRIORITY on the assumption that the fact we are direct
// reclaiming implies that kswapd is not keeping up and it is best to
// do a batch of work at once. For memcg reclaim one check is made to
// abort proportional reclaim if either the file or anon lru has already
// dropped to zero at the first pass.
//
    proportional_reclaim = (!cgroup_reclaim(sc) && !current_is_kswapd() &&
    sc.priority == DEF_PRIORITY);
    blk_start_plug(&plug);
    while (nr[LRU_INACTIVE_ANON] || nr[LRU_ACTIVE_FILE] ||
    nr[LRU_INACTIVE_FILE]) {
    unsigned long nr_anon, nr_file, percentage;
    let mut nr_scanned = 0;
    for_each_evictable_lru(lru) {
    if (nr[lru]) {
    nr_to_scan = min(nr[lru], SWAP_CLUSTER_MAX);
    nr[lru] -= nr_to_scan;
    nr_reclaimed += shrink_list(lru, nr_to_scan,
    lruvec, sc);
    }
    }
    cond_resched_tasks_rcu_qs();
    if (nr_reclaimed < nr_to_reclaim || proportional_reclaim) {
    continue;
    }
//
// For kswapd and memcg, reclaim at least the number of pages
// requested. Ensure that the anon and file LRUs are scanned
// proportionally what was requested by get_scan_count(). We
// stop reclaiming one LRU and reduce the amount scanning
// proportional to the original scan target.
//
    nr_file = nr[LRU_INACTIVE_FILE] + nr[LRU_ACTIVE_FILE];
    nr_anon = nr[LRU_INACTIVE_ANON] + nr[LRU_ACTIVE_ANON];
//
// It's just vindictive to attack the larger once the smaller
// has gone to zero.  And given the way we stop scanning the
// smaller below, this makes sure that we only make one nudge
// towards proportionality once we've got nr_to_reclaim.
//
    if (!nr_file || !nr_anon) {
    break;
    }
    if (nr_file > nr_anon) {
    let mut scan_target = targets[LRU_INACTIVE_ANON] +
    targets[LRU_ACTIVE_ANON] + 1;
    lru = LRU_BASE;
    percentage = nr_anon * 100 / scan_target;
    } else {
    let mut scan_target = targets[LRU_INACTIVE_FILE] +
    targets[LRU_ACTIVE_FILE] + 1;
    lru = LRU_FILE;
    percentage = nr_file * 100 / scan_target;
    }
// Stop scanning the smaller of the LRU
    nr[lru] = 0;
    nr[lru + LRU_ACTIVE] = 0;
//
// Recalculate the other LRU scan count based on its original
// scan target and the percentage scanning already complete
//
    lru = (lru == LRU_FILE) ? LRU_BASE : LRU_FILE;
    nr_scanned = targets[lru] - nr[lru];
    nr[lru] = targets[lru] * (100 - percentage) / 100;
    nr[lru] -= min(nr[lru], nr_scanned);
    lru += LRU_ACTIVE;
    nr_scanned = targets[lru] - nr[lru];
    nr[lru] = targets[lru] * (100 - percentage) / 100;
    nr[lru] -= min(nr[lru], nr_scanned);
    }
    blk_finish_plug(&plug);
    sc.nr_reclaimed += nr_reclaimed;
//
// Even if we did not try to evict anon pages at all, we want to
// rebalance the anon lru active/inactive ratio.
//
    if (can_age_anon_pages(lruvec, sc) &&
    inactive_is_low(lruvec, LRU_INACTIVE_ANON)) {
    shrink_active_list(SWAP_CLUSTER_MAX, lruvec,
    sc, LRU_ACTIVE_ANON);
    }
    }
// Use reclaim/compaction for costly allocs or under memory pressure
#[no_mangle]
unsafe extern "C" fn in_reclaim_compaction(sc: *mut scan_control) -> bool {
    if (gfp_compaction_allowed(sc.gfp_mask) && sc.order &&
    (sc.order > PAGE_ALLOC_COSTLY_ORDER ||
    sc.priority < DEF_PRIORITY - 2)) {
    return true;
    }
    return false;
    }
//
// Reclaim/compaction is used for high-order allocation requests. It reclaims
// order-0 pages before compacting the zone. should_continue_reclaim() returns
// true if more pages should be reclaimed such that when the page allocator
// calls try_to_compact_pages() that it will have enough free pages to succeed.
// It will give up earlier than that if there is difficulty reclaiming pages.
//
#[no_mangle]
pub unsafe extern "C" fn should_continue_reclaim(pgdat: *mut pglist_data, nr_reclaimed: c_ulong, sc: *mut scan_control) -> bool {
    let mut pages_for_compaction = 0;
    let mut inactive_lru_pages = 0;
    let mut z = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
// If not in reclaim/compaction mode, stop
    if (!in_reclaim_compaction(sc)) {
    return false;
    }
//
// Stop if we failed to reclaim any pages from the last SWAP_CLUSTER_MAX
// number of pages that were scanned. This will return to the caller
// with the risk reclaim/compaction and the resulting allocation attempt
// fails. In the past we have tried harder for __GFP_RETRY_MAYFAIL
// allocations through requiring that the full LRU list has been scanned
// first, by assuming that zero delta of sc->nr_scanned means full LRU
// scan, but that approximation was wrong, and there were corner cases
// where always a non-zero amount of pages were scanned.
//
    if (!nr_reclaimed) {
    return false;
    }
// If compaction would go ahead or the allocation would succeed, stop
    for_each_managed_zone_pgdat(zone, pgdat, z, sc.reclaim_idx) {
pub static mut watermark: c_ulong = 0;
// Allocation can already succeed, nothing to do
    if (zone_watermark_ok(zone, sc.order, watermark,
    sc.reclaim_idx, 0)) {
    return false;
    }
    if (compaction_suitable(zone, sc.order, watermark,
    sc.reclaim_idx)) {
    return false;
    }
    }
//
// If we have not reclaimed enough pages for compaction and the
// inactive lists are large enough, continue reclaiming
//
    pages_for_compaction = compact_gap(sc.order);
    inactive_lru_pages = node_page_state(pgdat, NR_INACTIVE_FILE);
    if (can_reclaim_anon_pages(core::ptr::null_mut(), pgdat.node_id, sc)) {
    inactive_lru_pages += node_page_state(pgdat, NR_INACTIVE_ANON);
    }
    return inactive_lru_pages > pages_for_compaction;
    }
#[no_mangle]
unsafe extern "C" fn shrink_node_memcgs(pgdat: *mut pg_data_t, sc: *mut scan_control) {
    let mut target_memcg = sc.target_mem_cgroup;
pub static mut mem_cgroup_reclaim_cookie: usize = 0;
    let mut partial = &reclaim;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
//
// In most cases, direct reclaimers can do partial walks
// through the cgroup tree, using an iterator state that
// persists across invocations. This strikes a balance between
// fairness and allocation latency.
//
// For kswapd, reliable forward progress is more important
// than a quick return to idle. Always do full walks.
//
    if (current_is_kswapd() || sc.memcg_full_walk) {
    partial = core::ptr::null_mut();
    }
    memcg = mem_cgroup_iter(target_memcg, core::ptr::null_mut(), partial);
    do {
    let mut lruvec = mem_cgroup_lruvec(memcg, pgdat);
    let mut reclaimed = 0;
    let mut scanned = 0;
//
// This loop can become CPU-bound when target memcgs
// aren't eligible for reclaim - either because they
// don't have any reclaimable pages, or because their
// memory is explicitly protected. Avoid soft lockups.
//
    cond_resched();
    mem_cgroup_calculate_protection(target_memcg, memcg);
    if (mem_cgroup_below_min(target_memcg, memcg)) {
//
// Hard protection.
// If there is no reclaimable memory, OOM.
//
    continue;
    } else if (mem_cgroup_below_low(target_memcg, memcg)) {
//
// Soft protection.
// Respect the protection only as long as
// there is an unprotected supply
// of reclaimable memory from other cgroups.
//
    if (!sc.memcg_low_reclaim) {
    sc.memcg_low_skipped = 1;
    continue;
    }
    memcg_memory_event(memcg, MEMCG_LOW);
    }
    reclaimed = sc.nr_reclaimed;
    scanned = sc.nr_scanned;
    shrink_lruvec(lruvec, sc);
    shrink_slab(sc.gfp_mask, pgdat.node_id, memcg,
    sc.priority);
// Record the group's reclaim efficiency
    if (!sc.proactive) {
    vmpressure(sc.gfp_mask, sc.order, memcg, false,
    sc.nr_scanned - scanned,
    sc.nr_reclaimed - reclaimed);
    }
// If partial walks are allowed, bail once goal is reached
    if (partial && sc.nr_reclaimed >= sc.nr_to_reclaim) {
    mem_cgroup_iter_break(target_memcg, memcg);
    break;
    }
    } while ((memcg = mem_cgroup_iter(target_memcg, memcg, partial)));
    }
#[no_mangle]
unsafe extern "C" fn shrink_node(pgdat: *mut pg_data_t, sc: *mut scan_control) {
    unsigned long nr_reclaimed, nr_scanned, nr_node_reclaimed;
pub static mut target_lruvec: *mut c_void = core::ptr::null_mut();
pub static mut reclaimable: bool = false;
    if ((lru_gen_enabled() || lru_gen_switching()) && root_reclaim(sc)) {
    memset(&sc.nr, 0, sizeof!(sc.nr));
    lru_gen_shrink_node(pgdat, sc);
    if (!lru_gen_switching()) {
    return;
    }
    }
    target_lruvec = mem_cgroup_lruvec(sc.target_mem_cgroup, pgdat);
// label;
    memset(&sc.nr, 0, sizeof!(sc.nr));
    nr_reclaimed = sc.nr_reclaimed;
    nr_scanned = sc.nr_scanned;
    prepare_scan_control(pgdat, sc);
    shrink_node_memcgs(pgdat, sc);
    flush_reclaim_state(sc);
    nr_node_reclaimed = sc.nr_reclaimed - nr_reclaimed;
// Record the subtree's reclaim efficiency
    if (!sc.proactive) {
    vmpressure(sc.gfp_mask, sc.order, sc.target_mem_cgroup, true,
    sc.nr_scanned - nr_scanned, nr_node_reclaimed);
    }
    if (nr_node_reclaimed) {
    reclaimable = true;
    }
    if (current_is_kswapd()) {
//
// If reclaim is isolating dirty pages under writeback,
// it implies that the long-lived page allocation rate
// is exceeding the page laundering rate. Either the
// global limits are not being effective at throttling
// processes due to the page distribution throughout
// zones or there is heavy usage of a slow backing
// device. The only option is to throttle from reclaim
// context which is not ideal as there is no guarantee
// the dirtying process is throttled in the same way
// balance_dirty_pages() manages.
//
// Once a node is flagged PGDAT_WRITEBACK, kswapd will
// count the number of pages under pages flagged for
// immediate reclaim and stall if any are encountered
// in the nr_immediate check below.
//
    if (sc.nr.writeback && sc.nr.writeback == sc.nr.taken) {
    set_bit(PGDAT_WRITEBACK, &pgdat.flags);
    }
//
// If kswapd scans pages marked for immediate
// reclaim and under writeback (nr_immediate), it
// implies that pages are cycling through the LRU
// faster than they are written so forcibly stall
// until some pages complete writeback.
//
    if (sc.nr.immediate) {
    reclaim_throttle(pgdat, VMSCAN_THROTTLE_WRITEBACK);
    }
    }
//
// Tag a node/memcg as congested if all the dirty pages were marked
// for writeback and immediate reclaim (counted in nr.congested).
//
// Legacy memcg will stall in page writeback so avoid forcibly
// stalling in reclaim_throttle().
//
    if (sc.nr.dirty && sc.nr.dirty == sc.nr.congested) {
    if (cgroup_reclaim(sc) && writeback_throttling_sane(sc)) {
    set_bit(LRUVEC_CGROUP_CONGESTED, &target_lruvec.flags);
    }
    if (current_is_kswapd()) {
    set_bit(LRUVEC_NODE_CONGESTED, &target_lruvec.flags);
    }
    }
//
// Stall direct reclaim for IO completions if the lruvec is
// node is congested. Allow kswapd to continue until it
// starts encountering unqueued dirty pages or cycling through
// the LRU too quickly.
//
    if (!current_is_kswapd() && current_may_throttle() &&
    !sc.hibernation_mode &&
    (test_bit(LRUVEC_CGROUP_CONGESTED, &target_lruvec.flags) ||
    test_bit(LRUVEC_NODE_CONGESTED, &target_lruvec.flags))) {
    reclaim_throttle(pgdat, VMSCAN_THROTTLE_CONGESTED);
    }
    if (should_continue_reclaim(pgdat, nr_node_reclaimed, sc)) {
// goto;
    }
//
// Kswapd gives up on balancing particular nodes after too
// many failures to reclaim anything from them and goes to
// sleep. On reclaim progress, reset the failure counter. A
// successful direct reclaim run will revive a dormant kswapd.
//
    if (reclaimable) {
    kswapd_try_clear_hopeless(pgdat, sc.order, sc.reclaim_idx);
    }

    else if (sc.cache_trim_mode) {
    sc.cache_trim_mode_failed = 1;
    }
    }
//
// Returns true if compaction should go ahead for a costly-order request, or
// the allocation would already succeed without compaction. Return false if we
// should reclaim first.
//
#[no_mangle]
pub unsafe extern "C" fn compaction_ready(zone: *mut zone, sc: *mut scan_control) -> bool {
    let mut watermark = 0;
    if (!gfp_compaction_allowed(sc.gfp_mask)) {
    return false;
    }
// Allocation can already succeed, nothing to do
    if (zone_watermark_ok(zone, sc.order, min_wmark_pages(zone),
    sc.reclaim_idx, 0)) {
    return true;
    }
//
// Direct reclaim usually targets the min watermark, but compaction
// takes time to run and there are potentially other callers using the
// pages just freed. So target a higher buffer to give compaction a
// reasonable chance of completing and allocating the pages.
//
// Note that we won't actually reclaim the whole buffer in one attempt
// as the target watermark in should_continue_reclaim() is lower. But if
// we are already above the high+gap watermark, don't reclaim at all.
//
    watermark = high_wmark_pages(zone);
    if (compaction_suitable(zone, sc.order, watermark, sc.reclaim_idx)) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn consider_reclaim_throttle(pgdat: *mut pg_data_t, sc: *mut scan_control) {
//
// If reclaim is making progress greater than 12% efficiency then
// wake all the NOPROGRESS throttled tasks.
//
    if (sc.nr_reclaimed > (sc.nr_scanned >> 3)) {
pub static mut wqh: *mut c_void = core::ptr::null_mut();
    wqh = &pgdat.reclaim_wait[VMSCAN_THROTTLE_NOPROGRESS];
    if (waitqueue_active(wqh)) {
    wake_up(wqh);
    }
    return;
    }
//
// Do not throttle kswapd or cgroup reclaim on NOPROGRESS as it will
// throttle on VMSCAN_THROTTLE_WRITEBACK if there are too many pages
// under writeback and marked for immediate reclaim at the tail of the
// LRU.
//
    if (current_is_kswapd() || cgroup_reclaim(sc)) {
    return;
    }
// Throttle if making no progress at high priorities.
    if (sc.priority == 1 && !sc.nr_reclaimed) {
    reclaim_throttle(pgdat, VMSCAN_THROTTLE_NOPROGRESS);
    }
    }
//
// This is the direct reclaim path, for page-allocating processes.  We only
// try to reclaim pages from zones which will satisfy the caller's allocation
// request.
//
// If a zone is deemed to be full of pinned pages then just give it a light
// scan then give up on it.
//
#[no_mangle]
unsafe extern "C" fn shrink_zones(zonelist: *mut zonelist, sc: *mut scan_control) {
pub static mut z: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut nr_soft_reclaimed = 0;
    let mut nr_soft_scanned = 0;
    let mut orig_mask;
    let mut last_pgdat = core::ptr::null_mut();
    let mut first_pgdat = core::ptr::null_mut();
//
// If the number of buffer_heads in the machine exceeds the maximum
// allowed level, force direct reclaim to scan the highmem zone as
// highmem pages could be pinning lowmem pages storing buffer_heads
//
    orig_mask = sc.gfp_mask;
    if (buffer_heads_over_limit) {
    sc.gfp_mask |= __GFP_HIGHMEM;
    sc.reclaim_idx = gfp_zone(sc.gfp_mask);
    }
    for_each_zone_zonelist_nodemask(zone, z, zonelist,
    sc.reclaim_idx, sc.nodemask) {
//
// Take care memory controller reclaiming has small influence
// to global LRU.
//
    if (!cgroup_reclaim(sc)) {
    if (!cpuset_zone_allowed(zone,
    GFP_KERNEL | __GFP_HARDWALL)) {
    continue;
    }
//
// If we already have plenty of memory free for
// compaction in this zone, don't free any more.
// Even though compaction is invoked for any
// non-zero order, only frequent costly order
// reclamation is disruptive enough to become a
// noticeable problem, like transparent huge
// page allocations.
//
    if (IS_ENABLED!(CONFIG_COMPACTION) &&
    sc.order > PAGE_ALLOC_COSTLY_ORDER &&
    compaction_ready(zone, sc)) {
    sc.compaction_ready = true;
    continue;
    }
//
// Shrink each node in the zonelist once. If the
// zonelist is ordered by zone (not the default) then a
// node may be shrunk multiple times but in that case
// the user prefers lower zones being preserved.
//
    if (zone.zone_pgdat == last_pgdat) {
    continue;
    }
//
// This steals pages from memory cgroups over softlimit
// and returns the number of reclaimed pages and
// scanned pages. This works for global memory pressure
// and balancing, not for a memcg's limit.
//
    nr_soft_scanned = 0;
    nr_soft_reclaimed = memcg1_soft_limit_reclaim(zone.zone_pgdat,
    sc.order, sc.gfp_mask,
    &nr_soft_scanned);
    sc.nr_reclaimed += nr_soft_reclaimed;
    sc.nr_scanned += nr_soft_scanned;
// need some check for avoid more shrink_zone()
    }
    if (!first_pgdat) {
    first_pgdat = zone.zone_pgdat;
    }
// See comment about same check for global reclaim above
    if (zone.zone_pgdat == last_pgdat) {
    continue;
    }
    last_pgdat = zone.zone_pgdat;
    shrink_node(zone.zone_pgdat, sc);
    }
    if (first_pgdat) {
    consider_reclaim_throttle(first_pgdat, sc);
    }
//
// Restore to original mask to avoid the impact on the caller if we
// promoted it to __GFP_HIGHMEM.
//
    sc.gfp_mask = orig_mask;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_refaults(target_memcg: *mut mem_cgroup, pgdat: *mut pg_data_t) {
pub static mut target_lruvec: *mut c_void = core::ptr::null_mut();
    let mut refaults = 0;
    if (lru_gen_enabled() && !lru_gen_switching()) {
    return;
    }
    target_lruvec = mem_cgroup_lruvec(target_memcg, pgdat);
    refaults = lruvec_page_state(target_lruvec, WORKINGSET_ACTIVATE_ANON);
    target_lruvec.refaults[WORKINGSET_ANON] = refaults;
    refaults = lruvec_page_state(target_lruvec, WORKINGSET_ACTIVATE_FILE);
    target_lruvec.refaults[WORKINGSET_FILE] = refaults;
    }
//
// This is the main entry point to direct page reclaim.
//
// If a full scan of the inactive list fails to free enough memory then we
// are "out of memory" and something needs to be killed.
//
// If the caller is !__GFP_FS then the probability of a failure is reasonably
// high - the zone may be full of dirty or under-writeback pages, which this
// caller can't do much about.  We kick the writeback threads and take explicit
// naps in the hope that some of these pages can be written.  But if the
// allocating task holds filesystem locks which prevent writeout this might not
// work, and the allocation attempt will fail.
//
// returns:	0, if no pages reclaimed
// else, the number of pages reclaimed
//
#[no_mangle]
pub unsafe extern "C" fn do_try_to_free_pages(zonelist: *mut zonelist, sc: *mut scan_control) -> c_ulong {
pub static mut initial_priority: c_int = 0;
pub static mut last_pgdat: *mut c_void = core::ptr::null_mut();
pub static mut z: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
// label;
    delayacct_freepages_start();
    if (!cgroup_reclaim(sc)) {
    __count_zid_vm_events(ALLOCSTALL, sc.reclaim_idx, 1);
    }
    do {
    if (!sc.proactive) {
    vmpressure_prio(sc.gfp_mask, sc.target_mem_cgroup,
    sc.priority);
    }
    sc.nr_scanned = 0;
    shrink_zones(zonelist, sc);
    if (sc.nr_reclaimed >= sc.nr_to_reclaim) {
    break;
    }
    if (sc.compaction_ready) {
    break;
    }
    } while (--sc.priority >= 0);
    last_pgdat = core::ptr::null_mut();
    for_each_zone_zonelist_nodemask(zone, z, zonelist, sc.reclaim_idx,
    sc.nodemask) {
    if (zone.zone_pgdat == last_pgdat) {
    continue;
    }
    last_pgdat = zone.zone_pgdat;
    snapshot_refaults(sc.target_mem_cgroup, zone.zone_pgdat);
    if (cgroup_reclaim(sc)) {
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    lruvec = mem_cgroup_lruvec(sc.target_mem_cgroup,
    zone.zone_pgdat);
    clear_bit(LRUVEC_CGROUP_CONGESTED, &lruvec.flags);
    }
    }
    delayacct_freepages_end();
    if (sc.nr_reclaimed) {
    return sc.nr_reclaimed;
    }
// Aborted reclaim to try compaction? don't OOM, then
    if (sc.compaction_ready) {
    return 1;
    }
//
// In most cases, direct reclaimers can do partial walks
// through the cgroup tree to meet the reclaim goal while
// keeping latency low. Since the iterator state is shared
// among all direct reclaim invocations (to retain fairness
// among cgroups), though, high concurrency can result in
// individual threads not seeing enough cgroups to make
// meaningful forward progress. Avoid false OOMs in this case.
//
    if (!sc.memcg_full_walk) {
    sc.priority = initial_priority;
    sc.memcg_full_walk = 1;
// goto;
    }
//
// We make inactive:active ratio decisions based on the node's
// composition of memory, but a restrictive reclaim_idx or a
// memory.low cgroup setting can exempt large amounts of
// memory from reclaim. Neither of which are very common, so
// instead of doing costly eligibility calculations of the
// entire cgroup subtree up front, we assume the estimates are
// good, and retry with forcible deactivation if that fails.
//
    if (sc.skipped_deactivate) {
    sc.priority = initial_priority;
    sc.force_deactivate = 1;
    sc.skipped_deactivate = 0;
// goto;
    }
// Untapped cgroup reserves?  Don't OOM, retry.
    if (sc.memcg_low_skipped) {
    sc.priority = initial_priority;
    sc.force_deactivate = 0;
    sc.memcg_low_reclaim = 1;
    sc.memcg_low_skipped = 0;
// goto;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn allow_direct_reclaim(pgdat: *mut pg_data_t) -> bool {
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut pfmemalloc_reserve: c_ulong = 0;
pub static mut free_pages: c_ulong = 0;
    let mut i = 0;
    let mut wmark_ok = 0;
    if (kswapd_test_hopeless(pgdat)) {
    return true;
    }
    for_each_managed_zone_pgdat(zone, pgdat, i, ZONE_NORMAL) {
    if (!zone_reclaimable_pages(zone) && zone_page_state_snapshot(zone, NR_FREE_PAGES)) {
    continue;
    }
    pfmemalloc_reserve += min_wmark_pages(zone);
    free_pages += zone_page_state_snapshot(zone, NR_FREE_PAGES);
    }
// If there are no reserves (unexpected config) then do not throttle
    if (!pfmemalloc_reserve) {
    return true;
    }
    wmark_ok = free_pages > pfmemalloc_reserve / 2;
// kswapd must be awake if processes are being throttled
    if (!wmark_ok && waitqueue_active(&pgdat.kswapd_wait)) {
    if (READ_ONCE(pgdat.kswapd_highest_zoneidx) > ZONE_NORMAL) {
    WRITE_ONCE(pgdat.kswapd_highest_zoneidx, ZONE_NORMAL);
    }
    wake_up_interruptible(&pgdat.kswapd_wait);
    }
    return wmark_ok;
    }
//
// Throttle direct reclaimers if backing storage is backed by the network
// and the PFMEMALLOC reserve for the preferred node is getting dangerously
// depleted. kswapd will continue to make progress and wake the processes
// when the low watermark is reached.
//
// Returns true if a fatal signal was delivered during throttling. If this
// happens, the page allocator should not consider triggering the OOM killer.
//
#[no_mangle]
pub unsafe extern "C" fn throttle_direct_reclaim(gfp_mask: gfp_t, zonelist: *mut zonelist, nodemask: *mut nodemask_t) -> bool {
pub static mut z: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut pgdat = core::ptr::null_mut();
//
// Kernel threads should not be throttled as they may be indirectly
// responsible for cleaning pages necessary for reclaim to make forward
// progress. kjournald for example may enter direct reclaim while
// committing a transaction where throttling it could forcing other
// processes to block on log_wait_commit().
//
    if (current.flags & PF_KTHREAD) {
// goto;
    }
//
// If a fatal signal is pending, this process should not throttle.
// It should return quickly so it can exit and free its memory
//
    if (fatal_signal_pending(current)) {
// goto;
    }
//
// Check if the pfmemalloc reserves are ok by finding the first node
// with a usable ZONE_NORMAL or lower zone. The expectation is that
// GFP_KERNEL will be required for allocating network buffers when
// swapping over the network so ZONE_HIGHMEM is unusable.
//
// Throttling is based on the first usable node and throttled processes
// wait on a queue until kswapd makes progress and wakes them. There
// is an affinity then between processes waking up and where reclaim
// progress has been made assuming the process wakes on the same node.
// More importantly, processes running on remote nodes will not compete
// for remote pfmemalloc reserves and processes on different nodes
// should make reasonable progress.
//
    for_each_zone_zonelist_nodemask(zone, z, zonelist,
    gfp_zone(gfp_mask), nodemask) {
    if (zone_idx(zone) > ZONE_NORMAL) {
    continue;
    }
// Throttle based on the first usable node
    pgdat = zone.zone_pgdat;
    if (allow_direct_reclaim(pgdat)) {
// goto;
    }
    break;
    }
// If no zone was usable by the allocation flags then do not throttle
    if (!pgdat) {
// goto;
    }
// Account for the throttling
    count_vm_event(PGSCAN_DIRECT_THROTTLE);
//
// If the caller cannot enter the filesystem, it's possible that it
// is due to the caller holding an FS lock or performing a journal
// transaction in the case of a filesystem like ext[3|4]. In this case,
// it is not safe to block on pfmemalloc_wait as kswapd could be
// blocked waiting on the same lock. Instead, throttle for up to a
// second before continuing.
//
    if (!(gfp_mask & __GFP_FS)) {
    wait_event_interruptible_timeout(pgdat.pfmemalloc_wait,
    allow_direct_reclaim(pgdat), HZ);
    }
    else {
// Throttle until kswapd wakes the process
    wait_event_killable(zone.zone_pgdat.pfmemalloc_wait,
    allow_direct_reclaim(pgdat));
    }
    if (fatal_signal_pending(current)) {
    return true;
    }
// label;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn try_to_free_pages(zonelist: *mut zonelist, order: c_int, gfp_mask: gfp_t, nodemask: *mut nodemask_t) -> c_ulong {
    let mut nr_reclaimed = 0;
pub static mut scan_control: usize = 0;
//
// scan_control uses s8 fields for order, priority, and reclaim_idx.
// Confirm they are large enough for max values.
//
    BUILD_BUG_ON!(MAX_PAGE_ORDER >= S8_MAX);
    BUILD_BUG_ON!(DEF_PRIORITY > S8_MAX);
    BUILD_BUG_ON!(MAX_NR_ZONES > S8_MAX);
//
// Do not enter reclaim if fatal signal was delivered while throttled.
// 1 is returned so that the page allocator does not OOM kill at this
// point.
//
    if (throttle_direct_reclaim(sc.gfp_mask, zonelist, nodemask)) {
    return 1;
    }
    set_task_reclaim_state(current, &sc.reclaim_state);
    trace_mm_vmscan_direct_reclaim_begin(sc.gfp_mask, order, core::ptr::null_mut());
    nr_reclaimed = do_try_to_free_pages(zonelist, &sc);
    trace_mm_vmscan_direct_reclaim_end(nr_reclaimed, core::ptr::null_mut());
    set_task_reclaim_state(current, core::ptr::null_mut());
    return nr_reclaimed;
    }

// Only used by soft limit reclaim. Do not reuse for anything else.
#[no_mangle]
pub unsafe extern "C" fn mem_cgroup_shrink_node(memcg: *mut mem_cgroup, gfp_mask: gfp_t, noswap: bool, pgdat: *mut pg_data_t, nr_scanned: *mut c_ulong) -> c_ulong {
    let mut lruvec = mem_cgroup_lruvec(memcg, pgdat);
pub static mut scan_control: usize = 0;
    WARN_ON_ONCE!(!current.reclaim_state);
    sc.gfp_mask = (gfp_mask & GFP_RECLAIM_MASK) |
    (GFP_HIGHUSER_MOVABLE & ~GFP_RECLAIM_MASK);
    trace_mm_vmscan_memcg_softlimit_reclaim_begin(sc.gfp_mask,
    sc.order,
    memcg);
//
// NOTE: Although we can get the priority field, using it
// here is not a good idea, since it limits the pages we can scan.
// if we don't reclaim here, the shrink_node from balance_pgdat
// will pick up pages from other mem cgroup's as well. We hack
// the priority and make it zero.
//
    shrink_lruvec(lruvec, &sc);
    trace_mm_vmscan_memcg_softlimit_reclaim_end(sc.nr_reclaimed, memcg);
// nr_scanned = sc.nr_scanned;
    return sc.nr_reclaimed;
    }
#[no_mangle]
pub unsafe extern "C" fn try_to_free_mem_cgroup_pages(memcg: *mut mem_cgroup, nr_pages: c_ulong, gfp_mask: gfp_t, reclaim_options: c_uint, swappiness: *mut c_int) -> c_ulong {
    let mut nr_reclaimed = 0;
    let mut noreclaim_flag = 0;
pub static mut scan_control: usize = 0;
//
// Traverse the ZONELIST_FALLBACK zonelist of the current node to put
// equal pressure on all the nodes. This is based on the assumption that
// the reclaim does not bail out early.
//
    let mut zonelist = node_zonelist(numa_node_id(), sc.gfp_mask);
    set_task_reclaim_state(current, &sc.reclaim_state);
    trace_mm_vmscan_memcg_reclaim_begin(sc.gfp_mask, 0, memcg);
    noreclaim_flag = memalloc_noreclaim_save();
    nr_reclaimed = do_try_to_free_pages(zonelist, &sc);
    memalloc_noreclaim_restore(noreclaim_flag);
    trace_mm_vmscan_memcg_reclaim_end(nr_reclaimed, memcg);
    set_task_reclaim_state(current, core::ptr::null_mut());
    return nr_reclaimed;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: try_to_free_mem_cgroup_pages
pub unsafe extern "C" fn try_to_free_mem_cgroup_pages_dup(memcg: *mut mem_cgroup, nr_pages: c_ulong, gfp_mask: gfp_t, reclaim_options: c_uint, swappiness: *mut c_int) -> c_ulong {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn kswapd_age_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    if (lru_gen_enabled() || lru_gen_switching()) {
    lru_gen_age_node(pgdat, sc);
    if (!lru_gen_switching()) {
    return;
    }
    }
    lruvec = mem_cgroup_lruvec(core::ptr::null_mut(), pgdat);
    if (!can_age_anon_pages(lruvec, sc)) {
    return;
    }
    if (!inactive_is_low(lruvec, LRU_INACTIVE_ANON)) {
    return;
    }
    memcg = mem_cgroup_iter(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    do {
    lruvec = mem_cgroup_lruvec(memcg, pgdat);
    shrink_active_list(SWAP_CLUSTER_MAX, lruvec,
    sc, LRU_ACTIVE_ANON);
    memcg = mem_cgroup_iter(core::ptr::null_mut(), memcg, core::ptr::null_mut());
    } while (memcg);
    }
#[no_mangle]
unsafe extern "C" fn pgdat_watermark_boosted(pgdat: *mut pg_data_t, highest_zoneidx: c_int) -> bool {
    let mut i = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
//
// Check for watermark boosts top-down as the higher zones
// are more likely to be boosted. Both watermarks and boosts
// should not be checked at the same time as reclaim would
// start prematurely when there is no boosting and a lower
// zone is balanced.
//
    while (i >= 0) {
    zone = pgdat.node_zones + i;
    if (!managed_zone(zone)) {
    continue;
    }
    if (zone.watermark_boost) {
    return true;
    }
    }
    return false;
    }
//
// Returns true if there is an eligible zone balanced for the request order
// and highest_zoneidx
//
#[no_mangle]
unsafe extern "C" fn pgdat_balanced(pgdat: *mut pg_data_t, order: c_int, highest_zoneidx: c_int) -> bool {
    let mut i = 0;
pub static mut mark: c_ulong = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
//
// Check watermarks bottom-up as lower zones are more likely to
// meet watermarks.
//
    for_each_managed_zone_pgdat(zone, pgdat, i, highest_zoneidx) {
    enum zone_stat_item item;
    let mut free_pages = 0;
    if (sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING) {
    mark = promo_wmark_pages(zone);
    }
    else {
    mark = high_wmark_pages(zone);
    }
//
// In defrag_mode, watermarks must be met in whole
// blocks to avoid polluting allocator fallbacks.
//
// However, kswapd usually cannot accomplish this on
// its own and needs kcompactd support. Once it's
// reclaimed a compaction gap, and kswapd_shrink_node
// has dropped order, simply ensure there are enough
// base pages for compaction, wake kcompactd & sleep.
//
    if (defrag_mode && order) {
    item = NR_FREE_PAGES_BLOCKS;
    }
    else {
    item = NR_FREE_PAGES;
    }
//
// When there is a high number of CPUs in the system,
// the cumulative error from the vmstat per-cpu cache
// can blur the line between the watermarks. In that
// case, be safe and get an accurate snapshot.
//
// TODO: NR_FREE_PAGES_BLOCKS moves in steps of
// pageblock_nr_pages, while the vmstat pcp threshold
// is limited to 125. On many configurations that
// counter won't actually be per-cpu cached. But keep
// things simple for now; revisit when somebody cares.
//
    free_pages = zone_page_state(zone, item);
    if (zone.percpu_drift_mark && free_pages < zone.percpu_drift_mark) {
    free_pages = zone_page_state_snapshot(zone, item);
    }
    if (__zone_watermark_ok(zone, order, mark, highest_zoneidx,
    0, free_pages)) {
    return true;
    }
    }
//
// If a node has no managed zone within highest_zoneidx, it does not
// need balancing by definition. This can happen if a zone-restricted
// allocation tries to wake a remote kswapd.
//
    if (mark == -1) {
    return true;
    }
    return false;
    }
// Clear pgdat state for congested, dirty or under writeback.
#[no_mangle]
unsafe extern "C" fn clear_pgdat_congested(pgdat: *mut pg_data_t) {
    let mut lruvec = mem_cgroup_lruvec(core::ptr::null_mut(), pgdat);
    clear_bit(LRUVEC_NODE_CONGESTED, &lruvec.flags);
    clear_bit(LRUVEC_CGROUP_CONGESTED, &lruvec.flags);
    clear_bit(PGDAT_WRITEBACK, &pgdat.flags);
    }
//
// Prepare kswapd for sleeping. This verifies that there are no processes
// waiting in throttle_direct_reclaim() and that watermarks have been met.
//
// Returns true if kswapd is ready to sleep
//
#[no_mangle]
pub unsafe extern "C" fn prepare_kswapd_sleep(pgdat: *mut pg_data_t, order: c_int, highest_zoneidx: c_int) -> bool {
//
// The throttled processes are normally woken up in balance_pgdat() as
// soon as allow_direct_reclaim() is true. But there is a potential
// race between when kswapd checks the watermarks and a process gets
// throttled. There is also a potential race if processes get
// throttled, kswapd wakes, a large process exits thereby balancing the
// zones, which causes kswapd to exit balance_pgdat() before reaching
// the wake up checks. If kswapd is going to sleep, no process should
// be sleeping on pfmemalloc_wait, so wake them now if necessary. If
// the wake up is premature, processes will wake kswapd and get
// throttled again. The difference from wake ups in balance_pgdat() is
// that here we are under prepare_to_wait().
//
    if (waitqueue_active(&pgdat.pfmemalloc_wait)) {
    wake_up_all(&pgdat.pfmemalloc_wait);
    }
// Hopeless node, leave it to direct reclaim
    if (kswapd_test_hopeless(pgdat)) {
    return true;
    }
    if (pgdat_balanced(pgdat, order, highest_zoneidx)) {
    clear_pgdat_congested(pgdat);
    return true;
    }
    return false;
    }
//
// kswapd shrinks a node of pages that are at or below the highest usable
// zone that is currently unbalanced.
//
// Returns true if kswapd scanned at least the requested number of pages to
// reclaim or if the lack of progress was due to pages under writeback.
// This is used to determine if the scanning priority needs to be raised.
//
#[no_mangle]
pub unsafe extern "C" fn kswapd_shrink_node(pgdat: *mut pg_data_t, sc: *mut scan_control) -> bool {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut z = 0;
pub static mut nr_reclaimed: c_ulong = 0;
// Reclaim a number of pages proportional to the number of zones
    sc.nr_to_reclaim = 0;
    for_each_managed_zone_pgdat(zone, pgdat, z, sc.reclaim_idx) {
    sc.nr_to_reclaim += max(high_wmark_pages(zone), SWAP_CLUSTER_MAX);
    }
//
// Historically care was taken to put equal pressure on all zones but
// now pressure is applied based on node LRU order.
//
    shrink_node(pgdat, sc);
//
// Fragmentation may mean that the system cannot be rebalanced for
// high-order allocations. If at least the compaction gap has been
// reclaimed then recheck watermarks only at order-0 to prevent
// excessive reclaim. Assume that a process requested a high-order
// can direct reclaim/compact.
//
    if (sc.order && sc.nr_reclaimed >= compact_gap(sc.order)) {
    sc.order = 0;
    }
// account for progress from mm_account_reclaimed_pages()
    return max(sc.nr_scanned, sc.nr_reclaimed - nr_reclaimed) >= sc.nr_to_reclaim;
    }
// Page allocator PCP high watermark is lowered if reclaim is active.
#[no_mangle]
pub unsafe extern "C" fn update_reclaim_active(pgdat: *mut pg_data_t, highest_zoneidx: c_int, active: bool) {
    let mut i = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_managed_zone_pgdat(zone, pgdat, i, highest_zoneidx) {
    if (active) {
    set_bit(ZONE_RECLAIM_ACTIVE, &zone.flags);
    }
    else {
    clear_bit(ZONE_RECLAIM_ACTIVE, &zone.flags);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_reclaim_active(pgdat: *mut pg_data_t, highest_zoneidx: c_int) {
    update_reclaim_active(pgdat, highest_zoneidx, true);
    }
#[no_mangle]
pub unsafe extern "C" fn clear_reclaim_active(pgdat: *mut pg_data_t, highest_zoneidx: c_int) {
    update_reclaim_active(pgdat, highest_zoneidx, false);
    }
//
// For kswapd, balance_pgdat() will reclaim pages across a node from zones
// that are eligible for use by the caller until at least one zone is
// balanced.
//
// Returns the order kswapd finished reclaiming at.
//
// kswapd scans the zones in the highmem->normal->dma direction.  It skips
// zones which have free_pages > high_wmark_pages(zone), but once a zone is
// found to have free_pages <= high_wmark_pages(zone), any page in that zone
// or lower is eligible for reclaim until at least one usable zone is
// balanced.
//
#[no_mangle]
unsafe extern "C" fn balance_pgdat(pgdat: *mut pg_data_t, order: c_int, highest_zoneidx: c_int) -> c_int {
    let mut i = 0;
    let mut nr_soft_reclaimed = 0;
    let mut nr_soft_scanned = 0;
    let mut pflags = 0;
    let mut nr_boost_reclaim = 0;
    unsigned long zone_boosts[MAX_NR_ZONES] = { 0, };
    let mut boosted = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut scan_control: usize = 0;
    trace_mm_vmscan_balance_pgdat_begin(pgdat.node_id, order,
    highest_zoneidx);
    set_task_reclaim_state(current, &sc.reclaim_state);
    psi_memstall_enter(&pflags);
    __fs_reclaim_acquire(_THIS_IP_);
    count_vm_event(PAGEOUTRUN);
//
// Account for the reclaim boost. Note that the zone boost is left in
// place so that parallel allocations that are near the watermark will
// stall or direct reclaim until kswapd is finished.
//
    nr_boost_reclaim = 0;
    for_each_managed_zone_pgdat(zone, pgdat, i, highest_zoneidx) {
    nr_boost_reclaim += zone.watermark_boost;
    zone_boosts[i] = zone.watermark_boost;
    }
    boosted = nr_boost_reclaim;
// label;
    set_reclaim_active(pgdat, highest_zoneidx);
    sc.priority = DEF_PRIORITY;
    do {
pub static mut nr_reclaimed: c_ulong = 0;
pub static mut raise_priority: bool = true;
    let mut balanced = 0;
    let mut ret = 0;
    let mut was_frozen = 0;
    sc.reclaim_idx = highest_zoneidx;
//
// If the number of buffer_heads exceeds the maximum allowed
// then consider reclaiming from all zones. This has a dual
// purpose -- on 64-bit systems it is expected that
// buffer_heads are stripped during active rotation. On 32-bit
// systems, highmem pages can pin lowmem memory and shrinking
// buffers can relieve lowmem pressure. Reclaim may still not
// go ahead if all eligible zones for the original allocation
// request are balanced to avoid excessive reclaim from kswapd.
//
    if (buffer_heads_over_limit) {
    while (i >= 0) {
    zone = pgdat.node_zones + i;
    if (!managed_zone(zone)) {
    continue;
    }
    sc.reclaim_idx = i;
    break;
    }
    }
//
// If the pgdat is imbalanced then ignore boosting and preserve
// the watermarks for a later time and restart. Note that the
// zone watermarks will be still reset at the end of balancing
// on the grounds that the normal reclaim should be enough to
// re-evaluate if boosting is required when kswapd next wakes.
//
    balanced = pgdat_balanced(pgdat, sc.order, highest_zoneidx);
    if (!balanced && nr_boost_reclaim) {
    nr_boost_reclaim = 0;
// goto;
    }
//
// If boosting is not active then only reclaim if there are no
// eligible zones. Note that sc.reclaim_idx is not used as
// buffer_heads_over_limit may have adjusted it.
//
    if (!nr_boost_reclaim && balanced) {
// goto;
    }
// Limit the priority of boosting to avoid reclaim writeback
    if (nr_boost_reclaim && sc.priority == DEF_PRIORITY - 2) {
    raise_priority = false;
    }
//
// Do not writeback or swap pages for boosted reclaim. The
// intent is to relieve pressure not issue sub-optimal IO
// from reclaim context. If no pages are reclaimed, the
// reclaim will be aborted.
//
    sc.may_writepage = !nr_boost_reclaim;
    sc.may_swap = !nr_boost_reclaim;
//
// Do some background aging, to give pages a chance to be
// referenced before reclaiming. All pages are rotated
// regardless of classzone as this is about consistent aging.
//
    kswapd_age_node(pgdat, &sc);
// Call soft limit reclaim before calling shrink_node.
    sc.nr_scanned = 0;
    nr_soft_scanned = 0;
    nr_soft_reclaimed = memcg1_soft_limit_reclaim(pgdat, sc.order,
    sc.gfp_mask, &nr_soft_scanned);
    sc.nr_reclaimed += nr_soft_reclaimed;
//
// There should be no need to raise the scanning priority if
// enough pages are already being scanned that the high
// watermark would be met at 100% efficiency.
//
    if (kswapd_shrink_node(pgdat, &sc)) {
    raise_priority = false;
    }
//
// If the low watermark is met there is no need for processes
// to be throttled on pfmemalloc_wait as they should now be
// able to safely make forward progress. Wake them
//
    if (waitqueue_active(&pgdat.pfmemalloc_wait) &&
    allow_direct_reclaim(pgdat)) {
    wake_up_all(&pgdat.pfmemalloc_wait);
    }
// Check if kswapd should be suspending
    __fs_reclaim_release(_THIS_IP_);
    ret = kthread_freezable_should_stop(&was_frozen);
    __fs_reclaim_acquire(_THIS_IP_);
    if (was_frozen || ret) {
    break;
    }
//
// Raise priority if scanning rate is too low or there was no
// progress in reclaiming pages
//
    nr_reclaimed = sc.nr_reclaimed - nr_reclaimed;
    nr_boost_reclaim -= min(nr_boost_reclaim, nr_reclaimed);
//
// If reclaim made no progress for a boost, stop reclaim as
// IO cannot be queued and it could be an infinite loop in
// extreme circumstances.
//
    if (nr_boost_reclaim && !nr_reclaimed) {
    break;
    }
    if (raise_priority || !nr_reclaimed) {
    sc.priority -= 1;
    }
    } while (sc.priority >= 1);
//
// Restart only if it went through the priority loop all the way,
// but cache_trim_mode didn't work.
//
    if (!sc.nr_reclaimed && sc.priority < 1 &&
    !sc.no_cache_trim_mode && sc.cache_trim_mode_failed) {
    sc.no_cache_trim_mode = 1;
// goto;
    }
//
// If the reclaim was boosted, we might still be far from the
// watermark_high at this point. We need to avoid increasing the
// failure count to prevent the kswapd thread from stopping.
//
    if (!sc.nr_reclaimed && !boosted) {
pub static mut fail_cnt: c_int = 0;
// kswapd context, low overhead to trace every failure
    trace_mm_vmscan_kswapd_reclaim_fail(pgdat.node_id, fail_cnt);
    }
// label;
    clear_reclaim_active(pgdat, highest_zoneidx);
// If reclaim was boosted, account for the reclaim done in this pass
    if (boosted) {
    let mut flags = 0;
    while (i <= highest_zoneidx) {
    if (!zone_boosts[i]) {
    continue;
    }
// Increments are under the zone lock
    zone = pgdat.node_zones + i;
    spin_lock_irqsave(&zone.lock, flags);
    zone.watermark_boost -= min(zone.watermark_boost, zone_boosts[i]);
    spin_unlock_irqrestore(&zone.lock, flags);
    }
//
// As there is now likely space, wakeup kcompact to defragment
// pageblocks.
//
    wakeup_kcompactd(pgdat, pageblock_order, highest_zoneidx);
    }
    snapshot_refaults(core::ptr::null_mut(), pgdat);
    __fs_reclaim_release(_THIS_IP_);
    psi_memstall_leave(&pflags);
    set_task_reclaim_state(current, core::ptr::null_mut());
    trace_mm_vmscan_balance_pgdat_end(pgdat.node_id, sc.order,
    highest_zoneidx, sc.nr_reclaimed);
//
// Return the order kswapd stopped reclaiming at as
// prepare_kswapd_sleep() takes it into account. If another caller
// entered the allocator slow path while kswapd was awake, order will
// remain at the higher level.
//
    return sc.order;
    }
//
// The pgdat->kswapd_highest_zoneidx is used to pass the highest zone index to
// be reclaimed by kswapd from the waker. If the value is MAX_NR_ZONES which is
// not a valid index then either kswapd runs for first time or kswapd couldn't
// sleep after previous reclaim attempt (node is still unbalanced). In that
// case return the zone index of the previous kswapd reclaim cycle.
//
    static enum zone_type kswapd_highest_zoneidx(pg_data_t *pgdat,
    enum zone_type prev_highest_zoneidx)
    {
pub static mut curr_idx: zone_type = 0;
pub static mut curr_idx: return = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kswapd_try_to_sleep(pgdat: *mut pg_data_t, alloc_order: c_int, reclaim_order: c_int, highest_zoneidx: c_uint) {
pub static mut remaining: c_long = 0;
pub static mut wait: usize = 0;
    if (freezing(current) || kthread_should_stop()) {
    return;
    }
    prepare_to_wait(&pgdat.kswapd_wait, &wait, TASK_INTERRUPTIBLE);
//
// Try to sleep for a short interval. Note that kcompactd will only be
// woken if it is possible to sleep for a short interval. This is
// deliberate on the assumption that if reclaim cannot keep an
// eligible zone balanced that it's also unlikely that compaction will
// succeed.
//
    if (prepare_kswapd_sleep(pgdat, reclaim_order, highest_zoneidx)) {
//
// Compaction records what page blocks it recently failed to
// isolate pages from and skips them in the future scanning.
// When kswapd is going to sleep, it is reasonable to assume
// that pages and compaction may succeed so reset the cache.
//
    reset_isolation_suitable(pgdat);
//
// We have freed the memory, now we should compact it to make
// allocation of the requested order possible.
//
    wakeup_kcompactd(pgdat, alloc_order, highest_zoneidx);
    remaining = schedule_timeout(HZ/10);
//
// If woken prematurely then reset kswapd_highest_zoneidx and
// order. The values will either be from a wakeup request or
// the previous request that slept prematurely.
//
    if (remaining) {
    WRITE_ONCE(pgdat.kswapd_highest_zoneidx,
    kswapd_highest_zoneidx(pgdat,
    highest_zoneidx));
    if (READ_ONCE(pgdat.kswapd_order) < reclaim_order) {
    WRITE_ONCE(pgdat.kswapd_order, reclaim_order);
    }
    }
    finish_wait(&pgdat.kswapd_wait, &wait);
    prepare_to_wait(&pgdat.kswapd_wait, &wait, TASK_INTERRUPTIBLE);
    }
//
// After a short sleep, check if it was a premature sleep. If not, then
// go fully to sleep until explicitly woken up.
//
    if (!remaining &&
    prepare_kswapd_sleep(pgdat, reclaim_order, highest_zoneidx)) {
    trace_mm_vmscan_kswapd_sleep(pgdat.node_id);
//
// vmstat counters are not perfectly accurate and the estimated
// value for counters such as NR_FREE_PAGES can deviate from the
// true value by nr_online_cpus * threshold. To avoid the zone
// watermarks being breached while under pressure, we reduce the
// per-cpu vmstat threshold while kswapd is awake and restore
// them before going back to sleep.
//
    set_pgdat_percpu_threshold(pgdat, calculate_normal_threshold);
    if (!kthread_should_stop()) {
    schedule();
    }
    set_pgdat_percpu_threshold(pgdat, calculate_pressure_threshold);
    } else {
    if (remaining) {
    count_vm_event(KSWAPD_LOW_WMARK_HIT_QUICKLY);
    }
    else {
    count_vm_event(KSWAPD_HIGH_WMARK_HIT_QUICKLY);
    }
    }
    finish_wait(&pgdat.kswapd_wait, &wait);
    }
//
// The background pageout daemon, started as a kernel thread
// from the init process.
//
// This basically trickles out pages so that we have _some_
// free memory available even if there is no other activity
// that frees anything up. This is needed for things like routing
// etc, where we otherwise might have all activity going on in
// asynchronous contexts that cannot page things out.
//
// If there are applications that are active memory-allocators
// (most normal use), this basically shouldn't matter.
//
#[no_mangle]
unsafe extern "C" fn kswapd(p: *mut c_void) -> c_int {
    let mut alloc_order = 0;
    let mut reclaim_order = 0;
pub static mut highest_zoneidx: c_uint = 0;
    let mut pgdat = p;
    let mut tsk = current;
//
// Tell the memory management that we're a "memory allocator",
// and that if we need more memory we should get access to it
// regardless (see "__alloc_pages()"). "kswapd" should
// never get caught in the normal page freeing logic.
//
// (Kswapd normally doesn't need memory anyway, but sometimes
// you need a small amount of memory in order to be able to
// page out something else, and this flag essentially protects
// us from recursively trying to free more memory as we're
// trying to free the first piece of memory in the first place).
//
    tsk.flags |= PF_MEMALLOC | PF_KSWAPD;
    set_freezable();
    WRITE_ONCE(pgdat.kswapd_order, 0);
    WRITE_ONCE(pgdat.kswapd_highest_zoneidx, MAX_NR_ZONES);
    atomic_set(&pgdat.nr_writeback_throttled, 0);
    while ( ) {
    let mut was_frozen = 0;
    alloc_order = reclaim_order = READ_ONCE(pgdat.kswapd_order);
    highest_zoneidx = kswapd_highest_zoneidx(pgdat,
    highest_zoneidx);
// label;
    kswapd_try_to_sleep(pgdat, alloc_order, reclaim_order,
    highest_zoneidx);
// Read the new order and highest_zoneidx
    alloc_order = READ_ONCE(pgdat.kswapd_order);
    highest_zoneidx = kswapd_highest_zoneidx(pgdat,
    highest_zoneidx);
    WRITE_ONCE(pgdat.kswapd_order, 0);
    WRITE_ONCE(pgdat.kswapd_highest_zoneidx, MAX_NR_ZONES);
    if (kthread_freezable_should_stop(&was_frozen)) {
    break;
    }
//
// We can speed up thawing tasks if we don't call balance_pgdat
// after returning from the refrigerator
//
    if (was_frozen) {
    continue;
    }
//
// Reclaim begins at the requested order but if a high-order
// reclaim fails then kswapd falls back to reclaiming for
// order-0. If that happens, kswapd will consider sleeping
// for the order it finished reclaiming at (reclaim_order)
// but kcompactd is woken to compact for the original
// request (alloc_order).
//
    trace_mm_vmscan_kswapd_wake(pgdat.node_id, highest_zoneidx,
    alloc_order);
    reclaim_order = balance_pgdat(pgdat, alloc_order,
    highest_zoneidx);
    if (reclaim_order < alloc_order) {
// goto;
    }
    }
    tsk.flags &= ~(PF_MEMALLOC | PF_KSWAPD);
    return 0;
    }
//
// A zone is low on free memory or too fragmented for high-order memory.  If
// kswapd should reclaim (direct reclaim is deferred), wake it up for the zone's
// pgdat.  It will wake up kcompactd after reclaiming memory.  If kswapd reclaim
// has failed or is not needed, still wake up kcompactd if only compaction is
// needed.
//
#[no_mangle]
pub unsafe extern "C" fn wakeup_kswapd(zone: *mut zone, gfp_flags: gfp_t, order: c_int, highest_zoneidx: zone_type) {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    enum zone_type curr_idx;
    if (!managed_zone(zone)) {
    return;
    }
    if (!cpuset_zone_allowed(zone, gfp_flags)) {
    return;
    }
    pgdat = zone.zone_pgdat;
    curr_idx = READ_ONCE(pgdat.kswapd_highest_zoneidx);
    if (curr_idx == MAX_NR_ZONES || curr_idx < highest_zoneidx) {
    WRITE_ONCE(pgdat.kswapd_highest_zoneidx, highest_zoneidx);
    }
    if (READ_ONCE(pgdat.kswapd_order) < order) {
    WRITE_ONCE(pgdat.kswapd_order, order);
    }
    if (!waitqueue_active(&pgdat.kswapd_wait)) {
    return;
    }
// Hopeless node, leave it to direct reclaim if possible
    if (kswapd_test_hopeless(pgdat) ||
    (pgdat_balanced(pgdat, order, highest_zoneidx) &&
    !pgdat_watermark_boosted(pgdat, highest_zoneidx))) {
//
// There may be plenty of free memory available, but it's too
// fragmented for high-order allocations.  Wake up kcompactd
// and rely on compaction_suitable() to determine if it's
// needed.  If it fails, it will defer subsequent attempts to
// ratelimit its work.
//
    if (!(gfp_flags & __GFP_DIRECT_RECLAIM)) {
    wakeup_kcompactd(pgdat, order, highest_zoneidx);
    }
    return;
    }
    trace_mm_vmscan_wakeup_kswapd(pgdat.node_id, highest_zoneidx, order,
    gfp_flags);
    wake_up_interruptible(&pgdat.kswapd_wait);
    }
#[no_mangle]
pub unsafe extern "C" fn kswapd_clear_hopeless(pgdat: *mut pg_data_t, reason: kswapd_clear_hopeless_reason) {
// Only trace actual resets, not redundant zero-to-zero
    if (atomic_xchg(&pgdat.kswapd_failures, 0)) {
    trace_mm_vmscan_kswapd_clear_hopeless(pgdat.node_id, reason);
    }
    }
//
// Reset kswapd_failures only when the node is balanced. Without this
// check, successful direct reclaim (e.g., from cgroup memory.high
// throttling) can keep resetting kswapd_failures even when the node
// cannot be balanced, causing kswapd to run endlessly.
//
#[no_mangle]
pub unsafe extern "C" fn kswapd_try_clear_hopeless(pgdat: *mut pglist_data, order: c_uint, highest_zoneidx: c_int) {
    if (pgdat_balanced(pgdat, order, highest_zoneidx)) {
    kswapd_clear_hopeless(pgdat, current_is_kswapd() ?
    KSWAPD_CLEAR_HOPELESS_KSWAPD : KSWAPD_CLEAR_HOPELESS_DIRECT);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kswapd_test_hopeless(pgdat: *mut pg_data_t) -> bool {
    return atomic_read(&pgdat.kswapd_failures) >= MAX_RECLAIM_RETRIES;
    }

//
// Try to free `nr_to_reclaim' of memory, system-wide, and return the number of
// freed pages.
//
// Rather than trying to age LRUs the aim is to preserve the overall
// LRU order by reclaiming preferentially
// inactive > active > active referenced > active mapped
//
#[no_mangle]
pub unsafe extern "C" fn shrink_all_memory(nr_to_reclaim: c_ulong) -> c_ulong {
pub static mut scan_control: usize = 0;
    let mut zonelist = node_zonelist(numa_node_id(), sc.gfp_mask);
    let mut nr_reclaimed = 0;
    let mut noreclaim_flag = 0;
    fs_reclaim_acquire(sc.gfp_mask);
    noreclaim_flag = memalloc_noreclaim_save();
    set_task_reclaim_state(current, &sc.reclaim_state);
    nr_reclaimed = do_try_to_free_pages(zonelist, &sc);
    set_task_reclaim_state(current, core::ptr::null_mut());
    memalloc_noreclaim_restore(noreclaim_flag);
    fs_reclaim_release(sc.gfp_mask);
    return nr_reclaimed;
    }

//
// This kswapd start function will be called by init and node-hot-add.
//
#[no_mangle]
pub unsafe extern "C" fn kswapd_run(nid: c_int) -> void __meminit {
    let mut pgdat = NODE_DATA(nid);
    pgdat_kswapd_lock(pgdat);
    if (!pgdat.kswapd) {
    pgdat.kswapd = kthread_create_on_node(kswapd, pgdat, nid, "kswapd%d", nid);
    if (IS_ERR(pgdat.kswapd)) {
// failure at boot is fatal
    pr_err!("Failed to start kswapd on node %d, ret=%pe\n",
    nid, pgdat.kswapd);
    BUG_ON!(system_state < SYSTEM_RUNNING);
    pgdat.kswapd = core::ptr::null_mut();
    } else {
    wake_up_process(pgdat.kswapd);
    }
    }
    pgdat_kswapd_unlock(pgdat);
    }
//
// Called by memory hotplug when all memory in a node is offlined.  Caller must
// be holding mem_hotplug_begin/done().
//
#[no_mangle]
pub unsafe extern "C" fn kswapd_stop(nid: c_int) -> void __meminit {
    let mut pgdat = NODE_DATA(nid);
pub static mut kswapd: *mut c_void = core::ptr::null_mut();
    pgdat_kswapd_lock(pgdat);
    kswapd = pgdat.kswapd;
    if (kswapd) {
    kthread_stop(kswapd);
    pgdat.kswapd = core::ptr::null_mut();
    }
    pgdat_kswapd_unlock(pgdat);
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn kswapd_init() -> c_int {
    let mut nid = 0;
    for_each_node_state(nid, N_MEMORY) {
    kswapd_run(nid);
    }
    register_sysctl_init("vm", vmscan_sysctl_table);
    return 0;
    }
    module_init!(kswapd_init)

//
// Node reclaim mode
//
// If non-zero call node_reclaim when the number of free pages falls below
// the watermarks.
//
    let mut node_reclaim_mode = 0;
//
// Priority for NODE_RECLAIM. This determines the fraction of pages
// of a node considered for each zone_reclaim. 4 scans 1/16th of
// a zone.
//
pub const NODE_RECLAIM_PRIORITY: c_int = 4;
//
// Percentage of pages in a zone that must be unmapped for node_reclaim to
// occur.
//
pub static mut sysctl_min_unmapped_ratio: c_int = 1;
//
// If the number of slab pages in a zone grows beyond this percentage then
// slab reclaim needs to occur.
//
pub static mut sysctl_min_slab_ratio: c_int = 5;
#[no_mangle]
pub unsafe extern "C" fn node_unmapped_file_pages(pgdat: *mut pglist_data) -> c_ulong {
pub static mut file_mapped: c_ulong = 0;
    let mut file_lru = node_page_state(pgdat, NR_INACTIVE_FILE) +
    node_page_state(pgdat, NR_ACTIVE_FILE);
//
// It's possible for there to be more file mapped pages than
// accounted for by the pages on the file LRU lists because
// tmpfs pages accounted for as ANON can also be FILE_MAPPED
//
    return (file_lru > file_mapped) ? (file_lru - file_mapped) : 0;
    }
// Work out how many page cache pages we can reclaim in this reclaim_mode
#[no_mangle]
unsafe extern "C" fn node_pagecache_reclaimable(pgdat: *mut pglist_data) -> c_ulong {
    let mut nr_pagecache_reclaimable = 0;
pub static mut delta: c_ulong = 0;
//
// If RECLAIM_UNMAP is set, then all file pages are considered
// potentially reclaimable. Otherwise, we have to worry about
// pages like swapcache and node_unmapped_file_pages() provides
// a better estimate
//
    if (node_reclaim_mode & RECLAIM_UNMAP) {
    nr_pagecache_reclaimable = node_page_state(pgdat, NR_FILE_PAGES);
    }
    else {
    nr_pagecache_reclaimable = node_unmapped_file_pages(pgdat);
    }
//
// Since we can't clean folios through reclaim, remove dirty file
// folios from consideration.
//
    delta += node_page_state(pgdat, NR_FILE_DIRTY);
// Watch for any possible underflows due to delta
    if (unlikely(delta > nr_pagecache_reclaimable)) {
    delta = nr_pagecache_reclaimable;
    }
    return nr_pagecache_reclaimable - delta;
    }
//
// Try to free up some pages from this node through reclaim.
//
#[no_mangle]
pub unsafe extern "C" fn __node_reclaim(pgdat: *mut pglist_data, nr_pages: c_ulong, sc: *mut scan_control) -> c_ulong {
    let mut p = current;
    let mut noreclaim_flag = 0;
    let mut pflags = 0;
    trace_mm_vmscan_node_reclaim_begin(pgdat.node_id, sc.order,
    sc.gfp_mask);
    cond_resched();
    psi_memstall_enter(&pflags);
    delayacct_freepages_start();
    fs_reclaim_acquire(sc.gfp_mask);
//
// We need to be able to allocate from the reserves for RECLAIM_UNMAP
//
    noreclaim_flag = memalloc_noreclaim_save();
    set_task_reclaim_state(p, &sc.reclaim_state);
    if (node_pagecache_reclaimable(pgdat) > pgdat.min_unmapped_pages ||
    node_page_state_pages(pgdat, NR_SLAB_RECLAIMABLE_B) > pgdat.min_slab_pages) {
//
// Free memory by calling shrink node with increasing
// priorities until we have enough memory freed.
//
    do {
    shrink_node(pgdat, sc);
    } while (sc.nr_reclaimed < nr_pages && --sc.priority >= 0);
    }
    set_task_reclaim_state(p, core::ptr::null_mut());
    memalloc_noreclaim_restore(noreclaim_flag);
    fs_reclaim_release(sc.gfp_mask);
    delayacct_freepages_end();
    psi_memstall_leave(&pflags);
    trace_mm_vmscan_node_reclaim_end(sc.nr_reclaimed, core::ptr::null_mut());
    return sc.nr_reclaimed;
    }
#[no_mangle]
pub unsafe extern "C" fn node_reclaim(pgdat: *mut pglist_data, gfp_mask: gfp_t, order: c_uint) -> c_ulong {
    let mut ret = 0;
// Minimum pages needed in order to stay on node
pub static mut nr_pages: c_ulong = 0;
pub static mut scan_control: usize = 0;
//
// Node reclaim reclaims unmapped file backed pages and
// slab pages if we are over the defined limits.
//
// A small portion of unmapped file backed pages is needed for
// file I/O otherwise pages read by file I/O will be immediately
// thrown out if the node is overallocated. So we do not reclaim
// if less than a specified percentage of the node is used by
// unmapped file backed pages.
//
    if (node_pagecache_reclaimable(pgdat) <= pgdat.min_unmapped_pages &&
    node_page_state_pages(pgdat, NR_SLAB_RECLAIMABLE_B) <=
    pgdat.min_slab_pages) {
    return 0;
    }
//
// Do not scan if the allocation should not be delayed.
//
    if (!gfpflags_allow_blocking(gfp_mask) || (current.flags & PF_MEMALLOC)) {
    return 0;
    }
//
// Only run node reclaim on the local node or on nodes that do not
// have associated processors. This will favor the local processor
// over remote processors and spread off node memory allocations
// as wide as possible.
//
    if (node_state(pgdat.node_id, N_CPU) && pgdat.node_id != numa_node_id()) {
    return 0;
    }
    if (test_and_set_bit_lock(PGDAT_RECLAIM_LOCKED, &pgdat.flags)) {
    return 0;
    }
    ret = __node_reclaim(pgdat, nr_pages, &sc);
    clear_bit_unlock(PGDAT_RECLAIM_LOCKED, &pgdat.flags);
    if (ret >= nr_pages) {
    count_vm_event(PGSCAN_ZONE_RECLAIM_SUCCESS);
    }
    else {
    count_vm_event(PGSCAN_ZONE_RECLAIM_FAILED);
    }
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __node_reclaim
pub unsafe extern "C" fn __node_reclaim_dup(pgdat: *mut pglist_data, nr_pages: c_ulong, sc: *mut scan_control) -> c_ulong {
    return 0;
    }

    enum {
    MEMORY_RECLAIM_SWAPPINESS = 0,
    MEMORY_RECLAIM_SWAPPINESS_MAX,
    MEMORY_RECLAIM_NULL,
    };
    static const match_table_t tokens = {
    { MEMORY_RECLAIM_SWAPPINESS, "swappiness=%d"},
    { MEMORY_RECLAIM_SWAPPINESS_MAX, "swappiness=max"},
    { MEMORY_RECLAIM_NULL, core::ptr::null_mut() },
    };
#[no_mangle]
pub unsafe extern "C" fn user_proactive_reclaim(buf: *mut c_char, memcg: *mut mem_cgroup, pgdat: *mut pg_data_t) -> c_int {
pub static mut nr_retries: c_uint = 0;
    unsigned long nr_to_reclaim, nr_reclaimed = 0;
pub static mut swappiness: c_int = 0;
    let mut old_buf = core::ptr::null_mut();
    let mut start = core::ptr::null_mut();
    substring_t args[MAX_OPT_ARGS];
pub static mut gfp_mask: gfp_t = 0;
    if (!buf || (!memcg && !pgdat) || (memcg && pgdat)) {
    return -EINVAL;
    }
    buf = strstrip(buf);
    old_buf = buf;
    nr_to_reclaim = memparse(buf, &buf) / PAGE_SIZE;
    if (buf == old_buf) {
    return -EINVAL;
    }
    buf = strstrip(buf);
    while ((start = strsep(&buf, " ")) != core::ptr::null_mut()) {
    if (!strlen(start)) {
    continue;
    }
    switch (match_token(start, tokens, args)) {
    case MEMORY_RECLAIM_SWAPPINESS:
    if (match_int(&args[0], &swappiness)) {
    return -EINVAL;
    }
    if (swappiness < MIN_SWAPPINESS ||
    swappiness > MAX_SWAPPINESS) {
    return -EINVAL;
    }
    break;
    case MEMORY_RECLAIM_SWAPPINESS_MAX:
    swappiness = SWAPPINESS_ANON_ONLY;
    break;
// label;
    return -EINVAL;
    }
    }
    while (nr_reclaimed < nr_to_reclaim) {
// Will converge on zero, but reclaim enforces a minimum
pub static mut batch_size: c_ulong = 0;
    let mut reclaimed = 0;
//
// Return -ERESTARTSYS to allow the freezer to interrupt the
// task. The syscall will be transparently restarted upon
// resume. For real signals, it either restarts the syscall
// (if SA_RESTART is set) or is converted to -EINTR by the
// signal layer.
//
    if (signal_pending(current)) {
    return -ERESTARTSYS;
    }
// cgroup_rmdir() waits for us with cgroup_mutex held.
    if (memcg && memcg_is_dying(memcg)) {
    return -EAGAIN;
    }
//
// This is the final attempt, drain percpu lru caches in the
// hope of introducing more evictable pages.
//
    if (!nr_retries) {
    lru_add_drain_all();
    }
    if (memcg) {
    let mut reclaim_options = 0;
    reclaim_options = MEMCG_RECLAIM_MAY_SWAP |
    MEMCG_RECLAIM_PROACTIVE;
    reclaimed = try_to_free_mem_cgroup_pages(memcg,
    batch_size, gfp_mask,
    reclaim_options,
    swappiness == -1 ? core::ptr::null_mut() : &swappiness);
    } else {
pub static mut scan_control: usize = 0;
    if (test_and_set_bit_lock(PGDAT_RECLAIM_LOCKED,
    &pgdat.flags)) {
    return -EBUSY;
    }
    reclaimed = __node_reclaim(pgdat, batch_size, &sc);
    clear_bit_unlock(PGDAT_RECLAIM_LOCKED, &pgdat.flags);
    }
    if (!reclaimed && !nr_retries--) {
    return -EAGAIN;
    }
    nr_reclaimed += reclaimed;
    }
    return 0;
    }
//
// check_move_unevictable_folios - Move evictable folios to appropriate zone
// lru list
// @fbatch: Batch of lru folios to check.
//
// Checks folios for evictability, if an evictable folio is in the unevictable
// lru list, moves it to the appropriate evictable lru list. This function
// should be only used for lru folios.
//
#[no_mangle]
pub unsafe extern "C" fn check_move_unevictable_folios(fbatch: *mut folio_batch) {
    let mut lruvec = core::ptr::null_mut();
pub static mut pgscanned: c_int = 0;
pub static mut pgrescued: c_int = 0;
    let mut i = 0;
    while (i < fbatch.nr) {
    let mut folio = fbatch.folios[i];
pub static mut nr_pages: c_int = 0;
    pgscanned += nr_pages;
// block memcg migration while the folio moves between lrus
    if (!folio_test_clear_lru(folio)) {
    continue;
    }
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
    if (folio_evictable(folio) && folio_test_unevictable(folio)) {
    lruvec_del_folio(lruvec, folio);
    folio_clear_unevictable(folio);
    lruvec_add_folio(lruvec, folio);
    pgrescued += nr_pages;
    }
    folio_set_lru(folio);
    }
    if (lruvec) {
    __count_vm_events(UNEVICTABLE_PGRESCUED, pgrescued);
    __count_vm_events(UNEVICTABLE_PGSCANNED, pgscanned);
    lruvec_unlock_irq(lruvec);
    } else if (pgscanned) {
    count_vm_events(UNEVICTABLE_PGSCANNED, pgscanned);
    }
    }
    EXPORT_SYMBOL_GPL(check_move_unevictable_folios);

#[no_mangle]
pub unsafe extern "C" fn reclaim_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    int ret, nid = dev.id;
    ret = user_proactive_reclaim(buf, core::ptr::null_mut(), NODE_DATA(nid));
    return ret ? ret : count;
    }
    static DEVICE_ATTR_WO(reclaim);
#[no_mangle]
pub unsafe extern "C" fn reclaim_register_node(node: *mut node) -> c_int {
    return device_create_file(&node.dev, &dev_attr_reclaim);
    }
#[no_mangle]
pub unsafe extern "C" fn reclaim_unregister_node(node: *mut node) {
    return device_remove_file(&node.dev, &dev_attr_reclaim);
    }