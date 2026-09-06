//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_disp_color.c
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
// Copyright (c) 2017 MediaTek Inc.
//

pub const DISP_COLOR_CFG_MAIN: c_uint = 0x0400;
pub const DISP_COLOR_START_MT2701: c_uint = 0x0f00;
pub const DISP_COLOR_START_MT8167: c_uint = 0x0400;
pub const DISP_COLOR_START_MT8173: c_uint = 0x0c00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_color_data {
    pub color_offset: c_uint,
}

//
// struct mtk_disp_color - DISP_COLOR driver structure
// @crtc: associated crtc to report irq events to
// @data: platform colour driver data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_color {
    pub crtc: *mut drm_crtc,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub cmdq_reg: cmdq_client_reg,
    pub data: *const mtk_disp_color_data,
}

#[no_mangle]
pub unsafe extern "C" fn mtk_color_clk_enable(dev: *mut device) -> c_int {
    int mtk_color_clk_enable(struct device *dev)
    {
    struct mtk_disp_color *color = dev_get_drvdata(dev);
    return clk_prepare_enable(color.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_color_clk_disable(dev: *mut device) {
    void mtk_color_clk_disable(struct device *dev)
    {
    struct mtk_disp_color *color = dev_get_drvdata(dev);
    clk_disable_unprepare(color.clk);
    }
    void mtk_color_config(struct device *dev, unsigned int w,
    unsigned int h, unsigned int vrefresh,
    unsigned int bpc, struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_color *color = dev_get_drvdata(dev);
    mtk_ddp_write(cmdq_pkt, w, &color.cmdq_reg, color.regs, DISP_COLOR_WIDTH(color));
    mtk_ddp_write(cmdq_pkt, h, &color.cmdq_reg, color.regs, DISP_COLOR_HEIGHT(color));
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_color_start(dev: *mut device) {
    void mtk_color_start(struct device *dev)
    {
    struct mtk_disp_color *color = dev_get_drvdata(dev);
    writel(COLOR_BYPASS_ALL | COLOR_SEQ_SEL,
    color.regs + DISP_COLOR_CFG_MAIN);
    writel(0x1, color.regs + DISP_COLOR_START(color));
    }
    static int mtk_disp_color_bind(struct device *dev, struct device *master,
    void *data)
    {
    return 0;
    }
    static void mtk_disp_color_unbind(struct device *dev, struct device *master,
    void *data)
    {
    }
    static const struct component_ops mtk_disp_color_component_ops = {
    .bind	= mtk_disp_color_bind,
    .unbind = mtk_disp_color_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_disp_color_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_disp_color_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_disp_color *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get color clk\n");
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "failed to ioremap color\n");

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "get mediatek,gce-client-reg fail!\n");

    priv.data = of_device_get_match_data(dev);
    platform_set_drvdata(pdev, priv);
    ret = component_add(dev, &mtk_disp_color_component_ops);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_disp_color_remove(pdev: *mut platform_device) {
    static void mtk_disp_color_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_disp_color_component_ops);
    }
    static const struct mtk_disp_color_data mt2701_color_driver_data = {
    .color_offset = DISP_COLOR_START_MT2701,
    };
    static const struct mtk_disp_color_data mt8167_color_driver_data = {
    .color_offset = DISP_COLOR_START_MT8167,
    };
    static const struct mtk_disp_color_data mt8173_color_driver_data = {
    .color_offset = DISP_COLOR_START_MT8173,
    };
    static const struct of_device_id mtk_disp_color_driver_dt_match[] = {
    { .compatible = "mediatek,mt2701-disp-color",
    .data = &mt2701_color_driver_data},
    { .compatible = "mediatek,mt8167-disp-color",
    .data = &mt8167_color_driver_data},
    { .compatible = "mediatek,mt8173-disp-color",
    .data = &mt8173_color_driver_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_disp_color_driver_dt_match);
    struct platform_driver mtk_disp_color_driver = {
    .probe		= mtk_disp_color_probe,
    .remove		= mtk_disp_color_remove,
    .driver		= {
    .name	= "mediatek-disp-color",
    .of_match_table = mtk_disp_color_driver_dt_match,
    },
    };
