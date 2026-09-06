//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/mem.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
// PPC44x/36-bit changes by Matt Porter (mporter@mvista.com)
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

    unsigned long long memory_limit __initdata;
    pgprot_t __phys_mem_access_prot(unsigned long pfn, unsigned long size,
    pgprot_t vma_prot)
    {
    if (ppc_md.phys_mem_access_prot)
    return ppc_md.phys_mem_access_prot(pfn, size, vma_prot);
    if (!page_is_ram(pfn))
    vma_prot = pgprot_noncached(vma_prot);
    return vma_prot;
    }
    EXPORT_SYMBOL(__phys_mem_access_prot);

    static DEFINE_MUTEX(linear_mapping_mutex);

#[no_mangle]
pub unsafe extern "C" fn memory_add_physaddr_to_nid(start: u64) -> c_int {
    int memory_add_physaddr_to_nid(u64 start)
    {
    return hot_add_scn_to_nid(start);
    }
    EXPORT_SYMBOL_GPL(memory_add_physaddr_to_nid);

    int __weak create_section_mapping(unsigned long start, unsigned long end,
    int nid, pgprot_t prot)
    {
    return -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_section_mapping(start: c_ulong, end: c_ulong) -> int __weak {
    int __weak remove_section_mapping(unsigned long start, unsigned long end)
    {
    return -ENODEV;
    }
    int __ref arch_create_linear_mapping(int nid, u64 start, u64 size,
    struct mhp_params *params)
    {
    int rc;
    start = (unsigned long)__va(start);
    mutex_lock(&linear_mapping_mutex);
    rc = create_section_mapping(start, start + size, nid,
    params.pgprot);
    mutex_unlock(&linear_mapping_mutex);
    if (rc) {
    pr_warn("Unable to create linear mapping for 0x%llx..0x%llx: %d\n",
    start, start + size, rc);
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_remove_linear_mapping(start: u64, size: u64) -> void __ref {
    void __ref arch_remove_linear_mapping(u64 start, u64 size)
    {
    int ret;
// Remove htab bolted mappings for this section of memory
    start = (unsigned long)__va(start);
    mutex_lock(&linear_mapping_mutex);
    ret = remove_section_mapping(start, start + size);
    mutex_unlock(&linear_mapping_mutex);
    if (ret)
    pr_warn("Unable to remove linear mapping for 0x%llx..0x%llx: %d\n",
    start, start + size, ret);
// Ensure all vmalloc mappings are flushed in case they also
// hit that section of memory
//
    vm_unmap_aliases();
    }
//
// After memory hotplug the variables max_pfn, max_low_pfn and high_memory need
// updating.
//
#[no_mangle]
unsafe extern "C" fn update_end_of_memory_vars(start: u64, size: u64) {
    static void update_end_of_memory_vars(u64 start, u64 size)
    {
    let mut end_pfn: c_ulong = PFN_UP(start + size);
    if (end_pfn > max_pfn) {
    max_pfn = end_pfn;
    max_low_pfn = end_pfn;
    high_memory = (void *)__va(max_pfn * PAGE_SIZE - 1) + 1;
    }
    }
    int __ref add_pages(int nid, unsigned long start_pfn, unsigned long nr_pages,
    struct mhp_params *params)
    {
    int ret;
    ret = __add_pages(nid, start_pfn, nr_pages, params);
    if (ret)
    return ret;
// update max_pfn, max_low_pfn and high_memory
    update_end_of_memory_vars(start_pfn << PAGE_SHIFT,
    nr_pages << PAGE_SHIFT);
    return ret;
    }
    int __ref arch_add_memory(int nid, u64 start, u64 size,
    struct mhp_params *params)
    {
    let mut start_pfn: c_ulong = start >> PAGE_SHIFT;
    let mut nr_pages: c_ulong = size >> PAGE_SHIFT;
    int rc;
    rc = arch_create_linear_mapping(nid, start, size, params);
    if (rc)
    return rc;
    rc = add_pages(nid, start_pfn, nr_pages, params);
    if (rc)
    arch_remove_linear_mapping(start, size);
    return rc;
    }
    void __ref arch_remove_memory(u64 start, u64 size, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    let mut start_pfn: c_ulong = start >> PAGE_SHIFT;
    let mut nr_pages: c_ulong = size >> PAGE_SHIFT;
    __remove_pages(start_pfn, nr_pages, altmap, pgmap);
    arch_remove_linear_mapping(start, size);
    }

#[no_mangle]
pub unsafe extern "C" fn mem_topology_setup() -> void __init {
    void __init mem_topology_setup(void)
    {
    max_low_pfn = max_pfn = memblock_end_of_DRAM() >> PAGE_SHIFT;
    min_low_pfn = MEMORY_START >> PAGE_SHIFT;

    max_low_pfn = lowmem_end_addr >> PAGE_SHIFT;

// Place all memblock_regions in the same node and merge contiguous
// memblock_regions
//
    memblock_set_node(0, PHYS_ADDR_MAX, &memblock.memory, 0);
    }
// mark pages that don't exist as nosave
#[no_mangle]
unsafe extern "C" fn mark_nonram_nosave() -> int __init {
    static int __init mark_nonram_nosave(void)
    {
    unsigned long spfn, epfn, prev = 0;
    int i;
    for_each_mem_pfn_range(i, MAX_NUMNODES, &spfn, &epfn, core::ptr::null_mut()) {
    if (prev && prev < spfn)
    register_nosave_region(prev, spfn);
    prev = epfn;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn mark_nonram_nosave() -> int __init {
    static int __init mark_nonram_nosave(void)
    {
    return 0;
    }

//
// Zones usage:
//
// We setup ZONE_DMA to be 31-bits on all platforms and ZONE_NORMAL to be
// everything else. GFP_DMA32 page allocations automatically fall back to
// ZONE_DMA.
//
// By using 31-bit unconditionally, we can exploit zone_dma_limit to inform the
// generic DMA mapping code.  32-bit only devices (if not handled by an IOMMU
// anyway) will take a first dip into ZONE_NORMAL and get otherwise served by
// ZONE_DMA.
//
#[no_mangle]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) -> void __init {
    void __init arch_zone_limits_init(unsigned long *max_zone_pfns)
    {

    max_zone_pfns[ZONE_DMA] = min((zone_dma_limit >> PAGE_SHIFT) + 1, max_low_pfn);

    max_zone_pfns[ZONE_NORMAL] = max_low_pfn;

    max_zone_pfns[ZONE_HIGHMEM] = max_pfn;

    }
//
// paging_init() sets up the page tables - in fact we've already done this.
//
#[no_mangle]
pub unsafe extern "C" fn paging_init() -> void __init {
    void __init paging_init(void)
    {
    let mut total_ram: c_ulonglong = memblock_phys_mem_size();
    let mut top_of_ram: phys_addr_t = memblock_end_of_DRAM();
    int zone_dma_bits;

    let mut v: c_ulong = __fix_to_virt(FIX_KMAP_END);
    let mut end: c_ulong = __fix_to_virt(FIX_KMAP_BEGIN);
    for (; v < end; v += PAGE_SIZE)
    map_kernel_page(v, 0, __pgprot(0)); /* XXX gross */
    map_kernel_page(PKMAP_BASE, 0, __pgprot(0));	/* XXX gross */
    pkmap_page_table = virt_to_kpte(PKMAP_BASE);

    printk(KERN_DEBUG "Top of RAM: 0x%llx, Total RAM: 0x%llx\n",
    (unsigned long long)top_of_ram, total_ram);
    printk(KERN_DEBUG "Memory hole size: %ldMB\n",
    (long int)((top_of_ram - total_ram) >> 20));
//
// Allow 30-bit DMA for very limited Broadcom wifi chips on many
// powerbooks.
//
    if (IS_ENABLED(CONFIG_PPC32))
    zone_dma_bits = 30;
    else
    zone_dma_bits = 31;
    zone_dma_limit = DMA_BIT_MASK(zone_dma_bits);
    mark_nonram_nosave();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_mm_preinit() -> void __init {
    void __init arch_mm_preinit(void)
    {
//
// Reserve large chunks of memory for use by CMA for kdump, fadump, KVM
// and hugetlb. These must be called after pageblock_order is
// initialised.
//
    fadump_cma_init();
    kdump_cma_reserve();
    kvm_cma_reserve();
//
// book3s is limited to 16 page sizes due to encoding this in
// a 4-bit field for slices.
//
    BUILD_BUG_ON(MMU_PAGE_COUNT > 16);

//
// Some platforms (e.g. 85xx) limit DMA-able memory way below
// 4G. We force memblock to bottom-up mode to ensure that the
// memory allocated in swiotlb_init() is DMA-able.
// As it's the last memblock allocation, no need to reset it
// back to to-down.
//
    memblock_set_bottom_up(true);
    swiotlb_init(ppc_swiotlb_enable, ppc_swiotlb_flags);

    kasan_late_init();

//
// If smp is enabled, next_tlbcam_idx is initialized in the cpu up
// functions.... do it here for the non-smp case.
//
    per_cpu(next_tlbcam_idx, smp_processor_id()) =
    (mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY) - 1;

    }
#[no_mangle]
pub unsafe extern "C" fn free_initmem() {
    void free_initmem(void)
    {
    ppc_md.progress = ppc_printk_progress;
    mark_initmem_nx();
    free_initmem_default(POISON_FREE_INITMEM);
    ftrace_free_init_tramp();
    }
//
// System memory should not be in /proc/iomem but various tools expect it
// (eg kdump).
//
#[no_mangle]
unsafe extern "C" fn add_system_ram_resources() -> int __init {
    static int __init add_system_ram_resources(void)
    {
    phys_addr_t start, end;
    u64 i;
    for_each_mem_range(i, &start, &end) {
    struct resource *res;
    res = kzalloc_obj(struct resource);
    WARN_ON(!res);
    if (res) {
    res.name = "System RAM";
    res.start = start;
//
// In memblock, end points to the first byte after
// the range while in resourses, end points to the
// last byte in the range.
//
    res.end = end - 1;
    res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    WARN_ON(insert_resource(&iomem_resource, res) < 0);
    }
    }
    return 0;
    }
    subsys_initcall(add_system_ram_resources);

//
// devmem_is_allowed(): check to see if /dev/mem access to a certain address
// is valid. The argument is a physical page number.
//
// Access has to be given to non-kernel-ram areas as well, these contain the
// PCI mmio resources as well as potential bios/acpi data regions.
//
#[no_mangle]
pub unsafe extern "C" fn devmem_is_allowed(pfn: c_ulong) -> c_int {
    int devmem_is_allowed(unsigned long pfn)
    {
    if (page_is_rtas_user_buf(pfn))
    return 1;
    if (iomem_is_exclusive(PFN_PHYS(pfn)))
    return 0;
    if (!page_is_ram(pfn))
    return 1;
    return 0;
    }

    static struct execmem_info execmem_info __ro_after_init;

#[no_mangle]
unsafe extern "C" fn prealloc_execmem_pgtable() {
    static void prealloc_execmem_pgtable(void)
    {
    unsigned long va;
    for (va = ALIGN_DOWN(MODULES_VADDR, PGDIR_SIZE); va < MODULES_END; va += PGDIR_SIZE)
    pte_alloc_kernel(pmd_off_k(va), va);
    }

    static void prealloc_execmem_pgtable(void) { }

    struct execmem_info __init *execmem_arch_setup(void)
    {
    let mut kprobes_prot: pgprot_t = strict_module_rwx_enabled() ? PAGE_KERNEL_ROX : PAGE_KERNEL_EXEC;
    let mut prot: pgprot_t = strict_module_rwx_enabled() ? PAGE_KERNEL : PAGE_KERNEL_EXEC;
    let mut fallback_start: c_ulong = 0, fallback_end = 0;
    unsigned long start, end;
//
// BOOK3S_32 and 8xx define MODULES_VADDR for text allocations and
// allow allocating data in the entire vmalloc space
//

    let mut limit: c_ulong = (unsigned long)_etext - SZ_32M;
// First try within 32M limit from _etext to avoid branch trampolines
    if (MODULES_VADDR < PAGE_OFFSET && MODULES_END > limit) {
    start = limit;
    fallback_start = MODULES_VADDR;
    fallback_end = MODULES_END;
    } else {
    start = MODULES_VADDR;
    }
    end = MODULES_END;

    start = VMALLOC_START;
    end = VMALLOC_END;

    prealloc_execmem_pgtable();
    execmem_info = (struct execmem_info){
    .ranges = {
    [EXECMEM_DEFAULT] = {
    .start	= start,
    .end	= end,
    .pgprot	= prot,
    .alignment = 1,
    .fallback_start	= fallback_start,
    .fallback_end	= fallback_end,
    },
    [EXECMEM_KPROBES] = {
    .start	= VMALLOC_START,
    .end	= VMALLOC_END,
    .pgprot	= kprobes_prot,
    .alignment = 1,
    },
    [EXECMEM_MODULE_DATA] = {
    .start	= VMALLOC_START,
    .end	= VMALLOC_END,
    .pgprot	= PAGE_KERNEL,
    .alignment = 1,
    },
    },
    };
    return &execmem_info;
    }
