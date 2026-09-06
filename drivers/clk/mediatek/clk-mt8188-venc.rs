//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8188-venc.c
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Garmin Chang <garmin.chang@mediatek.com>
//

    static const struct mtk_gate_regs venc1_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };

    GATE_MTK(_id, _name, _parent, &venc1_cg_regs, _shift, &mtk_clk_gate_ops_setclr_inv)
    static const struct mtk_gate venc1_clks[] = {
    GATE_VENC1(CLK_VENC1_LARB, "venc1_larb", "top_venc", 0),
    GATE_VENC1(CLK_VENC1_VENC, "venc1_venc", "top_venc", 4),
    GATE_VENC1(CLK_VENC1_JPGENC, "venc1_jpgenc", "top_venc", 8),
    GATE_VENC1(CLK_VENC1_JPGDEC, "venc1_jpgdec", "top_venc", 12),
    GATE_VENC1(CLK_VENC1_JPGDEC_C1, "venc1_jpgdec_c1", "top_venc", 16),
    GATE_VENC1(CLK_VENC1_GALS, "venc1_gals", "top_venc", 28),
    GATE_VENC1(CLK_VENC1_GALS_SRAM, "venc1_gals_sram", "top_venc", 31),
    };
    static const struct mtk_clk_desc venc1_desc = {
    .clks = venc1_clks,
    .num_clks = ARRAY_SIZE(venc1_clks),
    };
    static const struct of_device_id of_match_clk_mt8188_venc1[] = {
    { .compatible = "mediatek,mt8188-vencsys", .data = &venc1_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_venc1);
    static struct platform_driver clk_mt8188_venc1_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8188-venc1",
    .of_match_table = of_match_clk_mt8188_venc1,
    },
    };
    module_platform_driver(clk_mt8188_venc1_drv);
    MODULE_DESCRIPTION("MediaTek MT8188 Video Encoders clocks driver");
    MODULE_LICENSE("GPL");
