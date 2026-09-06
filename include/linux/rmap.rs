//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rmap.h
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
// Declarations for Reverse Mapping functions in mm/rmap.c
//

//
// The anon_vma heads a list of private "related" vmas, to scan if
// an anonymous page pointing to this anon_vma needs to be unmapped:
// the vmas on the list will be related by forking, or by splitting.
//
// Since vmas come and go as they are split and merged (particularly
// in mprotect), the mapping field of an anonymous page cannot point
// directly to a vma: instead it points to an anon_vma, on whose list
// the related vmas can be easily linked or unlinked.
//
// After unlinking the last vma on the list, we must garbage collect
// the anon_vma object itself: we're guaranteed no page can be
// pointing to this anon_vma once its vma list is empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_vma {
    pub /: *mut *mut *mut anon_vma root; / Root of this anon_vma tree,
    pub /: *mut *mut rw_semaphore rwsem; / W: modification, R: walking the list,
//
// The refcount is taken on an anon_vma when there is no
// guarantee that the vma of page tables will exist for
// the duration of the operation. A caller that takes
// the reference is responsible for clearing up the
// anon_vma if they are the last user on release
//
    pub refcount: core::sync::atomic::AtomicI32,
//
// Count of child anon_vmas. Equals to the count of all anon_vmas that
// have ->parent pointing to this one, including itself.
//
// This counter is used for making decision about reusing anon_vma
// instead of forking new one. See comments in function anon_vma_clone.
//
    pub num_children: c_ulong,
// Count of VMAs whose ->anon_vma pointer points to this object.
    pub num_active_vmas: c_ulong,
    pub /: *mut *mut *mut anon_vma parent; / Parent of this anon_vma,
//
// NOTE: the LSB of the rb_root.rb_node is set by
// mm_take_all_locks() _after_ taking the above lock. So the
// rb_root must only be read/written after taking the above lock
// to be sure to see a valid next pointer. The LSB bit itself
// is serialized by a system wide lock only visible to
// mm_take_all_locks() (mm_all_locks_mutex).
//
// Interval tree of private "related" vmas
    pub rb_root: rb_root_cached,
}

