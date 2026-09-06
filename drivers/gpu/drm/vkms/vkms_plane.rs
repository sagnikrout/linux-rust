//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vkms/vkms_plane.c
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

    static const u32 vkms_formats[] = {
    DRM_FORMAT_ARGB8888,
    DRM_FORMAT_ABGR8888,
    DRM_FORMAT_BGRA8888,
    DRM_FORMAT_RGBA8888,
    DRM_FORMAT_XRGB8888,
    DRM_FORMAT_XBGR8888,
    DRM_FORMAT_RGB888,
    DRM_FORMAT_BGR888,
    DRM_FORMAT_XRGB16161616,
    DRM_FORMAT_XBGR16161616,
    DRM_FORMAT_ARGB16161616,
    DRM_FORMAT_ABGR16161616,
    DRM_FORMAT_RGB565,
    DRM_FORMAT_BGR565,
    DRM_FORMAT_NV12,
    DRM_FORMAT_NV16,
    DRM_FORMAT_NV24,
    DRM_FORMAT_NV21,
    DRM_FORMAT_NV61,
    DRM_FORMAT_NV42,
    DRM_FORMAT_YUV420,
    DRM_FORMAT_YUV422,
    DRM_FORMAT_YUV444,
    DRM_FORMAT_YVU420,
    DRM_FORMAT_YVU422,
    DRM_FORMAT_YVU444,
    DRM_FORMAT_P010,
    DRM_FORMAT_P012,
    DRM_FORMAT_P016,
    DRM_FORMAT_R1,
    DRM_FORMAT_R2,
    DRM_FORMAT_R4,
    DRM_FORMAT_R8,
    };
    static struct drm_plane_state *
    vkms_plane_duplicate_state(struct drm_plane *plane)
    {
    struct vkms_plane_state *vkms_state;
    struct vkms_frame_info *frame_info;
    vkms_state = kzalloc_obj(*vkms_state);
    if (!vkms_state)
    return core::ptr::null_mut();
    frame_info = kzalloc_obj(*frame_info);
    if (!frame_info) {
    DRM_DEBUG_KMS("Couldn't allocate frame_info\n");
    kfree(vkms_state);
    return core::ptr::null_mut();
    }
    vkms_state.frame_info = frame_info;
    __drm_gem_duplicate_shadow_plane_state(plane, &vkms_state.base);
    return &vkms_state.base.base;
    }
    static void vkms_plane_destroy_state(struct drm_plane *plane,
    struct drm_plane_state *old_state)
    {
    struct vkms_plane_state *vkms_state = to_vkms_plane_state(old_state);
    struct drm_crtc *crtc = vkms_state.base.base.crtc;
    if (crtc && vkms_state.frame_info.fb) {
// dropping the reference we acquired in
// vkms_primary_plane_update()
//
    if (drm_framebuffer_read_refcount(vkms_state.frame_info.fb))
    drm_framebuffer_put(vkms_state.frame_info.fb);
    }
    kfree(vkms_state.frame_info);
    vkms_state.frame_info = core::ptr::null_mut();
    __drm_gem_destroy_shadow_plane_state(&vkms_state.base);
    kfree(vkms_state);
    }
