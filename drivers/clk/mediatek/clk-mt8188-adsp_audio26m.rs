//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8188-adsp_audio26m.c
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

    static const struct mtk_gate_regs adsp_audio26m_cg_regs = {
    .set_ofs = 0x80,
    .clr_ofs = 0x80,
    .sta_ofs = 0x80,
    };

    GATE_MTK(_id, _name, _parent, &adsp_audio26m_cg_regs, _shift,		\
    &mtk_clk_gate_ops_no_setclr)
    static const struct mtk_gate adsp_audio26m_clks[] = {
    GATE_ADSP_FLAGS(CLK_AUDIODSP_AUDIO26M, "audiodsp_audio26m", "clk26m", 3),
    };
    static const struct mtk_clk_desc adsp_audio26m_desc = {
    .clks = adsp_audio26m_clks,
    .num_clks = ARRAY_SIZE(adsp_audio26m_clks),
    };
    static const struct of_device_id of_match_clk_mt8188_adsp_audio26m[] = {
    { .compatible = "mediatek,mt8188-adsp-audio26m", .data = &adsp_audio26m_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_adsp_audio26m);
    static struct platform_driver clk_mt8188_adsp_audio26m_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8188-adsp_audio26m",
    .of_match_table = of_match_clk_mt8188_adsp_audio26m,
    },
    };
    module_platform_driver(clk_mt8188_adsp_audio26m_drv);
    MODULE_DESCRIPTION("MediaTek MT8188 AudioDSP clocks driver");
    MODULE_LICENSE("GPL");
