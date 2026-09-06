//! Automatically rewritten from C to Rust
//! Source: sound/soc/qcom/qdsp6/q6afe-clocks.c
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
// Copyright (c) 2020, Linaro Limited

    .clk_id	= id,				\
    .q6dsp_clk_id	= Q6AFE_##id,		\
    .name = #id,				\
    .rate = 19200000,			\
    }
    static const struct q6dsp_clk_init q6afe_clks[] = {
    Q6AFE_CLK(LPASS_CLK_ID_PRI_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_PRI_MI2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEC_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEC_MI2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_TER_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_TER_MI2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUAD_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUAD_MI2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SPEAKER_I2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SPEAKER_I2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SPEAKER_I2S_OSR),
    Q6AFE_CLK(LPASS_CLK_ID_QUI_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUI_MI2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEN_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEN_MI2S_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT0_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT1_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT2_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT3_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT4_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT5_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_INT6_MI2S_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUI_MI2S_OSR),
    Q6AFE_CLK(LPASS_CLK_ID_PRI_PCM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_PRI_PCM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEC_PCM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEC_PCM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_TER_PCM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_TER_PCM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUAD_PCM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUAD_PCM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUIN_PCM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUIN_PCM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUI_PCM_OSR),
    Q6AFE_CLK(LPASS_CLK_ID_PRI_TDM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_PRI_TDM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEC_TDM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_SEC_TDM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_TER_TDM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_TER_TDM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUAD_TDM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUAD_TDM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUIN_TDM_IBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUIN_TDM_EBIT),
    Q6AFE_CLK(LPASS_CLK_ID_QUIN_TDM_OSR),
    Q6AFE_CLK(LPASS_CLK_ID_MCLK_1),
    Q6AFE_CLK(LPASS_CLK_ID_MCLK_2),
    Q6AFE_CLK(LPASS_CLK_ID_MCLK_3),
    Q6AFE_CLK(LPASS_CLK_ID_MCLK_4),
    Q6AFE_CLK(LPASS_CLK_ID_INTERNAL_DIGITAL_CODEC_CORE),
    Q6AFE_CLK(LPASS_CLK_ID_INT_MCLK_0),
    Q6AFE_CLK(LPASS_CLK_ID_INT_MCLK_1),
    Q6AFE_CLK(LPASS_CLK_ID_WSA_CORE_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_WSA_CORE_NPL_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_VA_CORE_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_TX_CORE_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_TX_CORE_NPL_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_RX_CORE_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_RX_CORE_NPL_MCLK),
    Q6AFE_CLK(LPASS_CLK_ID_VA_CORE_2X_MCLK),
    Q6DSP_VOTE_CLK(LPASS_HW_AVTIMER_VOTE,
    Q6AFE_LPASS_CORE_AVTIMER_BLOCK,
    "LPASS_AVTIMER_MACRO"),
    Q6DSP_VOTE_CLK(LPASS_HW_MACRO_VOTE,
    Q6AFE_LPASS_CORE_HW_MACRO_BLOCK,
    "LPASS_HW_MACRO"),
    Q6DSP_VOTE_CLK(LPASS_HW_DCODEC_VOTE,
    Q6AFE_LPASS_CORE_HW_DCODEC_BLOCK,
    "LPASS_HW_DCODEC"),
    };
    static const struct q6dsp_clk_desc q6dsp_clk_q6afe __maybe_unused = {
    .clks = q6afe_clks,
    .num_clks = ARRAY_SIZE(q6afe_clks),
    .lpass_set_clk = q6afe_set_lpass_clock,
    .lpass_vote_clk = q6afe_vote_lpass_core_hw,
    .lpass_unvote_clk = q6afe_unvote_lpass_core_hw,
    };

    static const struct of_device_id q6afe_clock_device_id[] = {
    { .compatible = "qcom,q6afe-clocks", .data = &q6dsp_clk_q6afe },
    {},
    };
    MODULE_DEVICE_TABLE(of, q6afe_clock_device_id);

    static struct platform_driver q6afe_clock_platform_driver = {
    .driver = {
    .name = "q6afe-clock",
    .of_match_table = of_match_ptr(q6afe_clock_device_id),
    },
    .probe = q6dsp_clock_dev_probe,
    };
    module_platform_driver(q6afe_clock_platform_driver);
    MODULE_DESCRIPTION("Q6 Audio Frontend clock driver");
    MODULE_LICENSE("GPL v2");
