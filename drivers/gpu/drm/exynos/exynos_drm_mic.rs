//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/exynos/exynos_drm_mic.c
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
// Copyright (C) 2015 Samsung Electronics Co.Ltd
// Authors:
// Hyungwon Hwang <human.hwang@samsung.com>
//

// Sysreg registers for MIC
pub const DSD_CFG_MUX: c_uint = 0x1004;

// MIC registers
pub const MIC_OP: c_uint = 0x0;
pub const MIC_IP_VER: c_uint = 0x0004;
pub const MIC_V_TIMING_0: c_uint = 0x0008;
pub const MIC_V_TIMING_1: c_uint = 0x000C;
pub const MIC_IMG_SIZE: c_uint = 0x0010;
pub const MIC_INPUT_TIMING_0: c_uint = 0x0014;
pub const MIC_INPUT_TIMING_1: c_uint = 0x0018;
pub const MIC_2D_OUTPUT_TIMING_0: c_uint = 0x001C;
pub const MIC_2D_OUTPUT_TIMING_1: c_uint = 0x0020;
pub const MIC_2D_OUTPUT_TIMING_2: c_uint = 0x0024;
pub const MIC_3D_OUTPUT_TIMING_0: c_uint = 0x0028;
pub const MIC_3D_OUTPUT_TIMING_1: c_uint = 0x002C;
pub const MIC_3D_OUTPUT_TIMING_2: c_uint = 0x0030;
pub const MIC_CORE_PARA_0: c_uint = 0x0034;
pub const MIC_CORE_PARA_1: c_uint = 0x0038;
pub const MIC_CTC_CTRL: c_uint = 0x0040;
pub const MIC_RD_DATA: c_uint = 0x0044;

    static const char *const clk_names[] = { "pclk_mic0", "sclk_rgb_vclk_to_mic0" };

    static DEFINE_MUTEX(mic_mutex);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_mic {
    pub dev: *mut device,
    pub reg: *mut void __iomem,
    pub sysreg: *mut regmap,
    pub clks: [*mut clk; NUM_CLKS],
    pub i80_mode: bool,
    pub vm: videomode,
    pub bridge: drm_bridge,
    pub enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn mic_set_path(mic: *mut exynos_mic, enable: bool) {
    static void mic_set_path(struct exynos_mic *mic, bool enable)
    {
    int ret;
    unsigned int val;
    ret = regmap_read(mic.sysreg, DSD_CFG_MUX, &val);
    if (ret) {
    DRM_DEV_ERROR(mic.dev,
    "mic: Failed to read system register\n");
    return;
    }
    if (enable) {
    if (mic.i80_mode)
    val |= MIC0_I80_MUX;
    else
    val |= MIC0_RGB_MUX;
    val |=  MIC0_ON_MUX;
    } else
    val &= ~(MIC0_RGB_MUX | MIC0_I80_MUX | MIC0_ON_MUX);
    ret = regmap_write(mic.sysreg, DSD_CFG_MUX, val);
    if (ret)
    DRM_DEV_ERROR(mic.dev,
    "mic: Failed to read system register\n");
    }
#[no_mangle]
unsafe extern "C" fn mic_sw_reset(mic: *mut exynos_mic) -> c_int {
    static int mic_sw_reset(struct exynos_mic *mic)
    {
    let mut retry: c_uint = 100;
    int ret;
    writel(MIC_SW_RST, mic.reg + MIC_OP);
    while (retry-- > 0) {
    ret = readl(mic.reg + MIC_OP);
    if (!(ret & MIC_SW_RST))
    return 0;
    udelay(10);
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn mic_set_porch_timing(mic: *mut exynos_mic) {
    static void mic_set_porch_timing(struct exynos_mic *mic)
    {
    let mut vm: videomode = mic.vm;
    u32 reg;
    reg = MIC_V_PULSE_WIDTH(vm.vsync_len) +
    MIC_V_PERIOD_LINE(vm.vsync_len + vm.vactive +
    vm.vback_porch + vm.vfront_porch);
    writel(reg, mic.reg + MIC_V_TIMING_0);
    reg = MIC_VBP_SIZE(vm.vback_porch) +
    MIC_VFP_SIZE(vm.vfront_porch);
    writel(reg, mic.reg + MIC_V_TIMING_1);
    reg = MIC_V_PULSE_WIDTH(vm.hsync_len) +
    MIC_V_PERIOD_LINE(vm.hsync_len + vm.hactive +
    vm.hback_porch + vm.hfront_porch);
    writel(reg, mic.reg + MIC_INPUT_TIMING_0);
    reg = MIC_VBP_SIZE(vm.hback_porch) +
    MIC_VFP_SIZE(vm.hfront_porch);
    writel(reg, mic.reg + MIC_INPUT_TIMING_1);
    }
#[no_mangle]
unsafe extern "C" fn mic_set_img_size(mic: *mut exynos_mic) {
    static void mic_set_img_size(struct exynos_mic *mic)
    {
    struct videomode *vm = &mic.vm;
    u32 reg;
    reg = MIC_IMG_H_SIZE(vm.hactive) +
    MIC_IMG_V_SIZE(vm.vactive);
    writel(reg, mic.reg + MIC_IMG_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn mic_set_output_timing(mic: *mut exynos_mic) {
    static void mic_set_output_timing(struct exynos_mic *mic)
    {
    let mut vm: videomode = mic.vm;
    u32 reg, bs_size_2d;
    DRM_DEV_DEBUG(mic.dev, "w: %u, h: %u\n", vm.hactive, vm.vactive);
    bs_size_2d = ((vm.hactive >> 2) << 1) + (vm.vactive % 4);
    reg = MIC_BS_SIZE_2D(bs_size_2d);
    writel(reg, mic.reg + MIC_2D_OUTPUT_TIMING_2);
    if (!mic.i80_mode) {
    reg = MIC_H_PULSE_WIDTH_2D(vm.hsync_len) +
    MIC_H_PERIOD_PIXEL_2D(vm.hsync_len + bs_size_2d +
    vm.hback_porch + vm.hfront_porch);
    writel(reg, mic.reg + MIC_2D_OUTPUT_TIMING_0);
    reg = MIC_HBP_SIZE_2D(vm.hback_porch) +
    MIC_H_PERIOD_PIXEL_2D(vm.hfront_porch);
    writel(reg, mic.reg + MIC_2D_OUTPUT_TIMING_1);
    }
    }
#[no_mangle]
unsafe extern "C" fn mic_set_reg_on(mic: *mut exynos_mic, enable: bool) {
    static void mic_set_reg_on(struct exynos_mic *mic, bool enable)
    {
    let mut reg: u32 = readl(mic.reg + MIC_OP);
    if (enable) {
    reg &= ~(MIC_MODE_SEL_MASK | MIC_CORE_VER_CONTROL | MIC_PSR_EN);
    reg |= (MIC_CORE_EN | MIC_BS_CHG_OUT | MIC_ON_REG);
    reg  &= ~MIC_MODE_SEL_COMMAND_MODE;
    if (mic.i80_mode)
    reg |= MIC_MODE_SEL_COMMAND_MODE;
    } else {
    reg &= ~MIC_CORE_EN;
    }
    reg |= MIC_UPD_REG;
    writel(reg, mic.reg + MIC_OP);
    }
    static void mic_post_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
    struct exynos_mic *mic = bridge.driver_private;
    mutex_lock(&mic_mutex);
    if (!mic.enabled)
    goto already_disabled;
    mic_set_path(mic, 0);
    pm_runtime_put(mic.dev);
    mic.enabled = 0;
    already_disabled:
    mutex_unlock(&mic_mutex);
    }
    static void mic_mode_set(struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    struct exynos_mic *mic = bridge.driver_private;
    mutex_lock(&mic_mutex);
    drm_display_mode_to_videomode(mode, &mic.vm);
    mic.i80_mode = to_exynos_crtc(bridge.encoder.crtc).i80_mode;
    mutex_unlock(&mic_mutex);
    }
    static void mic_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
    struct exynos_mic *mic = bridge.driver_private;
    int ret;
    mutex_lock(&mic_mutex);
    if (mic.enabled)
    goto unlock;
    ret = pm_runtime_resume_and_get(mic.dev);
    if (ret < 0)
    goto unlock;
    mic_set_path(mic, 1);
    ret = mic_sw_reset(mic);
    if (ret) {
    DRM_DEV_ERROR(mic.dev, "Failed to reset\n");
    goto turn_off;
    }
    if (!mic.i80_mode)
    mic_set_porch_timing(mic);
    mic_set_img_size(mic);
    mic_set_output_timing(mic);
    mic_set_reg_on(mic, 1);
    mic.enabled = 1;
    mutex_unlock(&mic_mutex);
    return;
    turn_off:
    pm_runtime_put(mic.dev);
    unlock:
    mutex_unlock(&mic_mutex);
    }
    static const struct drm_bridge_funcs mic_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_post_disable = mic_post_disable,
    .mode_set = mic_mode_set,
    .atomic_pre_enable = mic_pre_enable,
    };
    static int exynos_mic_bind(struct device *dev, struct device *master,
    void *data)
    {
    struct exynos_mic *mic = dev_get_drvdata(dev);
    struct drm_device *drm_dev = data;
    struct exynos_drm_crtc *crtc = exynos_drm_crtc_get_by_type(drm_dev,
    EXYNOS_DISPLAY_TYPE_LCD);
    struct drm_encoder *e, *encoder = core::ptr::null_mut();
    drm_for_each_encoder(e, drm_dev)
    if (e.possible_crtcs == drm_crtc_mask(&crtc.base))
    encoder = e;
    if (!encoder)
    return -ENODEV;
    mic.bridge.driver_private = mic;
    return drm_bridge_attach(encoder, &mic.bridge, core::ptr::null_mut(), 0);
    }
    static void exynos_mic_unbind(struct device *dev, struct device *master,
    void *data)
    {
    struct exynos_mic *mic = dev_get_drvdata(dev);
    mutex_lock(&mic_mutex);
    if (!mic.enabled)
    goto already_disabled;
    pm_runtime_put(mic.dev);
    already_disabled:
    mutex_unlock(&mic_mutex);
    }
    static const struct component_ops exynos_mic_component_ops = {
    .bind	= exynos_mic_bind,
    .unbind	= exynos_mic_unbind,
    };
#[no_mangle]
unsafe extern "C" fn exynos_mic_suspend(dev: *mut device) -> c_int {
    static int exynos_mic_suspend(struct device *dev)
    {
    struct exynos_mic *mic = dev_get_drvdata(dev);
    int i;
    for (i = NUM_CLKS - 1; i > -1; i--)
    clk_disable_unprepare(mic.clks[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_mic_resume(dev: *mut device) -> c_int {
    static int exynos_mic_resume(struct device *dev)
    {
    struct exynos_mic *mic = dev_get_drvdata(dev);
    int ret, i;
    for (i = 0; i < NUM_CLKS; i++) {
    ret = clk_prepare_enable(mic.clks[i]);
    if (ret < 0) {
    DRM_DEV_ERROR(dev, "Failed to enable clock (%s)\n",
    clk_names[i]);
    while (--i > -1)
    clk_disable_unprepare(mic.clks[i]);
    return ret;
    }
    }
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(exynos_mic_pm_ops, exynos_mic_suspend,
    exynos_mic_resume, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn exynos_mic_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_mic_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct exynos_mic *mic;
    struct resource res;
    int ret, i;
    mic = devm_drm_bridge_alloc(dev, struct exynos_mic, bridge, &mic_bridge_funcs);
    if (IS_ERR(mic)) {
    DRM_DEV_ERROR(dev,
    "mic: Failed to allocate memory for MIC object\n");
    ret = PTR_ERR(mic);
    goto err;
    }
    mic.dev = dev;
    ret = of_address_to_resource(dev.of_node, 0, &res);
    if (ret) {
    DRM_DEV_ERROR(dev, "mic: Failed to get mem region for MIC\n");
    goto err;
    }
    mic.reg = devm_ioremap(dev, res.start, resource_size(&res));
    if (!mic.reg) {
    DRM_DEV_ERROR(dev, "mic: Failed to remap for MIC\n");
    ret = -ENOMEM;
    goto err;
    }
    mic.sysreg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "samsung,disp-syscon");
    if (IS_ERR(mic.sysreg)) {
    DRM_DEV_ERROR(dev, "mic: Failed to get system register.\n");
    ret = PTR_ERR(mic.sysreg);
    goto err;
    }
    for (i = 0; i < NUM_CLKS; i++) {
    mic.clks[i] = devm_clk_get(dev, clk_names[i]);
    if (IS_ERR(mic.clks[i])) {
    DRM_DEV_ERROR(dev, "mic: Failed to get clock (%s)\n",
    clk_names[i]);
    ret = PTR_ERR(mic.clks[i]);
    goto err;
    }
    }
    platform_set_drvdata(pdev, mic);
    mic.bridge.of_node = dev.of_node;
    ret = devm_drm_bridge_add(dev, &mic.bridge);
    if (ret)
    goto err;
    pm_runtime_enable(dev);
    ret = component_add(dev, &exynos_mic_component_ops);
    if (ret)
    goto err_pm;
    DRM_DEV_DEBUG_KMS(dev, "MIC has been probed\n");
    return 0;
    err_pm:
    pm_runtime_disable(dev);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_mic_remove(pdev: *mut platform_device) {
    static void exynos_mic_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &exynos_mic_component_ops);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id exynos_mic_of_match[] = {
    { .compatible = "samsung,exynos5433-mic" },
    { }
    };
    MODULE_DEVICE_TABLE(of, exynos_mic_of_match);
    struct platform_driver mic_driver = {
    .probe		= exynos_mic_probe,
    .remove		= exynos_mic_remove,
    .driver		= {
    .name	= "exynos-mic",
    .pm	= pm_ptr(&exynos_mic_pm_ops),
    .of_match_table = exynos_mic_of_match,
    },
    };
