//! Automatically rewritten from C to Rust
//! Source: sound/soc/mediatek/mt8186/mt8186-dai-hostless.c
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
// MediaTek ALSA SoC Audio DAI Hostless Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

    static const struct snd_pcm_hardware mt8186_hostless_hardware = {
    .info = (SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_MMAP_VALID),
    .period_bytes_min = 256,
    .period_bytes_max = 4 * 48 * 1024,
    .periods_min = 2,
    .periods_max = 256,
    .buffer_bytes_max = 4 * 48 * 1024,
    .fifo_size = 0,
    };
// dai component
    static const struct snd_soc_dapm_route mtk_dai_hostless_routes[] = {
// Hostless ADDA Loopback
    {"ADDA_DL_CH1", "ADDA_UL_CH1 Switch", "Hostless LPBK DL"},
    {"ADDA_DL_CH1", "ADDA_UL_CH2 Switch", "Hostless LPBK DL"},
    {"ADDA_DL_CH2", "ADDA_UL_CH1 Switch", "Hostless LPBK DL"},
    {"ADDA_DL_CH2", "ADDA_UL_CH2 Switch", "Hostless LPBK DL"},
    {"I2S1_CH1", "ADDA_UL_CH1 Switch", "Hostless LPBK DL"},
    {"I2S1_CH2", "ADDA_UL_CH2 Switch", "Hostless LPBK DL"},
    {"I2S3_CH1", "ADDA_UL_CH1 Switch", "Hostless LPBK DL"},
    {"I2S3_CH1", "ADDA_UL_CH2 Switch", "Hostless LPBK DL"},
    {"I2S3_CH2", "ADDA_UL_CH1 Switch", "Hostless LPBK DL"},
    {"I2S3_CH2", "ADDA_UL_CH2 Switch", "Hostless LPBK DL"},
    {"Hostless LPBK UL", core::ptr::null_mut(), "ADDA_UL_Mux"},
// Hostelss FM
// connsys_i2s to hw gain 1
    {"Hostless FM UL", core::ptr::null_mut(), "Connsys I2S"},
    {"HW_GAIN1_IN_CH1", "CONNSYS_I2S_CH1 Switch", "Hostless FM DL"},
    {"HW_GAIN1_IN_CH2", "CONNSYS_I2S_CH2 Switch", "Hostless FM DL"},
// hw gain to adda dl
    {"Hostless FM UL", core::ptr::null_mut(), "HW Gain 1 Out"},
    {"ADDA_DL_CH1", "GAIN1_OUT_CH1 Switch", "Hostless FM DL"},
    {"ADDA_DL_CH2", "GAIN1_OUT_CH2 Switch", "Hostless FM DL"},
// hw gain to i2s3
    {"I2S3_CH1", "GAIN1_OUT_CH1 Switch", "Hostless FM DL"},
    {"I2S3_CH2", "GAIN1_OUT_CH2 Switch", "Hostless FM DL"},
// hw gain to i2s1
    {"I2S1_CH1", "GAIN1_OUT_CH1 Switch", "Hostless FM DL"},
    {"I2S1_CH2", "GAIN1_OUT_CH2 Switch", "Hostless FM DL"},
// Hostless_SRC
    {"ADDA_DL_CH1", "SRC_1_OUT_CH1 Switch", "Hostless_SRC_1_DL"},
    {"ADDA_DL_CH2", "SRC_1_OUT_CH2 Switch", "Hostless_SRC_1_DL"},
    {"I2S1_CH1", "SRC_1_OUT_CH1 Switch", "Hostless_SRC_1_DL"},
    {"I2S1_CH2", "SRC_1_OUT_CH2 Switch", "Hostless_SRC_1_DL"},
    {"I2S3_CH1", "SRC_1_OUT_CH1 Switch", "Hostless_SRC_1_DL"},
    {"I2S3_CH2", "SRC_1_OUT_CH2 Switch", "Hostless_SRC_1_DL"},
    {"Hostless_SRC_1_UL", core::ptr::null_mut(), "HW_SRC_1_Out"},
// Hostless_SRC_bargein
    {"HW_SRC_1_IN_CH1", "I2S0_CH1 Switch", "Hostless_SRC_Bargein_DL"},
    {"HW_SRC_1_IN_CH2", "I2S0_CH2 Switch", "Hostless_SRC_Bargein_DL"},
    {"Hostless_SRC_Bargein_UL", core::ptr::null_mut(), "I2S0"},
// Hostless AAudio
    {"Hostless HW Gain AAudio In", core::ptr::null_mut(), "HW Gain 2 In"},
    {"Hostless SRC AAudio UL", core::ptr::null_mut(), "HW Gain 2 Out"},
    {"HW_SRC_2_IN_CH1", "HW_GAIN2_OUT_CH1 Switch", "Hostless SRC AAudio DL"},
    {"HW_SRC_2_IN_CH2", "HW_GAIN2_OUT_CH2 Switch", "Hostless SRC AAudio DL"},
    };
