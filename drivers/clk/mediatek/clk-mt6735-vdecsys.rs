//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6735-vdecsys.c
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

pub const VDEC_CKEN_SET: c_uint = 0x00;
pub const VDEC_CKEN_CLR: c_uint = 0x04;
pub const SMI_LARB1_CKEN_SET: c_uint = 0x08;
pub const SMI_LARB1_CKEN_CLR: c_uint = 0x0c;
pub const VDEC_RESETB_CON: c_uint = 0x10;
pub const SMI_LARB1_RESETB_CON: c_uint = 0x14;
pub const RST_NR_PER_BANK: c_int = 32;
    static struct mtk_gate_regs vdec_cg_regs = {
    .set_ofs = VDEC_CKEN_SET,
    .clr_ofs = VDEC_CKEN_CLR,
    .sta_ofs = VDEC_CKEN_SET,
    };
    static struct mtk_gate_regs smi_larb1_cg_regs = {
    .set_ofs = SMI_LARB1_CKEN_SET,
    .clr_ofs = SMI_LARB1_CKEN_CLR,
    .sta_ofs = SMI_LARB1_CKEN_SET,
    };
    static const struct mtk_gate vdecsys_gates[] = {
    GATE_MTK(CLK_VDEC_VDEC, "vdec", "vdec_sel", &vdec_cg_regs, 0, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK(CLK_VDEC_SMI_LARB1, "smi_larb1", "vdec_sel", &smi_larb1_cg_regs, 0, &mtk_clk_gate_ops_setclr_inv),
    };
    static u16 vdecsys_rst_bank_ofs[] = { VDEC_RESETB_CON, SMI_LARB1_RESETB_CON };
    static u16 vdecsys_rst_idx_map[] = {
    [MT6735_VDEC_RST0_VDEC]		= 0 * RST_NR_PER_BANK + 0,
    [MT6735_VDEC_RST1_SMI_LARB1]	= 1 * RST_NR_PER_BANK + 0,
    };
    static const struct mtk_clk_rst_desc vdecsys_resets = {
    .version = MTK_RST_SIMPLE,
    .rst_bank_ofs = vdecsys_rst_bank_ofs,
    .rst_bank_nr = ARRAY_SIZE(vdecsys_rst_bank_ofs),
    .rst_idx_map = vdecsys_rst_idx_map,
    .rst_idx_map_nr = ARRAY_SIZE(vdecsys_rst_idx_map)
    };
    static const struct mtk_clk_desc vdecsys_clks = {
    .clks = vdecsys_gates,
    .num_clks = ARRAY_SIZE(vdecsys_gates),
    .rst_desc = &vdecsys_resets
    };
    static const struct of_device_id of_match_mt6735_vdecsys[] = {
    { .compatible = "mediatek,mt6735-vdecsys", .data = &vdecsys_clks },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_mt6735_vdecsys);
    static struct platform_driver clk_mt6735_vdecsys = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6735-vdecsys",
    .of_match_table = of_match_mt6735_vdecsys,
    },
    };
    module_platform_driver(clk_mt6735_vdecsys);
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_DESCRIPTION("MediaTek MT6735 vdecsys clock and reset driver");
    MODULE_LICENSE("GPL");
