//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable.h
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
// Macro to mark a page protection value as UC-
//

extern "C" {
    pub fn __early_make_pgtable(address: c_ulong, pmd: pmdval_t) -> bool __init;
}
extern "C" {
    pub fn ptdump_walk_pgd_level(m: *mut seq_file, mm: *mut mm_struct);
}
extern "C" {
    pub fn ptdump_walk_pgd_level_checkwx() -> bool;
}

extern "C" {
    pub fn ptdump_walk_user_pgd_level_checkwx();
}
//
// Macros to add or remove encryption attribute
//

extern "C" {
    pub fn native_make_pmd(set: v |) -> return;
}
extern "C" {
    pub fn native_make_pmd(~clear: v &) -> return;
}
extern "C" {
    pub fn native_make_pud(set: v |) -> return;
}
extern "C" {
    pub fn native_make_pud(~clear: v &) -> return;
}
//
// The following only work if pte_present() is true.
// Undefined behaviour if not..
//
extern "C" {
    pub fn cc_mkdec(pte_val(pte: pte_val(pte)) ==) -> return;
}

//
// Shadow stack pages are logically writable, but do not have
// _PAGE_RW.  Check for them separately from _PAGE_RW itself.
//

//
// Shadow stack pages are logically writable, but do not have
// _PAGE_RW.  Check for them separately from _PAGE_RW itself.
//

// Entries that were set to PROT_NONE are inverted
extern "C" {
    pub fn protnone_mask(val: u64) -> u64;
}

extern "C" {
    pub fn boot_cpu_has(_arg: X86_FEATURE_PSE) -> return;
}

extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_SPECIAL) -> return;
}

extern "C" {
    pub fn pud_set_flags(_arg: pud, _arg: _PAGE_SPECIAL) -> return;
}

extern "C" {
    pub fn native_make_pte(set: v |) -> return;
}
extern "C" {
    pub fn native_make_pte(~clear: v &) -> return;
}
//
// Write protection operations can result in Dirty=1,Write=0 PTEs. But in the
// case of X86_FEATURE_USER_SHSTK, these PTEs denote shadow stack memory. So
// when creating dirty, write-protected memory, a software bit is used:
// _PAGE_BIT_SAVED_DIRTY. The following functions take a PTE and transition the
// Dirty bit to SavedDirty, and vice-vesra.
//
// This shifting is only done if needed. In the case of shifting
// Dirty->SavedDirty, the condition is if the PTE is Write=0. In the case of
// shifting SavedDirty->Dirty, the condition is Write=1.
//
extern "C" {
    pub fn native_make_pte(_arg: v) -> return;
}
extern "C" {
    pub fn native_make_pte(_arg: v) -> return;
}
//
// Blindly clearing _PAGE_RW might accidentally create
// a shadow stack PTE (Write=0,Dirty=1). Move the hardware
// dirty value to the software bit, if present.
//
extern "C" {
    pub fn pte_mksaveddirty(_arg: pte) -> return;
}

extern "C" {
    pub fn pte_wrprotect(_arg: pte_set_flags(pte, _arg: _PAGE_UFFD)) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_UFFD) -> return;
}

extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_DIRTY_BITS) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_NX) -> return;
}
extern "C" {
    pub fn pte_mksaveddirty(_arg: pte) -> return;
}
extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_DIRTY) -> return;
}
extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_RW) -> return;
}
extern "C" {
    pub fn pte_mkwrite(pte: pte_t, vma: *mut vm_area_struct) -> pte_t;
}

extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_PSE) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_PSE) -> return;
}
extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_GLOBAL) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_GLOBAL) -> return;
}
extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_SPECIAL) -> return;
}
// See comments above mksaveddirty_shift()
extern "C" {
    pub fn native_make_pmd(_arg: v) -> return;
}
// See comments above mksaveddirty_shift()
extern "C" {
    pub fn native_make_pmd(_arg: v) -> return;
}
//
// Blindly clearing _PAGE_RW might accidentally create
// a shadow stack PMD (RW=0, Dirty=1). Move the hardware
// dirty value to the software bit.
//
extern "C" {
    pub fn pmd_mksaveddirty(_arg: pmd) -> return;
}

