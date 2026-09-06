//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/rockchip/inno_hdmi-rockchip.c
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Zheng Yang <zhengyang@rock-chips.com>
// Andy Yan <andy.yan@rock-chips.com>
//

pub const RK3036_GRF_SOC_CON2: c_uint = 0x148;

    enum inno_hdmi_dev_type {
    RK3036_HDMI,
    RK3128_HDMI,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_hdmi_connector_state {
    pub base: drm_connector_state,
    pub colorimetry: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_inno_hdmi {
    pub base: *mut inno_hdmi,
    pub dev: *mut device,
    pub grf: *mut regmap,
    pub encoder: rockchip_encoder,
}

    static struct inno_hdmi_phy_config rk3036_hdmi_phy_configs[] = {
    {  74250000, 0x3f, 0xbb },
    { 165000000, 0x6f, 0xbb },
    {      ~0UL, 0x00, 0x00 }
    };
    static struct inno_hdmi_phy_config rk3128_hdmi_phy_configs[] = {
    {  74250000, 0x3f, 0xaa },
    { 165000000, 0x5f, 0xaa },
    {      ~0UL, 0x00, 0x00 }
    };
#[no_mangle]
unsafe extern "C" fn inno_hdmi_rk3036_enable(dev: *mut device, mode: *mut drm_display_mode) {
    static void inno_hdmi_rk3036_enable(struct device *dev, struct drm_display_mode *mode)
    {
    struct rockchip_inno_hdmi *hdmi = dev_get_drvdata(dev);
    int value, psync;
    psync = mode.flags & DRM_MODE_FLAG_PHSYNC ? 1 : 0;
    value = FIELD_PREP_WM16(RK3036_HDMI_PHSYNC, psync);
    psync = mode.flags & DRM_MODE_FLAG_PVSYNC ? 1 : 0;
    value |= FIELD_PREP_WM16(RK3036_HDMI_PVSYNC, psync);
    regmap_write(hdmi.grf, RK3036_GRF_SOC_CON2, value);
    }
    static int inno_hdmi_encoder_atomic_check(struct drm_encoder *encoder,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state)
    {
    struct rockchip_crtc_state *s = to_rockchip_crtc_state(crtc_state);
    s.output_mode = ROCKCHIP_OUT_MODE_P888;
    s.output_type = DRM_MODE_CONNECTOR_HDMIA;
    return 0;
    }
    static const struct drm_encoder_helper_funcs inno_hdmi_rockchip_encoder_helper_funcs = {
    .atomic_check	= inno_hdmi_encoder_atomic_check,
    };
#[no_mangle]
unsafe extern "C" fn inno_hdmi_rockchip_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int inno_hdmi_rockchip_bind(struct device *dev, struct device *master, void *data)
    {
    struct drm_device *drm = data;
    struct drm_connector *connector;
    struct drm_encoder *encoder;
    struct rockchip_inno_hdmi *hdmi;
    const struct inno_hdmi_plat_data *plat_data;
    int ret;
    hdmi = drmm_kzalloc(drm, sizeof(*hdmi), GFP_KERNEL);
    if (!hdmi)
    return -ENOMEM;
    hdmi.dev = dev;
    plat_data = of_device_get_match_data(hdmi.dev);
    if (!plat_data)
    return -EINVAL;
    if (of_device_is_compatible(dev.of_node, "rockchip,rk3036-inno-hdmi")) {
    hdmi.grf = syscon_regmap_lookup_by_phandle(dev.of_node, "rockchip,grf");
    if (IS_ERR(hdmi.grf))
    return dev_err_probe(dev,
    PTR_ERR(hdmi.grf), "Unable to get rockchip,grf\n");
    }
    encoder = &hdmi.encoder.encoder;
    encoder.possible_crtcs = drm_of_find_possible_crtcs(drm, dev.of_node);
//
// If we failed to find the CRTC(s) which this encoder is
// supposed to be connected to, it's because the CRTC has
// not been registered yet.  Defer probing, and hope that
// the required CRTC is added later.
//
    if (encoder.possible_crtcs == 0)
    return -EPROBE_DEFER;
    ret = drmm_encoder_init(drm, encoder, core::ptr::null_mut(), DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_encoder_helper_add(encoder, &inno_hdmi_rockchip_encoder_helper_funcs);
    dev_set_drvdata(dev, hdmi);
    hdmi.base = inno_hdmi_bind(dev, encoder, plat_data);
    connector = drm_bridge_connector_init(drm, encoder);
    if (IS_ERR(connector)) {
    ret = PTR_ERR(connector);
    dev_err(hdmi.dev, "failed to init bridge connector: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct component_ops inno_hdmi_rockchip_ops = {
    .bind	= inno_hdmi_rockchip_bind,
    };
#[no_mangle]
unsafe extern "C" fn inno_hdmi_rockchip_probe(pdev: *mut platform_device) -> c_int {
    static int inno_hdmi_rockchip_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &inno_hdmi_rockchip_ops);
    }
#[no_mangle]
unsafe extern "C" fn inno_hdmi_rockchip_remove(pdev: *mut platform_device) {
    static void inno_hdmi_rockchip_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &inno_hdmi_rockchip_ops);
    }
    static const struct inno_hdmi_plat_ops rk3036_inno_hdmi_plat_ops = {
    .enable = inno_hdmi_rk3036_enable,
    };
    static const struct inno_hdmi_plat_data rk3036_inno_hdmi_plat_data = {
    .ops = &rk3036_inno_hdmi_plat_ops,
    .phy_configs = rk3036_hdmi_phy_configs,
    .default_phy_config = &rk3036_hdmi_phy_configs[1],
    };
    static const struct inno_hdmi_plat_data rk3128_inno_hdmi_plat_data = {
    .phy_configs = rk3128_hdmi_phy_configs,
    .default_phy_config = &rk3128_hdmi_phy_configs[1],
    };
    static const struct of_device_id inno_hdmi_rockchip_dt_ids[] = {
    { .compatible = "rockchip,rk3036-inno-hdmi",
    .data = &rk3036_inno_hdmi_plat_data,
    },
    { .compatible = "rockchip,rk3128-inno-hdmi",
    .data = &rk3128_inno_hdmi_plat_data,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, inno_hdmi_rockchip_dt_ids);
    struct platform_driver inno_hdmi_driver = {
    .probe  = inno_hdmi_rockchip_probe,
    .remove = inno_hdmi_rockchip_remove,
    .driver = {
    .name = "innohdmi-rockchip",
    .of_match_table = inno_hdmi_rockchip_dt_ids,
    },
    };
