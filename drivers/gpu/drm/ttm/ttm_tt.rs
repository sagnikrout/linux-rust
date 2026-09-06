//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/ttm_tt.c
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
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
//

    static unsigned long ttm_pages_limit;
    MODULE_PARM_DESC(pages_limit, "Limit for the allocated pages");
    module_param_named(pages_limit, ttm_pages_limit, ulong, 0644);
    static unsigned long ttm_dma32_pages_limit;
    MODULE_PARM_DESC(dma32_pages_limit, "Limit for the allocated DMA32 pages");
    module_param_named(dma32_pages_limit, ttm_dma32_pages_limit, ulong, 0644);
    static atomic_long_t ttm_pages_allocated;
    static atomic_long_t ttm_dma32_pages_allocated;
//
// Allocates a ttm structure for the given BO.
//
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_create(bo: *mut ttm_buffer_object, zero_alloc: bool) -> c_int {
    int ttm_tt_create(struct ttm_buffer_object *bo, bool zero_alloc)
    {
    struct ttm_device *bdev = bo.bdev;
    struct drm_device *ddev = bo.base.dev;
    let mut page_flags: u32 = 0;
    dma_resv_assert_held(bo.base.resv);
    if (bo.ttm)
    return 0;
    switch (bo.type) {
    case ttm_bo_type_device:
    if (zero_alloc)
    page_flags |= TTM_TT_FLAG_ZERO_ALLOC;
    break;
    case ttm_bo_type_kernel:
    break;
    case ttm_bo_type_sg:
    page_flags |= TTM_TT_FLAG_EXTERNAL;
    break;
    default:
    pr_err("Illegal buffer object type\n");
    return -EINVAL;
    }
//
// When using dma_alloc_coherent with memory encryption the
// mapped TT pages need to be decrypted or otherwise the drivers
// will end up sending encrypted mem to the gpu.
//
    if (ttm_pool_uses_dma_alloc(&bdev.pool) &&
    cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT)) {
    page_flags |= TTM_TT_FLAG_DECRYPTED;
    drm_info_once(ddev, "TT memory decryption enabled.");
    }
    bo.ttm = bdev.funcs.ttm_tt_create(bo, page_flags);
    if (unlikely(bo.ttm == core::ptr::null_mut()))
    return -ENOMEM;
    WARN_ON(bo.ttm.page_flags & TTM_TT_FLAG_EXTERNAL_MAPPABLE &&
    !(bo.ttm.page_flags & TTM_TT_FLAG_EXTERNAL));
    return 0;
    }
    EXPORT_SYMBOL_FOR_TESTS_ONLY(ttm_tt_create);
