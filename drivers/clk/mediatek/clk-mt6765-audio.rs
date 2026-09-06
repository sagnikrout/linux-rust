//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt6765-audio.c
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
// Author: Owen Chen <owen.chen@mediatek.com>
//

    static const struct mtk_gate_regs audio0_cg_regs = {
    .set_ofs = 0x0,
    .clr_ofs = 0x0,
    .sta_ofs = 0x0,
    };
    static const struct mtk_gate_regs audio1_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x4,
    .sta_ofs = 0x4,
    };

    GATE_MTK(_id, _name, _parent, &audio0_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr)

    GATE_MTK(_id, _name, _parent, &audio1_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr)
    static const struct mtk_gate audio_clks[] = {
// AUDIO0
    GATE_AUDIO0(CLK_AUDIO_AFE, "aud_afe", "audio_ck", 2),
    GATE_AUDIO0(CLK_AUDIO_22M, "aud_22m", "aud_engen1_ck", 8),
    GATE_AUDIO0(CLK_AUDIO_APLL_TUNER, "aud_apll_tuner",
    "aud_engen1_ck", 19),
    GATE_AUDIO0(CLK_AUDIO_ADC, "aud_adc", "audio_ck", 24),
    GATE_AUDIO0(CLK_AUDIO_DAC, "aud_dac", "audio_ck", 25),
    GATE_AUDIO0(CLK_AUDIO_DAC_PREDIS, "aud_dac_predis",
    "audio_ck", 26),
    GATE_AUDIO0(CLK_AUDIO_TML, "aud_tml", "audio_ck", 27),
// AUDIO1
    GATE_AUDIO1(CLK_AUDIO_I2S1_BCLK, "aud_i2s1_bclk",
    "audio_ck", 4),
    GATE_AUDIO1(CLK_AUDIO_I2S2_BCLK, "aud_i2s2_bclk",
    "audio_ck", 5),
    GATE_AUDIO1(CLK_AUDIO_I2S3_BCLK, "aud_i2s3_bclk",
    "audio_ck", 6),
    GATE_AUDIO1(CLK_AUDIO_I2S4_BCLK, "aud_i2s4_bclk",
    "audio_ck", 7),
    };
    static const struct mtk_clk_desc audio_desc = {
    .clks = audio_clks,
    .num_clks = ARRAY_SIZE(audio_clks),
    };
    static const struct of_device_id of_match_clk_mt6765_audio[] = {
    {
    .compatible = "mediatek,mt6765-audsys",
    .data = &audio_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt6765_audio);
    static struct platform_driver clk_mt6765_audio_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt6765-audio",
    .of_match_table = of_match_clk_mt6765_audio,
    },
    };
    module_platform_driver(clk_mt6765_audio_drv);
    MODULE_DESCRIPTION("MediaTek MT6765 audio clocks driver");
    MODULE_LICENSE("GPL");
