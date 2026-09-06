//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8196-mcu.c
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
// Copyright (c) 2025 MediaTek Inc.
// Guangjie Song <guangjie.song@mediatek.com>
// Copyright (c) 2025 Collabora Ltd.
// Laura Nao <laura.nao@collabora.com>
//

pub const ARMPLL_LL_CON0: c_uint = 0x008;
pub const ARMPLL_LL_CON1: c_uint = 0x00c;
pub const ARMPLL_LL_CON2: c_uint = 0x010;
pub const ARMPLL_LL_CON3: c_uint = 0x014;
pub const ARMPLL_BL_CON0: c_uint = 0x008;
pub const ARMPLL_BL_CON1: c_uint = 0x00c;
pub const ARMPLL_BL_CON2: c_uint = 0x010;
pub const ARMPLL_BL_CON3: c_uint = 0x014;
pub const ARMPLL_B_CON0: c_uint = 0x008;
pub const ARMPLL_B_CON1: c_uint = 0x00c;
pub const ARMPLL_B_CON2: c_uint = 0x010;
pub const ARMPLL_B_CON3: c_uint = 0x014;
pub const CCIPLL_CON0: c_uint = 0x008;
pub const CCIPLL_CON1: c_uint = 0x00c;
pub const CCIPLL_CON2: c_uint = 0x010;
pub const CCIPLL_CON3: c_uint = 0x014;
pub const PTPPLL_CON0: c_uint = 0x008;
pub const PTPPLL_CON1: c_uint = 0x00c;
pub const PTPPLL_CON2: c_uint = 0x010;
pub const PTPPLL_CON3: c_uint = 0x014;

pub const MT8196_INTEGER_BITS: c_int = 8;

    _flags, _rst_bar_mask,				\
    _pd_reg, _pd_shift, _tuner_reg,			\
    _tuner_en_reg, _tuner_en_bit,			\
    _pcw_reg, _pcw_shift, _pcwbits) {			\
    .id = _id,					\
    .name = _name,					\
    .reg = _reg,					\
    .en_reg = _en_reg,				\
    .en_mask = _en_mask,				\
    .pll_en_bit = _pll_en_bit,			\
    .flags = _flags,				\
    .rst_bar_mask = _rst_bar_mask,			\
    .fmax = MT8196_PLL_FMAX,			\
    .fmin = MT8196_PLL_FMIN,			\
    .pd_reg = _pd_reg,				\
    .pd_shift = _pd_shift,				\
    .tuner_reg = _tuner_reg,			\
    .tuner_en_reg = _tuner_en_reg,			\
    .tuner_en_bit = _tuner_en_bit,			\
    .pcw_reg = _pcw_reg,				\
    .pcw_shift = _pcw_shift,			\
    .pcwbits = _pcwbits,				\
    .pcwibits = MT8196_INTEGER_BITS,		\
    }
    static const struct mtk_pll_data cpu_bl_plls[] = {
    PLL(CLK_CPBL_ARMPLL_BL, "armpll-bl", ARMPLL_BL_CON0, ARMPLL_BL_CON0, 0,
    0, PLL_AO, BIT(0), ARMPLL_BL_CON1, 24, 0, 0, 0, ARMPLL_BL_CON1, 0, 22),
    };
    static const struct mtk_pll_data cpu_b_plls[] = {
    PLL(CLK_CPB_ARMPLL_B, "armpll-b", ARMPLL_B_CON0, ARMPLL_B_CON0, 0, 0,
    PLL_AO, BIT(0), ARMPLL_B_CON1, 24, 0, 0, 0, ARMPLL_B_CON1, 0, 22),
    };
    static const struct mtk_pll_data cpu_ll_plls[] = {
    PLL(CLK_CPLL_ARMPLL_LL, "armpll-ll", ARMPLL_LL_CON0, ARMPLL_LL_CON0, 0,
    0, PLL_AO, BIT(0), ARMPLL_LL_CON1, 24, 0, 0, 0, ARMPLL_LL_CON1, 0, 22),
    };
    static const struct mtk_pll_data cci_plls[] = {
    PLL(CLK_CCIPLL, "ccipll", CCIPLL_CON0, CCIPLL_CON0, 0, 0, PLL_AO,
    BIT(0), CCIPLL_CON1, 24, 0, 0, 0, CCIPLL_CON1, 0, 22),
    };
    static const struct mtk_pll_data ptp_plls[] = {
    PLL(CLK_PTPPLL, "ptppll", PTPPLL_CON0, PTPPLL_CON0, 0, 0, PLL_AO,
    BIT(0), PTPPLL_CON1, 24, 0, 0, 0, PTPPLL_CON1, 0, 22),
    };
    static const struct of_device_id of_match_clk_mt8196_mcu[] = {
    { .compatible = "mediatek,mt8196-armpll-bl-pll-ctrl",
    .data = &cpu_bl_plls },
    { .compatible = "mediatek,mt8196-armpll-b-pll-ctrl",
    .data = &cpu_b_plls },
    { .compatible = "mediatek,mt8196-armpll-ll-pll-ctrl",
    .data = &cpu_ll_plls },
    { .compatible = "mediatek,mt8196-ccipll-pll-ctrl", .data = &cci_plls },
    { .compatible = "mediatek,mt8196-ptppll-pll-ctrl", .data = &ptp_plls },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_mcu);
#[no_mangle]
unsafe extern "C" fn clk_mt8196_mcu_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8196_mcu_probe(struct platform_device *pdev)
    {
    const struct mtk_pll_data *plls;
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    let mut num_plls: c_int = 1;
    int r;
    plls = of_device_get_match_data(&pdev.dev);
    if (!plls)
    return -EINVAL;
    clk_data = mtk_alloc_clk_data(num_plls);
    if (!clk_data)
    return -ENOMEM;
    r = mtk_clk_register_plls(&pdev.dev, plls, num_plls, clk_data);
    if (r)
    goto free_clk_data;
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r)
    goto unregister_plls;
    platform_set_drvdata(pdev, clk_data);
    return r;
    unregister_plls:
    mtk_clk_unregister_plls(plls, num_plls, clk_data);
    free_clk_data:
    mtk_free_clk_data(clk_data);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8196_mcu_remove(pdev: *mut platform_device) {
    static void clk_mt8196_mcu_remove(struct platform_device *pdev)
    {
    const struct mtk_pll_data *plls = of_device_get_match_data(&pdev.dev);
    struct clk_hw_onecell_data *clk_data = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(plls, 1, clk_data);
    mtk_free_clk_data(clk_data);
    }
    static struct platform_driver clk_mt8196_mcu_drv = {
    .probe = clk_mt8196_mcu_probe,
    .remove = clk_mt8196_mcu_remove,
    .driver = {
    .name = "clk-mt8196-mcu",
    .of_match_table = of_match_clk_mt8196_mcu,
    },
    };
    module_platform_driver(clk_mt8196_mcu_drv);
    MODULE_DESCRIPTION("MediaTek MT8196 mcusys clocks driver");
    MODULE_LICENSE("GPL");
