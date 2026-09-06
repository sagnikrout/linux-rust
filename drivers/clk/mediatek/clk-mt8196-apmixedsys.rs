//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8196-apmixedsys.c
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

// APMIXEDSYS PLL control register offsets
pub const MAINPLL_CON0: c_uint = 0x250;
pub const MAINPLL_CON1: c_uint = 0x254;
pub const UNIVPLL_CON0: c_uint = 0x264;
pub const UNIVPLL_CON1: c_uint = 0x268;
pub const MSDCPLL_CON0: c_uint = 0x278;
pub const MSDCPLL_CON1: c_uint = 0x27c;
pub const ADSPPLL_CON0: c_uint = 0x28c;
pub const ADSPPLL_CON1: c_uint = 0x290;
pub const EMIPLL_CON0: c_uint = 0x2a0;
pub const EMIPLL_CON1: c_uint = 0x2a4;
pub const EMIPLL2_CON0: c_uint = 0x2b4;
pub const EMIPLL2_CON1: c_uint = 0x2b8;
pub const NET1PLL_CON0: c_uint = 0x2c8;
pub const NET1PLL_CON1: c_uint = 0x2cc;
pub const SGMIIPLL_CON0: c_uint = 0x2dc;
pub const SGMIIPLL_CON1: c_uint = 0x2e0;
// APMIXEDSYS_GP2 PLL control register offsets
pub const MAINPLL2_CON0: c_uint = 0x250;
pub const MAINPLL2_CON1: c_uint = 0x254;
pub const UNIVPLL2_CON0: c_uint = 0x264;
pub const UNIVPLL2_CON1: c_uint = 0x268;
pub const MMPLL2_CON0: c_uint = 0x278;
pub const MMPLL2_CON1: c_uint = 0x27c;
pub const IMGPLL_CON0: c_uint = 0x28c;
pub const IMGPLL_CON1: c_uint = 0x290;
pub const TVDPLL1_CON0: c_uint = 0x2a0;
pub const TVDPLL1_CON1: c_uint = 0x2a4;
pub const TVDPLL2_CON0: c_uint = 0x2b4;
pub const TVDPLL2_CON1: c_uint = 0x2b8;
pub const TVDPLL3_CON0: c_uint = 0x2c8;
pub const TVDPLL3_CON1: c_uint = 0x2cc;
pub const PLLEN_ALL: c_uint = 0x080;
pub const PLLEN_ALL_SET: c_uint = 0x084;
pub const PLLEN_ALL_CLR: c_uint = 0x088;
pub const FENC_STATUS_CON0: c_uint = 0x03c;

