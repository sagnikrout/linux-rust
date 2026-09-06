//! Automatically rewritten from C to Rust
//! Source: mm/sparse-vmemmap.c
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
// Virtual Memory Map support
//
// (C) 2007 sgi. Christoph Lameter.
//
// Virtual memory maps allow VM primitives pfn_to_page, page_to_pfn,
// virt_to_page, page_address() to be implemented as a base offset
// calculation without memory access.
//
// However, virtual mappings need a page table and TLBs. Many Linux
// architectures already map their physical space using 1-1 mappings
// via TLBs. For those arches the virtual memory map is essentially
// for free if we use the same page size as the 1-1 mappings. In that
// case the overhead consists of a few additional pages that are
// allocated to create a view of memory for vmemmap.
//
// The architecture is expected to provide a vmemmap_populate() function
// to instantiate the mapping.
//

//
// Flags for vmemmap_populate_range and friends.
//
// Get a ref on the head page struct page, for ZONE_DEVICE compound pages
pub const VMEMMAP_POPULATE_PAGEREF: c_uint = 0x0001;

//
// Allocate a block of memory to be used to back the virtual memory map
// or to back the page tables that are used to create the mapping.
// Uses the main allocators if they are available, else bootmem.
//
    static void * __ref __earlyonly_bootmem_alloc(int node,
    unsigned long size,
    unsigned long align,
    unsigned long goal)
    {
    return memmap_alloc(size, align, goal, node, false);
    }
