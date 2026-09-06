//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/exynos/exynos_drm_dpi.c
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
// Exynos DRM Parallel output support.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd
//
// Contacts: Andrzej Hajda <a.hajda@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_dpi {
    pub encoder: drm_encoder,
    pub dev: *mut device,
    pub panel_node: *mut device_node,
    pub panel: *mut drm_panel,
    pub connector: drm_connector,
    pub vm: *mut videomode,
}

    static inline struct exynos_dpi *encoder_to_dpi(struct drm_encoder *e)
    {
    return container_of(e, struct exynos_dpi, encoder);
    }
    static enum drm_connector_status
    exynos_dpi_detect(struct drm_connector *connector, bool force)
    {
    return connector_status_connected;
    }
#[no_mangle]
unsafe extern "C" fn exynos_dpi_connector_destroy(connector: *mut drm_connector) {
    static void exynos_dpi_connector_destroy(struct drm_connector *connector)
    {
    drm_connector_unregister(connector);
    drm_connector_cleanup(connector);
    }
    static const struct drm_connector_funcs exynos_dpi_connector_funcs = {
    .detect = exynos_dpi_detect,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .destroy = exynos_dpi_connector_destroy,
    .reset = drm_atomic_helper_connector_reset,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    };
#[no_mangle]
unsafe extern "C" fn exynos_dpi_get_modes(connector: *mut drm_connector) -> c_int {
    static int exynos_dpi_get_modes(struct drm_connector *connector)
    {
    struct exynos_dpi *ctx = connector_to_dpi(connector);
// fimd timings gets precedence over panel modes
    if (ctx.vm) {
    struct drm_display_mode *mode;
    mode = drm_mode_create(connector.dev);
    if (!mode) {
    DRM_DEV_ERROR(ctx.dev,
    "failed to create a new display mode\n");
    return 0;
    }
    drm_display_mode_from_videomode(ctx.vm, mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    drm_mode_probed_add(connector, mode);
    return 1;
    }
    if (ctx.panel)
    return drm_panel_get_modes(ctx.panel, connector);
    return 0;
    }
    static const struct drm_connector_helper_funcs exynos_dpi_connector_helper_funcs = {
    .get_modes = exynos_dpi_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn exynos_dpi_create_connector(encoder: *mut drm_encoder) -> c_int {
    static int exynos_dpi_create_connector(struct drm_encoder *encoder)
    {
    struct exynos_dpi *ctx = encoder_to_dpi(encoder);
    struct drm_connector *connector = &ctx.connector;
    int ret;
    connector.polled = DRM_CONNECTOR_POLL_HPD;
    ret = drm_connector_init(encoder.dev, connector,
    &exynos_dpi_connector_funcs,
    DRM_MODE_CONNECTOR_DPI);
    if (ret) {
    DRM_DEV_ERROR(ctx.dev,
    "failed to initialize connector with drm\n");
    return ret;
    }
    drm_connector_helper_add(connector, &exynos_dpi_connector_helper_funcs);
    drm_connector_attach_encoder(connector, encoder);
    return 0;
    }
    static void exynos_dpi_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    }
#[no_mangle]
unsafe extern "C" fn exynos_dpi_enable(encoder: *mut drm_encoder) {
    static void exynos_dpi_enable(struct drm_encoder *encoder)
    {
    struct exynos_dpi *ctx = encoder_to_dpi(encoder);
    if (ctx.panel) {
    drm_panel_prepare(ctx.panel);
    drm_panel_enable(ctx.panel);
    }
    }
#[no_mangle]
unsafe extern "C" fn exynos_dpi_disable(encoder: *mut drm_encoder) {
    static void exynos_dpi_disable(struct drm_encoder *encoder)
    {
    struct exynos_dpi *ctx = encoder_to_dpi(encoder);
    if (ctx.panel) {
    drm_panel_disable(ctx.panel);
    drm_panel_unprepare(ctx.panel);
    }
    }
    static const struct drm_encoder_funcs exynos_dpi_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_encoder_helper_funcs exynos_dpi_encoder_helper_funcs = {
    .mode_set = exynos_dpi_mode_set,
    .enable = exynos_dpi_enable,
    .disable = exynos_dpi_disable,
    };
    enum {
    FIMD_PORT_IN0,
    FIMD_PORT_IN1,
    FIMD_PORT_IN2,
    FIMD_PORT_RGB,
    FIMD_PORT_WRB,
    };
#[no_mangle]
unsafe extern "C" fn exynos_dpi_parse_dt(ctx: *mut exynos_dpi) -> c_int {
    static int exynos_dpi_parse_dt(struct exynos_dpi *ctx)
    {
    struct device *dev = ctx.dev;
    struct device_node *dn = dev.of_node;
    struct device_node *np;
    ctx.panel_node = of_graph_get_remote_node(dn, FIMD_PORT_RGB, 0);
    np = of_get_child_by_name(dn, "display-timings");
    if (np) {
    struct videomode *vm;
    int ret;
    of_node_put(np);
    vm = devm_kzalloc(dev, sizeof(*ctx.vm), GFP_KERNEL);
    if (!vm)
    return -ENOMEM;
    ret = of_get_videomode(dn, vm, 0);
    if (ret < 0) {
    devm_kfree(dev, vm);
    return ret;
    }
    ctx.vm = vm;
    return 0;
    }
    if (!ctx.panel_node)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exynos_dpi_bind(dev: *mut drm_device, encoder: *mut drm_encoder) -> c_int {
    int exynos_dpi_bind(struct drm_device *dev, struct drm_encoder *encoder)
    {
    int ret;
    ret = drm_encoder_init(dev, encoder, &exynos_dpi_encoder_funcs,
    DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (ret) {
    DRM_DEV_ERROR(encoder_to_dpi(encoder).dev,
    "failed to create encoder ret = %d\n", ret);
    return ret;
    }
    drm_encoder_helper_add(encoder, &exynos_dpi_encoder_helper_funcs);
    ret = exynos_drm_set_possible_crtcs(encoder, EXYNOS_DISPLAY_TYPE_LCD);
    if (ret < 0)
    return ret;
    ret = exynos_dpi_create_connector(encoder);
    if (ret) {
    DRM_DEV_ERROR(encoder_to_dpi(encoder).dev,
    "failed to create connector ret = %d\n", ret);
    drm_encoder_cleanup(encoder);
    return ret;
    }
    return 0;
    }
    struct drm_encoder *exynos_dpi_probe(struct device *dev)
    {
    struct exynos_dpi *ctx;
    int ret;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return ERR_PTR(-ENOMEM);
    ctx.dev = dev;
    ret = exynos_dpi_parse_dt(ctx);
    if (ret < 0) {
    devm_kfree(dev, ctx);
    return core::ptr::null_mut();
    }
    if (ctx.panel_node) {
    ctx.panel = of_drm_find_panel(ctx.panel_node);
    if (IS_ERR(ctx.panel))
    return ERR_CAST(ctx.panel);
    }
    return &ctx.encoder;
    }
#[no_mangle]
pub unsafe extern "C" fn exynos_dpi_remove(encoder: *mut drm_encoder) -> c_int {
    int exynos_dpi_remove(struct drm_encoder *encoder)
    {
    struct exynos_dpi *ctx = encoder_to_dpi(encoder);
    exynos_dpi_disable(&ctx.encoder);
    if (ctx.panel)
    drm_panel_put(ctx.panel);
    return 0;
    }
