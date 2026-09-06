//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mm_inline.h
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
// folio_is_file_lru - Should the folio be on a file LRU or anon LRU?
// @folio: The folio to test.
//
// We would like to get this info without a page flag, but the state
// needs to survive until the folio is last deleted from the LRU, which
// could be as far down as __page_cache_release.
//
// Return: An integer (not a boolean!) used to sort a folio onto the
// right LRU list and to account folios correctly.
// 1 if @folio is a regular filesystem backed page cache folio
// or a lazily freed anonymous folio (e.g. via MADV_FREE).
// 0 if @folio is a normal anonymous folio, a tmpfs folio or otherwise
// ram or swap backed folio.
//

//
// __folio_clear_lru_flags - Clear page lru flags before releasing a page.
// @folio: The folio that was on lru and now has a zero reference.
//
// this shouldn't happen, so leave the flags to bad_page()
//
// folio_lru_list - Which LRU list should a folio be on?
// @folio: The folio to test.
//
// Return: The LRU list a folio should be on, as an index
// into the array of LRU lists.
//

extern "C" {
    pub fn static_branch_unlikely(_arg: &lru_switch) -> return;
}

extern "C" {
    pub fn static_branch_likely(_arg: &lru_gen_caps[LRU_GEN_CORE]) -> return;
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &lru_gen_caps[LRU_GEN_CORE]) -> return;
}

// see the comment on MAX_NR_TIERS
//
// Return the total number of accesses including PG_referenced. Also see
// the comment on LRU_REFS_FLAGS.
//
// see the comment on MIN_NR_GENS
// addition
// deletion
// promotion
// demotion requires isolation, e.g., lru_deactivate_fn()
//
// +-----------------------------------+-----------------------------------+
// | Accessed through page tables and  | Accessed through file descriptors |
// | promoted by folio_update_gen()    | and protected by folio_inc_gen()  |
// +-----------------------------------+-----------------------------------+
// | PG_active (set while isolated)    |                                   |
// +-----------------+-----------------+-----------------+-----------------+
// |  PG_workingset  |  PG_referenced  |  PG_workingset  |  LRU_REFS_FLAGS |
// +-----------------------------------+-----------------------------------+
// |<---------- MIN_NR_GENS ---------->|                                   |
// |<---------------------------- MAX_NR_GENS ---------------------------->|
//
extern "C" {
    pub fn max(1: READ_ONCE(lrugen->max_seq) - gen +, _arg: READ_ONCE(lrugen->min_seq[type])) -> return;
}
// see the comment on MIN_NR_GENS about PG_active
// for folio_rotate_reclaimable()
// for folio_migrate_flags()

// This is not expected to be used on LRU_UNEVICTABLE

// mmap_lock should be read-locked
// Prevent anon_name refcount saturation early on
extern "C" {
    pub fn anon_vma_name_alloc(_arg: anon_name->name) -> return;
}
//
// Not using anon_vma_name because it generates a warning if mmap_lock
// is not held, which might be the case here.
//

extern "C" {
    pub fn pfnmap_track_ctx_release(ref: *mut kref);
}
//
// The only time this value is relevant is when there are indeed pages
// to flush. And we'll only flush pages after changing them, which
// requires the PTL.
//
// So the ordering here is:
//
// atomic_inc(&mm->tlb_flush_pending);
// spin_lock(&ptl);
// ...
// set_pte_at();
// spin_unlock(&ptl);
//
// spin_lock(&ptl)
// mm_tlb_flush_pending();
// ....
// spin_unlock(&ptl);
//
// flush_tlb_range();
// atomic_dec(&mm->tlb_flush_pending);
//
// Where the increment if constrained by the PTL unlock, it thus
// ensures that the increment is visible if the PTE modification is
// visible. After all, if there is no PTE modification, nobody cares
// about TLB flushes either.
//
// This very much relies on users (mm_tlb_flush_pending() and
// mm_tlb_flush_nested()) only caring about _specific_ PTEs (and
// therefore specific PTLs), because with SPLIT_PTE_PTLOCKS and RCpc
// locks (PPC) the unlock of one doesn't order against the lock of
// another PTL.
//
// The decrement is ordered by the flush_tlb_range(), such that
// mm_tlb_flush_pending() will not return false unless all flushes have
// completed.
//
// See inc_tlb_flush_pending().
//
// This cannot be smp_mb__before_atomic() because smp_mb() simply does
// not order against TLB invalidate completion, which is what we need.
//
// Therefore we must rely on tlb_flush_*() to guarantee order.
//
// Must be called after having acquired the PTL; orders against that
// PTLs release and therefore ensures that if we observe the modified
// PTE we must also observe the increment from inc_tlb_flush_pending().
//
// That is, it only guarantees to return true if there is a flush
// pending for _this_ PTL.
//
extern "C" {
    pub fn atomic_read(_arg: &mm->tlb_flush_pending) -> return;
}
//
// Similar to mm_tlb_flush_pending(), we must have acquired the PTL
// for which there is a TLB flush pending in order to guarantee
// we've seen both that PTE modification and the increment.
//
// (no requirement on actually still holding the PTL, that is irrelevant)
//

//
// Computes the pte marker to copy from the given source entry into dst_vma.
// If no marker should be copied, returns 0.
// The caller should insert a new pte created with make_pte_marker().
//
// Always copy error entries.
// Only copy PTE markers if UFFD register matches.

//
// num_pages_contiguous() - determine the number of contiguous pages
// that represent contiguous PFNs
// @pages: an array of page pointers
// @nr_pages: length of the array, at least 1
//
// Determine the number of contiguous pages that represent contiguous PFNs
// in @pages, starting from the first page.
//
// In some kernel configs contiguous PFNs will not have contiguous struct
// pages. In these configurations num_pages_contiguous() will return a num
// smaller than ideal number. The caller should continue to check for pfn
// contiguity after each call to num_pages_contiguous().
//
// Returns the number of contiguous pages.
//
// In unproblematic kernel configs, page_to_section() == 0 and
// the whole check will get optimized out.
//