//
// The copy-on-write semantics of fork mean that an anon_vma
// can become associated with multiple processes. Furthermore,
// each child process will have its own anon_vma, where new
// pages for that process are instantiated.
//
// This structure allows us to find the anon_vmas associated
// with a VMA, or the VMAs associated with an anon_vma.
// The "same_vma" list contains the anon_vma_chains linking
// all the anon_vmas associated with this VMA.
// The "rb" field indexes on an interval tree the anon_vma_chains
// which link all the VMAs associated with this anon_vma.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_vma_chain {
    pub vma: *mut vm_area_struct,
    pub anon_vma: *mut anon_vma,
    pub /: *mut *mut list_head same_vma; / locked by mmap_lock & page_table_lock,
    pub /: *mut *mut rb_node rb; / locked by anon_vma->rwsem,
    pub rb_subtree_last: c_ulong,

    pub cached_vma_last: unsigned long cached_vma_start,,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttu_flags {
    TTU_USE_SHARED_ZEROPAGE	= 0x2,	/* for unused pages of large folios */
    TTU_SPLIT_HUGE_PMD	= 0x4,	/* split huge PMD if any */
    TTU_IGNORE_MLOCK	= 0x8,	/* ignore mlock */
    TTU_SYNC		= 0x10,	/* avoid racy checks with PVMW_SYNC */
    TTU_HWPOISON		= 0x20,	/* do convert pte to hwpoison entry */
    TTU_BATCH_FLUSH		= 0x40,	/* Batch TLB flushes where possible
// and caller guarantees they will
// do a final flush if necessary
    TTU_RMAP_LOCKED		= 0x80,	/* do not grab rmap lock:
// caller holds it
}

//
// Make sure we can detect at least one complete PTE mapping of the
// folio in a single MM as "exclusively mapped". This is primarily
// a check on 32bit, where we currently reduce the size of the per-MM
// mapcount to a short.
//
// Note: mapcounts start at -1.
//
// If a folio is mapped more than once into an MM on 32bit, we
// can in theory overflow the per-MM mapcount (although only for
// fairly large folios), turning it negative. In that case, just
// free up the slot and mark the folio "mapped shared", otherwise
// we might be in trouble when unmapping pages later.
//
// We might have other mappings already.
// Slot 0 certainly has mappings as well.

//
// There are valid corner cases where we might underflow a per-MM
// mapcount (some mappings added when no slot was free, some mappings
// added once a slot was free), so we always set it to -1 once we go
// negative.
//
// If one MM slot owns all mappings, the folio is mapped exclusively.
// Note that if the folio is now unmapped (new_mapcount_val == -1), both
// slots must be free (mapcount == -1), and we'll also mark it as
// exclusive.
//

//
// See __folio_rmap_sanity_checks(), we might map large folios even without
// CONFIG_TRANSPARENT_HUGEPAGE. We'll keep that working for now.
//
// Note: mapcounts start at -1.

// RMAP flags, currently only relevant for some anon rmap operations.
pub type rmap_t = int ;
//
// No special request: A mapped anonymous (sub)page is possibly shared between
// processes.
//

// The anonymous (sub)page is exclusive to a single process.

// hugetlb folios are handled separately.
// When (un)mapping zeropages, we should never touch ref+mapcount.
//
// TODO: we get driver-allocated folios that have nothing to do with
// the rmap using vm_insert_page(); therefore, we cannot assume that
// folio_test_large_rmappable() holds for large folios. We should
// handle any desired mapcount+stats accounting for these folios in
// VM_MIXEDMAP VMAs separately, and then sanity-check here that
// we really only get rmappable folios.
//
// We don't support folios larger than a single PMD yet. So
// when PGTABLE_LEVEL_PMD is set, we assume that we are creating
// a single "entire" mapping of the folio.
//
// Assume that we are creating a single "entire" mapping of the
// folio.
//
// Anon folios must have an associated live anon_vma as long as they're
// mapped into userspace.
// Note that the atomic_read() mainly does two things:
//
// 1. In KASAN builds with CONFIG_SLUB_RCU_DEBUG, it causes KASAN to
// check that the associated anon_vma has not yet been freed (subject
// to KASAN's usual limitations). This check will pass if the
// anon_vma's refcount has already dropped to 0 but an RCU grace
// period hasn't passed since then.
// 2. If the anon_vma has not yet been freed, it checks that the
// anon_vma still has a nonzero refcount (as opposed to being in the
// middle of an RCU delay for getting freed).
//
// rmap interfaces called when adding or removing pte of page
//
extern "C" {
    pub fn folio_move_anon_rmap(: *mut folio, : *mut vm_area_struct);
}

// See folio_try_dup_anon_rmap_*()
// See folio_try_share_anon_rmap_*()
// Paired with the memory barrier in try_grab_folio().
//
// This is conceptually a smp_wmb() paired with the smp_rmb() in
// gup_must_unshare().
//
// folio_dup_file_rmap_ptes - duplicate PTE mappings of a page range of a folio
// @folio:	The folio to duplicate the mappings of
// @page:	The first page to duplicate the mappings of
// @nr_pages:	The number of pages of which the mapping will be duplicated
// @dst_vma:	The destination vm area
//
// The page range of the folio is defined by [page, page + nr_pages)
//
// The caller needs to hold the page table lock.
//
// folio_dup_file_rmap_pmd - duplicate a PMD mapping of a page range of a folio
// @folio:	The folio to duplicate the mapping of
// @page:	The first page to duplicate the mapping of
// @dst_vma:	The destination vm area
//
// The page range of the folio is defined by [page, page + HPAGE_PMD_NR)
//
// The caller needs to hold the page table lock.
//

//
// If this folio may have been pinned by the parent process,
// don't allow to duplicate the mappings but instead require to e.g.,
// copy the subpage immediately for the child so that we'll always
// guarantee the pinned folio won't be randomly replaced in the
// future on write faults.
//
// No need to check+clear for already shared PTEs/PMDs of the
// folio. But if any page is PageAnonExclusive, we must fallback to
// copying if the folio maybe pinned.
//
// folio_try_dup_anon_rmap_ptes - try duplicating PTE mappings of a page range
// of a folio
// @folio:	The folio to duplicate the mappings of
// @page:	The first page to duplicate the mappings of
// @nr_pages:	The number of pages of which the mapping will be duplicated
// @dst_vma:	The destination vm area
// @src_vma:	The vm area from which the mappings are duplicated
//
// The page range of the folio is defined by [page, page + nr_pages)
//
// The caller needs to hold the page table lock and the
// vma->vma_mm->write_protect_seq.
//
// Duplicating the mappings can only fail if the folio may be pinned; device
// private folios cannot get pinned and consequently this function cannot fail
// for them.
//
// If duplicating the mappings succeeded, the duplicated PTEs have to be R/O in
// the parent and the child. They must *not* be writable after this call
// succeeded.
//
// Returns 0 if duplicating the mappings succeeded. Returns -EBUSY otherwise.
//
// folio_try_dup_anon_rmap_pmd - try duplicating a PMD mapping of a page range
// of a folio
// @folio:	The folio to duplicate the mapping of
// @page:	The first page to duplicate the mapping of
// @dst_vma:	The destination vm area
// @src_vma:	The vm area from which the mapping is duplicated
//
// The page range of the folio is defined by [page, page + HPAGE_PMD_NR)
//
// The caller needs to hold the page table lock and the
// vma->vma_mm->write_protect_seq.
//
// Duplicating the mapping can only fail if the folio may be pinned; device
// private folios cannot get pinned and consequently this function cannot fail
// for them.
//
// If duplicating the mapping succeeds, the duplicated PMD has to be R/O in
// the parent and the child. They must *not* be writable after this call
// succeeded.
//
// Returns 0 if duplicating the mapping succeeded. Returns -EBUSY otherwise.
//

// device private folios cannot get pinned via GUP.
//
// We have to make sure that when we clear PageAnonExclusive, that
// the page is not pinned and that concurrent GUP-fast won't succeed in
// concurrently pinning the page.
//
// Conceptually, PageAnonExclusive clearing consists of:
// (A1) Clear PTE
// (A2) Check if the page is pinned; back off if so.
// (A3) Clear PageAnonExclusive
// (A4) Restore PTE (optional, but certainly not writable)
//
// When clearing PageAnonExclusive, we cannot possibly map the page
// writable again, because anon pages that may be shared must never
// be writable. So in any case, if the PTE was writable it cannot
// be writable anymore afterwards and there would be a PTE change. Only
// if the PTE wasn't writable, there might not be a PTE change.
//
// Conceptually, GUP-fast pinning of an anon page consists of:
// (B1) Read the PTE
// (B2) FOLL_WRITE: check if the PTE is not writable; back off if so.
// (B3) Pin the mapped page
// (B4) Check if the PTE changed by re-reading it; back off if so.
// (B5) If the original PTE is not writable, check if
// PageAnonExclusive is not set; back off if so.
//
// If the PTE was writable, we only have to make sure that GUP-fast
// observes a PTE change and properly backs off.
//
// If the PTE was not writable, we have to make sure that GUP-fast either
// detects a (temporary) PTE change or that PageAnonExclusive is cleared
// and properly backs off.
//
// Consequently, when clearing PageAnonExclusive(), we have to make
// sure that (A1), (A2)/(A3) and (A4) happen in the right memory
// order. In GUP-fast pinning code, we have to make sure that (B3),(B4)
// and (B5) happen in the right memory order.
//
// We assume that there might not be a memory barrier after
// clearing/invalidating the PTE (A1) and before restoring the PTE (A4),
// so we use explicit ones here.
//
// Paired with the memory barrier in try_grab_folio().
//
// This is conceptually a smp_wmb() paired with the smp_rmb() in
// gup_must_unshare().
//
// folio_try_share_anon_rmap_pte - try marking an exclusive anonymous page
// mapped by a PTE possibly shared to prepare
// for KSM or temporary unmapping
// @folio:	The folio to share a mapping of
// @page:	The mapped exclusive page
//
// The caller needs to hold the page table lock and has to have the page table
// entries cleared/invalidated.
//
// This is similar to folio_try_dup_anon_rmap_pte(), however, not used during
// fork() to duplicate mappings, but instead to prepare for KSM or temporarily
// unmapping parts of a folio (swap, migration) via folio_remove_rmap_pte().
//
// Marking the mapped page shared can only fail if the folio maybe pinned;
// device private folios cannot get pinned and consequently this function cannot
// fail.
//
// Returns 0 if marking the mapped page possibly shared succeeded. Returns
// -EBUSY otherwise.
//
extern "C" {
    pub fn __folio_try_share_anon_rmap(_arg: folio, _arg: page, _arg: 1, _arg: PGTABLE_LEVEL_PTE) -> return;
}
//
// folio_try_share_anon_rmap_pmd - try marking an exclusive anonymous page
// range mapped by a PMD possibly shared to
// prepare for temporary unmapping
// @folio:	The folio to share the mapping of
// @page:	The first page to share the mapping of
//
// The page range of the folio is defined by [page, page + HPAGE_PMD_NR)
//
// The caller needs to hold the page table lock and has to have the page table
// entries cleared/invalidated.
//
// This is similar to folio_try_dup_anon_rmap_pmd(), however, not used during
// fork() to duplicate a mapping, but instead to prepare for temporarily
// unmapping parts of a folio (swap, migration) via folio_remove_rmap_pmd().
//
// Marking the mapped pages shared can only fail if the folio maybe pinned;
// device private folios cannot get pinned and consequently this function cannot
// fail.
//
// Returns 0 if marking the mapped pages possibly shared succeeded. Returns
// -EBUSY otherwise.
//

//
// Called from mm/vmscan.c to handle paging out
//
extern "C" {
    pub fn try_to_migrate(folio: *mut folio, flags: ttu_flags);
}
extern "C" {
    pub fn try_to_unmap(: *mut folio, flags: ttu_flags);
}
// Avoid racy checks

// Look for migration entries rather than present PTEs

// Result flags
// The page is mapped across page table boundary

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_vma_mapped_walk {
    pub pfn: c_ulong,
    pub nr_pages: c_ulong,
    pub /: *mut *mut pgoff_t pgoff; / Only meaningful if nr_pages > 1 and not a KSM walk,
    pub vma: *mut vm_area_struct,
    pub address: c_ulong,
    pub pmd: *mut pmd_t,
    pub pte: *mut pte_t,
    pub ptl: *mut spinlock_t,
    pub flags: c_uint,
    pub 1: bool pgoff_is_anon :,
}

// HugeTLB pte is set to the relevant page table entry without pte_mapped.
//
// page_vma_mapped_walk_restart - Restart the page table walk.
// @pvmw: Pointer to struct page_vma_mapped_walk.
//
// It restarts the page table walk when changes occur in the page
// table, such as splitting a PMD. Ensures that the PTL held during
// the previous walk is released and resets the state to allow for
// a new walk starting at the current address stored in pvmw->address.
//
extern "C" {
    pub fn page_vma_mapped_walk(pvmw: *mut page_vma_mapped_walk) -> bool;
}
//
// Cleans the PTEs of shared mappings.
// (and since clean PTEs should also be readonly, write protects them too)
//
// returns the number of cleaned PTEs.
//
extern "C" {
    pub fn folio_mkclean(: *mut folio) -> c_int;
}
//
// rmap_walk_control: To control rmap traversing for specific needs
//
// arg: passed to rmap_one() and invalid_vma()
// try_lock: bail out if the rmap lock is contended
// contended: indicate the rmap traversal bailed out due to lock contention
// rmap_one: executed on each vma where page is mapped
// done: for checking traversing termination condition
// anon_lock: for getting anon_lock by optimized way rather than default
// invalid_vma: for skipping uninterested vma
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmap_walk_control {
    pub arg: *mut c_void,
    pub try_lock: bool,
    pub contended: bool,
//
// Return false if page table scanning in rmap_walk should be stopped.
// Otherwise, return true.
//
    pub arg): *mut unsigned long addr, void,
    pub folio): *mut *mut int (done)(struct folio,
    pub rwc): *mut rmap_walk_control,
    pub arg): *mut *mut *mut bool (invalid_vma)(struct vm_area_struct vma, void,
}

extern "C" {
    pub fn rmap_walk(folio: *mut folio, rwc: *mut rmap_walk_control);
}
extern "C" {
    pub fn rmap_walk_locked(folio: *mut folio, rwc: *mut rmap_walk_control);
}

