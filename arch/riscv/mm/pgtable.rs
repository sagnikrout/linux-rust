//! Automatically rewritten from C to Rust
//! Source: arch/riscv/mm/pgtable.c
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

    int ptep_set_access_flags(struct vm_area_struct *vma,
    unsigned long address, pte_t *ptep,
    pte_t entry, int dirty)
    {
    if (riscv_has_extension_unlikely(RISCV_ISA_EXT_SVVPTC)) {
    if (!pte_same(ptep_get(ptep), entry)) {
    __set_pte_at(vma.vm_mm, ptep, entry);
// Here only not svadu is impacted
    flush_tlb_page(vma, address);
    return true;
    }
    return false;
    }
    if (!pte_same(ptep_get(ptep), entry))
    __set_pte_at(vma.vm_mm, ptep, entry);
//
// update_mmu_cache will unconditionally execute, handling both
// the case that the PTE changed and the spurious fault case.
//
    return true;
    }
    bool ptep_test_and_clear_young(struct vm_area_struct *vma,
    unsigned long address, pte_t *ptep)
    {
    if (!pte_young(ptep_get(ptep)))
    return false;
    return test_and_clear_bit(_PAGE_ACCESSED_OFFSET, &pte_val(*ptep));
    }
    EXPORT_SYMBOL_GPL(ptep_test_and_clear_young);

    pud_t *pud_offset(p4d_t *p4d, unsigned long address)
    {
    if (pgtable_l4_enabled)
    return p4d_pgtable(p4dp_get(p4d)) + pud_index(address);
    return (pud_t *)p4d;
    }
    EXPORT_SYMBOL_GPL(pud_offset);
    p4d_t *p4d_offset(pgd_t *pgd, unsigned long address)
    {
    if (pgtable_l5_enabled)
    return pgd_pgtable(pgdp_get(pgd)) + p4d_index(address);
    return (p4d_t *)pgd;
    }
    EXPORT_SYMBOL_GPL(p4d_offset);

#[no_mangle]
pub unsafe extern "C" fn p4d_set_huge(p4d: *mut p4d_t, addr: phys_addr_t, prot: pgprot_t) -> c_int {
    int p4d_set_huge(p4d_t *p4d, phys_addr_t addr, pgprot_t prot)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn p4d_clear_huge(p4d: *mut p4d_t) {
    void p4d_clear_huge(p4d_t *p4d)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn pud_set_huge(pud: *mut pud_t, phys: phys_addr_t, prot: pgprot_t) -> c_int {
    int pud_set_huge(pud_t *pud, phys_addr_t phys, pgprot_t prot)
    {
    let mut new_pud: pud_t = pfn_pud(__phys_to_pfn(phys), prot);
    set_pud(pud, new_pud);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pud_clear_huge(pud: *mut pud_t) -> c_int {
    int pud_clear_huge(pud_t *pud)
    {
    if (!pud_leaf(pudp_get(pud)))
    return 0;
    pud_clear(pud);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pud_free_pmd_page(pud: *mut pud_t, addr: c_ulong) -> c_int {
    int pud_free_pmd_page(pud_t *pud, unsigned long addr)
    {
    pmd_t *pmd = pud_pgtable(pudp_get(pud));
    int i;
    pud_clear(pud);
    flush_tlb_kernel_range(addr, addr + PUD_SIZE);
    for (i = 0; i < PTRS_PER_PMD; i++) {
    if (!pmd_none(pmd[i])) {
    pte_t *pte = (pte_t *)pmd_page_vaddr(pmd[i]);
    pte_free_kernel(core::ptr::null_mut(), pte);
    }
    }
    pmd_free(core::ptr::null_mut(), pmd);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_set_huge(pmd: *mut pmd_t, phys: phys_addr_t, prot: pgprot_t) -> c_int {
    int pmd_set_huge(pmd_t *pmd, phys_addr_t phys, pgprot_t prot)
    {
    let mut new_pmd: pmd_t = pfn_pmd(__phys_to_pfn(phys), prot);
    set_pmd(pmd, new_pmd);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_clear_huge(pmd: *mut pmd_t) -> c_int {
    int pmd_clear_huge(pmd_t *pmd)
    {
    if (!pmd_leaf(pmdp_get(pmd)))
    return 0;
    pmd_clear(pmd);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_free_pte_page(pmd: *mut pmd_t, addr: c_ulong) -> c_int {
    int pmd_free_pte_page(pmd_t *pmd, unsigned long addr)
    {
    pte_t *pte = (pte_t *)pmd_page_vaddr(pmdp_get(pmd));
    pmd_clear(pmd);
    flush_tlb_kernel_range(addr, addr + PMD_SIZE);
    pte_free_kernel(core::ptr::null_mut(), pte);
    return 1;
    }

    pmd_t pmdp_collapse_flush(struct vm_area_struct *vma,
    unsigned long address, pmd_t *pmdp)
    {
    let mut pmd: pmd_t = pmdp_huge_get_and_clear(vma.vm_mm, address, pmdp);
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    VM_BUG_ON(pmd_trans_huge(pmdp_get(pmdp)));
//
// When leaf PTE entries (regular pages) are collapsed into a leaf
// PMD entry (huge page), a valid non-leaf PTE is converted into a
// valid leaf PTE at the level 1 page table.  Since the sfence.vma
// forms that specify an address only apply to leaf PTEs, we need a
// global flush here.  collapse_huge_page() assumes these flushes are
// eager, so just do the fence here.
//
    flush_tlb_mm(vma.vm_mm);
    return pmd;
    }
    pud_t pudp_invalidate(struct vm_area_struct *vma, unsigned long address,
    pud_t *pudp)
    {
    VM_WARN_ON_ONCE(!pud_present(*pudp));
    let mut old: pud_t = pudp_establish(vma, address, pudp, pud_mkinvalid(*pudp));
    flush_pud_tlb_range(vma, address, address + HPAGE_PUD_SIZE);
    return old;
    }

#[no_mangle]
pub unsafe extern "C" fn pte_mkwrite(pte: pte_t, vma: *mut vm_area_struct) -> pte_t {
    pte_t pte_mkwrite(pte_t pte, struct vm_area_struct *vma)
    {
    if (vma.vm_flags & VM_SHADOW_STACK)
    return pte_mkwrite_shstk(pte);
    return pte_mkwrite_novma(pte);
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t {
    pmd_t pmd_mkwrite(pmd_t pmd, struct vm_area_struct *vma)
    {
    if (vma.vm_flags & VM_SHADOW_STACK)
    return pmd_mkwrite_shstk(pmd);
    return pmd_mkwrite_novma(pmd);
    }
