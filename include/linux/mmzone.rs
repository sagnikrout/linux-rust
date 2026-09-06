//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmzone.h
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

// Free memory management - zoned buddy allocator.

pub const MAX_PAGE_ORDER: c_int = 10;

// Defines the order for the number of pages that have a migrate type.

//
// The MAX_PAGE_ORDER, which defines the max order of pages to be allocated
// by the buddy allocator, has to be larger or equal to the PAGE_BLOCK_MAX_ORDER,
// which defines the order for the number of pages that can have a migrate type
//

//
// PAGE_ALLOC_COSTLY_ORDER is the order at which allocations are deemed
// costly to service.  That is between allocation orders which should
// coalesce naturally under reasonable reclaim pressure and those which
// will not.
//
pub const PAGE_ALLOC_COSTLY_ORDER: c_int = 3;

//
// We don't expect any folios that exceed buddy sizes (and consequently
// memory sections).
//

//
// Only pages within a single memory section are guaranteed to be
// contiguous. By limiting folios to a single memory section, all folio
// pages are guaranteed to be contiguous.
//

//
// There is no real limit on the folio size. We limit them to the maximum we
// currently expect (see CONFIG_HAVE_GIGANTIC_FOLIOS): with hugetlb, we expect
// no folios larger than 16 GiB on 64bit and 1 GiB on 32bit.
//

//
// Without hugetlb, gigantic folios that are bigger than a single PUD are
// currently impossible.
//

//
// HugeTLB Vmemmap Optimization (HVO) requires struct pages of the head page to
// be naturally aligned with regard to the folio size.
//
// HVO which is only active if the size of struct page is a power of 2.
//

//
// vmemmap optimization (like HVO) is only possible for page orders that fill
// two or more pages with struct pages.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum migratetype {
    MIGRATE_UNMOVABLE,
    MIGRATE_MOVABLE,
    MIGRATE_RECLAIMABLE,
    MIGRATE_PCPTYPES,	/* the number of types on the pcp lists */
    MIGRATE_HIGHATOMIC = MIGRATE_PCPTYPES,

//
// MIGRATE_CMA migration type is designed to mimic the way
// ZONE_MOVABLE works.  Only movable pages can be allocated
// from MIGRATE_CMA pageblocks and page allocator never
// implicitly change migration type of MIGRATE_CMA pageblock.
//
// The way to use it is to change migratetype of a range of
// pageblocks to MIGRATE_CMA which can be done by
// __free_pageblock_cma() function.
//
    MIGRATE_CMA,
    __MIGRATE_TYPE_END = MIGRATE_CMA,

    __MIGRATE_TYPE_END = MIGRATE_HIGHATOMIC,

    MIGRATE_ISOLATE,	/* can't allocate from here */

    MIGRATE_TYPES
}

// In mm/page_alloc.c; keep in sync also with show_migration_types() there

//
// __dump_folio() in mm/debug.c passes a folio pointer to on-stack struct folio,
// so folio_pfn() cannot be used and pfn is needed.
//

//
// Check whether a migratetype can be merged with another migratetype.
//
// It is only mergeable when it can fall back to other migratetypes for
// allocation. See fallbacks[MIGRATE_TYPES][3] in page_alloc.c.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct free_area {
    pub free_list: [list_head; MIGRATE_TYPES],
    pub nr_free: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum numa_stat_item {
    NUMA_HIT,		/* allocated in intended node */
    NUMA_MISS,		/* allocated in non intended node */
    NUMA_FOREIGN,		/* was intended here, hit elsewhere */
    NUMA_INTERLEAVE_HIT,	/* interleaver preferred this zone */
    NUMA_LOCAL,		/* allocation from local node */
    NUMA_OTHER,		/* allocation from other node */
    NR_VM_NUMA_EVENT_ITEMS
}

pub const NR_VM_NUMA_EVENT_ITEMS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zone_stat_item {
    NR_FREE_PAGES,
    NR_FREE_PAGES_BLOCKS,
    NR_ZONE_LRU_BASE, /* Used only for compaction and reclaim retry */
    NR_ZONE_INACTIVE_ANON = NR_ZONE_LRU_BASE,
    NR_ZONE_ACTIVE_ANON,
    NR_ZONE_INACTIVE_FILE,
    NR_ZONE_ACTIVE_FILE,
    NR_ZONE_UNEVICTABLE,
    NR_ZONE_WRITE_PENDING,	/* Count of dirty, writeback and unstable pages */
    NR_MLOCK,		/* mlock()ed pages found and moved off LRU */

    NR_ZSPAGES,		/* allocated in zsmalloc */

    NR_FREE_CMA_PAGES,

    NR_UNACCEPTED,

    NR_VM_ZONE_STAT_ITEMS };

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum node_stat_item {
    NR_LRU_BASE,
    NR_INACTIVE_ANON = NR_LRU_BASE, /* must match order of LRU_[IN]ACTIVE */
    NR_ACTIVE_ANON,		/*  "     "     "   "       "         */
    NR_INACTIVE_FILE,	/*  "     "     "   "       "         */
    NR_ACTIVE_FILE,		/*  "     "     "   "       "         */
    NR_UNEVICTABLE,		/*  "     "     "   "       "         */
    NR_SLAB_RECLAIMABLE_B,
    NR_SLAB_UNRECLAIMABLE_B,
    NR_ISOLATED_ANON,	/* Temporary isolated pages from anon lru */
    NR_ISOLATED_FILE,	/* Temporary isolated pages from file lru */
    WORKINGSET_NODES,
    WORKINGSET_REFAULT_BASE,
    WORKINGSET_REFAULT_ANON = WORKINGSET_REFAULT_BASE,
    WORKINGSET_REFAULT_FILE,
    WORKINGSET_ACTIVATE_BASE,
    WORKINGSET_ACTIVATE_ANON = WORKINGSET_ACTIVATE_BASE,
    WORKINGSET_ACTIVATE_FILE,
    WORKINGSET_RESTORE_BASE,
    WORKINGSET_RESTORE_ANON = WORKINGSET_RESTORE_BASE,
    WORKINGSET_RESTORE_FILE,
    WORKINGSET_NODERECLAIM,
    NR_ANON_MAPPED,	/* Mapped anonymous pages */
    NR_FILE_MAPPED,	/* pagecache pages mapped into pagetables.
    only modified from process context */
    NR_FILE_PAGES,
    NR_FILE_DIRTY,
    NR_WRITEBACK,
    NR_SHMEM,		/* shmem pages (included tmpfs/GEM pages) */
    NR_SHMEM_THPS,
    NR_SHMEM_PMDMAPPED,
    NR_FILE_THPS,
    NR_FILE_PMDMAPPED,
    NR_ANON_THPS,
    NR_VMSCAN_WRITE,
    NR_VMSCAN_IMMEDIATE,	/* Prioritise for reclaim when writeback ends */
    NR_DIRTIED,		/* page dirtyings since bootup */
    NR_WRITTEN,		/* page writings since bootup */
    NR_THROTTLED_WRITTEN,	/* NR_WRITTEN while reclaim throttled */
    NR_KERNEL_MISC_RECLAIMABLE,	/* reclaimable non-slab kernel pages */
    NR_FOLL_PIN_ACQUIRED,	/* via: pin_user_page(), gup flag: FOLL_PIN */
    NR_FOLL_PIN_RELEASED,	/* pages returned via unpin_user_page() */
    NR_VMALLOC,
    NR_KERNEL_STACK_KB,	/* measured in KiB */

    NR_KERNEL_SCS_KB,	/* measured in KiB */

    NR_PAGETABLE,		/* used for pagetables */
    NR_SECONDARY_PAGETABLE, /* secondary pagetables, KVM & IOMMU */

    NR_IOMMU_PAGES,		/* # of pages allocated by IOMMU */

    NR_SWAPCACHE,

    PGPROMOTE_SUCCESS,	/* promote successfully */
//
// Candidate pages for promotion based on hint fault latency.  This
// counter is used to control the promotion rate and adjust the hot
// threshold.
//
    PGPROMOTE_CANDIDATE,
//
// Not rate-limited (NRL) candidate pages for those can be promoted
// without considering hot threshold because of enough free pages in
// fast-tier node.  These promotions bypass the regular hotness checks
// and do NOT influence the promotion rate-limiter or
// threshold-adjustment logic.
// This is for statistics/monitoring purposes.
//
    PGPROMOTE_CANDIDATE_NRL,

// PGDEMOTE_*: pages demoted
    PGDEMOTE_KSWAPD,
    PGDEMOTE_DIRECT,
    PGDEMOTE_KHUGEPAGED,
    PGDEMOTE_PROACTIVE,
    PGSTEAL_KSWAPD,
    PGSTEAL_DIRECT,
    PGSTEAL_KHUGEPAGED,
    PGSTEAL_PROACTIVE,
    PGSTEAL_ANON,
    PGSTEAL_FILE,
    PGSCAN_KSWAPD,
    PGSCAN_DIRECT,
    PGSCAN_KHUGEPAGED,
    PGSCAN_PROACTIVE,
    PGSCAN_ANON,
    PGSCAN_FILE,
    PGROTATE_ANON,
    PGROTATE_FILE,
    PGREFILL,

    NR_HUGETLB,

    NR_BALLOON_PAGES,
    NR_KERNEL_FILE_PAGES,
    NR_GPU_ACTIVE,	/* Pages assigned to GPU objects */
    NR_GPU_RECLAIM,	/* Pages in shrinkable GPU pools */
    NR_VM_NODE_STAT_ITEMS
}

