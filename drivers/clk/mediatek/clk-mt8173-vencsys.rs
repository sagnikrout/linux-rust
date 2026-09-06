//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8173-vencsys.c
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
// Copyright (c) 2022 Collabora Ltd.
// Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

    static const struct mtk_gate_regs venc_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };

    GATE_MTK(_id, _name, _parent, &venc_cg_regs, _shift, &mtk_clk_gate_ops_setclr_inv)
    static const struct mtk_gate venc_clks[] = {
    GATE_DUMMY(CLK_DUMMY, "venc_dummy"),
    GATE_VENC(CLK_VENC_CKE0, "venc_cke0", "mm_sel", 0),
    GATE_VENC(CLK_VENC_CKE1, "venc_cke1", "venc_sel", 4),
    GATE_VENC(CLK_VENC_CKE2, "venc_cke2", "venc_sel", 8),
    GATE_VENC(CLK_VENC_CKE3, "venc_cke3", "venc_sel", 12),
    };
    static const struct mtk_gate venclt_clks[] = {
    GATE_DUMMY(CLK_DUMMY, "venclt_dummy"),
    GATE_VENC(CLK_VENCLT_CKE0, "venclt_cke0", "mm_sel", 0),
    GATE_VENC(CLK_VENCLT_CKE1, "venclt_cke1", "venclt_sel", 4),
    };
    static const struct mtk_clk_desc venc_desc = {
    .clks = venc_clks,
    .num_clks = ARRAY_SIZE(venc_clks),
    };
    static const struct mtk_clk_desc venc_lt_desc = {
    .clks = venclt_clks,
    .num_clks = ARRAY_SIZE(venclt_clks),
    };
    static const struct of_device_id of_match_clk_mt8173_vencsys[] = {
    { .compatible = "mediatek,mt8173-vencsys", .data = &venc_desc },
    { .compatible = "mediatek,mt8173-vencltsys", .data = &venc_lt_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8173_vencsys);
    static struct platform_driver clk_mt8173_vencsys_drv = {
    .driver = {
    .name = "clk-mt8173-vencsys",
    .of_match_table = of_match_clk_mt8173_vencsys,
    },
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    };
    module_platform_driver(clk_mt8173_vencsys_drv);
    MODULE_DESCRIPTION("MediaTek MT8173 vencsys clocks driver");
    MODULE_LICENSE("GPL");
