//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/pgtable.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 ARM Ltd.
//

//
// VMALLOC range.
//
// VMALLOC_START: beginning of the kernel vmalloc space
// VMALLOC_END: extends to the available space below vmemmap
//

//
// These barriers are emitted under certain conditions after a pte entry
// was modified (see e.g. __set_pte_complete()). The dsb makes the store
// visible to the table walker. The isb ensures that any previous
// speculative "invalid translation" marker that is in the CPU's
// pipeline gets cleared, so that any access to that address after
// setting the pte to valid won't cause a spurious fault. If the thread
// gets preempted after storing to the pgtable but before emitting these
// barriers, __switch_to() emits a dsb which ensure the walker gets to
// see the store. There is no guarantee of an isb being issued though.
// This is safe because it will still get issued (albeit on a
// potentially different CPU) when the thread starts running again,
// before any access to the address.
//
// Avoid the atomic op if already set.

// Set stride and tlb_level in flush_*_tlb_range

//
// We use local TLB invalidation instruction when reusing page in
// write protection fault handler to avoid TLBI broadcast in the hot
// path.  This will cause spurious page faults if stale read-only TLB
// entries exist.
//

//
// The following only work if pte_present(). Undefined behaviour otherwise.
//
extern "C" {
    pub fn pte_valid(pte_present_invalid(pte: pte) ||) -> return;
}

//
// Execute-only user mappings do not have the PTE_USER bit set. All valid
// kernel mappings have the PTE_UXN bit set.
//

//
// Returns true if the pte is valid and has the contiguous bit set.
//

//
// Could the pte be present in the TLB? We must check mm_tlb_flush_pending
// so that we don't erroneously return false for pages that have been
// remapped as PROT_NONE but are yet to be flushed from the TLB.
// Note that we can't make any assumptions based on the state of the access
// flag, since __ptep_clear_flush_young() elides a DSB when invalidating the
// TLB.
//

extern "C" {
    pub fn por_elx_allows_write(_arg: por, _arg: pkey) -> return;
}
extern "C" {
    pub fn por_elx_allows_exec(_arg: por, _arg: pkey) -> return;
}
extern "C" {
    pub fn por_elx_allows_read(_arg: por, _arg: pkey) -> return;
}
//
// p??_access_permitted() is true for valid user mappings (PTE_USER
// bit set, subject to the write permission check). For execute-only
// mappings, like PROT_EXEC with EPAN (both PTE_USER and PTE_UXN bits
// not set) must return false. PROT_NONE mappings do not have the
// PTE_VALID bit set.
//

//
// If hardware-dirty (PTE_WRITE/DBM bit set and PTE_RDONLY
// clear), set the PTE_DIRTY bit.
//
extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(PTE_AF)) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(PTE_AF)) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(PTE_SPECIAL)) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(PTE_CONT)) -> return;
}
extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(PTE_CONT)) -> return;
}
extern "C" {
    pub fn __pmd(PMD_SECT_CONT: pmd_val(pmd) |) -> return;
}
extern "C" {
    pub fn __pmd(~PMD_SECT_CONT: pmd_val(pmd) &) -> return;
}

extern "C" {
    pub fn pte_wrprotect(_arg: set_pte_bit(pte, _arg: __pgprot(PTE_UFFD))) -> return;
}
extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(PTE_UFFD)) -> return;
}

//
// Only if the new pte is valid and kernel, otherwise TLB maintenance
// has the necessary barriers.
//
extern "C" {
    pub fn READ_ONCE(_arg: *mut ptep) -> return;
}
extern "C" {
    pub fn __sync_icache_dcache(pteval: pte_t);
}
extern "C" {
    pub fn pgattr_change_is_safe(old: pteval_t, new: pteval_t) -> bool;
}
//
// PTE bits configuration in the presence of hardware Dirty Bit Management
// (PTE_WRITE == PTE_DBM):
//
// Dirty  Writable | PTE_RDONLY  PTE_WRITE  PTE_DIRTY (sw)
// 0      0      |   1           0          0
// 0      1      |   1           1          0
// 1      0      |   1           0          1
// 1      1      |   0           1          x
//
// When hardware DBM is not present, the software PTE_DIRTY bit is updated via
// the page fault mechanism. Checking the dirty status of a pte becomes:
//
// PTE_DIRTY || (PTE_WRITE && !PTE_RDONLY)
//
// Check for potential race with hardware updates of the pte
// (__ptep_set_access_flags safely changes valid ptes without going
// through an invalid entry).
//
// If the PTE would provide user space access to the tags associated
// with it then ensure that the MTE tags are synchronised.  Although
// pte_access_permitted_no_overlay() returns false for exec only
// mappings, they don't expose tags (instruction fetches don't check
// tags).
//
// Select all bits except the pfn
//

