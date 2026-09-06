//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tests/drm_atomic_test.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Kunit test for drm_atomic functions
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_atomic_test_priv {
    pub drm: drm_device,
    pub plane: *mut drm_plane,
    pub crtc: *mut drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

    static const struct drm_connector_helper_funcs drm_atomic_init_connector_helper_funcs = {
    };
    static const struct drm_connector_funcs drm_atomic_init_connector_funcs = {
    .atomic_destroy_state	= drm_atomic_helper_connector_destroy_state,
    .atomic_duplicate_state	= drm_atomic_helper_connector_duplicate_state,
    .reset			= drm_atomic_helper_connector_reset,
    };
    static struct drm_atomic_test_priv *create_device(struct kunit *test)
    {
    struct drm_atomic_test_priv *priv;
    struct drm_connector *connector;
    struct drm_encoder *enc;
    struct drm_device *drm;
    struct drm_plane *plane;
    struct drm_crtc *crtc;
    struct device *dev;
    int ret;
    dev = drm_kunit_helper_alloc_device(test);
    if (IS_ERR(dev))
    return ERR_CAST(dev);
    priv = drm_kunit_helper_alloc_drm_device(test, dev,
    struct drm_atomic_test_priv, drm,
    DRIVER_MODESET | DRIVER_ATOMIC);
    if (IS_ERR(priv))
    return ERR_CAST(priv);
    drm = &priv.drm;
    plane = drm_kunit_helper_create_primary_plane(test, drm,
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(), 0,
    core::ptr::null_mut());
    if (IS_ERR(plane))
    return ERR_CAST(plane);
    priv.plane = plane;
    crtc = drm_kunit_helper_create_crtc(test, drm,
    plane, core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut());
    if (IS_ERR(crtc))
    return ERR_CAST(crtc);
    priv.crtc = crtc;
    enc = &priv.encoder;
    ret = drmm_encoder_init(drm, enc, core::ptr::null_mut(), DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    enc.possible_crtcs = drm_crtc_mask(crtc);
    connector = &priv.connector;
    ret = drmm_connector_init(drm, connector,
    &drm_atomic_init_connector_funcs,
    DRM_MODE_CONNECTOR_VIRTUAL,
    core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    drm_connector_helper_add(connector, &drm_atomic_init_connector_helper_funcs);
    drm_connector_attach_encoder(connector, enc);
    drm_mode_config_reset(drm);
    return priv;
    }
#[no_mangle]
unsafe extern "C" fn drm_test_drm_atomic_get_connector_for_encoder(test: *mut kunit) {
    static void drm_test_drm_atomic_get_connector_for_encoder(struct kunit *test)
    {
    struct drm_modeset_acquire_ctx ctx;
    struct drm_atomic_test_priv *priv;
    struct drm_display_mode *mode;
    struct drm_connector *curr_connector;
    int ret;
    priv = create_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv);
    mode = drm_kunit_display_mode_from_cea_vic(test, &priv.drm, 16);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, mode);
    drm_modeset_acquire_init(&ctx, 0);
    retry_enable:
    ret = drm_kunit_helper_enable_crtc_connector(test, &priv.drm,
    priv.crtc, &priv.connector,
    mode, &ctx);
    if (ret == -EDEADLK) {
    drm_modeset_backoff(&ctx);
    goto retry_enable;
    }
    KUNIT_ASSERT_EQ(test, ret, 0);
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    drm_modeset_acquire_init(&ctx, 0);
    retry_conn:
    curr_connector = drm_atomic_get_connector_for_encoder(&priv.encoder,
    &ctx);
    if (PTR_ERR(curr_connector) == -EDEADLK) {
    drm_modeset_backoff(&ctx);
    goto retry_conn;
    }
    KUNIT_EXPECT_PTR_EQ(test, curr_connector, &priv.connector);
    drm_modeset_drop_locks(&ctx);
    drm_modeset_acquire_fini(&ctx);
    }
    static struct kunit_case drm_atomic_get_connector_for_encoder_tests[] = {
    KUNIT_CASE(drm_test_drm_atomic_get_connector_for_encoder),
    { }
    };
    static struct kunit_suite drm_atomic_get_connector_for_encoder_test_suite = {
    .name = "drm_test_atomic_get_connector_for_encoder",
    .test_cases = drm_atomic_get_connector_for_encoder_tests,
    };
    kunit_test_suite(drm_atomic_get_connector_for_encoder_test_suite);
    MODULE_AUTHOR("Maxime Ripard <mripard@kernel.org>");
    MODULE_DESCRIPTION("Kunit test for drm_atomic functions");
    MODULE_LICENSE("GPL");
