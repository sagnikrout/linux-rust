//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt7988-xfipll.c
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
// Copyright (c) 2023 Daniel Golle <daniel@makrotopia.org>
//

// Register to control USXGMII XFI PLL analog
pub const XFI_PLL_ANA_GLB8: c_uint = 0x108;
pub const RG_XFI_PLL_ANA_SWWA: c_uint = 0x02283248;
    static const struct mtk_gate_regs xfipll_cg_regs = {
    .set_ofs = 0x8,
    .clr_ofs = 0x8,
    .sta_ofs = 0x8,
    };

    {							\
    .id = _id,					\
    .name = _name,					\
    .parent_name = _parent,				\
    .regs = &xfipll_cg_regs,			\
    .shift = _shift,				\
    .ops = &mtk_clk_gate_ops_no_setclr_inv,		\
    }
    static const struct mtk_fixed_factor xfipll_divs[] = {
    FACTOR(CLK_XFIPLL_PLL, "xfipll_pll", "top_xtal", 125, 32),
    };
    static const struct mtk_gate xfipll_clks[] = {
    GATE_XFIPLL(CLK_XFIPLL_PLL_EN, "xfipll_pll_en", "xfipll_pll", 31),
    };
    static const struct mtk_clk_desc xfipll_desc = {
    .clks = xfipll_clks,
    .num_clks = ARRAY_SIZE(xfipll_clks),
    .factor_clks = xfipll_divs,
    .num_factor_clks = ARRAY_SIZE(xfipll_divs),
    };
#[no_mangle]
unsafe extern "C" fn clk_mt7988_xfipll_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt7988_xfipll_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    void __iomem *base = of_iomap(node, 0);
    if (!base)
    return -ENOMEM;
// Apply software workaround for USXGMII PLL TCL issue
    writel(RG_XFI_PLL_ANA_SWWA, base + XFI_PLL_ANA_GLB8);
    iounmap(base);
    return mtk_clk_simple_probe(pdev);
    };
    static const struct of_device_id of_match_clk_mt7988_xfipll[] = {
    { .compatible = "mediatek,mt7988-xfi-pll", .data = &xfipll_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt7988_xfipll);
    static struct platform_driver clk_mt7988_xfipll_drv = {
    .driver = {
    .name = "clk-mt7988-xfipll",
    .of_match_table = of_match_clk_mt7988_xfipll,
    },
    .probe = clk_mt7988_xfipll_probe,
    .remove = mtk_clk_simple_remove,
    };
    module_platform_driver(clk_mt7988_xfipll_drv);
    MODULE_DESCRIPTION("MediaTek MT7988 XFI PLL clock driver");
    MODULE_LICENSE("GPL");
