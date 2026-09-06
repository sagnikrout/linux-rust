//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/hugetlbpage.c
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


//
// PPC Huge TLB Page Support for Kernel.
//
// Copyright (C) 2003 David Gibson, IBM Corporation.
// Copyright (C) 2011 Becky Bruce, Freescale Semiconductor
//
// Based on the IA-32 version:
// Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
//

    let mut hugetlb_disabled: bool = false;

    __builtin_ffs(sizeof(void *)))
    pte_t *huge_pte_offset(struct mm_struct *mm, unsigned long addr, unsigned long sz)
    {
//
// Only called for hugetlbfs pages, hence can ignore THP and the
// irq disabled walk.
//
    return __find_linux_pte(mm.pgd, addr, core::ptr::null_mut(), core::ptr::null_mut());
    }
    pte_t *huge_pte_alloc(struct mm_struct *mm, struct vm_area_struct *vma,
    unsigned long addr, unsigned long sz)
    {
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    addr &= ~(sz - 1);
    p4d = p4d_offset(pgd_offset(mm, addr), addr);
    if (!mm_pud_folded(mm) && sz >= P4D_SIZE)
    return (pte_t *)p4d;
    pud = pud_alloc(mm, p4d, addr);
    if (!pud)
    return core::ptr::null_mut();
    if (!mm_pmd_folded(mm) && sz >= PUD_SIZE)
    return (pte_t *)pud;
    pmd = pmd_alloc(mm, pud, addr);
    if (!pmd)
    return core::ptr::null_mut();
    if (sz >= PMD_SIZE) {
// On 8xx, all hugepages are handled as contiguous PTEs
    if (IS_ENABLED(CONFIG_PPC_8xx)) {
    int i;
    for (i = 0; i < sz / PMD_SIZE; i++) {
    if (!pte_alloc_huge(mm, pmd + i, addr))
    return core::ptr::null_mut();
    }
    }
    return (pte_t *)pmd;
    }
    return pte_alloc_huge(mm, pmd, addr);
    }

//
// Tracks gpages after the device tree is scanned and before the
// huge_boot_pages list is ready on pseries.
//
pub const MAX_NUMBER_GPAGES: c_int = 1024;
    __initdata static u64 gpage_freearray[MAX_NUMBER_GPAGES];
    __initdata static unsigned nr_gpages;
//
// Build list of addresses of gigantic pages.  This function is used in early
// boot before the buddy allocator is setup.
//
#[no_mangle]
pub unsafe extern "C" fn pseries_add_gpage(addr: u64, page_size: u64, number_of_pages: c_ulong) -> void __init {
    void __init pseries_add_gpage(u64 addr, u64 page_size, unsigned long number_of_pages)
    {
    if (!addr)
    return;
    while (number_of_pages > 0) {
    gpage_freearray[nr_gpages] = addr;
    nr_gpages++;
    number_of_pages--;
    addr += page_size;
    }
    }
    static __init void *pseries_alloc_bootmem_huge_page(struct hstate *hstate)
    {
    void *m;
    if (nr_gpages == 0)
    return core::ptr::null_mut();
    m = phys_to_virt(gpage_freearray[--nr_gpages]);
    gpage_freearray[nr_gpages] = 0;
    return m;
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_node_alloc_supported() -> bool __init {
    bool __init hugetlb_node_alloc_supported(void)
    {
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_alloc_bootmem_huge_page(h: *mut hstate, nid: c_int) -> *mut void __init {
    void *__init arch_alloc_bootmem_huge_page(struct hstate *h, int nid)
    {

    if (firmware_has_feature(FW_FEATURE_LPAR) && !radix_enabled())
    return pseries_alloc_bootmem_huge_page(h);

    return __alloc_bootmem_huge_page(h, nid);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_valid_size(size: c_ulong) -> bool __init {
    bool __init arch_hugetlb_valid_size(unsigned long size)
    {
    let mut shift: c_int = __ffs(size);
    int mmu_psize;
// Check that it is a page size supported by the hardware and
// that it fits within pagetable and slice limits.
    if (size <= PAGE_SIZE || !is_power_of_2(size))
    return false;
    mmu_psize = check_and_get_huge_psize(shift);
    if (mmu_psize < 0)
    return false;
    BUG_ON(mmu_psize_defs[mmu_psize].shift != shift);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn add_huge_page_size(size: c_ulonglong) -> int __init {
    static int __init add_huge_page_size(unsigned long long size)
    {
    let mut shift: c_int = __ffs(size);
    if (!arch_hugetlb_valid_size((unsigned long)size))
    return -EINVAL;
    hugetlb_add_hstate(shift - PAGE_SHIFT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hugetlbpage_init() -> int __init {
    static int __init hugetlbpage_init(void)
    {
    let mut configured: bool = false;
    int psize;
    if (hugetlb_disabled) {
    pr_info("HugeTLB support is disabled!\n");
    return 0;
    }
    if (IS_ENABLED(CONFIG_PPC_BOOK3S_64) && !radix_enabled() &&
    !mmu_has_feature(MMU_FTR_16M_PAGE))
    return -ENODEV;
    for (psize = 0; psize < MMU_PAGE_COUNT; ++psize) {
    unsigned shift;
    if (!mmu_psize_defs[psize].shift)
    continue;
    shift = mmu_psize_to_shift(psize);
    if (add_huge_page_size(1ULL << shift) < 0)
    continue;
    configured = true;
    }
    if (!configured)
    pr_info("Failed to initialize. Disabling HugeTLB");
    return 0;
    }
    arch_initcall(hugetlbpage_init);
#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_cma_order() -> unsigned int __init {
    unsigned int __init arch_hugetlb_cma_order(void)
    {
    if (radix_enabled())
    return PUD_SHIFT - PAGE_SHIFT;
#[no_mangle]
pub unsafe extern "C" fn if(mmu_psize_defs[MMU_PAGE_16G].shift: !firmware_has_feature(FW_FEATURE_LPAR) &&) -> else {
    else if (!firmware_has_feature(FW_FEATURE_LPAR) && mmu_psize_defs[MMU_PAGE_16G].shift)
//
// For pseries we do use ibm,expected#pages for reserving 16G pages.
//
    return mmu_psize_to_shift(MMU_PAGE_16G) - PAGE_SHIFT;
    return 0;
    }
