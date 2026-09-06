//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6735-imgsys.c
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

pub const IMG_CG_CON: c_uint = 0x00;
pub const IMG_CG_SET: c_uint = 0x04;
pub const IMG_CG_CLR: c_uint = 0x08;
    static struct mtk_gate_regs imgsys_cg_regs = {
    .set_ofs = IMG_CG_SET,
    .clr_ofs = IMG_CG_CLR,
    .sta_ofs = IMG_CG_CON,
    };
    static const struct mtk_gate imgsys_gates[] = {
    GATE_MTK(CLK_IMG_SMI_LARB2, "smi_larb2", "mm_sel", &imgsys_cg_regs, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_CAM_SMI, "cam_smi", "mm_sel", &imgsys_cg_regs, 5, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_CAM_CAM, "cam_cam", "mm_sel", &imgsys_cg_regs, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_SEN_TG, "sen_tg", "mm_sel", &imgsys_cg_regs, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_SEN_CAM, "sen_cam", "mm_sel", &imgsys_cg_regs, 8, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_CAM_SV, "cam_sv", "mm_sel", &imgsys_cg_regs, 9, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_SUFOD, "sufod", "mm_sel", &imgsys_cg_regs, 10, &mtk_clk_gate_ops_setclr),
    GATE_MTK(CLK_IMG_FD, "fd", "mm_sel", &imgsys_cg_regs, 11, &mtk_clk_gate_ops_setclr),
    };
    static const struct mtk_clk_desc imgsys_clks = {
    .clks = imgsys_gates,
    .num_clks = ARRAY_SIZE(imgsys_gates),
    };
    static const struct of_device_id of_match_mt6735_imgsys[] = {
    { .compatible = "mediatek,mt6735-imgsys", .data = &imgsys_clks },
    { /* sentinel */ }
    };
    static struct platform_driver clk_mt6735_imgsys = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6735-imgsys",
    .of_match_table = of_match_mt6735_imgsys,
    },
    };
    module_platform_driver(clk_mt6735_imgsys);
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_DESCRIPTION("MediaTek MT6735 imgsys clock driver");
    MODULE_LICENSE("GPL");