extern "C" {
    pub fn __pgprot(_arg: pte_val(pfn_pte(pfn, pte_val(pte): __pgprot(0))) ^) -> return;
}

extern "C" {
    pub fn pfn_pte(nr: pte_pfn(pte) +, _arg: pte_pgprot(pte)) -> return;
}
//
// Hugetlb definitions.
//
pub const HUGE_MAX_HSTATE: c_int = 4;

extern "C" {
    pub fn __pte(_arg: pgd_val(pgd)) -> return;
}
extern "C" {
    pub fn __pte(_arg: p4d_val(p4d)) -> return;
}
extern "C" {
    pub fn __pte(_arg: pud_val(pud)) -> return;
}
extern "C" {
    pub fn __pud(_arg: pte_val(pte)) -> return;
}
extern "C" {
    pub fn __pmd(_arg: pud_val(pud)) -> return;
}
extern "C" {
    pub fn __pte(_arg: pmd_val(pmd)) -> return;
}
extern "C" {
    pub fn __pmd(_arg: pte_val(pte)) -> return;
}
extern "C" {
    pub fn __pgprot(PUD_TYPE_SECT: (pgprot_val(prot) & ~PUD_TYPE_MASK) |) -> return;
}
extern "C" {
    pub fn __pgprot(PMD_TYPE_SECT: (pgprot_val(prot) & ~PMD_TYPE_MASK) |) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(PTE_SWP_EXCLUSIVE)) -> return;
}
extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(PTE_SWP_EXCLUSIVE)) -> return;
}

extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(PTE_SWP_UFFD)) -> return;
}
extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(PTE_SWP_UFFD)) -> return;
}

//
// pte_present_invalid() tells us that the pte is invalid from HW
// perspective but present from SW perspective, so the fields are to be
// interpreted as per the HW layout. The second 2 checks are the unique
// encoding that we use for PROT_NONE. It is insufficient to only use
// the first check because we share the same encoding scheme with pmds
// which support pmd_mkinvalid(), so can be present-invalid without
// being PROT_NONE.
//
extern "C" {
    pub fn pte_present_invalid(!pte_user_exec(pte: pte) && !pte_user(pte) &&) -> return;
}
extern "C" {
    pub fn pte_protnone(_arg: pmd_pte(pmd)) -> return;
}

//
// It's possible that the pmd is present-invalid on entry
// and in that case it needs to remain present-invalid on
// exit. So ensure the VALID bit does not get modified.
//
extern "C" {
    pub fn __pmd(val: (pmd_val(pmd) & ~mask) |) -> return;
}

extern "C" {
    pub fn set_pmd_bit(_arg: pmd, _arg: __pgprot(PTE_SPECIAL)) -> return;
}

//
// It's possible that the pud is present-invalid on entry
// and in that case it needs to remain present-invalid on
// exit. So ensure the VALID bit does not get modified.
//
extern "C" {
    pub fn __pud(val: (pud_val(pud) & ~mask) |) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: pmd_val(pfn_pmd(pfn, pmd_val(pmd): __pgprot(0))) ^) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: pud_val(pfn_pud(pfn, pud_val(pud): __pgprot(0))) ^) -> return;
}

//
// Mark the prot value as uncacheable and unbufferable.
//

//
// DMA allocations for non-coherent devices use what the Arm architecture calls
// "Normal non-cacheable" memory, which permits speculation, unaligned accesses
// and merging of writes.  This is different from "Device-nGnR[nE]" memory which
// is intended for MMIO and thus forbids speculation, preserves access size,
// requires strict alignment and can also force write responses to come from the
// endpoint.
//

