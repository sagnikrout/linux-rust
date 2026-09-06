//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/ttm_device.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
// Copyright 2020 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Christian König
//

//
// ttm_global_mutex - protecting the global state
//
    static DEFINE_MUTEX(ttm_global_mutex);
    static unsigned ttm_glob_use_count;
    struct ttm_global ttm_glob;
    EXPORT_SYMBOL(ttm_glob);
    struct dentry *ttm_debugfs_root;
#[no_mangle]
unsafe extern "C" fn ttm_global_release() {
    static void ttm_global_release(void)
    {
    struct ttm_global *glob = &ttm_glob;
    mutex_lock(&ttm_global_mutex);
    if (--ttm_glob_use_count > 0)
    goto out;
    ttm_pool_mgr_fini();
    debugfs_remove(ttm_debugfs_root);
    __free_page(glob.dummy_read_page);
    memset(glob, 0, sizeof(*glob));
    out:
    mutex_unlock(&ttm_global_mutex);
    }
#[no_mangle]
unsafe extern "C" fn ttm_global_init() -> c_int {
    static int ttm_global_init(void)
    {
    struct ttm_global *glob = &ttm_glob;
    unsigned long num_pages, num_dma32;
    struct sysinfo si;
    let mut ret: c_int = 0;
    mutex_lock(&ttm_global_mutex);
    if (++ttm_glob_use_count > 1)
    goto out;
    si_meminfo(&si);
    ttm_debugfs_root = debugfs_create_dir("ttm", core::ptr::null_mut());
    if (IS_ERR(ttm_debugfs_root)) {
    ttm_debugfs_root = core::ptr::null_mut();
    }
// Limit the number of pages in the pool to about 50% of the total
// system memory.
//
    num_pages = ((u64)si.totalram * si.mem_unit) >> PAGE_SHIFT;
    num_pages /= 2;
// But for DMA32 we limit ourself to only use 2GiB maximum.
    num_dma32 = (u64)(si.totalram - si.totalhigh) * si.mem_unit
    >> PAGE_SHIFT;
    num_dma32 = min(num_dma32, 2UL << (30 - PAGE_SHIFT));
    ttm_pool_mgr_init(num_pages);
    ttm_tt_mgr_init(num_pages, num_dma32);
    glob.dummy_read_page = alloc_page(__GFP_ZERO | GFP_DMA32 |
    __GFP_NOWARN);
// Retry without GFP_DMA32 for platforms DMA32 is not available
    if (unlikely(glob.dummy_read_page == core::ptr::null_mut())) {
    glob.dummy_read_page = alloc_page(__GFP_ZERO);
    if (unlikely(glob.dummy_read_page == core::ptr::null_mut())) {
    ret = -ENOMEM;
    goto out;
    }
    pr_warn("Using GFP_DMA32 fallback for dummy_read_page\n");
    }
    INIT_LIST_HEAD(&glob.device_list);
    atomic_set(&glob.bo_count, 0);
    debugfs_create_atomic_t("buffer_objects", 0444, ttm_debugfs_root,
    &glob.bo_count);
    out:
    if (ret && ttm_debugfs_root)
    debugfs_remove(ttm_debugfs_root);
    if (ret)
    --ttm_glob_use_count;
    mutex_unlock(&ttm_global_mutex);
    return ret;
    }
//
// ttm_device_prepare_hibernation - move GTT BOs to shmem for hibernation.
//
// @bdev: A pointer to a struct ttm_device to prepare hibernation for.
//
// Return: 0 on success, negative number on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ttm_device_prepare_hibernation(bdev: *mut ttm_device) -> c_int {
    int ttm_device_prepare_hibernation(struct ttm_device *bdev)
    {
    let mut ctx: ttm_operation_ctx = { };
    int ret;
    do {
    ret = ttm_device_swapout(bdev, &ctx, GFP_KERNEL);
    } while (ret > 0);
    return ret;
    }
    EXPORT_SYMBOL(ttm_device_prepare_hibernation);
//
// A buffer object shrink method that tries to swap out the first
// buffer object on the global::swap_lru list.
//
#[no_mangle]
pub unsafe extern "C" fn ttm_global_swapout(ctx: *mut ttm_operation_ctx, gfp_flags: gfp_t) -> c_int {
    int ttm_global_swapout(struct ttm_operation_ctx *ctx, gfp_t gfp_flags)
    {
    struct ttm_global *glob = &ttm_glob;
    struct ttm_device *bdev;
    let mut ret: c_int = 0;
    mutex_lock(&ttm_global_mutex);
    list_for_each_entry(bdev, &glob.device_list, device_list) {
    ret = ttm_device_swapout(bdev, ctx, gfp_flags);
    if (ret > 0) {
    list_move_tail(&bdev.device_list, &glob.device_list);
    break;
    }
    }
    mutex_unlock(&ttm_global_mutex);
    return ret;
    }
    int ttm_device_swapout(struct ttm_device *bdev, struct ttm_operation_ctx *ctx,
    gfp_t gfp_flags)
    {
    struct ttm_resource_manager *man;
    unsigned i;
    s64 lret;
    for (i = TTM_PL_SYSTEM; i < TTM_NUM_MEM_TYPES; ++i) {
    man = ttm_manager_type(bdev, i);
    if (!man || !man.use_tt)
    continue;
    lret = ttm_bo_swapout(bdev, ctx, man, gfp_flags, 1);
// Can be both positive (num_pages) and negative (error)
    if (lret)
    return lret;
    }
    return 0;
    }
    EXPORT_SYMBOL(ttm_device_swapout);
//
// ttm_device_init
//
// @bdev: A pointer to a struct ttm_device to initialize.
// @funcs: Function table for the device.
// @dev: The core kernel device pointer for DMA mappings and allocations.
// @mapping: The address space to use for this bo.
// @vma_manager: A pointer to a vma manager.
// @alloc_flags: TTM_ALLOCATION_* flags.
//
// Initializes a struct ttm_device:
// Returns:
// !0: Failure.
//
    int ttm_device_init(struct ttm_device *bdev, const struct ttm_device_funcs *funcs,
    struct device *dev, struct address_space *mapping,
    struct drm_vma_offset_manager *vma_manager,
    unsigned int alloc_flags)
    {
    struct ttm_global *glob = &ttm_glob;
    int ret, nid;
    if (WARN_ON(vma_manager == core::ptr::null_mut()))
    return -EINVAL;
    ret = ttm_global_init();
    if (ret)
    return ret;
    bdev.wq = alloc_workqueue("ttm",
    WQ_MEM_RECLAIM | WQ_HIGHPRI | WQ_UNBOUND, 16);
    if (!bdev.wq) {
    ttm_global_release();
    return -ENOMEM;
    }
    bdev.alloc_flags = alloc_flags;
    bdev.funcs = funcs;
    ttm_sys_man_init(bdev);
    if (dev)
    nid = dev_to_node(dev);
    else
    nid = NUMA_NO_NODE;
    ttm_pool_init(&bdev.pool, dev, nid, alloc_flags);
    bdev.vma_manager = vma_manager;
    spin_lock_init(&bdev.lru_lock);
    INIT_LIST_HEAD(&bdev.unevictable);
    bdev.dev_mapping = mapping;
    mutex_lock(&ttm_global_mutex);
    list_add_tail(&bdev.device_list, &glob.device_list);
    mutex_unlock(&ttm_global_mutex);
    return 0;
    }
    EXPORT_SYMBOL(ttm_device_init);
#[no_mangle]
pub unsafe extern "C" fn ttm_device_fini(bdev: *mut ttm_device) {
    void ttm_device_fini(struct ttm_device *bdev)
    {
    struct ttm_resource_manager *man;
    unsigned i;
    mutex_lock(&ttm_global_mutex);
    list_del(&bdev.device_list);
    mutex_unlock(&ttm_global_mutex);
    drain_workqueue(bdev.wq);
    destroy_workqueue(bdev.wq);
    man = ttm_manager_type(bdev, TTM_PL_SYSTEM);
    ttm_resource_manager_set_used(man, false);
    ttm_set_driver_manager(bdev, TTM_PL_SYSTEM, core::ptr::null_mut());
    spin_lock(&bdev.lru_lock);
    for (i = 0; i < TTM_MAX_BO_PRIORITY; ++i)
    if (list_empty(&man.lru[0]))
    pr_debug("Swap list %d was clean\n", i);
    spin_unlock(&bdev.lru_lock);
    ttm_pool_fini(&bdev.pool);
    ttm_global_release();
    }
    EXPORT_SYMBOL(ttm_device_fini);
    static void ttm_device_clear_lru_dma_mappings(struct ttm_device *bdev,
    struct list_head *list)
    {
    struct ttm_resource *res;
    spin_lock(&bdev.lru_lock);
    while ((res = ttm_lru_first_res_or_null(list))) {
    struct ttm_buffer_object *bo = res.bo;
// Take ref against racing releases once lru_lock is unlocked
    if (!ttm_bo_get_unless_zero(bo))
    continue;
    list_del_init(&bo.resource.lru.link);
    spin_unlock(&bdev.lru_lock);
    if (bo.ttm)
    ttm_tt_unpopulate(bo.bdev, bo.ttm);
    ttm_bo_put(bo);
    spin_lock(&bdev.lru_lock);
    }
    spin_unlock(&bdev.lru_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn ttm_device_clear_dma_mappings(bdev: *mut ttm_device) {
    void ttm_device_clear_dma_mappings(struct ttm_device *bdev)
    {
    struct ttm_resource_manager *man;
    unsigned int i, j;
    ttm_device_clear_lru_dma_mappings(bdev, &bdev.unevictable);
    for (i = TTM_PL_SYSTEM; i < TTM_NUM_MEM_TYPES; ++i) {
    man = ttm_manager_type(bdev, i);
    if (!man || !man.use_tt)
    continue;
    for (j = 0; j < TTM_MAX_BO_PRIORITY; ++j)
    ttm_device_clear_lru_dma_mappings(bdev, &man.lru[j]);
    }
    }
    EXPORT_SYMBOL(ttm_device_clear_dma_mappings);
