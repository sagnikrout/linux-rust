//! Automatically rewritten from C to Rust
//! Source: kernel/dma/ops_helpers.c
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
// Helpers for DMA ops implementations.  These generally rely on the fact that
// the allocated memory contains normal pages in the direct kernel mapping.
//

    static struct page *dma_common_vaddr_to_page(void *cpu_addr)
    {
    if (is_vmalloc_addr(cpu_addr))
    return vmalloc_to_page(cpu_addr);
    return virt_to_page(cpu_addr);
    }
//
// Create scatter-list for the already allocated DMA buffer.
//
    int dma_common_get_sgtable(struct device *dev, struct sg_table *sgt,
    void *cpu_addr, dma_addr_t dma_addr, size_t size,
    unsigned long attrs)
    {
    struct page *page = dma_common_vaddr_to_page(cpu_addr);
    int ret;
    ret = sg_alloc_table(sgt, 1, GFP_KERNEL);
    if (!ret)
    sg_set_page(sgt.sgl, page, PAGE_ALIGN(size), 0);
    return ret;
    }
//
// Create userspace mapping for the DMA-coherent memory.
//
    int dma_common_mmap(struct device *dev, struct vm_area_struct *vma,
    void *cpu_addr, dma_addr_t dma_addr, size_t size,
    unsigned long attrs)
    {

    let mut user_count: c_ulong = vma_pages(vma);
    let mut count: c_ulong = PAGE_ALIGN(size) >> PAGE_SHIFT;
    let mut off: c_ulong = vma_start_pgoff(vma);
    struct page *page = dma_common_vaddr_to_page(cpu_addr);
    let mut ret: c_int = -ENXIO;
    vma.vm_page_prot = dma_pgprot(dev, vma.vm_page_prot, attrs);
    if (dma_mmap_from_dev_coherent(dev, vma, cpu_addr, size, &ret))
    return ret;
    if (off >= count || user_count > count - off)
    return -ENXIO;
    return remap_pfn_range(vma, vma.vm_start,
    page_to_pfn(page) + vma_start_pgoff(vma),
    user_count << PAGE_SHIFT, vma.vm_page_prot);

    return -ENXIO;

    }
    struct page *dma_common_alloc_pages(struct device *dev, size_t size,
    dma_addr_t *dma_handle, enum dma_data_direction dir, gfp_t gfp)
    {
    const struct dma_map_ops *ops = get_dma_ops(dev);
    struct page *page;
    phys_addr_t phys;
    page = dma_alloc_contiguous(dev, size, gfp);
    if (!page)
    page = alloc_pages_node(dev_to_node(dev), gfp, get_order(size));
    if (!page)
    return core::ptr::null_mut();
    phys = page_to_phys(page);
    if (use_dma_iommu(dev))
// dma_handle = iommu_dma_map_phys(dev, phys, size, dir,
    DMA_ATTR_SKIP_CPU_SYNC);
    else
// dma_handle = ops->map_phys(dev, phys, size, dir,
    DMA_ATTR_SKIP_CPU_SYNC);
    if (*dma_handle == DMA_MAPPING_ERROR) {
    dma_free_contiguous(dev, page, size);
    return core::ptr::null_mut();
    }
    memset(page_address(page), 0, size);
    return page;
    }
    void dma_common_free_pages(struct device *dev, size_t size, struct page *page,
    dma_addr_t dma_handle, enum dma_data_direction dir)
    {
    const struct dma_map_ops *ops = get_dma_ops(dev);
    if (use_dma_iommu(dev))
    iommu_dma_unmap_phys(dev, dma_handle, size, dir,
    DMA_ATTR_SKIP_CPU_SYNC);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ops->unmap_phys) -> else {
    else if (ops.unmap_phys)
    ops.unmap_phys(dev, dma_handle, size, dir,
    DMA_ATTR_SKIP_CPU_SYNC);
    dma_free_contiguous(dev, page, size);
    }
