//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tegra/fbdev.c
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
// Copyright (C) 2012-2013 Avionic Design GmbH
// Copyright (C) 2012 NVIDIA CORPORATION.  All rights reserved.
//
// Based on the KMS/FB DMA helpers
// Copyright (C) 2012 Analog Devices Inc.
//

#[no_mangle]
unsafe extern "C" fn tegra_fb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int tegra_fb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct drm_fb_helper *helper = info.par;
    struct tegra_bo *bo;
    int err;
    bo = tegra_fb_get_plane(helper.fb, 0);
    err = drm_gem_mmap_obj(&bo.gem, bo.gem.size, vma);
    if (err < 0)
    return err;
    return __tegra_gem_mmap(&bo.gem, vma);
    }
#[no_mangle]
unsafe extern "C" fn tegra_fbdev_fb_destroy(info: *mut fb_info) {
    static void tegra_fbdev_fb_destroy(struct fb_info *info)
    {
    struct drm_fb_helper *helper = info.par;
    struct tegra_bo *bo = tegra_fb_get_plane(helper.fb, 0);
    drm_fb_helper_fini(helper);
// Undo the special mapping we made in fbdev probe.
    if (bo.pages) {
    vunmap(bo.vaddr);
    bo.vaddr = core::ptr::null_mut();
    }
    drm_client_buffer_delete(helper.buffer);
    drm_client_release(&helper.client);
    }
    static const struct fb_ops tegra_fb_ops = {
    .owner = THIS_MODULE,
    __FB_DEFAULT_DMAMEM_OPS_RDWR,
    DRM_FB_HELPER_DEFAULT_OPS,
    __FB_DEFAULT_DMAMEM_OPS_DRAW,
    .fb_mmap = tegra_fb_mmap,
    .fb_destroy = tegra_fbdev_fb_destroy,
    };
    static const struct drm_fb_helper_funcs tegra_fbdev_helper_funcs = {
    };
    int tegra_fbdev_driver_fbdev_probe(struct drm_fb_helper *helper,
    struct drm_fb_helper_surface_size *sizes)
    {
    struct drm_client_dev *client = &helper.client;
    struct drm_device *drm = client.dev;
    struct drm_file *file = client.file;
    struct tegra_drm *tegra = drm.dev_private;
    struct fb_info *info = helper.info;
    u32 fourcc, pitch;
    u64 size;
    const struct drm_format_info *format;
    struct tegra_bo *bo;
    struct drm_gem_object *gem;
    u32 handle;
    struct drm_client_buffer *buffer;
    int err;
    fourcc = drm_mode_legacy_fb_format(sizes.surface_bpp, sizes.surface_depth);
    format = drm_get_format_info(drm, fourcc, DRM_FORMAT_MOD_LINEAR);
    pitch = round_up(drm_format_info_min_pitch(format, 0, sizes.surface_width),
    tegra.pitch_align);
    size = ALIGN(pitch * sizes.surface_height, PAGE_SIZE);
    bo = tegra_bo_create(drm, size, 0);
    if (IS_ERR(bo))
    return PTR_ERR(bo);
    gem = &bo.gem;
    err = drm_gem_handle_create(file, gem, &handle);
    if (err)
    goto err_drm_gem_object_put;
    buffer = drm_client_buffer_create(client, sizes.surface_width, sizes.surface_height,
    fourcc, handle, pitch);
    if (IS_ERR(buffer)) {
    err = PTR_ERR(buffer);
    goto err_drm_gem_handle_delete;
    }
    helper.funcs = &tegra_fbdev_helper_funcs;
    helper.buffer = buffer;
    helper.fb = buffer.fb;
    info.fbops = &tegra_fb_ops;
    drm_fb_helper_fill_info(info, helper, sizes);
    if (bo.pages) {
    bo.vaddr = vmap(bo.pages, bo.num_pages, VM_MAP,
    pgprot_writecombine(PAGE_KERNEL));
    if (!bo.vaddr) {
    dev_err(drm.dev, "failed to vmap() framebuffer\n");
    err = -ENOMEM;
    goto err_drm_client_buffer_delete;
    }
    }
    info.flags |= FBINFO_VIRTFB;
    info.screen_buffer = bo.vaddr;
    info.screen_size = gem.size;
    info.fix.smem_start = (unsigned long)(bo.iova);
    info.fix.smem_len = gem.size;
// The handle is only needed for creating the framebuffer.
    drm_gem_handle_delete(file, handle);
// The framebuffer still holds a reference on the GEM object.
    drm_gem_object_put(gem);
    return 0;
    err_drm_client_buffer_delete:
    drm_client_buffer_delete(buffer);
    err_drm_gem_handle_delete:
    drm_gem_handle_delete(file, handle);
    err_drm_gem_object_put:
    drm_gem_object_put(gem);
    return err;
    }
