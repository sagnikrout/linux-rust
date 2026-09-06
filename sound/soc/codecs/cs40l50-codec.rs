//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs40l50-codec.c
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
// CS40L50 Advanced Haptic Driver with waveform memory,
// integrated DSP, and closed-loop algorithms
//
// Copyright 2024 Cirrus Logic, Inc.
//
// Author: James Ogletree <james.ogletree@cirrus.com>

pub const CS40L50_REFCLK_INPUT: c_uint = 0x2C04;
pub const CS40L50_ASP_CONTROL2: c_uint = 0x4808;
pub const CS40L50_ASP_DATA_CONTROL5: c_uint = 0x4840;
// PLL Config
pub const CS40L50_PLL_REFCLK_BCLK: c_uint = 0x0;
pub const CS40L50_PLL_REFCLK_MCLK: c_uint = 0x5;
pub const CS40L50_PLL_REEFCLK_MCLK_CFG: c_uint = 0x00;

pub const CS40L50_PLL_REFCLK_OPEN_LOOP: c_int = 1;
pub const CS40L50_PLL_REFCLK_CLOSED_LOOP: c_int = 0;
pub const CS40L50_PLL_REFCLK_LOOP_SHIFT: c_int = 11;

pub const CS40L50_PLL_REFCLK_FREQ_SHIFT: c_int = 5;

pub const CS40L50_BCLK_RATIO_DEFAULT: c_int = 32;
// ASP Config
pub const CS40L50_ASP_RX_WIDTH_SHIFT: c_int = 24;