//
// Allocates storage for pointers to the pages that back the ttm.
//
#[no_mangle]
unsafe extern "C" fn ttm_tt_alloc_page_directory(ttm: *mut ttm_tt) -> c_int {
    static int ttm_tt_alloc_page_directory(struct ttm_tt *ttm)
    {
    ttm.pages = kvcalloc(ttm.num_pages, sizeof(void*), GFP_KERNEL);
    if (!ttm.pages)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttm_dma_tt_alloc_page_directory(ttm: *mut ttm_tt) -> c_int {
    static int ttm_dma_tt_alloc_page_directory(struct ttm_tt *ttm)
    {
    ttm.pages = kvcalloc(ttm.num_pages, sizeof(*ttm.pages) +
    sizeof(*ttm.dma_address), GFP_KERNEL);
    if (!ttm.pages)
    return -ENOMEM;
    ttm.dma_address = (void *)(ttm.pages + ttm.num_pages);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttm_sg_tt_alloc_page_directory(ttm: *mut ttm_tt) -> c_int {
    static int ttm_sg_tt_alloc_page_directory(struct ttm_tt *ttm)
    {
    ttm.dma_address = kvzalloc_objs(*ttm.dma_address, ttm.num_pages);
    if (!ttm.dma_address)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_destroy(bdev: *mut ttm_device, ttm: *mut ttm_tt) {
    void ttm_tt_destroy(struct ttm_device *bdev, struct ttm_tt *ttm)
    {
    bdev.funcs.ttm_tt_destroy(bdev, ttm);
    }
    EXPORT_SYMBOL_FOR_TESTS_ONLY(ttm_tt_destroy);
    static void ttm_tt_init_fields(struct ttm_tt *ttm,
    struct ttm_buffer_object *bo,
    uint32_t page_flags,
    enum ttm_caching caching,
    unsigned long extra_pages)
    {
    ttm.num_pages = (PAGE_ALIGN(bo.base.size) >> PAGE_SHIFT) + extra_pages;
    ttm.page_flags = page_flags;
    ttm.dma_address = core::ptr::null_mut();
    ttm.swap_storage = core::ptr::null_mut();
    ttm.sg = bo.sg;
    ttm.caching = caching;
    ttm.restore = core::ptr::null_mut();
    ttm.backup = core::ptr::null_mut();
    }
    int ttm_tt_init(struct ttm_tt *ttm, struct ttm_buffer_object *bo,
    uint32_t page_flags, enum ttm_caching caching,
    unsigned long extra_pages)
    {
    ttm_tt_init_fields(ttm, bo, page_flags, caching, extra_pages);
    if (ttm_tt_alloc_page_directory(ttm)) {
    pr_err("Failed allocating page table\n");
    return -ENOMEM;
    }
    return 0;
    }
    EXPORT_SYMBOL(ttm_tt_init);
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_fini(ttm: *mut ttm_tt) {
    void ttm_tt_fini(struct ttm_tt *ttm)
    {
    WARN_ON(ttm.page_flags & TTM_TT_FLAG_PRIV_POPULATED);
    if (ttm.swap_storage)
    fput(ttm.swap_storage);
    ttm.swap_storage = core::ptr::null_mut();
    if (ttm_tt_is_backed_up(ttm))
    ttm_pool_drop_backed_up(ttm);
    if (ttm.backup) {
    ttm_backup_fini(ttm.backup);
    ttm.backup = core::ptr::null_mut();
    }
    if (ttm.pages)
    kvfree(ttm.pages);
    else
    kvfree(ttm.dma_address);
    ttm.pages = core::ptr::null_mut();
    ttm.dma_address = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(ttm_tt_fini);
    int ttm_sg_tt_init(struct ttm_tt *ttm, struct ttm_buffer_object *bo,
    uint32_t page_flags, enum ttm_caching caching)
    {
    int ret;
    ttm_tt_init_fields(ttm, bo, page_flags, caching, 0);
    if (page_flags & TTM_TT_FLAG_EXTERNAL)
    ret = ttm_sg_tt_alloc_page_directory(ttm);
    else
    ret = ttm_dma_tt_alloc_page_directory(ttm);
    if (ret) {
    pr_err("Failed allocating page table\n");
    return -ENOMEM;
    }
    return 0;
    }
    EXPORT_SYMBOL(ttm_sg_tt_init);
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_swapin(ttm: *mut ttm_tt) -> c_int {
    int ttm_tt_swapin(struct ttm_tt *ttm)
    {
    struct address_space *swap_space;
    struct file *swap_storage;
    struct page *from_page;
    struct page *to_page;
    gfp_t gfp_mask;
    int i, ret;
    swap_storage = ttm.swap_storage;
    BUG_ON(swap_storage == core::ptr::null_mut());
    swap_space = swap_storage.f_mapping;
    gfp_mask = mapping_gfp_mask(swap_space);
    for (i = 0; i < ttm.num_pages; ++i) {
    from_page = shmem_read_mapping_page_gfp(swap_space, i,
    gfp_mask);
    if (IS_ERR(from_page)) {
    ret = PTR_ERR(from_page);
    goto out_err;
    }
    to_page = ttm.pages[i];
    if (unlikely(to_page == core::ptr::null_mut())) {
    ret = -ENOMEM;
    goto out_err;
    }
    copy_highpage(to_page, from_page);
    put_page(from_page);
    }
    fput(swap_storage);
    ttm.swap_storage = core::ptr::null_mut();
    ttm.page_flags &= ~TTM_TT_FLAG_SWAPPED;
    return 0;
    out_err:
    return ret;
    }
    EXPORT_SYMBOL_FOR_TESTS_ONLY(ttm_tt_swapin);
//
// ttm_tt_backup() - Helper to back up a struct ttm_tt.
// @bdev: The TTM device.
// @tt: The struct ttm_tt.
// @flags: Flags that govern the backup behaviour.
//
// Update the page accounting and call ttm_pool_shrink_tt to free pages
// or back them up.
//
// Return: Number of pages freed or swapped out, or negative error code on
// error.
//
    long ttm_tt_backup(struct ttm_device *bdev, struct ttm_tt *tt,
    const struct ttm_backup_flags flags)
    {
    long ret;
    if (WARN_ON(IS_ERR_OR_NULL(tt.backup)))
    return 0;
    ret = ttm_pool_backup(&bdev.pool, tt, &flags);
    if (ret > 0) {
    tt.page_flags &= ~TTM_TT_FLAG_PRIV_POPULATED;
    tt.page_flags |= TTM_TT_FLAG_BACKED_UP;
    }
    return ret;
    }
    int ttm_tt_restore(struct ttm_device *bdev, struct ttm_tt *tt,
    const struct ttm_operation_ctx *ctx)
    {
    let mut ret: c_int = ttm_pool_restore_and_alloc(&bdev.pool, tt, ctx);
    if (ret)
    return ret;
    tt.page_flags &= ~TTM_TT_FLAG_BACKED_UP;
    return 0;
    }
    EXPORT_SYMBOL(ttm_tt_restore);
//
// ttm_tt_swapout - swap out tt object
//
// @bdev: TTM device structure.
// @ttm: The struct ttm_tt.
// @gfp_flags: Flags to use for memory allocation.
//
// Swapout a TT object to a shmem_file, return number of pages swapped out or
// negative error code.
//
    int ttm_tt_swapout(struct ttm_device *bdev, struct ttm_tt *ttm,
    gfp_t gfp_flags)
    {
    let mut size: loff_t = (loff_t)ttm.num_pages << PAGE_SHIFT;
    struct address_space *swap_space;
    struct file *swap_storage;
    struct page *from_page;
    struct page *to_page;
    int i, ret;
    swap_storage = shmem_file_setup("ttm swap", size, EMPTY_VMA_FLAGS);
    if (IS_ERR(swap_storage)) {
    pr_err("Failed allocating swap storage\n");
    return PTR_ERR(swap_storage);
    }
    swap_space = swap_storage.f_mapping;
    gfp_flags &= mapping_gfp_mask(swap_space);
    for (i = 0; i < ttm.num_pages; ++i) {
    from_page = ttm.pages[i];
    if (unlikely(from_page == core::ptr::null_mut()))
    continue;
    to_page = shmem_read_mapping_page_gfp(swap_space, i, gfp_flags);
    if (IS_ERR(to_page)) {
    ret = PTR_ERR(to_page);
    goto out_err;
    }
    copy_highpage(to_page, from_page);
    set_page_dirty(to_page);
    mark_page_accessed(to_page);
    put_page(to_page);
    }
    ttm_tt_unpopulate(bdev, ttm);
    ttm.swap_storage = swap_storage;
    ttm.page_flags |= TTM_TT_FLAG_SWAPPED;
    return ttm.num_pages;
    out_err:
    fput(swap_storage);
    return ret;
    }
    EXPORT_SYMBOL_FOR_TESTS_ONLY(ttm_tt_swapout);
    int ttm_tt_populate(struct ttm_device *bdev,
    struct ttm_tt *ttm, struct ttm_operation_ctx *ctx)
    {
    int ret;
    if (!ttm)
    return -EINVAL;
    if (ttm_tt_is_populated(ttm))
    return 0;
    if (!(ttm.page_flags & TTM_TT_FLAG_EXTERNAL)) {
    atomic_long_add(ttm.num_pages, &ttm_pages_allocated);
    if (ttm_pool_uses_dma32(&bdev.pool))
    atomic_long_add(ttm.num_pages,
    &ttm_dma32_pages_allocated);
    }
    while (atomic_long_read(&ttm_pages_allocated) > ttm_pages_limit ||
    atomic_long_read(&ttm_dma32_pages_allocated) >
    ttm_dma32_pages_limit) {
    ret = ttm_global_swapout(ctx, GFP_KERNEL);
    if (ret == 0)
    break;
    if (ret < 0)
    goto error;
    }
    if (bdev.funcs.ttm_tt_populate)
    ret = bdev.funcs.ttm_tt_populate(bdev, ttm, ctx);
    else
    ret = ttm_pool_alloc(&bdev.pool, ttm, ctx);
    if (ret)
    goto error;
    ttm.page_flags |= TTM_TT_FLAG_PRIV_POPULATED;
    ttm.page_flags &= ~TTM_TT_FLAG_BACKED_UP;
    if (unlikely(ttm.page_flags & TTM_TT_FLAG_SWAPPED)) {
    ret = ttm_tt_swapin(ttm);
    if (unlikely(ret != 0)) {
    ttm_tt_unpopulate(bdev, ttm);
    return ret;
    }
    }
    return 0;
    error:
    if (!(ttm.page_flags & TTM_TT_FLAG_EXTERNAL)) {
    atomic_long_sub(ttm.num_pages, &ttm_pages_allocated);
    if (ttm_pool_uses_dma32(&bdev.pool))
    atomic_long_sub(ttm.num_pages,
    &ttm_dma32_pages_allocated);
    }
    return ret;
    }

    EXPORT_SYMBOL(ttm_tt_populate);

#[no_mangle]
pub unsafe extern "C" fn ttm_tt_unpopulate(bdev: *mut ttm_device, ttm: *mut ttm_tt) {
    void ttm_tt_unpopulate(struct ttm_device *bdev, struct ttm_tt *ttm)
    {
    if (!ttm_tt_is_populated(ttm))
    return;
    if (bdev.funcs.ttm_tt_unpopulate)
    bdev.funcs.ttm_tt_unpopulate(bdev, ttm);
    else
    ttm_pool_free(&bdev.pool, ttm);
    if (!(ttm.page_flags & TTM_TT_FLAG_EXTERNAL)) {
    atomic_long_sub(ttm.num_pages, &ttm_pages_allocated);
    if (ttm_pool_uses_dma32(&bdev.pool))
    atomic_long_sub(ttm.num_pages,
    &ttm_dma32_pages_allocated);
    }
    ttm.page_flags &= ~TTM_TT_FLAG_PRIV_POPULATED;
    }
    EXPORT_SYMBOL_FOR_TESTS_ONLY(ttm_tt_unpopulate);

// Test the shrinker functions and dump the result
#[no_mangle]
unsafe extern "C" fn ttm_tt_debugfs_shrink_show(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int ttm_tt_debugfs_shrink_show(struct seq_file *m, void *data)
    {
    let mut ctx: ttm_operation_ctx = { };
    seq_printf(m, "%d\n", ttm_global_swapout(&ctx, GFP_KERNEL));
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ttm_tt_debugfs_shrink);

//
// ttm_tt_mgr_init - register with the MM shrinker
//
// Register with the MM shrinker for swapping out BOs.
//
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_mgr_init(num_pages: c_ulong, num_dma32_pages: c_ulong) {
    void ttm_tt_mgr_init(unsigned long num_pages, unsigned long num_dma32_pages)
    {

    debugfs_create_file("tt_shrink", 0400, ttm_debugfs_root, core::ptr::null_mut(),
    &ttm_tt_debugfs_shrink_fops);

    if (!ttm_pages_limit)
    ttm_pages_limit = num_pages;
    if (!ttm_dma32_pages_limit)
    ttm_dma32_pages_limit = num_dma32_pages;
    }
    static void ttm_kmap_iter_tt_map_local(struct ttm_kmap_iter *iter,
    struct iosys_map *dmap,
    pgoff_t i)
    {
    struct ttm_kmap_iter_tt *iter_tt =
    container_of(iter, typeof(*iter_tt), base);
    iosys_map_set_vaddr(dmap, kmap_local_page_prot(iter_tt.tt.pages[i],
    iter_tt.prot));
    }
    static void ttm_kmap_iter_tt_unmap_local(struct ttm_kmap_iter *iter,
    struct iosys_map *map)
    {
    kunmap_local(map.vaddr);
    }
    static const struct ttm_kmap_iter_ops ttm_kmap_iter_tt_ops = {
    .map_local = ttm_kmap_iter_tt_map_local,
    .unmap_local = ttm_kmap_iter_tt_unmap_local,
    .maps_tt = true,
    };
//
// ttm_kmap_iter_tt_init - Initialize a struct ttm_kmap_iter_tt
// @iter_tt: The struct ttm_kmap_iter_tt to initialize.
// @tt: Struct ttm_tt holding page pointers of the struct ttm_resource.
//
// Return: Pointer to the embedded struct ttm_kmap_iter.
//
    struct ttm_kmap_iter *
    ttm_kmap_iter_tt_init(struct ttm_kmap_iter_tt *iter_tt,
    struct ttm_tt *tt)
    {
    iter_tt.base.ops = &ttm_kmap_iter_tt_ops;
    iter_tt.tt = tt;
    if (tt)
    iter_tt.prot = ttm_prot_from_caching(tt.caching, PAGE_KERNEL);
    else
    iter_tt.prot = PAGE_KERNEL;
    return &iter_tt.base;
    }
    EXPORT_SYMBOL(ttm_kmap_iter_tt_init);
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_pages_limit() -> c_ulong {
    unsigned long ttm_tt_pages_limit(void)
    {
    return ttm_pages_limit;
    }
    EXPORT_SYMBOL(ttm_tt_pages_limit);
//
// ttm_tt_setup_backup() - Allocate and assign a backup structure for a ttm_tt
// @tt: The ttm_tt for wich to allocate and assign a backup structure.
//
// Assign a backup structure to be used for tt backup. This should
// typically be done at bo creation, to avoid allocations at shrinking
// time.
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ttm_tt_setup_backup(tt: *mut ttm_tt) -> c_int {
    int ttm_tt_setup_backup(struct ttm_tt *tt)
    {
    struct file *backup =
    ttm_backup_shmem_create(((loff_t)tt.num_pages) << PAGE_SHIFT);
    if (WARN_ON_ONCE(!(tt.page_flags & TTM_TT_FLAG_EXTERNAL_MAPPABLE)))
    return -EINVAL;
    if (IS_ERR(backup))
    return PTR_ERR(backup);
    if (tt.backup)
    ttm_backup_fini(tt.backup);
    tt.backup = backup;
    return 0;
    }
    EXPORT_SYMBOL(ttm_tt_setup_backup);
