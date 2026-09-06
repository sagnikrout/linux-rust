//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/tlb.h
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
// include/asm-generic/tlb.h
//
// Generic TLB shootdown code
//
// Copyright 2001 Red Hat, Inc.
// Based on code from mm/memory.c Copyright Linus Torvalds and others.
//
// Copyright 2011 Red Hat, Inc., Peter Zijlstra
//

//
// Blindly accessing user memory from NMI context can be dangerous
// if we're in the middle of switching the current user task or switching
// the loaded mm.
//

//
// Generic MMU-gather implementation.
//
// The mmu_gather data structure is used by the mm code to implement the
// correct and efficient ordering of freeing pages and TLB invalidations.
//
// This correct ordering is:
//
// 1) unhook page
// 2) TLB invalidate page
// 3) free page
//
// That is, we must never free a page before we have ensured there are no live
// translations left to it. Otherwise it might be possible to observe (or
// worse, change) the page content after it has been reused.
//
// The mmu_gather API consists of:
//
// - tlb_gather_mmu() / tlb_gather_mmu_fullmm() / tlb_gather_mmu_vma()
// tlb_finish_mmu()
//
// start and finish a mmu_gather
//
// Finish in particular will issue a (final) TLB invalidate and free
// all (remaining) queued pages.
//
// - tlb_start_vma() / tlb_end_vma(); marks the start / end of a VMA
//
// Defaults to flushing at tlb_end_vma() to reset the range; helps when
// there's large holes between the VMAs.
//
// - tlb_free_vmas()
//
// tlb_free_vmas() marks the start of unlinking of one or more vmas
// and freeing page-tables.
//
// - tlb_remove_table()
//
// tlb_remove_table() is the basic primitive to free page-table directories
// (__p*_free_tlb()).  In it's most primitive form it is an alias for
// tlb_remove_page() below, for when page directories are pages and have no
// additional constraints.
//
// See also MMU_GATHER_TABLE_FREE and MMU_GATHER_RCU_TABLE_FREE.
//
// - tlb_remove_page() / tlb_remove_page_size()
// - __tlb_remove_folio_pages() / __tlb_remove_page_size()
// - __tlb_remove_folio_pages_size()
//
// __tlb_remove_folio_pages_size() is the basic primitive that queues pages
// for freeing. It will return a boolean indicating if the queue is (now)
// full and a call to tlb_flush_mmu() is required.
//
// tlb_remove_page() and tlb_remove_page_size() imply the call to
// tlb_flush_mmu() when required and has no return value.
//
// __tlb_remove_folio_pages() is similar to __tlb_remove_page_size(),
// however, instead of removing a single page, assume PAGE_SIZE and remove
// the given number of consecutive pages that are all part of the
// same (large) folio.
//
// - tlb_change_page_size()
//
// call before __tlb_remove_page*() to set the current page-size; implies a
// possible tlb_flush_mmu() call.
//
// - tlb_flush_mmu() / tlb_flush_mmu_tlbonly()
//
// tlb_flush_mmu_tlbonly() - does the TLB invalidate (and resets
// related state, like the range)
//
// tlb_flush_mmu() - in addition to the above TLB invalidate, also frees
// whatever pages are still batched.
//
// - mmu_gather::fullmm
//
// A flag set by tlb_gather_mmu_fullmm() to indicate we're going to free
// the entire mm; this allows a number of optimizations.
//
// - We can ignore tlb_{start,end}_vma(); because we don't
// care about ranges. Everything will be shot down.
//
// - (RISC) architectures that use ASIDs can cycle to a new ASID
// and delay the invalidation until ASID space runs out.
//
// - mmu_gather::need_flush_all
//
// A flag that can be set by the arch code if it wants to force
// flush the entire TLB irrespective of the range. For instance
// x86-PAE needs this when changing top-level entries.
//
// And allows the architecture to provide and implement tlb_flush():
//
// tlb_flush() may, in addition to the above mentioned mmu_gather fields, make
// use of:
//
// - mmu_gather::start / mmu_gather::end
//
// which provides the range that needs to be flushed to cover the pages to
// be freed.
//
// - mmu_gather::freed_tables
//
// set when we freed page table pages
//
// - tlb_get_unmap_shift() / tlb_get_unmap_size()
//
// returns the smallest TLB entry size unmapped in this range.
//
// If an architecture does not provide tlb_flush() a default implementation
// based on flush_tlb_range() will be used, unless MMU_GATHER_NO_RANGE is
// specified, in which case we'll default to flush_tlb_mm().
//
// Additionally there are a few opt-in features:
//
// MMU_GATHER_PAGE_SIZE
//
// This ensures we call tlb_flush() every time tlb_change_page_size() actually
// changes the size and provides mmu_gather::page_size to tlb_flush().
//
// This might be useful if your architecture has size specific TLB
// invalidation instructions.
//
// MMU_GATHER_TABLE_FREE
//
// This provides tlb_remove_table(), to be used instead of tlb_remove_page()
// for page directores (__p*_free_tlb()).
//
// Useful if your architecture has non-page page directories.
//
// When used, an architecture is expected to provide __tlb_remove_table() or
// use the generic __tlb_remove_table(), which does the actual freeing of these
// pages.
//
// MMU_GATHER_RCU_TABLE_FREE
//
// Like MMU_GATHER_TABLE_FREE, and adds semi-RCU semantics to the free (see
// comment below).
//
// Useful if your architecture doesn't use IPIs for remote TLB invalidates
// and therefore doesn't naturally serialize with software page-table walkers.
//
// MMU_GATHER_NO_FLUSH_CACHE
//
// Indicates the architecture has flush_cache_range() but it needs *NOT* be called
// before unmapping a VMA.
//
// NOTE: strictly speaking we shouldn't have this knob and instead rely on
// flush_cache_range() being a NOP, except Sparc64 seems to be
// different here.
//
// MMU_GATHER_MERGE_VMAS
//
// Indicates the architecture wants to merge ranges over VMAs; typical when
// multiple range invalidates are more expensive than a full invalidate.
//
// MMU_GATHER_NO_RANGE
//
// Use this if your architecture lacks an efficient flush_tlb_range(). This
// option implies MMU_GATHER_MERGE_VMAS above.
//
// MMU_GATHER_NO_GATHER
//
// If the option is set the mmu_gather will not track individual pages for
// delayed page free anymore. A platform that enables the option needs to
// provide its own implementation of the __tlb_remove_page_size() function to
// free pages.
//
// This is useful if your architecture already flushes TLB entries in the
// various ptep_get_and_clear() functions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_table_batch {

    pub rcu: rcu_head,

    pub nr: c_uint,
    pub tables: [*mut c_void; ],
}

