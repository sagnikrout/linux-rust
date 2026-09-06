//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/vmem.c
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
// Copyright IBM Corp. 2006
//

    static DEFINE_MUTEX(vmem_mutex);
    static void __ref *vmem_alloc_pages(unsigned int order)
    {
    let mut size: c_ulong = PAGE_SIZE << order;
    if (slab_is_available())
    return (void *)__get_free_pages(GFP_KERNEL, order);
    return memblock_alloc(size, size);
    }
#[no_mangle]
unsafe extern "C" fn vmem_free_pages(addr: c_ulong, order: c_int, altmap: *mut vmem_altmap) {
    static void vmem_free_pages(unsigned long addr, int order, struct vmem_altmap *altmap)
    {
    struct page *page;
    if (altmap) {
    vmem_altmap_free(altmap, 1 << order);
    return;
    }
    page = virt_to_page((void *)addr);
    if (PageReserved(page)) {
// allocated from memblock
    free_reserved_pages(page, order);
    } else {
    free_pages(addr, order);
    }
    }
    void *vmem_crst_alloc(unsigned long val)
    {
    unsigned long *table;
    table = vmem_alloc_pages(CRST_ALLOC_ORDER);
    if (!table)
    return core::ptr::null_mut();
    crst_table_init(table, val);
    __arch_set_page_dat(table, 1UL << CRST_ALLOC_ORDER);
    return table;
    }
    pte_t __ref *vmem_pte_alloc(void)
    {
    pte_t *pte;
    if (slab_is_available())
    pte = (pte_t *)page_table_alloc(&init_mm);
    else
    pte = (pte_t *)memblock_alloc(PAGE_SIZE, PAGE_SIZE);
    if (!pte)
    return core::ptr::null_mut();
    memset64((u64 *)pte, _PAGE_INVALID, PTRS_PER_PTE);
    __arch_set_page_dat(pte, 1);
    return pte;
    }
#[no_mangle]
unsafe extern "C" fn vmem_pte_free(table: *mut c_ulong) {
    static void vmem_pte_free(unsigned long *table)
    {
    page_table_free(&init_mm, table);
    }
pub const PAGE_UNUSED: c_uint = 0xFD;
//
// The unused vmemmap range, which was not yet memset(PAGE_UNUSED) ranges
// from unused_sub_pmd_start to next PMD_SIZE boundary.
//
    static unsigned long unused_sub_pmd_start;
#[no_mangle]
unsafe extern "C" fn vmemmap_flush_unused_sub_pmd() {
    static void vmemmap_flush_unused_sub_pmd(void)
    {
    if (!unused_sub_pmd_start)
    return;
    memset((void *)unused_sub_pmd_start, PAGE_UNUSED,
    ALIGN(unused_sub_pmd_start, PMD_SIZE) - unused_sub_pmd_start);
    unused_sub_pmd_start = 0;
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_mark_sub_pmd_used(start: c_ulong, end: c_ulong) {
    static void vmemmap_mark_sub_pmd_used(unsigned long start, unsigned long end)
    {
//
// As we expect to add in the same granularity as we remove, it's
// sufficient to mark only some piece used to block the memmap page from
// getting removed (just in case the memmap never gets initialized,
// e.g., because the memory block never gets onlined).
//
    memset((void *)start, 0, sizeof(struct page));
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_use_sub_pmd(start: c_ulong, end: c_ulong) {
    static void vmemmap_use_sub_pmd(unsigned long start, unsigned long end)
    {
//
// We only optimize if the new used range directly follows the
// previously unused range (esp., when populating consecutive sections).
//
    if (unused_sub_pmd_start == start) {
    unused_sub_pmd_start = end;
    if (likely(IS_ALIGNED(unused_sub_pmd_start, PMD_SIZE)))
    unused_sub_pmd_start = 0;
    return;
    }
    vmemmap_flush_unused_sub_pmd();
    vmemmap_mark_sub_pmd_used(start, end);
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_use_new_sub_pmd(start: c_ulong, end: c_ulong) {
    static void vmemmap_use_new_sub_pmd(unsigned long start, unsigned long end)
    {
    let mut page: c_ulong = ALIGN_DOWN(start, PMD_SIZE);
    vmemmap_flush_unused_sub_pmd();
// Could be our memmap page is filled with PAGE_UNUSED already ...
    vmemmap_mark_sub_pmd_used(start, end);
// Mark the unused parts of the new memmap page PAGE_UNUSED.
    if (!IS_ALIGNED(start, PMD_SIZE))
    memset((void *)page, PAGE_UNUSED, start - page);
//
// We want to avoid memset(PAGE_UNUSED) when populating the vmemmap of
// consecutive sections. Remember for the last added PMD the last
// unused range in the populated PMD.
//
    if (!IS_ALIGNED(end, PMD_SIZE))
    unused_sub_pmd_start = end;
    }
// Returns true if the PMD is completely unused and can be freed.
#[no_mangle]
unsafe extern "C" fn vmemmap_unuse_sub_pmd(start: c_ulong, end: c_ulong) -> bool {
    static bool vmemmap_unuse_sub_pmd(unsigned long start, unsigned long end)
    {
    let mut page: c_ulong = ALIGN_DOWN(start, PMD_SIZE);
    vmemmap_flush_unused_sub_pmd();
    memset((void *)start, PAGE_UNUSED, end - start);
    return !memchr_inv((void *)page, PAGE_UNUSED, PMD_SIZE);
    }
// __ref: we'll only call vmemmap_alloc_block() via vmemmap_populate()
    static int __ref modify_pte_table(pmd_t *pmd, unsigned long addr,
    unsigned long end, bool add, bool direct,
    struct vmem_altmap *altmap)
    {
    unsigned long prot, pages = 0;
    let mut ret: c_int = -ENOMEM;
    pte_t *pte, entry;
    prot = pgprot_val(PAGE_KERNEL);
    pte = pte_offset_kernel(pmd, addr);
    for (; addr < end; addr += PAGE_SIZE, pte++) {
    entry = ptep_get(pte);
    if (!add) {
    if (pte_none(entry))
    continue;
    if (!direct)
    vmem_free_pages((unsigned long)pfn_to_virt(pte_pfn(entry)), get_order(PAGE_SIZE), altmap);
    pte_clear(&init_mm, addr, pte);
    } else if (pte_none(entry)) {
    if (!direct) {
    void *new_page = vmemmap_alloc_block_buf(PAGE_SIZE, NUMA_NO_NODE, altmap);
    if (!new_page)
    goto out;
    set_pte(pte, __pte(__pa(new_page) | prot));
    } else {
    set_pte(pte, __pte(__pa(addr) | prot));
    }
    } else {
    continue;
    }
    pages++;
    }
    ret = 0;
    out:
    if (direct)
    update_page_count(PG_DIRECT_MAP_4K, add ? pages : -pages);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn try_free_pte_table(pmd: *mut pmd_t, start: c_ulong) {
    static void try_free_pte_table(pmd_t *pmd, unsigned long start)
    {
    pte_t *pte;
    int i;
// We can safely assume this is fully in 1:1 mapping & vmemmap area
    pte = pte_offset_kernel(pmd, start);
    for (i = 0; i < PTRS_PER_PTE; i++, pte++) {
    if (!pte_none(ptep_get(pte)))
    return;
    }
    vmem_pte_free((unsigned long *)pmd_deref(pmdp_get(pmd)));
    pmd_clear(pmd);
    }
// __ref: we'll only call vmemmap_alloc_block() via vmemmap_populate()
    static int __ref modify_pmd_table(pud_t *pud, unsigned long addr,
    unsigned long end, bool add, bool direct,
    struct vmem_altmap *altmap)
    {
    unsigned long next, prot, pages = 0;
    let mut ret: c_int = -ENOMEM;
    pmd_t entry;
    pmd_t *pmd;
    pte_t *pte;
    prot = pgprot_val(SEGMENT_KERNEL);
    pmd = pmd_offset(pud, addr);
    for (; addr < end; addr = next, pmd++) {
    next = pmd_addr_end(addr, end);
    entry = pmdp_get(pmd);
    if (!add) {
    if (pmd_none(entry))
    continue;
    if (pmd_leaf(entry)) {
    if (IS_ALIGNED(addr, PMD_SIZE) &&
    IS_ALIGNED(next, PMD_SIZE)) {
    if (!direct)
    vmem_free_pages(pmd_deref(entry), get_order(PMD_SIZE), altmap);
    pmd_clear(pmd);
    pages++;
    } else if (!direct && vmemmap_unuse_sub_pmd(addr, next)) {
    vmem_free_pages(pmd_deref(entry), get_order(PMD_SIZE), altmap);
    pmd_clear(pmd);
    }
    continue;
    }
    } else if (pmd_none(entry)) {
    if (IS_ALIGNED(addr, PMD_SIZE) &&
    IS_ALIGNED(next, PMD_SIZE) &&
    cpu_has_edat1() && direct &&
    !debug_pagealloc_enabled()) {
    set_pmd(pmd, __pmd(__pa(addr) | prot));
    pages++;
    continue;
    } else if (!direct && cpu_has_edat1()) {
    void *new_page;
//
// Use 1MB frames for vmemmap if available. We
// always use large frames even if they are only
// partially used. Otherwise we would have also
// page tables since vmemmap_populate gets
// called for each section separately.
//
    new_page = vmemmap_alloc_block_buf(PMD_SIZE, NUMA_NO_NODE, altmap);
    if (new_page) {
    set_pmd(pmd, __pmd(__pa(new_page) | prot));
    if (!IS_ALIGNED(addr, PMD_SIZE) ||
    !IS_ALIGNED(next, PMD_SIZE)) {
    vmemmap_use_new_sub_pmd(addr, next);
    }
    continue;
    }
    }
    pte = vmem_pte_alloc();
    if (!pte)
    goto out;
    pmd_populate(&init_mm, pmd, pte);
    } else if (pmd_leaf(entry)) {
    if (!direct)
    vmemmap_use_sub_pmd(addr, next);
    continue;
    }
    ret = modify_pte_table(pmd, addr, next, add, direct, altmap);
    if (ret)
    goto out;
    if (!add)
    try_free_pte_table(pmd, addr & PMD_MASK);
    }
    ret = 0;
    out:
    if (direct)
    update_page_count(PG_DIRECT_MAP_1M, add ? pages : -pages);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn try_free_pmd_table(pud: *mut pud_t, start: c_ulong) {
    static void try_free_pmd_table(pud_t *pud, unsigned long start)
    {
    pmd_t *pmd;
    int i;
    pmd = pmd_offset(pud, start);
    for (i = 0; i < PTRS_PER_PMD; i++, pmd++)
    if (!pmd_none(pmdp_get(pmd)))
    return;
    vmem_free_pages(pud_deref(pudp_get(pud)), CRST_ALLOC_ORDER, core::ptr::null_mut());
    pud_clear(pud);
    }
    static int modify_pud_table(p4d_t *p4d, unsigned long addr, unsigned long end,
    bool add, bool direct, struct vmem_altmap *altmap)
    {
    unsigned long next, prot, pages = 0;
    let mut ret: c_int = -ENOMEM;
    pud_t *pud, entry;
    pmd_t *pmd;
    prot = pgprot_val(REGION3_KERNEL);
    pud = pud_offset(p4d, addr);
    for (; addr < end; addr = next, pud++) {
    next = pud_addr_end(addr, end);
    entry = pudp_get(pud);
    if (!add) {
    if (pud_none(entry))
    continue;
    if (pud_leaf(entry)) {
    if (IS_ALIGNED(addr, PUD_SIZE) &&
    IS_ALIGNED(next, PUD_SIZE)) {
    if (!direct)
    vmem_free_pages(pud_deref(entry), get_order(PUD_SIZE), altmap);
    pud_clear(pud);
    pages++;
    continue;
    } else {
    split_pud_page(pud, addr & PUD_MASK);
    }
    }
    } else if (pud_none(entry)) {
    if (IS_ALIGNED(addr, PUD_SIZE) &&
    IS_ALIGNED(next, PUD_SIZE) &&
    cpu_has_edat2() && direct &&
    !debug_pagealloc_enabled()) {
    set_pud(pud, __pud(__pa(addr) | prot));
    pages++;
    continue;
    }
    pmd = vmem_crst_alloc(_SEGMENT_ENTRY_EMPTY);
    if (!pmd)
    goto out;
    pud_populate(&init_mm, pud, pmd);
    } else if (pud_leaf(entry)) {
    continue;
    }
    ret = modify_pmd_table(pud, addr, next, add, direct, altmap);
    if (ret)
    goto out;
    if (!add)
    try_free_pmd_table(pud, addr & PUD_MASK);
    }
    ret = 0;
    out:
    if (direct)
    update_page_count(PG_DIRECT_MAP_2G, add ? pages : -pages);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn try_free_pud_table(p4d: *mut p4d_t, start: c_ulong) {
    static void try_free_pud_table(p4d_t *p4d, unsigned long start)
    {
    pud_t *pud;
    int i;
    pud = pud_offset(p4d, start);
    for (i = 0; i < PTRS_PER_PUD; i++, pud++) {
    if (!pud_none(pudp_get(pud)))
    return;
    }
    vmem_free_pages(p4d_deref(p4dp_get(p4d)), CRST_ALLOC_ORDER, core::ptr::null_mut());
    p4d_clear(p4d);
    }
    static int modify_p4d_table(pgd_t *pgd, unsigned long addr, unsigned long end,
    bool add, bool direct, struct vmem_altmap *altmap)
    {
    unsigned long next;
    let mut ret: c_int = -ENOMEM;
    p4d_t *p4d, entry;
    pud_t *pud;
    p4d = p4d_offset(pgd, addr);
    for (; addr < end; addr = next, p4d++) {
    next = p4d_addr_end(addr, end);
    entry = p4dp_get(p4d);
    if (!add) {
    if (p4d_none(entry))
    continue;
    } else if (p4d_none(entry)) {
    pud = vmem_crst_alloc(_REGION3_ENTRY_EMPTY);
    if (!pud)
    goto out;
    p4d_populate(&init_mm, p4d, pud);
    }
    ret = modify_pud_table(p4d, addr, next, add, direct, altmap);
    if (ret)
    goto out;
    if (!add)
    try_free_pud_table(p4d, addr & P4D_MASK);
    }
    ret = 0;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn try_free_p4d_table(pgd: *mut pgd_t, start: c_ulong) {
    static void try_free_p4d_table(pgd_t *pgd, unsigned long start)
    {
    p4d_t *p4d;
    int i;
    p4d = p4d_offset(pgd, start);
    for (i = 0; i < PTRS_PER_P4D; i++, p4d++) {
    if (!p4d_none(p4dp_get(p4d)))
    return;
    }
    vmem_free_pages(pgd_deref(pgdp_get(pgd)), CRST_ALLOC_ORDER, core::ptr::null_mut());
    pgd_clear(pgd);
    }
    static int modify_pagetable(unsigned long start, unsigned long end, bool add,
    bool direct, struct vmem_altmap *altmap)
    {
    unsigned long addr, next;
    let mut ret: c_int = -ENOMEM;
    pgd_t *pgd, entry;
    p4d_t *p4d;
    if (WARN_ON_ONCE(!PAGE_ALIGNED(start | end)))
    return -EINVAL;
// Don't mess with any tables not fully in 1:1 mapping, vmemmap & kasan area

    if (WARN_ON_ONCE(!(start >= KASAN_SHADOW_START && end <= KASAN_SHADOW_END) &&
    end > __abs_lowcore))
    return -EINVAL;

    if (WARN_ON_ONCE(end > __abs_lowcore))
    return -EINVAL;

    for (addr = start; addr < end; addr = next) {
    next = pgd_addr_end(addr, end);
    pgd = pgd_offset_k(addr);
    entry = pgdp_get(pgd);
    if (!add) {
    if (pgd_none(entry))
    continue;
    } else if (pgd_none(entry)) {
    p4d = vmem_crst_alloc(_REGION2_ENTRY_EMPTY);
    if (!p4d)
    goto out;
    pgd_populate(&init_mm, pgd, p4d);
    }
    ret = modify_p4d_table(pgd, addr, next, add, direct, altmap);
    if (ret)
    goto out;
    if (!add)
    try_free_p4d_table(pgd, addr & PGDIR_MASK);
    }
    ret = 0;
    out:
    if (!add)
    flush_tlb_kernel_range(start, end);
    return ret;
    }
    static int add_pagetable(unsigned long start, unsigned long end, bool direct,
    struct vmem_altmap *altmap)
    {
    return modify_pagetable(start, end, true, direct, altmap);
    }
    static int remove_pagetable(unsigned long start, unsigned long end, bool direct,
    struct vmem_altmap *altmap)
    {
    return modify_pagetable(start, end, false, direct, altmap);
    }
//
// Add a physical memory range to the 1:1 mapping.
//
#[no_mangle]
unsafe extern "C" fn vmem_add_range(start: c_ulong, size: c_ulong) -> c_int {
    static int vmem_add_range(unsigned long start, unsigned long size)
    {
    start = (unsigned long)__va(start);
    return add_pagetable(start, start + size, true, core::ptr::null_mut());
    }
//
// Remove a physical memory range from the 1:1 mapping.
//
#[no_mangle]
unsafe extern "C" fn vmem_remove_range(start: c_ulong, size: c_ulong) {
    static void vmem_remove_range(unsigned long start, unsigned long size)
    {
    start = (unsigned long)__va(start);
    remove_pagetable(start, start + size, true, core::ptr::null_mut());
    }
//
// Add a backed mem_map array to the virtual mem_map array.
//
    int __meminit vmemmap_populate(unsigned long start, unsigned long end, int node,
    struct vmem_altmap *altmap)
    {
    int ret;
    mutex_lock(&vmem_mutex);
// We don't care about the node, just use NUMA_NO_NODE on allocations
    ret = add_pagetable(start, end, false, altmap);
    if (ret)
    remove_pagetable(start, end, false, altmap);
    mutex_unlock(&vmem_mutex);
    return ret;
    }

    void vmemmap_free(unsigned long start, unsigned long end,
    struct vmem_altmap *altmap)
    {
    mutex_lock(&vmem_mutex);
    remove_pagetable(start, end, false, altmap);
    mutex_unlock(&vmem_mutex);
    }

#[no_mangle]
pub unsafe extern "C" fn vmem_remove_mapping(start: c_ulong, size: c_ulong) {
    void vmem_remove_mapping(unsigned long start, unsigned long size)
    {
    mutex_lock(&vmem_mutex);
    vmem_remove_range(start, size);
    mutex_unlock(&vmem_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_mappable_range() -> range {
    struct range arch_get_mappable_range(void)
    {
    struct range mhp_range;
    mhp_range.start = 0;
    mhp_range.end = max_mappable - 1;
    return mhp_range;
    }
#[no_mangle]
pub unsafe extern "C" fn vmem_add_mapping(start: c_ulong, size: c_ulong) -> c_int {
    int vmem_add_mapping(unsigned long start, unsigned long size)
    {
    let mut range: range = arch_get_mappable_range();
    int ret;
    if (start < range.start ||
    start + size > range.end + 1 ||
    start + size < start)
    return -ERANGE;
    mutex_lock(&vmem_mutex);
    ret = vmem_add_range(start, size);
    if (ret)
    vmem_remove_range(start, size);
    mutex_unlock(&vmem_mutex);
    return ret;
    }
//
// Allocate new or return existing page-table entry, but do not map it
// to any physical address. If missing, allocate segment- and region-
// table entries along. Meeting a large segment- or region-table entry
// while traversing is an error, since the function is expected to be
// called against virtual regions reserved for 4KB mappings only.
//
    pte_t *vmem_get_alloc_pte(unsigned long addr, bool alloc)
    {
    pte_t *ptep = core::ptr::null_mut();
    pud_t pud_entry;
    pmd_t pmd_entry;
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    pgd = pgd_offset_k(addr);
    if (pgd_none(pgdp_get(pgd))) {
    if (!alloc)
    goto out;
    p4d = vmem_crst_alloc(_REGION2_ENTRY_EMPTY);
    if (!p4d)
    goto out;
    pgd_populate(&init_mm, pgd, p4d);
    }
    p4d = p4d_offset(pgd, addr);
    if (p4d_none(p4dp_get(p4d))) {
    if (!alloc)
    goto out;
    pud = vmem_crst_alloc(_REGION3_ENTRY_EMPTY);
    if (!pud)
    goto out;
    p4d_populate(&init_mm, p4d, pud);
    }
    pud = pud_offset(p4d, addr);
    pud_entry = pudp_get(pud);
    if (pud_none(pud_entry)) {
    if (!alloc)
    goto out;
    pmd = vmem_crst_alloc(_SEGMENT_ENTRY_EMPTY);
    if (!pmd)
    goto out;
    pud_populate(&init_mm, pud, pmd);
    } else if (WARN_ON_ONCE(pud_leaf(pud_entry))) {
    goto out;
    }
    pmd = pmd_offset(pud, addr);
    pmd_entry = pmdp_get(pmd);
    if (pmd_none(pmd_entry)) {
    if (!alloc)
    goto out;
    pte = vmem_pte_alloc();
    if (!pte)
    goto out;
    pmd_populate(&init_mm, pmd, pte);
    } else if (WARN_ON_ONCE(pmd_leaf(pmd_entry))) {
    goto out;
    }
    ptep = pte_offset_kernel(pmd, addr);
    out:
    return ptep;
    }
#[no_mangle]
pub unsafe extern "C" fn __vmem_map_4k_page(addr: c_ulong, phys: c_ulong, prot: pgprot_t, alloc: bool) -> c_int {
    int __vmem_map_4k_page(unsigned long addr, unsigned long phys, pgprot_t prot, bool alloc)
    {
    pte_t *ptep, pte;
    if (!IS_ALIGNED(addr, PAGE_SIZE))
    return -EINVAL;
    ptep = vmem_get_alloc_pte(addr, alloc);
    if (!ptep)
    return -ENOMEM;
    __ptep_ipte(addr, ptep, 0, 0, IPTE_GLOBAL);
    pte = mk_pte_phys(phys, prot);
    set_pte(ptep, pte);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vmem_map_4k_page(addr: c_ulong, phys: c_ulong, prot: pgprot_t) -> c_int {
    int vmem_map_4k_page(unsigned long addr, unsigned long phys, pgprot_t prot)
    {
    int rc;
    mutex_lock(&vmem_mutex);
    rc = __vmem_map_4k_page(addr, phys, prot, true);
    mutex_unlock(&vmem_mutex);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn vmem_unmap_4k_page(addr: c_ulong) {
    void vmem_unmap_4k_page(unsigned long addr)
    {
    pte_t *ptep;
    mutex_lock(&vmem_mutex);
    ptep = virt_to_kpte(addr);
    __ptep_ipte(addr, ptep, 0, 0, IPTE_GLOBAL);
    pte_clear(&init_mm, addr, ptep);
    mutex_unlock(&vmem_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn vmem_map_init() -> void __init {
    void __init vmem_map_init(void)
    {
    __set_memory_rox(_stext, _etext);
    __set_memory_ro(_etext, __end_rodata);
    __set_memory_rox(__stext_amode31, __etext_amode31);
//
// If the BEAR-enhancement facility is not installed the first
// prefix page is used to return to the previous context with
// an LPSWE instruction and therefore must be executable.
//
    if (!cpu_has_bear())
    set_memory_x(0, 1);
    if (debug_pagealloc_enabled())
    __set_memory_4k(__va(0), absolute_pointer(__va(0)) + ident_map_size);
    pr_info("Write protected kernel read-only data: %luk\n",
    (unsigned long)(__end_rodata - _stext) >> 10);
    }
