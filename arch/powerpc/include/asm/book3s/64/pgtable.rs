//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/pgtable.h
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
// Common bits between hash and Radix page table
//
pub const _PAGE_EXEC: c_uint = 0x00001 /* execute permission */;
pub const _PAGE_WRITE: c_uint = 0x00002 /* write access allowed */;
pub const _PAGE_READ: c_uint = 0x00004	/* read access allowed */;
pub const _PAGE_PRIVILEGED: c_uint = 0x00008 /* kernel access only */;
pub const _PAGE_SAO: c_uint = 0x00010 /* Strong access order */;
pub const _PAGE_NON_IDEMPOTENT: c_uint = 0x00020 /* non idempotent memory */;
pub const _PAGE_TOLERANT: c_uint = 0x00030 /* tolerant memory, cache inhibited */;
pub const _PAGE_DIRTY: c_uint = 0x00080 /* C: page changed */;
pub const _PAGE_ACCESSED: c_uint = 0x00100 /* R: page referenced */;
//
// Software bits
//
pub const _RPAGE_SW0: c_uint = 0x2000000000000000UL;
pub const _RPAGE_SW1: c_uint = 0x00800;
pub const _RPAGE_SW2: c_uint = 0x00400;
pub const _RPAGE_SW3: c_uint = 0x00200;
pub const _RPAGE_RSV1: c_uint = 0x00040UL;
pub const _RPAGE_PKEY_BIT4: c_uint = 0x1000000000000000UL;
pub const _RPAGE_PKEY_BIT3: c_uint = 0x0800000000000000UL;
pub const _RPAGE_PKEY_BIT2: c_uint = 0x0400000000000000UL;
pub const _RPAGE_PKEY_BIT1: c_uint = 0x0200000000000000UL;
pub const _RPAGE_PKEY_BIT0: c_uint = 0x0100000000000000UL;
pub const _PAGE_PTE: c_uint = 0x4000000000000000UL	/* distinguishes PTEs from pointers */;
pub const _PAGE_PRESENT: c_uint = 0x8000000000000000UL	/* pte contains a translation */;
//
// We need to mark a pmd pte invalid while splitting. We can do that by clearing
// the _PAGE_PRESENT bit. But then that will be taken as a swap pte. In order to
// differentiate between two use a SW field when invalidating.
//
// We do that temporary invalidate for regular pte entry in ptep_set_access_flags
//
// This is used only when _PAGE_PRESENT is cleared.
//

//
// Top and bottom bits of RPN which can be used by hash
// translation mode, because we expect them to be zero
// otherwise.
//
pub const _RPAGE_RPN0: c_uint = 0x01000;
pub const _RPAGE_RPN1: c_uint = 0x02000;
pub const _RPAGE_RPN43: c_uint = 0x0080000000000000UL;
pub const _RPAGE_RPN42: c_uint = 0x0040000000000000UL;
pub const _RPAGE_RPN41: c_uint = 0x0020000000000000UL;
// Max physical address bit as per radix table
pub const _RPAGE_PA_MAX: c_int = 56;
//
// Max physical address bit we will use for now.
//
// This is mostly a hardware limitation and for now Power9 has
// a 51 bit limit.
//
// This is different from the number of physical bit required to address
// the last byte of memory. That is defined by MAX_PHYSMEM_BITS.
// MAX_PHYSMEM_BITS is a linux limitation imposed by the maximum
// number of sections we can support (SECTIONS_SHIFT).
//
// This is different from Radix page table limitation above and
// should always be less than that. The limit is done such that
// we can overload the bits between _RPAGE_PA_MAX and _PAGE_PA_MAX
// for hash linux page table specific bits.
//
// In order to be compatible with future hardware generations we keep
// some offsets and limit this for now to 53
//
pub const _PAGE_PA_MAX: c_int = 53;

//
// Drivers request for cache inhibited pte mapping using _PAGE_NO_CACHE
// Instead of fixing all of them, add an alternate define which
// maps CI pte mapping.
//

//
// We support _RPAGE_PA_MAX bit real address in pte. On the linux side
// we are limited by _PAGE_PA_MAX. Clear everything above _PAGE_PA_MAX
// and every thing below PAGE_SHIFT;
//

//
// set of bits not changed in pmd_modify. Even though we have hash specific bits
// in here, on radix we expect them to be zero.
//

