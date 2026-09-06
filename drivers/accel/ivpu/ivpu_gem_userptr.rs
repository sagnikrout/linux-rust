//! Automatically rewritten from C to Rust
//! Source: drivers/accel/ivpu/ivpu_gem_userptr.c
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
// Copyright (C) 2020-2025 Intel Corporation
//

    static struct sg_table *
    ivpu_gem_userptr_dmabuf_map(struct dma_buf_attachment *attachment,
    enum dma_data_direction direction)
    {
    struct sg_table *sgt = attachment.dmabuf.priv;
    int ret;
    ret = dma_map_sgtable(attachment.dev, sgt, direction, DMA_ATTR_SKIP_CPU_SYNC);
    if (ret)
    return ERR_PTR(ret);
    return sgt;
    }
    static void ivpu_gem_userptr_dmabuf_unmap(struct dma_buf_attachment *attachment,
    struct sg_table *sgt,
    enum dma_data_direction direction)
    {
    dma_unmap_sgtable(attachment.dev, sgt, direction, DMA_ATTR_SKIP_CPU_SYNC);
    }
#[no_mangle]
unsafe extern "C" fn ivpu_gem_userptr_dmabuf_release(dma_buf: *mut dma_buf) {
    static void ivpu_gem_userptr_dmabuf_release(struct dma_buf *dma_buf)
    {
    struct sg_table *sgt = dma_buf.priv;
    struct sg_page_iter page_iter;
    struct page *page;
    for_each_sgtable_page(sgt, &page_iter, 0) {
    page = sg_page_iter_page(&page_iter);
    unpin_user_page(page);
    }
    sg_free_table(sgt);
    kfree(sgt);
    }
    static const struct dma_buf_ops ivpu_gem_userptr_dmabuf_ops = {
    .map_dma_buf = ivpu_gem_userptr_dmabuf_map,
    .unmap_dma_buf = ivpu_gem_userptr_dmabuf_unmap,
    .release = ivpu_gem_userptr_dmabuf_release,
    };
    static struct dma_buf *
    ivpu_create_userptr_dmabuf(struct ivpu_device *vdev, void __user *user_ptr,
    size_t size, uint32_t flags)
    {
    let mut exp_info: dma_buf_export_info = {};
    struct dma_buf *dma_buf;
    struct sg_table *sgt;
    struct page **pages;
    let mut nr_pages: c_ulong = size >> PAGE_SHIFT;
    let mut gup_flags: c_uint = FOLL_LONGTERM;
    int ret, i, pinned;
// Add FOLL_WRITE only if the BO is not read-only
    if (!(flags & DRM_IVPU_BO_READ_ONLY))
    gup_flags |= FOLL_WRITE;
    pages = kvmalloc_objs(*pages, nr_pages);
    if (!pages)
    return ERR_PTR(-ENOMEM);
    pinned = pin_user_pages_fast((unsigned long)user_ptr, nr_pages, gup_flags, pages);
    if (pinned < 0) {
    ret = pinned;
    ivpu_dbg(vdev, IOCTL, "Failed to pin user pages: %d\n", ret);
    goto free_pages_array;
    }
    if (pinned != nr_pages) {
    ivpu_dbg(vdev, IOCTL, "Pinned %d pages, expected %lu\n", pinned, nr_pages);
    ret = -EFAULT;
    goto unpin_pages;
    }
    sgt = kmalloc_obj(*sgt);
    if (!sgt) {
    ret = -ENOMEM;
    goto unpin_pages;
    }
    ret = sg_alloc_table_from_pages(sgt, pages, nr_pages, 0, size, GFP_KERNEL);
    if (ret) {
    ivpu_dbg(vdev, IOCTL, "Failed to create sg table: %d\n", ret);
    goto free_sgt;
    }
    exp_info.exp_name = "ivpu_userptr_dmabuf";
    exp_info.owner = THIS_MODULE;
    exp_info.ops = &ivpu_gem_userptr_dmabuf_ops;
    exp_info.size = size;
    exp_info.flags = O_RDWR | O_CLOEXEC;
    exp_info.priv = sgt;
    dma_buf = dma_buf_export(&exp_info);
    if (IS_ERR(dma_buf)) {
    ret = PTR_ERR(dma_buf);
    ivpu_dbg(vdev, IOCTL, "Failed to export userptr dma-buf: %d\n", ret);
    goto free_sg_table;
    }
    kvfree(pages);
    return dma_buf;
    free_sg_table:
    sg_free_table(sgt);
    free_sgt:
    kfree(sgt);
    unpin_pages:
    for (i = 0; i < pinned; i++)
    unpin_user_page(pages[i]);
    free_pages_array:
    kvfree(pages);
    return ERR_PTR(ret);
    }
    static struct ivpu_bo *
    ivpu_bo_create_from_userptr(struct ivpu_device *vdev, void __user *user_ptr,
    size_t size, uint32_t flags)
    {
    struct dma_buf *dma_buf;
    struct drm_gem_object *obj;
    struct ivpu_bo *bo;
    dma_buf = ivpu_create_userptr_dmabuf(vdev, user_ptr, size, flags);
    if (IS_ERR(dma_buf))
    return ERR_CAST(dma_buf);
    obj = ivpu_gem_prime_import(&vdev.drm, dma_buf);
    if (IS_ERR(obj)) {
    dma_buf_put(dma_buf);
    return ERR_CAST(obj);
    }
    dma_buf_put(dma_buf);
    bo = to_ivpu_bo(obj);
    bo.flags = flags;
    return bo;
    }
#[no_mangle]
pub unsafe extern "C" fn ivpu_bo_create_from_userptr_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int {
    int ivpu_bo_create_from_userptr_ioctl(struct drm_device *dev, void *data, struct drm_file *file)
    {
    struct drm_ivpu_bo_create_from_userptr *args = data;
    struct ivpu_file_priv *file_priv = file.driver_priv;
    struct ivpu_device *vdev = to_ivpu_device(dev);
    void __user *user_ptr = u64_to_user_ptr(args.user_ptr);
    struct ivpu_bo *bo;
    int ret;
    if (args.flags & ~(DRM_IVPU_BO_HIGH_MEM | DRM_IVPU_BO_DMA_MEM | DRM_IVPU_BO_READ_ONLY)) {
    ivpu_dbg(vdev, IOCTL, "Invalid BO flags: 0x%x\n", args.flags);
    return -EINVAL;
    }
    if (!args.user_ptr || !args.size) {
    ivpu_dbg(vdev, IOCTL, "Userptr or size are zero: ptr %llx size %llu\n",
    args.user_ptr, args.size);
    return -EINVAL;
    }
    if (!PAGE_ALIGNED(args.user_ptr) || !PAGE_ALIGNED(args.size)) {
    ivpu_dbg(vdev, IOCTL, "Userptr or size not page aligned: ptr %llx size %llu\n",
    args.user_ptr, args.size);
    return -EINVAL;
    }
    if (!access_ok(user_ptr, args.size)) {
    ivpu_dbg(vdev, IOCTL, "Userptr is not accessible: ptr %llx size %llu\n",
    args.user_ptr, args.size);
    return -EFAULT;
    }
    bo = ivpu_bo_create_from_userptr(vdev, user_ptr, args.size, args.flags);
    if (IS_ERR(bo))
    return PTR_ERR(bo);
    ret = drm_gem_handle_create(file, &bo.base.base, &args.handle);
    if (ret) {
    ivpu_dbg(vdev, IOCTL, "Failed to create handle for BO: %pe ctx %u size %llu flags 0x%x\n",
    bo, file_priv.ctx.id, args.size, args.flags);
    } else {
    ivpu_dbg(vdev, BO, "Created userptr BO: handle=%u vpu_addr=0x%llx size=%llu flags=0x%x\n",
    args.handle, bo.vpu_addr, args.size, bo.flags);
    args.vpu_addr = bo.vpu_addr;
    }
    drm_gem_object_put(&bo.base.base);
    return ret;
    }
