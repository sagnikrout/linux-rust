//! Automatically rewritten from C to Rust
//! Source: sound/soc/mediatek/mt8365/mt8365-dai-pcm.c
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
// MediaTek 8365 ALSA SoC Audio DAI PCM Control
//
// Copyright (c) 2024 MediaTek Inc.
// Authors: Jia Zeng <jia.zeng@mediatek.com>
// Alexandre Mergnat <amergnat@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_pcm_intf_data {
    pub slave_mode: bool,
    pub lrck_inv: bool,
    pub bck_inv: bool,
    pub format: c_uint,
}

// DAI Drivers
#[no_mangle]
unsafe extern "C" fn mt8365_dai_enable_pcm1(afe: *mut mtk_base_afe) {
    static void mt8365_dai_enable_pcm1(struct mtk_base_afe *afe)
    {
    regmap_update_bits(afe.regmap, PCM_INTF_CON1,
    PCM_INTF_CON1_EN, PCM_INTF_CON1_EN);
    }
#[no_mangle]
unsafe extern "C" fn mt8365_dai_disable_pcm1(afe: *mut mtk_base_afe) {
    static void mt8365_dai_disable_pcm1(struct mtk_base_afe *afe)
    {
    regmap_update_bits(afe.regmap, PCM_INTF_CON1,
    PCM_INTF_CON1_EN, 0x0);
    }
    static int mt8365_dai_configure_pcm1(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_pcm_intf_data *pcm_priv = afe_priv.dai_priv[MT8365_AFE_IO_PCM1];
    let mut slave_mode: bool = pcm_priv.slave_mode;
    let mut lrck_inv: bool = pcm_priv.lrck_inv;
    let mut bck_inv: bool = pcm_priv.bck_inv;
    let mut fmt: c_uint = pcm_priv.format;
    let mut bit_width: c_uint = dai.symmetric_sample_bits;
    let mut val: c_uint = 0;
    if (!slave_mode) {
    val |= PCM_INTF_CON1_MASTER_MODE |
    PCM_INTF_CON1_BYPASS_ASRC;
    if (lrck_inv)
    val |= PCM_INTF_CON1_SYNC_OUT_INV;
    if (bck_inv)
    val |= PCM_INTF_CON1_BCLK_OUT_INV;
    } else {
    val |= PCM_INTF_CON1_SLAVE_MODE;
    if (lrck_inv)
    val |= PCM_INTF_CON1_SYNC_IN_INV;
    if (bck_inv)
    val |= PCM_INTF_CON1_BCLK_IN_INV;
// TODO: add asrc setting
    }
    val |= FIELD_PREP(PCM_INTF_CON1_FORMAT_MASK, fmt);
    if (fmt == MT8365_PCM_FORMAT_PCMA ||
    fmt == MT8365_PCM_FORMAT_PCMB)
    val |= PCM_INTF_CON1_SYNC_LEN(1);
    else
    val |= PCM_INTF_CON1_SYNC_LEN(bit_width);
    switch (substream.runtime.rate) {
    case 48000:
    val |= PCM_INTF_CON1_FS_48K;
    break;
    case 32000:
    val |= PCM_INTF_CON1_FS_32K;
    break;
    case 16000:
    val |= PCM_INTF_CON1_FS_16K;
    break;
    case 8000:
    val |= PCM_INTF_CON1_FS_8K;
    break;
    default:
    return -EINVAL;
    }
    if (bit_width > 16)
    val |= PCM_INTF_CON1_24BIT | PCM_INTF_CON1_64BCK;
    else
    val |= PCM_INTF_CON1_16BIT | PCM_INTF_CON1_32BCK;
    val |= PCM_INTF_CON1_EXT_MODEM;
    regmap_update_bits(afe.regmap, PCM_INTF_CON1,
    PCM_INTF_CON1_CONFIG_MASK, val);
    return 0;
    }
    static int mt8365_dai_pcm1_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    if (snd_soc_dai_active(dai))
    return 0;
    mt8365_afe_enable_main_clk(afe);
    return 0;
    }
    static void mt8365_dai_pcm1_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    if (snd_soc_dai_active(dai))
    return;
    mt8365_dai_disable_pcm1(afe);
    mt8365_afe_disable_main_clk(afe);
    }
    static int mt8365_dai_pcm1_prepare(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    int ret;
    if ((snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_PLAYBACK) +
    snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_CAPTURE)) > 1) {
    dev_info(afe.dev, "%s '%s' active(%u-%u) already\n",
    __func__, snd_pcm_stream_str(substream),
    snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_PLAYBACK),
    snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_CAPTURE));
    return 0;
    }
    ret = mt8365_dai_configure_pcm1(substream, dai);
    if (ret)
    return ret;
    mt8365_dai_enable_pcm1(afe);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt8365_dai_pcm1_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int mt8365_dai_pcm1_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_pcm_intf_data *pcm_priv = afe_priv.dai_priv[MT8365_AFE_IO_PCM1];
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    pcm_priv.format = MT8365_PCM_FORMAT_I2S;
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    pcm_priv.bck_inv = false;
    pcm_priv.lrck_inv = false;
    break;
    case SND_SOC_DAIFMT_NB_IF:
    pcm_priv.bck_inv = false;
    pcm_priv.lrck_inv = true;
    break;
    case SND_SOC_DAIFMT_IB_NF:
    pcm_priv.bck_inv = true;
    pcm_priv.lrck_inv = false;
    break;
    case SND_SOC_DAIFMT_IB_IF:
    pcm_priv.bck_inv = true;
    pcm_priv.lrck_inv = true;
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    pcm_priv.slave_mode = true;
    break;
    case SND_SOC_DAIFMT_CBC_CFC:
    pcm_priv.slave_mode = false;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct snd_soc_dai_ops mt8365_dai_pcm1_ops = {
    .startup	= mt8365_dai_pcm1_startup,
    .shutdown	= mt8365_dai_pcm1_shutdown,
    .prepare	= mt8365_dai_pcm1_prepare,
    .set_fmt	= mt8365_dai_pcm1_set_fmt,
    };
    static struct snd_soc_dai_driver mtk_dai_pcm_driver[] = {
    {
    .name = "PCM1",
    .id = MT8365_AFE_IO_PCM1,
    .playback = {
    .stream_name = "PCM1 Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000 |
    SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    },
    .capture = {
    .stream_name = "PCM1 Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000 |
    SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    },
    .ops = &mt8365_dai_pcm1_ops,
    .symmetric_rate = 1,
    .symmetric_sample_bits = 1,
    }
    };