//
// Returns true if the item should be printed in THPs (/proc/vmstat
// currently prints number of anon, file and shmem THPs. But the item
// is charged in pages).
//
// Returns true if the value is measured in bytes (most vmstat values are
// measured in pages). This defines the API part, the internal representation
// might be different.
//
// Global and per-node slab counters track slab pages.
// It's expected that changes are multiples of PAGE_SIZE.
// Internally values are stored in pages.
//
// Per-memcg and per-lruvec counters track memory, consumed
// by individual slab objects. These counters are actually
// byte-precise.
//
// We do arithmetic on the LRU lists in various places in the code,
// so it is important to keep the active lists LRU_ACTIVE higher in
// the array than the corresponding inactive lists, and to keep
// the *_FILE lists LRU_FILE higher than the corresponding _ANON lists.
//
// This has to be kept in sync with the statistics in zone_stat_item
// above and the descriptions in vmstat_text in mm/vmstat.c
//
pub const LRU_BASE: c_int = 0;
pub const LRU_ACTIVE: c_int = 1;
pub const LRU_FILE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lru_list {
    LRU_INACTIVE_ANON = LRU_BASE,
    LRU_ACTIVE_ANON = LRU_BASE + LRU_ACTIVE,
    LRU_INACTIVE_FILE = LRU_BASE + LRU_FILE,
    LRU_ACTIVE_FILE = LRU_BASE + LRU_FILE + LRU_ACTIVE,
    LRU_UNEVICTABLE,
    NR_LRU_LISTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmscan_throttle_state {
    VMSCAN_THROTTLE_WRITEBACK,
    VMSCAN_THROTTLE_ISOLATED,
    VMSCAN_THROTTLE_NOPROGRESS,
    VMSCAN_THROTTLE_CONGESTED,
    NR_VMSCAN_THROTTLE,
}

pub const WORKINGSET_ANON: c_int = 0;
pub const WORKINGSET_FILE: c_int = 1;
pub const ANON_AND_FILE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lruvec_flags {
//
// An lruvec has many dirty pages backed by a congested BDI:
// 1. LRUVEC_CGROUP_CONGESTED is set by cgroup-level reclaim.
// It can be cleared by cgroup reclaim or kswapd.
// 2. LRUVEC_NODE_CONGESTED is set by kswapd node-level reclaim.
// It can only be cleared by kswapd.
//
// Essentially, kswapd can unthrottle an lruvec throttled by cgroup
// reclaim, but not vice versa. This only applies to the root cgroup.
// The goal is to prevent cgroup reclaim on the root cgroup (e.g.
// memory.reclaim) to unthrottle an unbalanced node (that was throttled
// by kswapd).
//
    LRUVEC_CGROUP_CONGESTED,
    LRUVEC_NODE_CONGESTED,
}

//
// Evictable folios are divided into multiple generations. The youngest and the
// oldest generation numbers, max_seq and min_seq, are monotonically increasing.
// They form a sliding window of a variable size [MIN_NR_GENS, MAX_NR_GENS]. An
// offset within MAX_NR_GENS, i.e., gen, indexes the LRU list of the
// corresponding generation. The gen counter in folio->flags stores gen+1 while
// a folio is on one of lrugen->folios[]. Otherwise it stores 0.
//
// After a folio is faulted in, the aging needs to check the accessed bit at
// least twice before handing this folio over to the eviction. The first check
// clears the accessed bit from the initial fault; the second check makes sure
// this folio hasn't been used since then. This process, AKA second chance,
// requires a minimum of two generations, hence MIN_NR_GENS. And to maintain ABI
// compatibility with the active/inactive LRU, e.g., /proc/vmstat, these two
// generations are considered active; the rest of generations, if they exist,
// are considered inactive. See lru_gen_is_active().
//
// PG_active is always cleared while a folio is on one of lrugen->folios[] so
// that the sliding window needs not to worry about it. And it's set again when
// a folio considered active is isolated for non-reclaiming purposes, e.g.,
// migration. See lru_gen_add_folio() and lru_gen_del_folio().
//
// MAX_NR_GENS is set to 4 so that the multi-gen LRU can support twice the
// number of categories of the active/inactive LRU when keeping track of
// accesses through page tables. This requires order_base_2(MAX_NR_GENS+1) bits
// in folio->flags, masked by LRU_GEN_MASK.
//

//
// Each generation is divided into multiple tiers. A folio accessed N times
// through file descriptors is in tier order_base_2(N). A folio in the first
// tier (N=0,1) is marked by PG_referenced unless it was faulted in through page
// tables or read ahead. A folio in the last tier (MAX_NR_TIERS-1) is marked by
// PG_workingset. A folio in any other tier (1<N<5) between the first and last
// is marked by additional bits of LRU_REFS_WIDTH in folio->flags.
//
// In contrast to moving across generations which requires the LRU lock, moving
// across tiers only involves atomic operations on folio->flags and therefore
// has a negligible cost in the buffered access path. In the eviction path,
// comparisons of refaulted/(evicted+protected) from the first tier and the rest
// infer whether folios accessed multiple times through file descriptors are
// statistically hot and thus worth protecting.
//
// MAX_NR_TIERS is set to 4 so that the multi-gen LRU can support twice the
// number of categories of the active/inactive LRU when keeping track of
// accesses through file descriptors. This uses MAX_NR_TIERS-2 spare bits in
// folio->flags, masked by LRU_REFS_MASK.
//

//
// For folios accessed multiple times through file descriptors,
// lru_gen_inc_refs() sets additional bits of LRU_REFS_WIDTH in folio->flags
// after PG_referenced, then PG_workingset after LRU_REFS_WIDTH. After all its
// bits are set, i.e., LRU_REFS_FLAGS|BIT(PG_workingset), a folio is lazily
// promoted into the second oldest generation in the eviction path. And when
// folio_inc_gen() does that, it clears LRU_REFS_FLAGS so that
// lru_gen_inc_refs() can start over. Note that for this case, LRU_REFS_MASK is
// only valid when PG_referenced is set.
//
// For folios accessed multiple times through page tables, folio_update_gen()
// from a page table walk or lru_gen_set_refs() from a rmap walk sets
// PG_referenced after the accessed bit is cleared for the first time.
// Thereafter, those two paths set PG_workingset and promote folios to the
// youngest generation. Like folio_inc_gen(), folio_update_gen() also clears
// PG_referenced. Note that for this case, LRU_REFS_MASK is not used.
//
// For both cases above, after PG_workingset is set on a folio, it remains until
// this folio is either reclaimed, or "deactivated" by lru_gen_clear_refs(). It
// can be set again if lru_gen_test_recent() returns true upon a refault.
//

// whether to keep historical stats from evicted generations

//
// The youngest generation number is stored in max_seq for both anon and file
// types as they are aged on an equal footing. The oldest generation numbers are
// stored in min_seq[] separately for anon and file types so that they can be
// incremented independently. Ideally min_seq[] are kept in sync when both anon
// and file types are evictable. However, to adapt to situations like extreme
// swappiness, they are allowed to be out of sync by at most
// MAX_NR_GENS-MIN_NR_GENS-1.
//
// The number of pages in each generation is eventually consistent and therefore
// can be transiently negative when reset_batch_size() is pending.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_gen_folio {
// the aging increments the youngest generation number
    pub max_seq: c_ulong,
// the eviction increments the oldest generation numbers
    pub min_seq: [c_ulong; ANON_AND_FILE],
// the birth time of each generation in jiffies
    pub timestamps: [c_ulong; MAX_NR_GENS],
// the multi-gen LRU lists, lazily sorted on eviction
    pub folios: [list_head; MAX_NR_GENS][ANON_AND_FILE][MAX_NR_ZONES],
// the multi-gen LRU sizes, eventually consistent
    pub nr_pages: [c_long; MAX_NR_GENS][ANON_AND_FILE][MAX_NR_ZONES],
// the exponential moving average of refaulted
    pub avg_refaulted: [c_ulong; ANON_AND_FILE][MAX_NR_TIERS],
// the exponential moving average of evicted+protected
    pub avg_total: [c_ulong; ANON_AND_FILE][MAX_NR_TIERS],
// can only be modified under the LRU lock
    pub protected: [c_ulong; NR_HIST_GENS][ANON_AND_FILE][MAX_NR_TIERS],
// can be modified without holding the LRU lock
    pub evicted: [atomic_long_t; NR_HIST_GENS][ANON_AND_FILE][MAX_NR_TIERS],
    pub refaulted: [atomic_long_t; NR_HIST_GENS][ANON_AND_FILE][MAX_NR_TIERS],
// whether the multi-gen LRU is enabled
    pub enabled: bool,
// the memcg generation this lru_gen_folio belongs to
    pub gen: u8,
// the list segment this lru_gen_folio belongs to
    pub seg: u8,
// per-node lru_gen_folio list for global reclaim
    pub list: hlist_nulls_node,
}

