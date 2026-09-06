//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/init_32.c
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
// Copyright (C) 1995  Linus Torvalds
//
// Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
//

    unsigned long highstart_pfn, highend_pfn;
    let mut __vmalloc_start_set: bool __read_mostly = false;
//
// Creates a middle page table and puts a pointer to it in the
// given global directory entry. This only returns the gd entry
// in non-PAE compilation mode, since the middle layer is folded.
//
#[no_mangle]
unsafe extern "C" fn one_md_table_init(pgd: *mut pgd_t) -> *mut pmd_t  __init {
    static pmd_t * __init one_md_table_init(pgd_t *pgd)
    {
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd_table;

    if (!(pgd_val(*pgd) & _PAGE_PRESENT)) {
    pmd_table = (pmd_t *)alloc_low_page();
    set_pgd(pgd, __pgd(__pa(pmd_table) | _PAGE_PRESENT));
    p4d = p4d_offset(pgd, 0);
    pud = pud_offset(p4d, 0);
    BUG_ON(pmd_table != pmd_offset(pud, 0));
    return pmd_table;
    }

    p4d = p4d_offset(pgd, 0);
    pud = pud_offset(p4d, 0);
    pmd_table = pmd_offset(pud, 0);
    return pmd_table;
    }
//
// Create a page table and place a pointer to it in a middle page
// directory entry:
//
#[no_mangle]
unsafe extern "C" fn one_page_table_init(pmd: *mut pmd_t) -> *mut pte_t  __init {
    static pte_t * __init one_page_table_init(pmd_t *pmd)
    {
    if (!(pmd_val(*pmd) & _PAGE_PRESENT)) {
    pte_t *page_table = (pte_t *)alloc_low_page();
    set_pmd(pmd, __pmd(__pa(page_table) | _PAGE_TABLE));
    BUG_ON(page_table != pte_offset_kernel(pmd, 0));
    }
    return pte_offset_kernel(pmd, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn populate_extra_pmd(vaddr: c_ulong) -> *mut pmd_t  __init {
    pmd_t * __init populate_extra_pmd(unsigned long vaddr)
    {
    let mut pgd_idx: c_int = pgd_index(vaddr);
    let mut pmd_idx: c_int = pmd_index(vaddr);
    return one_md_table_init(swapper_pg_dir + pgd_idx) + pmd_idx;
    }
#[no_mangle]
pub unsafe extern "C" fn populate_extra_pte(vaddr: c_ulong) -> *mut pte_t  __init {
    pte_t * __init populate_extra_pte(unsigned long vaddr)
    {
    let mut pte_idx: c_int = pte_index(vaddr);
    pmd_t *pmd;
    pmd = populate_extra_pmd(vaddr);
    return one_page_table_init(pmd) + pte_idx;
    }
    static unsigned long __init
    page_table_range_init_count(unsigned long start, unsigned long end)
    {
    let mut count: c_ulong = 0;

    let mut pmd_idx_kmap_begin: c_int = fix_to_virt(FIX_KMAP_END) >> PMD_SHIFT;
    let mut pmd_idx_kmap_end: c_int = fix_to_virt(FIX_KMAP_BEGIN) >> PMD_SHIFT;
    int pgd_idx, pmd_idx;
    unsigned long vaddr;
    if (pmd_idx_kmap_begin == pmd_idx_kmap_end)
    return 0;
    vaddr = start;
    pgd_idx = pgd_index(vaddr);
    pmd_idx = pmd_index(vaddr);
    for ( ; (pgd_idx < PTRS_PER_PGD) && (vaddr != end); pgd_idx++) {
    for (; (pmd_idx < PTRS_PER_PMD) && (vaddr != end);
    pmd_idx++) {
    if ((vaddr >> PMD_SHIFT) >= pmd_idx_kmap_begin &&
    (vaddr >> PMD_SHIFT) <= pmd_idx_kmap_end)
    count++;
    vaddr += PMD_SIZE;
    }
    pmd_idx = 0;
    }

    return count;
    }
    static pte_t *__init page_table_kmap_check(pte_t *pte, pmd_t *pmd,
    unsigned long vaddr, pte_t *lastpte,
    void **adr)
    {

//
// Something (early fixmap) may already have put a pte
// page here, which causes the page table allocation
// to become nonlinear. Attempt to fix it, and if it
// is still nonlinear then we have to bug.
//
    let mut pmd_idx_kmap_begin: c_int = fix_to_virt(FIX_KMAP_END) >> PMD_SHIFT;
    let mut pmd_idx_kmap_end: c_int = fix_to_virt(FIX_KMAP_BEGIN) >> PMD_SHIFT;
    if (pmd_idx_kmap_begin != pmd_idx_kmap_end
    && (vaddr >> PMD_SHIFT) >= pmd_idx_kmap_begin
    && (vaddr >> PMD_SHIFT) <= pmd_idx_kmap_end) {
    pte_t *newpte;
    int i;
    BUG_ON(after_bootmem);
    newpte = *adr;
    for (i = 0; i < PTRS_PER_PTE; i++)
    set_pte(newpte + i, pte[i]);
// adr = (void *)(((unsigned long)(*adr)) + PAGE_SIZE);
    set_pmd(pmd, __pmd(__pa(newpte)|_PAGE_TABLE));
    BUG_ON(newpte != pte_offset_kernel(pmd, 0));
    __flush_tlb_all();
    pte = newpte;
    }
    BUG_ON(vaddr < fix_to_virt(FIX_KMAP_BEGIN - 1)
    && vaddr > fix_to_virt(FIX_KMAP_END)
    && lastpte && lastpte + PTRS_PER_PTE != pte);

    return pte;
    }
//
// This function initializes a certain range of kernel virtual memory
// with new bootmem page tables, everywhere page tables are missing in
// the given range.
//
// NOTE: The pagetables are allocated contiguous on the physical space
// so we can cache the place of the first one and move around without
// checking the pgd every time.
//
    static void __init
    page_table_range_init(unsigned long start, unsigned long end, pgd_t *pgd_base)
    {
    int pgd_idx, pmd_idx;
    unsigned long vaddr;
    pgd_t *pgd;
    pmd_t *pmd;
    pte_t *pte = core::ptr::null_mut();
    let mut count: c_ulong = page_table_range_init_count(start, end);
    void *adr = core::ptr::null_mut();
    if (count)
    adr = alloc_low_pages(count);
    vaddr = start;
    pgd_idx = pgd_index(vaddr);
    pmd_idx = pmd_index(vaddr);
    pgd = pgd_base + pgd_idx;
    for ( ; (pgd_idx < PTRS_PER_PGD) && (vaddr != end); pgd++, pgd_idx++) {
    pmd = one_md_table_init(pgd);
    pmd = pmd + pmd_index(vaddr);
    for (; (pmd_idx < PTRS_PER_PMD) && (vaddr != end);
    pmd++, pmd_idx++) {
    pte = page_table_kmap_check(one_page_table_init(pmd),
    pmd, vaddr, pte, &adr);
    vaddr += PMD_SIZE;
    }
    pmd_idx = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn is_x86_32_kernel_text(addr: c_ulong) -> c_int {
    static inline int is_x86_32_kernel_text(unsigned long addr)
    {
    if (addr >= (unsigned long)_text && addr <= (unsigned long)__init_end)
    return 1;
    return 0;
    }
//
// This maps the physical memory to kernel virtual address space, a total
// of max_low_pfn pages, by creating page tables starting from address
// PAGE_OFFSET:
//
    unsigned long __init
    kernel_physical_mapping_init(unsigned long start,
    unsigned long end,
    unsigned long page_size_mask,
    pgprot_t prot)
    {
    let mut use_pse: c_int = page_size_mask == (1<<PG_LEVEL_2M);
    let mut last_map_addr: c_ulong = end;
    unsigned long start_pfn, end_pfn;
    pgd_t *pgd_base = swapper_pg_dir;
    int pgd_idx, pmd_idx, pte_ofs;
    unsigned long pfn;
    pgd_t *pgd;
    pmd_t *pmd;
    pte_t *pte;
    unsigned pages_2m, pages_4k;
    int mapping_iter;
    start_pfn = start >> PAGE_SHIFT;
    end_pfn = end >> PAGE_SHIFT;
//
// First iteration will setup identity mapping using large/small pages
// based on use_pse, with other attributes same as set by
// the early code in head_32.S
//
// Second iteration will setup the appropriate attributes (NX, GLOBAL..)
// as desired for the kernel identity mapping.
//
// This two pass mechanism conforms to the TLB app note which says:
//
// "Software should not write to a paging-structure entry in a way
// that would change, for any linear address, both the page size
// and either the page frame or attributes."
//
    mapping_iter = 1;
    if (!boot_cpu_has(X86_FEATURE_PSE))
    use_pse = 0;
    repeat:
    pages_2m = pages_4k = 0;
    pfn = start_pfn;
    pgd_idx = pgd_index((pfn<<PAGE_SHIFT) + PAGE_OFFSET);
    pgd = pgd_base + pgd_idx;
    for (; pgd_idx < PTRS_PER_PGD; pgd++, pgd_idx++) {
    pmd = one_md_table_init(pgd);
    if (pfn >= end_pfn)
    continue;

    pmd_idx = pmd_index((pfn<<PAGE_SHIFT) + PAGE_OFFSET);
    pmd += pmd_idx;

    pmd_idx = 0;

    for (; pmd_idx < PTRS_PER_PMD && pfn < end_pfn;
    pmd++, pmd_idx++) {
    let mut addr: c_uint = pfn * PAGE_SIZE + PAGE_OFFSET;
//
// Map with big pages if possible, otherwise
// create normal page tables:
//
    if (use_pse) {
    unsigned int addr2;
    let mut prot: pgprot_t = PAGE_KERNEL_LARGE;
//
// first pass will use the same initial
// identity mapping attribute + _PAGE_PSE.
//
    pgprot_t init_prot =
    __pgprot(PTE_IDENT_ATTR |
    _PAGE_PSE);
    pfn &= PMD_MASK >> PAGE_SHIFT;
    addr2 = (pfn + PTRS_PER_PTE-1) * PAGE_SIZE +
    PAGE_OFFSET + PAGE_SIZE-1;
    if (is_x86_32_kernel_text(addr) ||
    is_x86_32_kernel_text(addr2))
    prot = PAGE_KERNEL_LARGE_EXEC;
    pages_2m++;
    if (mapping_iter == 1)
    set_pmd(pmd, pfn_pmd(pfn, init_prot));
    else
    set_pmd(pmd, pfn_pmd(pfn, prot));
    pfn += PTRS_PER_PTE;
    continue;
    }
    pte = one_page_table_init(pmd);
    pte_ofs = pte_index((pfn<<PAGE_SHIFT) + PAGE_OFFSET);
    pte += pte_ofs;
    for (; pte_ofs < PTRS_PER_PTE && pfn < end_pfn;
    pte++, pfn++, pte_ofs++, addr += PAGE_SIZE) {
    let mut prot: pgprot_t = PAGE_KERNEL;
//
// first pass will use the same initial
// identity mapping attribute.
//
    let mut init_prot: pgprot_t = __pgprot(PTE_IDENT_ATTR);
    if (is_x86_32_kernel_text(addr))
    prot = PAGE_KERNEL_EXEC;
    pages_4k++;
    if (mapping_iter == 1) {
    set_pte(pte, pfn_pte(pfn, init_prot));
    last_map_addr = (pfn << PAGE_SHIFT) + PAGE_SIZE;
    } else
    set_pte(pte, pfn_pte(pfn, prot));
    }
    }
    }
    if (mapping_iter == 1) {
//
// update direct mapping page count only in the first
// iteration.
//
    update_page_count(PG_LEVEL_2M, pages_2m);
    update_page_count(PG_LEVEL_4K, pages_4k);
//
// local global flush tlb, which will flush the previous
// mappings present in both small and large page TLB's.
//
    __flush_tlb_all();
//
// Second iteration will set the actual desired PTE attributes.
//
    mapping_iter = 2;
    goto repeat;
    }
    return last_map_addr;
    }

#[no_mangle]
unsafe extern "C" fn permanent_kmaps_init(pgd_base: *mut pgd_t) -> void __init {
    static void __init permanent_kmaps_init(pgd_t *pgd_base)
    {
    let mut vaddr: c_ulong = PKMAP_BASE;
    page_table_range_init(vaddr, vaddr + PAGE_SIZE*LAST_PKMAP, pgd_base);
    pkmap_page_table = virt_to_kpte(vaddr);
    }

#[no_mangle]
pub unsafe extern "C" fn permanent_kmaps_init(pgd_base: *mut pgd_t) {
    static inline void permanent_kmaps_init(pgd_t *pgd_base)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn sync_initial_page_table() -> void __init {
    void __init sync_initial_page_table(void)
    {
    clone_pgd_range(initial_page_table + KERNEL_PGD_BOUNDARY,
    swapper_pg_dir     + KERNEL_PGD_BOUNDARY,
    KERNEL_PGD_PTRS);
//
// sync back low identity map too.  It is used for example
// in the 32-bit EFI stub.
//
    clone_pgd_range(initial_page_table,
    swapper_pg_dir     + KERNEL_PGD_BOUNDARY,
    min(KERNEL_PGD_PTRS, KERNEL_PGD_BOUNDARY));
    }
#[no_mangle]
pub unsafe extern "C" fn native_pagetable_init() -> void __init {
    void __init native_pagetable_init(void)
    {
    unsigned long pfn, va;
    pgd_t *pgd, *base = swapper_pg_dir;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
//
// Remove any mappings which extend past the end of physical
// memory from the boot time page table.
// In virtual address space, we should have at least two pages
// from VMALLOC_END to pkmap or fixmap according to VMALLOC_END
// definition. And max_low_pfn is set to VMALLOC_END physical
// address. If initial memory mapping is doing right job, we
// should have pte used near max_low_pfn or one pmd is not present.
//
    for (pfn = max_low_pfn; pfn < 1<<(32-PAGE_SHIFT); pfn++) {
    va = PAGE_OFFSET + (pfn<<PAGE_SHIFT);
    pgd = base + pgd_index(va);
    if (!pgd_present(*pgd))
    break;
    p4d = p4d_offset(pgd, va);
    pud = pud_offset(p4d, va);
    pmd = pmd_offset(pud, va);
    if (!pmd_present(*pmd))
    break;
// should not be large page here
    if (pmd_leaf(*pmd)) {
    pr_warn("try to clear pte for ram above max_low_pfn: pfn: %lx pmd: %p pmd phys: %lx, but pmd is big page and is not using pte !\n",
    pfn, pmd, __pa(pmd));
    BUG_ON(1);
    }
    pte = pte_offset_kernel(pmd, va);
    if (!pte_present(*pte))
    break;
    printk(KERN_DEBUG "clearing pte for ram above max_low_pfn: pfn: %lx pmd: %p pmd phys: %lx pte: %p pte phys: %lx\n",
    pfn, pmd, __pa(pmd), pte, __pa(pte));
    pte_clear(core::ptr::null_mut(), va, pte);
    }
    paging_init();
    }
//
// Build a proper pagetable for the kernel mappings.  Up until this
// point, we've been running on some set of pagetables constructed by
// the boot process.
//
// This will be a pagetable constructed in arch/x86/kernel/head_32.S.
// The root of the pagetable will be swapper_pg_dir.
//
// In general, pagetable_init() assumes that the pagetable may already
// be partially populated, and so it avoids stomping on any existing
// mappings.
//
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_page_table_range_init() -> void __init {
    void __init early_ioremap_page_table_range_init(void)
    {
    pgd_t *pgd_base = swapper_pg_dir;
    unsigned long vaddr, end;
//
// Fixed mappings, only the page table structure has to be
// created - mappings will be set by set_fixmap():
//
    vaddr = __fix_to_virt(__end_of_fixed_addresses - 1) & PMD_MASK;
    end = (FIXADDR_TOP + PMD_SIZE - 1) & PMD_MASK;
    page_table_range_init(vaddr, end, pgd_base);
    early_ioremap_reset();
    }
#[no_mangle]
unsafe extern "C" fn pagetable_init() -> void __init {
    static void __init pagetable_init(void)
    {
    pgd_t *pgd_base = swapper_pg_dir;
    permanent_kmaps_init(pgd_base);
    }

// Bits supported by the hardware:
    let mut __read_mostly: pteval_t __supported_pte_mask = DEFAULT_PTE_MASK;
// Bits allowed in normal kernel mappings:
    let mut __read_mostly: pteval_t __default_kernel_pte_mask = DEFAULT_PTE_MASK;
    EXPORT_SYMBOL_GPL(__supported_pte_mask);
// Used in PAGE_KERNEL_* macros which are reasonably used out-of-tree:
    EXPORT_SYMBOL(__default_kernel_pte_mask);
// user-defined highmem size
    let mut highmem_pages: static unsigned int = -1;
//
// highmem=size forces highmem to be exactly 'size' bytes.
// This works even on boxes that have no highmem otherwise.
// This also works to reduce highmem size on bigger boxes.
//
#[no_mangle]
unsafe extern "C" fn parse_highmem(arg: *mut c_char) -> int __init {
    static int __init parse_highmem(char *arg)
    {
    if (!arg)
    return -EINVAL;
    highmem_pages = memparse(arg, &arg) >> PAGE_SHIFT;
    return 0;
    }
    early_param("highmem", parse_highmem);

    "highmem size (%luMB) is bigger than pages available (%luMB)!\n"

    "highmem size (%luMB) results in <64MB lowmem, ignoring it!\n"
//
// All of RAM fits into lowmem - but if user wants highmem
// artificially via the highmem=x boot parameter then create
// it:
//
#[no_mangle]
unsafe extern "C" fn lowmem_pfn_init() -> void __init {
    static void __init lowmem_pfn_init(void)
    {
// max_low_pfn is 0, we already have early_res support
    max_low_pfn = max_pfn;
    if (highmem_pages == -1)
    highmem_pages = 0;

    if (highmem_pages >= max_pfn) {
    printk(KERN_ERR MSG_HIGHMEM_TOO_BIG,
    pages_to_mb(highmem_pages), pages_to_mb(max_pfn));
    highmem_pages = 0;
    }
    if (highmem_pages) {
    if (max_low_pfn - highmem_pages < 64*1024*1024/PAGE_SIZE) {
    printk(KERN_ERR MSG_LOWMEM_TOO_SMALL,
    pages_to_mb(highmem_pages));
    highmem_pages = 0;
    }
    max_low_pfn -= highmem_pages;
    }

    if (highmem_pages)
    printk(KERN_ERR "ignoring highmem size on non-highmem kernel!\n");

    }

    "only %luMB highmem pages available, ignoring highmem size of %luMB!\n"

    "Warning: only 4GB will be used. Support for CONFIG_HIGHMEM64G was removed!\n"
//
// We have more RAM than fits into lowmem - we try to put it into
// highmem, also taking the highmem=x boot parameter into account:
//
#[no_mangle]
unsafe extern "C" fn highmem_pfn_init() -> void __init {
    static void __init highmem_pfn_init(void)
    {
    max_low_pfn = MAXMEM_PFN;
    if (highmem_pages == -1)
    highmem_pages = max_pfn - MAXMEM_PFN;
    if (highmem_pages + MAXMEM_PFN < max_pfn)
    max_pfn = MAXMEM_PFN + highmem_pages;
    if (highmem_pages + MAXMEM_PFN > max_pfn) {
    printk(KERN_WARNING MSG_HIGHMEM_TOO_SMALL,
    pages_to_mb(max_pfn - MAXMEM_PFN),
    pages_to_mb(highmem_pages));
    highmem_pages = 0;
    }

// Maximum memory usable is what is directly addressable
    printk(KERN_WARNING "Warning only %ldMB will be used.\n", MAXMEM>>20);
    printk(KERN_WARNING "Use a HIGHMEM enabled kernel.\n");
    max_pfn = MAXMEM_PFN;

    if (max_pfn > MAX_NONPAE_PFN) {
    max_pfn = MAX_NONPAE_PFN;
    printk(KERN_WARNING MSG_HIGHMEM_TRIMMED);
    }

    }
//
// Determine low and high memory ranges:
//
#[no_mangle]
pub unsafe extern "C" fn find_low_pfn_range() -> void __init {
    void __init find_low_pfn_range(void)
    {
// it could update max_pfn
    if (max_pfn <= MAXMEM_PFN)
    lowmem_pfn_init();
    else
    highmem_pfn_init();
    }
#[no_mangle]
pub unsafe extern "C" fn initmem_init() -> void __init {
    void __init initmem_init(void)
    {

    highstart_pfn = highend_pfn = max_pfn;
    if (max_pfn > max_low_pfn)
    highstart_pfn = max_low_pfn;
    printk(KERN_NOTICE "%ldMB HIGHMEM available.\n",
    pages_to_mb(highend_pfn - highstart_pfn));
    high_memory = (void *) __va(highstart_pfn * PAGE_SIZE - 1) + 1;

    high_memory = (void *) __va(max_low_pfn * PAGE_SIZE - 1) + 1;

    memblock_set_node(0, PHYS_ADDR_MAX, &memblock.memory, 0);
    __vmalloc_start_set = true;
    printk(KERN_NOTICE "%ldMB LOWMEM available.\n",
    pages_to_mb(max_low_pfn));
    printk(KERN_INFO "  mapped low ram: 0 - %08lx\n",
    max_pfn_mapped<<PAGE_SHIFT);
    printk(KERN_INFO "  low ram: 0 - %08lx\n", max_low_pfn<<PAGE_SHIFT);
    }
//
// paging_init() sets up the page tables - note that the first 8MB are
// already mapped by head.S.
//
// This routines also unmaps the page at virtual kernel address 0, so
// that we can trap those pesky NULL-reference errors in the kernel.
//
#[no_mangle]
pub unsafe extern "C" fn paging_init() -> void __init {
    void __init paging_init(void)
    {
    pagetable_init();
    __flush_tlb_all();
//
// NOTE: at this point the bootmem allocator is fully available.
//
    olpc_dt_build_devicetree();
    }
//
// Test if the WP bit works in supervisor mode. It isn't supported on 386's
// and also on some strange 486's. All 586+'s are OK. This used to involve
// black magic jumps to work around some nasty CPU bugs, but fortunately the
// switch to using exceptions got rid of all that.
//
#[no_mangle]
unsafe extern "C" fn test_wp_bit() -> void __init {
    static void __init test_wp_bit(void)
    {
    let mut z: c_char = 0;
    printk(KERN_INFO "Checking if this processor honours the WP bit even in supervisor mode...");
    __set_fixmap(FIX_WP_TEST, __pa_symbol(empty_zero_page), PAGE_KERNEL_RO);
    if (copy_to_kernel_nofault((char *)fix_to_virt(FIX_WP_TEST), &z, 1)) {
    clear_fixmap(FIX_WP_TEST);
    printk(KERN_CONT "Ok.\n");
    return;
    }
    printk(KERN_CONT "No.\n");
    panic("Linux doesn't support CPUs with broken WP.");
    }
#[no_mangle]
pub unsafe extern "C" fn arch_mm_preinit() -> void __init {
    void __init arch_mm_preinit(void)
    {
    pci_iommu_alloc();

    BUG_ON(!mem_map);

    }
#[no_mangle]
pub unsafe extern "C" fn mem_init() -> void __init {
    void __init mem_init(void)
    {
    after_bootmem = 1;
    x86_init.hyper.init_after_bootmem();
//
// Check boundaries twice: Some fundamental inconsistencies can
// be detected at build time already.
//

    BUILD_BUG_ON(PKMAP_BASE + LAST_PKMAP*PAGE_SIZE	> FIXADDR_START);
    BUILD_BUG_ON(VMALLOC_END			> PKMAP_BASE);

    BUILD_BUG_ON(VMALLOC_START			>= VMALLOC_END);

    BUG_ON(PKMAP_BASE + LAST_PKMAP*PAGE_SIZE	> FIXADDR_START);
    BUG_ON(VMALLOC_END				> PKMAP_BASE);

    BUG_ON(VMALLOC_START				>= VMALLOC_END);
    BUG_ON((unsigned long)high_memory		> VMALLOC_START);
    test_wp_bit();
    }
    int kernel_set_to_readonly __read_mostly;
#[no_mangle]
unsafe extern "C" fn mark_nxdata_nx() {
    static void mark_nxdata_nx(void)
    {
//
// When this called, init has already been executed and released,
// so everything past _etext should be NX.
//
    let mut start: c_ulong = PFN_ALIGN(_etext);
//
// This comes from is_x86_32_kernel_text upper limit. Also HPAGE where used:
//
    let mut size: c_ulong = (((unsigned long)__init_end + HPAGE_SIZE) & HPAGE_MASK) - start;
    if (__supported_pte_mask & _PAGE_NX)
    printk(KERN_INFO "NX-protecting the kernel data: %luk\n", size >> 10);
    set_memory_nx(start, size >> PAGE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    void mark_rodata_ro(void)
    {
    let mut start: c_ulong = PFN_ALIGN(_text);
    let mut size: c_ulong = (unsigned long)__end_rodata - start;
    set_pages_ro(virt_to_page(start), size >> PAGE_SHIFT);
    pr_info("Write protecting kernel text and read-only data: %luk\n",
    size >> 10);
    kernel_set_to_readonly = 1;

    pr_info("Testing CPA: Reverting %lx-%lx\n", start, start + size);
    set_pages_rw(virt_to_page(start), size >> PAGE_SHIFT);
    pr_info("Testing CPA: write protecting again\n");
    set_pages_ro(virt_to_page(start), size >> PAGE_SHIFT);

    mark_nxdata_nx();
    }
