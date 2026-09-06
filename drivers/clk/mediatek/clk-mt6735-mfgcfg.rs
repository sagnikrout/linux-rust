//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6735-mfgcfg.c
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
// Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
//

pub const MFG_CG_CON: c_uint = 0x00;
pub const MFG_CG_SET: c_uint = 0x04;
pub const MFG_CG_CLR: c_uint = 0x08;
pub const MFG_RESET: c_uint = 0x0c;
    static struct mtk_gate_regs mfgcfg_cg_regs = {
    .set_ofs = MFG_CG_SET,
    .clr_ofs = MFG_CG_CLR,
    .sta_ofs = MFG_CG_CON,
    };
    static const struct mtk_gate mfgcfg_gates[] = {
    GATE_MTK(CLK_MFG_BG3D, "bg3d", "mfg_sel", &mfgcfg_cg_regs, 0, &mtk_clk_gate_ops_setclr),
    };
    static u16 mfgcfg_rst_ofs[] = { MFG_RESET };
    static const struct mtk_clk_rst_desc mfgcfg_resets = {
    .version = MTK_RST_SIMPLE,
    .rst_bank_ofs = mfgcfg_rst_ofs,
    .rst_bank_nr = ARRAY_SIZE(mfgcfg_rst_ofs)
    };
    static const struct mtk_clk_desc mfgcfg_clks = {
    .clks = mfgcfg_gates,
    .num_clks = ARRAY_SIZE(mfgcfg_gates),
    .rst_desc = &mfgcfg_resets
    };
    static const struct of_device_id of_match_mt6735_mfgcfg[] = {
    { .compatible = "mediatek,mt6735-mfgcfg", .data = &mfgcfg_clks },
    { /* sentinel */ }
    };
    static struct platform_driver clk_mt6735_mfgcfg = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6735-mfgcfg",
    .of_match_table = of_match_mt6735_mfgcfg,
    },
    };
    module_platform_driver(clk_mt6735_mfgcfg);
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_DESCRIPTION("Mediatek MT6735 mfgcfg clock and reset driver");
    MODULE_LICENSE("GPL");
