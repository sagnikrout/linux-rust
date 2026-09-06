//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/pageattr.c
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_change_data {
    pub set_mask: pgprot_t,
    pub clear_mask: pgprot_t,
}

#[no_mangle]
unsafe extern "C" fn set_pageattr_masks(val: ptval_t, walk: *mut mm_walk) -> ptval_t {
    static ptval_t set_pageattr_masks(ptval_t val, struct mm_walk *walk)
    {
    struct page_change_data *masks = walk.private;
//
// Some users clear and set bits which alias each other (e.g. PTE_NG and
// PTE_PRESENT_INVALID). It is therefore important that we always clear
// first then set.
//
    val &= ~(pgprot_val(masks.clear_mask));
    val |= (pgprot_val(masks.set_mask));
    return val;
    }
    static int pageattr_pud_entry(pud_t *pud, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pud_t = pudp_get(pud);
    if (pud_leaf(val)) {
    if (WARN_ON_ONCE((next - addr) != PUD_SIZE))
    return -EINVAL;
    val = __pud(set_pageattr_masks(pud_val(val), walk));
    set_pud(pud, val);
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int pageattr_pmd_entry(pmd_t *pmd, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pmd_t = pmdp_get(pmd);
    if (pmd_leaf(val)) {
    if (WARN_ON_ONCE((next - addr) != PMD_SIZE))
    return -EINVAL;
    val = __pmd(set_pageattr_masks(pmd_val(val), walk));
    set_pmd(pmd, val);
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int pageattr_pte_entry(pte_t *pte, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pte_t = __ptep_get(pte);
    val = __pte(set_pageattr_masks(pte_val(val), walk));
    __set_pte(pte, val);
    return 0;
    }
    static const struct mm_walk_ops pageattr_ops = {
    .pud_entry	= pageattr_pud_entry,
    .pmd_entry	= pageattr_pmd_entry,
    .pte_entry	= pageattr_pte_entry,
    };
    let mut __ro_after_init: bool rodata_full = true;
#[no_mangle]
pub unsafe extern "C" fn can_set_direct_map() -> bool {
    bool can_set_direct_map(void)
    {
//
// rodata_full, DEBUG_PAGEALLOC and a Realm guest all require linear
// map to be mapped at page granularity, so that it is possible to
// protect/unprotect single pages.
//
// KFENCE pool requires page-granular mapping if initialized late.
//
// Realms need to make pages shared/protected at page granularity.
//
    return rodata_full || debug_pagealloc_enabled() ||
    arm64_kfence_can_set_direct_map() || is_realm_world();
    }
    static int update_range_prot(unsigned long start, unsigned long size,
    pgprot_t set_mask, pgprot_t clear_mask)
    {
    struct page_change_data data;
    int ret;
    data.set_mask = set_mask;
    data.clear_mask = clear_mask;
    ret = split_kernel_leaf_mapping(start, start + size);
    if (WARN_ON_ONCE(ret))
    return ret;
    lazy_mmu_mode_enable();
//
// The caller must ensure that the range we are operating on does not
// partially overlap a block mapping, or a cont mapping. Any such case
// must be eliminated by splitting the mapping.
//
    ret = walk_kernel_page_table_range_lockless(start, start + size,
    &pageattr_ops, core::ptr::null_mut(), &data);
    lazy_mmu_mode_disable();
    return ret;
    }
    static int __change_memory_common(unsigned long start, unsigned long size,
    pgprot_t set_mask, pgprot_t clear_mask)
    {
    int ret;
    ret = update_range_prot(start, size, set_mask, clear_mask);
//
// If the memory is being switched from present-invalid to valid without
// changing any other bits then a TLBI isn't required as a non-valid
// entry cannot be cached in the TLB.
//
    if (pgprot_val(set_mask) != PTE_PRESENT_VALID_KERNEL ||
    pgprot_val(clear_mask) != PTE_PRESENT_INVALID)
    flush_tlb_kernel_range(start, start + size);
    return ret;
    }
    static int change_memory_common(unsigned long addr, int numpages,
    pgprot_t set_mask, pgprot_t clear_mask)
    {
    let mut start: c_ulong = addr;
    let mut size: c_ulong = PAGE_SIZE * numpages;
    let mut end: c_ulong = start + size;
    struct vm_struct *area;
    int ret;
    if (!PAGE_ALIGNED(addr)) {
    start &= PAGE_MASK;
    end = start + size;
    WARN_ON_ONCE(1);
    }
//
// Kernel VA mappings are always live, and splitting live section
// mappings into page mappings may cause TLB conflicts. This means
// we have to ensure that changing the permission bits of the range
// we are operating on does not result in such splitting.
//
// Let's restrict ourselves to mappings created by vmalloc (or vmap).
// Disallow VM_ALLOW_HUGE_VMAP mappings to guarantee that only page
// mappings are updated and splitting is never needed.
//
// So check whether the [addr, addr + size) interval is entirely
// covered by precisely one VM area that has the VM_ALLOC flag set.
//
    area = find_vm_area((void *)addr);
    if (!area ||
    ((unsigned long)kasan_reset_tag((void *)end) >
    (unsigned long)kasan_reset_tag(area.addr) + area.size) ||
    ((area.flags & (VM_ALLOC | VM_ALLOW_HUGE_VMAP)) != VM_ALLOC))
    return -EINVAL;
    if (!numpages)
    return 0;
//
// If we are manipulating read-only permissions, apply the same
// change to the linear mapping of the pages that back this VM area.
//
    if (rodata_full && (pgprot_val(set_mask) == PTE_RDONLY ||
    pgprot_val(clear_mask) == PTE_RDONLY)) {
    unsigned long idx = ((unsigned long)kasan_reset_tag((void *)start) -
    (unsigned long)kasan_reset_tag(area.addr))
    >> PAGE_SHIFT;
    for (; numpages; idx++, numpages--) {
    ret = __change_memory_common((u64)page_address(area.pages[idx]),
    PAGE_SIZE, set_mask, clear_mask);
    if (ret)
    return ret;
    }
    }
//
// Get rid of potentially aliasing lazily unmapped vm areas that may
// have permissions set that deviate from the ones we are setting here.
//
    vm_unmap_aliases();
    return __change_memory_common(start, size, set_mask, clear_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_ro(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_ro(unsigned long addr, int numpages)
    {
    return change_memory_common(addr, numpages,
    __pgprot(PTE_RDONLY),
    __pgprot(PTE_WRITE));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_rw(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_rw(unsigned long addr, int numpages)
    {
    return change_memory_common(addr, numpages,
    __pgprot(PTE_WRITE),
    __pgprot(PTE_RDONLY));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_nx(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_nx(unsigned long addr, int numpages)
    {
    return change_memory_common(addr, numpages,
    __pgprot(PTE_PXN),
    __pgprot(PTE_MAYBE_GP));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_x(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_x(unsigned long addr, int numpages)
    {
    return change_memory_common(addr, numpages,
    __pgprot(PTE_MAYBE_GP),
    __pgprot(PTE_PXN));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_valid(addr: c_ulong, numpages: c_int, enable: c_int) -> c_int {
    int set_memory_valid(unsigned long addr, int numpages, int enable)
    {
    if (enable)
    return __change_memory_common(addr, PAGE_SIZE * numpages,
    __pgprot(PTE_PRESENT_VALID_KERNEL),
    __pgprot(PTE_PRESENT_INVALID));
    else
    return __change_memory_common(addr, PAGE_SIZE * numpages,
    __pgprot(PTE_PRESENT_INVALID),
    __pgprot(PTE_PRESENT_VALID_KERNEL));
    }
#[no_mangle]
pub unsafe extern "C" fn set_direct_map_invalid_noflush(page: *mut page) -> c_int {
    int set_direct_map_invalid_noflush(struct page *page)
    {
    let mut clear_mask: pgprot_t = __pgprot(PTE_PRESENT_VALID_KERNEL);
    let mut set_mask: pgprot_t = __pgprot(PTE_PRESENT_INVALID);
    if (!can_set_direct_map())
    return 0;
    return update_range_prot((unsigned long)page_address(page),
    PAGE_SIZE, set_mask, clear_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn set_direct_map_default_noflush(page: *mut page) -> c_int {
    int set_direct_map_default_noflush(struct page *page)
    {
    let mut set_mask: pgprot_t = __pgprot(PTE_PRESENT_VALID_KERNEL | PTE_WRITE);
    let mut clear_mask: pgprot_t = __pgprot(PTE_PRESENT_INVALID | PTE_RDONLY);
    if (!can_set_direct_map())
    return 0;
    return update_range_prot((unsigned long)page_address(page),
    PAGE_SIZE, set_mask, clear_mask);
    }
    static int __set_memory_enc_dec(unsigned long addr,
    int numpages,
    bool encrypt)
    {
    let mut set_prot: c_ulong = 0, clear_prot = 0;
    phys_addr_t start, end;
    int ret;
    if (!is_realm_world())
    return 0;
    if (!__is_lm_address(addr))
    return -EINVAL;
    start = __virt_to_phys(addr);
    end = start + numpages * PAGE_SIZE;
    if (encrypt)
    clear_prot = PROT_NS_SHARED;
    else
    set_prot = PROT_NS_SHARED;
//
// Break the mapping before we make any changes to avoid stale TLB
// entries or Synchronous External Aborts caused by RIPAS_EMPTY
//
    ret = __change_memory_common(addr, PAGE_SIZE * numpages,
    __pgprot(set_prot | PTE_PRESENT_INVALID),
    __pgprot(clear_prot | PTE_PRESENT_VALID_KERNEL));
    if (ret)
    return ret;
    if (encrypt)
    ret = rsi_set_memory_range_protected(start, end);
    else
    ret = rsi_set_memory_range_shared(start, end);
    if (ret)
    return ret;
    return __change_memory_common(addr, PAGE_SIZE * numpages,
    __pgprot(PTE_PRESENT_VALID_KERNEL),
    __pgprot(PTE_PRESENT_INVALID));
    }
#[no_mangle]
unsafe extern "C" fn realm_set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int {
    static int realm_set_memory_encrypted(unsigned long addr, int numpages)
    {
    let mut ret: c_int = __set_memory_enc_dec(addr, numpages, true);
//
// If the request to change state fails, then the only sensible cause
// of action for the caller is to leak the memory
//
    WARN(ret, "Failed to encrypt memory, %d pages will be leaked",
    numpages);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn realm_set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int {
    static int realm_set_memory_decrypted(unsigned long addr, int numpages)
    {
    let mut ret: c_int = __set_memory_enc_dec(addr, numpages, false);
    WARN(ret, "Failed to decrypt memory, %d pages will be leaked",
    numpages);
    return ret;
    }
    static const struct arm64_mem_crypt_ops realm_crypt_ops = {
    .encrypt = realm_set_memory_encrypted,
    .decrypt = realm_set_memory_decrypted,
    };
#[no_mangle]
pub unsafe extern "C" fn realm_register_memory_enc_ops() -> c_int {
    int realm_register_memory_enc_ops(void)
    {
    return arm64_mem_crypt_ops_register(&realm_crypt_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn set_direct_map_valid_noflush(page: *mut page, nr: unsigned, valid: bool) -> c_int {
    int set_direct_map_valid_noflush(struct page *page, unsigned nr, bool valid)
    {
    let mut addr: c_ulong = (unsigned long)page_address(page);
    if (!can_set_direct_map())
    return 0;
    return set_memory_valid(addr, nr, valid);
    }

//
// This is - apart from the return value - doing the same
// thing as the new set_direct_map_valid_noflush() function.
//
// Unify? Explain the conceptual differences?
//
#[no_mangle]
pub unsafe extern "C" fn __kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) {
    void __kernel_map_pages(struct page *page, int numpages, int enable)
    {
    if (!can_set_direct_map())
    return;
    set_memory_valid((unsigned long)page_address(page), numpages, enable);
    }

//
// This function is used to determine if a linear map page has been marked as
// not-valid. Walk the page table and check the PTE_VALID bit.
//
// Because this is only called on the kernel linear map,  p?d_sect() implies
// p?d_present(). When debug_pagealloc is enabled, sections mappings are
// disabled.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_page_present(page: *mut page) -> bool {
    bool kernel_page_present(struct page *page)
    {
    pgd_t *pgdp;
    p4d_t *p4dp;
    pud_t *pudp, pud;
    pmd_t *pmdp, pmd;
    pte_t *ptep;
    let mut addr: c_ulong = (unsigned long)page_address(page);
    pgdp = pgd_offset_k(addr);
    if (pgd_none(READ_ONCE(*pgdp)))
    return false;
    p4dp = p4d_offset(pgdp, addr);
    if (p4d_none(READ_ONCE(*p4dp)))
    return false;
    pudp = pud_offset(p4dp, addr);
    pud = READ_ONCE(*pudp);
    if (pud_none(pud))
    return false;
    if (pud_leaf(pud))
    return pud_valid(pud);
    pmdp = pmd_offset(pudp, addr);
    pmd = READ_ONCE(*pmdp);
    if (pmd_none(pmd))
    return false;
    if (pmd_leaf(pmd))
    return pmd_valid(pmd);
    ptep = pte_offset_kernel(pmdp, addr);
    return pte_valid(__ptep_get(ptep));
    }
