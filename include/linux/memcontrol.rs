//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memcontrol.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// memcontrol.h - Memory Controller
//
// Copyright IBM Corporation, 2007
// Author Balbir Singh <balbir@linux.vnet.ibm.com>
//
// Copyright 2007 OpenVZ SWsoft Inc
// Author: Pavel Emelianov <xemul@openvz.org>
//

// Cgroup-specific page state, on top of universal node page state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memcg_stat_item {
    MEMCG_SWAP = NR_VM_NODE_STAT_ITEMS,
    MEMCG_SOCK,
    MEMCG_PERCPU_B,
    MEMCG_KMEM,
    MEMCG_ZSWAP_B,
    MEMCG_ZSWAPPED,
    MEMCG_ZSWAP_INCOMP,
    MEMCG_NR_STAT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memcg_memory_event {
    MEMCG_LOW,
    MEMCG_HIGH,
    MEMCG_MAX,
    MEMCG_OOM,
    MEMCG_OOM_KILL,
    MEMCG_OOM_GROUP_KILL,
    MEMCG_SWAP_HIGH,
    MEMCG_SWAP_MAX,
    MEMCG_SWAP_FAIL,
    MEMCG_SOCK_THROTTLED,
    MEMCG_NR_MEMORY_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_reclaim_cookie {
    pub pgdat: *mut pg_data_t,
    pub generation: c_int,
}

pub const MEM_CGROUP_ID_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_private_id {
    pub id: c_int,
    pub ref: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_reclaim_iter {
    pub position: *mut mem_cgroup,
// scan generation, increased every round-trip
    pub generation: core::sync::atomic::AtomicI32,
}

//
// per-node information in memory controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_per_node {
// Keep the read-only fields at the start
    pub /: *mut *mut *mut mem_cgroup memcg; / Back pointer, we cannot,
// use container_of
    pub lruvec_stats_percpu: *mut lruvec_stats_percpu __percpu,
    pub lruvec_stats: *mut lruvec_stats,
    pub shrinker_info: *mut shrinker_info __rcu,

//
// Memcg-v1 only stuff in middle as buffer between read mostly fields
// and update often fields to avoid false sharing. If v1 stuff is
// not present, an explicit padding is needed.
//
    pub /: *mut *mut rb_node tree_node; / RB tree node,
    pub /: *mut *mut unsigned long usage_in_excess;/ Set to the value by which,
// the soft limit is exceeded
    pub on_tree: bool,

// Fields which get updated often at the end.
    pub lruvec: lruvec,
    pub lru_zone_size: [c_ulong; MAX_NR_ZONES][NR_LRU_LISTS],
    pub iter: mem_cgroup_reclaim_iter,
//
// objcg is wiped out as a part of the objcg repaprenting process.
// orig_objcg preserves a pointer (and a reference) to the original
// objcg until the end of live of memcg.
//
    pub objcg: *mut obj_cgroup __rcu,
    pub orig_objcg: *mut obj_cgroup,
// list of inherited objcgs, protected by objcg_lock
    pub objcg_list: list_head,

// slab stats for nmi context
    pub slab_reclaimable: core::sync::atomic::AtomicI32,
    pub slab_unreclaimable: core::sync::atomic::AtomicI32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_threshold {
    pub eventfd: *mut eventfd_ctx,
    pub threshold: c_ulong,
}

// For threshold
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_threshold_ary {
// An array index points to threshold just below or equal to usage.
    pub current_threshold: c_int,
// Size of entries[]
    pub size: c_uint,
// Array of thresholds
    pub __counted_by(size): mem_cgroup_threshold entries[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup_thresholds {
// Primary thresholds array
    pub primary: *mut mem_cgroup_threshold_ary,
//
// Spare threshold array.
// This is needed to make mem_cgroup_unregister_event() "never fail".
// It must be able to store at least primary->size - 1 entries.
//
    pub spare: *mut mem_cgroup_threshold_ary,
}

//
// Remember four most recent foreign writebacks with dirty pages in this
// cgroup.  Inode sharing is expected to be uncommon and, even if we miss
// one in a given round, we're likely to catch it later if it keeps
// foreign-dirtying, so a fairly low count should be enough.
//
// See mem_cgroup_track_foreign_dirty_slowpath() for details.
//
pub const MEMCG_CGWB_FRN_CNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memcg_cgwb_frn {
    pub /: *mut *mut u64 bdi_id; / bdi->id of the foreign inode,
    pub /: *mut *mut int memcg_id; / memcg->css.id of foreign inode,
    pub /: *mut *mut u64 at; / jiffies_64 at the time of dirtying,
    pub /: *mut *mut wb_completion done; / tracks in-flight foreign writebacks,
}

//
// Bucket for arbitrarily byte-sized objects charged to a memory
// cgroup. The bucket can be reparented in one piece when the cgroup
// is destroyed, without having to round up the individual references
// of all live memory objects in the wild.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct obj_cgroup {
    pub refcnt: percpu_ref,
    pub memcg: *mut mem_cgroup,
    pub nr_charged_bytes: core::sync::atomic::AtomicI32,
    pub /: *mut *mut list_head list; / protected by objcg_lock,
    pub rcu: rcu_head,
}

//
// The memory controller data structure. The memory controller controls both
// page cache and RSS per cgroup. We would eventually like to provide
// statistics based on the statistics developed by Rik Van Riel for clock-pro,
// to help the administrator determine what knobs to tune.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_cgroup {
    pub css: cgroup_subsys_state,
// Private memcg ID. Used to ID objects that outlive the cgroup
    pub id: mem_cgroup_private_id,
// Accounted resources
    pub /: *mut *mut page_counter memory; / Both v1 & v2,
    pub /: *mut *mut page_counter swap; / v2 only,
    pub /: *mut *mut page_counter memsw; / v1 only,
}

// registered local peak watchers
// Range enforcement for interrupt charges

//
// Prevent pages from this memcg from being written back from zswap to
// swap, and from being swapped out on zswap store failures.
//

// vmpressure notifications
//
// Should the OOM killer kill all belonging tasks, had it kill one?
//
// memory.events and memory.events.local
// handle for "memory.swap.events"
// memory.stat
// memory.events

// MEMCG_KMEM for nmi context

//
// Hint of reclaim pressure for socket memroy management. Note
// that this indicator should NOT be used in legacy cgroup mode
// where socket memory is accounted/charged separately.
//

// Keep the hot per-CPU stats pointer away from memory event counters.

// per-memcg mm_struct list

// Legacy consumer-oriented counters
// protected by memcg_oom_lock
// OOM-Killer disable
// protect arrays of thresholds
// thresholds for memory usage. RCU-protected
// thresholds for mem+swap usage. RCU-protected
// For oom notifier event fd
// Legacy tcp memory accounting
// List of events which userspace want to receive

//
// size of first charge trial.
// TODO: maybe necessary to use big numbers in big irons or dynamic based of the
// workload.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum page_memcg_data_flags {
// page->memcg_data is a pointer to an slabobj_ext vector
    MEMCG_DATA_OBJEXTS = (1UL << 0),
// page has been accounted as a non-slab kernel page
    MEMCG_DATA_KMEM = (1UL << 1),
// the next bit after the last actual flag
    __NR_MEMCG_DATA_FLAGS  = (1UL << 2),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum objext_flags {
//
// Use bit 0 with zero other bits to signal that slabobj_ext vector
// failed to allocate. The same bit 0 with valid upper bits means
// MEMCG_DATA_OBJEXTS.
//
    OBJEXTS_ALLOC_FAIL = __OBJEXTS_ALLOC_FAIL,
    __OBJEXTS_FLAG_UNUSED = __FIRST_OBJEXT_FLAG,
// the next bit after the last actual flag
    __NR_OBJEXTS_FLAGS  = (__FIRST_OBJEXT_FLAG << 1),
}

//
// After the initialization objcg->memcg is always pointing at
// a valid memcg, but can be atomically swapped to the parent memcg.
//
// The caller must ensure that the returned memcg won't be released.
//
extern "C" {
    pub fn READ_ONCE(_arg: objcg->memcg) -> return;
}
//
// folio_objcg - get the object cgroup associated with a folio.
// @folio: Pointer to the folio.
//
// Returns a pointer to the object cgroup associated with the folio,
// or NULL. This function assumes that the folio is known to have a
// proper object cgroup pointer.
//
// folio_memcg - Get the memory cgroup associated with a folio.
// @folio: Pointer to the folio.
//
// Returns a pointer to the memory cgroup associated with the folio,
// or NULL. This function assumes that the folio is known to have a
// proper memory cgroup pointer. It's not safe to call this function
// against some type of folios, e.g. slab folios or ex-slab folios.
//
// For a folio any of the following ensures folio and objcg binding stability:
//
// - the folio lock
// - LRU isolation
// - exclusive reference
//
// Based on the stable binding of folio and objcg, for a folio any of the
// following ensures folio and memcg binding stability:
//
// - cgroup_mutex
// - the lruvec lock
//
// If the caller only want to ensure that the page counters of memcg are
// updated correctly, ensure that the binding stability of folio and objcg
// is sufficient.
//
// Note: The caller should hold an rcu read lock or cgroup_mutex to protect
// memcg associated with a folio from being released.
//
// folio_memcg_charged - If a folio is charged to a memory cgroup.
// @folio: Pointer to the folio.
//
// Returns true if folio is charged to a memory cgroup, otherwise returns false.
//
// folio_memcg_check - Get the memory cgroup associated with a folio.
// @folio: Pointer to the folio.
//
// Returns a pointer to the memory cgroup associated with the folio,
// or NULL. This function unlike folio_memcg() can take any folio
// as an argument. It has to be used in cases when it's not known if a folio
// has an associated memory cgroup pointer or an object cgroups vector or
// an object cgroup.
//
// The page and objcg or memcg binding rules can refer to folio_memcg().
//
// A caller should hold an rcu read lock to protect memcg associated with a
// page from being released.
//
// Because folio->memcg_data might be changed asynchronously
// for slabs, READ_ONCE() should be used here.
//
extern "C" {
    pub fn folio_memcg_check()page: *mut (struct folio) -> return;
}
//
// folio_memcg_kmem - Check if the folio has the memcg_kmem flag set.
// @folio: Pointer to the folio.
//
// Checks if the folio has MemcgKmem flag set. The caller must ensure
// that the folio has an associated memory cgroup. It's not safe to call
// this function against some types of folios, e.g. slab folios.
//
extern "C" {
    pub fn folio_memcg_kmem(_arg: page_folio(page)) -> return;
}
//
// mem_cgroup_shrink_is_root - is this a global or root-memcg shrink invocation?
// @sc: shrink_control describing the current shrinker call
//
// Returns true when @sc represents a global reclaim shrink (sc->memcg == NULL)
// or a root-memcg shrink, i.e. not a per-memcg iteration of
// shrink_slab_memcg(). Filesystems whose ->nr_cached_objects()
// ->free_cached_objects() implementations operate on filesystem-global state
// and do not honour sc->memcg can use this to early-return 0 in per-memcg
// contexts.
//
// min = *low = *usage = 0;
// usage = page_counter_read(&memcg->memory);
//
// There is no reclaim protection applied to a targeted reclaim.
// We are special casing this specific case here because
// mem_cgroup_calculate_protection is not robust enough to keep
// the protection invariant for calculated effective values for
// parallel reclaimers with different reclaim target. This is
// especially a problem for tail memcgs (as they have pages on LRU)
// which would want to have effective values 0 for targeted reclaim
// but a different value for external reclaim.
//
// Example
// Let's have global and A's reclaim in parallel:
// |
// A (low=2G, usage = 3G, max = 3G, children_low_usage = 1.5G)
// |\
// | C (low = 1G, usage = 2.5G)
// B (low = 1G, usage = 0.5G)
//
// For the global reclaim
// A.elow = A.low
// B.elow = min(B.usage, B.low) because children_low_usage <= A.elow
// C.elow = min(C.usage, C.low)
//
// With the effective values resetting we have A reclaim
// A.elow = 0
// B.elow = B.low
// C.elow = C.low
//
// If the global reclaim races with A's reclaim then
// B.elow = C.elow = 0 because children_low_usage > A.elow)
// is possible and reclaiming B would be violating the protection.
//
// min = READ_ONCE(memcg->memory.emin);
// low = READ_ONCE(memcg->memory.elow);
//
// The root memcg doesn't account charges, and doesn't support
// protection. The target memcg's protection is ignored, see
// mem_cgroup_calculate_protection() and mem_cgroup_protection()
//
extern "C" {
    pub fn __mem_cgroup_charge(folio: *mut folio, mm: *mut mm_struct, gfp: gfp_t) -> c_int;
}
//
// mem_cgroup_charge - Charge a newly allocated folio to a cgroup.
// @folio: Folio to charge.
// @mm: mm context of the allocating task.
// @gfp: Reclaim mode.
//
// Try to charge @folio to the memcg that @mm belongs to, reclaiming
// pages according to @gfp if necessary.  If @mm is NULL, try to
// charge to the active memcg.
//
// Do not use this for folios allocated for swapin.
//
// Return: 0 on success. Otherwise, an error code is returned.
//
extern "C" {
    pub fn __mem_cgroup_charge(_arg: folio, _arg: mm, _arg: gfp) -> return;
}
extern "C" {
    pub fn mem_cgroup_charge_hugetlb(folio: *mut *mut folio, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn __mem_cgroup_uncharge(folio: *mut folio);
}
//
// mem_cgroup_uncharge - Uncharge a folio.
// @folio: Folio to uncharge.
//
// Uncharge a folio previously charged with mem_cgroup_charge().
//
extern "C" {
    pub fn __mem_cgroup_uncharge_folios(folios: *mut folio_batch);
}
extern "C" {
    pub fn mem_cgroup_replace_folio(old: *mut folio, new: *mut folio);
}
extern "C" {
    pub fn mem_cgroup_migrate(old: *mut folio, new: *mut folio);
}
//
// mem_cgroup_lruvec - get the lru list vector for a memcg & node
// @memcg: memcg of the wanted lruvec
// @pgdat: pglist_data
//
// Returns the lru list vector holding pages for a given @memcg &
// @pgdat combination. This can be the node lruvec, if the memory
// controller is disabled.
//
// Since a node can be onlined after the mem_cgroup was created,
// we have to be prepared to initialize lruvec->pgdat here;
// and if offlined then reonlined, we need to reinitialize it.
//
// folio_lruvec - return lruvec for isolating/putting an LRU folio
// @folio: Pointer to the folio.
//
// Call with rcu_read_lock() held to ensure the lifetime of the returned lruvec.
// Note that this alone will NOT guarantee the stability of the folio->lruvec
// association; the folio can be reparented to an ancestor if this races with
// cgroup deletion.
//
// Use folio_lruvec_lock() to ensure both lifetime and stability of the binding.
// Once a lruvec is locked, folio_lruvec() can be called on other folios, and
// their binding is stable if the returned lruvec matches the one the caller has
// locked. Useful for lock batching.
//
extern "C" {
    pub fn mem_cgroup_lruvec(_arg: memcg, _arg: folio_pgdat(folio)) -> return;
}
extern "C" {
    pub fn percpu_ref_tryget(_arg: &objcg->refcnt) -> return;
}

extern "C" {
    pub fn mem_cgroup_iter_break(: *mut mem_cgroup, : *mut mem_cgroup);
}
extern "C" {
    pub fn mem_cgroup_from_css(_arg: seq_css(m)) -> return;
}
//
// parent_mem_cgroup - find the accounting parent of a memcg
// @memcg: memcg whose parent to find
//
// Returns the parent memcg, or NULL if this is the root.
//
extern "C" {
    pub fn mem_cgroup_from_css(_arg: memcg->css.parent) -> return;
}
extern "C" {
    pub fn cgroup_is_descendant(_arg: memcg->css.cgroup, _arg: root->css.cgroup) -> return;
}
extern "C" {
    pub fn page_cgroup_ino(page: *mut page) -> ino_t;
}
extern "C" {
    pub fn css_is_online(_arg: &memcg->css) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: mz->lru_zone_size[zone_idx][lru]) -> return;
}
extern "C" {
    pub fn __mem_cgroup_handle_over_high(gfp_mask: gfp_t);
}
extern "C" {
    pub fn mem_cgroup_get_max(memcg: *mut mem_cgroup) -> c_ulong;
}
extern "C" {
    pub fn mem_cgroup_print_oom_meminfo(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn mem_cgroup_print_oom_group(memcg: *mut mem_cgroup);
}
// idx can be of type enum memcg_stat_item or node_stat_item
extern "C" {
    pub fn memcg_events(memcg: *mut mem_cgroup, event: c_int) -> c_ulong;
}
extern "C" {
    pub fn memcg_page_state(memcg: *mut mem_cgroup, idx: c_int) -> c_ulong;
}
extern "C" {
    pub fn memcg_page_state_output(memcg: *mut mem_cgroup, item: c_int) -> c_ulong;
}
extern "C" {
    pub fn memcg_stat_item_valid(idx: c_int) -> bool;
}
extern "C" {
    pub fn memcg_vm_event_item_valid(idx: vm_event_item) -> bool;
}
extern "C" {
    pub fn lruvec_page_state(lruvec: *mut lruvec, idx: node_stat_item) -> c_ulong;
}
extern "C" {
    pub fn mem_cgroup_flush_stats(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn mem_cgroup_flush_stats_ratelimited(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn mod_lruvec_kmem_state(p: *mut c_void, idx: node_stat_item, val: c_int);
}
extern "C" {
    pub fn split_page_memcg(first: *mut page, order: unsigned);
}
extern "C" {
    pub fn mem_cgroup_flush_workqueue();
}
extern "C" {
    pub fn mem_cgroup_init() -> c_int;
}

pub const MEM_CGROUP_ID_SHIFT: c_int = 0;

// min = *low = *usage = 0;
// XXX: This should always return root_mem_cgroup
extern "C" {
    pub fn node_page_state(_arg: lruvec_pgdat(lruvec), _arg: idx) -> return;
}
extern "C" {
    pub fn node_page_state_monotonic(_arg: lruvec_pgdat(lruvec), _arg: idx) -> return;
}
extern "C" {
    pub fn node_page_state(_arg: lruvec_pgdat(lruvec), _arg: idx) -> return;
}

extern "C" {
    pub fn mem_cgroup_lruvec(_arg: memcg, _arg: lruvec_pgdat(lruvec)) -> return;
}

//
// The memcg can be NULL when the memory controller is disabled.
// Otherwise, the caller keeps the memcg owning @lruvec alive.
//

// Test requires a stable folio->memcg binding, see folio_memcg()
// Don't lock again iff page's lruvec locked
extern "C" {
    pub fn folio_lruvec_lock_irq(_arg: folio) -> return;
}
// Don't lock again iff folio's lruvec locked
// lruvecp = folio_lruvec_lock_irqsave(folio, flags);

extern "C" {
    pub fn mem_cgroup_flush_foreign(wb: *mut bdi_writeback);
}

extern "C" {
    pub fn mem_cgroup_sk_alloc(sk: *mut sock);
}
extern "C" {
    pub fn mem_cgroup_sk_free(sk: *mut sock);
}
extern "C" {
    pub fn mem_cgroup_sk_inherit(sk: *const sock, newsk: *mut sock);
}
extern "C" {
    pub fn mem_cgroup_sk_uncharge(sk: *const sock, nr_pages: c_uint);
}

extern "C" {
    pub fn READ_ONCE(_arg: memcg->socket_pressure) -> return;
}

extern "C" {
    pub fn alloc_shrinker_info(memcg: *mut mem_cgroup) -> c_int;
}
extern "C" {
    pub fn free_shrinker_info(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn set_shrinker_bit(memcg: *mut mem_cgroup, nid: c_int, shrinker_id: c_int);
}
extern "C" {
    pub fn reparent_shrinker_deferred(memcg: *mut mem_cgroup);
}

pub const mem_cgroup_sockets_enabled: c_int = 0;

extern "C" {
    pub fn mem_cgroup_kmem_disabled() -> bool;
}
extern "C" {
    pub fn __memcg_kmem_charge_page(page: *mut page, gfp: gfp_t, order: c_int) -> c_int;
}
extern "C" {
    pub fn __memcg_kmem_uncharge_page(page: *mut page, order: c_int);
}
//
// The returned objcg pointer is safe to use without additional
// protection within a scope. The scope is defined either by
// the current task (similar to the "current" global variable)
// or by set_active_memcg() pair.
// Please, use obj_cgroup_get() to get a reference if the pointer
// needs to be used outside of the local scope.
//
extern "C" {
    pub fn obj_cgroup_charge(objcg: *mut obj_cgroup, gfp: gfp_t, size: usize) -> c_int;
}
extern "C" {
    pub fn obj_cgroup_uncharge(objcg: *mut obj_cgroup, size: usize);
}
extern "C" {
    pub fn static_branch_likely(_arg: &memcg_bpf_enabled_key) -> return;
}
extern "C" {
    pub fn static_branch_likely(_arg: &memcg_kmem_online_key) -> return;
}
extern "C" {
    pub fn __memcg_kmem_charge_page(_arg: page, _arg: gfp, _arg: order) -> return;
}
//
// A helper for accessing memcg's kmem_id, used for getting
// corresponding LRU lists.
//
extern "C" {
    pub fn mem_cgroup_node_filter_allowed(memcg: *mut mem_cgroup, mask: *mut nodemask_t);
}
extern "C" {
    pub fn mem_cgroup_show_protected_memory(memcg: *mut mem_cgroup);
}

extern "C" {
    pub fn obj_cgroup_may_zswap(objcg: *mut obj_cgroup) -> bool;
}
extern "C" {
    pub fn obj_cgroup_charge_zswap(objcg: *mut obj_cgroup, size: usize);
}
extern "C" {
    pub fn obj_cgroup_uncharge_zswap(objcg: *mut obj_cgroup, size: usize);
}
extern "C" {
    pub fn mem_cgroup_zswap_writeback_enabled(memcg: *mut mem_cgroup) -> bool;
}

// if zswap is disabled, do not block pages going to the swapping device

// Cgroup v1-related declarations

extern "C" {
    pub fn mem_cgroup_oom_synchronize(wait: bool) -> bool;
}

extern "C" {
    pub fn __memcg1_swapout(folio: *mut folio, ci: *mut swap_cluster_info);
}
extern "C" {
    pub fn memcg1_swapin(folio: *mut folio);
}