pub const MT8196_INTEGER_BITS: c_int = 8;

    _flags, _pd_reg, _pd_shift,		\
    _pcw_reg, _pcw_shift, _pcwbits,		\
    _pll_en_bit) {				\
    .id = _id,					\
    .name = _name,					\
    .reg = _reg,					\
    .fenc_sta_ofs = _fenc_sta_ofs,			\
    .fenc_sta_bit = _fenc_sta_bit,			\
    .flags = _flags,				\
    .fmax = MT8196_PLL_FMAX,			\
    .fmin = MT8196_PLL_FMIN,			\
    .pd_reg = _pd_reg,				\
    .pd_shift = _pd_shift,				\
    .pcw_reg = _pcw_reg,				\
    .pcw_shift = _pcw_shift,			\
    .pcwbits = _pcwbits,				\
    .pcwibits = MT8196_INTEGER_BITS,		\
    .en_reg = PLLEN_ALL,				\
    .en_set_reg = PLLEN_ALL_SET,			\
    .en_clr_reg = PLLEN_ALL_CLR,			\
    .pll_en_bit = _pll_en_bit,			\
    .ops = &mtk_pll_fenc_clr_set_ops,		\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pll_desc {
    pub clks: *const mtk_pll_data,
    pub num_clks: usize,
}

    static const struct mtk_pll_data apmixed_plls[] = {
    PLL_FENC(CLK_APMIXED_MAINPLL, "mainpll", MAINPLL_CON0, FENC_STATUS_CON0,
    7, PLL_AO, MAINPLL_CON1, 24, MAINPLL_CON1, 0, 22, 0),
    PLL_FENC(CLK_APMIXED_UNIVPLL, "univpll", UNIVPLL_CON0, FENC_STATUS_CON0,
    6, 0, UNIVPLL_CON1, 24, UNIVPLL_CON1, 0, 22, 1),
    PLL_FENC(CLK_APMIXED_MSDCPLL, "msdcpll", MSDCPLL_CON0, FENC_STATUS_CON0,
    5, 0, MSDCPLL_CON1, 24, MSDCPLL_CON1, 0, 22, 2),
    PLL_FENC(CLK_APMIXED_ADSPPLL, "adsppll", ADSPPLL_CON0, FENC_STATUS_CON0,
    4, 0, ADSPPLL_CON1, 24, ADSPPLL_CON1, 0, 22, 3),
    PLL_FENC(CLK_APMIXED_EMIPLL, "emipll", EMIPLL_CON0, FENC_STATUS_CON0, 3,
    PLL_AO, EMIPLL_CON1, 24, EMIPLL_CON1, 0, 22, 4),
    PLL_FENC(CLK_APMIXED_EMIPLL2, "emipll2", EMIPLL2_CON0, FENC_STATUS_CON0,
    2, PLL_AO, EMIPLL2_CON1, 24, EMIPLL2_CON1, 0, 22, 5),
    PLL_FENC(CLK_APMIXED_NET1PLL, "net1pll", NET1PLL_CON0, FENC_STATUS_CON0,
    1, 0, NET1PLL_CON1, 24, NET1PLL_CON1, 0, 22, 6),
    PLL_FENC(CLK_APMIXED_SGMIIPLL, "sgmiipll", SGMIIPLL_CON0, FENC_STATUS_CON0,
    0, 0, SGMIIPLL_CON1, 24, SGMIIPLL_CON1, 0, 22, 7),
    };
    static const struct mtk_pll_desc apmixed_desc = {
    .clks = apmixed_plls,
    .num_clks = ARRAY_SIZE(apmixed_plls),
    };
    static const struct mtk_pll_data apmixed2_plls[] = {
    PLL_FENC(CLK_APMIXED2_MAINPLL2, "mainpll2", MAINPLL2_CON0, FENC_STATUS_CON0,
    6, 0, MAINPLL2_CON1, 24, MAINPLL2_CON1, 0, 22, 0),
    PLL_FENC(CLK_APMIXED2_UNIVPLL2, "univpll2", UNIVPLL2_CON0, FENC_STATUS_CON0,
    5, 0, UNIVPLL2_CON1, 24, UNIVPLL2_CON1, 0, 22, 1),
    PLL_FENC(CLK_APMIXED2_MMPLL2, "mmpll2", MMPLL2_CON0, FENC_STATUS_CON0,
    4, 0, MMPLL2_CON1, 24, MMPLL2_CON1, 0, 22, 2),
    PLL_FENC(CLK_APMIXED2_IMGPLL, "imgpll", IMGPLL_CON0, FENC_STATUS_CON0,
    3, 0, IMGPLL_CON1, 24, IMGPLL_CON1, 0, 22, 3),
    PLL_FENC(CLK_APMIXED2_TVDPLL1, "tvdpll1", TVDPLL1_CON0, FENC_STATUS_CON0,
    2, 0, TVDPLL1_CON1, 24, TVDPLL1_CON1, 0, 22, 4),
    PLL_FENC(CLK_APMIXED2_TVDPLL2, "tvdpll2", TVDPLL2_CON0, FENC_STATUS_CON0,
    1, 0, TVDPLL2_CON1, 24, TVDPLL2_CON1, 0, 22, 5),
    PLL_FENC(CLK_APMIXED2_TVDPLL3, "tvdpll3", TVDPLL3_CON0, FENC_STATUS_CON0,
    0, 0, TVDPLL3_CON1, 24, TVDPLL3_CON1, 0, 22, 6),
    };
    static const struct mtk_pll_desc apmixed2_desc = {
    .clks = apmixed2_plls,
    .num_clks = ARRAY_SIZE(apmixed2_plls),
    };
#[no_mangle]
unsafe extern "C" fn clk_mt8196_apmixed_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8196_apmixed_probe(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    const struct mtk_pll_desc *mcd;
    int r;
    mcd = device_get_match_data(&pdev.dev);
    if (!mcd)
    return -EINVAL;
    clk_data = mtk_alloc_clk_data(mcd.num_clks);
    if (!clk_data)
    return -ENOMEM;
    r = mtk_clk_register_plls(&pdev.dev, mcd.clks, mcd.num_clks,
    clk_data);
    if (r)
    goto free_apmixed_data;
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r)
    goto unregister_plls;
    platform_set_drvdata(pdev, clk_data);
    return r;
    unregister_plls:
    mtk_clk_unregister_plls(mcd.clks, mcd.num_clks, clk_data);
    free_apmixed_data:
    mtk_free_clk_data(clk_data);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8196_apmixed_remove(pdev: *mut platform_device) {
    static void clk_mt8196_apmixed_remove(struct platform_device *pdev)
    {
    const struct mtk_pll_desc *mcd = device_get_match_data(&pdev.dev);
    struct clk_hw_onecell_data *clk_data = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(mcd.clks, mcd.num_clks, clk_data);
    mtk_free_clk_data(clk_data);
    }
    static const struct of_device_id of_match_clk_mt8196_apmixed[] = {
    { .compatible = "mediatek,mt8196-apmixedsys", .data = &apmixed_desc },
    { .compatible = "mediatek,mt8196-apmixedsys-gp2",
    .data = &apmixed2_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_apmixed);
    static struct platform_driver clk_mt8196_apmixed_drv = {
    .probe = clk_mt8196_apmixed_probe,
    .remove = clk_mt8196_apmixed_remove,
    .driver = {
    .name = "clk-mt8196-apmixed",
    .of_match_table = of_match_clk_mt8196_apmixed,
    },
    };
    module_platform_driver(clk_mt8196_apmixed_drv);
    MODULE_DESCRIPTION("MediaTek MT8196 apmixedsys clocks driver");
    MODULE_LICENSE("GPL");