//
// user access blocked by key
//

//
// _PAGE_CHG_MASK masks of bits that are to be preserved across
// pgprot changes
//

//
// We define 2 sets of base prot bits, one for basic pages (ie,
// cacheable kernel and user pages) and one for non cacheable
// pages. We always set _PAGE_COHERENT when SMP is enabled or
// the processor might need it for DMA coherency.
//

// Permission masks used for kernel mappings

//
// page table defines
//

// pmd table use page table fragments
pub const PMD_CACHE_INDEX: c_int = 0;

//
// Because of use of pte fragments and THP, size of page table
// are not always derived out of index size above.
//

// PMD_SHIFT determines what a second-level page table entry can map

// PUD_SHIFT determines what a third-level page table entry can map

// PGDIR_SHIFT determines what a fourth-level page table entry can map

// Bits to mask out from a PMD to get to the PTE page
pub const PMD_MASKED_BITS: c_uint = 0xc0000000000000ffUL;
// Bits to mask out from a PUD to get to the PMD page
pub const PUD_MASKED_BITS: c_uint = 0xc0000000000000ffUL;
// Bits to mask out from a PGD to get to the PUD page
pub const P4D_MASKED_BITS: c_uint = 0xc0000000000000ffUL;
//
// Used as an indicator for rcu callback functions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pgtable_index {
    PTE_INDEX = 0,
    PMD_INDEX,
    PUD_INDEX,
    PGD_INDEX,
//
// Below are used with 4k page size and hugetlb
//
    HTLB_16M_INDEX,
    HTLB_16G_INDEX,
}

// hash 4k can't share hugetlb and also doesn't support THP

//
// IO space itself carved into the PIO region (ISA and PHB IO space) and
// the ioremap space
//
// ISA_IO_BASE = KERN_IO_START, 64K reserved area
// PHB_IO_BASE = ISA_IO_BASE + 64K to ISA_IO_BASE + 2G, PHB IO spaces
// IOREMAP_BASE = ISA_IO_BASE + 2G to VMALLOC_START + PGTABLE_RANGE
//
pub const FULL_IO_SIZE: c_uint = 0x80000000ul;

extern "C" {
    pub fn radix__pte_update(_arg: mm, _arg: addr, _arg: ptep, _arg: clr, _arg: set, _arg: huge) -> return;
}
extern "C" {
    pub fn hash__pte_update(_arg: mm, _arg: addr, _arg: ptep, _arg: clr, _arg: set, _arg: huge) -> return;
}
//
// For hash even if we have _PAGE_ACCESSED = 0, we do a pte_update.
// We currently remove entries from the hashtable regardless of whether
// the entry was young or dirty.
//
// We should be more intelligent about this but for the moment we override
// these functions and force a tlb flush unconditionally
// For radix: H_PAGE_HASHPTE should be zero. Hence we can use the same
// function for both hash and radix.
//

//
// On Book3S CPUs, clearing the accessed bit without a TLB flush
// doesn't cause data corruption. [ It could cause incorrect
// page aging and the (mistaken) reclaim of hot pages, but the
// chance of that should be relatively low. ]
//
// So as a performance optimization don't flush the TLB when
// clearing the accessed bit, it will eventually be flushed by
// a context switch or a VM operation anyway. [ In the rare
// event of it not getting flushed for a long time the delay
// shouldn't really matter because there's no real memory
// pressure for swapout to react to. ]
//
// Note: this optimisation also exists in pte_needs_flush() and
// huge_pmd_needs_flush().
//

//
// We know that this is a full mm pte clear and
// hence can be sure there is no parallel set_pte.
//
extern "C" {
    pub fn ptep_get_and_clear(_arg: mm, _arg: addr, _arg: ptep) -> return;
}

extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_SOFT_DIRTY): pte_raw(pte) |) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_SOFT_DIRTY): pte_raw(pte) &) -> return;
}

//
// A pte is considerent present if _PAGE_PRESENT is set.
// We also need to consider the pte present which is marked
// invalid during ptep_set_access_flags. Hence we look for _PAGE_INVALID
// if we find _PAGE_PRESENT cleared.
//

extern "C" {
    pub fn arch_pte_access_permitted(pte: u64, write: bool, execute: bool) -> bool;
}