pub const CS40L50_ASP_FMT_I2S: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_pll_config {
    pub freq: c_uint,
    pub cfg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_codec {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub daifmt: c_uint,
    pub bclk_ratio: c_uint,
    pub rate: c_uint,
}

    static const struct cs40l50_pll_config cs40l50_pll_cfg[] = {
    { 32768, 0x00 },
    { 1536000, 0x1B },
    { 3072000, 0x21 },
    { 6144000, 0x28 },
    { 9600000, 0x30 },
    { 12288000, 0x33 },
    };
#[no_mangle]
unsafe extern "C" fn cs40l50_get_clk_config(freq: c_uint, cfg: *mut c_uint) -> c_int {
    static int cs40l50_get_clk_config(const unsigned int freq, unsigned int *cfg)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(cs40l50_pll_cfg); i++) {
    if (cs40l50_pll_cfg[i].freq == freq) {
// cfg = cs40l50_pll_cfg[i].cfg;
    return 0;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_swap_ext_clk(codec: *mut cs40l50_codec, clk_src: c_uint) -> c_int {
    static int cs40l50_swap_ext_clk(struct cs40l50_codec *codec, const unsigned int clk_src)
    {
    unsigned int cfg;
    int ret;
    switch (clk_src) {
    case CS40L50_PLL_REFCLK_BCLK:
    ret = cs40l50_get_clk_config(codec.bclk_ratio * codec.rate, &cfg);
    if (ret)
    return ret;
    break;
    case CS40L50_PLL_REFCLK_MCLK:
    cfg = CS40L50_PLL_REEFCLK_MCLK_CFG;
    break;
    default:
    return -EINVAL;
    }
    ret = regmap_update_bits(codec.regmap, CS40L50_REFCLK_INPUT,
    CS40L50_PLL_REFCLK_LOOP_MASK,
    CS40L50_PLL_REFCLK_OPEN_LOOP <<
    CS40L50_PLL_REFCLK_LOOP_SHIFT);
    if (ret)
    return ret;
    ret = regmap_update_bits(codec.regmap, CS40L50_REFCLK_INPUT,
    CS40L50_PLL_REFCLK_FREQ_MASK |
    CS40L50_PLL_REFCLK_SEL_MASK,
    (cfg << CS40L50_PLL_REFCLK_FREQ_SHIFT) | clk_src);
    if (ret)
    return ret;
    return regmap_update_bits(codec.regmap, CS40L50_REFCLK_INPUT,
    CS40L50_PLL_REFCLK_LOOP_MASK,
    CS40L50_PLL_REFCLK_CLOSED_LOOP <<
    CS40L50_PLL_REFCLK_LOOP_SHIFT);
    }
    static int cs40l50_clk_en(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *comp = snd_soc_dapm_to_component(w.dapm);
    struct cs40l50_codec *codec = snd_soc_component_get_drvdata(comp);
    int ret;
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    ret = cs40l50_dsp_write(codec.dev, codec.regmap, CS40L50_STOP_PLAYBACK);
    if (ret)
    return ret;
    ret = cs40l50_dsp_write(codec.dev, codec.regmap, CS40L50_START_I2S);
    if (ret)
    return ret;
    ret = cs40l50_swap_ext_clk(codec, CS40L50_PLL_REFCLK_BCLK);
    if (ret)
    return ret;
    break;
    case SND_SOC_DAPM_PRE_PMD:
    ret = cs40l50_swap_ext_clk(codec, CS40L50_PLL_REFCLK_MCLK);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget cs40l50_dapm_widgets[] = {
    SND_SOC_DAPM_SUPPLY_S("ASP PLL", 0, SND_SOC_NOPM, 0, 0, cs40l50_clk_en,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_AIF_IN("ASPRX1", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("ASPRX2", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT("OUT"),
    };
    static const struct snd_soc_dapm_route cs40l50_dapm_routes[] = {
    { "ASP Playback", core::ptr::null_mut(), "ASP PLL" },
    { "ASPRX1", core::ptr::null_mut(), "ASP Playback" },
    { "ASPRX2", core::ptr::null_mut(), "ASP Playback" },
    { "OUT", core::ptr::null_mut(), "ASPRX1" },
    { "OUT", core::ptr::null_mut(), "ASPRX2" },
    };
#[no_mangle]
unsafe extern "C" fn cs40l50_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int cs40l50_set_dai_fmt(struct snd_soc_dai *codec_dai, unsigned int fmt)
    {
    struct cs40l50_codec *codec = snd_soc_component_get_drvdata(codec_dai.component);
    if ((fmt & SND_SOC_DAIFMT_MASTER_MASK) != SND_SOC_DAIFMT_CBC_CFC)
    return -EINVAL;
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    codec.daifmt = 0;
    break;
    case SND_SOC_DAIFMT_NB_IF:
    codec.daifmt = CS40L50_ASP_FSYNC_INV_MASK;
    break;
    case SND_SOC_DAIFMT_IB_NF:
    codec.daifmt = CS40L50_ASP_BCLK_INV_MASK;
    break;
    case SND_SOC_DAIFMT_IB_IF:
    codec.daifmt = CS40L50_ASP_FSYNC_INV_MASK | CS40L50_ASP_BCLK_INV_MASK;
    break;
    default:
    dev_err(codec.dev, "Invalid clock invert\n");
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    codec.daifmt |= FIELD_PREP(CS40L50_ASP_FMT_MASK, CS40L50_ASP_FMT_I2S);
    break;
    default:
    dev_err(codec.dev, "Unsupported DAI format\n");
    return -EINVAL;
    }
    return 0;
    }
    static int cs40l50_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct cs40l50_codec *codec = snd_soc_component_get_drvdata(dai.component);
    let mut asp_rx_wl: c_uint = params_width(params);
    int ret;
    codec.rate = params_rate(params);
    ret = regmap_update_bits(codec.regmap, CS40L50_ASP_DATA_CONTROL5,
    CS40L50_ASP_RX_WL_MASK, asp_rx_wl);
    if (ret)
    return ret;
    codec.daifmt |= (asp_rx_wl << CS40L50_ASP_RX_WIDTH_SHIFT);
    return regmap_update_bits(codec.regmap, CS40L50_ASP_CONTROL2,
    CS40L50_ASP_FSYNC_INV_MASK |
    CS40L50_ASP_BCLK_INV_MASK |
    CS40L50_ASP_FMT_MASK |
    CS40L50_ASP_RX_WIDTH_MASK, codec.daifmt);
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_set_dai_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    static int cs40l50_set_dai_bclk_ratio(struct snd_soc_dai *dai, unsigned int ratio)
    {
    struct cs40l50_codec *codec = snd_soc_component_get_drvdata(dai.component);
    codec.bclk_ratio = ratio;
    return 0;
    }
    static const struct snd_soc_dai_ops cs40l50_dai_ops = {
    .set_fmt = cs40l50_set_dai_fmt,
    .set_bclk_ratio = cs40l50_set_dai_bclk_ratio,
    .hw_params = cs40l50_hw_params,
    };
    static struct snd_soc_dai_driver cs40l50_dai[] = {
    {
    .name = "cs40l50-pcm",
    .id = 0,
    .playback = {
    .stream_name = "ASP Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    .ops = &cs40l50_dai_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn cs40l50_codec_probe(component: *mut snd_soc_component) -> c_int {
    static int cs40l50_codec_probe(struct snd_soc_component *component)
    {
    struct cs40l50_codec *codec = snd_soc_component_get_drvdata(component);
    codec.bclk_ratio = CS40L50_BCLK_RATIO_DEFAULT;
    return 0;
    }
    static const struct snd_soc_component_driver soc_codec_dev_cs40l50 = {
    .probe = cs40l50_codec_probe,
    .dapm_widgets = cs40l50_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(cs40l50_dapm_widgets),
    .dapm_routes = cs40l50_dapm_routes,
    .num_dapm_routes = ARRAY_SIZE(cs40l50_dapm_routes),
    };
#[no_mangle]
unsafe extern "C" fn cs40l50_codec_driver_probe(pdev: *mut platform_device) -> c_int {
    static int cs40l50_codec_driver_probe(struct platform_device *pdev)
    {
    struct cs40l50 *cs40l50 = dev_get_drvdata(pdev.dev.parent);
    struct cs40l50_codec *codec;
    codec = devm_kzalloc(&pdev.dev, sizeof(*codec), GFP_KERNEL);
    if (!codec)
    return -ENOMEM;
    codec.regmap = cs40l50.regmap;
    codec.dev = &pdev.dev;
    return devm_snd_soc_register_component(&pdev.dev, &soc_codec_dev_cs40l50,
    cs40l50_dai, ARRAY_SIZE(cs40l50_dai));
    }
    static const struct platform_device_id cs40l50_id[] = {
    { .name = "cs40l50-codec" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, cs40l50_id);
    static struct platform_driver cs40l50_codec_driver = {
    .probe = cs40l50_codec_driver_probe,
    .id_table = cs40l50_id,
    .driver = {
    .name = "cs40l50-codec",
    },
    };
    module_platform_driver(cs40l50_codec_driver);
    MODULE_DESCRIPTION("ASoC CS40L50 driver");
    MODULE_AUTHOR("James Ogletree <james.ogletree@cirrus.com>");
    MODULE_LICENSE("GPL");