extern "C" {
    pub fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut c_void);
}

extern "C" {
    pub fn tlb_remove_page(tlb: *mut mmu_gather, page: *mut page);
}
//
// Without MMU_GATHER_TABLE_FREE the architecture is assumed to have page based
// page directories and we can use the normal page batching to free them.
//

//
// This allows an architecture that does not use the linux page-tables for
// hardware to skip the TLBI when freeing page tables.
//

extern "C" {
    pub fn tlb_remove_table_sync_one();
}
extern "C" {
    pub fn tlb_remove_table_sync_rcu();
}

//
// If we can't allocate a page to make a big batch of page pointers
// to work on, then just handle a few from the on-stack structure.
//
pub const MMU_GATHER_BUNDLE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_gather_batch {
    pub next: *mut mmu_gather_batch,
    pub nr: c_uint,
    pub max: c_uint,
    pub encoded_pages: [*mut encoded_page; ],
}

//
// Limit the maximum number of mmu_gather batches to reduce a risk of soft
// lockups for non-preemptible kernels on huge machines when a lot of memory
// is zapped during unmapping.
// 10K pages freed at once should be safe even without a preemption point.
//

extern "C" {
    pub fn __tlb_remove_page_size(tlb: *mut mmu_gather, page: *mut page, page_size: c_int) -> bool;
}

