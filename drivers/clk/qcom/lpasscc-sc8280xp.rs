//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/lpasscc-sc8280xp.c
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
// Copyright (c) 2022, Linaro Limited
//

    static const struct qcom_reset_map lpass_audiocc_sc8280xp_resets[] = {
    [LPASS_AUDIO_SWR_RX_CGCR] = { 0xa0, 1 },
    [LPASS_AUDIO_SWR_WSA_CGCR] = { 0xb0, 1 },
    [LPASS_AUDIO_SWR_WSA2_CGCR] = { 0xd8, 1 },
    };
    static const struct regmap_config lpass_audiocc_sc8280xp_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .name = "lpass-audio-csr",
    .max_register = 0x1000,
    };
    static const struct qcom_cc_desc lpass_audiocc_sc8280xp_reset_desc = {
    .config = &lpass_audiocc_sc8280xp_regmap_config,
    .resets = lpass_audiocc_sc8280xp_resets,
    .num_resets = ARRAY_SIZE(lpass_audiocc_sc8280xp_resets),
    };
    static const struct qcom_reset_map lpasscc_sc8280xp_resets[] = {
    [LPASS_AUDIO_SWR_TX_CGCR] = { 0xc010, 1 },
    };
    static const struct regmap_config lpasscc_sc8280xp_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .name = "lpass-tcsr",
    .max_register = 0x12000,
    };
    static const struct qcom_cc_desc lpasscc_sc8280xp_reset_desc = {
    .config = &lpasscc_sc8280xp_regmap_config,
    .resets = lpasscc_sc8280xp_resets,
    .num_resets = ARRAY_SIZE(lpasscc_sc8280xp_resets),
    };
    static const struct of_device_id lpasscc_sc8280xp_match_table[] = {
    {
    .compatible = "qcom,sc8280xp-lpassaudiocc",
    .data = &lpass_audiocc_sc8280xp_reset_desc,
    }, {
    .compatible = "qcom,sc8280xp-lpasscc",
    .data = &lpasscc_sc8280xp_reset_desc,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpasscc_sc8280xp_match_table);
#[no_mangle]
unsafe extern "C" fn lpasscc_sc8280xp_probe(pdev: *mut platform_device) -> c_int {
    static int lpasscc_sc8280xp_probe(struct platform_device *pdev)
    {
    const struct qcom_cc_desc *desc = of_device_get_match_data(&pdev.dev);
    return qcom_cc_probe_by_index(pdev, 0, desc);
    }
    static struct platform_driver lpasscc_sc8280xp_driver = {
    .probe = lpasscc_sc8280xp_probe,
    .driver = {
    .name = "lpasscc-sc8280xp",
    .of_match_table = lpasscc_sc8280xp_match_table,
    },
    };
    module_platform_driver(lpasscc_sc8280xp_driver);
    MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org>");
    MODULE_DESCRIPTION("QTI LPASSCC SC8280XP Driver");
    MODULE_LICENSE("GPL");