extern "C" {
    pub fn pmd_wrprotect(_arg: pmd_set_flags(pmd, _arg: _PAGE_UFFD)) -> return;
}
extern "C" {
    pub fn pmd_clear_flags(_arg: pmd, _arg: _PAGE_UFFD) -> return;
}

extern "C" {
    pub fn pmd_clear_flags(_arg: pmd, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pmd_clear_flags(_arg: pmd, _arg: _PAGE_DIRTY_BITS) -> return;
}
extern "C" {
    pub fn pmd_mksaveddirty(_arg: pmd) -> return;
}
extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_DIRTY) -> return;
}
extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_PSE) -> return;
}
extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_RW) -> return;
}
extern "C" {
    pub fn pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t;
}

// See comments above mksaveddirty_shift()
extern "C" {
    pub fn native_make_pud(_arg: v) -> return;
}
// See comments above mksaveddirty_shift()
extern "C" {
    pub fn native_make_pud(_arg: v) -> return;
}
extern "C" {
    pub fn pud_clear_flags(_arg: pud, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pud_clear_flags(_arg: pud, _arg: _PAGE_DIRTY_BITS) -> return;
}
//
// Blindly clearing _PAGE_RW might accidentally create
// a shadow stack PUD (RW=0, Dirty=1). Move the hardware
// dirty value to the software bit.
//
extern "C" {
    pub fn pud_mksaveddirty(_arg: pud) -> return;
}
extern "C" {
    pub fn pud_mksaveddirty(_arg: pud) -> return;
}
extern "C" {
    pub fn pud_set_flags(_arg: pud, _arg: _PAGE_PSE) -> return;
}
extern "C" {
    pub fn pud_set_flags(_arg: pud, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pud_clear_saveddirty(_arg: pud) -> return;
}

extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pud_set_flags(_arg: pud, _arg: _PAGE_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pmd_clear_flags(_arg: pmd, _arg: _PAGE_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pud_clear_flags(_arg: pud, _arg: _PAGE_SOFT_DIRTY) -> return;
}

//
// Mask out unsupported bits in a present pgprot.  Non-present pgprots
// can use those bits for other purposes, so leave them be.
//
// mmdebug.h can not be included here because of dependencies

// This bit combination is used to mark shadow stacks
extern "C" {
    pub fn __pte(check_pgprot(pgprot): pfn |) -> return;
}
extern "C" {
    pub fn __pmd(check_pgprot(pgprot): pfn |) -> return;
}
extern "C" {
    pub fn __pud(check_pgprot(pgprot): pfn |) -> return;
}
extern "C" {
    pub fn flip_protnone_guard(oldval: u64, val: u64, mask: u64) -> u64;
}
//
// Chop off the NX bit (if present), and add the NX portion of
// the newprot (if present):
//
// To avoid creating Write=0,Dirty=1 PTEs, pte_modify() needs to avoid:
// 1. Marking Write=0 PTEs Dirty=1
// 2. Marking Dirty=1 PTEs Write=0
//
// The first case cannot happen because the _PAGE_CHG_MASK will filter
// out any Dirty bit passed in newprot. Handle the second case by
// going through the mksaveddirty exercise. Only do this if the old
// value was Write=1 to avoid doing this on Shadow Stack PTEs.
//
// Avoid creating shadow stack PMD by accident.  See comment in
// pte_modify().
//
// Avoid creating shadow stack PUD by accident.  See comment in
// pte_modify().
//
// mprotect needs to preserve PAT and encryption bits when updating
// vm_page_prot
//

extern "C" {
    pub fn __pgprot(addbits: preservebits |) -> return;
}

//
// PAT type is always WB for untracked ranges, so no need to check.
//
// Certain new memtypes are not allowed with certain
// requested memtype:
// - request is uncached, return cannot be write-back
// - request is write-combine, return cannot be write-back
// - request is write-through, return cannot be write-back
// - request is write-through, return cannot be write-combine
//

extern "C" {
    pub fn __pti_set_user_pgtbl(pgdp: *mut pgd_t, pgd: pgd_t) -> pgd_t;
}
//
// Take a PGD location (pgdp) and a pgd value that needs to be set there.
// Populates the user and returns the resulting PGD that must be set in
// the kernel copy of the page tables.
//
extern "C" {
    pub fn __pti_set_user_pgtbl(_arg: pgdp, _arg: pgd) -> return;
}

extern "C" {
    pub fn __pte(PFN_PTE_SHIFT): pte_val(pte) - (nr <<) -> return;
}
extern "C" {
    pub fn __pte(PFN_PTE_SHIFT): pte_val(pte) + (nr <<) -> return;
}

extern "C" {
    pub fn pte_flags(_PAGE_PROTNONE: a) & (_PAGE_PRESENT |) -> return;
}

//
// Checking for _PAGE_PSE is needed too because
// split_huge_page will temporarily clear the present bit (but
// the _PAGE_PSE flag will remain set at all times while the
// _PAGE_PRESENT bit is clear).
//
extern "C" {
    pub fn pmd_flags(_PAGE_PSE: pmd) & (_PAGE_PRESENT | _PAGE_PROTNONE |) -> return;
}

// Only check low word on 32-bit platforms, since it might be
//
// Currently stuck as a macro due to indirect forward reference to
// linux/mmzone.h's __section_mem_map_addr() definition:
//

//
// Currently stuck as a macro due to indirect forward reference to
// linux/mmzone.h's __section_mem_map_addr() definition:
//

//
// Currently stuck as a macro due to indirect forward reference to
// linux/mmzone.h's __section_mem_map_addr() definition:
//

//
// Currently stuck as a macro due to indirect forward reference to
// linux/mmzone.h's __section_mem_map_addr() definition:
//

// to find an entry in a page-table-directory.
//
// There is no need to do a workaround for the KNL stray
// A/D bit erratum here.  PGDs only point to page tables
// except on 32-bit non-PAE which is not supported on
// KNL.
//

extern "C" {
    pub fn init_mem_mapping();
}
extern "C" {
    pub fn early_alloc_pgt_buf();
}
extern "C" {
    pub fn poking_init() -> void __init;
}

// local pte updates need not use xchg for locking
// Pure native function needs no input for mm, addr
//
// We only update the dirty/accessed state if we set
// the dirty bit by hand in the kernel, since the hardware
// will do the accessed bit for us, and we don't want to
// race with other CPU's that might be updating the dirty
// bit at the same time.
//
// Full address destruction in progress; paravirt does not
// care about updates and native needs no locking
//
// Avoid accidentally creating shadow stack PTEs
// (Write=0,Dirty=1).  Use cmpxchg() to prevent races with
// the hardware setting Dirty=1.
//
// Note: strictly-zero compare is narrower than pte_none(), but the gap is
// harmless: _PAGE_DIRTY and _PAGE_ACCESSED aren't set on untouched kernel PTEs.
//
extern "C" {
    pub fn try_cmpxchg()&ptep->pte: *mut (long, )&old_pte: *mut (long, )&new_pte: *mut *mut (long) -> return;
}

//
// Avoid accidentally creating shadow stack PTEs
// (Write=0,Dirty=1).  Use cmpxchg() to prevent races with
// the hardware setting Dirty=1.
//

extern "C" {
    pub fn xchg(_arg: pmdp, _arg: pmd) -> return;
}

extern "C" {
    pub fn xchg(_arg: pudp, _arg: pud) -> return;
}

//
// Page table pages are page-aligned.  The lower half of the top
// level is used for userspace and the top half for the kernel.
//
// Returns true for parts of the PGD that map userspace and
// false for the parts that map the kernel.
//

//
// All top-level MITIGATION_PAGE_TABLE_ISOLATION page tables are order-1 pages
// (8k-aligned and 8k in size).  The kernel one is at the beginning 4k and
// the user one is in the last 4k.  To switch between them, you
// just need to flip the 12th bit in their addresses.
//

//
// This generates better code than the inline assembly in
// __set_bit().
//
extern "C" {
    pub fn ptr_set_bit(_arg: pgdp, _arg: PTI_PGTABLE_SWITCH_BIT) -> return;
}
extern "C" {
    pub fn ptr_clear_bit(_arg: pgdp, _arg: PTI_PGTABLE_SWITCH_BIT) -> return;
}
extern "C" {
    pub fn ptr_set_bit(_arg: p4dp, _arg: PTI_PGTABLE_SWITCH_BIT) -> return;
}
extern "C" {
    pub fn ptr_clear_bit(_arg: p4dp, _arg: PTI_PGTABLE_SWITCH_BIT) -> return;
}

//
// clone_pgd_range(pgd_t *dst, pgd_t *src, int count);
//
// dst - pointer to pgd range anywhere on a pgd page
// src - ""
// count - the number of pgds to copy.
//
// dst and src can be on the same page, but the range must not overlap,
// and must not cross a page boundary.
//

// Clone the user space pgd as well
extern "C" {
    pub fn sizeof(_arg: pgd_t)) -> *mut count;
}

//
// The x86 doesn't have any external MMU info: the kernel page
// tables contain all the necessary information.
//
extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_SWP_EXCLUSIVE) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_SWP_EXCLUSIVE) -> return;
}

extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_SWP_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_SWP_SOFT_DIRTY) -> return;
}

extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_SWP_SOFT_DIRTY) -> return;
}
extern "C" {
    pub fn pmd_clear_flags(_arg: pmd, _arg: _PAGE_SWP_SOFT_DIRTY) -> return;
}

extern "C" {
    pub fn pte_set_flags(_arg: pte, _arg: _PAGE_SWP_UFFD) -> return;
}
extern "C" {
    pub fn pte_clear_flags(_arg: pte, _arg: _PAGE_SWP_UFFD) -> return;
}
extern "C" {
    pub fn pmd_set_flags(_arg: pmd, _arg: _PAGE_SWP_UFFD) -> return;
}
extern "C" {
    pub fn pmd_clear_flags(_arg: pmd, _arg: _PAGE_SWP_UFFD) -> return;
}

// ifdef to avoid doing 59-bit shift on 32-bit values

//
// 'pteval' can come from a PTE, PMD or PUD.  We only check
// _PAGE_PRESENT, _PAGE_USER, and _PAGE_RW in here which are the
// same value on all 3 types.
//
// Write=0,Dirty=1 PTEs are shadow stack, which the kernel
// shouldn't generally allow access to, but since they
// are already Write=0, the below logic covers both cases.
//
extern "C" {
    pub fn __pkru_allows_pkey(_arg: pte_flags_pkey(pteval), _arg: write) -> return;
}

