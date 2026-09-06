//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6735-vencsys.c
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

pub const VENC_CG_CON: c_uint = 0x00;
pub const VENC_CG_SET: c_uint = 0x04;
pub const VENC_CG_CLR: c_uint = 0x08;
    static struct mtk_gate_regs venc_cg_regs = {
    .set_ofs = VENC_CG_SET,
    .clr_ofs = VENC_CG_CLR,
    .sta_ofs = VENC_CG_CON,
    };
    static const struct mtk_gate vencsys_gates[] = {
    GATE_MTK(CLK_VENC_SMI_LARB3, "smi_larb3", "mm_sel", &venc_cg_regs, 0, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK(CLK_VENC_VENC, "venc", "mm_sel", &venc_cg_regs, 4, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK(CLK_VENC_JPGENC, "jpgenc", "mm_sel", &venc_cg_regs, 8, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK(CLK_VENC_JPGDEC, "jpgdec", "mm_sel", &venc_cg_regs, 12, &mtk_clk_gate_ops_setclr_inv),
    };
    static const struct mtk_clk_desc vencsys_clks = {
    .clks = vencsys_gates,
    .num_clks = ARRAY_SIZE(vencsys_gates),
    };
    static const struct of_device_id of_match_mt6735_vencsys[] = {
    { .compatible = "mediatek,mt6735-vencsys", .data = &vencsys_clks },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_mt6735_vencsys);
    static struct platform_driver clk_mt6735_vencsys = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6735-vencsys",
    .of_match_table = of_match_mt6735_vencsys,
    },
    };
    module_platform_driver(clk_mt6735_vencsys);
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_DESCRIPTION("Mediatek MT6735 vencsys clock driver");
    MODULE_LICENSE("GPL");
