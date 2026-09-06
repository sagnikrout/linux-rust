//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8183-ipu0.c
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
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

    static const struct mtk_gate_regs ipu_core0_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };

    GATE_MTK(_id, _name, _parent, &ipu_core0_cg_regs, _shift,	\
    &mtk_clk_gate_ops_setclr)
    static const struct mtk_gate ipu_core0_clks[] = {
    GATE_IPU_CORE0(CLK_IPU_CORE0_JTAG, "ipu_core0_jtag", "dsp_sel", 0),
    GATE_IPU_CORE0(CLK_IPU_CORE0_AXI, "ipu_core0_axi", "dsp_sel", 1),
    GATE_IPU_CORE0(CLK_IPU_CORE0_IPU, "ipu_core0_ipu", "dsp_sel", 2),
    };
    static const struct mtk_clk_desc ipu_core0_desc = {
    .clks = ipu_core0_clks,
    .num_clks = ARRAY_SIZE(ipu_core0_clks),
    };
    static const struct of_device_id of_match_clk_mt8183_ipu_core0[] = {
    {
    .compatible = "mediatek,mt8183-ipu_core0",
    .data = &ipu_core0_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_ipu_core0);
    static struct platform_driver clk_mt8183_ipu_core0_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8183-ipu_core0",
    .of_match_table = of_match_clk_mt8183_ipu_core0,
    },
    };
    module_platform_driver(clk_mt8183_ipu_core0_drv);
    MODULE_DESCRIPTION("MediaTek MT8183 Pri. Image Processing Unit clocks driver");
    MODULE_LICENSE("GPL");
