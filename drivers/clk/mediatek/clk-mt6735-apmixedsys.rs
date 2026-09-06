//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6735-apmixedsys.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
//

pub const AP_PLL_CON_5: c_uint = 0x014;
pub const ARMPLL_CON0: c_uint = 0x200;
pub const ARMPLL_CON1: c_uint = 0x204;
pub const ARMPLL_PWR_CON0: c_uint = 0x20c;
pub const MAINPLL_CON0: c_uint = 0x210;
pub const MAINPLL_CON1: c_uint = 0x214;
pub const MAINPLL_PWR_CON0: c_uint = 0x21c;
pub const UNIVPLL_CON0: c_uint = 0x220;
pub const UNIVPLL_CON1: c_uint = 0x224;
pub const UNIVPLL_PWR_CON0: c_uint = 0x22c;
pub const MMPLL_CON0: c_uint = 0x230;
pub const MMPLL_CON1: c_uint = 0x234;
pub const MMPLL_PWR_CON0: c_uint = 0x23c;
pub const MSDCPLL_CON0: c_uint = 0x240;
pub const MSDCPLL_CON1: c_uint = 0x244;
pub const MSDCPLL_PWR_CON0: c_uint = 0x24c;
pub const VENCPLL_CON0: c_uint = 0x250;
pub const VENCPLL_CON1: c_uint = 0x254;
pub const VENCPLL_PWR_CON0: c_uint = 0x25c;
pub const TVDPLL_CON0: c_uint = 0x260;
pub const TVDPLL_CON1: c_uint = 0x264;
pub const TVDPLL_PWR_CON0: c_uint = 0x26c;
pub const APLL1_CON0: c_uint = 0x270;
pub const APLL1_CON1: c_uint = 0x274;
pub const APLL1_CON2: c_uint = 0x278;
pub const APLL1_PWR_CON0: c_uint = 0x280;
pub const APLL2_CON0: c_uint = 0x284;
pub const APLL2_CON1: c_uint = 0x288;
pub const APLL2_CON2: c_uint = 0x28c;
pub const APLL2_PWR_CON0: c_uint = 0x294;

    _pd_reg, _pd_shift, _tuner_reg, _tuner_en_reg,		\
    _tuner_en_bit, _pcw_reg, _pcwbits, _flags) {		\
    .id = _id,						\
    .name = _name,						\
    .parent_name = "clk26m",				\
    .reg = _reg,						\
    .pwr_reg = _pwr_reg,					\
    .en_mask = _en_mask,					\
    .rst_bar_mask = _rst_bar_mask,				\
    .pd_reg = _pd_reg,					\
    .pd_shift = _pd_shift,					\
    .tuner_reg = _tuner_reg,				\
    .tuner_en_reg = _tuner_en_reg,				\
    .tuner_en_bit = _tuner_en_bit,				\
    .pcw_reg = _pcw_reg,					\
    .pcw_chg_reg = _pcw_reg,				\
    .pcwbits = _pcwbits,					\
    .flags = _flags,					\
    }
    static const struct mtk_pll_data apmixedsys_plls[] = {
    PLL(CLK_APMIXED_ARMPLL, "armpll", ARMPLL_CON0, ARMPLL_PWR_CON0, 0x00000001, 0, ARMPLL_CON1, 24, 0, 0, 0, ARMPLL_CON1, 21, PLL_AO),
    PLL(CLK_APMIXED_MAINPLL, "mainpll", MAINPLL_CON0, MAINPLL_PWR_CON0, 0xf0000101, CON0_RST_BAR, MAINPLL_CON1, 24, 0, 0, 0, MAINPLL_CON1, 21, HAVE_RST_BAR),
    PLL(CLK_APMIXED_UNIVPLL, "univpll", UNIVPLL_CON0, UNIVPLL_PWR_CON0, 0xfc000001, CON0_RST_BAR, UNIVPLL_CON1, 24, 0, 0, 0, UNIVPLL_CON1, 21, HAVE_RST_BAR),
    PLL(CLK_APMIXED_MMPLL, "mmpll", MMPLL_CON0, MMPLL_PWR_CON0, 0x00000001, 0, MMPLL_CON1, 24, 0, 0, 0, MMPLL_CON1, 21, 0),
    PLL(CLK_APMIXED_MSDCPLL, "msdcpll", MSDCPLL_CON0, MSDCPLL_PWR_CON0, 0x00000001, 0, MSDCPLL_CON1, 24, 0, 0, 0, MSDCPLL_CON1, 21, 0),
    PLL(CLK_APMIXED_VENCPLL, "vencpll", VENCPLL_CON0, VENCPLL_PWR_CON0, 0x00000001, CON0_RST_BAR, VENCPLL_CON1, 24, 0, 0, 0, VENCPLL_CON1, 21, HAVE_RST_BAR),
    PLL(CLK_APMIXED_TVDPLL, "tvdpll", TVDPLL_CON0, TVDPLL_PWR_CON0, 0x00000001, 0, TVDPLL_CON1, 24, 0, 0, 0, TVDPLL_CON1, 21, 0),
    PLL(CLK_APMIXED_APLL1, "apll1", APLL1_CON0, APLL1_PWR_CON0, 0x00000001, 0, APLL1_CON0, 4, APLL1_CON2, AP_PLL_CON_5, 0, APLL1_CON1, 31, 0),
    PLL(CLK_APMIXED_APLL2, "apll2", APLL2_CON0, APLL2_PWR_CON0, 0x00000001, 0, APLL2_CON0, 4, APLL2_CON2, AP_PLL_CON_5, 1, APLL2_CON1, 31, 0)
    };
#[no_mangle]
unsafe extern "C" fn clk_mt6735_apmixed_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt6735_apmixed_probe(struct platform_device *pdev)
    {
    void __iomem *base;
    struct resource *res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    struct clk_hw_onecell_data *clk_data;
    int ret;
    base = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    clk_data = mtk_devm_alloc_clk_data(&pdev.dev, ARRAY_SIZE(apmixedsys_plls));
    if (!clk_data)
    return -ENOMEM;
    platform_set_drvdata(pdev, clk_data);
    ret = mtk_clk_register_plls(&pdev.dev, apmixedsys_plls,
    ARRAY_SIZE(apmixedsys_plls), clk_data);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register PLLs: %d\n", ret);
    return ret;
    }
    ret = devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_onecell_get,
    clk_data);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to register clock provider: %d\n", ret);
    mtk_clk_unregister_plls(apmixedsys_plls, ARRAY_SIZE(apmixedsys_plls),
    clk_data);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt6735_apmixed_remove(pdev: *mut platform_device) {
    static void clk_mt6735_apmixed_remove(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data = platform_get_drvdata(pdev);
    mtk_clk_unregister_plls(apmixedsys_plls, ARRAY_SIZE(apmixedsys_plls), clk_data);
    }
    static const struct of_device_id of_match_mt6735_apmixedsys[] = {
    { .compatible = "mediatek,mt6735-apmixedsys" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_mt6735_apmixedsys);
    static struct platform_driver clk_mt6735_apmixedsys = {
    .probe = clk_mt6735_apmixed_probe,
    .remove = clk_mt6735_apmixed_remove,
    .driver = {
    .name = "clk-mt6735-apmixedsys",
    .of_match_table = of_match_mt6735_apmixedsys,
    },
    };
    module_platform_driver(clk_mt6735_apmixedsys);
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_DESCRIPTION("MediaTek MT6735 apmixedsys clock driver");
    MODULE_LICENSE("GPL");