// double-buffering Bloom filters
pub const NR_BLOOM_FILTERS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_gen_mm_state {
// synced with max_seq after each iteration
    pub seq: c_ulong,
// where the current iteration continues after
    pub head: *mut list_head,
// where the last iteration ended before
    pub tail: *mut list_head,
// Bloom filters flip after each iteration
    pub filters: [*mut c_ulong; NR_BLOOM_FILTERS],
// the mm stats for debugging
    pub stats: [c_ulong; NR_HIST_GENS][NR_MM_STATS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_gen_mm_walk {
// the lruvec under reclaim
    pub lruvec: *mut lruvec,
// max_seq from lru_gen_folio: can be out of date
    pub seq: c_ulong,
// the next address within an mm to scan
    pub next_addr: c_ulong,
// to batch promoted pages
    pub nr_pages: [c_int; MAX_NR_GENS][ANON_AND_FILE][MAX_NR_ZONES],
// to batch the mm stats
    pub mm_stats: [c_int; NR_MM_STATS],
// total batched items
    pub batched: c_int,
    pub swappiness: c_int,
    pub force_scan: bool,
}

//
// For each node, memcgs are divided into two generations: the old and the
// young. For each generation, memcgs are randomly sharded into multiple bins
// to improve scalability. For each bin, the hlist_nulls is virtually divided
// into three segments: the head, the tail and the default.
//
// An onlining memcg is added to the tail of a random bin in the old generation.
// The eviction starts at the head of a random bin in the old generation. The
// per-node memcg generation counter, whose reminder (mod MEMCG_NR_GENS) indexes
// the old generation, is incremented when all its bins become empty.
//
// There are four operations:
// 1. MEMCG_LRU_HEAD, which moves a memcg to the head of a random bin in its
// current generation (old or young) and updates its "seg" to "head";
// 2. MEMCG_LRU_TAIL, which moves a memcg to the tail of a random bin in its
// current generation (old or young) and updates its "seg" to "tail";
// 3. MEMCG_LRU_OLD, which moves a memcg to the head of a random bin in the old
// generation, updates its "gen" to "old" and resets its "seg" to "default";
// 4. MEMCG_LRU_YOUNG, which moves a memcg to the tail of a random bin in the
// young generation, updates its "gen" to "young" and resets its "seg" to
// "default".
//
// The events that trigger the above operations are:
// 1. Exceeding the soft limit, which triggers MEMCG_LRU_HEAD;
// 2. The first attempt to reclaim a memcg below low, which triggers
// MEMCG_LRU_TAIL;
// 3. The first attempt to reclaim a memcg offlined or below reclaimable size
// threshold, which triggers MEMCG_LRU_TAIL;
// 4. The second attempt to reclaim a memcg offlined or below reclaimable size
// threshold, which triggers MEMCG_LRU_YOUNG;
// 5. Attempting to reclaim a memcg below min, which triggers MEMCG_LRU_YOUNG;
// 6. Finishing the aging on the eviction path, which triggers MEMCG_LRU_YOUNG;
// 7. Offlining a memcg, which triggers MEMCG_LRU_OLD.
//
// Notes:
// 1. Memcg LRU only applies to global reclaim, and the round-robin incrementing
// of their max_seq counters ensures the eventual fairness to all eligible
// memcgs. For memcg reclaim, it still relies on mem_cgroup_iter().
// 2. There are only two valid generations: old (seq) and young (seq+1).
// MEMCG_NR_GENS is set to three so that when reading the generation counter
// locklessly, a stale value (seq-1) does not wraparound to young.
//
pub const MEMCG_NR_GENS: c_int = 3;
pub const MEMCG_NR_BINS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_gen_memcg {
// the per-node memcg generation counter
    pub seq: c_ulong,
// each memcg has one lru_gen_folio per node
    pub nr_memcgs: [c_ulong; MEMCG_NR_GENS],
// per-node lru_gen_folio list for global reclaim
    pub fifo: [hlist_nulls_head; MEMCG_NR_GENS][MEMCG_NR_BINS],
// protects the above
    pub lock: spinlock_t,
}

extern "C" {
    pub fn lru_gen_init_pgdat(pgdat: *mut pglist_data);
}
extern "C" {
    pub fn lru_gen_init_lruvec(lruvec: *mut lruvec);
}
extern "C" {
    pub fn lru_gen_look_around(pvmw: *mut page_vma_mapped_walk, nr: c_uint) -> bool;
}
extern "C" {
    pub fn lru_gen_init_memcg(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn lru_gen_exit_memcg(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn lru_gen_online_memcg(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn lru_gen_offline_memcg(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn lru_gen_release_memcg(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn lru_gen_soft_reclaim(memcg: *mut mem_cgroup, nid: c_int);
}
extern "C" {
    pub fn max_lru_gen_memcg(memcg: *mut mem_cgroup, nid: c_int);
}
extern "C" {
    pub fn recheck_lru_gen_max_memcg(memcg: *mut mem_cgroup, nid: c_int) -> bool;
}
extern "C" {
    pub fn lru_gen_reparent_memcg(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, nid: c_int);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_cost {
    pub count: c_ulong,
    pub last_rotated: c_ulong,
    pub last_io: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lruvec {
    pub lists: [list_head; NR_LRU_LISTS],
// per lruvec lru_lock for memcg
    pub lru_lock: spinlock_t,
//
// These track the cost of reclaiming one LRU - file or anon -
// over the other. As the observed cost of reclaiming one LRU
// increases, the reclaim scan balance tips toward the other.
// Updated and decayed at prepare_scan_control() time; cost_lock
// serialises that update.
//
    pub cost: [lru_cost; ANON_AND_FILE],
// Protects cost[].
    pub cost_lock: spinlock_t,
// Non-resident age, driven by LRU movement
    pub nonresident_age: atomic_long_t,
// Refaults at the time of last reclaim cycle
    pub refaults: [c_ulong; ANON_AND_FILE],
// Various lruvec state flags (enum lruvec_flags)
    pub flags: c_ulong,

// evictable pages divided into generations
    pub lrugen: lru_gen_folio,

// to concurrently iterate lru_gen_mm_list
    pub mm_state: lru_gen_mm_state,

    pub pgdat: *mut pglist_data,

    pub zswap_lruvec_state: zswap_lruvec_state,
}

// Isolate for asynchronous migration

// Isolate unevictable pages

// LRU Isolation modes.
pub type isolate_mode_t = unsigned ;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zone_watermarks {
    WMARK_MIN,
    WMARK_LOW,
    WMARK_HIGH,
    WMARK_PROMO,
    NR_WMARK
}

//
// One per migratetype for each PAGE_ALLOC_COSTLY_ORDER. Two additional lists
// are added for THP. One PCP list is used by GPF_MOVABLE, and the other PCP list
// is used by GFP_UNMOVABLE and GFP_RECLAIMABLE.
//

pub const NR_PCP_THP: c_int = 2;

pub const NR_PCP_THP: c_int = 0;

//
// Flags used in pcp->flags field.
//
// PCPF_PREV_FREE_HIGH_ORDER: a high-order page is freed in the
// previous page freeing.  To avoid to drain PCP for an accident
// high-order page freeing.
//
// PCPF_FREE_HIGH_BATCH: preserve "pcp->batch" pages in PCP before
// draining PCP for consecutive high-order pages freeing without
// allocation if data cache slice of CPU is large enough.  To reduce
// zone lock contention and keep cache-hot pages reusing.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_cpu_pages {
    pub /: *mut *mut spinlock_t lock; / Protects lists field,
    pub /: *mut *mut int count; / number of pages in the list,
    pub /: *mut *mut int high; / high watermark, emptying needed,
    pub /: *mut *mut int high_min; / min high watermark,
    pub /: *mut *mut int high_max; / max high watermark,
    pub /: *mut *mut int batch; / chunk size for buddy add/remove,
    pub /: *mut *mut u8 flags; / protected by pcp->lock,
    pub /: *mut *mut u8 alloc_factor; / batch scaling factor during allocate,

    pub /: *mut *mut u8 expire; / When 0, remote pagesets are drained,

    pub /: *mut *mut short free_count; / consecutive free count,
// Lists of pages, one per migrate type stored on the pcp-lists
    pub lists: [list_head; NR_PCP_LISTS],
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_cpu_zonestat {
    pub vm_stat_diff: [i8; NR_VM_ZONE_STAT_ITEMS],
    pub stat_threshold: i8,

//
// Low priority inaccurate counters that are only folded
// on demand. Use a large type to avoid the overhead of
// folding during refresh_cpu_vm_stats.
//
    pub vm_numa_event: [c_ulong; NR_VM_NUMA_EVENT_ITEMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_cpu_nodestat {
    pub stat_threshold: i8,
    pub vm_node_stat_diff: [i8; NR_VM_NODE_STAT_ITEMS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zone_type {
//
// ZONE_DMA and ZONE_DMA32 are used when there are peripherals not able
// to DMA to all of the addressable memory (ZONE_NORMAL).
// On architectures where this area covers the whole 32 bit address
// space ZONE_DMA32 is used. ZONE_DMA is left for the ones with smaller
// DMA addressing constraints. This distinction is important as a 32bit
// DMA mask is assumed when ZONE_DMA32 is defined. Some 64-bit
// platforms may need both zones as they support peripherals with
// different DMA addressing limitations.
//

    ZONE_DMA,

    ZONE_DMA32,

//
// Normal addressable memory is in ZONE_NORMAL. DMA operations can be
// performed on pages in ZONE_NORMAL if the DMA devices support
// transfers to all addressable memory.
//
    ZONE_NORMAL,

//
// A memory area that is only addressable by the kernel through
// mapping portions into its own address space. This is for example
// used by i386 to allow the kernel to address the memory beyond
// 900MB. The kernel will set up special mappings (page
// table entries on i386) for each page that the kernel needs to
// access.
//
    ZONE_HIGHMEM,

//
// ZONE_MOVABLE is similar to ZONE_NORMAL, except that it contains
// movable pages with few exceptional cases described below. Main use
// cases for ZONE_MOVABLE are to make memory offlining/unplug more
// likely to succeed, and to locally limit unmovable allocations - e.g.,
// to increase the number of THP/huge pages. Notable special cases are:
//
// 1. Pinned pages: (long-term) pinning of movable pages might
// essentially turn such pages unmovable. Therefore, we do not allow
// pinning long-term pages in ZONE_MOVABLE. When pages are pinned and
// faulted, they come from the right zone right away. However, it is
// still possible that address space already has pages in
// ZONE_MOVABLE at the time when pages are pinned (i.e. user has
// touches that memory before pinning). In such case we migrate them
// to a different zone. When migration fails - pinning fails.
// 2. memblock allocations: kernelcore/movablecore setups might create
// situations where ZONE_MOVABLE contains unmovable allocations
// after boot. Memory offlining and allocations fail early.
// 3. Memory holes: kernelcore/movablecore setups might create very rare
// situations where ZONE_MOVABLE contains memory holes after boot,
// for example, if we have sections that are only partially
// populated. Memory offlining and allocations fail early.
// 4. PG_hwpoison pages: while poisoned pages can be skipped during
// memory offlining, such pages cannot be allocated.
// 5. Unmovable PG_offline pages: in paravirtualized environments,
// hotplugged memory blocks might only partially be managed by the
// buddy (e.g., via XEN-balloon, Hyper-V balloon, virtio-mem). The
// parts not manged by the buddy are unmovable PG_offline pages. In
// some cases (virtio-mem), such pages can be skipped during
// memory offlining, however, cannot be moved/allocated. These
// techniques might use alloc_contig_range() to hide previously
// exposed pages from the buddy again (e.g., to implement some sort
// of memory unplug in virtio-mem).
// 6. ZERO_PAGE(0), kernelcore/movablecore setups might create
// situations where ZERO_PAGE(0) which is allocated differently
// on different platforms may end up in a movable zone. ZERO_PAGE(0)
// cannot be migrated.
// 7. Memory-hotplug: when using memmap_on_memory and onlining the
// memory to the MOVABLE zone, the vmemmap pages are also placed in
// such zone. Such pages cannot be really moved around as they are
// self-stored in the range, but they are treated as movable when
// the range they describe is about to be offlined.
//
// In general, no unmovable allocations that degrade memory offlining
// should end up in ZONE_MOVABLE. Allocators (like alloc_contig_range())
// have to expect that migrating pages in ZONE_MOVABLE can fail (even
// if has_unmovable_pages() states that there are no unmovable pages,
// there can be false negatives).
//
    ZONE_MOVABLE,

    ZONE_DEVICE,

    __MAX_NR_ZONES

}

pub const ASYNC_AND_SYNC: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zone {
// Read-mostly fields
// zone watermarks, access with *_wmark_pages(zone) macros
    pub _watermark: [c_ulong; NR_WMARK],
    pub watermark_boost: c_ulong,
    pub nr_reserved_highatomic: c_ulong,
    pub nr_free_highatomic: c_ulong,
//
// We don't know if the memory that we're going to allocate will be
// freeable or/and it will be released eventually, so to avoid totally
// wasting several GB of ram we must reserve some of the lower zone
// memory (otherwise we risk to run OOM on the lower zones despite
// there being tons of freeable ram on the higher zones).  This array is
// recalculated at runtime if the sysctl_lowmem_reserve_ratio sysctl
// changes.
//
    pub lowmem_reserve: [c_long; MAX_NR_ZONES],
    pub node: c_int,

    pub zone_pgdat: *mut pglist_data,
    pub per_cpu_pageset: *mut per_cpu_pages __percpu,
    pub per_cpu_zonestats: *mut per_cpu_zonestat __percpu,
//
// the high and batch values are copied to individual pagesets for
// faster access
//
    pub pageset_high_min: c_int,
    pub pageset_high_max: c_int,
    pub pageset_batch: c_int,

//
// Flags for a pageblock_nr_pages block. See pageblock-flags.h.
// In SPARSEMEM, this map is stored in struct mem_section
//
    pub pageblock_flags: *mut c_ulong,

// zone_start_pfn == zone_start_paddr >> PAGE_SHIFT
    pub zone_start_pfn: c_ulong,
//
// spanned_pages is the total pages spanned by the zone, including
// holes, which is calculated as:
// spanned_pages = zone_end_pfn - zone_start_pfn;
//
// present_pages is physical pages existing within the zone, which
// is calculated as:
// present_pages = spanned_pages - absent_pages(pages in holes);
//
// present_early_pages is present pages existing within the zone
// located on memory available since early boot, excluding hotplugged
// memory.
//
// managed_pages is present pages managed by the buddy system, which
// is calculated as (reserved_pages includes pages allocated by the
// bootmem allocator):
// managed_pages = present_pages - reserved_pages;
//
// cma pages is present pages that are assigned for CMA use
// (MIGRATE_CMA).
//
// So present_pages may be used by memory hotplug or memory power
// management logic to figure out unmanaged pages by checking
// (present_pages - managed_pages). And managed_pages should be used
// by page allocator and vm scanner to calculate all kinds of watermarks
// and thresholds.
//
// Locking rules:
//
// zone_start_pfn and spanned_pages are protected by span_seqlock.
// It is a seqlock because it has to be read outside of zone->lock,
// and it is done in the main allocator path.  But, it is written
// quite infrequently.
//
// The span_seq lock is declared along with zone->lock because it is
// frequently read in proximity to zone->lock.  It's good to
// give them a chance of being in the same cacheline.
//
// Write access to present_pages at runtime should be protected by
// mem_hotplug_begin/done(). Any reader who can't tolerant drift of
// present_pages should use get_online_mems() to get a stable value.
//
    pub managed_pages: atomic_long_t,
    pub spanned_pages: c_ulong,
    pub present_pages: c_ulong,

    pub present_early_pages: c_ulong,

    pub cma_pages: c_ulong,

    pub name: *const c_char,

//
// Number of isolated pageblock. It is used to solve incorrect
// freepage counting problem due to racy retrieving migratetype
// of pageblock. Protected by zone->lock.
//
    pub nr_isolate_pageblock: c_ulong,

// see spanned/present_pages for more description
    pub span_seqlock: seqlock_t,

    pub initialized: c_int,
// Write-intensive fields used from the page allocator
// free areas of different sizes
    pub free_area: [free_area; NR_PAGE_ORDERS],
// Pages to be accepted. All pages on the list are MAX_PAGE_ORDER
    pub unaccepted_pages: list_head,
// To be called once the last page in the zone is accepted
    pub unaccepted_cleanup: work_struct,

// zone flags, see below
    pub flags: c_ulong,
// Primarily protects free_area
    pub lock: spinlock_t,
// Pages to be freed when next trylock succeeds
    pub trylock_free_pages: llist_head,
// Write-intensive fields used by compaction and vmstats.
//
// When free pages are below this point, additional steps are taken
// when reading the number of free pages to avoid per-cpu counter
// drift allowing watermarks to be breached
//
    pub percpu_drift_mark: c_ulong,

// pfn where compaction free scanner should start
    pub compact_cached_free_pfn: c_ulong,
// pfn where compaction migration scanner should start
    pub compact_cached_migrate_pfn: [c_ulong; ASYNC_AND_SYNC],
    pub compact_init_migrate_pfn: c_ulong,
    pub compact_init_free_pfn: c_ulong,

//
// On compaction failure, 1<<compact_defer_shift compactions
// are skipped before trying again. The number attempted since
// last failure is tracked with compact_considered.
// compact_order_failed is the minimum compaction failed order.
//
    pub compact_considered: c_uint,
    pub compact_defer_shift: c_uint,
    pub compact_order_failed: c_int,

// Set to true when the PG_migrate_skip bits should be cleared
    pub compact_blockskip_flush: bool,

    pub contiguous: bool,
// Zone statistics
    pub vm_stat: [atomic_long_t; NR_VM_ZONE_STAT_ITEMS],
    pub vm_numa_event: [atomic_long_t; NR_VM_NUMA_EVENT_ITEMS],    pub vmemmap_tails: [*mut page; NR_VMEMMAP_TAILS],
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pgdat_flags {
    PGDAT_WRITEBACK,		/* reclaim scanning has recently found
// many pages under writeback
//
    PGDAT_RECLAIM_LOCKED,		/* prevents concurrent reclaim */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zone_flags {
    ZONE_BOOSTED_WATERMARK,		/* zone recently boosted watermarks.
// Cleared when kswapd is woken.
//
    ZONE_RECLAIM_ACTIVE,		/* kswapd may be scanning the zone. */
    ZONE_BELOW_HIGH,		/* zone is below high watermark. */
}

    pub z->watermark_boost: return z->_watermark[w] +,
    pub WMARK_MIN): return wmark_pages(z,,
    pub WMARK_LOW): return wmark_pages(z,,
    pub WMARK_HIGH): return wmark_pages(z,,
    pub WMARK_PROMO): return wmark_pages(z,,
    pub long)atomic_long_read(&zone->managed_pages): return (unsigned,

    pub zone->cma_pages: return,

    pub 0: return,

    pub zone->spanned_pages: return zone->zone_start_pfn +,
    pub zone_end_pfn(zone): return zone->zone_start_pfn <= pfn && pfn <,
    pub zone->initialized: return,
    pub 0: return zone->spanned_pages ==,

//
// The zone field is never updated after free_area_init_core()
// sets it, so none of the operations on it need to be atomic.
//
// Page flags: | [SECTION] | [NODE] | ZONE | [LAST_CPUPID] | ... | FLAGS |

//
// Define the bit shifts to access each section.  For non-existent
// sections we define the shift as 0; that plus a 0 mask ensures
// the compiler will optimise away reference to them.
//

// NODE:ZONE or SECTION:ZONE is used to ID a zone for the buddy allocator

    pub ZONES_PGSHIFT): ASSERT_EXCLUSIVE_BITS(flags->f, ZONES_MASK <<,

    pub ZONES_MASK: return (flags->f >> ZONES_PGSHIFT) &,
    pub memdesc_zonenum(&page->flags): return,
    pub memdesc_zonenum(&folio->flags): return,

    pub ZONE_DEVICE: return memdesc_zonenum(mdf) ==,
    pub page): VM_WARN_ON_ONCE_PAGE(!memdesc_is_zone_device(&page->flags),,
    pub page_folio(page)->pgmap: return,
//
// Consecutive zone device pages should not be merged into the same sgl
// or bvec segment with other types of pages or if they belong to different
// pgmaps. Otherwise getting the pgmap of a given segment is not possible
// without scanning the entire segment. This helper returns true either if
// both pages are not zone device pages or both pages are zone device pages
// with the same pgmap.
//
    pub false: return,
    pub true: return,
    pub page_pgmap(b): return page_pgmap(a) ==,
    pub ): *mut unsigned long, struct dev_pagemap,

    pub false: return,
    pub true: return,
    pub NULL: return,

    pub memdesc_is_zone_device(&page->flags): return,
    pub memdesc_is_zone_device(&folio->flags): return,
    pub ZONE_MOVABLE: return page_zonenum(page) ==,
    pub ZONE_MOVABLE: return folio_zonenum(folio) ==,

//
// Return true if [start_pfn, start_pfn + nr_pages) range has a non-empty
// intersection with the given zone
//
    pub false: return,
    pub false: return,
    pub true: return,
//
// The "priority" of VM scanning is how much of the queues we will scan in one
// go. A value of 12 for DEF_PRIORITY implies that we will scan 1/4096th of the
// queues ("queue_length >> 12") during an aging round.
//
pub const DEF_PRIORITY: c_int = 12;
// Maximum number of zones on a zonelist

//
// The NUMA zonelists are doubled because we need zonelists that
// restrict the allocations to a single node for __GFP_THISNODE.
//

}

//
// This struct contains information about a zone in a zonelist. It is stored
// here to avoid dereferences into large structures and lookups of tables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoneref {
    pub /: *mut *mut *mut zone zone; / Pointer to actual zone,
    pub /: *mut *mut int zone_idx; / zone_idx(zoneref->zone),
}

//
// One allocation request operates on a zonelist. A zonelist
// is a list of zones, the first one is the 'goal' of the
// allocation, the other zones are fallback zones, in decreasing
// priority.
//
// To speed the reading of the zonelist, the zonerefs contain the zone index
// of the entry being read. Helper functions to access information given
// a struct zoneref are
//
// zonelist_zone()	- Return the struct zone * for an entry in _zonerefs
// zonelist_zone_idx()	- Return the index of the zone for an entry
// zonelist_node_idx()	- Return the index of the node for an entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zonelist {
    pub 1]: zoneref _zonerefs[MAX_ZONES_PER_ZONELIST +,
}

//
// The array of struct pages for flatmem.
// It must be declared for SPARSEMEM as well because there are configurations
// that rely on that.
//

//
// Per NUMA node memory failure handling statistics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_failure_stats {
//
// Number of raw pages poisoned.
// Cases not accounted: memory outside kernel control, offline page,
// arch-specific memory_failure (SGX), hwpoison_filter() filtered
// error events, and unpoison actions from hwpoison_unpoison.
//
    pub total: c_ulong,
//
// Recovery results of poisoned raw pages handled by memory_failure,
// in sync with mf_result.
// total = ignored + failed + delayed + recovered.
// total * PAGE_SIZE * #nodes = /proc/meminfo/HardwareCorrupted.
//
    pub ignored: c_ulong,
    pub failed: c_ulong,
    pub delayed: c_ulong,
    pub recovered: c_ulong,
}

//
// On NUMA machines, each NUMA node would have a pg_data_t to describe
// it's memory layout. On UMA machines there is a single pglist_data which
// describes the whole memory.
//
// Memory statistics and page replacement data structures are maintained on a
// per-zone basis.
//
// node_zones contains just the zones for THIS node. Not all of the
// zones may be populated, but it is the full list. It is referenced by
// this node's node_zonelists as well as other node's node_zonelists.
//
// node_zonelists contains references to all zones in all nodes.
// Generally the first zones will be references to this node's
// node_zones.
//

//
// Must be held any time you expect node_start_pfn,
// node_present_pages, node_spanned_pages or nr_zones to stay constant.
// Also synchronizes pgdat->first_deferred_pfn during deferred page
// init.
//
// pgdat_resize_lock() and pgdat_resize_unlock() are provided to
// manipulate node_size_lock without checking for CONFIG_MEMORY_HOTPLUG
// or CONFIG_DEFERRED_STRUCT_PAGE_INIT.
//
// Nests above zone->lock and zone->span_seqlock
//

// workqueues for throttling reclaim for different reasons.
// when throttling started.

//
// This is a per-node reserve of pages that are not available
// to userspace allocations.
//

//
// node reclaim becomes active if more unmapped pages exist.
//

// Write-intensive fields used by page reclaim

//
// If memory initialisation on large machines is deferred then this
// is the first PFN that needs to be initialised.
//

// start time in ms of current promote rate limit period
// number of promote candidate pages at start time of current rate limit period
// promote threshold in ms
// start time in ms of current promote threshold adjustment period
//
// number of promote candidate pages at start time of current promote
// threshold adjustment period
//

// Fields commonly accessed by the page reclaim scanner
//
// NOTE: THIS IS UNUSED IF MEMCG IS ENABLED.
//
// Use mem_cgroup_lruvec() to look up lruvecs.
//

// kswap mm walk data
// lru_gen_folio list

// Per-node vmstats

extern "C" {
    pub fn build_all_zonelists(pgdat: *mut pg_data_t);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kswapd_clear_hopeless_reason {
    KSWAPD_CLEAR_HOPELESS_OTHER = 0,
    KSWAPD_CLEAR_HOPELESS_KSWAPD,
    KSWAPD_CLEAR_HOPELESS_DIRECT,
    KSWAPD_CLEAR_HOPELESS_PCP,
}

extern "C" {
    pub fn kswapd_clear_hopeless(pgdat: *mut pg_data_t, reason: kswapd_clear_hopeless_reason);
}
extern "C" {
    pub fn kswapd_test_hopeless(pgdat: *mut pg_data_t) -> bool;
}
//
// Memory initialization context, use to differentiate memory added by
// the platform statically or via memory hotplug interface.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum meminit_context {
    MEMINIT_EARLY,
    MEMINIT_HOTPLUG,
}

extern "C" {
    pub fn lruvec_init(lruvec: *mut lruvec);
}

extern "C" {
    pub fn container_of(_arg: lruvec, pglist_data: struct, _arg: __lruvec) -> return;
}

extern "C" {
    pub fn local_memory_node(node_id: c_int) -> c_int;
}

//
// zone_idx() returns 0 for the ZONE_DMA zone, 1 for the ZONE_NORMAL zone, etc.
//

//
// Returns true if a zone has pages managed by the buddy allocator.
// All the reclaim decisions have to use this function rather than
// populated_zone(). If the whole zone is reserved then we can easily
// end up with populated_zone() && !managed_zone().
//
extern "C" {
    pub fn zone_managed_pages(_arg: zone) -> return;
}
// Returns true if a zone has memory

//
// is_highmem - helper function to quickly check if a struct zone is a
// highmem zone or not.  This is an attempt to keep references
// to ZONE_{DMA/NORMAL/HIGHMEM/etc} in general code to a minimum.
// @zone: pointer to struct zone variable
// Return: 1 for a highmem zone, 0 otherwise
//
extern "C" {
    pub fn is_highmem_idx(_arg: zone_idx(zone)) -> return;
}
extern "C" {
    pub fn has_managed_zone(zone: zone_type) -> bool;
}

extern "C" {
    pub fn has_managed_zone(_arg: ZONE_DMA) -> return;
}

//
// for_each_online_pgdat - helper macro to iterate over all online nodes
// @pgdat: pointer to a pg_data_t variable
//

//
// for_each_zone - helper macro to iterate over all memory zones
// @zone: pointer to struct zone variable
//
// The user only needs to declare the zone variable, for_each_zone
// fills it in.
//

extern "C" {
    pub fn zone_to_nid(_arg: zoneref->zone) -> return;
}
//
// next_zones_zonelist - Returns the next zone at or below highest_zoneidx within the allowed nodemask using a cursor within a zonelist as a starting point
// @z: The cursor used as a starting point for the search
// @highest_zoneidx: The zone index of the highest zone to return
// @nodes: An optional nodemask to filter the zonelist with
//
// This function returns the next zone at or below a given zone index that is
// within the allowed nodemask using a cursor as the starting point for the
// search. The zoneref returned is a cursor that represents the current zone
// being examined. It should be advanced by one before calling
// next_zones_zonelist again.
//
// Return: the next zone at or below highest_zoneidx within the allowed
// nodemask using a cursor within a zonelist as a starting point
//
extern "C" {
    pub fn __next_zones_zonelist(_arg: z, _arg: highest_zoneidx, _arg: nodes) -> return;
}
//
// first_zones_zonelist - Returns the first zone at or below highest_zoneidx within the allowed nodemask in a zonelist
// @zonelist: The zonelist to search for a suitable zone
// @highest_zoneidx: The zone index of the highest zone to return
// @nodes: An optional nodemask to filter the zonelist with
//
// This function returns the first zone at or below a given zone index that is
// within the allowed nodemask. The zoneref returned is a cursor that can be
// used to iterate the zonelist with next_zones_zonelist by advancing it by
// one before calling.
//
// When no eligible zone is found, zoneref->zone is NULL (zoneref itself is
// never NULL). This may happen either genuinely, or due to concurrent nodemask
// update due to cpuset modification.
//
// Return: Zoneref pointer for the first suitable zone found
//
// for_each_zone_zonelist_nodemask - helper macro to iterate over valid zones in a zonelist at or below a given zone index and within a nodemask
// @zone: The current zone in the iterator
// @z: The current pointer within zonelist->_zonerefs being iterated
// @zlist: The zonelist being iterated
// @highidx: The zone index of the highest zone to return
// @nodemask: Nodemask allowed by the allocator
//
// This iterator iterates though all zones at or below a given zone index and
// within a given nodemask
//

//
// for_each_zone_zonelist - helper macro to iterate over valid zones in a zonelist at or below a given zone index
// @zone: The current zone in the iterator
// @z: The current pointer within zonelist->zones being iterated
// @zlist: The zonelist being iterated
// @highidx: The zone index of the highest zone to return
//
// This iterator iterates though all zones at or below a given zone index.
//

// Whether the 'nodes' are all movable nodes
//
// We can chose arbitrary node from the nodemask to get a
// zonelist as they are interlinked. We just need to find
// at least one zone that can satisfy kernel allocations.
//

//
// PA_SECTION_SHIFT		physical address to/from section number
// PFN_SECTION_SHIFT		pfn to/from section number
//

pub const SUBSECTION_SHIFT: c_int = 21;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_section_usage {
    pub rcu: rcu_head,

    pub SUBSECTIONS_PER_SECTION): DECLARE_BITMAP(subsection_map,,

// See declaration of similar field in struct zone
    pub pageblock_flags: [c_ulong; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_section {
//
// This is, logically, a pointer to an array of struct
// pages.  However, it is stored with some other magic.
// (see sparse_init_one_section())
//
// Additionally during early boot we encode node id of
// the location of the section here to guide allocation.
// (see sparse.c::memory_present())
//
// Making it a UL at least makes someone do a cast
// before using it wrong.
//
    pub section_mem_map: c_ulong,
    pub usage: *mut mem_section_usage,

//
// If SPARSEMEM, pgdat doesn't have page_ext pointer. We use
// section. (see page_ext.h about this.)
//
    pub page_ext: *mut page_ext,
    pub pad: c_ulong,

//
// WARNING: mem_section must be a power-of-2 in size for the
// calculation and use of SECTION_ROOT_MASK to make sense.
//
}

pub const SECTIONS_PER_ROOT: c_int = 1;

//
// We use the lower bits of the mem_map pointer to store a little bit of
// information. The pointer is calculated as mem_map - section_nr_to_pfn().
// The result is aligned to the minimum alignment of the two values:
//
// 1. All mem_map arrays are page-aligned.
// 2. section_nr_to_pfn() always clears PFN_SECTION_SHIFT lowest bits.
//
// We always expect a single section to cover full pages. Therefore,
// we can safely assume that PFN_SECTION_SHIFT is large enough to
// accommodate SECTION_MAP_LAST_BIT. We use BUILD_BUG_ON() to ensure this.
//

extern "C" {
    pub fn present_section(_arg: __nr_to_section(nr)) -> return;
}
extern "C" {
    pub fn valid_section(_arg: __nr_to_section(nr)) -> return;
}

extern "C" {
    pub fn sparse_vmemmap_init_nid_early(nid: c_int);
}

extern "C" {
    pub fn online_section(_arg: __nr_to_section(nr)) -> return;
}

extern "C" {
    pub fn online_mem_sections(start_pfn: c_ulong, end_pfn: c_ulong);
}
extern "C" {
    pub fn offline_mem_sections(start_pfn: c_ulong, end_pfn: c_ulong);
}

extern "C" {
    pub fn __nr_to_section(_arg: pfn_to_section_nr(pfn)) -> return;
}

// Find the next subsection that exists
// pfn = (*pfn & PAGE_SECTION_MASK) + (bit * PAGES_PER_SUBSECTION);

//
// pfn_valid - check if there is a valid memory map entry for a PFN
// @pfn: the page frame number to check
//
// Check if there is a valid memory map entry aka struct page for the @pfn.
// Note, that availability of the memory map entry does not imply that
// there is actual usable memory at that @pfn. The struct page may
// represent a hole or an unusable page frame.
//
// Return: 1 for PFNs that have memory map entries and 0 otherwise
//
// Ensure the upper PAGE_SHIFT bits are clear in the
// pfn. Else it might lead to false positives when
// some of the upper bits are set, but the lower bits
// match a valid pfn.
//
// Traditionally early sections always returned pfn_valid() for
// the entire section-sized span.
//
// Returns end_pfn or higher if no valid PFN remaining in range
// Nothing left in this section? Skip to next section
//
// Either every PFN within the section (or subsection for VMEMMAP) is
// valid, or none of them are. So there's no point repeating the check
// for every PFN; only call first_valid_pfn() again when crossing a
// (sub)section boundary (i.e. !(pfn & ~PAGE_{SUB,}SECTION_MASK)).
//
extern "C" {
    pub fn first_valid_pfn(_arg: pfn, _arg: end_pfn) -> return;
}

extern "C" {
    pub fn present_section(_arg: __pfn_to_section(pfn)) -> return;
}

//
// These are _only_ used during initialisation, therefore they
// can use __initdata ...  They could have names to indicate
// this restriction.
//

//
// Fallback case for when the architecture provides its own pfn_valid() but
// not a corresponding for_each_valid_pfn().
//

