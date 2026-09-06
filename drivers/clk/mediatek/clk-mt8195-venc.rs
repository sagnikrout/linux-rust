//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8195-venc.c
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

    static const struct mtk_gate_regs venc_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };

    GATE_MTK(_id, _name, _parent, &venc_cg_regs, _shift, &mtk_clk_gate_ops_setclr_inv)
    static const struct mtk_gate venc_clks[] = {
    GATE_VENC(CLK_VENC_LARB, "venc_larb", "top_venc", 0),
    GATE_VENC(CLK_VENC_VENC, "venc_venc", "top_venc", 4),
    GATE_VENC(CLK_VENC_JPGENC, "venc_jpgenc", "top_venc", 8),
    GATE_VENC(CLK_VENC_JPGDEC, "venc_jpgdec", "top_venc", 12),
    GATE_VENC(CLK_VENC_JPGDEC_C1, "venc_jpgdec_c1", "top_venc", 16),
    GATE_VENC(CLK_VENC_GALS, "venc_gals", "top_venc", 28),
    };
    static const struct mtk_gate venc_core1_clks[] = {
    GATE_VENC(CLK_VENC_CORE1_LARB, "venc_core1_larb", "top_venc", 0),
    GATE_VENC(CLK_VENC_CORE1_VENC, "venc_core1_venc", "top_venc", 4),
    GATE_VENC(CLK_VENC_CORE1_JPGENC, "venc_core1_jpgenc", "top_venc", 8),
    GATE_VENC(CLK_VENC_CORE1_JPGDEC, "venc_core1_jpgdec", "top_venc", 12),
    GATE_VENC(CLK_VENC_CORE1_JPGDEC_C1, "venc_core1_jpgdec_c1", "top_venc", 16),
    GATE_VENC(CLK_VENC_CORE1_GALS, "venc_core1_gals", "top_venc", 28),
    };
    static const struct mtk_clk_desc venc_desc = {
    .clks = venc_clks,
    .num_clks = ARRAY_SIZE(venc_clks),
    };
    static const struct mtk_clk_desc venc_core1_desc = {
    .clks = venc_core1_clks,
    .num_clks = ARRAY_SIZE(venc_core1_clks),
    };
    static const struct of_device_id of_match_clk_mt8195_venc[] = {
    {
    .compatible = "mediatek,mt8195-vencsys",
    .data = &venc_desc,
    }, {
    .compatible = "mediatek,mt8195-vencsys_core1",
    .data = &venc_core1_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_venc);
    static struct platform_driver clk_mt8195_venc_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8195-venc",
    .of_match_table = of_match_clk_mt8195_venc,
    },
    };
    module_platform_driver(clk_mt8195_venc_drv);
    MODULE_DESCRIPTION("MediaTek MT8195 Video Encoders clocks driver");
    MODULE_LICENSE("GPL");