//
// This both sets 'delayed_rmap', and returns true. It would be an inline
// function, except we define it before the 'struct mmu_gather'.
//

extern "C" {
    pub fn tlb_flush_rmaps(tlb: *mut mmu_gather, vma: *mut vm_area_struct);
}

//
// We have a no-op version of the rmap removal that doesn't
// delay anything. That is used on S390, which flushes remote
// TLBs synchronously, and on UP, which doesn't have any
// remote TLBs to flush and is not preemptible due to this
// all happening under the page table lock.
//

//
// struct mmu_gather is an opaque type used by the mm code for passing around
// any data needed by arch specific code for tlb_remove_page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_gather {
    pub mm: *mut mm_struct,

    pub batch: *mut mmu_table_batch,

    pub start: c_ulong,
    pub end: c_ulong,
//
// we are in the middle of an operation to clear
// a full mm and can make some optimizations
//
    pub 1: unsigned int fullmm :,
//
// we have performed an operation which
// requires a complete flush of the tlb
//
    pub 1: unsigned int need_flush_all :,
//
// we have removed page directories
//
    pub 1: unsigned int freed_tables :,
//
// Do we have pending delayed rmap removals?
//
    pub 1: unsigned int delayed_rmap :,
//
// at which levels have we cleared entries?
//
    pub 1: unsigned int cleared_ptes :,
    pub 1: unsigned int cleared_pmds :,
    pub 1: unsigned int cleared_puds :,
    pub 1: unsigned int cleared_p4ds :,
//
// tracks VM_EXEC | VM_HUGETLB in tlb_start_vma
//
    pub 1: unsigned int vma_exec :,
    pub 1: unsigned int vma_huge :,
    pub 1: unsigned int vma_pfn :,
//
// Did we unshare (unmap) any shared page tables? For now only
// used for hugetlb PMD table sharing.
//
    pub 1: unsigned int unshared_tables :,
//
// Did we unshare any page tables such that they are now exclusive
// and could get reused+modified by the new owner? When setting this
// flag, "unshared_tables" will be set as well. For now only used
// for hugetlb PMD table sharing.
//
    pub 1: unsigned int fully_unshared_tables :,
    pub batch_count: c_uint,

    pub active: *mut mmu_gather_batch,
    pub local: mmu_gather_batch,
    pub __pages: [*mut page; MMU_GATHER_BUNDLE],
    pub page_size: c_uint,

}

extern "C" {
    pub fn tlb_flush_mmu(tlb: *mut mmu_gather);
}
//
// Do not reset mmu_gather::vma_* fields here, we do not
// call into tlb_start_vma() again to set them if there is an
// intermediate flush.
//

//
// When an architecture does not have efficient means of range flushing TLBs
// there is no point in doing intermediate flushes on tlb_end_vma() to keep the
// range small. We equally don't have to worry about page granularity or other
// things.
//
// All we need to do is issue a full flush for any !0 range.
//

//
// When an architecture does not provide its own tlb_flush() implementation
// but does have a reasonably efficient flush_vma_range() implementation
// use that.
//

//
// flush_tlb_range() implementations that look at VM_HUGETLB (tile,
// mips-4k) flush only large pages.
//
// flush_tlb_range() implementations that flush I-TLB also flush D-TLB
// (tile, xtensa, arm), so it's ok to just add VM_EXEC to an existing
// range.
//
// We rely on tlb_end_vma() to issue a flush, such that when we reset
// these values the batch is empty.
//
// Track if there's at least one VM_PFNMAP/VM_MIXEDMAP vma
// in the tracked range, see tlb_free_vmas().
//
// Anything calling __tlb_adjust_range() also sets at least one of
// these bits.
//
extern "C" {
    pub fn tlb_remove_page_size(_arg: tlb, _arg: page, _arg: PAGE_SIZE) -> return;
}

