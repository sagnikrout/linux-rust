//! Automatically rewritten from C to Rust
//! Source: sound/soc/qcom/qdsp6/q6prm-clocks.c
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
// Copyright (c) 2021, Linaro Limited

    .clk_id	= id,				\
    .q6dsp_clk_id	= Q6PRM_##id,		\
    .name = #id,				\
    .rate = 19200000,			\
    }
    static const struct q6dsp_clk_init q6prm_clks[] = {
    Q6PRM_CLK(LPASS_CLK_ID_PRI_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_PRI_MI2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SEC_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SEC_MI2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_TER_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_TER_MI2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QUAD_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QUAD_MI2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SPEAKER_I2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SPEAKER_I2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SPEAKER_I2S_OSR),
    Q6PRM_CLK(LPASS_CLK_ID_QUI_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QUI_MI2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SEN_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_SEN_MI2S_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT0_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT1_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT2_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT3_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT4_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT5_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_INT6_MI2S_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QUI_MI2S_OSR),
    Q6PRM_CLK(LPASS_CLK_ID_MCLK_1),
    Q6PRM_CLK(LPASS_CLK_ID_MCLK_2),
    Q6PRM_CLK(LPASS_CLK_ID_MCLK_3),
    Q6PRM_CLK(LPASS_CLK_ID_MCLK_4),
    Q6PRM_CLK(LPASS_CLK_ID_MCLK_5),
    Q6PRM_CLK(LPASS_CLK_ID_WSA_CORE_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA_CORE_NPL_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_VA_CORE_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_TX_CORE_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_TX_CORE_NPL_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_RX_CORE_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_RX_CORE_NPL_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_VA_CORE_2X_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA2_CORE_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA2_CORE_2X_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_RX_CORE_TX_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_RX_CORE_TX_2X_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA_CORE_TX_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA_CORE_TX_2X_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA2_CORE_TX_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_WSA2_CORE_TX_2X_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_RX_CORE_MCLK2_2X_MCLK),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF0_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF0_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF1_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF1_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF2_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF2_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF3_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF3_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF4_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF4_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF5_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF5_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF6_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF6_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF7_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF7_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF8_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF8_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF9_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF9_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF10_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF10_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF11_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF11_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF12_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_QAIF_IF12_EBIT),
    Q6PRM_CLK(LPASS_CLK_ID_VA_QAIF_IF0_IBIT),
    Q6PRM_CLK(LPASS_CLK_ID_VA_QAIF_IF0_EBIT),
    Q6DSP_VOTE_CLK(LPASS_HW_MACRO_VOTE, Q6PRM_HW_CORE_ID_LPASS,
    "LPASS_HW_MACRO"),
    Q6DSP_VOTE_CLK(LPASS_HW_DCODEC_VOTE, Q6PRM_HW_CORE_ID_DCODEC,
    "LPASS_HW_DCODEC"),
    Q6DSP_VOTE_CLK(LPASS_HW_LPR_VOTE, Q6PRM_HW_LPR_VOTE,
    "LPASS_HW_LPR_VOTE"),
    };
    static const struct q6dsp_clk_desc q6dsp_clk_q6prm __maybe_unused = {
    .clks = q6prm_clks,
    .num_clks = ARRAY_SIZE(q6prm_clks),
    .lpass_set_clk = q6prm_set_lpass_clock,
    .lpass_vote_clk = q6prm_vote_lpass_core_hw,
    .lpass_unvote_clk = q6prm_unvote_lpass_core_hw,
    };

    static const struct of_device_id q6prm_clock_device_id[] = {
    { .compatible = "qcom,q6prm-lpass-clocks", .data = &q6dsp_clk_q6prm },
    {},
    };
    MODULE_DEVICE_TABLE(of, q6prm_clock_device_id);

    static struct platform_driver q6prm_clock_platform_driver = {
    .driver = {
    .name = "q6prm-lpass-clock",
    .of_match_table = of_match_ptr(q6prm_clock_device_id),
    },
    .probe = q6dsp_clock_dev_probe,
    };
    module_platform_driver(q6prm_clock_platform_driver);
    MODULE_DESCRIPTION("Q6 Proxy Resource Manager LPASS clock driver");
    MODULE_LICENSE("GPL");
