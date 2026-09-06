//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/fixmap.c
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
// Fixmap manipulation code
//

// ensure that the fixmap region does not grow down into the PCI I/O region
    static_assert(FIXADDR_TOT_START > PCI_IO_END);

    SPAN_NR_ENTRIES(FIXADDR_TOT_START, FIXADDR_TOP, PMD_SHIFT)

    SPAN_NR_ENTRIES(FIXADDR_TOT_START, FIXADDR_TOP, PUD_SHIFT)
    static_assert(NR_BM_PMD_TABLES == 1);

    (((addr) >> (shift)) - (FIXADDR_TOT_START >> (shift)))

    static pte_t bm_pte[NR_BM_PTE_TABLES][PTRS_PER_PTE] __bss_pgtbl;
    static pmd_t bm_pmd[PTRS_PER_PMD] __bss_pgtbl __maybe_unused;
    static pud_t bm_pud[PTRS_PER_PUD] __bss_pgtbl __maybe_unused;
    static inline pte_t *fixmap_pte(unsigned long addr)
    {
    return &bm_pte[BM_PTE_TABLE_IDX(addr)][pte_index(addr)];
    }
#[no_mangle]
unsafe extern "C" fn early_fixmap_init_pte(pmdp: *mut pmd_t, addr: c_ulong) -> void __init {
    static void __init early_fixmap_init_pte(pmd_t *pmdp, unsigned long addr)
    {
    let mut pmd: pmd_t = READ_ONCE(*pmdp);
    pte_t *ptep;
    if (pmd_none(pmd)) {
    ptep = bm_pte[BM_PTE_TABLE_IDX(addr)];
    __pmd_populate(pmdp, __pa_symbol(ptep),
    PMD_TYPE_TABLE | PMD_TABLE_AF);
    }
    }
    static void __init early_fixmap_init_pmd(pud_t *pudp, unsigned long addr,
    unsigned long end)
    {
    unsigned long next;
    let mut pud: pud_t = READ_ONCE(*pudp);
    pmd_t *pmdp;
    if (pud_none(pud))
    __pud_populate(pudp, __pa_symbol(bm_pmd),
    PUD_TYPE_TABLE | PUD_TABLE_AF);
    pmdp = pmd_offset_kimg(pudp, addr);
    do {
    next = pmd_addr_end(addr, end);
    early_fixmap_init_pte(pmdp, addr);
    } while (pmdp++, addr = next, addr != end);
    }
    static void __init early_fixmap_init_pud(p4d_t *p4dp, unsigned long addr,
    unsigned long end)
    {
    let mut p4d: p4d_t = READ_ONCE(*p4dp);
    pud_t *pudp;
    if (CONFIG_PGTABLE_LEVELS > 3 && !p4d_none(p4d) &&
    p4d_page_paddr(p4d) != __pa_symbol(bm_pud)) {
//
// We only end up here if the kernel mapping and the fixmap
// share the top level pgd entry, which should only happen on
// 16k/4 levels configurations.
//
    BUG_ON(!IS_ENABLED(CONFIG_ARM64_16K_PAGES));
    }
    if (p4d_none(p4d))
    __p4d_populate(p4dp, __pa_symbol(bm_pud),
    P4D_TYPE_TABLE | P4D_TABLE_AF);
    pudp = pud_offset_kimg(p4dp, addr);
    early_fixmap_init_pmd(pudp, addr, end);
    }
//
// The p*d_populate functions call virt_to_phys implicitly so they can't be used
// directly on kernel symbols (bm_p*d). This function is called too early to use
// lm_alias so __p*d_populate functions must be used to populate with the
// physical address from __pa_symbol.
//
#[no_mangle]
pub unsafe extern "C" fn early_fixmap_init() -> void __init {
    void __init early_fixmap_init(void)
    {
    let mut addr: c_ulong = FIXADDR_TOT_START;
    let mut end: c_ulong = FIXADDR_TOP;
    pgd_t *pgdp = pgd_offset_k(addr);
    p4d_t *p4dp = p4d_offset_kimg(pgdp, addr);
    early_fixmap_init_pud(p4dp, addr, end);
    }
//
// Unusually, this is also called in IRQ context (ghes_iounmap_irq) so if we
// ever need to use IPIs for TLB broadcasting, then we're in trouble here.
//
    void __set_fixmap(enum fixed_addresses idx,
    phys_addr_t phys, pgprot_t flags)
    {
    let mut addr: c_ulong = __fix_to_virt(idx);
    pte_t *ptep;
    BUG_ON(idx <= FIX_HOLE || idx >= __end_of_fixed_addresses);
    ptep = fixmap_pte(addr);
    if (pgprot_val(flags)) {
    __set_pte(ptep, pfn_pte(phys >> PAGE_SHIFT, flags));
    } else {
    __pte_clear(&init_mm, addr, ptep);
    flush_tlb_kernel_range(addr, addr+PAGE_SIZE);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fixmap_remap_fdt(dt_phys: phys_addr_t, size: *mut c_int, prot: pgprot_t) -> *mut void __init {
    void *__init fixmap_remap_fdt(phys_addr_t dt_phys, int *size, pgprot_t prot)
    {
    let mut dt_virt_base: u64 = __fix_to_virt(FIX_FDT);
    phys_addr_t dt_phys_base;
    int offset;
    void *dt_virt;
//
// Check whether the physical FDT address is set and meets the minimum
// alignment requirement. Since we are relying on MIN_FDT_ALIGN to be
// at least 8 bytes so that we can always access the magic and size
// fields of the FDT header after mapping the first chunk, double check
// here if that is indeed the case.
//
    BUILD_BUG_ON(MIN_FDT_ALIGN < 8);
    if (!dt_phys || dt_phys % MIN_FDT_ALIGN)
    return core::ptr::null_mut();
    dt_phys_base = round_down(dt_phys, PAGE_SIZE);
    offset = dt_phys % PAGE_SIZE;
    dt_virt = (void *)dt_virt_base + offset;
// map the first chunk so we can read the size from the header
    create_mapping_noalloc(dt_phys_base, dt_virt_base, PAGE_SIZE, prot);
    if (fdt_magic(dt_virt) != FDT_MAGIC)
    return core::ptr::null_mut();
// size = fdt_totalsize(dt_virt);
    if (*size > MAX_FDT_SIZE)
    return core::ptr::null_mut();
    if (offset + *size > PAGE_SIZE) {
    create_mapping_noalloc(dt_phys_base, dt_virt_base,
    offset + *size, prot);
    }
    return dt_virt;
    }
