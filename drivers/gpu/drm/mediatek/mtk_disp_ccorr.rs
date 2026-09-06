//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_disp_ccorr.c
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

pub const DISP_CCORR_EN: c_uint = 0x0000;

pub const DISP_CCORR_CFG: c_uint = 0x0020;

pub const DISP_CCORR_SIZE: c_uint = 0x0030;
pub const DISP_CCORR_COEF_0: c_uint = 0x0080;
pub const DISP_CCORR_COEF_1: c_uint = 0x0084;
pub const DISP_CCORR_COEF_2: c_uint = 0x0088;
pub const DISP_CCORR_COEF_3: c_uint = 0x008C;
pub const DISP_CCORR_COEF_4: c_uint = 0x0090;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_ccorr_data {
    pub matrix_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_ccorr {
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub cmdq_reg: cmdq_client_reg,
    pub data: *const mtk_disp_ccorr_data,
}

#[no_mangle]
pub unsafe extern "C" fn mtk_ccorr_clk_enable(dev: *mut device) -> c_int {
    int mtk_ccorr_clk_enable(struct device *dev)
    {
    struct mtk_disp_ccorr *ccorr = dev_get_drvdata(dev);
    return clk_prepare_enable(ccorr.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_ccorr_clk_disable(dev: *mut device) {
    void mtk_ccorr_clk_disable(struct device *dev)
    {
    struct mtk_disp_ccorr *ccorr = dev_get_drvdata(dev);
    clk_disable_unprepare(ccorr.clk);
    }
    void mtk_ccorr_config(struct device *dev, unsigned int w,
    unsigned int h, unsigned int vrefresh,
    unsigned int bpc, struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_ccorr *ccorr = dev_get_drvdata(dev);
    mtk_ddp_write(cmdq_pkt, w << 16 | h, &ccorr.cmdq_reg, ccorr.regs,
    DISP_CCORR_SIZE);
    mtk_ddp_write(cmdq_pkt, CCORR_ENGINE_EN, &ccorr.cmdq_reg, ccorr.regs,
    DISP_CCORR_CFG);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_ccorr_start(dev: *mut device) {
    void mtk_ccorr_start(struct device *dev)
    {
    struct mtk_disp_ccorr *ccorr = dev_get_drvdata(dev);
    writel(CCORR_EN, ccorr.regs + DISP_CCORR_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_ccorr_stop(dev: *mut device) {
    void mtk_ccorr_stop(struct device *dev)
    {
    struct mtk_disp_ccorr *ccorr = dev_get_drvdata(dev);
    writel_relaxed(0x0, ccorr.regs + DISP_CCORR_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_ccorr_ctm_set(dev: *mut device, state: *mut drm_crtc_state) {
    void mtk_ccorr_ctm_set(struct device *dev, struct drm_crtc_state *state)
    {
    struct mtk_disp_ccorr *ccorr = dev_get_drvdata(dev);
    struct drm_property_blob *blob = state.ctm;
    struct drm_color_ctm *ctm;
    const u64 *input;
    uint16_t coeffs[9] = { 0 };
    int i;
    struct cmdq_pkt *cmdq_pkt = core::ptr::null_mut();
    let mut matrix_bits: u32 = ccorr.data.matrix_bits;
    if (!blob)
    return;
    ctm = (struct drm_color_ctm *)blob.data;
    input = ctm.matrix;
    for (i = 0; i < ARRAY_SIZE(coeffs); i++)
    coeffs[i] = drm_color_ctm_s31_32_to_qm_n(input[i], 2, matrix_bits);
    mtk_ddp_write(cmdq_pkt, coeffs[0] << 16 | coeffs[1],
    &ccorr.cmdq_reg, ccorr.regs, DISP_CCORR_COEF_0);
    mtk_ddp_write(cmdq_pkt, coeffs[2] << 16 | coeffs[3],
    &ccorr.cmdq_reg, ccorr.regs, DISP_CCORR_COEF_1);
    mtk_ddp_write(cmdq_pkt, coeffs[4] << 16 | coeffs[5],
    &ccorr.cmdq_reg, ccorr.regs, DISP_CCORR_COEF_2);
    mtk_ddp_write(cmdq_pkt, coeffs[6] << 16 | coeffs[7],
    &ccorr.cmdq_reg, ccorr.regs, DISP_CCORR_COEF_3);
    mtk_ddp_write(cmdq_pkt, coeffs[8] << 16,
    &ccorr.cmdq_reg, ccorr.regs, DISP_CCORR_COEF_4);
    }
    static int mtk_disp_ccorr_bind(struct device *dev, struct device *master,
    void *data)
    {
    return 0;
    }
    static void mtk_disp_ccorr_unbind(struct device *dev, struct device *master,
    void *data)
    {
    }
    static const struct component_ops mtk_disp_ccorr_component_ops = {
    .bind	= mtk_disp_ccorr_bind,
    .unbind	= mtk_disp_ccorr_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_disp_ccorr_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_disp_ccorr_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_disp_ccorr *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get ccorr clk\n");
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "failed to ioremap ccorr\n");

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "get mediatek,gce-client-reg fail!\n");

    priv.data = of_device_get_match_data(dev);
    platform_set_drvdata(pdev, priv);
    ret = component_add(dev, &mtk_disp_ccorr_component_ops);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_disp_ccorr_remove(pdev: *mut platform_device) {
    static void mtk_disp_ccorr_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_disp_ccorr_component_ops);
    }
    static const struct mtk_disp_ccorr_data mt8183_ccorr_driver_data = {
    .matrix_bits = 10,
    };
    static const struct mtk_disp_ccorr_data mt8192_ccorr_driver_data = {
    .matrix_bits = 11,
    };
    static const struct of_device_id mtk_disp_ccorr_driver_dt_match[] = {
    { .compatible = "mediatek,mt8183-disp-ccorr",
    .data = &mt8183_ccorr_driver_data},
    { .compatible = "mediatek,mt8192-disp-ccorr",
    .data = &mt8192_ccorr_driver_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_disp_ccorr_driver_dt_match);
    struct platform_driver mtk_disp_ccorr_driver = {
    .probe		= mtk_disp_ccorr_probe,
    .remove		= mtk_disp_ccorr_remove,
    .driver		= {
    .name	= "mediatek-disp-ccorr",
    .of_match_table = mtk_disp_ccorr_driver_dt_match,
    },
    };
