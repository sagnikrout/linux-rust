//! Automatically rewritten from C to Rust
//! Source: sound/soc/mediatek/mt8183/mt8183-dai-hostless.c
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
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// dai component
    static const struct snd_soc_dapm_route mtk_dai_hostless_routes[] = {
// Hostless ADDA Loopback
    {"ADDA_DL_CH1", "ADDA_UL_CH1", "Hostless LPBK DL"},
    {"ADDA_DL_CH1", "ADDA_UL_CH2", "Hostless LPBK DL"},
    {"ADDA_DL_CH2", "ADDA_UL_CH1", "Hostless LPBK DL"},
    {"ADDA_DL_CH2", "ADDA_UL_CH2", "Hostless LPBK DL"},
    {"Hostless LPBK UL", core::ptr::null_mut(), "ADDA Capture"},
// Hostless Speech
    {"ADDA_DL_CH1", "PCM_1_CAP_CH1", "Hostless Speech DL"},
    {"ADDA_DL_CH2", "PCM_1_CAP_CH1", "Hostless Speech DL"},
    {"ADDA_DL_CH2", "PCM_1_CAP_CH2", "Hostless Speech DL"},
    {"ADDA_DL_CH1", "PCM_2_CAP_CH1", "Hostless Speech DL"},
    {"ADDA_DL_CH2", "PCM_2_CAP_CH1", "Hostless Speech DL"},
    {"ADDA_DL_CH2", "PCM_2_CAP_CH2", "Hostless Speech DL"},
    {"PCM_1_PB_CH1", "ADDA_UL_CH1", "Hostless Speech DL"},
    {"PCM_1_PB_CH2", "ADDA_UL_CH2", "Hostless Speech DL"},
    {"PCM_2_PB_CH1", "ADDA_UL_CH1", "Hostless Speech DL"},
    {"PCM_2_PB_CH2", "ADDA_UL_CH2", "Hostless Speech DL"},
    {"Hostless Speech UL", core::ptr::null_mut(), "PCM 1 Capture"},
    {"Hostless Speech UL", core::ptr::null_mut(), "PCM 2 Capture"},
    {"Hostless Speech UL", core::ptr::null_mut(), "ADDA Capture"},
    };
// dai ops
    static int mtk_dai_hostless_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    return snd_soc_set_runtime_hwparams(substream, afe.mtk_afe_hardware);
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
    .id = MT8183_DAI_HOSTLESS_LPBK,
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
    .name = "Hostless Speech DAI",
    .id = MT8183_DAI_HOSTLESS_SPEECH,
    .playback = {
    .stream_name = "Hostless Speech DL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .capture = {
    .stream_name = "Hostless Speech UL",
    .channels_min = 1,
    .channels_max = 2,
    .rates = MTK_HOSTLESS_RATES,
    .formats = MTK_HOSTLESS_FORMATS,
    },
    .ops = &mtk_dai_hostless_ops,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int {
    int mt8183_dai_hostless_register(struct mtk_base_afe *afe)
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