//
// _PAGE_READ is needed for any access and will be cleared for
// PROT_NONE. Execute-only mapping via PROT_EXEC also returns false.
//
extern "C" {
    pub fn arch_pte_access_permitted(_arg: pte_val(pte), _arg: write, _arg: 0) -> return;
}
extern "C" {
    pub fn pte_present(pte_user(pte: pte) &&) -> return;
}
//
// Conversion functions: convert a page and protection to a page entry,
// and a page entry and page directory to the page they refer to.
//
// Even if PTEs can be unsigned long long, a PFN is always an unsigned
// long for now.
//
extern "C" {
    pub fn __pte(_PAGE_PTE: ((pte_basic_t)pfn << PAGE_SHIFT) | pgprot_val(pgprot) |) -> return;
}
// Generic modifiers for PTE bits
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_WRITE): pte_raw(pte) &) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_EXEC): pte_raw(pte) &) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_DIRTY): pte_raw(pte) &) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_ACCESSED): pte_raw(pte) &) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_EXEC): pte_raw(pte) |) -> return;
}
//
// write implies read, hence set both
//
extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_RW): pte_raw(pte) |) -> return;
}
extern "C" {
    pub fn __pte_raw(_PAGE_SOFT_DIRTY): pte_raw(pte) | cpu_to_be64(_PAGE_DIRTY |) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_ACCESSED): pte_raw(pte) |) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_SPECIAL): pte_raw(pte) |) -> return;
}
// FIXME!! check whether this need to be a conditional
// Encode and de-code a swap entry

// \
// Don't have overlapping bits with _PAGE_HPTEFLAGS	\
// We filter HPTEFLAGS on set_pte.			\
// \
pub const SWP_TYPE_BITS: c_int = 5;

//
// swp_entry_t must be independent of pte bits. We build a swp_entry_t from
// swap type and offset we get from swap and convert that to pte to find a
// matching pte in linux page table.
// Clear bits not found in swap entries here.
//

extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_SWP_SOFT_DIRTY): pte_raw(pte) |) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_SWP_SOFT_DIRTY): pte_raw(pte) &) -> return;
}

extern "C" {
    pub fn __pte_raw(cpu_to_be64(_PAGE_SWP_EXCLUSIVE): pte_raw(pte) |) -> return;
}
extern "C" {
    pub fn __pte_raw(cpu_to_be64(~_PAGE_SWP_EXCLUSIVE): pte_raw(pte) &) -> return;
}
//
// This check for _PAGE_RWX and _PAGE_PRESENT bits
//
// This check for access to privilege space
//
// Generic functions with hash/radix callbacks
//
extern "C" {
    pub fn hash__ptep_set_access_flags(_arg: ptep, _arg: entry) -> return;
}
extern "C" {
    pub fn radix__pte_same(_arg: pte_a, _arg: pte_b) -> return;
}
extern "C" {
    pub fn hash__pte_same(_arg: pte_a, _arg: pte_b) -> return;
}
extern "C" {
    pub fn radix__pte_none(_arg: pte) -> return;
}
extern "C" {
    pub fn hash__pte_none(_arg: pte) -> return;
}
//
// Keep the _PAGE_PTE added till we are sure we handle _PAGE_PTE
// in all the callers.
//
extern "C" {
    pub fn radix__set_pte_at(_arg: mm, _arg: addr, _arg: ptep, _arg: pte, _arg: percpu) -> return;
}
extern "C" {
    pub fn hash__set_pte_at(_arg: mm, _arg: addr, _arg: ptep, _arg: pte, _arg: percpu) -> return;
}

extern "C" {
    pub fn __pgprot(~_PAGE_CACHE_CTL): (pgprot_val(prot) &) -> return;
}

