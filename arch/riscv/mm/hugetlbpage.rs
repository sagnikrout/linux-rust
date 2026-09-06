//! Automatically rewritten from C to Rust
//! Source: arch/riscv/mm/hugetlbpage.c
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

#[no_mangle]
pub unsafe extern "C" fn huge_ptep_get(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t {
    pte_t huge_ptep_get(struct mm_struct *mm, unsigned long addr, pte_t *ptep)
    {
    unsigned long pte_num;
    int i;
    let mut orig_pte: pte_t = ptep_get(ptep);
    if (!pte_present(orig_pte) || !pte_napot(orig_pte))
    return orig_pte;
    pte_num = napot_pte_num(napot_cont_order(orig_pte));
    for (i = 0; i < pte_num; i++, ptep++) {
    let mut pte: pte_t = ptep_get(ptep);
    if (pte_dirty(pte))
    orig_pte = pte_mkdirty(orig_pte);
    if (pte_young(pte))
    orig_pte = pte_mkyoung(orig_pte);
    }
    return orig_pte;
    }
    pte_t *huge_pte_alloc(struct mm_struct *mm,
    struct vm_area_struct *vma,
    unsigned long addr,
    unsigned long sz)
    {
    unsigned long order;
    pte_t *pte = core::ptr::null_mut();
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pgd = pgd_offset(mm, addr);
    p4d = p4d_alloc(mm, pgd, addr);
    if (!p4d)
    return core::ptr::null_mut();
    pud = pud_alloc(mm, p4d, addr);
    if (!pud)
    return core::ptr::null_mut();
    if (sz == PUD_SIZE) {
    pte = (pte_t *)pud;
    goto out;
    }
    if (sz == PMD_SIZE) {
    if (want_pmd_share(vma, addr) && pud_none(pudp_get(pud)))
    pte = huge_pmd_share(mm, vma, addr, pud);
    else
    pte = (pte_t *)pmd_alloc(mm, pud, addr);
    goto out;
    }
    pmd = pmd_alloc(mm, pud, addr);
    if (!pmd)
    return core::ptr::null_mut();
    for_each_napot_order(order) {
    if (napot_cont_size(order) == sz) {
    pte = pte_alloc_huge(mm, pmd, addr & napot_cont_mask(order));
    break;
    }
    }
    out:
    if (pte) {
    let mut pteval: pte_t = ptep_get_lockless(pte);
    WARN_ON_ONCE(pte_present(pteval) && !pte_huge(pteval));
    }
    return pte;
    }
    pte_t *huge_pte_offset(struct mm_struct *mm,
    unsigned long addr,
    unsigned long sz)
    {
    unsigned long order;
    pte_t *pte = core::ptr::null_mut();
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pgd = pgd_offset(mm, addr);
    if (!pgd_present(pgdp_get(pgd)))
    return core::ptr::null_mut();
    p4d = p4d_offset(pgd, addr);
    if (!p4d_present(p4dp_get(p4d)))
    return core::ptr::null_mut();
    pud = pud_offset(p4d, addr);
    if (sz == PUD_SIZE)
// must be pud huge, non-present or none
    return (pte_t *)pud;
    if (!pud_present(pudp_get(pud)))
    return core::ptr::null_mut();
    pmd = pmd_offset(pud, addr);
    if (sz == PMD_SIZE)
// must be pmd huge, non-present or none
    return (pte_t *)pmd;
    if (!pmd_present(pmdp_get(pmd)))
    return core::ptr::null_mut();
    for_each_napot_order(order) {
    if (napot_cont_size(order) == sz) {
    pte = pte_offset_huge(pmd, addr & napot_cont_mask(order));
    break;
    }
    }
    return pte;
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_mask_last_page(h: *mut hstate) -> c_ulong {
    unsigned long hugetlb_mask_last_page(struct hstate *h)
    {
    let mut hp_size: c_ulong = huge_page_size(h);
    switch (hp_size) {

    case PUD_SIZE:
    return P4D_SIZE - PUD_SIZE;

    case PMD_SIZE:
    return PUD_SIZE - PMD_SIZE;
    case napot_cont_size(NAPOT_CONT64KB_ORDER):
    return PMD_SIZE - napot_cont_size(NAPOT_CONT64KB_ORDER);
    default:
    break;
    }
    return 0UL;
    }
    static pte_t get_clear_contig(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep,
    unsigned long ncontig)
    {
    pte_t pte, tmp_pte;
    bool present;
    pte = ptep_get_and_clear(mm, addr, ptep);
    present = pte_present(pte);
    while (--ncontig) {
    ptep++;
    addr += PAGE_SIZE;
    tmp_pte = ptep_get_and_clear(mm, addr, ptep);
    if (present) {
    if (pte_dirty(tmp_pte))
    pte = pte_mkdirty(pte);
    if (pte_young(tmp_pte))
    pte = pte_mkyoung(pte);
    }
    }
    return pte;
    }
    static pte_t get_clear_contig_flush(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep,
    unsigned long pte_num)
    {
    let mut orig_pte: pte_t = get_clear_contig(mm, addr, ptep, pte_num);
    let mut vma: vm_area_struct = TLB_FLUSH_VMA(mm, 0);
    let mut valid: bool = !pte_none(orig_pte);
    if (valid)
    flush_tlb_range(&vma, addr, addr + (PAGE_SIZE * pte_num));
    return orig_pte;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_make_huge_pte(entry: pte_t, shift: c_uint, flags: vm_flags_t) -> pte_t {
    pte_t arch_make_huge_pte(pte_t entry, unsigned int shift, vm_flags_t flags)
    {
    unsigned long order;
    for_each_napot_order(order) {
    if (shift == napot_cont_shift(order)) {
    entry = pte_mknapot(entry, order);
    break;
    }
    }
    if (order == NAPOT_ORDER_MAX)
    entry = pte_mkhuge(entry);
    return entry;
    }
    static void clear_flush(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep,
    unsigned long pgsize,
    unsigned long ncontig)
    {
    let mut vma: vm_area_struct = TLB_FLUSH_VMA(mm, 0);
    unsigned long i, saddr = addr;
    for (i = 0; i < ncontig; i++, addr += pgsize, ptep++)
    ptep_get_and_clear(mm, addr, ptep);
    flush_tlb_range(&vma, saddr, addr);
    }
#[no_mangle]
unsafe extern "C" fn num_contig_ptes_from_size(sz: c_ulong, pgsize: *mut usize) -> c_int {
    static int num_contig_ptes_from_size(unsigned long sz, size_t *pgsize)
    {
    unsigned long hugepage_shift;
    if (sz >= PGDIR_SIZE)
    hugepage_shift = PGDIR_SHIFT;
#[no_mangle]
pub unsafe extern "C" fn if(P4D_SIZE: sz >=) -> else {
    else if (sz >= P4D_SIZE)
    hugepage_shift = P4D_SHIFT;
#[no_mangle]
pub unsafe extern "C" fn if(PUD_SIZE: sz >=) -> else {
    else if (sz >= PUD_SIZE)
    hugepage_shift = PUD_SHIFT;
#[no_mangle]
pub unsafe extern "C" fn if(PMD_SIZE: sz >=) -> else {
    else if (sz >= PMD_SIZE)
    hugepage_shift = PMD_SHIFT;
    else
    hugepage_shift = PAGE_SHIFT;
// pgsize = 1 << hugepage_shift;
    return sz >> hugepage_shift;
    }
//
// When dealing with NAPOT mappings, the privileged specification indicates that
// "if an update needs to be made, the OS generally should first mark all of the
// PTEs invalid, then issue SFENCE.VMA instruction(s) covering all 4 KiB regions
// within the range, [...] then update the PTE(s), as described in Section
// 4.2.1.". That's the equivalent of the Break-Before-Make approach used by
// arm64.
//
    void set_huge_pte_at(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep,
    pte_t pte,
    unsigned long sz)
    {
    size_t pgsize;
    int i, pte_num;
    pte_num = num_contig_ptes_from_size(sz, &pgsize);
    if (!pte_present(pte)) {
    for (i = 0; i < pte_num; i++, ptep++, addr += pgsize)
    set_ptes(mm, addr, ptep, pte, 1);
    return;
    }
    if (!pte_napot(pte)) {
    set_ptes(mm, addr, ptep, pte, 1);
    return;
    }
    clear_flush(mm, addr, ptep, pgsize, pte_num);
    for (i = 0; i < pte_num; i++, ptep++, addr += pgsize)
    set_pte_at(mm, addr, ptep, pte);
    }
    int huge_ptep_set_access_flags(struct vm_area_struct *vma,
    unsigned long addr,
    pte_t *ptep,
    pte_t pte,
    int dirty)
    {
    struct mm_struct *mm = vma.vm_mm;
    unsigned long order;
    pte_t orig_pte;
    int i, pte_num;
    if (!pte_napot(pte))
    return ptep_set_access_flags(vma, addr, ptep, pte, dirty);
    order = napot_cont_order(pte);
    pte_num = napot_pte_num(order);
    ptep = huge_pte_offset(mm, addr, napot_cont_size(order));
    orig_pte = get_clear_contig_flush(mm, addr, ptep, pte_num);
    if (pte_dirty(orig_pte))
    pte = pte_mkdirty(pte);
    if (pte_young(orig_pte))
    pte = pte_mkyoung(pte);
    for (i = 0; i < pte_num; i++, addr += PAGE_SIZE, ptep++)
    set_pte_at(mm, addr, ptep, pte);
    return true;
    }
    pte_t huge_ptep_get_and_clear(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep, unsigned long sz)
    {
    size_t pgsize;
    let mut orig_pte: pte_t = ptep_get(ptep);
    int pte_num;
    if (!pte_napot(orig_pte))
    return ptep_get_and_clear(mm, addr, ptep);
    pte_num = num_contig_ptes_from_size(sz, &pgsize);
    return get_clear_contig(mm, addr, ptep, pte_num);
    }
    void huge_ptep_set_wrprotect(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep)
    {
    let mut pte: pte_t = ptep_get(ptep);
    unsigned long order;
    pte_t orig_pte;
    int i, pte_num;
    if (!pte_napot(pte)) {
    ptep_set_wrprotect(mm, addr, ptep);
    return;
    }
    order = napot_cont_order(pte);
    pte_num = napot_pte_num(order);
    ptep = huge_pte_offset(mm, addr, napot_cont_size(order));
    orig_pte = get_clear_contig_flush(mm, addr, ptep, pte_num);
    orig_pte = pte_wrprotect(orig_pte);
    for (i = 0; i < pte_num; i++, addr += PAGE_SIZE, ptep++)
    set_pte_at(mm, addr, ptep, orig_pte);
    }
    pte_t huge_ptep_clear_flush(struct vm_area_struct *vma,
    unsigned long addr,
    pte_t *ptep)
    {
    let mut pte: pte_t = ptep_get(ptep);
    int pte_num;
    if (!pte_napot(pte))
    return ptep_clear_flush(vma, addr, ptep);
    pte_num = napot_pte_num(napot_cont_order(pte));
    return get_clear_contig_flush(vma.vm_mm, addr, ptep, pte_num);
    }
    void huge_pte_clear(struct mm_struct *mm,
    unsigned long addr,
    pte_t *ptep,
    unsigned long sz)
    {
    size_t pgsize;
    let mut pte: pte_t = ptep_get(ptep);
    int i, pte_num;
    if (!pte_napot(pte)) {
    pte_clear(mm, addr, ptep);
    return;
    }
    pte_num = num_contig_ptes_from_size(sz, &pgsize);
    for (i = 0; i < pte_num; i++, addr += pgsize, ptep++)
    pte_clear(mm, addr, ptep);
    }
#[no_mangle]
unsafe extern "C" fn is_napot_size(size: c_ulong) -> bool {
    static bool is_napot_size(unsigned long size)
    {
    unsigned long order;
    if (!has_svnapot())
    return false;
    for_each_napot_order(order) {
    if (size == napot_cont_size(order))
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn napot_hugetlbpages_init() -> __init int {
    static __init int napot_hugetlbpages_init(void)
    {
    if (has_svnapot()) {
    unsigned long order;
    for_each_napot_order(order)
    hugetlb_add_hstate(order);
    }
    return 0;
    }
    arch_initcall(napot_hugetlbpages_init);

#[no_mangle]
unsafe extern "C" fn is_napot_size(size: c_ulong) -> bool {
    static bool is_napot_size(unsigned long size)
    {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn __hugetlb_valid_size(size: c_ulong) -> bool {
    static bool __hugetlb_valid_size(unsigned long size)
    {
    if (size == HPAGE_SIZE)
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(PUD_SIZE: IS_ENABLED(CONFIG_64BIT) && size ==) -> else {
    else if (IS_ENABLED(CONFIG_64BIT) && size == PUD_SIZE)
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_napot_size(size)) -> else {
    else if (is_napot_size(size))
    return true;
    else
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_valid_size(size: c_ulong) -> bool __init {
    bool __init arch_hugetlb_valid_size(unsigned long size)
    {
    return __hugetlb_valid_size(size);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_migration_supported(h: *mut hstate) -> bool {
    bool arch_hugetlb_migration_supported(struct hstate *h)
    {
    return __hugetlb_valid_size(huge_page_size(h));
    }

#[no_mangle]
unsafe extern "C" fn gigantic_pages_init() -> __init int {
    static __init int gigantic_pages_init(void)
    {
// With CONTIG_ALLOC, we can allocate gigantic pages at runtime
    if (IS_ENABLED(CONFIG_64BIT))
    hugetlb_add_hstate(PUD_SHIFT - PAGE_SHIFT);
    return 0;
    }
    arch_initcall(gigantic_pages_init);

#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_cma_order() -> unsigned int __init {
    unsigned int __init arch_hugetlb_cma_order(void)
    {
    if (IS_ENABLED(CONFIG_64BIT))
    return PUD_SHIFT - PAGE_SHIFT;
    return 0;
    }
