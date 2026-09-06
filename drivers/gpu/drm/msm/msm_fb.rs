//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/msm_fb.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_framebuffer {
    pub base: drm_framebuffer,
    pub format: *const msm_format,
// Count of # of attached planes which need dirtyfb:
    pub dirtyfb: refcount_t,
// Framebuffer per-plane address, if pinned, else zero:
    pub iova: [u64; DRM_FORMAT_MAX_PLANES],
    pub prepare_count: core::sync::atomic::AtomicI32,
}

    static int msm_framebuffer_dirtyfb(struct drm_framebuffer *fb,
    struct drm_file *file_priv, unsigned int flags,
    unsigned int color, struct drm_clip_rect *clips,
    unsigned int num_clips)
    {
    struct msm_framebuffer *msm_fb = to_msm_framebuffer(fb);
// If this fb is not used on any display requiring pixel data to be
// flushed, then skip dirtyfb
//
    if (refcount_read(&msm_fb.dirtyfb) == 1)
    return 0;
    return drm_atomic_helper_dirtyfb(fb, file_priv, flags, color,
    clips, num_clips);
    }
    static const struct drm_framebuffer_funcs msm_framebuffer_funcs = {
    .create_handle = drm_gem_fb_create_handle,
    .destroy = drm_gem_fb_destroy,
    .dirty = msm_framebuffer_dirtyfb,
    };

