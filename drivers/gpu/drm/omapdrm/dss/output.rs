//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/omapdrm/dss/output.c
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
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
// Author: Archit Taneja <archit@ti.com>
//

    int omapdss_device_init_output(struct omap_dss_device *out,
    struct drm_bridge *local_bridge)
    {
    struct device_node *remote_node;
    int ret;
    remote_node = of_graph_get_remote_node(out.dev.of_node,
    out.of_port, 0);
    if (!remote_node) {
    dev_dbg(out.dev, "failed to find video sink\n");
    return 0;
    }
    out.panel = of_drm_find_panel(remote_node);
    if (IS_ERR(out.panel))
    out.panel = core::ptr::null_mut();
    if (!out.panel)
    out.bridge = of_drm_find_and_get_bridge(remote_node);
    of_node_put(remote_node);
    if (out.panel) {
    struct drm_bridge *bridge;
    bridge = drm_panel_bridge_add(out.panel);
    drm_panel_put(out.panel);
    if (IS_ERR(bridge)) {
    dev_err(out.dev,
    "unable to create panel bridge (%ld)\n",
    PTR_ERR(bridge));
    ret = PTR_ERR(bridge);
    goto error;
    }
    out.bridge = drm_bridge_get(bridge);
    }
    if (local_bridge) {
    if (!out.bridge) {
    ret = -EPROBE_DEFER;
    goto error;
    }
    out.next_bridge = out.bridge;
    out.bridge = drm_bridge_get(local_bridge);
    }
    if (!out.bridge) {
    ret = -EPROBE_DEFER;
    goto error;
    }
    return 0;
    error:
    omapdss_device_cleanup_output(out);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn omapdss_device_cleanup_output(out: *mut omap_dss_device) {
    void omapdss_device_cleanup_output(struct omap_dss_device *out)
    {
    if (out.bridge && out.panel)
    drm_panel_bridge_remove(out.next_bridge ?
    out.next_bridge : out.bridge);
    drm_bridge_put(out.next_bridge);
    drm_bridge_put(out.bridge);
    }
    void dss_mgr_set_timings(struct omap_dss_device *dssdev,
    const struct videomode *vm)
    {
    omap_crtc_dss_set_timings(dssdev.dss.mgr_ops_priv,
    dssdev.dispc_channel, vm);
    }
    void dss_mgr_set_lcd_config(struct omap_dss_device *dssdev,
    const struct dss_lcd_mgr_config *config)
    {
    omap_crtc_dss_set_lcd_config(dssdev.dss.mgr_ops_priv,
    dssdev.dispc_channel, config);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_mgr_enable(dssdev: *mut omap_dss_device) -> c_int {
    int dss_mgr_enable(struct omap_dss_device *dssdev)
    {
    return omap_crtc_dss_enable(dssdev.dss.mgr_ops_priv,
    dssdev.dispc_channel);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_mgr_disable(dssdev: *mut omap_dss_device) {
    void dss_mgr_disable(struct omap_dss_device *dssdev)
    {
    omap_crtc_dss_disable(dssdev.dss.mgr_ops_priv,
    dssdev.dispc_channel);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_mgr_start_update(dssdev: *mut omap_dss_device) {
    void dss_mgr_start_update(struct omap_dss_device *dssdev)
    {
    omap_crtc_dss_start_update(dssdev.dss.mgr_ops_priv,
    dssdev.dispc_channel);
    }
    int dss_mgr_register_framedone_handler(struct omap_dss_device *dssdev,
    void (*handler)(void *), void *data)
    {
    struct dss_device *dss = dssdev.dss;
    return omap_crtc_dss_register_framedone(dss.mgr_ops_priv,
    dssdev.dispc_channel,
    handler, data);
    }
    void dss_mgr_unregister_framedone_handler(struct omap_dss_device *dssdev,
    void (*handler)(void *), void *data)
    {
    struct dss_device *dss = dssdev.dss;
    omap_crtc_dss_unregister_framedone(dss.mgr_ops_priv,
    dssdev.dispc_channel,
    handler, data);
    }
