//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt2701-hif.c
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
// Author: Shunli Wang <shunli.wang@mediatek.com>
//

    static const struct mtk_gate_regs hif_cg_regs = {
    .sta_ofs = 0x0030,
    };

    GATE_MTK(_id, _name, _parent, &hif_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)
    static const struct mtk_gate hif_clks[] = {
    GATE_DUMMY(CLK_DUMMY, "hif_dummy"),
    GATE_HIF(CLK_HIFSYS_USB0PHY, "usb0_phy_clk", "ethpll_500m_ck", 21),
    GATE_HIF(CLK_HIFSYS_USB1PHY, "usb1_phy_clk", "ethpll_500m_ck", 22),
    GATE_HIF(CLK_HIFSYS_PCIE0, "pcie0_clk", "ethpll_500m_ck", 24),
    GATE_HIF(CLK_HIFSYS_PCIE1, "pcie1_clk", "ethpll_500m_ck", 25),
    GATE_HIF(CLK_HIFSYS_PCIE2, "pcie2_clk", "ethpll_500m_ck", 26),
    };
    static u16 rst_ofs[] = { 0x34, };
    static const struct mtk_clk_rst_desc clk_rst_desc = {
    .version = MTK_RST_SIMPLE,
    .rst_bank_ofs = rst_ofs,
    .rst_bank_nr = ARRAY_SIZE(rst_ofs),
    };
    static const struct mtk_clk_desc hif_desc = {
    .clks = hif_clks,
    .num_clks = ARRAY_SIZE(hif_clks),
    .rst_desc = &clk_rst_desc,
    };
    static const struct of_device_id of_match_clk_mt2701_hif[] = {
    { .compatible = "mediatek,mt2701-hifsys", .data = &hif_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt2701_hif);
    static struct platform_driver clk_mt2701_hif_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt2701-hif",
    .of_match_table = of_match_clk_mt2701_hif,
    },
    };
    module_platform_driver(clk_mt2701_hif_drv);
    MODULE_DESCRIPTION("MediaTek MT2701 HIFSYS clocks driver");
    MODULE_LICENSE("GPL");
