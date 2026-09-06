//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8188-mfg.c
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

    static const struct mtk_gate_regs mfgcfg_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };

    GATE_MTK_FLAGS(_id, _name, _parent, &mfgcfg_cg_regs, _shift,	\
    &mtk_clk_gate_ops_setclr, CLK_SET_RATE_PARENT)
    static const struct mtk_gate mfgcfg_clks[] = {
    GATE_MFG(CLK_MFGCFG_BG3D, "mfgcfg_bg3d", "mfg_ck_fast_ref", 0),
    };
    static const struct mtk_clk_desc mfgcfg_desc = {
    .clks = mfgcfg_clks,
    .num_clks = ARRAY_SIZE(mfgcfg_clks),
    };
    static const struct of_device_id of_match_clk_mt8188_mfgcfg[] = {
    { .compatible = "mediatek,mt8188-mfgcfg", .data = &mfgcfg_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_mfgcfg);
    static struct platform_driver clk_mt8188_mfgcfg_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8188-mfgcfg",
    .of_match_table = of_match_clk_mt8188_mfgcfg,
    },
    };
    module_platform_driver(clk_mt8188_mfgcfg_drv);
    MODULE_DESCRIPTION("MediaTek MT8186 GPU mfg clocks driver");
    MODULE_LICENSE("GPL");
