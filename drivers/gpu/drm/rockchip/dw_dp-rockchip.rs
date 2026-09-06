//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/rockchip/dw_dp-rockchip.c
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
// Copyright (c) 2020 Rockchip Electronics Co., Ltd.
//
// Author: Zhang Yubing <yubing.zhang@rock-chips.com>
// Author: Andy Yan <andy.yan@rock-chips.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_dw_dp {
    pub base: *mut dw_dp,
    pub dev: *mut device,
    pub encoder: rockchip_encoder,
}

    static int dw_dp_encoder_atomic_check(struct drm_encoder *encoder,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state)
    {
    struct rockchip_crtc_state *s = to_rockchip_crtc_state(crtc_state);
    struct drm_atomic_commit *state = conn_state.state;
    struct drm_display_info *di = &conn_state.connector.display_info;
    struct drm_bridge *bridge  = drm_bridge_chain_get_first_bridge(encoder);
    struct drm_bridge_state *bridge_state = drm_atomic_get_new_bridge_state(state, bridge);
    let mut bus_format: u32 = bridge_state.input_bus_cfg.format;
    switch (bus_format) {
    case MEDIA_BUS_FMT_UYYVYY10_0_5X30:
    case MEDIA_BUS_FMT_UYYVYY8_0_5X24:
    s.output_mode = ROCKCHIP_OUT_MODE_YUV420;
    break;
    case MEDIA_BUS_FMT_YUYV10_1X20:
    case MEDIA_BUS_FMT_YUYV8_1X16:
    s.output_mode = ROCKCHIP_OUT_MODE_S888_DUMMY;
    break;
    case MEDIA_BUS_FMT_RGB101010_1X30:
    case MEDIA_BUS_FMT_RGB888_1X24:
    case MEDIA_BUS_FMT_RGB666_1X24_CPADHI:
    case MEDIA_BUS_FMT_YUV10_1X30:
    case MEDIA_BUS_FMT_YUV8_1X24:
    default:
    s.output_mode = ROCKCHIP_OUT_MODE_AAAA;
    break;
    }
    s.output_type = DRM_MODE_CONNECTOR_DisplayPort;
    s.bus_format = bus_format;
    s.bus_flags = di.bus_flags;
    s.color_space = V4L2_COLORSPACE_DEFAULT;
    return 0;
    }
    static const struct drm_encoder_helper_funcs dw_dp_encoder_helper_funcs = {
    .atomic_check		= dw_dp_encoder_atomic_check,
    };
#[no_mangle]
unsafe extern "C" fn dw_dp_rockchip_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int dw_dp_rockchip_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    const struct dw_dp_plat_data *plat_data;
    struct drm_device *drm_dev = data;
    struct rockchip_dw_dp *dp;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    int ret;
    dp = drmm_kzalloc(drm_dev, sizeof(*dp), GFP_KERNEL);
    if (!dp)
    return -ENOMEM;
    dp.dev = dev;
    platform_set_drvdata(pdev, dp);
    plat_data = of_device_get_match_data(dev);
    if (!plat_data)
    return -ENODEV;
    encoder = &dp.encoder.encoder;
    encoder.possible_crtcs = drm_of_find_possible_crtcs(drm_dev, dev.of_node);
    rockchip_drm_encoder_set_crtc_endpoint_id(&dp.encoder, dev.of_node, 0, 0);
    ret = drmm_encoder_init(drm_dev, encoder, core::ptr::null_mut(), DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_encoder_helper_add(encoder, &dw_dp_encoder_helper_funcs);
    dp.base = dw_dp_bind(dev, encoder, plat_data);
    if (IS_ERR(dp.base))
    return PTR_ERR(dp.base);
    connector = drm_bridge_connector_init(drm_dev, encoder);
    if (IS_ERR(connector)) {
    dw_dp_unbind(dp.base);
    return dev_err_probe(dev, PTR_ERR(connector),
    "Failed to init bridge connector\n");
    }
    return 0;
    }
    static void dw_dp_rockchip_unbind(struct device *dev, struct device *master,
    void *data)
    {
    struct rockchip_dw_dp *dp = dev_get_drvdata(dev);
    dw_dp_unbind(dp.base);
    }
    static const struct component_ops dw_dp_rockchip_component_ops = {
    .bind = dw_dp_rockchip_bind,
    .unbind = dw_dp_rockchip_unbind,
    };
#[no_mangle]
unsafe extern "C" fn dw_dp_probe(pdev: *mut platform_device) -> c_int {
    static int dw_dp_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &dw_dp_rockchip_component_ops);
    }
#[no_mangle]
unsafe extern "C" fn dw_dp_remove(pdev: *mut platform_device) {
    static void dw_dp_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &dw_dp_rockchip_component_ops);
    }
    static const struct dw_dp_plat_data rk3588_dp_plat_data = {
    .max_link_rate = 810000,
    .pixel_mode = DW_DP_MP_QUAD_PIXEL,
    };
    static const struct dw_dp_plat_data rk3576_dp_plat_data = {
    .max_link_rate = 810000,
    .pixel_mode = DW_DP_MP_DUAL_PIXEL,
    };
    static const struct of_device_id dw_dp_of_match[] = {
    {
    .compatible = "rockchip,rk3588-dp",
    .data = &rk3588_dp_plat_data,
    }, {
    .compatible = "rockchip,rk3576-dp",
    .data = &rk3576_dp_plat_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, dw_dp_of_match);
    struct platform_driver dw_dp_driver = {
    .probe	= dw_dp_probe,
    .remove = dw_dp_remove,
    .driver = {
    .name = "dw-dp",
    .of_match_table = dw_dp_of_match,
    },
    };