// DAI widget
    static const struct snd_soc_dapm_widget mtk_dai_pcm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("PCM1 Out"),
    SND_SOC_DAPM_INPUT("PCM1 In"),
    };
// DAI route
    static const struct snd_soc_dapm_route mtk_dai_pcm_routes[] = {
    {"PCM1 Playback", core::ptr::null_mut(), "O07"},
    {"PCM1 Playback", core::ptr::null_mut(), "O08"},
    {"PCM1 Out", core::ptr::null_mut(), "PCM1 Playback"},
    {"I09", core::ptr::null_mut(), "PCM1 Capture"},
    {"I22", core::ptr::null_mut(), "PCM1 Capture"},
    {"PCM1 Capture", core::ptr::null_mut(), "PCM1 In"},
    };
#[no_mangle]
unsafe extern "C" fn init_pcmif_priv_data(afe: *mut mtk_base_afe) -> c_int {
    static int init_pcmif_priv_data(struct mtk_base_afe *afe)
    {
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_pcm_intf_data *pcmif_priv;
    pcmif_priv = devm_kzalloc(afe.dev, sizeof(struct mt8365_pcm_intf_data),
    GFP_KERNEL);
    if (!pcmif_priv)
    return -ENOMEM;
    afe_priv.dai_priv[MT8365_AFE_IO_PCM1] = pcmif_priv;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt8365_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int {
    int mt8365_dai_pcm_register(struct mtk_base_afe *afe)
    {
    struct mtk_base_afe_dai *dai;
    dai = devm_kzalloc(afe.dev, sizeof(*dai), GFP_KERNEL);
    if (!dai)
    return -ENOMEM;
    list_add(&dai.list, &afe.sub_dais);
    dai.dai_drivers = mtk_dai_pcm_driver;
    dai.num_dai_drivers = ARRAY_SIZE(mtk_dai_pcm_driver);
    dai.dapm_widgets = mtk_dai_pcm_widgets;
    dai.num_dapm_widgets = ARRAY_SIZE(mtk_dai_pcm_widgets);
    dai.dapm_routes = mtk_dai_pcm_routes;
    dai.num_dapm_routes = ARRAY_SIZE(mtk_dai_pcm_routes);
    return init_pcmif_priv_data(afe);
    }