extern "C" {
    pub fn pgprot_noncached_wc(_arg: prot) -> return;
}
//
// check a pte mapping have cache inhibited property
//
// Don't use this if we can possibly have a hash page table
// entry mapping this.
//
// pmdp = __pmd(0);
//
// A pmd is considerent present if _PAGE_PRESENT is set.
// We also need to consider the pmd present which is marked
// invalid during a split. Hence we look for _PAGE_INVALID
// if we find _PAGE_PRESENT cleared.
//
// If the pmd is undergoing a split, the _PAGE_PRESENT bit is clear
// and _PAGE_INVALID is set (see pmd_present, pmdp_invalidate).
//
// This condition may also occur when flushing a pmd while flushing
// it (see ptep_modify_prot_start), so callers must ensure this
// case is fine as well.
//
extern "C" {
    pub fn radix__pmd_bad(_arg: pmd) -> return;
}
extern "C" {
    pub fn hash__pmd_bad(_arg: pmd) -> return;
}
//
// Don't use this if we can possibly have a hash page table
// entry mapping this.
//
// pudp = __pud(0);
extern "C" {
    pub fn __pte_raw(_arg: pud_raw(pud)) -> return;
}
extern "C" {
    pub fn __pud_raw(_arg: pte_raw(pte)) -> return;
}

extern "C" {
    pub fn radix__pud_bad(_arg: pud) -> return;
}
extern "C" {
    pub fn hash__pud_bad(_arg: pud) -> return;
}

extern "C" {
    pub fn pte_access_permitted(_arg: pud_pte(pud), _arg: write) -> return;
}

extern "C" {
    pub fn pud_leaf(pte_user_accessible_page(mm: pud) &&, _arg: addr, _arg: pud_pte(pud)) -> return;
}

extern "C" {
    pub fn pgd_raw(_arg: x.pgd) -> return;
}

// p4dp = __p4d(0);
extern "C" {
    pub fn __pte_raw(_arg: p4d_raw(p4d)) -> return;
}
extern "C" {
    pub fn __p4d_raw(_arg: pte_raw(pte)) -> return;
}
extern "C" {
    pub fn radix__p4d_bad(_arg: p4d) -> return;
}
extern "C" {
    pub fn hash__p4d_bad(_arg: p4d) -> return;
}

extern "C" {
    pub fn pte_access_permitted(_arg: p4d_pte(p4d), _arg: write) -> return;
}
// Pointers in the page table tree are physical addresses

extern "C" {
    pub fn radix__map_kernel_page(_arg: ea, _arg: pa, _arg: prot, _arg: PAGE_SIZE) -> return;
}
extern "C" {
    pub fn hash__map_kernel_page(_arg: ea, _arg: pa, _arg: prot) -> return;
}
extern "C" {
    pub fn unmap_kernel_page(va: c_ulong);
}
extern "C" {
    pub fn radix__vmemmap_create_mapping(_arg: start, _arg: page_size, _arg: phys) -> return;
}
extern "C" {
    pub fn hash__vmemmap_create_mapping(_arg: start, _arg: page_size, _arg: phys) -> return;
}

extern "C" {
    pub fn radix__vmemmap_remove_mapping(_arg: start, _arg: page_size) -> return;
}
extern "C" {
    pub fn hash__vmemmap_remove_mapping(_arg: start, _arg: page_size) -> return;
}

extern "C" {
    pub fn __pte_raw(_arg: pmd_raw(pmd)) -> return;
}
extern "C" {
    pub fn __pmd_raw(_arg: pte_raw(pte)) -> return;
}

extern "C" {
    pub fn pte_protnone(_arg: pmd_pte(pmd)) -> return;
}

//
// pmdp_invalidate sets this combination (which is not caught by
// !pte_present() check in pte_access_permitted), to prevent
// lock-free lookups, as part of the serialize_against_pte_lookup()
// synchronisation.
//
// This also catches the case where the PTE's hardware PRESENT bit is
// cleared while TLB is flushed, which is suboptimal but should not
// be frequent.
//
extern "C" {
    pub fn pte_access_permitted(_arg: pmd_pte(pmd), _arg: write) -> return;
}

extern "C" {
    pub fn pmd_leaf(pte_user_accessible_page(mm: pmd) &&, _arg: addr, _arg: pmd_pte(pmd)) -> return;
}

extern "C" {
    pub fn pfn_pmd(pfn: c_ulong, pgprot: pgprot_t) -> pmd_t;
}
extern "C" {
    pub fn pfn_pud(pfn: c_ulong, pgprot: pgprot_t) -> pud_t;
}
extern "C" {
    pub fn pmd_modify(pmd: pmd_t, newprot: pgprot_t) -> pmd_t;
}
extern "C" {
    pub fn pud_modify(pud: pud_t, newprot: pgprot_t) -> pud_t;
}
extern "C" {
    pub fn hash__has_transparent_hugepage() -> c_int;
}
extern "C" {
    pub fn radix__has_transparent_hugepage() -> return;
}
extern "C" {
    pub fn hash__has_transparent_hugepage() -> return;
}