#[no_mangle]
pub unsafe extern "C" fn vmemmap_alloc_block(size: c_ulong, node: c_int) -> *mut void  __meminit {
    void * __meminit vmemmap_alloc_block(unsigned long size, int node)
    {
// If the main allocator is up use that, fallback to bootmem.
    if (slab_is_available()) {
    let mut gfp_mask: gfp_t = GFP_KERNEL|__GFP_RETRY_MAYFAIL|__GFP_NOWARN;
    let mut order: c_int = get_order(size);
    static bool warned __meminitdata;
    struct page *page;
    page = alloc_pages_node(node, gfp_mask, order);
    if (page)
    return page_address(page);
    if (!warned) {
    warn_alloc(gfp_mask & ~__GFP_NOWARN, core::ptr::null_mut(),
    "vmemmap alloc failure: order:%u", order);
    warned = true;
    }
    return core::ptr::null_mut();
    } else
    return __earlyonly_bootmem_alloc(node, size, size,
    __pa(MAX_DMA_ADDRESS));
    }
    static void * __meminit altmap_alloc_block_buf(unsigned long size,
    struct vmem_altmap *altmap);
// need to make sure size is all the same during early stage
    void * __meminit vmemmap_alloc_block_buf(unsigned long size, int node,
    struct vmem_altmap *altmap)
    {
    if (altmap)
    return altmap_alloc_block_buf(size, altmap);
    return vmemmap_alloc_block(size, node);
    }
#[no_mangle]
unsafe extern "C" fn vmem_altmap_next_pfn(altmap: *mut vmem_altmap) -> unsigned long __meminit {
    static unsigned long __meminit vmem_altmap_next_pfn(struct vmem_altmap *altmap)
    {
    return altmap.base_pfn + altmap.reserve + altmap.alloc
    + altmap.align;
    }
#[no_mangle]
unsafe extern "C" fn vmem_altmap_nr_free(altmap: *mut vmem_altmap) -> unsigned long __meminit {
    static unsigned long __meminit vmem_altmap_nr_free(struct vmem_altmap *altmap)
    {
    let mut allocated: c_ulong = altmap.alloc + altmap.align;
    if (altmap.free > allocated)
    return altmap.free - allocated;
    return 0;
    }
    static void * __meminit altmap_alloc_block_buf(unsigned long size,
    struct vmem_altmap *altmap)
    {
    unsigned long pfn, nr_pfns, nr_align;
    if (size & ~PAGE_MASK) {
    pr_warn_once("%s: allocations must be multiple of PAGE_SIZE (%ld)\n",
    __func__, size);
    return core::ptr::null_mut();
    }
    pfn = vmem_altmap_next_pfn(altmap);
    nr_pfns = size >> PAGE_SHIFT;
    nr_align = 1UL << find_first_bit(&nr_pfns, BITS_PER_LONG);
    nr_align = ALIGN(pfn, nr_align) - pfn;
    if (nr_pfns + nr_align > vmem_altmap_nr_free(altmap))
    return core::ptr::null_mut();
    altmap.alloc += nr_pfns;
    altmap.align += nr_align;
    pfn += nr_align;
    pr_debug("%s: pfn: %#lx alloc: %ld align: %ld nr: %#lx\n",
    __func__, pfn, altmap.alloc, altmap.align, nr_pfns);
    return __va(__pfn_to_phys(pfn));
    }
    void __meminit vmemmap_verify(pte_t *pte, int node,
    unsigned long start, unsigned long end)
    {
    let mut pfn: c_ulong = pte_pfn(ptep_get(pte));
    let mut actual_node: c_int = early_pfn_to_nid(pfn);
    if (node_distance(actual_node, node) > LOCAL_DISTANCE)
    pr_warn_once("[%lx-%lx] potential offnode page_structs\n",
    start, end - 1);
    }
    static pte_t * __meminit vmemmap_pte_populate(pmd_t *pmd, unsigned long addr, int node,
    struct vmem_altmap *altmap,
    unsigned long ptpfn, unsigned long flags)
    {
    pte_t *pte = pte_offset_kernel(pmd, addr);
    if (pte_none(ptep_get(pte))) {
    pte_t entry;
    void *p;
    if (ptpfn == (unsigned long)-1) {
    p = vmemmap_alloc_block_buf(PAGE_SIZE, node, altmap);
    if (!p)
    return core::ptr::null_mut();
    ptpfn = PHYS_PFN(__pa(p));
    } else {
//
// When a PTE/PMD entry is freed from the init_mm
// there's a free_pages() call to this page allocated
// above. Thus this get_page() is paired with the
// put_page_testzero() on the freeing path.
// This can only called by certain ZONE_DEVICE path,
// and through vmemmap_populate_compound_pages() when
// slab is available.
//
    if (flags & VMEMMAP_POPULATE_PAGEREF)
    get_page(pfn_to_page(ptpfn));
    }
    entry = pfn_pte(ptpfn, PAGE_KERNEL);
    set_pte_at(&init_mm, addr, pte, entry);
    }
    return pte;
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_alloc_block_zero(size: c_ulong, node: c_int) -> *mut void  __meminit {
    static void * __meminit vmemmap_alloc_block_zero(unsigned long size, int node)
    {
    void *p = vmemmap_alloc_block(size, node);
    if (!p)
    return core::ptr::null_mut();
    memset(p, 0, size);
    return p;
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_pmd_populate(pud: *mut pud_t, addr: c_ulong, node: c_int) -> *mut pmd_t  __meminit {
    static pmd_t * __meminit vmemmap_pmd_populate(pud_t *pud, unsigned long addr, int node)
    {
    pmd_t *pmd = pmd_offset(pud, addr);
    if (pmd_none(*pmd)) {
    void *p = vmemmap_alloc_block_zero(PAGE_SIZE, node);
    if (!p)
    return core::ptr::null_mut();
    kernel_pte_init(p);
    pmd_populate_kernel(&init_mm, pmd, p);
    }
    return pmd;
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_pud_populate(p4d: *mut p4d_t, addr: c_ulong, node: c_int) -> *mut pud_t  __meminit {
    static pud_t * __meminit vmemmap_pud_populate(p4d_t *p4d, unsigned long addr, int node)
    {
    pud_t *pud = pud_offset(p4d, addr);
    if (pud_none(*pud)) {
    void *p = vmemmap_alloc_block_zero(PAGE_SIZE, node);
    if (!p)
    return core::ptr::null_mut();
    pmd_init(p);
    pud_populate(&init_mm, pud, p);
    }
    return pud;
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_p4d_populate(pgd: *mut pgd_t, addr: c_ulong, node: c_int) -> *mut p4d_t  __meminit {
    static p4d_t * __meminit vmemmap_p4d_populate(pgd_t *pgd, unsigned long addr, int node)
    {
    p4d_t *p4d = p4d_offset(pgd, addr);
    if (p4d_none(*p4d)) {
    void *p = vmemmap_alloc_block_zero(PAGE_SIZE, node);
    if (!p)
    return core::ptr::null_mut();
    pud_init(p);
    p4d_populate_kernel(addr, p4d, p);
    }
    return p4d;
    }
#[no_mangle]
unsafe extern "C" fn vmemmap_pgd_populate(addr: c_ulong, node: c_int) -> *mut pgd_t  __meminit {
    static pgd_t * __meminit vmemmap_pgd_populate(unsigned long addr, int node)
    {
    pgd_t *pgd = pgd_offset_k(addr);
    if (pgd_none(*pgd)) {
    void *p = vmemmap_alloc_block_zero(PAGE_SIZE, node);
    if (!p)
    return core::ptr::null_mut();
    pgd_populate_kernel(addr, pgd, p);
    }
    return pgd;
    }
    static pte_t * __meminit vmemmap_populate_address(unsigned long addr, int node,
    struct vmem_altmap *altmap,
    unsigned long ptpfn,
    unsigned long flags)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    pgd = vmemmap_pgd_populate(addr, node);
    if (!pgd)
    return core::ptr::null_mut();
    p4d = vmemmap_p4d_populate(pgd, addr, node);
    if (!p4d)
    return core::ptr::null_mut();
    pud = vmemmap_pud_populate(p4d, addr, node);
    if (!pud)
    return core::ptr::null_mut();
    pmd = vmemmap_pmd_populate(pud, addr, node);
    if (!pmd)
    return core::ptr::null_mut();
    pte = vmemmap_pte_populate(pmd, addr, node, altmap, ptpfn, flags);
    if (!pte)
    return core::ptr::null_mut();
    vmemmap_verify(pte, node, addr, addr + PAGE_SIZE);
    return pte;
    }
    static int __meminit vmemmap_populate_range(unsigned long start,
    unsigned long end, int node,
    struct vmem_altmap *altmap,
    unsigned long ptpfn,
    unsigned long flags)
    {
    let mut addr: c_ulong = start;
    pte_t *pte;
    for (; addr < end; addr += PAGE_SIZE) {
    pte = vmemmap_populate_address(addr, node, altmap,
    ptpfn, flags);
    if (!pte)
    return -ENOMEM;
    }
    return 0;
    }
    int __meminit vmemmap_populate_basepages(unsigned long start, unsigned long end,
    int node, struct vmem_altmap *altmap)
    {
    return vmemmap_populate_range(start, end, node, altmap, -1, 0);
    }
//
// Write protect the mirrored tail page structs for HVO. This will be
// called from the hugetlb code when gathering and initializing the
// memblock allocated gigantic pages. The write protect can't be
// done earlier, since it can't be guaranteed that the reserved
// page structures will not be written to during initialization,
// even if CONFIG_DEFERRED_STRUCT_PAGE_INIT is enabled.
//
// The PTEs are known to exist, and nothing else should be touching
// these pages. The caller is responsible for any TLB flushing.
//
    void vmemmap_wrprotect_hvo(unsigned long addr, unsigned long end,
    int node, unsigned long headsize)
    {
    unsigned long maddr;
    pte_t *pte;
    for (maddr = addr + headsize; maddr < end; maddr += PAGE_SIZE) {
    pte = virt_to_kpte(maddr);
    ptep_set_wrprotect(&init_mm, maddr, pte);
    }
    }

    static __meminit struct page *vmemmap_get_tail(unsigned int order, struct zone *zone)
    {
    struct page *p, *tail;
    unsigned int idx;
    let mut node: c_int = zone_to_nid(zone);
    if (WARN_ON_ONCE(order < VMEMMAP_TAIL_MIN_ORDER))
    return core::ptr::null_mut();
    if (WARN_ON_ONCE(order > MAX_FOLIO_ORDER))
    return core::ptr::null_mut();
    idx = order - VMEMMAP_TAIL_MIN_ORDER;
    tail = zone.vmemmap_tails[idx];
    if (tail)
    return tail;
//
// Only allocate the page, but do not initialize it.
//
// Any initialization done here will be overwritten by memmap_init().
//
// hugetlb_bootmem_struct_page_init() will take care of initialization
// after memmap_init().
//
    p = vmemmap_alloc_block_zero(PAGE_SIZE, node);
    if (!p)
    return core::ptr::null_mut();
    tail = virt_to_page(p);
    zone.vmemmap_tails[idx] = tail;
    return tail;
    }
    int __meminit vmemmap_populate_hvo(unsigned long addr, unsigned long end,
    unsigned int order, struct zone *zone,
    unsigned long headsize)
    {
    unsigned long maddr;
    struct page *tail;
    pte_t *pte;
    let mut node: c_int = zone_to_nid(zone);
    tail = vmemmap_get_tail(order, zone);
    if (!tail)
    return -ENOMEM;
    for (maddr = addr; maddr < addr + headsize; maddr += PAGE_SIZE) {
    pte = vmemmap_populate_address(maddr, node, core::ptr::null_mut(), -1, 0);
    if (!pte)
    return -ENOMEM;
    }
//
// Reuse the last page struct page mapped above for the rest.
//
    return vmemmap_populate_range(maddr, end, node, core::ptr::null_mut(),
    page_to_pfn(tail), 0);
    }

    void __weak __meminit vmemmap_set_pmd(pmd_t *pmd, void *p, int node,
    unsigned long addr, unsigned long next)
    {
    WARN_ON_ONCE(!pmd_set_huge(pmd, virt_to_phys(p), PAGE_KERNEL));
    }
    int __weak __meminit vmemmap_check_pmd(pmd_t *pmd, int node,
    unsigned long addr, unsigned long next)
    {
    if (!pmd_leaf(pmdp_get(pmd)))
    return 0;
    vmemmap_verify((pte_t *)pmd, node, addr, next);
    return 1;
    }
    int __meminit vmemmap_populate_hugepages(unsigned long start, unsigned long end,
    int node, struct vmem_altmap *altmap)
    {
    unsigned long addr;
    unsigned long next;
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    for (addr = start; addr < end; addr = next) {
    next = pmd_addr_end(addr, end);
    pgd = vmemmap_pgd_populate(addr, node);
    if (!pgd)
    return -ENOMEM;
    p4d = vmemmap_p4d_populate(pgd, addr, node);
    if (!p4d)
    return -ENOMEM;
    pud = vmemmap_pud_populate(p4d, addr, node);
    if (!pud)
    return -ENOMEM;
    pmd = pmd_offset(pud, addr);
    if (pmd_none(pmdp_get(pmd))) {
    void *p;
    p = vmemmap_alloc_block_buf(PMD_SIZE, node, altmap);
    if (p) {
    vmemmap_set_pmd(pmd, p, node, addr, next);
    continue;
    } else if (altmap) {
//
// No fallback: In any case we care about, the
// altmap should be reasonably sized and aligned
// such that vmemmap_alloc_block_buf() will always
// succeed. For consistency with the PTE case,
// return an error here as failure could indicate
// a configuration issue with the size of the altmap.
//
    return -ENOMEM;
    }
    } else if (vmemmap_check_pmd(pmd, node, addr, next))
    continue;
    if (vmemmap_populate_basepages(addr, next, node, altmap))
    return -ENOMEM;
    }
    return 0;
    }

//
// For compound pages bigger than section size (e.g. x86 1G compound
// pages with 2M subsection size) fill the rest of sections as tail
// pages.
//
// Note that memremap_pages() resets @nr_range value and will increment
// it after each range successful onlining. Thus the value or @nr_range
// at section memmap populate corresponds to the in-progress range
// being onlined here.
//
    static bool __meminit reuse_compound_section(unsigned long start_pfn,
    struct dev_pagemap *pgmap)
    {
    let mut nr_pages: c_ulong = pgmap_vmemmap_nr(pgmap);
    unsigned long offset = start_pfn -
    PHYS_PFN(pgmap.ranges[pgmap.nr_range].start);
    return !IS_ALIGNED(offset, nr_pages) && nr_pages > PAGES_PER_SUBSECTION;
    }
#[no_mangle]
unsafe extern "C" fn compound_section_tail_page(addr: c_ulong) -> *mut pte_t  __meminit {
    static pte_t * __meminit compound_section_tail_page(unsigned long addr)
    {
    pte_t *pte;
    addr -= PAGE_SIZE;
//
// Assuming sections are populated sequentially, the previous section's
// page data can be reused.
//
    pte = pte_offset_kernel(pmd_off_k(addr), addr);
    if (!pte)
    return core::ptr::null_mut();
    return pte;
    }
    static int __meminit vmemmap_populate_compound_pages(unsigned long start_pfn,
    unsigned long start,
    unsigned long end, int node,
    struct dev_pagemap *pgmap)
    {
    unsigned long size, addr;
    pte_t *pte;
    int rc;
    if (reuse_compound_section(start_pfn, pgmap)) {
    pte = compound_section_tail_page(start);
    if (!pte)
    return -ENOMEM;
//
// Reuse the page that was populated in the prior iteration
// with just tail struct pages.
//
    return vmemmap_populate_range(start, end, node, core::ptr::null_mut(),
    pte_pfn(ptep_get(pte)),
    VMEMMAP_POPULATE_PAGEREF);
    }
    size = min(end - start, pgmap_vmemmap_nr(pgmap) * sizeof(struct page));
    for (addr = start; addr < end; addr += size) {
    unsigned long next, last = addr + size;
// Populate the head page vmemmap page
    pte = vmemmap_populate_address(addr, node, core::ptr::null_mut(), -1, 0);
    if (!pte)
    return -ENOMEM;
// Populate the tail pages vmemmap page
    next = addr + PAGE_SIZE;
    pte = vmemmap_populate_address(next, node, core::ptr::null_mut(), -1, 0);
    if (!pte)
    return -ENOMEM;
//
// Reuse the previous page for the rest of tail pages
// See layout diagram in Documentation/mm/vmemmap_dedup.rst
//
    next += PAGE_SIZE;
    rc = vmemmap_populate_range(next, last, node, core::ptr::null_mut(),
    pte_pfn(ptep_get(pte)),
    VMEMMAP_POPULATE_PAGEREF);
    if (rc)
    return -ENOMEM;
    }
    return 0;
    }

    struct page * __meminit __populate_section_memmap(unsigned long pfn,
    unsigned long nr_pages, int nid, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    let mut start: c_ulong = (unsigned long) pfn_to_page(pfn);
    let mut end: c_ulong = start + nr_pages * sizeof(struct page);
    int r;
    if (WARN_ON_ONCE(!IS_ALIGNED(pfn, PAGES_PER_SUBSECTION) ||
    !IS_ALIGNED(nr_pages, PAGES_PER_SUBSECTION)))
    return core::ptr::null_mut();
    if (vmemmap_can_optimize(altmap, pgmap))
    r = vmemmap_populate_compound_pages(pfn, start, end, nid, pgmap);
    else
    r = vmemmap_populate(start, end, nid, altmap);
    if (r < 0)
    return core::ptr::null_mut();
    flush_cache_vmap(start, end);
    return pfn_to_page(pfn);
    }

//
// This is called just before initializing sections for a NUMA node.
// Any special initialization that needs to be done before the
// generic initialization can be done from here. Sections that
// are initialized in hooks called from here will be skipped by
// the generic initialization.
//
#[no_mangle]
pub unsafe extern "C" fn sparse_vmemmap_init_nid_early(nid: c_int) -> void __init {
    void __init sparse_vmemmap_init_nid_early(int nid)
    {
    hugetlb_vmemmap_init_early(nid);
    }

    static void subsection_mask_set(unsigned long *map, unsigned long pfn,
    unsigned long nr_pages)
    {
    let mut idx: c_int = subsection_map_index(pfn);
    let mut end: c_int = subsection_map_index(pfn + nr_pages - 1);
    bitmap_set(map, idx, end - idx + 1);
    }
#[no_mangle]
unsafe extern "C" fn sparse_init_subsection_map_range(pfn: c_ulong, nr_pages: c_ulong) -> void __init {
    static void __init sparse_init_subsection_map_range(unsigned long pfn, unsigned long nr_pages)
    {
    let mut end_sec_nr: c_int = pfn_to_section_nr(pfn + nr_pages - 1);
    unsigned long nr, start_sec_nr = pfn_to_section_nr(pfn);
    for (nr = start_sec_nr; nr <= end_sec_nr; nr++) {
    struct mem_section *ms;
    unsigned long pfns;
    pfns = min(nr_pages, PAGES_PER_SECTION
    - (pfn & ~PAGE_SECTION_MASK));
    ms = __nr_to_section(nr);
    subsection_mask_set(ms.usage.subsection_map, pfn, pfns);
    pr_debug("%s: sec: %lu pfns: %lu set(%d, %d)\n", __func__, nr,
    pfns, subsection_map_index(pfn),
    subsection_map_index(pfn + pfns - 1));
    pfn += pfns;
    nr_pages -= pfns;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sparse_init_subsection_map() -> void __init {
    void __init sparse_init_subsection_map(void)
    {
    int i, nid;
    unsigned long start, end;
    for_each_mem_pfn_range(i, MAX_NUMNODES, &start, &end, &nid)
    sparse_init_subsection_map_range(start, end - start);
    }

// Mark all memory sections within the pfn range as online
#[no_mangle]
pub unsafe extern "C" fn online_mem_sections(start_pfn: c_ulong, end_pfn: c_ulong) {
    void online_mem_sections(unsigned long start_pfn, unsigned long end_pfn)
    {
    unsigned long pfn;
    for (pfn = start_pfn; pfn < end_pfn; pfn += PAGES_PER_SECTION) {
    let mut section_nr: c_ulong = pfn_to_section_nr(pfn);
    struct mem_section *ms = __nr_to_section(section_nr);
    ms.section_mem_map |= SECTION_IS_ONLINE;
    }
    }
// Mark all memory sections within the pfn range as offline
#[no_mangle]
pub unsafe extern "C" fn offline_mem_sections(start_pfn: c_ulong, end_pfn: c_ulong) {
    void offline_mem_sections(unsigned long start_pfn, unsigned long end_pfn)
    {
    unsigned long pfn;
    for (pfn = start_pfn; pfn < end_pfn; pfn += PAGES_PER_SECTION) {
    let mut section_nr: c_ulong = pfn_to_section_nr(pfn);
    struct mem_section *ms = __nr_to_section(section_nr);
    ms.section_mem_map &= ~SECTION_IS_ONLINE;
    }
    }
    static int __meminit section_nr_vmemmap_pages(unsigned long pfn, unsigned long nr_pages,
    struct vmem_altmap *altmap, struct dev_pagemap *pgmap)
    {
    let mut order: c_uint = pgmap ? pgmap.vmemmap_shift : 0;
    let mut pages_per_compound: c_ulong = 1UL << order;
    VM_WARN_ON_ONCE(!IS_ALIGNED(pfn | nr_pages, PAGES_PER_SUBSECTION));
    VM_WARN_ON_ONCE(nr_pages > PAGES_PER_SECTION);
    if (!vmemmap_can_optimize(altmap, pgmap))
    return DIV_ROUND_UP(nr_pages * sizeof(struct page), PAGE_SIZE);
    if (order < PFN_SECTION_SHIFT) {
    VM_WARN_ON_ONCE(!IS_ALIGNED(pfn | nr_pages, pages_per_compound));
    return VMEMMAP_RESERVE_NR * nr_pages / pages_per_compound;
    }
    VM_WARN_ON_ONCE(!IS_ALIGNED(pfn | nr_pages, PAGES_PER_SECTION));
    if (IS_ALIGNED(pfn, pages_per_compound))
    return VMEMMAP_RESERVE_NR;
    return 0;
    }
    static struct page * __meminit populate_section_memmap(unsigned long pfn,
    unsigned long nr_pages, int nid, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    struct page *page = __populate_section_memmap(pfn, nr_pages, nid, altmap,
    pgmap);
    memmap_pages_add(section_nr_vmemmap_pages(pfn, nr_pages, altmap, pgmap));
    return page;
    }
    static void depopulate_section_memmap(unsigned long pfn, unsigned long nr_pages,
    struct vmem_altmap *altmap, struct dev_pagemap *pgmap)
    {
    let mut start: c_ulong = (unsigned long) pfn_to_page(pfn);
    let mut end: c_ulong = start + nr_pages * sizeof(struct page);
    memmap_pages_add(-section_nr_vmemmap_pages(pfn, nr_pages, altmap, pgmap));
    vmemmap_free(start, end, altmap);
    }
#[no_mangle]
unsafe extern "C" fn free_map_bootmem(memmap: *mut page) {
    static void free_map_bootmem(struct page *memmap)
    {
    let mut start: c_ulong = (unsigned long)memmap;
    let mut end: c_ulong = (unsigned long)(memmap + PAGES_PER_SECTION);
    let mut pfn: c_ulong = page_to_pfn(memmap);
    memmap_boot_pages_add(-section_nr_vmemmap_pages(pfn, PAGES_PER_SECTION,
    core::ptr::null_mut(), core::ptr::null_mut()));
    vmemmap_free(start, end, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn clear_subsection_map(pfn: c_ulong, nr_pages: c_ulong) -> c_int {
    static int clear_subsection_map(unsigned long pfn, unsigned long nr_pages)
    {
    DECLARE_BITMAP(map, SUBSECTIONS_PER_SECTION) = { 0 };
    DECLARE_BITMAP(tmp, SUBSECTIONS_PER_SECTION) = { 0 };
    struct mem_section *ms = __pfn_to_section(pfn);
    unsigned long *subsection_map = ms.usage
    ? &ms.usage.subsection_map[0] : core::ptr::null_mut();
    subsection_mask_set(map, pfn, nr_pages);
    if (subsection_map)
    bitmap_and(tmp, map, subsection_map, SUBSECTIONS_PER_SECTION);
    if (WARN(!subsection_map || !bitmap_equal(tmp, map, SUBSECTIONS_PER_SECTION),
    "section already deactivated (%#lx + %ld)\n",
    pfn, nr_pages))
    return -EINVAL;
    bitmap_xor(subsection_map, map, subsection_map, SUBSECTIONS_PER_SECTION);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_subsection_map_empty(ms: *mut mem_section) -> bool {
    static bool is_subsection_map_empty(struct mem_section *ms)
    {
    return bitmap_empty(&ms.usage.subsection_map[0],
    SUBSECTIONS_PER_SECTION);
    }
#[no_mangle]
unsafe extern "C" fn fill_subsection_map(pfn: c_ulong, nr_pages: c_ulong) -> c_int {
    static int fill_subsection_map(unsigned long pfn, unsigned long nr_pages)
    {
    struct mem_section *ms = __pfn_to_section(pfn);
    DECLARE_BITMAP(map, SUBSECTIONS_PER_SECTION) = { 0 };
    unsigned long *subsection_map;
    let mut rc: c_int = 0;
    subsection_mask_set(map, pfn, nr_pages);
    subsection_map = &ms.usage.subsection_map[0];
    if (bitmap_empty(map, SUBSECTIONS_PER_SECTION))
    rc = -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: bitmap_intersects(map, _arg: subsection_map, _arg: SUBSECTIONS_PER_SECTION)) -> else {
    else if (bitmap_intersects(map, subsection_map, SUBSECTIONS_PER_SECTION))
    rc = -EEXIST;
    else
    bitmap_or(subsection_map, map, subsection_map,
    SUBSECTIONS_PER_SECTION);
    return rc;
    }
//
// To deactivate a memory region, there are 3 cases to handle:
//
// 1. deactivation of a partial hot-added section:
// a) section was present at memory init.
// b) section was hot-added post memory init.
// 2. deactivation of a complete hot-added section.
// 3. deactivation of a complete section from memory init.
//
// For 1, when subsection_map does not empty we will not be freeing the
// usage map, but still need to free the vmemmap range.
//
    static void section_deactivate(unsigned long pfn, unsigned long nr_pages,
    struct vmem_altmap *altmap, struct dev_pagemap *pgmap)
    {
    struct mem_section *ms = __pfn_to_section(pfn);
    let mut section_is_early: bool = early_section(ms);
    struct page *memmap = core::ptr::null_mut();
    bool empty;
    if (clear_subsection_map(pfn, nr_pages))
    return;
    empty = is_subsection_map_empty(ms);
    if (empty) {
//
// Mark the section invalid so that valid_section()
// return false. This prevents code from dereferencing
// ms->usage array.
//
    ms.section_mem_map &= ~SECTION_HAS_MEM_MAP;
//
// When removing an early section, the usage map is kept (as the
// usage maps of other sections fall into the same page). It
// will be re-used when re-adding the section - which is then no
// longer an early section. If the usage map is PageReserved, it
// was allocated during boot.
//
    if (!PageReserved(virt_to_page(ms.usage))) {
    kfree_rcu(ms.usage, rcu);
    WRITE_ONCE(ms.usage, core::ptr::null_mut());
    }
    memmap = pfn_to_page(SECTION_ALIGN_DOWN(pfn));
    }
//
// The memmap of early sections is always fully populated. See
// section_activate() and pfn_valid() .
//
    if (!section_is_early)
    depopulate_section_memmap(pfn, nr_pages, altmap, pgmap);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: memmap) -> else {
    else if (memmap)
    free_map_bootmem(memmap);
    if (empty)
    ms.section_mem_map = (unsigned long)core::ptr::null_mut();
    }
    static struct page * __meminit section_activate(int nid, unsigned long pfn,
    unsigned long nr_pages, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    struct mem_section *ms = __pfn_to_section(pfn);
    struct mem_section_usage *usage = core::ptr::null_mut();
    struct page *memmap;
    int rc;
    if (!ms.usage) {
    usage = kzalloc(mem_section_usage_size(), GFP_KERNEL);
    if (!usage)
    return ERR_PTR(-ENOMEM);
    ms.usage = usage;
    }
    rc = fill_subsection_map(pfn, nr_pages);
    if (rc) {
    if (usage)
    ms.usage = core::ptr::null_mut();
    kfree(usage);
    return ERR_PTR(rc);
    }
//
// The early init code does not consider partially populated
// initial sections, it simply assumes that memory will never be
// referenced.  If we hot-add memory into such a section then we
// do not need to populate the memmap and can simply reuse what
// is already there.
//
    if (nr_pages < PAGES_PER_SECTION && early_section(ms))
    return pfn_to_page(pfn);
    memmap = populate_section_memmap(pfn, nr_pages, nid, altmap, pgmap);
    if (!memmap) {
    section_deactivate(pfn, nr_pages, altmap, pgmap);
    return ERR_PTR(-ENOMEM);
    }
    return memmap;
    }
//
// sparse_add_section - add a memory section, or populate an existing one
// @nid: The node to add section on
// @start_pfn: start pfn of the memory range
// @nr_pages: number of pfns to add in the section
// @altmap: alternate pfns to allocate the memmap backing store
// @pgmap: alternate compound page geometry for devmap mappings
//
// This is only intended for hotplug.
//
// Note that only VMEMMAP supports sub-section aligned hotplug,
// the proper alignment and size are gated by check_pfn_span().
//
// Return:
// * 0		- On success.
// * -EEXIST	- Section has been present.
// * -ENOMEM	- Out of memory.
//
    int __meminit sparse_add_section(int nid, unsigned long start_pfn,
    unsigned long nr_pages, struct vmem_altmap *altmap,
    struct dev_pagemap *pgmap)
    {
    let mut section_nr: c_ulong = pfn_to_section_nr(start_pfn);
    struct mem_section *ms;
    struct page *memmap;
    int ret;
    ret = sparse_index_init(section_nr, nid);
    if (ret < 0)
    return ret;
    memmap = section_activate(nid, start_pfn, nr_pages, altmap, pgmap);
    if (IS_ERR(memmap))
    return PTR_ERR(memmap);
//
// Poison uninitialized struct pages in order to catch invalid flags
// combinations.
//
    page_init_poison(memmap, sizeof(struct page) * nr_pages);
    ms = __nr_to_section(section_nr);
    __section_mark_present(ms, section_nr);
// Align memmap to section boundary in the subsection case
    if (section_nr_to_pfn(section_nr) != start_pfn)
    memmap = pfn_to_page(section_nr_to_pfn(section_nr));
    sparse_init_one_section(ms, section_nr, memmap, ms.usage, 0);
    return 0;
    }
    void sparse_remove_section(unsigned long pfn, unsigned long nr_pages,
    struct vmem_altmap *altmap, struct dev_pagemap *pgmap)
    {
    struct mem_section *ms = __pfn_to_section(pfn);
    if (WARN_ON_ONCE(!valid_section(ms)))
    return;
    section_deactivate(pfn, nr_pages, altmap, pgmap);
    }