extern "C" {
    pub fn __pte_access_permitted(_arg: pte_val(pte), _arg: write) -> return;
}

extern "C" {
    pub fn __pte_access_permitted(_arg: pmd_val(pmd), _arg: write) -> return;
}

extern "C" {
    pub fn __pte_access_permitted(_arg: pud_val(pud), _arg: write) -> return;
}
pub const __HAVE_ARCH_PFN_MODIFY_ALLOWED: c_int = 1;
extern "C" {
    pub fn pfn_modify_allowed(pfn: c_ulong, prot: pgprot_t) -> bool;
}
extern "C" {
    pub fn boot_cpu_has_bug(_arg: X86_BUG_L1TF) -> return;
}

extern "C" {
    pub fn arch_check_zapped_pte(vma: *mut vm_area_struct, pte: pte_t);
}

extern "C" {
    pub fn arch_check_zapped_pmd(vma: *mut vm_area_struct, pmd: pmd_t);
}

extern "C" {
    pub fn arch_check_zapped_pud(vma: *mut vm_area_struct, pud: pud_t);
}

extern "C" {
    pub fn pmd_leaf(_PAGE_USER: pmd) && (pmd_val(pmd) & _PAGE_PRESENT) && (pmd_val(pmd) &) -> return;
}
extern "C" {
    pub fn pud_leaf(_PAGE_USER: pud) && (pud_val(pud) & _PAGE_PRESENT) && (pud_val(pud) &) -> return;
}

extern "C" {
    pub fn arch_memory_failure(pfn: c_ulong, flags: c_int) -> c_int;
}

extern "C" {
    pub fn arch_is_platform_page(paddr: u64) -> bool;
}

//
// Use set_p*_safe(), and elide TLB flushing, when confident that *no
// TLB flush will be required as a result of the "set". For example, use
// in scenarios where it is known ahead of time that the routine is
// setting non-present entries, or re-setting an existing entry to the
// same value. Otherwise, use the typical "set" helpers and flush the
// TLB.
//