//
// In the case of tlb vma handling, we can optimise these away in the
// case where we're doing a full MM flush.  When we're doing a munmap,
// the vmas are adjusted to only cover the region to be torn down.
//

//
// Do a TLB flush and reset the range at VMA boundaries; this avoids
// the ranges growing with the unused space between consecutive VMAs,
// but also the mmu_gather::vma_* flags from tlb_start_vma() rely on
// this.
//
// VM_PFNMAP is more fragile because the core mm will not track the
// page mapcount -- there might not be page-frames for these PFNs
// after all.
//
// Specifically() there is a race between munmap() and
// unmap_mapping_range(), where munmap() will unlink the VMA, such
// that unmap_mapping_range() will no longer observe the VMA and
// no-op, without observing the TLBI, returning prematurely.
//
// So if we're about to unlink such a VMA, and we have pending
// TLBI for such a vma, flush things now.
//
// tlb_flush_{pte|pmd|pud|p4d}_range() adjust the tlb->start and tlb->end,
// and set corresponding cleared_*.
//

//
// tlb_remove_tlb_entry - remember a pte unmapping for later tlb invalidation.
//
// Record the fact that pte's were really unmapped by updating the range,
// so we can later optimise away the tlb invalidate.   This helps when
// userspace is unmapping already-unmapped pages, which happens quite a lot.
//

//
// tlb_remove_tlb_entries - remember unmapping of multiple consecutive ptes for
// later tlb invalidation.
//
// Similar to tlb_remove_tlb_entry(), but remember unmapping of multiple
// consecutive ptes instead of only a single one.
//

//
// tlb_remove_pmd_tlb_entry - remember a pmd mapping for later tlb invalidation
// This is a nop so far, because only x86 needs it.
//

//
// tlb_remove_pud_tlb_entry - remember a pud mapping for later tlb
// invalidation. This is a nop so far, because only x86 needs it.
//

//
// For things like page tables caches (ie caching addresses "inside" the
// page tables, like x86 does), for legacy reasons, flushing an
// individual page had better flush the page table caches behind it. This
// is definitely how x86 works, for example. And if you have an
// architected non-legacy page table cache (which I'm not aware of
// anybody actually doing), you're going to have some architecturally
// explicit flushing for that, likely *separate* from a regular TLB entry
// flush, and thus you'd need more than just some range expansion..
//
// So if we ever find an architecture
// that would want something that odd, I think it is up to that
// architecture to do its own odd thing, not cause pain for others
// http://lkml.kernel.org/r/CA+55aFzBggoXtNXQeng5d_mRoDnaMBE5Y+URs+PHR67nUpMtaw@mail.gmail.com
//
// For now w.r.t page table cache, mark the range_size as PAGE_SIZE
//

//
// The caller must make sure that concurrent unsharing + exclusive
// reuse is impossible until tlb_flush_unshared_tables() was called.
//
// Clearing a PUD pointing at a PMD table with PMD leaves.
//
// If the page table is now exclusively owned, we fully unshared
// a page table.
//
// As soon as the caller drops locks to allow for reuse of
// previously-shared tables, these tables could get modified and
// even reused outside of hugetlb context, so we have to make sure that
// any page table walkers (incl. TLB, GUP-fast) are aware of that
// change.
//
// Even if we are not fully unsharing a PMD table, we must
// flush the TLB for the unsharer now.
//
// Similarly, we must make sure that concurrent GUP-fast will not
// walk previously-shared page tables that are getting modified+reused
// elsewhere. So broadcast an IPI to wait for any concurrent GUP-fast.
//
// We only perform this when we are the last sharer of a page table,
// as the IPI will reach all CPUs: any GUP-fast.
//
// Note that on configs where tlb_remove_table_sync_one() is a NOP,
// the expectation is that the tlb_flush_mmu_tlbonly() would have issued
// required IPIs already for us.
//

