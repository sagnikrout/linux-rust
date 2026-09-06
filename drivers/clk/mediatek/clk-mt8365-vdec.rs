//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8365-vdec.c
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
// Copyright (C) 2022 MediaTek Inc.
//

    static const struct mtk_gate_regs vdec0_cg_regs = {
    .set_ofs = 0x0,
    .clr_ofs = 0x4,
    .sta_ofs = 0x0,
    };
    static const struct mtk_gate_regs vdec1_cg_regs = {
    .set_ofs = 0x8,
    .clr_ofs = 0xc,
    .sta_ofs = 0x8,
    };

    GATE_MTK(_id, _name, _parent, &vdec0_cg_regs, _shift, \
    &mtk_clk_gate_ops_setclr_inv)

    GATE_MTK(_id, _name, _parent, &vdec1_cg_regs, _shift, \
    &mtk_clk_gate_ops_setclr_inv)
    static const struct mtk_gate vdec_clks[] = {
// VDEC0
    GATE_VDEC0(CLK_VDEC_VDEC, "vdec_fvdec_ck", "mm_sel", 0),
// VDEC1
    GATE_VDEC1(CLK_VDEC_LARB1, "vdec_flarb1_ck", "mm_sel", 0),
    };
    static const struct mtk_clk_desc vdec_desc = {
    .clks = vdec_clks,
    .num_clks = ARRAY_SIZE(vdec_clks),
    };
    static const struct of_device_id of_match_clk_mt8365_vdec[] = {
    {
    .compatible = "mediatek,mt8365-vdecsys",
    .data = &vdec_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8365_vdec);
    static struct platform_driver clk_mt8365_vdec_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8365-vdec",
    .of_match_table = of_match_clk_mt8365_vdec,
    },
    };
    module_platform_driver(clk_mt8365_vdec_drv);
    MODULE_DESCRIPTION("MediaTek MT8365 Video Decoders clocks driver");
    MODULE_LICENSE("GPL");
