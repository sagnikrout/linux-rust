//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8195-apusys_pll.c
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
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

//
// The "en_reg" and "pcw_chg_reg" fields are standard offset register compared
// with "reg" field, so set zero to imply it.
// No tuner control in apu pll, so set "tuner_XXX" as zero to imply it.
// No rst or post divider enable in apu pll, so set "rst_bar_mask" and "en_mask"
// as zero to imply it.
//

    .id = _id,						\
    .name = _name,						\
    .reg = _reg,						\
    .pwr_reg = _pwr_reg,					\
    .en_mask = 0,						\
    .flags = 0,						\
    .rst_bar_mask = 0,					\
    .fmax = MT8195_PLL_FMAX,				\
    .fmin = MT8195_PLL_FMIN,				\
    .pcwbits = MT8195_PCW_BITS,				\
    .pcwibits = MT8195_INTEGER_BITS,			\
    .pd_reg = _pd_reg,					\
    .pd_shift = MT8195_POSDIV_SHIFT,			\
    .tuner_reg = 0,						\
    .tuner_en_reg = 0,					\
    .tuner_en_bit = 0,					\
    .pcw_reg = _pcw_reg,					\
    .pcw_shift = MT8195_PCW_SHIFT,				\
    .pcw_chg_reg = 0,					\
    .en_reg = 0,						\
    .pll_en_bit = MT8195_PLL_EN_BIT,			\
    }
    static const struct mtk_pll_data apusys_plls[] = {
    PLL(CLK_APUSYS_PLL_APUPLL, "apusys_pll_apupll", 0x008, 0x014, 0x00c, 0x00c),
    PLL(CLK_APUSYS_PLL_NPUPLL, "apusys_pll_npupll", 0x018, 0x024, 0x01c, 0x01c),
    PLL(CLK_APUSYS_PLL_APUPLL1, "apusys_pll_apupll1", 0x028, 0x034, 0x02c, 0x02c),
    PLL(CLK_APUSYS_PLL_APUPLL2, "apusys_pll_apupll2", 0x038, 0x044, 0x03c, 0x03c),
    };
#[no_mangle]
unsafe extern "C" fn clk_mt8195_apusys_pll_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8195_apusys_pll_probe(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    int r;
    clk_data = mtk_alloc_clk_data(CLK_APUSYS_PLL_NR_CLK);
    if (!clk_data)
    return -ENOMEM;
    r = mtk_clk_register_plls(&pdev.dev, apusys_plls,
    ARRAY_SIZE(apusys_plls), clk_data);
    if (r)
    goto free_apusys_pll_data;
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r)
    goto unregister_plls;
    platform_set_drvdata(pdev, clk_data);
    return r;
    unregister_plls:
    mtk_clk_unregister_plls(apusys_plls, ARRAY_SIZE(apusys_plls), clk_data);
    free_apusys_pll_data:
    mtk_free_clk_data(clk_data);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8195_apusys_pll_remove(pdev: *mut platform_device) {
    static void clk_mt8195_apusys_pll_remove(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(apusys_plls, ARRAY_SIZE(apusys_plls), clk_data);
    mtk_free_clk_data(clk_data);
    }
    static const struct of_device_id of_match_clk_mt8195_apusys_pll[] = {
    { .compatible = "mediatek,mt8195-apusys_pll", },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_apusys_pll);
    static struct platform_driver clk_mt8195_apusys_pll_drv = {
    .probe = clk_mt8195_apusys_pll_probe,
    .remove = clk_mt8195_apusys_pll_remove,
    .driver = {
    .name = "clk-mt8195-apusys_pll",
    .of_match_table = of_match_clk_mt8195_apusys_pll,
    },
    };
    module_platform_driver(clk_mt8195_apusys_pll_drv);
    MODULE_DESCRIPTION("MediaTek MT8195 AI Processing Unit PLL clocks driver");
    MODULE_LICENSE("GPL");