// dai ops
    static int mtk_dai_hostless_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    struct snd_pcm_runtime *runtime = substream.runtime;
    int ret;
    snd_soc_set_runtime_hwparams(substream, &mt8186_hostless_hardware);
    ret = snd_pcm_hw_constraint_integer(runtime,
    SNDRV_PCM_HW_PARAM_PERIODS);
    if (ret < 0) {
    dev_err(afe.dev, "snd_pcm_hw_constraint_integer failed\n");
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_dai_ops mtk_dai_hostless_ops = {
    .startup = mtk_dai_hostless_startup,
    };
// dai driver

    SNDRV_PCM_RATE_88200 |\
    SNDRV_PCM_RATE_96000 |\
    SNDRV_PCM_RATE_176400 |\
    SNDRV_PCM_RATE_192000)

    SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S32_LE)
    static struct snd_soc_dai_driver mtk_dai_hostless_driver[] = {
    {
    .name = "Hostless LPBK DAI",
    .id = MT8186_DAI_HOSTLESS_LPBK,
    .playback = {
    .stream_name = "Hostless LPBK DL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .capture = {
    .stream_name = "Hostless LPBK UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless FM DAI",
    .id = MT8186_DAI_HOSTLESS_FM,
    .playback = {
    .stream_name = "Hostless FM DL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .capture = {
    .stream_name = "Hostless FM UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless_SRC_1_DAI",
    .id = MT8186_DAI_HOSTLESS_SRC_1,
    .playback = {
    .stream_name = "Hostless_SRC_1_DL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .capture = {
    .stream_name = "Hostless_SRC_1_UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless_SRC_Bargein_DAI",
    .id = MT8186_DAI_HOSTLESS_SRC_BARGEIN,
    .playback = {
    .stream_name = "Hostless_SRC_Bargein_DL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .capture = {
    .stream_name = "Hostless_SRC_Bargein_UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
// BE dai
    {
    .name = "Hostless_UL1 DAI",
    .id = MT8186_DAI_HOSTLESS_UL1,
    .capture = {
    .stream_name = "Hostless_UL1 UL",
    .channels_min = 1,
    .channels_max = 4,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless_UL2 DAI",
    .id = MT8186_DAI_HOSTLESS_UL2,
    .capture = {
    .stream_name = "Hostless_UL2 UL",
    .channels_min = 1,
    .channels_max = 4,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless_UL3 DAI",
    .id = MT8186_DAI_HOSTLESS_UL3,
    .capture = {
    .stream_name = "Hostless_UL3 UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless_UL5 DAI",
    .id = MT8186_DAI_HOSTLESS_UL5,
    .capture = {
    .stream_name = "Hostless_UL5 UL",
    .channels_min = 1,
    .channels_max = 12,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless_UL6 DAI",
    .id = MT8186_DAI_HOSTLESS_UL6,
    .capture = {
    .stream_name = "Hostless_UL6 UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless HW Gain AAudio DAI",
    .id = MT8186_DAI_HOSTLESS_HW_GAIN_AAUDIO,
    .capture = {
    .stream_name = "Hostless HW Gain AAudio In",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    {
    .name = "Hostless SRC AAudio DAI",
    .id = MT8186_DAI_HOSTLESS_SRC_AAUDIO,
    .playback = {
    .stream_name = "Hostless SRC AAudio DL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .capture = {
    .stream_name = "Hostless SRC AAudio UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int {
    int mt8186_dai_hostless_register(struct mtk_base_afe *afe)
    {
    struct mtk_base_afe_dai *dai;
    dai = devm_kzalloc(afe.dev, sizeof(*dai), GFP_KERNEL);
    if (!dai)
    return -ENOMEM;
    list_add(&dai.list, &afe.sub_dais);
    dai.dai_drivers = mtk_dai_hostless_driver;
    dai.num_dai_drivers = ARRAY_SIZE(mtk_dai_hostless_driver);
    dai.dapm_routes = mtk_dai_hostless_routes;
    dai.num_dapm_routes = ARRAY_SIZE(mtk_dai_hostless_routes);
    return 0;
    }
