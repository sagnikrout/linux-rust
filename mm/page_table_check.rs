//! Automatically rewritten from C to Rust
//! Source: mm/page_table_check.c
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
// Copyright (c) 2021, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_table_check {
    pub anon_map_count: core::sync::atomic::AtomicI32,
    pub file_map_count: core::sync::atomic::AtomicI32,
}

    static bool __page_table_check_enabled __initdata =
    IS_ENABLED(CONFIG_PAGE_TABLE_CHECK_ENFORCED);
    DEFINE_STATIC_KEY_TRUE(page_table_check_disabled);
    EXPORT_SYMBOL(page_table_check_disabled);
#[no_mangle]
unsafe extern "C" fn early_page_table_check_param(buf: *mut c_char) -> int __init {
    static int __init early_page_table_check_param(char *buf)
    {
    return kstrtobool(buf, &__page_table_check_enabled);
    }
    early_param("page_table_check", early_page_table_check_param);
#[no_mangle]
unsafe extern "C" fn need_page_table_check() -> bool __init {
    static bool __init need_page_table_check(void)
    {
    return __page_table_check_enabled;
    }
#[no_mangle]
unsafe extern "C" fn init_page_table_check() -> void __init {
    static void __init init_page_table_check(void)
    {
    if (!__page_table_check_enabled)
    return;
    static_branch_disable(&page_table_check_disabled);
    }
    struct page_ext_operations page_table_check_ops = {
    .size = sizeof(struct page_table_check),
    .need = need_page_table_check,
    .init = init_page_table_check,
    .need_shared_flags = false,
    };
    static struct page_table_check *get_page_table_check(struct page_ext *page_ext)
    {
    BUG_ON(!page_ext);
    return page_ext_data(page_ext, &page_table_check_ops);
    }
//
// An entry is removed from the page table, decrement the counters for that page
// verify that it is of correct type and counters do not become negative.
//
#[no_mangle]
unsafe extern "C" fn page_table_check_clear(pfn: c_ulong, pgcnt: c_ulong) {
    static void page_table_check_clear(unsigned long pfn, unsigned long pgcnt)
    {
    struct page_ext_iter iter;
    struct page_ext *page_ext;
    struct page *page;
    bool anon;
    if (!pfn_valid(pfn))
    return;
    page = pfn_to_page(pfn);
    BUG_ON(PageSlab(page));
    anon = PageAnon(page);
    rcu_read_lock();
    for_each_page_ext(page, pgcnt, page_ext, iter) {
    struct page_table_check *ptc = get_page_table_check(page_ext);
    if (anon) {
    BUG_ON(atomic_read(&ptc.file_map_count));
    BUG_ON(atomic_dec_return(&ptc.anon_map_count) < 0);
    } else {
    BUG_ON(atomic_read(&ptc.anon_map_count));
    BUG_ON(atomic_dec_return(&ptc.file_map_count) < 0);
    }
    }
    rcu_read_unlock();
    }
//
// A new entry is added to the page table, increment the counters for that page
// verify that it is of correct type and is not being mapped with a different
// type to a different process.
//
    static void page_table_check_set(unsigned long pfn, unsigned long pgcnt,
    bool rw)
    {
    struct page_ext_iter iter;
    struct page_ext *page_ext;
    struct page *page;
    bool anon;
    if (!pfn_valid(pfn))
    return;
    page = pfn_to_page(pfn);
    BUG_ON(PageSlab(page));
    anon = PageAnon(page);
    rcu_read_lock();
    for_each_page_ext(page, pgcnt, page_ext, iter) {
    struct page_table_check *ptc = get_page_table_check(page_ext);
    if (anon) {
    BUG_ON(atomic_read(&ptc.file_map_count));
    BUG_ON(atomic_inc_return(&ptc.anon_map_count) > 1 && rw);
    } else {
    BUG_ON(atomic_read(&ptc.anon_map_count));
    BUG_ON(atomic_inc_return(&ptc.file_map_count) < 0);
    }
    }
    rcu_read_unlock();
    }
//
// page is on free list, or is being allocated, verify that counters are zeroes
// crash if they are not.
//
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_zero(page: *mut page, order: c_uint) {
    void __page_table_check_zero(struct page *page, unsigned int order)
    {
    struct page_ext_iter iter;
    struct page_ext *page_ext;
    BUG_ON(PageSlab(page));
    rcu_read_lock();
    for_each_page_ext(page, 1 << order, page_ext, iter) {
    struct page_table_check *ptc = get_page_table_check(page_ext);
    BUG_ON(atomic_read(&ptc.anon_map_count));
    BUG_ON(atomic_read(&ptc.file_map_count));
    }
    rcu_read_unlock();
    }
    void __page_table_check_pte_clear(struct mm_struct *mm, unsigned long addr,
    pte_t pte)
    {
    if (&init_mm == mm)
    return;
    if (pte_user_accessible_page(mm, addr, pte) && !pte_special(pte))
    page_table_check_clear(pte_pfn(pte), PAGE_SIZE >> PAGE_SHIFT);
    }
    EXPORT_SYMBOL(__page_table_check_pte_clear);
#[no_mangle]
pub unsafe extern "C" fn page_table_check_huge_zero_pmd(pmd: pmd_t) -> bool {
    static inline bool page_table_check_huge_zero_pmd(pmd_t pmd)
    {
    let mut pfn: c_ulong = pmd_pfn(pmd);
    if (!pfn_valid(pfn))
    return false;
    return is_huge_zero_folio(page_folio(pfn_to_page(pfn)));
    }
    void __page_table_check_pmd_clear(struct mm_struct *mm, unsigned long addr,
    pmd_t pmd)
    {
    if (&init_mm == mm)
    return;
    if (pmd_user_accessible_page(mm, addr, pmd) &&
    !page_table_check_huge_zero_pmd(pmd))
    page_table_check_clear(pmd_pfn(pmd), PMD_SIZE >> PAGE_SHIFT);
    }
    EXPORT_SYMBOL(__page_table_check_pmd_clear);
    void __page_table_check_pud_clear(struct mm_struct *mm, unsigned long addr,
    pud_t pud)
    {
    if (&init_mm == mm)
    return;
    if (pud_user_accessible_page(mm, addr, pud))
    page_table_check_clear(pud_pfn(pud), PUD_SIZE >> PAGE_SHIFT);
    }
    EXPORT_SYMBOL(__page_table_check_pud_clear);
// Whether the swap entry cached writable information
#[no_mangle]
pub unsafe extern "C" fn softleaf_cached_writable(entry: softleaf_t) -> bool {
    static inline bool softleaf_cached_writable(softleaf_t entry)
    {
    return softleaf_is_device_private_write(entry) ||
    softleaf_is_migration_write(entry);
    }
#[no_mangle]
unsafe extern "C" fn page_table_check_pte_flags(pte: pte_t) {
    static void page_table_check_pte_flags(pte_t pte)
    {
    if (pte_present(pte)) {
    WARN_ON_ONCE(pte_uffd(pte) && pte_write(pte));
    } else if (pte_swp_uffd(pte)) {
    let mut entry: softleaf_t = softleaf_from_pte(pte);
    WARN_ON_ONCE(softleaf_cached_writable(entry));
    }
    }
    void __page_table_check_ptes_set(struct mm_struct *mm, unsigned long addr,
    pte_t *ptep, pte_t pte, unsigned int nr)
    {
    unsigned int i;
    if (&init_mm == mm)
    return;
    page_table_check_pte_flags(pte);
    for (i = 0; i < nr; i++)
    __page_table_check_pte_clear(mm, addr + PAGE_SIZE * i, ptep_get(ptep + i));
    if (pte_user_accessible_page(mm, addr, pte) && !pte_special(pte))
    page_table_check_set(pte_pfn(pte), nr, pte_write(pte));
    }
    EXPORT_SYMBOL(__page_table_check_ptes_set);
#[no_mangle]
pub unsafe extern "C" fn page_table_check_pmd_flags(pmd: pmd_t) {
    static inline void page_table_check_pmd_flags(pmd_t pmd)
    {
    if (pmd_present(pmd)) {
    if (pmd_uffd(pmd))
    WARN_ON_ONCE(pmd_write(pmd));
    } else if (pmd_swp_uffd(pmd)) {
    let mut entry: softleaf_t = softleaf_from_pmd(pmd);
    WARN_ON_ONCE(softleaf_cached_writable(entry));
    }
    }
    void __page_table_check_pmds_set(struct mm_struct *mm, unsigned long addr,
    pmd_t *pmdp, pmd_t pmd, unsigned int nr)
    {
    let mut stride: c_ulong = PMD_SIZE >> PAGE_SHIFT;
    unsigned int i;
    if (&init_mm == mm)
    return;
    page_table_check_pmd_flags(pmd);
    for (i = 0; i < nr; i++)
    __page_table_check_pmd_clear(mm, addr + PMD_SIZE * i, *(pmdp + i));
    if (pmd_user_accessible_page(mm, addr, pmd) &&
    !page_table_check_huge_zero_pmd(pmd))
    page_table_check_set(pmd_pfn(pmd), stride * nr, pmd_write(pmd));
    }
    EXPORT_SYMBOL(__page_table_check_pmds_set);
    void __page_table_check_puds_set(struct mm_struct *mm, unsigned long addr,
    pud_t *pudp, pud_t pud,	unsigned int nr)
    {
    let mut stride: c_ulong = PUD_SIZE >> PAGE_SHIFT;
    unsigned int i;
    if (&init_mm == mm)
    return;
    for (i = 0; i < nr; i++)
    __page_table_check_pud_clear(mm, addr + PUD_SIZE * i, *(pudp + i));
    if (pud_user_accessible_page(mm, addr, pud))
    page_table_check_set(pud_pfn(pud), stride * nr, pud_write(pud));
    }
    EXPORT_SYMBOL(__page_table_check_puds_set);
    void __page_table_check_pte_clear_range(struct mm_struct *mm,
    unsigned long addr,
    pmd_t pmd)
    {
    if (&init_mm == mm)
    return;
    if (!pmd_bad(pmd) && !pmd_leaf(pmd)) {
    pte_t *ptep = pte_offset_map(&pmd, addr);
    unsigned long i;
    if (WARN_ON(!ptep))
    return;
    for (i = 0; i < PTRS_PER_PTE; i++) {
    __page_table_check_pte_clear(mm, addr, ptep_get(ptep));
    addr += PAGE_SIZE;
    ptep++;
    }
    pte_unmap(ptep - PTRS_PER_PTE);
    }
    }
