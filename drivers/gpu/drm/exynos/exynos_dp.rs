//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/exynos/exynos_dp.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Samsung SoC DP (Display Port) interface driver.
//
// Copyright (C) 2012 Samsung Electronics Co., Ltd.
// Author: Jingoo Han <jg1.han@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_dp_device {
    pub encoder: drm_encoder,
    pub drm_dev: *mut drm_device,
    pub dev: *mut device,
    pub adp: *mut analogix_dp_device,
    pub plat_data: analogix_dp_plat_data,
}

    static int exynos_dp_crtc_clock_enable(struct analogix_dp_plat_data *plat_data,
    bool enable)
    {
    struct exynos_dp_device *dp = to_dp(plat_data);
    struct drm_encoder *encoder = &dp.encoder;
    if (!encoder.crtc)
    return -EPERM;
    exynos_drm_pipe_clk_enable(to_exynos_crtc(encoder.crtc), enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_dp_poweron(plat_data: *mut analogix_dp_plat_data) -> c_int {
    static int exynos_dp_poweron(struct analogix_dp_plat_data *plat_data)
    {
    return exynos_dp_crtc_clock_enable(plat_data, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos_dp_poweroff(plat_data: *mut analogix_dp_plat_data) -> c_int {
    static int exynos_dp_poweroff(struct analogix_dp_plat_data *plat_data)
    {
    return exynos_dp_crtc_clock_enable(plat_data, false);
    }
    static void exynos_dp_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    }
#[no_mangle]
unsafe extern "C" fn exynos_dp_nop(encoder: *mut drm_encoder) {
    static void exynos_dp_nop(struct drm_encoder *encoder)
    {
// do nothing
    }
    static const struct drm_encoder_funcs exynos_dp_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_encoder_helper_funcs exynos_dp_encoder_helper_funcs = {
    .mode_set = exynos_dp_mode_set,
    .enable = exynos_dp_nop,
    .disable = exynos_dp_nop,
    };
#[no_mangle]
unsafe extern "C" fn exynos_dp_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int exynos_dp_bind(struct device *dev, struct device *master, void *data)
    {
    struct exynos_dp_device *dp = dev_get_drvdata(dev);
    struct drm_encoder *encoder = &dp.encoder;
    struct drm_device *drm_dev = data;
    struct drm_connector *connector;
    int ret;
    dp.drm_dev = drm_dev;
    ret = drm_encoder_init(drm_dev, encoder, &exynos_dp_encoder_funcs,
    DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (ret) {
    dev_err(dp.dev, "Failed to initialize encoder\n");
    return ret;
    }
    drm_encoder_helper_add(encoder, &exynos_dp_encoder_helper_funcs);
    ret = exynos_drm_set_possible_crtcs(encoder, EXYNOS_DISPLAY_TYPE_LCD);
    if (ret < 0)
    return ret;
    dp.plat_data.encoder = encoder;
    ret = analogix_dp_bind(dp.adp, dp.drm_dev);
    if (ret) {
    dp.encoder.funcs.destroy(&dp.encoder);
    return ret;
    }
    connector = drm_bridge_connector_init(dp.drm_dev, dp.plat_data.encoder);
    if (IS_ERR(connector)) {
    ret = PTR_ERR(connector);
    dev_err(dp.dev, "Failed to initialize bridge_connector\n");
    return ret;
    }
    return 0;
    }
    static void exynos_dp_unbind(struct device *dev, struct device *master,
    void *data)
    {
    struct exynos_dp_device *dp = dev_get_drvdata(dev);
    analogix_dp_unbind(dp.adp);
    dp.encoder.funcs.destroy(&dp.encoder);
    }
    static const struct component_ops exynos_dp_ops = {
    .bind	= exynos_dp_bind,
    .unbind	= exynos_dp_unbind,
    };
#[no_mangle]
unsafe extern "C" fn exynos_dp_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_dp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np;
    struct exynos_dp_device *dp;
    dp = devm_kzalloc(&pdev.dev, sizeof(struct exynos_dp_device),
    GFP_KERNEL);
    if (!dp)
    return -ENOMEM;
    dp.dev = dev;
//
// We just use the drvdata until driver run into component
// add function, and then we would set drvdata to null, so
// that analogix dp driver would take charge of the drvdata.
//
    platform_set_drvdata(pdev, dp);
// This is for the backward compatibility.
    np = of_parse_phandle(dev.of_node, "panel", 0);
    if (np) {
    dp.plat_data.panel = of_drm_find_panel(np);
    of_node_put(np);
    if (IS_ERR(dp.plat_data.panel))
    return PTR_ERR(dp.plat_data.panel);
    goto out;
    }
    if (of_get_display_timings(dev.of_node)) {
    dp.plat_data.next_bridge = devm_drm_of_display_mode_bridge(dp.dev,
    dp.dev.of_node,
    DRM_MODE_CONNECTOR_eDP);
    if (IS_ERR(dp.plat_data.next_bridge))
    return PTR_ERR(dp.plat_data.next_bridge);
    }
// The remote port can be either a panel or a bridge
    dp.plat_data.dev_type = EXYNOS_DP;
    dp.plat_data.power_on = exynos_dp_poweron;
    dp.plat_data.power_off = exynos_dp_poweroff;
    dp.plat_data.ops = &exynos_dp_ops;
    out:
    dp.adp = analogix_dp_probe(dev, &dp.plat_data);
    if (IS_ERR(dp.adp)) {
//
// The driver core does not invoke remove() for failed probes,
// so release the probe-time panel reference here.
//
    if (dp.plat_data.panel)
    drm_panel_put(dp.plat_data.panel);
    return PTR_ERR(dp.adp);
    }
    if (dp.plat_data.panel || dp.plat_data.next_bridge)
    return component_add(&pdev.dev, &exynos_dp_ops);
    else
    return analogix_dp_finish_probe(dp.adp);
    }
#[no_mangle]
unsafe extern "C" fn exynos_dp_remove(pdev: *mut platform_device) {
    static void exynos_dp_remove(struct platform_device *pdev)
    {
    struct exynos_dp_device *dp = platform_get_drvdata(pdev);
//
// Release the probe-time reference from of_drm_find_panel(). If bind
// ran, the panel_bridge holds a second reference that devm cleanup
// will release when the bridge is destroyed after remove() returns.
//
    if (dp.plat_data.panel)
    drm_panel_put(dp.plat_data.panel);
    component_del(&pdev.dev, &exynos_dp_ops);
    }
#[no_mangle]
unsafe extern "C" fn exynos_dp_suspend(dev: *mut device) -> c_int {
    static int exynos_dp_suspend(struct device *dev)
    {
    struct exynos_dp_device *dp = dev_get_drvdata(dev);
    return analogix_dp_suspend(dp.adp);
    }
#[no_mangle]
unsafe extern "C" fn exynos_dp_resume(dev: *mut device) -> c_int {
    static int exynos_dp_resume(struct device *dev)
    {
    struct exynos_dp_device *dp = dev_get_drvdata(dev);
    return analogix_dp_resume(dp.adp);
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(exynos_dp_pm_ops, exynos_dp_suspend,
    exynos_dp_resume, core::ptr::null_mut());
    static const struct of_device_id exynos_dp_match[] = {
    { .compatible = "samsung,exynos5-dp" },
    {},
    };
    MODULE_DEVICE_TABLE(of, exynos_dp_match);
    struct platform_driver dp_driver = {
    .probe		= exynos_dp_probe,
    .remove		= exynos_dp_remove,
    .driver		= {
    .name	= "exynos-dp",
    .pm	= pm_ptr(&exynos_dp_pm_ops),
    .of_match_table = exynos_dp_match,
    },
    };
    MODULE_AUTHOR("Jingoo Han <jg1.han@samsung.com>");
    MODULE_DESCRIPTION("Samsung Specific Analogix-DP Driver Extension");
    MODULE_LICENSE("GPL v2");
