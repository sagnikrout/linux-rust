//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/virtio/virtgpu_object.c
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


//
// Copyright (C) 2015 Red Hat, Inc.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

    let mut virtio_gpu_virglrenderer_workaround: static int = 1;
    module_param_named(virglhack, virtio_gpu_virglrenderer_workaround, int, 0400);
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_resource_id_get(vgdev: *mut virtio_gpu_device, resid: *mut u32) -> c_int {
    int virtio_gpu_resource_id_get(struct virtio_gpu_device *vgdev, uint32_t *resid)
    {
    if (virtio_gpu_virglrenderer_workaround) {
//
// Hack to avoid re-using resource IDs.
//
// virglrenderer versions up to (and including) 0.7.0
// can't deal with that.  virglrenderer commit
// "f91a9dd35715 Fix unlinking resources from hash
// table." (Feb 2019) fixes the bug.
//
    let mut seqno: static atomic_t = ATOMIC_INIT(0);
    let mut handle: c_int = atomic_inc_return(&seqno);
// resid = handle + 1;
    } else {
    let mut handle: c_int = ida_alloc(&vgdev.resource_ida, GFP_KERNEL);
    if (handle < 0)
    return handle;
// resid = handle + 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtio_gpu_resource_id_put(vgdev: *mut virtio_gpu_device, id: u32) {
    static void virtio_gpu_resource_id_put(struct virtio_gpu_device *vgdev, uint32_t id)
    {
    if (!virtio_gpu_virglrenderer_workaround)
    ida_free(&vgdev.resource_ida, id - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_remove_from_restore_list(bo: *mut virtio_gpu_object) {
    void virtio_gpu_remove_from_restore_list(struct virtio_gpu_object *bo)
    {
    struct virtio_gpu_device *vgdev = bo.base.base.dev.dev_private;
    mutex_lock(&vgdev.obj_restore_lock);
    list_del_init(&bo.restore_node);
    mutex_unlock(&vgdev.obj_restore_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_cleanup_object(bo: *mut virtio_gpu_object) {
    void virtio_gpu_cleanup_object(struct virtio_gpu_object *bo)
    {
    struct virtio_gpu_device *vgdev = bo.base.base.dev.dev_private;
    virtio_gpu_resource_id_put(vgdev, bo.hw_res_handle);
    if (virtio_gpu_is_shmem(bo)) {
    drm_gem_shmem_free(&bo.base);
    } else if (virtio_gpu_is_vram(bo)) {
    struct virtio_gpu_object_vram *vram = to_virtio_gpu_vram(bo);
    spin_lock(&vgdev.host_visible_lock);
    if (drm_mm_node_allocated(&vram.vram_node))
    drm_mm_remove_node(&vram.vram_node);
    spin_unlock(&vgdev.host_visible_lock);
    drm_gem_free_mmap_offset(&vram.base.base.base);
    drm_gem_object_release(&vram.base.base.base);
    kfree(vram);
    } else {
    drm_gem_object_release(&bo.base.base);
    kfree(bo);
    }
    }
#[no_mangle]
unsafe extern "C" fn virtio_gpu_free_object(obj: *mut drm_gem_object) {
    static void virtio_gpu_free_object(struct drm_gem_object *obj)
    {
    struct virtio_gpu_object *bo = gem_to_virtio_gpu_obj(obj);
    struct virtio_gpu_device *vgdev = bo.base.base.dev.dev_private;
    if (bo.created) {
    virtio_gpu_remove_from_restore_list(bo);
    virtio_gpu_cmd_unref_resource(vgdev, bo, false);
    virtio_gpu_notify(vgdev);
// completion handler calls virtio_gpu_cleanup_object()
    return;
    }
    virtio_gpu_cleanup_object(bo);
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_detach_object_fenced(bo: *mut virtio_gpu_object) -> c_int {
    int virtio_gpu_detach_object_fenced(struct virtio_gpu_object *bo)
    {
    struct virtio_gpu_device *vgdev = bo.base.base.dev.dev_private;
    struct virtio_gpu_fence *fence;
    if (!bo.attached)
    return 0;
    fence = virtio_gpu_fence_alloc(vgdev, vgdev.fence_drv.context, 0);
    if (!fence)
    return -ENOMEM;
    virtio_gpu_object_detach(vgdev, bo, fence);
    virtio_gpu_notify(vgdev);
    dma_fence_wait(&fence.f, false);
    dma_fence_put(&fence.f);
    return 0;
    }
    static const struct drm_gem_object_funcs virtio_gpu_shmem_funcs = {
    .free = virtio_gpu_free_object,
    .open = virtio_gpu_gem_object_open,
    .close = virtio_gpu_gem_object_close,
    .print_info = drm_gem_shmem_object_print_info,
    .export = virtgpu_gem_prime_export,
    .pin = drm_gem_shmem_object_pin,
    .unpin = drm_gem_shmem_object_unpin,
    .get_sg_table = drm_gem_shmem_object_get_sg_table,
    .vmap = drm_gem_shmem_object_vmap,
    .vunmap = drm_gem_shmem_object_vunmap,
    .mmap = drm_gem_shmem_object_mmap,
    .vm_ops = &drm_gem_shmem_vm_ops,
    };
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_is_shmem(bo: *mut virtio_gpu_object) -> bool {
    bool virtio_gpu_is_shmem(struct virtio_gpu_object *bo)
    {
    return bo.base.base.funcs == &virtio_gpu_shmem_funcs;
    }
    struct drm_gem_object *virtio_gpu_create_object(struct drm_device *dev,
    size_t size)
    {
    struct virtio_gpu_object_shmem *shmem;
    struct drm_gem_shmem_object *dshmem;
    shmem = kzalloc_obj(*shmem);
    if (!shmem)
    return ERR_PTR(-ENOMEM);
    dshmem = &shmem.base.base;
    dshmem.base.funcs = &virtio_gpu_shmem_funcs;
    return &dshmem.base;
    }
    static int virtio_gpu_object_shmem_init(struct virtio_gpu_device *vgdev,
    struct virtio_gpu_object *bo,
    struct virtio_gpu_mem_entry **ents,
    unsigned int *nents)
    {
    let mut use_dma_api: bool = virtio_gpu_use_dma_api(vgdev.vdev);
    struct scatterlist *sg;
    struct sg_table *pages;
    int si;
    pages = drm_gem_shmem_get_pages_sgt(&bo.base);
    if (IS_ERR(pages))
    return PTR_ERR(pages);
    if (use_dma_api)
// nents = pages->nents;
    else
// nents = pages->orig_nents;
// ents = kvmalloc_objs(struct virtio_gpu_mem_entry, *nents);
    if (!(*ents)) {
    DRM_ERROR("failed to allocate ent list\n");
    return -ENOMEM;
    }
    if (use_dma_api) {
    for_each_sgtable_dma_sg(pages, sg, si) {
    (*ents)[si].addr = cpu_to_le64(sg_dma_address(sg));
    (*ents)[si].length = cpu_to_le32(sg_dma_len(sg));
    (*ents)[si].padding = 0;
    }
    } else {
    for_each_sgtable_sg(pages, sg, si) {
    (*ents)[si].addr = cpu_to_le64(sg_phys(sg));
    (*ents)[si].length = cpu_to_le32(sg.length);
    (*ents)[si].padding = 0;
    }
    }
    return 0;
    }
    int virtio_gpu_object_create(struct virtio_gpu_device *vgdev,
    struct virtio_gpu_object_params *params,
    struct virtio_gpu_object **bo_ptr,
    struct virtio_gpu_fence *fence)
    {
    struct virtio_gpu_object_array *objs = core::ptr::null_mut();
    struct drm_gem_shmem_object *shmem_obj;
    struct virtio_gpu_object *bo;
    struct virtio_gpu_mem_entry *ents = core::ptr::null_mut();
    unsigned int nents;
    int ret;
// bo_ptr = NULL;
    params.size = roundup(params.size, PAGE_SIZE);
    shmem_obj = drm_gem_shmem_create(vgdev.ddev, params.size);
    if (IS_ERR(shmem_obj))
    return PTR_ERR(shmem_obj);
    bo = gem_to_virtio_gpu_obj(&shmem_obj.base);
    INIT_LIST_HEAD(&bo.restore_node);
    ret = virtio_gpu_resource_id_get(vgdev, &bo.hw_res_handle);
    if (ret < 0)
    goto err_free_gem;
    bo.dumb = params.dumb;
    ret = virtio_gpu_object_shmem_init(vgdev, bo, &ents, &nents);
    if (ret != 0)
    goto err_put_id;
    if (fence) {
    ret = -ENOMEM;
    objs = virtio_gpu_array_alloc(1);
    if (!objs)
    goto err_free_entry;
    virtio_gpu_array_add_obj(objs, &bo.base.base);
    ret = virtio_gpu_array_lock_resv(objs);
    if (ret != 0)
    goto err_put_objs;
    }
    if (params.blob) {
    if (params.blob_mem == VIRTGPU_BLOB_MEM_GUEST)
    bo.guest_blob = true;
    virtio_gpu_cmd_resource_create_blob(vgdev, bo, params,
    ents, nents);
    } else if (params.virgl) {
    virtio_gpu_cmd_resource_create_3d(vgdev, bo, params,
    objs, fence);
    virtio_gpu_object_attach(vgdev, bo, ents, nents);
    } else {
    virtio_gpu_cmd_create_resource(vgdev, bo, params,
    objs, fence);
    virtio_gpu_object_attach(vgdev, bo, ents, nents);
    }
    if (!params.virgl) {
// store non-virgl object with its param to the restore list
    bo.params = *params;
    virtio_gpu_add_object_to_restore_list(vgdev, bo);
    }
// bo_ptr = bo;
    return 0;
    err_put_objs:
    virtio_gpu_array_put_free(objs);
    err_free_entry:
    kvfree(ents);
    err_put_id:
    virtio_gpu_resource_id_put(vgdev, bo.hw_res_handle);
    err_free_gem:
    drm_gem_shmem_free(shmem_obj);
    return ret;
    }
    void virtio_gpu_add_object_to_restore_list(struct virtio_gpu_device *vgdev,
    struct virtio_gpu_object *bo)
    {
    mutex_lock(&vgdev.obj_restore_lock);
    list_add_tail(&bo.restore_node, &vgdev.obj_restore_list);
    mutex_unlock(&vgdev.obj_restore_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_object_restore_all(vgdev: *mut virtio_gpu_device) -> c_int {
    int virtio_gpu_object_restore_all(struct virtio_gpu_device *vgdev)
    {
    struct virtio_gpu_object *bo, *tmp;
    struct virtio_gpu_mem_entry *ents;
    unsigned int nents;
    let mut ret: c_int = 0;
    mutex_lock(&vgdev.obj_restore_lock);
    list_for_each_entry_safe(bo, tmp, &vgdev.obj_restore_list,
    restore_node) {
    if (drm_gem_is_imported(&bo.base.base)) {
    ret = virtgpu_dma_buf_obj_resubmit(vgdev, bo);
    if (ret)
    break;
    continue;
    }
    if (bo.params.blob || bo.attached) {
    ret = virtio_gpu_object_shmem_init(vgdev, bo, &ents,
    &nents);
    if (ret)
    break;
    }
    if (bo.params.blob) {
    virtio_gpu_cmd_resource_create_blob(vgdev, bo,
    &bo.params,
    ents, nents);
    } else {
    virtio_gpu_cmd_create_resource(vgdev, bo, &bo.params,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (bo.attached) {
    bo.attached = false;
    virtio_gpu_object_attach(vgdev, bo, ents,
    nents);
    }
    }
    }
    mutex_unlock(&vgdev.obj_restore_lock);
    if (ret)
    DRM_ERROR("failed to restore virtio-gpu objects: %d\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn virtio_gpu_object_unref_all(vgdev: *mut virtio_gpu_device) {
    void virtio_gpu_object_unref_all(struct virtio_gpu_device *vgdev)
    {
    struct virtio_gpu_object *bo, *tmp;
    mutex_lock(&vgdev.obj_restore_lock);
    list_for_each_entry_safe(bo, tmp, &vgdev.obj_restore_list,
    restore_node)
    if (bo.created) {
    virtio_gpu_cmd_unref_resource(vgdev, bo, true);
    virtio_gpu_notify(vgdev);
    }
    mutex_unlock(&vgdev.obj_restore_lock);
    }
