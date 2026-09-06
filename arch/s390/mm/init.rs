//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/init.c
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
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Hartmut Penner (hp@de.ibm.com)
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1995  Linus Torvalds
//

    pgd_t swapper_pg_dir[PTRS_PER_PGD] __section(".bss..swapper_pg_dir");
    pgd_t invalid_pg_dir[PTRS_PER_PGD] __section(".bss..invalid_pg_dir");
    struct ctlreg __bootdata_preserved(s390_invalid_asce);
    unsigned long __bootdata_preserved(page_noexec_mask);
    EXPORT_SYMBOL(page_noexec_mask);
    unsigned long __bootdata_preserved(segment_noexec_mask);
    EXPORT_SYMBOL(segment_noexec_mask);
    unsigned long __bootdata_preserved(region_noexec_mask);
    EXPORT_SYMBOL(region_noexec_mask);
    unsigned long empty_zero_page, zero_page_mask;
    EXPORT_SYMBOL(empty_zero_page);
    EXPORT_SYMBOL(zero_page_mask);
#[no_mangle]
pub unsafe extern "C" fn arch_setup_zero_pages() -> void __init {
    void __init arch_setup_zero_pages(void)
    {
    let mut total_pages: c_ulong = memblock_estimated_nr_free_pages();
    unsigned int order;
// Latest machines require a mapping granularity of 512KB
    order = 7;
// Limit number of empty zero pages for small memory sizes
    while (order > 2 && (total_pages >> 10) < (1UL << order))
    order--;
    empty_zero_page = (unsigned long)memblock_alloc_or_panic(PAGE_SIZE << order, PAGE_SIZE);
    zero_page_mask = ((PAGE_SIZE << order) - 1) & PAGE_MASK;
    set_memory_ro(empty_zero_page, 1UL << order);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) -> void __init {
    void __init arch_zone_limits_init(unsigned long *max_zone_pfns)
    {
    max_zone_pfns[ZONE_DMA] = virt_to_pfn(MAX_DMA_ADDRESS);
    max_zone_pfns[ZONE_NORMAL] = max_low_pfn;
    }
//
// paging_init() sets up the page tables
//
#[no_mangle]
pub unsafe extern "C" fn paging_init() -> void __init {
    void __init paging_init(void)
    {
    vmem_map_init();
    zone_dma_limit = DMA_BIT_MASK(31);
    }
#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    void mark_rodata_ro(void)
    {
    let mut size: c_ulong = __end_ro_after_init - __start_ro_after_init;
    if (cpu_has_nx())
    system_ctl_set_bit(0, CR0_INSTRUCTION_EXEC_PROTECTION_BIT);
    __set_memory_ro(__start_ro_after_init, __end_ro_after_init);
    pr_info("Write protected read-only-after-init data: %luk\n", size >> 10);
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_encrypted(vaddr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_encrypted(unsigned long vaddr, int numpages)
    {
    int i;
// make specified pages unshared, (swiotlb, dma_free)
    for (i = 0; i < numpages; ++i) {
    uv_remove_shared(virt_to_phys((void *)vaddr));
    vaddr += PAGE_SIZE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_decrypted(vaddr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_decrypted(unsigned long vaddr, int numpages)
    {
    int i;
// make specified pages shared (swiotlb, dma_alloca)
    for (i = 0; i < numpages; ++i) {
    uv_set_shared(virt_to_phys((void *)vaddr));
    vaddr += PAGE_SIZE;
    }
    return 0;
    }
// are we a protected virtualization guest?
#[no_mangle]
pub unsafe extern "C" fn force_dma_unencrypted(dev: *mut device) -> bool {
    bool force_dma_unencrypted(struct device *dev)
    {
    return is_prot_virt_guest();
    }
#[no_mangle]
pub unsafe extern "C" fn cc_platform_has(attr: enum cc_attr) -> bool {
    bool cc_platform_has(enum cc_attr attr)
    {
    switch (attr) {
    case CC_ATTR_MEM_ENCRYPT:
    case CC_ATTR_GUEST_MEM_ENCRYPT:
    return is_prot_virt_guest();
    default:
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(cc_platform_has);
// protected virtualization
#[no_mangle]
unsafe extern "C" fn pv_init() -> void __init {
    static void __init pv_init(void)
    {
    if (!is_prot_virt_guest())
    return;
    virtio_set_mem_acc_cb(virtio_require_restricted_mem_acc);
// make sure bounce buffers are shared
    swiotlb_init(true, SWIOTLB_VERBOSE | SWIOTLB_ANY);
    swiotlb_update_mem_attributes();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_mm_preinit() -> void __init {
    void __init arch_mm_preinit(void)
    {
    cpumask_set_cpu(0, &init_mm.context.cpu_attach_mask);
    cpumask_set_cpu(0, mm_cpumask(&init_mm));
    pv_init();
    }
#[no_mangle]
pub unsafe extern "C" fn memory_block_size_bytes() -> c_ulong {
    unsigned long memory_block_size_bytes(void)
    {
//
// Make sure the memory block size is always greater
// or equal than the memory increment size.
//
    return max_t(unsigned long, MIN_MEMORY_BLOCK_SIZE, sclp.rzm);
    }
    unsigned long __per_cpu_offset[NR_CPUS] __read_mostly;
    EXPORT_SYMBOL(__per_cpu_offset);
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_distance(from: c_uint, to: c_uint) -> int __init {
    static int __init pcpu_cpu_distance(unsigned int from, unsigned int to)
    {
    return LOCAL_DISTANCE;
    }
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_to_node(cpu: c_int) -> int __init {
    static int __init pcpu_cpu_to_node(int cpu)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn setup_per_cpu_areas() -> void __init {
    void __init setup_per_cpu_areas(void)
    {
    unsigned long delta;
    unsigned int cpu;
    int rc;
//
// Always reserve area for module percpu variables.  That's
// what the legacy allocator did.
//
    rc = pcpu_embed_first_chunk(PERCPU_MODULE_RESERVE,
    PERCPU_DYNAMIC_RESERVE, PAGE_SIZE,
    pcpu_cpu_distance,
    pcpu_cpu_to_node);
    if (rc < 0)
    panic("Failed to initialize percpu areas.");
    delta = (unsigned long)pcpu_base_addr - (unsigned long)__per_cpu_start;
    for_each_possible_cpu(cpu)
    __per_cpu_offset[cpu] = delta + pcpu_unit_offsets[cpu];
    }

// Prevent memory blocks which contain cma regions from going offline
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_cma_mem_data {
    pub start: c_ulong,
    pub end: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn s390_cma_check_range(cma: *mut cma, data: *mut c_void) -> c_int {
    static int s390_cma_check_range(struct cma *cma, void *data)
    {
    struct s390_cma_mem_data *mem_data;
    mem_data = data;
    if (cma_intersects(cma, mem_data.start, mem_data.end))
    return -EBUSY;
    return 0;
    }
    static int s390_cma_mem_notifier(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct s390_cma_mem_data mem_data;
    struct memory_notify *arg;
    let mut rc: c_int = 0;
    arg = data;
    mem_data.start = arg.start_pfn << PAGE_SHIFT;
    mem_data.end = mem_data.start + (arg.nr_pages << PAGE_SHIFT);
    if (action == MEM_GOING_OFFLINE)
    rc = cma_for_each_area(s390_cma_check_range, &mem_data);
    return notifier_from_errno(rc);
    }
    static struct notifier_block s390_cma_mem_nb = {
    .notifier_call = s390_cma_mem_notifier,
    };
#[no_mangle]
unsafe extern "C" fn s390_cma_mem_init() -> int __init {
    static int __init s390_cma_mem_init(void)
    {
    return register_memory_notifier(&s390_cma_mem_nb);
    }
    device_initcall(s390_cma_mem_init);

    int arch_add_memory(int nid, u64 start, u64 size,
    struct mhp_params *params)
    {
    let mut start_pfn: c_ulong = PFN_DOWN(start);
    let mut size_pages: c_ulong = PFN_DOWN(size);
    int rc;
    if (WARN_ON_ONCE(pgprot_val(params.pgprot) != pgprot_val(PAGE_KERNEL)))
    return -EINVAL;
    VM_BUG_ON(!mhp_range_allowed(start, size, true));
    rc = vmem_add_mapping(start, size);
    if (rc)
    return rc;
    rc = __add_pages(nid, start_pfn, size_pages, params);
    if (rc)
    vmem_remove_mapping(start, size);
    return rc;
    }
    void arch_remove_memory(u64 start, u64 size, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    let mut start_pfn: c_ulong = start >> PAGE_SHIFT;
    let mut nr_pages: c_ulong = size >> PAGE_SHIFT;
    __remove_pages(start_pfn, nr_pages, altmap, pgmap);
    vmem_remove_mapping(start, size);
    }

    static struct execmem_info execmem_info __ro_after_init;
    struct execmem_info __init *execmem_arch_setup(void)
    {
    let mut module_load_offset: c_ulong = 0;
    unsigned long start;
    if (kaslr_enabled())
    module_load_offset = get_random_u32_inclusive(1, 1024) * PAGE_SIZE;
    start = MODULES_VADDR + module_load_offset;
    execmem_info = (struct execmem_info){
    .ranges = {
    [EXECMEM_DEFAULT] = {
    .flags	= EXECMEM_KASAN_SHADOW,
    .start	= start,
    .end	= MODULES_END,
    .pgprot	= PAGE_KERNEL,
    .alignment = MODULE_ALIGN,
    },
    },
    };
    return &execmem_info;
    }
