//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nouveau_prime.c
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
// Copyright 2011 Red Hat Inc.
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
// Authors: Dave Airlie
//

    struct sg_table *nouveau_gem_prime_get_sg_table(struct drm_gem_object *obj)
    {
    struct nouveau_bo *nvbo = nouveau_gem_object(obj);
    return drm_prime_pages_to_sg(obj.dev, nvbo.bo.ttm.pages,
    nvbo.bo.ttm.num_pages);
    }
    struct drm_gem_object *nouveau_gem_prime_import_sg_table(struct drm_device *dev,
    struct dma_buf_attachment *attach,
    struct sg_table *sg)
    {
    struct nouveau_drm *drm = nouveau_drm(dev);
    struct drm_gem_object *obj;
    struct nouveau_bo *nvbo;
    struct dma_resv *robj = attach.dmabuf.resv;
    let mut size: u64 = attach.dmabuf.size;
    let mut align: c_int = 0;
    int ret;
    dma_resv_lock(robj, core::ptr::null_mut());
    nvbo = nouveau_bo_alloc(&drm.client, &size, &align,
    NOUVEAU_GEM_DOMAIN_GART, 0, 0, true);
    if (IS_ERR(nvbo)) {
    obj = ERR_CAST(nvbo);
    goto unlock;
    }
    nvbo.valid_domains = NOUVEAU_GEM_DOMAIN_GART;
    nvbo.bo.base.funcs = &nouveau_gem_object_funcs;
// Initialize the embedded gem-object. We return a single gem-reference
// to the caller, instead of a normal nouveau_bo ttm reference.
    ret = drm_gem_object_init(dev, &nvbo.bo.base, size);
    if (ret) {
    drm_gem_object_release(&nvbo.bo.base);
    kfree(nvbo);
    obj = ERR_PTR(-ENOMEM);
    goto unlock;
    }
    ret = nouveau_bo_init(nvbo, size, align, NOUVEAU_GEM_DOMAIN_GART,
    sg, robj);
    if (ret) {
    obj = ERR_PTR(ret);
    goto unlock;
    }
    obj = &nvbo.bo.base;
    unlock:
    dma_resv_unlock(robj);
    return obj;
    }
#[no_mangle]
pub unsafe extern "C" fn nouveau_gem_prime_pin(obj: *mut drm_gem_object) -> c_int {
    int nouveau_gem_prime_pin(struct drm_gem_object *obj)
    {
    struct nouveau_bo *nvbo = nouveau_gem_object(obj);
    int ret;
// pin buffer into GTT
    ret = nouveau_bo_pin_locked(nvbo, NOUVEAU_GEM_DOMAIN_GART, false);
    if (ret)
    ret = -EINVAL;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn nouveau_gem_prime_unpin(obj: *mut drm_gem_object) {
    void nouveau_gem_prime_unpin(struct drm_gem_object *obj)
    {
    struct nouveau_bo *nvbo = nouveau_gem_object(obj);
    nouveau_bo_unpin_locked(nvbo);
    }
    struct dma_buf *nouveau_gem_prime_export(struct drm_gem_object *gobj,
    int flags)
    {
    struct nouveau_bo *nvbo = nouveau_gem_object(gobj);
    struct ttm_operation_ctx ctx = {
    .interruptible = true,
    .no_wait_gpu = true,
// We opt to avoid OOM on system pages allocations
    .gfp_retry_mayfail = true,
    .allow_res_evict = false,
    };
    int ret;
    if (nvbo.no_share)
    return ERR_PTR(-EPERM);
    ret = ttm_bo_setup_export(&nvbo.bo, &ctx);
    if (ret)
    return ERR_PTR(ret);
    return drm_gem_prime_export(gobj, flags);
    }
