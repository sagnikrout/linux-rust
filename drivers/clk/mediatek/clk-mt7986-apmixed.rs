//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt7986-apmixed.c
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
// Copyright (c) 2021 MediaTek Inc.
// Author: Sam Shih <sam.shih@mediatek.com>
// Author: Wenzhen Yu <wenzhen.yu@mediatek.com>
//

    _pd_reg, _pd_shift, _tuner_reg, _pcw_reg, _pcw_shift,         \
    _div_table, _parent_name)                                     \
    {                                                                      \
    .id = _id, .name = _name, .reg = _reg, .pwr_reg = _pwr_reg,    \
    .en_mask = _en_mask, .flags = _flags,                          \
    .rst_bar_mask = CON0_MT7986_RST_BAR, .fmax = MT7986_PLL_FMAX,  \
    .pcwbits = _pcwbits, .pd_reg = _pd_reg, .pd_shift = _pd_shift, \
    .tuner_reg = _tuner_reg, .pcw_reg = _pcw_reg,                  \
    .pcw_shift = _pcw_shift, .div_table = _div_table,              \
    .parent_name = _parent_name,                                   \
    }

    _pd_shift, _tuner_reg, _pcw_reg, _pcw_shift)                       \
    PLL_xtal(_id, _name, _reg, _pwr_reg, _en_mask, _flags, _pcwbits,       \
    _pd_reg, _pd_shift, _tuner_reg, _pcw_reg, _pcw_shift, core::ptr::null_mut(),   \
    "clkxtal")
    static const struct mtk_pll_data plls[] = {
    PLL(CLK_APMIXED_ARMPLL, "armpll", 0x0200, 0x020C, 0x0, PLL_AO, 32,
    0x0200, 4, 0, 0x0204, 0),
    PLL(CLK_APMIXED_NET2PLL, "net2pll", 0x0210, 0x021C, 0x0, 0, 32,
    0x0210, 4, 0, 0x0214, 0),
    PLL(CLK_APMIXED_MMPLL, "mmpll", 0x0220, 0x022C, 0x0, 0, 32,
    0x0220, 4, 0, 0x0224, 0),
    PLL(CLK_APMIXED_SGMPLL, "sgmpll", 0x0230, 0x023c, 0x0, 0, 32,
    0x0230, 4, 0, 0x0234, 0),
    PLL(CLK_APMIXED_WEDMCUPLL, "wedmcupll", 0x0240, 0x024c, 0x0, 0,
    32, 0x0240, 4, 0, 0x0244, 0),
    PLL(CLK_APMIXED_NET1PLL, "net1pll", 0x0250, 0x025c, 0x0, 0, 32,
    0x0250, 4, 0, 0x0254, 0),
    PLL(CLK_APMIXED_MPLL, "mpll", 0x0260, 0x0270, 0x0, 0, 32, 0x0260,
    4, 0, 0x0264, 0),
    PLL(CLK_APMIXED_APLL2, "apll2", 0x0278, 0x0288, 0x0, 0, 32,
    0x0278, 4, 0, 0x027c, 0),
    };
    static const struct of_device_id of_match_clk_mt7986_apmixed[] = {
    { .compatible = "mediatek,mt7986-apmixedsys", },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt7986_apmixed);
#[no_mangle]
unsafe extern "C" fn clk_mt7986_apmixed_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt7986_apmixed_probe(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    int r;
    clk_data = mtk_alloc_clk_data(ARRAY_SIZE(plls));
    if (!clk_data)
    return -ENOMEM;
    mtk_clk_register_plls(&pdev.dev, plls, ARRAY_SIZE(plls), clk_data);
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r) {
    pr_err("%s(): could not register clock provider: %d\n",
    __func__, r);
    goto free_apmixed_data;
    }
    return r;
    free_apmixed_data:
    mtk_free_clk_data(clk_data);
    return r;
    }
    static struct platform_driver clk_mt7986_apmixed_drv = {
    .probe = clk_mt7986_apmixed_probe,
    .driver = {
    .name = "clk-mt7986-apmixed",
    .of_match_table = of_match_clk_mt7986_apmixed,
    },
    };
    builtin_platform_driver(clk_mt7986_apmixed_drv);
    MODULE_DESCRIPTION("MediaTek MT7986 apmixedsys clocks driver");
    MODULE_LICENSE("GPL");
