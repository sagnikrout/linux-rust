//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8183-audio.c
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

    GATE_MTK(_id, _name, _parent, &audio0_cg_regs, _shift,	\
    &mtk_clk_gate_ops_no_setclr)

    GATE_MTK(_id, _name, _parent, &audio1_cg_regs, _shift,	\
    &mtk_clk_gate_ops_no_setclr)
    static const struct mtk_gate audio_clks[] = {
// AUDIO0
    GATE_AUDIO0(CLK_AUDIO_AFE, "aud_afe", "audio_sel",
    2),
    GATE_AUDIO0(CLK_AUDIO_22M, "aud_22m", "aud_eng1_sel",
    8),
    GATE_AUDIO0(CLK_AUDIO_24M, "aud_24m", "aud_eng2_sel",
    9),
    GATE_AUDIO0(CLK_AUDIO_APLL2_TUNER, "aud_apll2_tuner", "aud_eng2_sel",
    18),
    GATE_AUDIO0(CLK_AUDIO_APLL_TUNER, "aud_apll_tuner", "aud_eng1_sel",
    19),
    GATE_AUDIO0(CLK_AUDIO_TDM, "aud_tdm", "apll12_divb",
    20),
    GATE_AUDIO0(CLK_AUDIO_ADC, "aud_adc", "audio_sel",
    24),
    GATE_AUDIO0(CLK_AUDIO_DAC, "aud_dac", "audio_sel",
    25),
    GATE_AUDIO0(CLK_AUDIO_DAC_PREDIS, "aud_dac_predis", "audio_sel",
    26),
    GATE_AUDIO0(CLK_AUDIO_TML, "aud_tml", "audio_sel",
    27),
// AUDIO1
    GATE_AUDIO1(CLK_AUDIO_I2S1, "aud_i2s1", "audio_sel",
    4),
    GATE_AUDIO1(CLK_AUDIO_I2S2, "aud_i2s2", "audio_sel",
    5),
    GATE_AUDIO1(CLK_AUDIO_I2S3, "aud_i2s3", "audio_sel",
    6),
    GATE_AUDIO1(CLK_AUDIO_I2S4, "aud_i2s4", "audio_sel",
    7),
    GATE_AUDIO1(CLK_AUDIO_PDN_ADDA6_ADC, "aud_pdn_adda6_adc", "audio_sel",
    20),
    };
    static const struct mtk_clk_desc audio_desc = {
    .clks = audio_clks,
    .num_clks = ARRAY_SIZE(audio_clks),
    };
#[no_mangle]
unsafe extern "C" fn clk_mt8183_audio_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8183_audio_probe(struct platform_device *pdev)
    {
    int r;
    r = mtk_clk_simple_probe(pdev);
    if (r)
    return r;
    r = devm_of_platform_populate(&pdev.dev);
    if (r)
    mtk_clk_simple_remove(pdev);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8183_audio_remove(pdev: *mut platform_device) {
    static void clk_mt8183_audio_remove(struct platform_device *pdev)
    {
    of_platform_depopulate(&pdev.dev);
    mtk_clk_simple_remove(pdev);
    }
    static const struct of_device_id of_match_clk_mt8183_audio[] = {
    { .compatible = "mediatek,mt8183-audiosys", .data = &audio_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_audio);
    static struct platform_driver clk_mt8183_audio_drv = {
    .probe = clk_mt8183_audio_probe,
    .remove = clk_mt8183_audio_remove,
    .driver = {
    .name = "clk-mt8183-audio",
    .of_match_table = of_match_clk_mt8183_audio,
    },
    };
    module_platform_driver(clk_mt8183_audio_drv);
    MODULE_DESCRIPTION("MediaTek MT8183 audio clocks driver");
    MODULE_LICENSE("GPL");
