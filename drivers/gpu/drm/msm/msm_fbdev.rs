//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/msm_fbdev.c
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

    let mut fbdev: static bool = true;
    MODULE_PARM_DESC(fbdev, "Enable fbdev compat layer");
    module_param(fbdev, bool, 0600);
//
// fbdev funcs, to implement legacy fbdev interface on top of drm driver
//
    FB_GEN_DEFAULT_DEFERRED_SYSMEM_OPS(msm_fbdev,
    drm_fb_helper_damage_range,
    drm_fb_helper_damage_area)
#[no_mangle]
unsafe extern "C" fn msm_fbdev_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int msm_fbdev_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct drm_fb_helper *helper = (struct drm_fb_helper *)info.par;
    struct drm_gem_object *bo = msm_framebuffer_bo(helper.fb, 0);
    return drm_gem_prime_mmap(bo, vma);
    }
#[no_mangle]
unsafe extern "C" fn msm_fbdev_fb_destroy(info: *mut fb_info) {
    static void msm_fbdev_fb_destroy(struct fb_info *info)
    {
    struct drm_fb_helper *helper = (struct drm_fb_helper *)info.par;
    struct drm_gem_object *bo = msm_framebuffer_bo(helper.fb, 0);
    DBG();
    drm_fb_helper_fini(helper);
    msm_gem_put_vaddr(bo);
    drm_client_buffer_delete(helper.buffer);
    drm_client_release(&helper.client);
    }
    static const struct fb_ops msm_fb_ops = {
    .owner = THIS_MODULE,
    __FB_DEFAULT_DEFERRED_OPS_RDWR(msm_fbdev),
    DRM_FB_HELPER_DEFAULT_OPS,
    __FB_DEFAULT_DEFERRED_OPS_DRAW(msm_fbdev),
    .fb_mmap = msm_fbdev_mmap,
    .fb_destroy = msm_fbdev_fb_destroy,
    };
    static int msm_fbdev_fb_dirty(struct drm_fb_helper *helper,
    struct drm_clip_rect *clip)
    {
    struct drm_device *dev = helper.dev;
    int ret;
// Call damage handlers only if necessary
    if (!(clip.x1 < clip.x2 && clip.y1 < clip.y2))
    return 0;
    if (helper.fb.funcs.dirty) {
    ret = helper.fb.funcs.dirty(helper.fb, core::ptr::null_mut(), 0, 0, clip, 1);
    if (drm_WARN_ONCE(dev, ret, "Dirty helper failed: ret=%d\n", ret))
    return ret;
    }
    return 0;
    }
    static const struct drm_fb_helper_funcs msm_fbdev_helper_funcs = {
    .fb_dirty = msm_fbdev_fb_dirty,
    };
    int msm_fbdev_driver_fbdev_probe(struct drm_fb_helper *helper,
    struct drm_fb_helper_surface_size *sizes)
    {
    struct drm_client_dev *client = &helper.client;
    struct drm_device *dev = client.dev;
    struct drm_file *file = client.file;
    struct msm_drm_private *priv = dev.dev_private;
    struct fb_info *fbi = helper.info;
    const struct drm_format_info *format;
    u32 fourcc, pitch, handle;
    u64 size;
    struct drm_gem_object *bo;
    struct drm_client_buffer *buffer;
    uint64_t paddr;
    int ret;
    DBG("create fbdev: %dx%d@%d (%dx%d)", sizes.surface_width,
    sizes.surface_height, sizes.surface_bpp,
    sizes.fb_width, sizes.fb_height);
    fourcc = drm_mode_legacy_fb_format(sizes.surface_bpp, sizes.surface_depth);
    format = drm_get_format_info(dev, fourcc, DRM_FORMAT_MOD_LINEAR);
// adreno needs pitch aligned to 32 pixels:
    pitch = drm_format_info_min_pitch(format, 0, ALIGN(sizes.surface_width, 32));
    size = ALIGN(pitch * sizes.surface_height, PAGE_SIZE);
// allocate backing bo
    DBG("allocating %llu bytes for fb %d", size, dev.primary.index);
    bo = msm_gem_new(dev, size, MSM_BO_SCANOUT | MSM_BO_WC | MSM_BO_STOLEN, core::ptr::null_mut());
    if (IS_ERR(bo)) {
    drm_warn(dev, "could not allocate stolen bo\n");
// try regular bo:
    bo = msm_gem_new(dev, size, MSM_BO_SCANOUT | MSM_BO_WC, core::ptr::null_mut());
    if (IS_ERR(bo)) {
    drm_err(dev, "failed to allocate buffer object\n");
    return PTR_ERR(bo);
    }
    }
    msm_gem_object_set_name(bo, "stolenfb");
    ret = drm_gem_handle_create(file, bo, &handle);
    if (ret)
    goto err_drm_gem_object_put;
    buffer = drm_client_buffer_create(client, sizes.surface_width, sizes.surface_height,
    fourcc, handle, pitch);
    if (IS_ERR(buffer)) {
    ret = PTR_ERR(buffer);
    goto err_drm_gem_handle_delete;
    }
//
// NOTE: if we can be guaranteed to be able to map buffer
// in panic (ie. lock-safe, etc) we could avoid pinning the
// buffer now:
//
    ret = msm_gem_get_and_pin_iova(bo, priv.kms.vm, &paddr);
    if (ret) {
    drm_err(dev, "failed to get buffer obj iova: %d\n", ret);
    goto err_drm_client_buffer_delete;
    }
    DBG("fbi=%p, dev=%p", fbi, dev);
    helper.funcs = &msm_fbdev_helper_funcs;
    helper.buffer = buffer;
    helper.fb = buffer.fb;
    fbi.fbops = &msm_fb_ops;
    drm_fb_helper_fill_info(fbi, helper, sizes);
    fbi.screen_buffer = msm_gem_get_vaddr(bo);
    if (IS_ERR(fbi.screen_buffer)) {
    ret = PTR_ERR(fbi.screen_buffer);
    goto err_msm_gem_unpin;
    }
    fbi.screen_size = bo.size;
    fbi.fix.smem_start = paddr;
    fbi.fix.smem_len = bo.size;
    DBG("par=%p, %dx%d", fbi.par, fbi.var.xres, fbi.var.yres);
    DBG("allocated %dx%d fb", buffer.fb.width, buffer.fb.height);
// The handle is only needed for creating the framebuffer.
    drm_gem_handle_delete(file, handle);
// The framebuffer still holds a reference on the GEM object.
    drm_gem_object_put(bo);
    return 0;
    err_msm_gem_unpin:
    msm_gem_unpin_iova(bo, priv.kms.vm);
    msm_gem_vma_put(bo);
    err_drm_client_buffer_delete:
    drm_client_buffer_delete(buffer);
    err_drm_gem_handle_delete:
    drm_gem_handle_delete(file, handle);
    err_drm_gem_object_put:
    drm_gem_object_put(bo);
    return ret;
    }
