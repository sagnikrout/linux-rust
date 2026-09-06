//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_disp_aal.c
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
// Copyright (c) 2021 MediaTek Inc.
//

pub const DISP_AAL_EN: c_uint = 0x0000;

pub const DISP_AAL_CFG: c_uint = 0x0020;

pub const DISP_AAL_SIZE: c_uint = 0x0030;

pub const DISP_AAL_OUTPUT_SIZE: c_uint = 0x04d8;
pub const DISP_AAL_GAMMA_LUT: c_uint = 0x0700;

pub const DISP_AAL_LUT_BITS: c_int = 10;
pub const DISP_AAL_LUT_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_aal_data {
    pub has_gamma: bool,
}

//
// struct mtk_disp_aal - Display Adaptive Ambient Light driver structure
// @clk:      clock for DISP_AAL controller
// @regs:     MMIO registers base
// @cmdq_reg: CMDQ Client register
// @data:     platform specific data for DISP_AAL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_aal {
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub cmdq_reg: cmdq_client_reg,
    pub data: *const mtk_disp_aal_data,
}

#[no_mangle]
pub unsafe extern "C" fn mtk_aal_clk_enable(dev: *mut device) -> c_int {
    int mtk_aal_clk_enable(struct device *dev)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    return clk_prepare_enable(aal.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_aal_clk_disable(dev: *mut device) {
    void mtk_aal_clk_disable(struct device *dev)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    clk_disable_unprepare(aal.clk);
    }
    void mtk_aal_config(struct device *dev, unsigned int w,
    unsigned int h, unsigned int vrefresh,
    unsigned int bpc, struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    u32 sz;
    sz = FIELD_PREP(DISP_AAL_SIZE_HSIZE, w);
    sz |= FIELD_PREP(DISP_AAL_SIZE_VSIZE, h);
    mtk_ddp_write(cmdq_pkt, sz, &aal.cmdq_reg, aal.regs, DISP_AAL_SIZE);
    mtk_ddp_write(cmdq_pkt, sz, &aal.cmdq_reg, aal.regs, DISP_AAL_OUTPUT_SIZE);
    }
//
// mtk_aal_gamma_get_lut_size() - Get gamma LUT size for AAL
// @dev: Pointer to struct device
//
// Return: 0 if gamma control not supported in AAL or gamma LUT size
//
#[no_mangle]
pub unsafe extern "C" fn mtk_aal_gamma_get_lut_size(dev: *mut device) -> c_uint {
    unsigned int mtk_aal_gamma_get_lut_size(struct device *dev)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    if (aal.data && aal.data.has_gamma)
    return DISP_AAL_LUT_SIZE;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_aal_gamma_set(dev: *mut device, state: *mut drm_crtc_state) {
    void mtk_aal_gamma_set(struct device *dev, struct drm_crtc_state *state)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    struct drm_color_lut *lut;
    unsigned int i;
    u32 cfg_val;
// If gamma is not supported in AAL, go out immediately
    if (!(aal.data && aal.data.has_gamma))
    return;
// Also, if there's no gamma lut there's nothing to do here.
    if (!state.gamma_lut)
    return;
    lut = (struct drm_color_lut *)state.gamma_lut.data;
    for (i = 0; i < DISP_AAL_LUT_SIZE; i++) {
    struct drm_color_lut hwlut = {
    .red = drm_color_lut_extract(lut[i].red, DISP_AAL_LUT_BITS),
    .green = drm_color_lut_extract(lut[i].green, DISP_AAL_LUT_BITS),
    .blue = drm_color_lut_extract(lut[i].blue, DISP_AAL_LUT_BITS)
    };
    u32 word;
    word = FIELD_PREP(DISP_AAL_GAMMA_LUT_R, hwlut.red);
    word |= FIELD_PREP(DISP_AAL_GAMMA_LUT_G, hwlut.green);
    word |= FIELD_PREP(DISP_AAL_GAMMA_LUT_B, hwlut.blue);
    writel(word, aal.regs + DISP_AAL_GAMMA_LUT + i * 4);
    }
    cfg_val = readl(aal.regs + DISP_AAL_CFG);
// Enable the gamma table
    cfg_val |= FIELD_PREP(AAL_GAMMA_LUT_EN, 1);
// Disable RELAY mode to pass the processed image
    cfg_val &= ~AAL_RELAY_MODE;
    writel(cfg_val, aal.regs + DISP_AAL_CFG);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_aal_start(dev: *mut device) {
    void mtk_aal_start(struct device *dev)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    writel(AAL_EN, aal.regs + DISP_AAL_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_aal_stop(dev: *mut device) {
    void mtk_aal_stop(struct device *dev)
    {
    struct mtk_disp_aal *aal = dev_get_drvdata(dev);
    writel_relaxed(0x0, aal.regs + DISP_AAL_EN);
    }
    static int mtk_disp_aal_bind(struct device *dev, struct device *master,
    void *data)
    {
    return 0;
    }
    static void mtk_disp_aal_unbind(struct device *dev, struct device *master,
    void *data)
    {
    }
    static const struct component_ops mtk_disp_aal_component_ops = {
    .bind	= mtk_disp_aal_bind,
    .unbind = mtk_disp_aal_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_disp_aal_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_disp_aal_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_disp_aal *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get aal clk\n");
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "failed to ioremap aal\n");

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "get mediatek,gce-client-reg fail!\n");

    priv.data = of_device_get_match_data(dev);
    platform_set_drvdata(pdev, priv);
    ret = component_add(dev, &mtk_disp_aal_component_ops);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_disp_aal_remove(pdev: *mut platform_device) {
    static void mtk_disp_aal_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_disp_aal_component_ops);
    }
    static const struct mtk_disp_aal_data mt8173_aal_driver_data = {
    .has_gamma = true,
    };
    static const struct of_device_id mtk_disp_aal_driver_dt_match[] = {
    { .compatible = "mediatek,mt8173-disp-aal", .data = &mt8173_aal_driver_data },
    { .compatible = "mediatek,mt8183-disp-aal" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mtk_disp_aal_driver_dt_match);
    struct platform_driver mtk_disp_aal_driver = {
    .probe		= mtk_disp_aal_probe,
    .remove		= mtk_disp_aal_remove,
    .driver		= {
    .name	= "mediatek-disp-aal",
    .of_match_table = mtk_disp_aal_driver_dt_match,
    },
    };