#[no_mangle]
pub unsafe extern "C" fn msm_framebuffer_describe(fb: *mut drm_framebuffer, m: *mut seq_file) {
    void msm_framebuffer_describe(struct drm_framebuffer *fb, struct seq_file *m)
    {
    let mut stats: msm_gem_stats = {};
    int i, n = fb.format.num_planes;
    seq_printf(m, "fb: %dx%d@%4.4s (%2d, ID:%d)\n",
    fb.width, fb.height, (char *)&fb.format.format,
    drm_framebuffer_read_refcount(fb), fb.base.id);
    for (i = 0; i < n; i++) {
    seq_printf(m, "   %d: offset=%d pitch=%d, obj: ",
    i, fb.offsets[i], fb.pitches[i]);
    msm_gem_describe(fb.obj[i], m, &stats);
    }
    }

// prepare/pin all the fb's bo's for scanout.
//
#[no_mangle]
pub unsafe extern "C" fn msm_framebuffer_prepare(fb: *mut drm_framebuffer, needs_dirtyfb: bool) -> c_int {
    int msm_framebuffer_prepare(struct drm_framebuffer *fb, bool needs_dirtyfb)
    {
    struct msm_drm_private *priv = fb.dev.dev_private;
    struct drm_gpuvm *vm = priv.kms.vm;
    struct msm_framebuffer *msm_fb = to_msm_framebuffer(fb);
    int ret, i, n = fb.format.num_planes;
    if (needs_dirtyfb)
    refcount_inc(&msm_fb.dirtyfb);
    if (atomic_inc_return(&msm_fb.prepare_count) > 1)
    return 0;
    for (i = 0; i < n; i++) {
    msm_gem_vma_get(fb.obj[i]);
    ret = msm_gem_get_and_pin_iova(fb.obj[i], vm, &msm_fb.iova[i]);
    drm_dbg_state(fb.dev, "FB[%u]: iova[%d]: %08llx (%d)\n",
    fb.base.id, i, msm_fb.iova[i], ret);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn msm_framebuffer_cleanup(fb: *mut drm_framebuffer, needed_dirtyfb: bool) {
    void msm_framebuffer_cleanup(struct drm_framebuffer *fb, bool needed_dirtyfb)
    {
    struct msm_drm_private *priv = fb.dev.dev_private;
    struct drm_gpuvm *vm = priv.kms.vm;
    struct msm_framebuffer *msm_fb = to_msm_framebuffer(fb);
    int i, n = fb.format.num_planes;
    if (needed_dirtyfb)
    refcount_dec(&msm_fb.dirtyfb);
    if (atomic_dec_return(&msm_fb.prepare_count))
    return;
    memset(msm_fb.iova, 0, sizeof(msm_fb.iova));
    for (i = 0; i < n; i++) {
    msm_gem_unpin_iova(fb.obj[i], vm);
    msm_gem_vma_put(fb.obj[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn msm_framebuffer_iova(fb: *mut drm_framebuffer, plane: c_int) -> u32 {
    uint32_t msm_framebuffer_iova(struct drm_framebuffer *fb, int plane)
    {
    struct msm_framebuffer *msm_fb = to_msm_framebuffer(fb);
    return msm_fb.iova[plane] + fb.offsets[plane];
    }
    struct drm_gem_object *msm_framebuffer_bo(struct drm_framebuffer *fb, int plane)
    {
    return drm_gem_fb_get_obj(fb, plane);
    }
    const struct msm_format *msm_framebuffer_format(struct drm_framebuffer *fb)
    {
    struct msm_framebuffer *msm_fb = to_msm_framebuffer(fb);
    return msm_fb.format;
    }
    static struct drm_framebuffer *
    msm_framebuffer_init(struct drm_device *dev, const struct drm_format_info *info,
    const struct drm_mode_fb_cmd2 *mode_cmd,
    struct drm_gem_object **bos)
    {
    struct msm_drm_private *priv = dev.dev_private;
    struct msm_kms *kms = priv.kms;
    struct msm_framebuffer *msm_fb = core::ptr::null_mut();
    struct drm_framebuffer *fb;
    const struct msm_format *format;
    int ret, i, n;
    drm_dbg_state(dev, "create framebuffer: mode_cmd=%p (%dx%d@%p4cc)\n",
    mode_cmd, mode_cmd.width, mode_cmd.height,
    &mode_cmd.pixel_format);
    n = info.num_planes;
    format = mdp_get_format(kms, mode_cmd.pixel_format,
    mode_cmd.modifier[0]);
    if (!format) {
    DRM_DEV_ERROR(dev.dev, "unsupported pixel format: %p4cc\n",
    &mode_cmd.pixel_format);
    ret = -EINVAL;
    goto fail;
    }
    msm_fb = kzalloc_obj(*msm_fb);
    if (!msm_fb) {
    ret = -ENOMEM;
    goto fail;
    }
    fb = &msm_fb.base;
    msm_fb.format = format;
    if (n > ARRAY_SIZE(fb.obj)) {
    ret = -EINVAL;
    goto fail;
    }
    for (i = 0; i < n; i++) {
    let mut width: c_uint = mode_cmd.width / (i ? info.hsub : 1);
    let mut height: c_uint = mode_cmd.height / (i ? info.vsub : 1);
    unsigned int min_size;
    min_size = (height - 1) * mode_cmd.pitches[i]
    + width * info.cpp[i]
    + mode_cmd.offsets[i];
    if (bos[i].size < min_size) {
    ret = UERR(EINVAL, dev, "plane %d too small", i);
    goto fail;
    }
    if (to_msm_bo(bos[i]).flags & MSM_BO_NO_SHARE) {
    ret = UERR(EINVAL, dev, "Cannot map _NO_SHARE to kms vm");
    goto fail;
    }
    msm_fb.base.obj[i] = bos[i];
    }
    drm_helper_mode_fill_fb_struct(dev, fb, info, mode_cmd);
    ret = drm_framebuffer_init(dev, fb, &msm_framebuffer_funcs);
    if (ret) {
    DRM_DEV_ERROR(dev.dev, "framebuffer init failed: %d\n", ret);
    goto fail;
    }
    refcount_set(&msm_fb.dirtyfb, 1);
    drm_dbg_state(dev, "create: FB ID: %d (%p)\n", fb.base.id, fb);
    return fb;
    fail:
    kfree(msm_fb);
    return ERR_PTR(ret);
    }
    struct drm_framebuffer *msm_framebuffer_create(struct drm_device *dev,
    struct drm_file *file,
    const struct drm_format_info *info,
    const struct drm_mode_fb_cmd2 *mode_cmd)
    {
    struct drm_gem_object *bos[4] = {0};
    struct drm_framebuffer *fb;
    int ret, i, n = info.num_planes;
    for (i = 0; i < n; i++) {
    bos[i] = drm_gem_object_lookup(file, mode_cmd.handles[i]);
    if (!bos[i]) {
    ret = -ENXIO;
    goto out_unref;
    }
    }
    fb = msm_framebuffer_init(dev, info, mode_cmd, bos);
    if (IS_ERR(fb)) {
    ret = PTR_ERR(fb);
    goto out_unref;
    }
    return fb;
    out_unref:
    for (i = 0; i < n; i++)
    drm_gem_object_put(bos[i]);
    return ERR_PTR(ret);
    }
