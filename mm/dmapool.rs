//! Automatically rewritten from C to Rust
//! Source: mm/dmapool.c
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
// DMA Pool allocator
//
// Copyright 2001 David Brownell
// Copyright 2007 Intel Corporation
// Author: Matthew Wilcox <willy@linux.intel.com>
//
// This allocator returns small blocks of a given size which are DMA-able by
// the given device.  It uses the dma_alloc_coherent page allocator to get
// new pages, then splits them up into blocks of the required size.
// Many older drivers still have their own code to do this.
//
// The current design of this allocator is fairly simple.  The pool is
// represented by the 'struct dma_pool' which keeps a doubly-linked list of
// allocated pages.  Each page in the page_list is split into blocks of at
// least 'size' bytes.  Free blocks are tracked in an unsorted singly-linked
// list of free blocks across all pages.  Used blocks aren't tracked, but we
// keep a count of how many are currently allocated from each page.
//

pub const DMAPOOL_DEBUG: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_block {
    pub next_block: *mut dma_block,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pool {
    pub page_list: list_head,
    pub lock: spinlock_t,
    pub next_block: *mut dma_block,
    pub nr_blocks: usize,
    pub nr_active: usize,
    pub nr_pages: usize,
    pub dev: *mut device,
    pub size: c_uint,
    pub allocation: c_uint,
    pub boundary: c_uint,
    pub node: c_int,
    pub name: [c_char; 32],
    pub pools: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_page {
    pub page_list: list_head,
    pub vaddr: *mut c_void,
    pub dma: dma_addr_t,
}

    static DEFINE_MUTEX(pools_lock);
    static DEFINE_MUTEX(pools_reg_lock);
#[no_mangle]
unsafe extern "C" fn pools_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t pools_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct dma_pool *pool;
    unsigned size;
    size = sysfs_emit(buf, "poolinfo - 0.1\n");
    mutex_lock(&pools_lock);
    list_for_each_entry(pool, &dev.dma_pools, pools) {
// per-pool info, no real statistics yet
    size += sysfs_emit_at(buf, size, "%-16s %4zu %4zu %4u %2zu\n",
    pool.name, pool.nr_active,
    pool.nr_blocks, pool.size,
    pool.nr_pages);
    }
    mutex_unlock(&pools_lock);
    return size;
    }
    static DEVICE_ATTR_RO(pools);

    static void pool_check_block(struct dma_pool *pool, struct dma_block *block,
    gfp_t mem_flags)
    {
    u8 *data = (void *)block;
    int i;
    for (i = sizeof(struct dma_block); i < pool.size; i++) {
    if (data[i] == POOL_POISON_FREED)
    continue;
    dev_err(pool.dev, "%s %s, %p (corrupted)\n", __func__,
    pool.name, block);
//
// Dump the first 4 bytes even if they are not
// POOL_POISON_FREED
//
    print_hex_dump(KERN_ERR, "", DUMP_PREFIX_OFFSET, 16, 1,
    data, pool.size, 1);
    break;
    }
    if (!want_init_on_alloc(mem_flags))
    memset(block, POOL_POISON_ALLOCATED, pool.size);
    }
    static struct dma_page *pool_find_page(struct dma_pool *pool, dma_addr_t dma)
    {
    struct dma_page *page;
    list_for_each_entry(page, &pool.page_list, page_list) {
    if (dma < page.dma)
    continue;
    if ((dma - page.dma) < pool.allocation)
    return page;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pool_block_err(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) -> bool {
    static bool pool_block_err(struct dma_pool *pool, void *vaddr, dma_addr_t dma)
    {
    struct dma_block *block = pool.next_block;
    struct dma_page *page;
    page = pool_find_page(pool, dma);
    if (!page) {
    dev_err(pool.dev, "%s %s, %p/%pad (bad dma)\n",
    __func__, pool.name, vaddr, &dma);
    return true;
    }
    while (block) {
    if (block != vaddr) {
    block = block.next_block;
    continue;
    }
    dev_err(pool.dev, "%s %s, dma %pad already free\n",
    __func__, pool.name, &dma);
    return true;
    }
    memset(vaddr, POOL_POISON_FREED, pool.size);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pool_init_page(pool: *mut dma_pool, page: *mut dma_page) {
    static void pool_init_page(struct dma_pool *pool, struct dma_page *page)
    {
    memset(page.vaddr, POOL_POISON_FREED, pool.allocation);
    }

    static void pool_check_block(struct dma_pool *pool, struct dma_block *block,
    gfp_t mem_flags)
    {
    }
#[no_mangle]
unsafe extern "C" fn pool_block_err(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) -> bool {
    static bool pool_block_err(struct dma_pool *pool, void *vaddr, dma_addr_t dma)
    {
    if (want_init_on_free())
    memset(vaddr, 0, pool.size);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pool_init_page(pool: *mut dma_pool, page: *mut dma_page) {
    static void pool_init_page(struct dma_pool *pool, struct dma_page *page)
    {
    }

    static struct dma_block *pool_block_pop(struct dma_pool *pool)
    {
    struct dma_block *block = pool.next_block;
    if (block) {
    pool.next_block = block.next_block;
    pool.nr_active++;
    }
    return block;
    }
    static void pool_block_push(struct dma_pool *pool, struct dma_block *block,
    dma_addr_t dma)
    {
    block.dma = dma;
    block.next_block = pool.next_block;
    pool.next_block = block;
    }
//
// dma_pool_create_node - Creates a pool of coherent DMA memory blocks.
// @name: name of pool, for diagnostics
// @dev: device that will be doing the DMA
// @size: size of the blocks in this pool.
// @align: alignment requirement for blocks; must be a power of two
// @boundary: returned blocks won't cross this power of two boundary
// @node: optional NUMA node to allocate structs 'dma_pool' and 'dma_page' on
// Context: not in_interrupt()
//
// Given one of these pools, dma_pool_alloc()
// may be used to allocate memory.  Such memory will all have coherent
// DMA mappings, accessible by the device and its driver without using
// cache flushing primitives.  The actual size of blocks allocated may be
// larger than requested because of alignment.
//
// If @boundary is nonzero, objects returned from dma_pool_alloc() won't
// cross that size boundary.  This is useful for devices which have
// addressing restrictions on individual DMA transfers, such as not crossing
// boundaries of 4KBytes.
//
// Return: a dma allocation pool with the requested characteristics, or
// %NULL if one can't be created.
//
    struct dma_pool *dma_pool_create_node(const char *name, struct device *dev,
    size_t size, size_t align, size_t boundary, int node)
    {
    struct dma_pool *retval;
    size_t allocation;
    bool empty;
    if (!dev)
    return core::ptr::null_mut();
    if (align == 0)
    align = 1;
#[no_mangle]
pub unsafe extern "C" fn if(1): align & (align -) -> else {
    else if (align & (align - 1))
    return core::ptr::null_mut();
    if (size == 0 || size > INT_MAX)
    return core::ptr::null_mut();
    if (size < sizeof(struct dma_block))
    size = sizeof(struct dma_block);
    size = ALIGN(size, align);
    allocation = max_t(size_t, size, PAGE_SIZE);
    if (!boundary)
    boundary = allocation;
#[no_mangle]
pub unsafe extern "C" fn if(1)): (boundary < size) || (boundary & (boundary -) -> else {
    else if ((boundary < size) || (boundary & (boundary - 1)))
    return core::ptr::null_mut();
    boundary = min(boundary, allocation);
    retval = kzalloc_node(sizeof(*retval), GFP_KERNEL, node);
    if (!retval)
    return retval;
    strscpy(retval.name, name, sizeof(retval.name));
    retval.dev = dev;
    INIT_LIST_HEAD(&retval.page_list);
    spin_lock_init(&retval.lock);
    retval.size = size;
    retval.boundary = boundary;
    retval.allocation = allocation;
    retval.node = node;
    INIT_LIST_HEAD(&retval.pools);
//
// pools_lock ensures that the ->dma_pools list does not get corrupted.
// pools_reg_lock ensures that there is not a race between
// dma_pool_create() and dma_pool_destroy() or within dma_pool_create()
// when the first invocation of dma_pool_create() failed on
// device_create_file() and the second assumes that it has been done (I
// know it is a short window).
//
    mutex_lock(&pools_reg_lock);
    mutex_lock(&pools_lock);
    empty = list_empty(&dev.dma_pools);
    list_add(&retval.pools, &dev.dma_pools);
    mutex_unlock(&pools_lock);
    if (empty) {
    int err;
    err = device_create_file(dev, &dev_attr_pools);
    if (err) {
    mutex_lock(&pools_lock);
    list_del(&retval.pools);
    mutex_unlock(&pools_lock);
    mutex_unlock(&pools_reg_lock);
    kfree(retval);
    return core::ptr::null_mut();
    }
    }
    mutex_unlock(&pools_reg_lock);
    return retval;
    }
    EXPORT_SYMBOL(dma_pool_create_node);
#[no_mangle]
unsafe extern "C" fn pool_initialise_page(pool: *mut dma_pool, page: *mut dma_page) {
    static void pool_initialise_page(struct dma_pool *pool, struct dma_page *page)
    {
    let mut next_boundary: c_uint = pool.boundary, offset = 0;
    struct dma_block *block, *first = core::ptr::null_mut(), *last = core::ptr::null_mut();
    pool_init_page(pool, page);
    while (offset + pool.size <= pool.allocation) {
    if (offset + pool.size > next_boundary) {
    offset = next_boundary;
    next_boundary += pool.boundary;
    continue;
    }
    block = page.vaddr + offset;
    block.dma = page.dma + offset;
    block.next_block = core::ptr::null_mut();
    if (last)
    last.next_block = block;
    else
    first = block;
    last = block;
    offset += pool.size;
    pool.nr_blocks++;
    }
    last.next_block = pool.next_block;
    pool.next_block = first;
    list_add(&page.page_list, &pool.page_list);
    pool.nr_pages++;
    }
    static struct dma_page *pool_alloc_page(struct dma_pool *pool, gfp_t mem_flags)
    {
    struct dma_page *page;
    page = kmalloc_node(sizeof(*page), mem_flags, pool.node);
    if (!page)
    return core::ptr::null_mut();
    page.vaddr = dma_alloc_coherent(pool.dev, pool.allocation,
    &page.dma, mem_flags);
    if (!page.vaddr) {
    kfree(page);
    return core::ptr::null_mut();
    }
    return page;
    }
//
// dma_pool_destroy - destroys a pool of dma memory blocks.
// @pool: dma pool that will be destroyed
// Context: !in_interrupt()
//
// Caller guarantees that no more memory from the pool is in use,
// and that nothing will try to use the pool after this call.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pool_destroy(pool: *mut dma_pool) {
    void dma_pool_destroy(struct dma_pool *pool)
    {
    struct dma_page *page, *tmp;
    bool empty, busy = false;
    if (unlikely(!pool))
    return;
    mutex_lock(&pools_reg_lock);
    mutex_lock(&pools_lock);
    list_del(&pool.pools);
    empty = list_empty(&pool.dev.dma_pools);
    mutex_unlock(&pools_lock);
    if (empty)
    device_remove_file(pool.dev, &dev_attr_pools);
    mutex_unlock(&pools_reg_lock);
    if (pool.nr_active) {
    dev_err(pool.dev, "%s %s busy\n", __func__, pool.name);
    busy = true;
    }
    list_for_each_entry_safe(page, tmp, &pool.page_list, page_list) {
    if (!busy)
    dma_free_coherent(pool.dev, pool.allocation,
    page.vaddr, page.dma);
    list_del(&page.page_list);
    kfree(page);
    }
    kfree(pool);
    }
    EXPORT_SYMBOL(dma_pool_destroy);
//
// dma_pool_alloc - get a block of coherent memory
// @pool: dma pool that will produce the block
// @mem_flags: GFP_* bitmask
// @handle: pointer to dma address of block
//
// Return: the kernel virtual address of a currently unused block,
// and reports its dma address through the handle.
// If such a memory block can't be allocated, %NULL is returned.
//
    void *dma_pool_alloc(struct dma_pool *pool, gfp_t mem_flags,
    dma_addr_t *handle)
    {
    struct dma_block *block;
    struct dma_page *page;
    unsigned long flags;
    might_alloc(mem_flags);
    spin_lock_irqsave(&pool.lock, flags);
    block = pool_block_pop(pool);
    if (!block) {
//
// pool_alloc_page() might sleep, so temporarily drop
// &pool->lock
//
    spin_unlock_irqrestore(&pool.lock, flags);
    page = pool_alloc_page(pool, mem_flags & (~__GFP_ZERO));
    if (!page)
    return core::ptr::null_mut();
    spin_lock_irqsave(&pool.lock, flags);
    pool_initialise_page(pool, page);
    block = pool_block_pop(pool);
    }
    spin_unlock_irqrestore(&pool.lock, flags);
// handle = block->dma;
    pool_check_block(pool, block, mem_flags);
    if (want_init_on_alloc(mem_flags))
    memset(block, 0, pool.size);
    return block;
    }
    EXPORT_SYMBOL(dma_pool_alloc);
//
// dma_pool_free - put block back into dma pool
// @pool: the dma pool holding the block
// @vaddr: virtual address of block
// @dma: dma address of block
//
// Caller promises neither device nor driver will again touch this block
// unless it is first re-allocated.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pool_free(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) {
    void dma_pool_free(struct dma_pool *pool, void *vaddr, dma_addr_t dma)
    {
    struct dma_block *block = vaddr;
    unsigned long flags;
    spin_lock_irqsave(&pool.lock, flags);
    if (!pool_block_err(pool, vaddr, dma)) {
    pool_block_push(pool, block, dma);
    pool.nr_active--;
    }
    spin_unlock_irqrestore(&pool.lock, flags);
    }
    EXPORT_SYMBOL(dma_pool_free);
//
// Managed DMA pool
//
#[no_mangle]
unsafe extern "C" fn dmam_pool_release(dev: *mut device, res: *mut c_void) {
    static void dmam_pool_release(struct device *dev, void *res)
    {
    struct dma_pool *pool = *(struct dma_pool **)res;
    dma_pool_destroy(pool);
    }
#[no_mangle]
unsafe extern "C" fn dmam_pool_match(dev: *mut device, res: *mut c_void, match_data: *mut c_void) -> c_int {
    static int dmam_pool_match(struct device *dev, void *res, void *match_data)
    {
    return *(struct dma_pool **)res == match_data;
    }
//
// dmam_pool_create - Managed dma_pool_create()
// @name: name of pool, for diagnostics
// @dev: device that will be doing the DMA
// @size: size of the blocks in this pool.
// @align: alignment requirement for blocks; must be a power of two
// @allocation: returned blocks won't cross this boundary (or zero)
//
// Managed dma_pool_create().  DMA pool created with this function is
// automatically destroyed on driver detach.
//
// Return: a managed dma allocation pool with the requested
// characteristics, or %NULL if one can't be created.
//
    struct dma_pool *dmam_pool_create(const char *name, struct device *dev,
    size_t size, size_t align, size_t allocation)
    {
    struct dma_pool **ptr, *pool;
    ptr = devres_alloc(dmam_pool_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return core::ptr::null_mut();
    pool = *ptr = dma_pool_create(name, dev, size, align, allocation);
    if (pool)
    devres_add(dev, ptr);
    else
    devres_free(ptr);
    return pool;
    }
    EXPORT_SYMBOL(dmam_pool_create);
//
// dmam_pool_destroy - Managed dma_pool_destroy()
// @pool: dma pool that will be destroyed
//
// Managed dma_pool_destroy().
//
#[no_mangle]
pub unsafe extern "C" fn dmam_pool_destroy(pool: *mut dma_pool) {
    void dmam_pool_destroy(struct dma_pool *pool)
    {
    struct device *dev = pool.dev;
    WARN_ON(devres_release(dev, dmam_pool_release, dmam_pool_match, pool));
    }
    EXPORT_SYMBOL(dmam_pool_destroy);
