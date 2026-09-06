//! Automatically rewritten from C to Rust
//! Source: drivers/xen/swiotlb-xen.c
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
// Copyright 2010
// by Konrad Rzeszutek Wilk <konrad.wilk@oracle.com>
//
// This code provides a IOMMU for Xen PV guests with PCI passthrough.
//
// PV guests under Xen are running in an non-contiguous memory architecture.
//
// When PCI pass-through is utilized, this necessitates an IOMMU for
// translating bus (DMA) to virtual and vice-versa and also providing a
// mechanism to have contiguous pages for device drivers operations (say DMA
// operations).
//
// Specifically, under Xen the Linux idea of pages is an illusion. It
// assumes that pages start at zero and go up to the available memory. To
// help with that, the Linux Xen MMU provides a lookup mechanism to
// translate the page frame numbers (PFN) to machine frame numbers (MFN)
// and vice-versa. The MFN are the "real" frame numbers. Furthermore
// memory is not contiguous. Xen hypervisor stitches memory for guests
// from different pools, which means there is no guarantee that PFN==MFN
// and PFN+1==MFN+1. Lastly with Xen 4.0, pages (in debug mode) are
// allocated in descending order (high to low), meaning the guest might
// never get any MFN's under the 4GB mark.
//

pub const MAX_DMA_BITS: c_int = 32;
//
// Quick lookup value of the bus address of the IOTLB.
//
#[no_mangle]
pub unsafe extern "C" fn xen_phys_to_bus(dev: *mut device, paddr: phys_addr_t) -> phys_addr_t {
    static inline phys_addr_t xen_phys_to_bus(struct device *dev, phys_addr_t paddr)
    {
    let mut bfn: c_ulong = pfn_to_bfn(XEN_PFN_DOWN(paddr));
    let mut baddr: phys_addr_t = (phys_addr_t)bfn << XEN_PAGE_SHIFT;
    baddr |= paddr & ~XEN_PAGE_MASK;
    return baddr;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    static inline dma_addr_t xen_phys_to_dma(struct device *dev, phys_addr_t paddr)
    {
    return phys_to_dma(dev, xen_phys_to_bus(dev, paddr));
    }
    static inline phys_addr_t xen_bus_to_phys(struct device *dev,
    phys_addr_t baddr)
    {
    let mut xen_pfn: c_ulong = bfn_to_pfn(XEN_PFN_DOWN(baddr));
    phys_addr_t paddr = (xen_pfn << XEN_PAGE_SHIFT) |
    (baddr & ~XEN_PAGE_MASK);
    return paddr;
    }
    static inline phys_addr_t xen_dma_to_phys(struct device *dev,
    dma_addr_t dma_addr)
    {
    return xen_bus_to_phys(dev, dma_to_phys(dev, dma_addr));
    }
#[no_mangle]
pub unsafe extern "C" fn range_requires_alignment(p: phys_addr_t, size: usize) -> bool {
    static inline bool range_requires_alignment(phys_addr_t p, size_t size)
    {
    let mut algn: phys_addr_t = 1ULL << (get_order(size) + PAGE_SHIFT);
    let mut bus_addr: phys_addr_t = pfn_to_bfn(XEN_PFN_DOWN(p)) << XEN_PAGE_SHIFT;
    return IS_ALIGNED(p, algn) && !IS_ALIGNED(bus_addr, algn);
    }
#[no_mangle]
pub unsafe extern "C" fn range_straddles_page_boundary(p: phys_addr_t, size: usize) -> c_int {
    static inline int range_straddles_page_boundary(phys_addr_t p, size_t size)
    {
    unsigned long next_bfn, xen_pfn = XEN_PFN_DOWN(p);
    unsigned int i, nr_pages = XEN_PFN_UP(xen_offset_in_page(p) + size);
    next_bfn = pfn_to_bfn(xen_pfn);
    for (i = 1; i < nr_pages; i++)
    if (pfn_to_bfn(++xen_pfn) != ++next_bfn)
    return 1;
    return 0;
    }
    static struct io_tlb_pool *xen_swiotlb_find_pool(struct device *dev,
    dma_addr_t dma_addr)
    {
    let mut bfn: c_ulong = XEN_PFN_DOWN(dma_to_phys(dev, dma_addr));
    let mut xen_pfn: c_ulong = bfn_to_local_pfn(bfn);
    let mut paddr: phys_addr_t = (phys_addr_t)xen_pfn << XEN_PAGE_SHIFT;
// If the address is outside our domain, it CAN
// have the same virtual address as another address
// in our domain. Therefore _only_ check address within our domain.
//
    if (pfn_valid(PFN_DOWN(paddr)))
    return swiotlb_find_pool(dev, paddr);
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn xen_swiotlb_fixup(buf: *mut c_void, nslabs: c_ulong) -> int __init {
    int __init xen_swiotlb_fixup(void *buf, unsigned long nslabs)
    {
    int rc;
    let mut order: c_uint = get_order(IO_TLB_SEGSIZE << IO_TLB_SHIFT);
    unsigned int i, dma_bits = order + PAGE_SHIFT;
    dma_addr_t dma_handle;
    let mut p: phys_addr_t = virt_to_phys(buf);
    BUILD_BUG_ON(IO_TLB_SEGSIZE & (IO_TLB_SEGSIZE - 1));
    BUG_ON(nslabs % IO_TLB_SEGSIZE);
    i = 0;
    do {
    do {
    rc = xen_create_contiguous_region(
    p + (i << IO_TLB_SHIFT), order,
    dma_bits, &dma_handle);
    } while (rc && dma_bits++ < MAX_DMA_BITS);
    if (rc)
    return rc;
    i += IO_TLB_SEGSIZE;
    } while (i < nslabs);
    return 0;
    }
    static void *
    xen_swiotlb_alloc_coherent(struct device *dev, size_t size,
    dma_addr_t *dma_handle, gfp_t flags, unsigned long attrs)
    {
    let mut dma_mask: u64 = dev.coherent_dma_mask;
    let mut order: c_int = get_order(size);
    phys_addr_t phys;
    void *ret;
// Align the allocation to the Xen page size
    size = ALIGN(size, XEN_PAGE_SIZE);
    ret = (void *)__get_free_pages(flags, get_order(size));
    if (!ret)
    return ret;
    phys = virt_to_phys(ret);
// dma_handle = xen_phys_to_dma(dev, phys);
    if (*dma_handle + size - 1 > dma_mask ||
    range_straddles_page_boundary(phys, size) ||
    range_requires_alignment(phys, size)) {
    if (xen_create_contiguous_region(phys, order, fls64(dma_mask),
    dma_handle) != 0)
    goto out_free_pages;
    SetPageXenRemapped(virt_to_page(ret));
    }
    memset(ret, 0, size);
    return ret;
    out_free_pages:
    free_pages((unsigned long)ret, get_order(size));
    return core::ptr::null_mut();
    }
    static void
    xen_swiotlb_free_coherent(struct device *dev, size_t size, void *vaddr,
    dma_addr_t dma_handle, unsigned long attrs)
    {
    let mut phys: phys_addr_t = virt_to_phys(vaddr);
    let mut order: c_int = get_order(size);
// Convert the size to actually allocated.
    size = ALIGN(size, XEN_PAGE_SIZE);
    if (WARN_ON_ONCE(dma_handle + size - 1 > dev.coherent_dma_mask) ||
    WARN_ON_ONCE(range_straddles_page_boundary(phys, size) ||
    range_requires_alignment(phys, size)))
    return;
    if (TestClearPageXenRemapped(virt_to_page(vaddr)))
    xen_destroy_contiguous_region(phys, order);
    free_pages((unsigned long)vaddr, get_order(size));
    }

//
// Map a single buffer of the indicated size for DMA in streaming mode.  The
// physical address to use is returned.
//
// Once the device is given the dma address, the device owns this memory until
// either xen_swiotlb_unmap_phys or xen_swiotlb_dma_sync_single is performed.
//
    static dma_addr_t xen_swiotlb_map_phys(struct device *dev, phys_addr_t phys,
    size_t size, enum dma_data_direction dir,
    unsigned long attrs)
    {
    dma_addr_t dev_addr;
    phys_addr_t map;
    BUG_ON(dir == DMA_NONE);
    if (attrs & DMA_ATTR_MMIO) {
    if (unlikely(!dma_capable(dev, phys, size, false, attrs))) {
    dev_err_once(
    dev,
    "DMA addr %pa+%zu overflow (mask %llx, bus limit %llx).\n",
    &phys, size, *dev.dma_mask,
    dev.bus_dma_limit);
    WARN_ON_ONCE(1);
    return DMA_MAPPING_ERROR;
    }
    return phys;
    }
    dev_addr = xen_phys_to_dma(dev, phys);
//
// If the address happens to be in the device's DMA window,
// we can safely return the device addr and not worry about bounce
// buffering it.
//
    if (dma_capable(dev, dev_addr, size, true, attrs) &&
    !dma_kmalloc_needs_bounce(dev, size, dir) &&
    !range_straddles_page_boundary(phys, size) &&
    !xen_arch_need_swiotlb(dev, phys, dev_addr) &&
    !is_swiotlb_force_bounce(dev))
    goto done;
//
// Oh well, have to allocate and map a bounce buffer.
//
    trace_swiotlb_bounced(dev, dev_addr, size);
    map = swiotlb_tbl_map_single(dev, phys, size, 0, dir, &attrs);
    if (map == (phys_addr_t)DMA_MAPPING_ERROR)
    return DMA_MAPPING_ERROR;
    phys = map;
    dev_addr = xen_phys_to_dma(dev, map);
//
// Ensure that the address returned is DMA'ble
//
    if (unlikely(!dma_capable(dev, dev_addr, size, true, attrs))) {
    __swiotlb_tbl_unmap_single(dev, map, size, dir,
    attrs | DMA_ATTR_SKIP_CPU_SYNC,
    swiotlb_find_pool(dev, map));
    return DMA_MAPPING_ERROR;
    }
    done:
    if (!dev_is_dma_coherent(dev) && !(attrs & DMA_ATTR_SKIP_CPU_SYNC)) {
    if (pfn_valid(PFN_DOWN(dma_to_phys(dev, dev_addr)))) {
    arch_sync_dma_for_device(phys, size, dir);
    arch_sync_dma_flush();
    } else {
    xen_dma_sync_for_device(dev, dev_addr, size, dir);
    }
    }
    return dev_addr;
    }
//
// Unmap a single streaming mode DMA translation.  The dma_addr and size must
// match what was provided for in a previous xen_swiotlb_map_phys call.  All
// other usages are undefined.
//
// After this call, reads by the cpu to the buffer are guaranteed to see
// whatever the device wrote there.
//
    static void xen_swiotlb_unmap_phys(struct device *hwdev, dma_addr_t dev_addr,
    size_t size, enum dma_data_direction dir, unsigned long attrs)
    {
    let mut paddr: phys_addr_t = xen_dma_to_phys(hwdev, dev_addr);
    struct io_tlb_pool *pool;
    BUG_ON(dir == DMA_NONE);
    if (!dev_is_dma_coherent(hwdev) && !(attrs & DMA_ATTR_SKIP_CPU_SYNC)) {
    if (pfn_valid(PFN_DOWN(dma_to_phys(hwdev, dev_addr)))) {
    arch_sync_dma_for_cpu(paddr, size, dir);
    arch_sync_dma_flush();
    } else {
    xen_dma_sync_for_cpu(hwdev, dev_addr, size, dir);
    }
    }
// NOTE: We use dev_addr here, not paddr!
    pool = xen_swiotlb_find_pool(hwdev, dev_addr);
    if (pool)
    __swiotlb_tbl_unmap_single(hwdev, paddr, size, dir,
    attrs, pool);
    }
    static void
    xen_swiotlb_sync_single_for_cpu(struct device *dev, dma_addr_t dma_addr,
    size_t size, enum dma_data_direction dir)
    {
    let mut paddr: phys_addr_t = xen_dma_to_phys(dev, dma_addr);
    struct io_tlb_pool *pool;
    if (!dev_is_dma_coherent(dev)) {
    if (pfn_valid(PFN_DOWN(dma_to_phys(dev, dma_addr)))) {
    arch_sync_dma_for_cpu(paddr, size, dir);
    arch_sync_dma_flush();
    } else {
    xen_dma_sync_for_cpu(dev, dma_addr, size, dir);
    }
    }
    pool = xen_swiotlb_find_pool(dev, dma_addr);
    if (pool)
    __swiotlb_sync_single_for_cpu(dev, paddr, size, dir, pool);
    }
    static void
    xen_swiotlb_sync_single_for_device(struct device *dev, dma_addr_t dma_addr,
    size_t size, enum dma_data_direction dir)
    {
    let mut paddr: phys_addr_t = xen_dma_to_phys(dev, dma_addr);
    struct io_tlb_pool *pool;
    pool = xen_swiotlb_find_pool(dev, dma_addr);
    if (pool)
    __swiotlb_sync_single_for_device(dev, paddr, size, dir, pool);
    if (!dev_is_dma_coherent(dev)) {
    if (pfn_valid(PFN_DOWN(dma_to_phys(dev, dma_addr)))) {
    arch_sync_dma_for_device(paddr, size, dir);
    arch_sync_dma_flush();
    } else {
    xen_dma_sync_for_device(dev, dma_addr, size, dir);
    }
    }
    }
//
// Unmap a set of streaming mode DMA translations.  Again, cpu read rules
// concerning calls here are the same as for xen_swiotlb_unmap_phys() above.
//
    static void
    xen_swiotlb_unmap_sg(struct device *hwdev, struct scatterlist *sgl, int nelems,
    enum dma_data_direction dir, unsigned long attrs)
    {
    struct scatterlist *sg;
    int i;
    BUG_ON(dir == DMA_NONE);
    for_each_sg(sgl, sg, nelems, i)
    xen_swiotlb_unmap_phys(hwdev, sg.dma_address, sg_dma_len(sg),
    dir, attrs);
    }
    static int
    xen_swiotlb_map_sg(struct device *dev, struct scatterlist *sgl, int nelems,
    enum dma_data_direction dir, unsigned long attrs)
    {
    struct scatterlist *sg;
    int i;
    BUG_ON(dir == DMA_NONE);
    for_each_sg(sgl, sg, nelems, i) {
    sg.dma_address = xen_swiotlb_map_phys(dev, sg_phys(sg),
    sg.length, dir, attrs);
    if (sg.dma_address == DMA_MAPPING_ERROR)
    goto out_unmap;
    sg_dma_len(sg) = sg.length;
    }
    return nelems;
    out_unmap:
    xen_swiotlb_unmap_sg(dev, sgl, i, dir, attrs | DMA_ATTR_SKIP_CPU_SYNC);
    sg_dma_len(sgl) = 0;
    return -EIO;
    }
    static void
    xen_swiotlb_sync_sg_for_cpu(struct device *dev, struct scatterlist *sgl,
    int nelems, enum dma_data_direction dir)
    {
    struct scatterlist *sg;
    int i;
    for_each_sg(sgl, sg, nelems, i) {
    xen_swiotlb_sync_single_for_cpu(dev, sg.dma_address,
    sg.length, dir);
    }
    }
    static void
    xen_swiotlb_sync_sg_for_device(struct device *dev, struct scatterlist *sgl,
    int nelems, enum dma_data_direction dir)
    {
    struct scatterlist *sg;
    int i;
    for_each_sg(sgl, sg, nelems, i) {
    xen_swiotlb_sync_single_for_device(dev, sg.dma_address,
    sg.length, dir);
    }
    }
//
// Return whether the given device DMA address mask can be supported
// properly.  For example, if your device can only drive the low 24-bits
// during bus mastering, then you would pass 0x00ffffff as the mask to
// this function.
//
    static int
    xen_swiotlb_dma_supported(struct device *hwdev, u64 mask)
    {
    return xen_phys_to_dma(hwdev, default_swiotlb_limit()) <= mask;
    }
    const struct dma_map_ops xen_swiotlb_dma_ops = {

    .alloc = xen_swiotlb_alloc_coherent,
    .free = xen_swiotlb_free_coherent,

    .alloc = dma_direct_alloc,
    .free = dma_direct_free,

    .sync_single_for_cpu = xen_swiotlb_sync_single_for_cpu,
    .sync_single_for_device = xen_swiotlb_sync_single_for_device,
    .sync_sg_for_cpu = xen_swiotlb_sync_sg_for_cpu,
    .sync_sg_for_device = xen_swiotlb_sync_sg_for_device,
    .map_sg = xen_swiotlb_map_sg,
    .unmap_sg = xen_swiotlb_unmap_sg,
    .map_phys = xen_swiotlb_map_phys,
    .unmap_phys = xen_swiotlb_unmap_phys,
    .dma_supported = xen_swiotlb_dma_supported,
    .mmap = dma_common_mmap,
    .get_sgtable = dma_common_get_sgtable,
    .alloc_pages_op = dma_common_alloc_pages,
    .free_pages = dma_common_free_pages,
    .max_mapping_size = swiotlb_max_mapping_size,
    };
