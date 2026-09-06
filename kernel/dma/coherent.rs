//! Automatically rewritten from C to Rust
//! Source: kernel/dma/coherent.c
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
// Coherent per-device memory handling.
// Borrowed from i386
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_coherent_mem {
    pub virt_base: *mut c_void,
    pub device_base: dma_addr_t,
    pub pfn_base: c_ulong,
    pub size: c_int,
    pub bitmap: *mut c_ulong,
    pub spinlock: spinlock_t,
    pub use_dev_dma_pfn_offset: bool,
}

    static inline struct dma_coherent_mem *dev_get_coherent_memory(struct device *dev)
    {
    if (dev && dev.dma_mem)
    return dev.dma_mem;
    return core::ptr::null_mut();
    }
    static inline dma_addr_t dma_get_device_base(struct device *dev,
    struct dma_coherent_mem *mem)
    {
    if (mem.use_dev_dma_pfn_offset)
    return phys_to_dma(dev, PFN_PHYS(mem.pfn_base));
    return mem.device_base;
    }
    static struct dma_coherent_mem *dma_init_coherent_memory(phys_addr_t phys_addr,
    dma_addr_t device_addr, size_t size, bool use_dma_pfn_offset)
    {
    struct dma_coherent_mem *dma_mem;
    let mut pages: c_int = size >> PAGE_SHIFT;
    void *mem_base;
    if (!size)
    return ERR_PTR(-EINVAL);
    mem_base = memremap(phys_addr, size, MEMREMAP_WC);
    if (!mem_base)
    return ERR_PTR(-EINVAL);
    dma_mem = kzalloc_obj(struct dma_coherent_mem);
    if (!dma_mem)
    goto out_unmap_membase;
    dma_mem.bitmap = bitmap_zalloc(pages, GFP_KERNEL);
    if (!dma_mem.bitmap)
    goto out_free_dma_mem;
    dma_mem.virt_base = mem_base;
    dma_mem.device_base = device_addr;
    dma_mem.pfn_base = PFN_DOWN(phys_addr);
    dma_mem.size = pages;
    dma_mem.use_dev_dma_pfn_offset = use_dma_pfn_offset;
    spin_lock_init(&dma_mem.spinlock);
    return dma_mem;
    out_free_dma_mem:
    kfree(dma_mem);
    out_unmap_membase:
    memunmap(mem_base);
    pr_err("Reserved memory: failed to init DMA memory pool at %pa, size %zu KiB\n",
    &phys_addr, size / SZ_1K);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn _dma_release_coherent_memory(mem: *mut dma_coherent_mem) {
    static void _dma_release_coherent_memory(struct dma_coherent_mem *mem)
    {
    if (!mem)
    return;
    memunmap(mem.virt_base);
    bitmap_free(mem.bitmap);
    kfree(mem);
    }
    static int dma_assign_coherent_memory(struct device *dev,
    struct dma_coherent_mem *mem)
    {
    if (!dev)
    return -ENODEV;
    if (dev.dma_mem)
    return -EBUSY;
    dev.dma_mem = mem;
    return 0;
    }
//
// Declare a region of memory to be handed out by dma_alloc_coherent() when it
// is asked for coherent memory for this device.  This shall only be used
// from platform code, usually based on the device tree description.
//
// phys_addr is the CPU physical address to which the memory is currently
// assigned (this will be ioremapped so the CPU can access the region).
//
// device_addr is the DMA address the device needs to be programmed with to
// actually address this memory (this will be handed out as the dma_addr_t in
// dma_alloc_coherent()).
//
// size is the size of the area (must be a multiple of PAGE_SIZE).
//
// As a simplification for the platforms, only *one* such region of memory may
// be declared per device.
//
    int dma_declare_coherent_memory(struct device *dev, phys_addr_t phys_addr,
    dma_addr_t device_addr, size_t size)
    {
    struct dma_coherent_mem *mem;
    int ret;
    mem = dma_init_coherent_memory(phys_addr, device_addr, size, false);
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    ret = dma_assign_coherent_memory(dev, mem);
    if (ret)
    _dma_release_coherent_memory(mem);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dma_release_coherent_memory(dev: *mut device) {
    void dma_release_coherent_memory(struct device *dev)
    {
    if (dev) {
    _dma_release_coherent_memory(dev.dma_mem);
    dev.dma_mem = core::ptr::null_mut();
    }
    }
    static void *__dma_alloc_from_coherent(struct device *dev,
    struct dma_coherent_mem *mem,
    ssize_t size, dma_addr_t *dma_handle)
    {
    let mut order: c_int = get_order(size);
    unsigned long flags;
    int pageno;
    void *ret;
    spin_lock_irqsave(&mem.spinlock, flags);
    if (unlikely(size > ((dma_addr_t)mem.size << PAGE_SHIFT)))
    goto err;
    pageno = bitmap_find_free_region(mem.bitmap, mem.size, order);
    if (unlikely(pageno < 0))
    goto err;
//
// Memory was found in the coherent area.
//
// dma_handle = dma_get_device_base(dev, mem) +
    ((dma_addr_t)pageno << PAGE_SHIFT);
    ret = mem.virt_base + ((dma_addr_t)pageno << PAGE_SHIFT);
    spin_unlock_irqrestore(&mem.spinlock, flags);
    memset(ret, 0, size);
    return ret;
    err:
    spin_unlock_irqrestore(&mem.spinlock, flags);
    return core::ptr::null_mut();
    }
//
// dma_alloc_from_dev_coherent() - allocate memory from device coherent pool
// @dev:	device from which we allocate memory
// @size:	size of requested memory area
// @dma_handle:	This will be filled with the correct dma handle
// @ret:	This pointer will be filled with the virtual address
// to allocated area.
//
// This function should be only called from per-arch dma_alloc_coherent()
// to support allocation from per-device coherent memory pools.
//
// Returns 0 if dma_alloc_coherent should continue with allocating from
// generic memory areas, or !0 if dma_alloc_coherent should return @ret.
//
    int dma_alloc_from_dev_coherent(struct device *dev, ssize_t size,
    dma_addr_t *dma_handle, void **ret)
    {
    struct dma_coherent_mem *mem = dev_get_coherent_memory(dev);
    if (!mem)
    return 0;
// ret = __dma_alloc_from_coherent(dev, mem, size, dma_handle);
    return 1;
    }
    static int __dma_release_from_coherent(struct dma_coherent_mem *mem,
    int order, void *vaddr)
    {
    if (mem && vaddr >= mem.virt_base && vaddr <
    (mem.virt_base + ((dma_addr_t)mem.size << PAGE_SHIFT))) {
    let mut page: c_int = (vaddr - mem.virt_base) >> PAGE_SHIFT;
    unsigned long flags;
    spin_lock_irqsave(&mem.spinlock, flags);
    bitmap_release_region(mem.bitmap, page, order);
    spin_unlock_irqrestore(&mem.spinlock, flags);
    return 1;
    }
    return 0;
    }
//
// dma_release_from_dev_coherent() - free memory to device coherent memory pool
// @dev:	device from which the memory was allocated
// @order:	the order of pages allocated
// @vaddr:	virtual address of allocated pages
//
// This checks whether the memory was allocated from the per-device
// coherent memory pool and if so, releases that memory.
//
// Returns 1 if we correctly released the memory, or 0 if the caller should
// proceed with releasing memory from generic pools.
//
#[no_mangle]
pub unsafe extern "C" fn dma_release_from_dev_coherent(dev: *mut device, order: c_int, vaddr: *mut c_void) -> c_int {
    int dma_release_from_dev_coherent(struct device *dev, int order, void *vaddr)
    {
    struct dma_coherent_mem *mem = dev_get_coherent_memory(dev);
    return __dma_release_from_coherent(mem, order, vaddr);
    }
    static int __dma_mmap_from_coherent(struct dma_coherent_mem *mem,
    struct vm_area_struct *vma, void *vaddr, size_t size, int *ret)
    {
    if (mem && vaddr >= mem.virt_base && vaddr + size <=
    (mem.virt_base + ((dma_addr_t)mem.size << PAGE_SHIFT))) {
    let mut pgoff_start: pgoff_t = vma_start_pgoff(vma);
    let mut pgoff_end: pgoff_t = vma_end_pgoff(vma);
    let mut start: c_int = (vaddr - mem.virt_base) >> PAGE_SHIFT;
    let mut user_count: c_ulong = vma_pages(vma);
    let mut count: c_int = PAGE_ALIGN(size) >> PAGE_SHIFT;
// ret = -ENXIO;
    if (pgoff_start < count && pgoff_end <= count) {
    let mut pfn: c_ulong = mem.pfn_base + start + pgoff_start;
// ret = remap_pfn_range(vma, vma->vm_start, pfn,
    user_count << PAGE_SHIFT,
    vma.vm_page_prot);
    }
    return 1;
    }
    return 0;
    }
//
// dma_mmap_from_dev_coherent() - mmap memory from the device coherent pool
// @dev:	device from which the memory was allocated
// @vma:	vm_area for the userspace memory
// @vaddr:	cpu address returned by dma_alloc_from_dev_coherent
// @size:	size of the memory buffer allocated
// @ret:	result from remap_pfn_range()
//
// This checks whether the memory was allocated from the per-device
// coherent memory pool and if so, maps that memory to the provided vma.
//
// Returns 1 if @vaddr belongs to the device coherent pool and the caller
// should return @ret, or 0 if they should proceed with mapping memory from
// generic areas.
//
    int dma_mmap_from_dev_coherent(struct device *dev, struct vm_area_struct *vma,
    void *vaddr, size_t size, int *ret)
    {
    struct dma_coherent_mem *mem = dev_get_coherent_memory(dev);
    return __dma_mmap_from_coherent(mem, vma, vaddr, size, ret);
    }

    static struct dma_coherent_mem *dma_coherent_default_memory __ro_after_init;
    void *dma_alloc_from_global_coherent(struct device *dev, ssize_t size,
    dma_addr_t *dma_handle)
    {
    if (!dma_coherent_default_memory)
    return core::ptr::null_mut();
    return __dma_alloc_from_coherent(dev, dma_coherent_default_memory, size,
    dma_handle);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_release_from_global_coherent(order: c_int, vaddr: *mut c_void) -> c_int {
    int dma_release_from_global_coherent(int order, void *vaddr)
    {
    if (!dma_coherent_default_memory)
    return 0;
    return __dma_release_from_coherent(dma_coherent_default_memory, order,
    vaddr);
    }
    int dma_mmap_from_global_coherent(struct vm_area_struct *vma, void *vaddr,
    size_t size, int *ret)
    {
    if (!dma_coherent_default_memory)
    return 0;
    return __dma_mmap_from_coherent(dma_coherent_default_memory, vma,
    vaddr, size, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_init_global_coherent(phys_addr: phys_addr_t, size: usize) -> c_int {
    int dma_init_global_coherent(phys_addr_t phys_addr, size_t size)
    {
    struct dma_coherent_mem *mem;
    mem = dma_init_coherent_memory(phys_addr, phys_addr, size, true);
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    dma_coherent_default_memory = mem;
    pr_info("DMA: default coherent area is set\n");
    return 0;
    }

//
// Support for reserved memory regions defined in device tree
//

    static phys_addr_t dma_reserved_default_memory_base __initdata;
    static phys_addr_t dma_reserved_default_memory_size __initdata;

#[no_mangle]
unsafe extern "C" fn rmem_dma_device_init(rmem: *mut reserved_mem, dev: *mut device) -> c_int {
    static int rmem_dma_device_init(struct reserved_mem *rmem, struct device *dev)
    {
    struct dma_coherent_mem *mem = rmem.priv;
    if (!mem) {
    mem = dma_init_coherent_memory(rmem.base, rmem.base,
    rmem.size, true);
    if (IS_ERR(mem))
    return PTR_ERR(mem);
    rmem.priv = mem;
    }
// Warn if the device potentially can't use the reserved memory
    if (mem.device_base + rmem.size - 1 >
    min_not_zero(dev.coherent_dma_mask, dev.bus_dma_limit))
    dev_warn(dev, "reserved memory is beyond device's set DMA address range\n");
    dma_assign_coherent_memory(dev, mem);
    return 0;
    }
    static void rmem_dma_device_release(struct reserved_mem *rmem,
    struct device *dev)
    {
    if (dev)
    dev.dma_mem = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn rmem_dma_setup(node: c_ulong, rmem: *mut reserved_mem) -> int __init {
    static int __init rmem_dma_setup(unsigned long node, struct reserved_mem *rmem)
    {
    if (of_get_flat_dt_prop(node, "reusable", core::ptr::null_mut()))
    return -ENODEV;

    if (!of_get_flat_dt_prop(node, "no-map", core::ptr::null_mut())) {
    pr_err("Reserved memory: regions without no-map are not yet supported\n");
    return -EINVAL;
    }

    if (of_get_flat_dt_prop(node, "linux,dma-default", core::ptr::null_mut())) {
    WARN(dma_reserved_default_memory_size,
    "Reserved memory: region for default DMA coherent area is redefined\n");
    dma_reserved_default_memory_base = rmem.base;
    dma_reserved_default_memory_size = rmem.size;
    }

    pr_info("Reserved memory: created DMA memory pool at %pa, size %llu KiB\n",
    &rmem.base, (unsigned long long)(rmem.size / SZ_1K));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dma_init_reserved_memory() -> int __init {
    static int __init dma_init_reserved_memory(void)
    {
    if (!dma_reserved_default_memory_size)
    return -ENOMEM;
    return dma_init_global_coherent(dma_reserved_default_memory_base,
    dma_reserved_default_memory_size);
    }
    core_initcall(dma_init_reserved_memory);

    static const struct reserved_mem_ops rmem_dma_ops = {
    .node_init	= rmem_dma_setup,
    .device_init	= rmem_dma_device_init,
    .device_release	= rmem_dma_device_release,
    };
    RESERVEDMEM_OF_DECLARE(dma, "shared-dma-pool", &rmem_dma_ops);
