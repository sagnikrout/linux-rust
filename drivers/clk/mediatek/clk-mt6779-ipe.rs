//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6779-ipe.c
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
// Copyright (c) 2019 MediaTek Inc.
// Author: Wendell Lin <wendell.lin@mediatek.com>
//

    static const struct mtk_gate_regs ipe_cg_regs = {
    .set_ofs = 0x0004,
    .clr_ofs = 0x0008,
    .sta_ofs = 0x0000,
    };

    GATE_MTK(_id, _name, _parent, &ipe_cg_regs, _shift,	\
    &mtk_clk_gate_ops_setclr)
    static const struct mtk_gate ipe_clks[] = {
    GATE_IPE(CLK_IPE_LARB7, "ipe_larb7", "ipe_sel", 0),
    GATE_IPE(CLK_IPE_LARB8, "ipe_larb8", "ipe_sel", 1),
    GATE_IPE(CLK_IPE_SMI_SUBCOM, "ipe_smi_subcom", "ipe_sel", 2),
    GATE_IPE(CLK_IPE_FD, "ipe_fd", "ipe_sel", 3),
    GATE_IPE(CLK_IPE_FE, "ipe_fe", "ipe_sel", 4),
    GATE_IPE(CLK_IPE_RSC, "ipe_rsc", "ipe_sel", 5),
    GATE_IPE(CLK_IPE_DPE, "ipe_dpe", "ipe_sel", 6),
    };
    static const struct mtk_clk_desc ipe_desc = {
    .clks = ipe_clks,
    .num_clks = ARRAY_SIZE(ipe_clks),
    };
    static const struct of_device_id of_match_clk_mt6779_ipe[] = {
    {
    .compatible = "mediatek,mt6779-ipesys",
    .data = &ipe_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt6779_ipe);
    static struct platform_driver clk_mt6779_ipe_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6779-ipe",
    .of_match_table = of_match_clk_mt6779_ipe,
    },
    };
    module_platform_driver(clk_mt6779_ipe_drv);
    MODULE_DESCRIPTION("MediaTek MT6779 Image Processing Engine clocks driver");
    MODULE_LICENSE("GPL");