#[no_mangle]
unsafe extern "C" fn vkms_plane_reset(plane: *mut drm_plane) {
    static void vkms_plane_reset(struct drm_plane *plane)
    {
    struct vkms_plane_state *vkms_state;
    if (plane.state) {
    vkms_plane_destroy_state(plane, plane.state);
    plane.state = core::ptr::null_mut(); /* must be set to core::ptr::null_mut() here */
    }
    vkms_state = kzalloc_obj(*vkms_state);
    if (!vkms_state) {
    DRM_ERROR("Cannot allocate vkms_plane_state\n");
    return;
    }
    __drm_gem_reset_shadow_plane(plane, &vkms_state.base);
    }
    static const struct drm_plane_funcs vkms_plane_funcs = {
    .update_plane		= drm_atomic_helper_update_plane,
    .disable_plane		= drm_atomic_helper_disable_plane,
    .reset			= vkms_plane_reset,
    .atomic_duplicate_state = vkms_plane_duplicate_state,
    .atomic_destroy_state	= vkms_plane_destroy_state,
    };
    static void vkms_plane_atomic_update(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *new_state = drm_atomic_get_new_plane_state(state,
    plane);
    struct vkms_plane_state *vkms_plane_state;
    struct drm_shadow_plane_state *shadow_plane_state;
    struct drm_framebuffer *fb = new_state.fb;
    struct vkms_frame_info *frame_info;
    u32 fmt;
    if (!new_state.crtc || !fb)
    return;
    fmt = fb.format.format;
    vkms_plane_state = to_vkms_plane_state(new_state);
    shadow_plane_state = &vkms_plane_state.base;
    frame_info = vkms_plane_state.frame_info;
    memcpy(&frame_info.src, &new_state.src, sizeof(struct drm_rect));
    memcpy(&frame_info.dst, &new_state.dst, sizeof(struct drm_rect));
    frame_info.fb = fb;
    memcpy(&frame_info.map, &shadow_plane_state.data, sizeof(frame_info.map));
    drm_framebuffer_get(frame_info.fb);
    frame_info.rotation = new_state.rotation;
    vkms_plane_state.pixel_read_line = get_pixel_read_line_function(fmt);
    get_conversion_matrix_to_argb_u16(fmt, new_state.color_encoding, new_state.color_range,
    &vkms_plane_state.conversion_matrix);
    }
    static int vkms_plane_atomic_check(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *new_plane_state = drm_atomic_get_new_plane_state(state,
    plane);
    struct drm_crtc_state *crtc_state;
    int ret;
    if (!new_plane_state.fb || WARN_ON(!new_plane_state.crtc))
    return 0;
    crtc_state = drm_atomic_get_crtc_state(state,
    new_plane_state.crtc);
    if (IS_ERR(crtc_state))
    return PTR_ERR(crtc_state);
    ret = drm_atomic_helper_check_plane_state(new_plane_state, crtc_state,
    DRM_PLANE_NO_SCALING,
    DRM_PLANE_NO_SCALING,
    true, true);
    if (ret != 0)
    return ret;
    return 0;
    }
    static int vkms_prepare_fb(struct drm_plane *plane,
    struct drm_plane_state *state)
    {
    struct drm_shadow_plane_state *shadow_plane_state;
    struct drm_framebuffer *fb = state.fb;
    int ret;
    if (!fb)
    return 0;
    shadow_plane_state = to_drm_shadow_plane_state(state);
    ret = drm_gem_plane_helper_prepare_fb(plane, state);
    if (ret)
    return ret;
    return drm_gem_fb_vmap(fb, shadow_plane_state.map, shadow_plane_state.data);
    }
    static void vkms_cleanup_fb(struct drm_plane *plane,
    struct drm_plane_state *state)
    {
    struct drm_shadow_plane_state *shadow_plane_state;
    struct drm_framebuffer *fb = state.fb;
    if (!fb)
    return;
    shadow_plane_state = to_drm_shadow_plane_state(state);
    drm_gem_fb_vunmap(fb, shadow_plane_state.map);
    }
    static const struct drm_plane_helper_funcs vkms_plane_helper_funcs = {
    .atomic_update		= vkms_plane_atomic_update,
    .atomic_check		= vkms_plane_atomic_check,
    .prepare_fb		= vkms_prepare_fb,
    .cleanup_fb		= vkms_cleanup_fb,
    };
    struct vkms_plane *vkms_plane_init(struct vkms_device *vkmsdev,
    struct vkms_config_plane *plane_cfg)
    {
    struct drm_device *dev = &vkmsdev.drm;
    struct vkms_plane *plane;
    plane = drmm_universal_plane_alloc(dev, struct vkms_plane, base, 0,
    &vkms_plane_funcs,
    vkms_formats, ARRAY_SIZE(vkms_formats),
    core::ptr::null_mut(), vkms_config_plane_get_type(plane_cfg),
    core::ptr::null_mut());
    if (IS_ERR(plane))
    return plane;
    drm_plane_helper_add(&plane.base, &vkms_plane_helper_funcs);
    drm_plane_create_rotation_property(&plane.base, DRM_MODE_ROTATE_0,
    DRM_MODE_ROTATE_MASK | DRM_MODE_REFLECT_MASK);
    drm_plane_create_color_properties(&plane.base,
    BIT(DRM_COLOR_YCBCR_BT601) |
    BIT(DRM_COLOR_YCBCR_BT709) |
    BIT(DRM_COLOR_YCBCR_BT2020),
    BIT(DRM_COLOR_YCBCR_LIMITED_RANGE) |
    BIT(DRM_COLOR_YCBCR_FULL_RANGE),
    DRM_COLOR_YCBCR_BT601,
    DRM_COLOR_YCBCR_FULL_RANGE);
    if (vkms_config_plane_get_default_pipeline(plane_cfg))
    vkms_initialize_colorops(&plane.base);
    return plane;
    }
