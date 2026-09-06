//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_disp_gamma.c
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

pub const DISP_GAMMA_EN: c_uint = 0x0000;

pub const DISP_GAMMA_CFG: c_uint = 0x0020;

pub const DISP_GAMMA_SIZE: c_uint = 0x0030;

pub const DISP_GAMMA_BANK: c_uint = 0x0100;

pub const DISP_GAMMA_LUT: c_uint = 0x0700;
pub const DISP_GAMMA_LUT1: c_uint = 0x0b00;
// For 10 bit LUT layout, R/G/B are in the same register

// For 12 bit LUT layout, R/G are in LUT, B is in LUT1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_gamma_data {
    pub has_dither: bool,
    pub lut_diff: bool,
    pub lut_bank_size: u16,
    pub lut_size: u16,
    pub lut_bits: u8,
}

//
// struct mtk_disp_gamma - Display Gamma driver structure
// @clk:      clock for DISP_GAMMA block
// @regs:     MMIO registers base
// @cmdq_reg: CMDQ Client register
// @data:     platform data for DISP_GAMMA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_gamma {
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub cmdq_reg: cmdq_client_reg,
    pub data: *const mtk_disp_gamma_data,
}

#[no_mangle]
pub unsafe extern "C" fn mtk_gamma_clk_enable(dev: *mut device) -> c_int {
    int mtk_gamma_clk_enable(struct device *dev)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    return clk_prepare_enable(gamma.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_gamma_clk_disable(dev: *mut device) {
    void mtk_gamma_clk_disable(struct device *dev)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    clk_disable_unprepare(gamma.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_gamma_get_lut_size(dev: *mut device) -> c_uint {
    unsigned int mtk_gamma_get_lut_size(struct device *dev)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    if (gamma && gamma.data)
    return gamma.data.lut_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_gamma_lut_is_descending(lut: *mut drm_color_lut, lut_size: u32) -> bool {
    static bool mtk_gamma_lut_is_descending(struct drm_color_lut *lut, u32 lut_size)
    {
    u64 first, last;
    let mut last_entry: c_int = lut_size - 1;
    first = lut[0].red + lut[0].green + lut[0].blue;
    last = lut[last_entry].red + lut[last_entry].green + lut[last_entry].blue;
    return !!(first > last);
    }
//
// SoCs supporting 12-bits LUTs are using a new register layout that does
// always support (by HW) both 12-bits and 10-bits LUT but, on those, we
// ignore the support for 10-bits in this driver and always use 12-bits.
//
// Summarizing:
// - SoC HW support 9/10-bits LUT only
// - Old register layout
// - 10-bits LUT supported
// - 9-bits LUT not supported
// - SoC HW support both 10/12bits LUT
// - New register layout
// - 12-bits LUT supported
// - 10-its LUT not supported
//
#[no_mangle]
pub unsafe extern "C" fn mtk_gamma_set(dev: *mut device, state: *mut drm_crtc_state) {
    void mtk_gamma_set(struct device *dev, struct drm_crtc_state *state)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    void __iomem *lut0_base = gamma.regs + DISP_GAMMA_LUT;
    void __iomem *lut1_base = gamma.regs + DISP_GAMMA_LUT1;
    u32 cfg_val, data_mode, lbank_val, word[2];
    let mut lut_bits: u8 = gamma.data.lut_bits;
    int cur_bank, num_lut_banks;
    struct drm_color_lut *lut;
    unsigned int i;
// If there's no gamma lut there's nothing to do here.
    if (!state.gamma_lut)
    return;
    num_lut_banks = gamma.data.lut_size / gamma.data.lut_bank_size;
    lut = (struct drm_color_lut *)state.gamma_lut.data;
// Switch to 12 bits data mode if supported
    data_mode = FIELD_PREP(DISP_GAMMA_BANK_DATA_MODE, !!(lut_bits == 12));
    for (cur_bank = 0; cur_bank < num_lut_banks; cur_bank++) {
// Switch gamma bank and set data mode before writing LUT
    if (num_lut_banks > 1) {
    lbank_val = FIELD_PREP(DISP_GAMMA_BANK_BANK, cur_bank);
    lbank_val |= data_mode;
    writel(lbank_val, gamma.regs + DISP_GAMMA_BANK);
    }
    for (i = 0; i < gamma.data.lut_bank_size; i++) {
    let mut n: c_int = cur_bank * gamma.data.lut_bank_size + i;
    struct drm_color_lut diff, hwlut;
    hwlut.red = drm_color_lut_extract(lut[n].red, lut_bits);
    hwlut.green = drm_color_lut_extract(lut[n].green, lut_bits);
    hwlut.blue = drm_color_lut_extract(lut[n].blue, lut_bits);
    if (!gamma.data.lut_diff || (i % 2 == 0)) {
    if (lut_bits == 12) {
    word[0] = FIELD_PREP(DISP_GAMMA_LUT_12BIT_R, hwlut.red);
    word[0] |= FIELD_PREP(DISP_GAMMA_LUT_12BIT_G, hwlut.green);
    word[1] = FIELD_PREP(DISP_GAMMA_LUT_12BIT_B, hwlut.blue);
    } else {
    word[0] = FIELD_PREP(DISP_GAMMA_LUT_10BIT_R, hwlut.red);
    word[0] |= FIELD_PREP(DISP_GAMMA_LUT_10BIT_G, hwlut.green);
    word[0] |= FIELD_PREP(DISP_GAMMA_LUT_10BIT_B, hwlut.blue);
    }
    } else {
    diff.red = lut[n].red - lut[n - 1].red;
    diff.red = drm_color_lut_extract(diff.red, lut_bits);
    diff.green = lut[n].green - lut[n - 1].green;
    diff.green = drm_color_lut_extract(diff.green, lut_bits);
    diff.blue = lut[n].blue - lut[n - 1].blue;
    diff.blue = drm_color_lut_extract(diff.blue, lut_bits);
    if (lut_bits == 12) {
    word[0] = FIELD_PREP(DISP_GAMMA_LUT_12BIT_R, diff.red);
    word[0] |= FIELD_PREP(DISP_GAMMA_LUT_12BIT_G, diff.green);
    word[1] = FIELD_PREP(DISP_GAMMA_LUT_12BIT_B, diff.blue);
    } else {
    word[0] = FIELD_PREP(DISP_GAMMA_LUT_10BIT_R, diff.red);
    word[0] |= FIELD_PREP(DISP_GAMMA_LUT_10BIT_G, diff.green);
    word[0] |= FIELD_PREP(DISP_GAMMA_LUT_10BIT_B, diff.blue);
    }
    }
    writel(word[0], lut0_base + i * 4);
    if (lut_bits == 12)
    writel(word[1], lut1_base + i * 4);
    }
    }
    cfg_val = readl(gamma.regs + DISP_GAMMA_CFG);
    if (!gamma.data.has_dither) {
// Descending or Rising LUT
    if (mtk_gamma_lut_is_descending(lut, gamma.data.lut_size - 1))
    cfg_val |= FIELD_PREP(GAMMA_LUT_TYPE, 1);
    else
    cfg_val &= ~GAMMA_LUT_TYPE;
    }
// Enable the gamma table
    cfg_val |= FIELD_PREP(GAMMA_LUT_EN, 1);
// Disable RELAY mode to pass the processed image
    cfg_val &= ~GAMMA_RELAY_MODE;
    writel(cfg_val, gamma.regs + DISP_GAMMA_CFG);
    }
    void mtk_gamma_config(struct device *dev, unsigned int w,
    unsigned int h, unsigned int vrefresh,
    unsigned int bpc, struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    u32 sz;
    sz = FIELD_PREP(DISP_GAMMA_SIZE_HSIZE, w);
    sz |= FIELD_PREP(DISP_GAMMA_SIZE_VSIZE, h);
    mtk_ddp_write(cmdq_pkt, sz, &gamma.cmdq_reg, gamma.regs, DISP_GAMMA_SIZE);
    if (gamma.data && gamma.data.has_dither)
    mtk_dither_set_common(gamma.regs, &gamma.cmdq_reg, bpc,
    DISP_GAMMA_CFG, GAMMA_DITHERING, cmdq_pkt);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_gamma_start(dev: *mut device) {
    void mtk_gamma_start(struct device *dev)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    writel(GAMMA_EN, gamma.regs + DISP_GAMMA_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_gamma_stop(dev: *mut device) {
    void mtk_gamma_stop(struct device *dev)
    {
    struct mtk_disp_gamma *gamma = dev_get_drvdata(dev);
    writel_relaxed(0x0, gamma.regs + DISP_GAMMA_EN);
    }
    static int mtk_disp_gamma_bind(struct device *dev, struct device *master,
    void *data)
    {
    return 0;
    }
    static void mtk_disp_gamma_unbind(struct device *dev, struct device *master,
    void *data)
    {
    }
    static const struct component_ops mtk_disp_gamma_component_ops = {
    .bind	= mtk_disp_gamma_bind,
    .unbind = mtk_disp_gamma_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_disp_gamma_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_disp_gamma_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_disp_gamma *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get gamma clk\n");
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "failed to ioremap gamma\n");

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "get mediatek,gce-client-reg fail!\n");

    priv.data = of_device_get_match_data(dev);
    platform_set_drvdata(pdev, priv);
    ret = component_add(dev, &mtk_disp_gamma_component_ops);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_disp_gamma_remove(pdev: *mut platform_device) {
    static void mtk_disp_gamma_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_disp_gamma_component_ops);
    }
    static const struct mtk_disp_gamma_data mt8173_gamma_driver_data = {
    .has_dither = true,
    .lut_bank_size = 512,
    .lut_bits = 10,
    .lut_size = 512,
    };
    static const struct mtk_disp_gamma_data mt8183_gamma_driver_data = {
    .lut_bank_size = 512,
    .lut_bits = 10,
    .lut_diff = true,
    .lut_size = 512,
    };
    static const struct mtk_disp_gamma_data mt8195_gamma_driver_data = {
    .lut_bank_size = 256,
    .lut_bits = 12,
    .lut_diff = true,
    .lut_size = 1024,
    };
    static const struct of_device_id mtk_disp_gamma_driver_dt_match[] = {
    { .compatible = "mediatek,mt8173-disp-gamma",
    .data = &mt8173_gamma_driver_data},
    { .compatible = "mediatek,mt8183-disp-gamma",
    .data = &mt8183_gamma_driver_data},
    { .compatible = "mediatek,mt8195-disp-gamma",
    .data = &mt8195_gamma_driver_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_disp_gamma_driver_dt_match);
    struct platform_driver mtk_disp_gamma_driver = {
    .probe		= mtk_disp_gamma_probe,
    .remove		= mtk_disp_gamma_remove,
    .driver		= {
    .name	= "mediatek-disp-gamma",
    .of_match_table = mtk_disp_gamma_driver_dt_match,
    },
    };