extern "C" {
    pub fn pmd_present(!pmd_table(pmd: pmd) &&) -> return;
}

//
// If pmd is present-invalid, pmd_table() won't detect it
// as a table, so force the valid bit for the comparison.
//
extern "C" {
    pub fn pmd_present(PTE_VALID): pmd) && !pmd_table(__pmd(pmd_val(pmd) |) -> return;
}

extern "C" {
    pub fn set_swapper_pgd(pgdp: *mut pgd_t, pgd: pgd_t);
}

extern "C" {
    pub fn __pmd_to_phys(_arg: pmd) -> return;
}
// Find an entry in the third-level page table.

// use ONLY for statically allocated translation tables

extern "C" {
    pub fn pud_present(!pud_table(pud: pud) &&) -> return;
}

extern "C" {
    pub fn pgtable_l4_enabled() -> bool;
}
extern "C" {
    pub fn __pud_to_phys(_arg: pud) -> return;
}
// Find an entry in the second-level page table.

// use ONLY for statically allocated translation tables

// Match pmd_offset folding in <asm/generic/pgtable-nopmd.h>

// Macro flag: #define pmd_clear_fixmap()

extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_VA52) -> return;
}

extern "C" {
    pub fn __p4d_to_phys(_arg: p4d) -> return;
}

// Ensure that 'p4dp' indexes a page table according to 'addr'
extern "C" {
    pub fn p4d_page_paddr(sizeof(pud_t: *mut *mut *mut READ_ONCE(p4dp)) + pud_index(addr)) -> return;
}
extern "C" {
    pub fn p4d_to_folded_pud(_arg: p4dp, _arg: addr) -> return;
}

extern "C" {
    pub fn pud_offset_lockless(_arg: p4dp, _arg: *mut READ_ONCE(p4dp), _arg: addr) -> return;
}

extern "C" {
    pub fn p4d_to_folded_pud(_arg: p4dp, _arg: addr) -> return;
}
extern "C" {
    pub fn pud_set_fixmap(_arg: pud_offset_phys(p4dp, _arg: addr)) -> return;
}
// use ONLY for statically allocated translation tables
extern "C" {
    pub fn p4d_to_folded_pud(_arg: p4dp, _arg: addr) -> return;
}

// Match pud_offset folding in <asm/generic/pgtable-nopud.h>

// Macro flag: #define pud_clear_fixmap()

extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_VA52) -> return;
}

extern "C" {
    pub fn __pgd_to_phys(_arg: pgd) -> return;
}

// Ensure that 'pgdp' indexes a page table according to 'addr'
extern "C" {
    pub fn pgd_page_paddr(sizeof(p4d_t: *mut *mut *mut READ_ONCE(pgdp)) + p4d_index(addr)) -> return;
}
extern "C" {
    pub fn pgd_to_folded_p4d(_arg: pgdp, _arg: addr) -> return;
}

extern "C" {
    pub fn p4d_offset_lockless(_arg: pgdp, _arg: *mut READ_ONCE(pgdp), _arg: addr) -> return;
}
extern "C" {
    pub fn pgd_to_folded_p4d(_arg: pgdp, _arg: addr) -> return;
}
extern "C" {
    pub fn p4d_set_fixmap(_arg: p4d_offset_phys(pgdp, _arg: addr)) -> return;
}
// use ONLY for statically allocated translation tables
extern "C" {
    pub fn pgd_to_folded_p4d(_arg: pgdp, _arg: addr) -> return;
}

// Match p4d_offset folding in <asm/generic/pgtable-nop4d.h>

// Macro flag: #define p4d_clear_fixmap()

//
// With runtime folding of the pud, pud_offset_lockless() passes
// the 'pgd_t *' we return here to p4d_to_folded_pud(), which
// will offset the pointer assuming that it points into
// a page-table page. However, the fast GUP path passes us a
// pgd_t allocated on the stack and so we must use the original
// pointer in 'pgdp' to construct the p4d pointer instead of
// using the generic p4d_offset_lockless() implementation.
//
// Note: reusing the original pointer means that we may
// dereference the same (live) page-table entry multiple times.
// This is safe because it is still only loaded once in the
// context of each level and the CPU guarantees same-address
// read-after-read ordering.
//
extern "C" {
    pub fn p4d_offset(_arg: pgdp, _arg: addr) -> return;
}

