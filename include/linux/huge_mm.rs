//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/huge_mm.h
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

extern "C" {
    pub fn do_huge_pmd_anonymous_page(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn huge_pmd_set_accessed(vmf: *mut vm_fault) -> bool;
}

extern "C" {
    pub fn huge_pud_set_accessed(vmf: *mut vm_fault, orig_pud: pud_t);
}

extern "C" {
    pub fn do_huge_pmd_wp_page(vmf: *mut vm_fault) -> vm_fault_t;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transparent_hugepage_flag {
    TRANSPARENT_HUGEPAGE_UNSUPPORTED,
    TRANSPARENT_HUGEPAGE_FLAG,
    TRANSPARENT_HUGEPAGE_REQ_MADV_FLAG,
    TRANSPARENT_HUGEPAGE_DEFRAG_DIRECT_FLAG,
    TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_FLAG,
    TRANSPARENT_HUGEPAGE_DEFRAG_KSWAPD_OR_MADV_FLAG,
    TRANSPARENT_HUGEPAGE_DEFRAG_REQ_MADV_FLAG,
    TRANSPARENT_HUGEPAGE_DEFRAG_KHUGEPAGED_FLAG,
    TRANSPARENT_HUGEPAGE_USE_ZERO_PAGE_FLAG,
}

//
// Mask of all large folio orders supported for anonymous THP; all orders up to
// and including PMD_ORDER, except order-0 (which is not "huge") and order-1
// (which is a limitation of the THP implementation).
//

//
// Mask of all large folio orders supported for file THP. Folios in a DAX
// file is never split and the MAX_PAGECACHE_ORDER limit does not apply to
// it.  Same to PFNMAPs where there's neither page* nor pagecache.
//

//
// Mask of all large folio orders supported for THP.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tva_type {
    TVA_SMAPS,		/* Exposing "THPeligible:" in smaps. */
    TVA_PAGEFAULT,		/* Serving a page fault. */
    TVA_KHUGEPAGED,		/* Khugepaged collapse. */
    TVA_FORCED_COLLAPSE,	/* Forced collapse (e.g. MADV_COLLAPSE). */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mthp_stat_item {
    MTHP_STAT_ANON_FAULT_ALLOC,
    MTHP_STAT_ANON_FAULT_FALLBACK,
    MTHP_STAT_ANON_FAULT_FALLBACK_CHARGE,
    MTHP_STAT_COLLAPSE_ALLOC,
    MTHP_STAT_COLLAPSE_ALLOC_FAILED,
    MTHP_STAT_ZSWPOUT,
    MTHP_STAT_SWPIN,
    MTHP_STAT_SWPIN_FALLBACK,
    MTHP_STAT_SWPIN_FALLBACK_CHARGE,
    MTHP_STAT_SWPOUT,
    MTHP_STAT_SWPOUT_FALLBACK,
    MTHP_STAT_SHMEM_ALLOC,
    MTHP_STAT_SHMEM_FALLBACK,
    MTHP_STAT_SHMEM_FALLBACK_CHARGE,
    MTHP_STAT_SPLIT,
    MTHP_STAT_SPLIT_FAILED,
    MTHP_STAT_SPLIT_DEFERRED,
    MTHP_STAT_NR_ANON,
    MTHP_STAT_NR_ANON_PARTIALLY_MAPPED,
    MTHP_STAT_COLLAPSE_EXCEED_SWAP,
    MTHP_STAT_COLLAPSE_EXCEED_NONE,
    MTHP_STAT_COLLAPSE_EXCEED_SHARED,
    __MTHP_STAT_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthp_stat {
    pub 1][__MTHP_STAT_COUNT]: unsigned long stats[ilog2(MAX_PTRS_PER_PTE) +,
}

// orders &= ~BIT(prev);
extern "C" {
    pub fn highest_order(_arg: *mut orders) -> return;
}
//
// Do the below checks:
// - For file vma, check if the linear page offset of vma is
// order-aligned within the file.  The hugepage is
// guaranteed to be order-aligned within the file, but we must
// check that the order-aligned addresses in the VMA map to
// order-aligned offsets within the file, else the hugepage will
// not be mappable.
// - For all vmas, check if the haddr is in an aligned hugepage
// area.
//
// Don't have to check pgoff for anonymous vma
// vma_start_pgoff() in mm.h so not available.
//
// Make sure huge_gfp is always more limited than limit_gfp.
// Some shmem users want THP allocation to be done less aggressively
// and only in certain zone.
//
// Allow allocations only from the originally specified zones.
//
// Minimize the result gfp by taking the union with the deny flags,
// and the intersection of the allow flags.
//
// Filter the bitfield of input orders to the ones suitable for use in the vma.
// See thp_vma_suitable_order().
// All orders that pass the checks are returned as a bitfield.
//
// Iterate over orders, highest to lowest, removing orders that don't
// meet alignment requirements from the set. Exit loop at first order
// that meets requirements, since all lower orders must also meet
// requirements.
//
// thp_vma_allowable_orders - determine hugepage orders that are allowed for vma
// @vma:  the vm area to check
// @vm_flags: use these vm_flags instead of vma->vm_flags
// @type: TVA type
// @orders: bitfield of all orders to consider
//
// Calculates the intersection of the requested hugepage orders and the allowed
// hugepage orders for the provided vma. Permitted orders are encoded as a set
// bit at the corresponding bit position (bit-2 corresponds to order-2, bit-3
// corresponds to order-3, etc). Order-0 is never considered a hugepage order.
//
// Return: bitfield of orders allowed for hugepage in the vma. 0 if no hugepage
// orders are allowed.
//
// Optimization to check if required orders are enabled early. Only
// forced collapse ignores sysfs configs.
//
extern "C" {
    pub fn __thp_vma_allowable_orders(_arg: vma, _arg: vm_flags, _arg: type, _arg: orders) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thpsize {
    pub kobj: kobject,
    pub node: list_head,
    pub order: c_int,
}

//
// Check whether THPs are explicitly disabled for this VMA, for example,
// through madvise or prctl.
//
// Are THPs disabled for this VMA?
// Are THPs disabled for all VMAs in the whole process?
//
// Are THPs disabled only for VMAs where we didn't get an explicit
// advise to use them?
//
// Forcing a collapse (e.g., madv_collapse), is a clear advice to
// use THPs.
//
extern "C" {
    pub fn mm_flags_test(_arg: MMF_DISABLE_THP_EXCEPT_ADVISED, _arg: vma->vm_mm) -> return;
}
// If the hardware/firmware marked hugepage support disabled.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum split_type {
    SPLIT_TYPE_UNIFORM,
    SPLIT_TYPE_NON_UNIFORM,
}

extern "C" {
    pub fn folio_split_unmapped(folio: *mut folio, new_order: c_uint) -> c_int;
}
extern "C" {
    pub fn min_order_for_split(folio: *mut folio) -> c_uint;
}
extern "C" {
    pub fn split_folio_to_list(folio: *mut folio, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn __split_huge_page_to_list_to_order(_arg: page, _arg: list, _arg: new_order) -> return;
}
extern "C" {
    pub fn split_huge_page_to_list_to_order(_arg: page, _arg: NULL, _arg: new_order) -> return;
}
extern "C" {
    pub fn split_huge_page_to_list_to_order(_arg: page, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn folio_memcg_alloc_deferred(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn deferred_split_folio(folio: *mut folio, partially_mapped: bool);
}
//
// pmd_is_huge() - Is this PMD either a huge PMD entry or a software leaf entry?
// @pmd: The PMD to check.
//
// A huge PMD entry is a non-empty entry which is present and marked huge or a
// software leaf entry. This check be performed without the appropriate locks
// held, in which case the condition should be rechecked after they are
// acquired.
//
// Returns: true if this PMD is huge, false otherwise.
//
extern "C" {
    pub fn pmd_trans_huge(_arg: pmd) -> return;
}
//
// Non-present PMDs must be valid huge non-present entries. We
// cannot assert that here due to header dependency issues.
//

//
// pud_is_huge() - Is this PUD either a huge PUD entry or a software leaf entry?
// @pud: The PUD to check.
//
// This is similar to pmd_is_huge(), but it checks at the PUD level.
//
// Returns: true if this PUD is huge, false otherwise.
//
extern "C" {
    pub fn pud_trans_huge(_arg: pud) -> return;
}

// mmap_lock must be held on entry
extern "C" {
    pub fn __pmd_trans_huge_lock(_arg: pmd, _arg: vma) -> return;
}
extern "C" {
    pub fn __pud_trans_huge_lock(_arg: pud, _arg: vma) -> return;
}
//
// folio_test_pmd_mappable - Can we map this folio with a PMD?
// @folio: The folio to test
//
// Return: true - @folio can be mapped, false - @folio cannot be mapped.
//
extern "C" {
    pub fn do_huge_pmd_numa_page(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn do_huge_pmd_uffd_rwp(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn do_huge_pmd_device_private(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn READ_ONCE(1): huge_zero_pfn) == (pfn & ~(HPAGE_PMD_NR -) -> return;
}
extern "C" {
    pub fn pmd_present(is_huge_zero_pfn(pmd_pfn(pmd): pmd) &&) -> return;
}
extern "C" {
    pub fn mm_put_huge_zero_folio(mm: *mut mm_struct);
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARCH_HAS_PMD_SOFTLEAVES) -> return;
}

extern "C" {
    pub fn split_huge_page_to_list_to_order(_arg: &folio->page, _arg: NULL, _arg: new_order) -> return;
}
//
// largest_zero_folio - Get the largest zero size folio available
//
// This function shall be used when mm_get_huge_zero_folio() cannot be
// used as there is no appropriate mm lifetime to tie the huge zero folio
// from the caller.
//
// Deduce the size of the folio with folio_size instead of assuming the
// folio size.
//
// Return: pointer to PMD sized zero folio if CONFIG_PERSISTENT_HUGE_ZERO_FOLIO
// is enabled or a single page sized zero folio
//
extern "C" {
    pub fn page_folio(_arg: ZERO_PAGE(0)) -> return;
}
