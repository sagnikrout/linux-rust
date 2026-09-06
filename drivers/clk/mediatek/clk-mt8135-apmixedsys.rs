//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8135-apmixedsys.c
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
// Copyright (c) 2014 MediaTek Inc.
// James Liao <jamesjj.liao@mediatek.com>
// Copyright (c) 2023 Collabora, Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

    .id = _id,						\
    .name = _name,						\
    .reg = _reg,						\
    .pwr_reg = _pwr_reg,					\
    .en_mask = _en_mask,					\
    .flags = _flags,					\
    .rst_bar_mask = CON0_MT8135_RST_BAR,			\
    .fmax = MT8135_PLL_FMAX,				\
    .pcwbits = _pcwbits,					\
    .pd_reg = _pd_reg,					\
    .pd_shift = _pd_shift,					\
    .tuner_reg = _tuner_reg,				\
    .pcw_reg = _pcw_reg,					\
    .pcw_shift = _pcw_shift,				\
    }
    static const struct mtk_pll_data plls[] = {
    PLL(CLK_APMIXED_ARMPLL1, "armpll1", 0x200, 0x218, 0x80000000, 0, 21, 0x204, 24, 0x0, 0x204, 0),
    PLL(CLK_APMIXED_ARMPLL2, "armpll2", 0x2cc, 0x2e4, 0x80000000, 0, 21, 0x2d0, 24, 0x0, 0x2d0, 0),
    PLL(CLK_APMIXED_MAINPLL, "mainpll", 0x21c, 0x234, 0xf0000000, HAVE_RST_BAR, 21, 0x21c, 6, 0x0, 0x220, 0),
    PLL(CLK_APMIXED_UNIVPLL, "univpll", 0x238, 0x250, 0xf3000000, HAVE_RST_BAR, 7, 0x238, 6, 0x0, 0x238, 9),
    PLL(CLK_APMIXED_MMPLL, "mmpll", 0x254, 0x26c, 0xf0000000, HAVE_RST_BAR, 21, 0x254, 6, 0x0, 0x258, 0),
    PLL(CLK_APMIXED_MSDCPLL, "msdcpll", 0x278, 0x290, 0x80000000, 0, 21, 0x278, 6, 0x0, 0x27c, 0),
    PLL(CLK_APMIXED_TVDPLL, "tvdpll", 0x294, 0x2ac, 0x80000000, 0, 31, 0x294, 6, 0x0, 0x298, 0),
    PLL(CLK_APMIXED_LVDSPLL, "lvdspll", 0x2b0, 0x2c8, 0x80000000, 0, 21, 0x2b0, 6, 0x0, 0x2b4, 0),
    PLL(CLK_APMIXED_AUDPLL, "audpll", 0x2e8, 0x300, 0x80000000, 0, 31, 0x2e8, 6, 0x2f8, 0x2ec, 0),
    PLL(CLK_APMIXED_VDECPLL, "vdecpll", 0x304, 0x31c, 0x80000000, 0, 21, 0x2b0, 6, 0x0, 0x308, 0),
    };
#[no_mangle]
unsafe extern "C" fn clk_mt8135_apmixed_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8135_apmixed_probe(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    int ret;
    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if (!clk_data)
    return -ENOMEM;
    ret = mtk_clk_register_plls(&pdev.dev, plls, ARRAY_SIZE(plls),
    clk_data);
    if (ret)
    goto free_clk_data;
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (ret)
    goto unregister_plls;
    return 0;
    unregister_plls:
    mtk_clk_unregister_plls(plls, ARRAY_SIZE(plls), clk_data);
    free_clk_data:
    mtk_free_clk_data(clk_data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8135_apmixed_remove(pdev: *mut platform_device) {
    static void clk_mt8135_apmixed_remove(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct clk_hw_onecell_data *clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(plls, ARRAY_SIZE(plls), clk_data);
    mtk_free_clk_data(clk_data);
    }
    static const struct of_device_id of_match_clk_mt8135_apmixed[] = {
    { .compatible = "mediatek,mt8135-apmixedsys" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8135_apmixed);
    static struct platform_driver clk_mt8135_apmixed_drv = {
    .probe = clk_mt8135_apmixed_probe,
    .remove = clk_mt8135_apmixed_remove,
    .driver = {
    .name = "clk-mt8135-apmixed",
    .of_match_table = of_match_clk_mt8135_apmixed,
    },
    };
    module_platform_driver(clk_mt8135_apmixed_drv)
    MODULE_DESCRIPTION("MediaTek MT8135 apmixedsys clocks driver");
    MODULE_LICENSE("GPL");