//
// Normal and Normal-Tagged are two different memory types and indices
// in MAIR_EL1. The mask below has to include PTE_ATTRINDX_MASK.
//
// preserve the hardware dirty information
//
// If we end up clearing hw dirtiness for a sw-dirty PTE, set hardware
// dirtiness again.
//
extern "C" {
    pub fn pte_pmd(_arg: pte_modify(pmd_pte(pmd), _arg: newprot)) -> return;
}

extern "C" {
    pub fn pte_valid(pte_user_exec(pte): pte) && (pte_user(pte) ||) -> return;
}
extern "C" {
    pub fn pmd_valid(pmd_user_exec(pmd): pmd) && !pmd_table(pmd) && (pmd_user(pmd) ||) -> return;
}
extern "C" {
    pub fn pud_valid(pud_user_exec(pud): pud) && !pud_table(pud) && (pud_user(pud) ||) -> return;
}

//
// Atomic pte/pmd modifications.
//
extern "C" {
    pub fn pte_young(_arg: pte) -> return;
}
//
// We can elide the trailing DSB here since the worst that can
// happen is that a CPU continues to use the young entry in its
// TLB and we mistakenly reclaim the associated page. The
// window for such an event is bounded by the next
// context-switch, which provides a DSB to complete the TLB
// invalidation.
//

// Operation applies to PMD table entry only if FEAT_HAFT is enabled
extern "C" {
    pub fn __ptep_test_and_clear_young(_arg: vma, _arg: address, )pmdp: *mut (pte_t) -> return;
}

extern "C" {
    pub fn __ptep_get_and_clear_anysz(_arg: mm, _arg: address, _arg: ptep, _arg: PAGE_SIZE) -> return;
}

extern "C" {
    pub fn pte_pmd(_arg: __ptep_get_and_clear_anysz(mm, _arg: address, )pmdp: *mut (pte_t, _arg: PMD_SIZE)) -> return;
}

//
// __ptep_set_wrprotect - mark read-only while transferring potential hardware
// dirty status (PTE_DBM && !PTE_RDONLY) to the software PTE_DIRTY bit.
//

extern "C" {
    pub fn __pmd(_arg: *mut xchg_relaxed(&pmd_val(pmdp), _arg: pmd_val(pmd))) -> return;
}

//
// Encode and decode a swap entry:
// bits 0-1:	present (must be zero)
// bits 2:		remember PG_anon_exclusive
// bit  3:		remember uffd state
// bits 6-10:	swap type
// bit  11:	PTE_PRESENT_INVALID (must be zero)
// bits 12-61:	swap offset
//
pub const __SWP_TYPE_SHIFT: c_int = 6;
pub const __SWP_TYPE_BITS: c_int = 5;

pub const __SWP_OFFSET_SHIFT: c_int = 12;
pub const __SWP_OFFSET_BITS: c_int = 50;

//
// Ensure that there are not more swap files than can be encoded in the kernel
// PTEs.
//