extern "C" {
    pub fn radix__has_transparent_pud_hugepage() -> return;
}

extern "C" {
    pub fn radix__pmd_hugepage_update(_arg: mm, _arg: addr, _arg: pmdp, _arg: clr, _arg: set) -> return;
}
extern "C" {
    pub fn hash__pmd_hugepage_update(_arg: mm, _arg: addr, _arg: pmdp, _arg: clr, _arg: set) -> return;
}
extern "C" {
    pub fn radix__pud_hugepage_update(_arg: mm, _arg: addr, _arg: pudp, _arg: clr, _arg: set) -> return;
}
extern "C" {
    pub fn pud_val(_arg: *mut pudp) -> return;
}
//
// For radix we should always find H_PAGE_HASHPTE zero. Hence
// the below will work for radix too
//
// Only returns true for a THP. False for pmd migration entry.
// We also need to return true when we come across a pte that
// in between a thp split. While splitting THP, we mark the pmd
// invalid (pmdp_invalidate()) before we set it with pte page
// address. A pmd_trans_huge() check against a pmd entry during that time
// should return true.
// We should not call this on a hugetlb entry. We should check for HugeTLB
// entry using vma->vm_flags
// The page table walk rule is explained in Documentation/mm/transhuge.rst
//
extern "C" {
    pub fn radix__pmd_trans_huge(_arg: pmd) -> return;
}
extern "C" {
    pub fn hash__pmd_trans_huge(_arg: pmd) -> return;
}
extern "C" {
    pub fn radix__pud_trans_huge(_arg: pud) -> return;
}
extern "C" {
    pub fn radix__pmd_same(_arg: pmd_a, _arg: pmd_b) -> return;
}
extern "C" {
    pub fn hash__pmd_same(_arg: pmd_a, _arg: pmd_b) -> return;
}

extern "C" {
    pub fn radix__pud_same(_arg: pud_a, _arg: pud_b) -> return;
}
extern "C" {
    pub fn hash__pud_same(_arg: pud_a, _arg: pud_b) -> return;
}
extern "C" {
    pub fn radix__pmd_mkhuge(_arg: pmd) -> return;
}
extern "C" {
    pub fn hash__pmd_mkhuge(_arg: pmd) -> return;
}
extern "C" {
    pub fn radix__pud_mkhuge(_arg: pud) -> return;
}
//
// pfn_pmd return a pmd_t that can be used as pmd pte entry.
//

extern "C" {
    pub fn pte_special(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkspecial(pmd_pte(pmd))) -> return;
}

extern "C" {
    pub fn pte_special(_arg: pud_pte(pud)) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkspecial(pud_pte(pud))) -> return;
}

//
// Non-present PMDs can be migration entries or device-private THP
// entries. This can happen at 2 places:
// - When the address space is being unmapped zap_huge_pmd(), and we
// encounter non-present pmds.
// - migrate_vma_collect_huge_pmd() could calls this during migration
// of device-private pmd entries.
//
extern "C" {
    pub fn radix__pmdp_collapse_flush(_arg: vma, _arg: address, _arg: pmdp) -> return;
}
extern "C" {
    pub fn hash__pmdp_collapse_flush(_arg: vma, _arg: address, _arg: pmdp) -> return;
}

extern "C" {
    pub fn radix__pgtable_trans_huge_deposit(_arg: mm, _arg: pmdp, _arg: pgtable) -> return;
}
extern "C" {
    pub fn hash__pgtable_trans_huge_deposit(_arg: mm, _arg: pmdp, _arg: pgtable) -> return;
}
extern "C" {
    pub fn radix__pgtable_trans_huge_withdraw(_arg: mm, _arg: pmdp) -> return;
}
extern "C" {
    pub fn hash__pgtable_trans_huge_withdraw(_arg: mm, _arg: pmdp) -> return;
}

//
// Hash translation mode use the deposited table to store hash pte
// slot information.
//

extern "C" {
    pub fn ptep_modify_prot_start(: *mut vm_area_struct, long: unsigned, : *mut pte_t) -> pte_t;
}
//
// Returns true for a R -> RW upgrade of pte
//

