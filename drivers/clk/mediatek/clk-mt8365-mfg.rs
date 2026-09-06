//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8365-mfg.c
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

    static const struct mtk_gate_regs mfg0_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };
    static const struct mtk_gate_regs mfg1_cg_regs = {
    .set_ofs = 0x280,
    .clr_ofs = 0x280,
    .sta_ofs = 0x280,
    };

    GATE_MTK(_id, _name, _parent, &mfg0_cg_regs, _shift, \
    &mtk_clk_gate_ops_setclr)

    GATE_MTK(_id, _name, _parent, &mfg1_cg_regs, _shift, \
    &mtk_clk_gate_ops_no_setclr)
    static const struct mtk_gate mfg_clks[] = {
// MFG0
    GATE_MFG0(CLK_MFG_BG3D, "mfg_bg3d", "mfg_sel", 0),
// MFG1
    GATE_MFG1(CLK_MFG_MBIST_DIAG, "mfg_mbist_diag", "mbist_diag_sel", 24),
    };
    static const struct mtk_clk_desc mfg_desc = {
    .clks = mfg_clks,
    .num_clks = ARRAY_SIZE(mfg_clks),
    };
    static const struct of_device_id of_match_clk_mt8365_mfg[] = {
    {
    .compatible = "mediatek,mt8365-mfgcfg",
    .data = &mfg_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8365_mfg);
    static struct platform_driver clk_mt8365_mfg_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8365-mfg",
    .of_match_table = of_match_clk_mt8365_mfg,
    },
    };
    module_platform_driver(clk_mt8365_mfg_drv);
    MODULE_DESCRIPTION("MediaTek MT8365 GPU mfg clocks driver");
    MODULE_LICENSE("GPL");