extern "C" {
    pub fn arch_prepare_to_swap(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn arch_swap_restore(entry: swp_entry_t, folio: *mut folio);
}

//
// On AArch64, the cache coherency is handled via the __set_ptes() function.
//
// We don't do anything here, so there's a very small chance of
// us retaking a user fault which we just fixed up. The alternative
// is doing a dsb(ishst), but that penalises the fastpath.
//

//
// On arm64 without hardware Access Flag, copying from user will fail because
// the pte is old and cannot be marked young. So we always end up with zeroed
// page after fork() + CoW for pfn mappings. We don't always have a
// hardware-managed access flag on arm64.
//

//
// Experimentally, it's cheap to set the access flag in hardware and we
// benefit from prefaulting mappings as 'old' to start with.
//

//
// Request exec memory is read into pagecache in at least 64K folios. This size
// can be contpte-mapped when 4K base pages are in use (16 pages into 1 iTLB
// entry), and HPA can coalesce it (4 pages into 1 TLB entry) when 16K base
// pages are in use.
//

//
// The contpte APIs are used to transparently manage the contiguous bit in ptes
// where it is possible and makes sense to do so. The PTE_CONT bit is considered
// a private implementation detail of the public ptep API (see below).
//
extern "C" {
    pub fn contpte_ptep_get(ptep: *mut pte_t, orig_pte: pte_t) -> pte_t;
}
extern "C" {
    pub fn contpte_ptep_get_lockless(orig_ptep: *mut pte_t) -> pte_t;
}
//
// Only bother trying if both the virtual and physical addresses are
// aligned and correspond to the last entry in a contig range. The core
// code mostly modifies ranges from low to high, so this is the likely
// the last modification in the contig range, so a good time to fold.
// We can't fold special mappings, because there is no associated folio.
//

//
// The below functions constitute the public API that arm64 presents to the
// core-mm to manipulate PTE entries within their page tables (or at least this
// is the subset of the API that arm64 needs to implement). These public
// versions will automatically and transparently apply the contiguous bit where
// it makes sense to do so. Therefore any users that are contig-aware (e.g.
// hugetlb, kernel mapper) should NOT use these APIs, but instead use the
// private versions, which are prefixed with double underscore. All of these
// APIs except for ptep_get_lockless() are expected to be called with the PTL
// held. Although the contiguous bit is considered private to the
// implementation, it is deliberately allowed to leak through the getters (e.g.
// ptep_get()), back to core code. This is required so that pte_leaf_size() can
// provide an accurate size for perf_get_pgtable_size(). But this leakage means
// its possible a pte will be passed to a setter with the contiguous bit set, so
// we explicitly clear the contiguous bit in those cases to prevent accidentally
// setting it in the pgtable.
//

extern "C" {
    pub fn contpte_ptep_get(_arg: ptep, _arg: pte) -> return;
}

extern "C" {
    pub fn contpte_ptep_get_lockless(_arg: ptep) -> return;
}
//
// We don't have the mm or vaddr so cannot unfold contig entries (since
// it requires tlb maintenance). set_pte() is not used in core code, so
// this should never even be called. Regardless do our best to service
// any call and emit a warning if there is any attempt to set a pte on
// top of an existing contig range.
//

extern "C" {
    pub fn __ptep_get_and_clear(_arg: mm, _arg: addr, _arg: ptep) -> return;
}
//
// The store must be complete by the time this returns, but the caller
// may be in lazy MMU mode, where __set_pte_complete() would defer the
// barriers. Issue them directly.
//

//
// arm64 mandates break-before-make: a cleared kernel PTE must have its TLB
// invalidated before a different page is installed in its place. The broadcast
// TLBI is an instruction, not an IPI, so this is safe with interrupts disabled.
//

extern "C" {
    pub fn __ptep_test_and_clear_young(_arg: vma, _arg: addr, _arg: ptep) -> return;
}
extern "C" {
    pub fn contpte_test_and_clear_young_ptes(_arg: vma, _arg: addr, _arg: ptep, _arg: nr) -> return;
}
extern "C" {
    pub fn test_and_clear_young_ptes(_arg: vma, _arg: addr, _arg: ptep, _arg: 1) -> return;
}
extern "C" {
    pub fn __ptep_clear_flush_young(_arg: vma, _arg: addr, _arg: ptep) -> return;
}
extern "C" {
    pub fn contpte_clear_flush_young_ptes(_arg: vma, _arg: addr, _arg: ptep, _arg: 1) -> return;
}

extern "C" {
    pub fn __ptep_clear_flush_young(_arg: vma, _arg: addr, _arg: ptep) -> return;
}
extern "C" {
    pub fn contpte_clear_flush_young_ptes(_arg: vma, _arg: addr, _arg: ptep, _arg: nr) -> return;
}

//
// Optimization: wrprotect_ptes() can only be called for present
// ptes so we only need to check contig bit as condition for
// unfold, and we can remove the contig bit from the pte we read
// to avoid re-reading. This speeds up fork() which is sensitive
// for order-0 folios. Equivalent to contpte_try_unfold().
//
extern "C" {
    pub fn __ptep_set_access_flags(_arg: vma, _arg: addr, _arg: ptep, _arg: entry, _arg: dirty) -> return;
}
extern "C" {
    pub fn contpte_ptep_set_access_flags(_arg: vma, _arg: addr, _arg: ptep, _arg: entry, _arg: dirty) -> return;
}

