//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/imx/imx8mp-hdmi-pvi.c
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
// Copyright (C) 2022 Pengutronix, Lucas Stach <kernel@pengutronix.de>
//

pub const HTX_PVI_CTRL: c_uint = 0x0;

pub const PVI_CTRL_MODE_LCDIF: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mp_hdmi_pvi {
    pub bridge: drm_bridge,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
}

    static inline struct imx8mp_hdmi_pvi *
    to_imx8mp_hdmi_pvi(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct imx8mp_hdmi_pvi, bridge);
    }
    static int imx8mp_hdmi_pvi_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct imx8mp_hdmi_pvi *pvi = to_imx8mp_hdmi_pvi(bridge);
    return drm_bridge_attach(encoder, pvi.bridge.next_bridge,
    bridge, flags);
    }
    static void imx8mp_hdmi_pvi_bridge_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct imx8mp_hdmi_pvi *pvi = to_imx8mp_hdmi_pvi(bridge);
    struct drm_connector_state *conn_state;
    struct drm_bridge_state *bridge_state;
    const struct drm_display_mode *mode;
    struct drm_crtc_state *crtc_state;
    struct drm_connector *connector;
    let mut bus_flags: u32 = 0, val;
    bridge_state = drm_atomic_get_new_bridge_state(state, bridge);
    connector = drm_atomic_get_new_connector_for_encoder(state, bridge.encoder);
    conn_state = drm_atomic_get_new_connector_state(state, connector);
    crtc_state = drm_atomic_get_new_crtc_state(state, conn_state.crtc);
    if (WARN_ON(pm_runtime_resume_and_get(pvi.dev)))
    return;
    mode = &crtc_state.adjusted_mode;
    val = FIELD_PREP(PVI_CTRL_MODE_MASK, PVI_CTRL_MODE_LCDIF) | PVI_CTRL_EN;
    if (mode.flags & DRM_MODE_FLAG_PVSYNC)
    val |= PVI_CTRL_OP_VSYNC_POL | PVI_CTRL_INP_VSYNC_POL;
    if (mode.flags & DRM_MODE_FLAG_PHSYNC)
    val |= PVI_CTRL_OP_HSYNC_POL | PVI_CTRL_INP_HSYNC_POL;
    if (pvi.bridge.next_bridge.timings)
    bus_flags = pvi.bridge.next_bridge.timings.input_bus_flags;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: bridge_state) -> else {
    else if (bridge_state)
    bus_flags = bridge_state.input_bus_cfg.flags;
    if (bus_flags & DRM_BUS_FLAG_DE_HIGH)
    val |= PVI_CTRL_OP_DE_POL | PVI_CTRL_INP_DE_POL;
    writel(val, pvi.regs + HTX_PVI_CTRL);
    }
    static void imx8mp_hdmi_pvi_bridge_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct imx8mp_hdmi_pvi *pvi = to_imx8mp_hdmi_pvi(bridge);
    writel(0x0, pvi.regs + HTX_PVI_CTRL);
    pm_runtime_put(pvi.dev);
    }
    static u32 *
    imx8mp_hdmi_pvi_bridge_get_input_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    u32 output_fmt,
    unsigned int *num_input_fmts)
    {
    struct imx8mp_hdmi_pvi *pvi = to_imx8mp_hdmi_pvi(bridge);
    struct drm_bridge *next_bridge = pvi.bridge.next_bridge;
    struct drm_bridge_state *next_state;
    if (!next_bridge.funcs.atomic_get_input_bus_fmts)
    return core::ptr::null_mut();
    next_state = drm_atomic_get_new_bridge_state(crtc_state.state,
    next_bridge);
    return next_bridge.funcs.atomic_get_input_bus_fmts(next_bridge,
    next_state,
    crtc_state,
    conn_state,
    output_fmt,
    num_input_fmts);
    }
    static const struct drm_bridge_funcs imx_hdmi_pvi_bridge_funcs = {
    .attach		= imx8mp_hdmi_pvi_bridge_attach,
    .atomic_enable	= imx8mp_hdmi_pvi_bridge_enable,
    .atomic_disable	= imx8mp_hdmi_pvi_bridge_disable,
    .atomic_get_input_bus_fmts = imx8mp_hdmi_pvi_bridge_get_input_bus_fmts,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    };
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_pvi_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mp_hdmi_pvi_probe(struct platform_device *pdev)
    {
    struct device_node *remote;
    struct imx8mp_hdmi_pvi *pvi;
    pvi = devm_drm_bridge_alloc(&pdev.dev, struct imx8mp_hdmi_pvi,
    bridge, &imx_hdmi_pvi_bridge_funcs);
    if (IS_ERR(pvi))
    return PTR_ERR(pvi);
    platform_set_drvdata(pdev, pvi);
    pvi.dev = &pdev.dev;
    pvi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pvi.regs))
    return PTR_ERR(pvi.regs);
// Get the next bridge in the pipeline.
    remote = of_graph_get_remote_node(pdev.dev.of_node, 1, -1);
    if (!remote)
    return -EINVAL;
    pvi.bridge.next_bridge = of_drm_find_and_get_bridge(remote);
    of_node_put(remote);
    if (!pvi.bridge.next_bridge)
    return dev_err_probe(&pdev.dev, -EPROBE_DEFER,
    "could not find next bridge\n");
    pm_runtime_enable(&pdev.dev);
// Register the bridge.
    pvi.bridge.of_node = pdev.dev.of_node;
    pvi.bridge.timings = pvi.bridge.next_bridge.timings;
    drm_bridge_add(&pvi.bridge);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_pvi_remove(pdev: *mut platform_device) {
    static void imx8mp_hdmi_pvi_remove(struct platform_device *pdev)
    {
    struct imx8mp_hdmi_pvi *pvi = platform_get_drvdata(pdev);
    drm_bridge_remove(&pvi.bridge);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id imx8mp_hdmi_pvi_match[] = {
    {
    .compatible = "fsl,imx8mp-hdmi-pvi",
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, imx8mp_hdmi_pvi_match);
    static struct platform_driver imx8mp_hdmi_pvi_driver = {
    .probe	= imx8mp_hdmi_pvi_probe,
    .remove = imx8mp_hdmi_pvi_remove,
    .driver		= {
    .name = "imx-hdmi-pvi",
    .of_match_table	= imx8mp_hdmi_pvi_match,
    },
    };
    module_platform_driver(imx8mp_hdmi_pvi_driver);
    MODULE_DESCRIPTION("i.MX8MP HDMI TX Parallel Video Interface bridge driver");
    MODULE_LICENSE("GPL");
