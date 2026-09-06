//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/of-display-mode-bridge.c
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
//
// Copyright (C) 2012 Sascha Hauer, Pengutronix
//
// bridge driver for legacy DT bindings, utilizing display-timings node
//
// Author: Dmitry Baryshkov <dmitry.baryshkov@oss.qualcomm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_display_mode_bridge {
    pub base: drm_bridge,
    pub mode: drm_display_mode,
    pub bus_flags: u32,
}

    static int of_display_mode_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    if (!(flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR))
    return -EINVAL;
    return 0;
    }
    static int of_display_mode_bridge_get_modes(struct drm_bridge *bridge,
    struct drm_connector *connector)
    {
    struct of_display_mode_bridge *of_bridge = to_of_display_mode_bridge(bridge);
    int ret;
    ret = drm_connector_helper_get_modes_fixed(connector, &of_bridge.mode);
    if (ret)
    return ret;
    connector.display_info.bus_flags = of_bridge.bus_flags;
    return 0;
    }
    static const struct drm_bridge_funcs of_display_mode_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .attach = of_display_mode_bridge_attach,
    .get_modes = of_display_mode_bridge_get_modes,
    };
    struct drm_bridge *devm_drm_of_display_mode_bridge(struct device *dev,
    struct device_node *np,
    int type)
    {
    struct of_display_mode_bridge *of_bridge;
    int ret;
    of_bridge = devm_drm_bridge_alloc(dev, struct of_display_mode_bridge,
    base, &of_display_mode_bridge_funcs);
    if (IS_ERR(of_bridge))
    return ERR_CAST(of_bridge);
    ret = of_get_drm_display_mode(np,
    &of_bridge.mode,
    &of_bridge.bus_flags,
    OF_USE_NATIVE_MODE);
    if (ret)
    return ERR_PTR(ret);
    of_bridge.mode.type |= DRM_MODE_TYPE_DRIVER;
    of_bridge.base.of_node = np;
    of_bridge.base.ops = DRM_BRIDGE_OP_MODES;
    of_bridge.base.type = type;
    ret = devm_drm_bridge_add(dev, &of_bridge.base);
    if (ret)
    return ERR_PTR(ret);
    return &of_bridge.base;
    }
    EXPORT_SYMBOL_GPL(devm_drm_of_display_mode_bridge);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("DRM bridge driver for legacy DT bindings");
