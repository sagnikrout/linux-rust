//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt2701-g3d.c
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
// Copyright (c) 2018 MediaTek Inc.
// Author: Sean Wang <sean.wang@mediatek.com>
//

    GATE_MTK(_id, _name, _parent, &g3d_cg_regs, _shift, &mtk_clk_gate_ops_setclr)
    static const struct mtk_gate_regs g3d_cg_regs = {
    .sta_ofs = 0x0,
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    };
    static const struct mtk_gate g3d_clks[] = {
    GATE_DUMMY(CLK_DUMMY, "g3d_dummy"),
    GATE_G3D(CLK_G3DSYS_CORE, "g3d_core", "mfg_sel", 0),
    };
    static u16 rst_ofs[] = { 0xc, };
    static const struct mtk_clk_rst_desc clk_rst_desc = {
    .version = MTK_RST_SIMPLE,
    .rst_bank_ofs = rst_ofs,
    .rst_bank_nr = ARRAY_SIZE(rst_ofs),
    };
    static const struct mtk_clk_desc g3d_desc = {
    .clks = g3d_clks,
    .num_clks = ARRAY_SIZE(g3d_clks),
    .rst_desc = &clk_rst_desc,
    };
    static const struct of_device_id of_match_clk_mt2701_g3d[] = {
    { .compatible = "mediatek,mt2701-g3dsys", .data = &g3d_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt2701_g3d);
    static struct platform_driver clk_mt2701_g3d_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt2701-g3d",
    .of_match_table = of_match_clk_mt2701_g3d,
    },
    };
    module_platform_driver(clk_mt2701_g3d_drv);
    MODULE_DESCRIPTION("MediaTek MT2701 GPU g3d clocks driver");
    MODULE_LICENSE("GPL");
