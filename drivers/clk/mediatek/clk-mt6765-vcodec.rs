//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6765-vcodec.c
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
// Author: Owen Chen <owen.chen@mediatek.com>
//

    static const struct mtk_gate_regs venc_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };

    GATE_MTK(_id, _name, _parent, &venc_cg_regs, _shift, &mtk_clk_gate_ops_setclr_inv)
    static const struct mtk_gate venc_clks[] = {
    GATE_VENC(CLK_VENC_SET0_LARB, "venc_set0_larb", "mm_ck", 0),
    GATE_VENC(CLK_VENC_SET1_VENC, "venc_set1_venc", "mm_ck", 4),
    GATE_VENC(CLK_VENC_SET2_JPGENC, "jpgenc", "mm_ck", 8),
    GATE_VENC(CLK_VENC_SET3_VDEC, "venc_set3_vdec", "mm_ck", 12),
    };
    static const struct mtk_clk_desc venc_desc = {
    .clks = venc_clks,
    .num_clks = ARRAY_SIZE(venc_clks),
    };
    static const struct of_device_id of_match_clk_mt6765_vcodec[] = {
    {
    .compatible = "mediatek,mt6765-vcodecsys",
    .data = &venc_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt6765_vcodec);
    static struct platform_driver clk_mt6765_vcodec_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6765-vcodec",
    .of_match_table = of_match_clk_mt6765_vcodec,
    },
    };
    module_platform_driver(clk_mt6765_vcodec_drv);
    MODULE_DESCRIPTION("MediaTek MT6765 Video Codec clocks driver");
    MODULE_LICENSE("GPL");
