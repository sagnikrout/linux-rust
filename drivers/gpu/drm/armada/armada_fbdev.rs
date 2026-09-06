//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/armada/armada_fbdev.c
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
// Copyright (C) 2012 Russell King
// Written from the i915 driver.
//

#[no_mangle]
unsafe extern "C" fn armada_fbdev_fb_destroy(info: *mut fb_info) {
    static void armada_fbdev_fb_destroy(struct fb_info *info)
    {
    struct drm_fb_helper *fbh = info.par;
    drm_fb_helper_fini(fbh);
    fbh.fb.funcs.destroy(fbh.fb);
    drm_client_release(&fbh.client);
    }
    static const struct fb_ops armada_fb_ops = {
    .owner		= THIS_MODULE,
    FB_DEFAULT_IOMEM_OPS,
    DRM_FB_HELPER_DEFAULT_OPS,
    .fb_destroy	= armada_fbdev_fb_destroy,
    };
    static const struct drm_fb_helper_funcs armada_fbdev_helper_funcs;
    int armada_fbdev_driver_fbdev_probe(struct drm_fb_helper *fbh,
    struct drm_fb_helper_surface_size *sizes)
    {
    struct drm_device *dev = fbh.dev;
    struct fb_info *info = fbh.info;
    struct drm_mode_fb_cmd2 mode;
    struct armada_framebuffer *dfb;
    struct armada_gem_object *obj;
    int size, ret;
    void *ptr;
    memset(&mode, 0, sizeof(mode));
    mode.width = sizes.surface_width;
    mode.height = sizes.surface_height;
    mode.pitches[0] = armada_pitch(mode.width, sizes.surface_bpp);
    mode.pixel_format = drm_mode_legacy_fb_format(sizes.surface_bpp,
    sizes.surface_depth);
    size = mode.pitches[0] * mode.height;
    obj = armada_gem_alloc_private_object(dev, size);
    if (!obj) {
    DRM_ERROR("failed to allocate fb memory\n");
    return -ENOMEM;
    }
    ret = armada_gem_linear_back(dev, obj);
    if (ret) {
    drm_gem_object_put(&obj.obj);
    return ret;
    }
    ptr = armada_gem_map_object(dev, obj);
    if (!ptr) {
    drm_gem_object_put(&obj.obj);
    return -ENOMEM;
    }
    dfb = armada_framebuffer_create(dev,
    drm_get_format_info(dev, mode.pixel_format,
    mode.modifier[0]),
    &mode, obj);
//
// A reference is now held by the framebuffer object if
// successful, otherwise this drops the ref for the error path.
//
    drm_gem_object_put(&obj.obj);
    if (IS_ERR(dfb))
    return PTR_ERR(dfb);
    info.fbops = &armada_fb_ops;
    info.fix.smem_start = obj.phys_addr;
    info.fix.smem_len = obj.obj.size;
    info.screen_size = obj.obj.size;
    info.screen_base = ptr;
    fbh.funcs = &armada_fbdev_helper_funcs;
    fbh.fb = &dfb.fb;
    drm_fb_helper_fill_info(info, fbh, sizes);
    DRM_DEBUG_KMS("allocated %dx%d %dbpp fb: 0x%08llx\n",
    dfb.fb.width, dfb.fb.height, dfb.fb.format.cpp[0] * 8,
    (unsigned long long)obj.phys_addr);
    return 0;
    }
