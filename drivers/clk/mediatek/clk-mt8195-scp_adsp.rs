//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8195-scp_adsp.c
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
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

    static const struct mtk_gate_regs scp_adsp_cg_regs = {
    .set_ofs = 0x180,
    .clr_ofs = 0x180,
    .sta_ofs = 0x180,
    };

    GATE_MTK(_id, _name, _parent, &scp_adsp_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr)
    static const struct mtk_gate scp_adsp_clks[] = {
    GATE_SCP_ADSP(CLK_SCP_ADSP_AUDIODSP, "scp_adsp_audiodsp", "top_adsp", 0),
    };
    static const struct mtk_clk_desc scp_adsp_desc = {
    .clks = scp_adsp_clks,
    .num_clks = ARRAY_SIZE(scp_adsp_clks),
    };
    static const struct of_device_id of_match_clk_mt8195_scp_adsp[] = {
    {
    .compatible = "mediatek,mt8195-scp_adsp",
    .data = &scp_adsp_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_scp_adsp);
    static struct platform_driver clk_mt8195_scp_adsp_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8195-scp_adsp",
    .of_match_table = of_match_clk_mt8195_scp_adsp,
    },
    };
    module_platform_driver(clk_mt8195_scp_adsp_drv);
    MODULE_DESCRIPTION("MediaTek MT8195 SCP AudioDSP clocks driver");
    MODULE_LICENSE("GPL");
