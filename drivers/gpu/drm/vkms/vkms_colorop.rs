//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vkms/vkms_colorop.c
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


// SPDX-License-Identifier: GPL-2.0+

    static const u64 supported_tfs =
    BIT(DRM_COLOROP_1D_CURVE_SRGB_EOTF) |
    BIT(DRM_COLOROP_1D_CURVE_SRGB_INV_EOTF);
    static const struct drm_colorop_funcs vkms_colorop_funcs = {
    .destroy = drm_colorop_destroy,
    };
pub const MAX_COLOR_PIPELINE_OPS: c_int = 4;
#[no_mangle]
unsafe extern "C" fn vkms_initialize_color_pipeline(plane: *mut drm_plane, list: *mut drm_prop_enum_list) -> c_int {
    static int vkms_initialize_color_pipeline(struct drm_plane *plane, struct drm_prop_enum_list *list)
    {
    struct drm_colorop *ops[MAX_COLOR_PIPELINE_OPS];
    struct drm_device *dev = plane.dev;
    int ret;
    let mut i: c_int = 0, j = 0;
    memset(ops, 0, sizeof(ops));
// 1st op: 1d curve
    ops[i] = kzalloc_obj(*ops[i]);
    if (!ops[i]) {
    drm_err(dev, "KMS: Failed to allocate colorop\n");
    ret = -ENOMEM;
    goto cleanup;
    }
    ret = drm_plane_colorop_curve_1d_init(dev, ops[i], plane, &vkms_colorop_funcs,
    supported_tfs,
    DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if (ret)
    goto cleanup;
    list.type = ops[i].base.id;
    i++;
// 2nd op: 3x4 matrix
    ops[i] = kzalloc_obj(*ops[i]);
    if (!ops[i]) {
    drm_err(dev, "KMS: Failed to allocate colorop\n");
    ret = -ENOMEM;
    goto cleanup;
    }
    ret = drm_plane_colorop_ctm_3x4_init(dev, ops[i], plane, &vkms_colorop_funcs,
    DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if (ret)
    goto cleanup;
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    i++;
// 3rd op: 3x4 matrix
    ops[i] = kzalloc_obj(*ops[i]);
    if (!ops[i]) {
    drm_err(dev, "KMS: Failed to allocate colorop\n");
    ret = -ENOMEM;
    goto cleanup;
    }
    ret = drm_plane_colorop_ctm_3x4_init(dev, ops[i], plane, &vkms_colorop_funcs,
    DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if (ret)
    goto cleanup;
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    i++;
// 4th op: 1d curve
    ops[i] = kzalloc_obj(*ops[i]);
    if (!ops[i]) {
    drm_err(dev, "KMS: Failed to allocate colorop\n");
    ret = -ENOMEM;
    goto cleanup;
    }
    ret = drm_plane_colorop_curve_1d_init(dev, ops[i], plane, &vkms_colorop_funcs,
    supported_tfs,
    DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if (ret)
    goto cleanup;
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    list.name = kasprintf(GFP_KERNEL, "Color Pipeline %d", ops[0].base.id);
    return 0;
    cleanup:
    for (j = 0; j < i; j++) {
    if (ops[j]) {
    drm_colorop_cleanup(ops[j]);
    kfree(ops[j]);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn vkms_initialize_colorops(plane: *mut drm_plane) -> c_int {
    int vkms_initialize_colorops(struct drm_plane *plane)
    {
    let mut pipeline: drm_prop_enum_list = {};
    let mut ret: c_int = 0;
// Add color pipeline
    ret = vkms_initialize_color_pipeline(plane, &pipeline);
    if (ret)
    goto out;
// Create COLOR_PIPELINE property and attach
    ret = drm_plane_create_color_pipeline_property(plane, &pipeline, 1);
    kfree(pipeline.name);
    out:
    return ret;
    }
