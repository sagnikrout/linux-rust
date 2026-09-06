//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_padding.c
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
// Copyright (c) 2023 MediaTek Inc.
//

pub const PADDING_CONTROL_REG: c_uint = 0x00;

pub const PADDING_PIC_SIZE_REG: c_uint = 0x04;
pub const PADDING_H_REG: c_uint = 0x08 /* horizontal */;
pub const PADDING_V_REG: c_uint = 0x0c /* vertical */;
pub const PADDING_COLOR_REG: c_uint = 0x10;
//
// struct mtk_padding - Basic information of the Padding
// @clk: Clock of the module
// @reg: Virtual address of the Padding for CPU to access
// @cmdq_reg: CMDQ setting of the Padding
//
// Every Padding should have different clock source, register base, and
// CMDQ settings, we stored these differences all together.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_padding {
    pub clk: *mut clk,
    pub reg: *mut void __iomem,
    pub cmdq_reg: cmdq_client_reg,
}

#[no_mangle]
pub unsafe extern "C" fn mtk_padding_clk_enable(dev: *mut device) -> c_int {
    int mtk_padding_clk_enable(struct device *dev)
    {
    struct mtk_padding *padding = dev_get_drvdata(dev);
    return clk_prepare_enable(padding.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_padding_clk_disable(dev: *mut device) {
    void mtk_padding_clk_disable(struct device *dev)
    {
    struct mtk_padding *padding = dev_get_drvdata(dev);
    clk_disable_unprepare(padding.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_padding_start(dev: *mut device) {
    void mtk_padding_start(struct device *dev)
    {
    struct mtk_padding *padding = dev_get_drvdata(dev);
    writel(PADDING_ENABLE | PADDING_BYPASS,
    padding.reg + PADDING_CONTROL_REG);
//
// Notice that even the padding is in bypass mode,
// all the settings must be cleared to 0 or
// undefined behaviors could happen
//
    writel(0, padding.reg + PADDING_PIC_SIZE_REG);
    writel(0, padding.reg + PADDING_H_REG);
    writel(0, padding.reg + PADDING_V_REG);
    writel(0, padding.reg + PADDING_COLOR_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_padding_stop(dev: *mut device) {
    void mtk_padding_stop(struct device *dev)
    {
    struct mtk_padding *padding = dev_get_drvdata(dev);
    writel(0, padding.reg + PADDING_CONTROL_REG);
    }
#[no_mangle]
unsafe extern "C" fn mtk_padding_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int mtk_padding_bind(struct device *dev, struct device *master, void *data)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_padding_unbind(dev: *mut device, master: *mut device, data: *mut c_void) {
    static void mtk_padding_unbind(struct device *dev, struct device *master, void *data)
    {
    }
    static const struct component_ops mtk_padding_component_ops = {
    .bind	= mtk_padding_bind,
    .unbind = mtk_padding_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_padding_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_padding_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_padding *priv;
    struct resource *res;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get clk\n");
    priv.reg = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.reg))
    return dev_err_probe(dev, PTR_ERR(priv.reg),
    "failed to do ioremap\n");

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get gce client reg\n");

    platform_set_drvdata(pdev, priv);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    ret = component_add(dev, &mtk_padding_component_ops);
    if (ret) {
    pm_runtime_disable(dev);
    return dev_err_probe(dev, ret, "failed to add component\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_padding_remove(pdev: *mut platform_device) {
    static void mtk_padding_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_padding_component_ops);
    }
    static const struct of_device_id mtk_padding_driver_dt_match[] = {
    { .compatible = "mediatek,mt8188-disp-padding" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mtk_padding_driver_dt_match);
    struct platform_driver mtk_padding_driver = {
    .probe		= mtk_padding_probe,
    .remove		= mtk_padding_remove,
    .driver		= {
    .name	= "mediatek-disp-padding",
    .of_match_table = mtk_padding_driver_dt_match,
    },
    };
