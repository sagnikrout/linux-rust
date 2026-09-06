//! Automatically rewritten from C to Rust
//! Source: sound/soc/meson/t9015.c
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
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

pub const BLOCK_EN: c_uint = 0x00;
pub const LORN_EN: c_int = 0;
pub const LORP_EN: c_int = 1;
pub const LOLN_EN: c_int = 2;
pub const LOLP_EN: c_int = 3;
pub const DACR_EN: c_int = 4;
pub const DACL_EN: c_int = 5;
pub const DACR_INV: c_int = 20;
pub const DACL_INV: c_int = 21;
pub const DACR_SRC: c_int = 22;
pub const DACL_SRC: c_int = 23;

pub const VOL_CTRL0: c_uint = 0x04;
pub const GAIN_H: c_int = 31;
pub const GAIN_L: c_int = 23;
pub const VOL_CTRL1: c_uint = 0x08;
pub const DAC_MONO: c_int = 8;
pub const RAMP_RATE: c_int = 10;
pub const VC_RAMP_MODE: c_int = 12;
pub const MUTE_MODE: c_int = 13;
pub const UNMUTE_MODE: c_int = 14;
pub const DAC_SOFT_MUTE: c_int = 15;
pub const DACR_VC: c_int = 16;
pub const DACL_VC: c_int = 24;
pub const LINEOUT_CFG: c_uint = 0x0c;
pub const LORN_POL: c_int = 0;
pub const LORP_POL: c_int = 4;
pub const LOLN_POL: c_int = 8;
pub const LOLP_POL: c_int = 12;
pub const POWER_CFG: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t9015 {
    pub avdd: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn t9015_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int t9015_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct snd_soc_component *component = dai.component;
    unsigned int val;
    switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    val = I2S_MODE;
    break;
    case SND_SOC_DAIFMT_CBC_CFC:
    val = 0;
    break;
    default:
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, BLOCK_EN, I2S_MODE, val);
    if (((fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S) &&
    ((fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_LEFT_J))
    return -EINVAL;
    return 0;
    }
    static const u64 t9015_dai_selectable_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J;
    static const struct snd_soc_dai_ops t9015_dai_ops = {
    .set_fmt = t9015_dai_set_fmt,
    .auto_selectable_formats = &t9015_dai_selectable_formats,
    .num_auto_selectable_formats = 1,
    };
    static struct snd_soc_dai_driver t9015_dai = {
    .name = "t9015-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = (SNDRV_PCM_FMTBIT_S8 |
    SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S20_LE |
    SNDRV_PCM_FMTBIT_S24_LE),
    },
    .ops = &t9015_dai_ops,
    };
    static const DECLARE_TLV_DB_MINMAX_MUTE(dac_vol_tlv, -9525, 0);
    static const char * const ramp_rate_txt[] = { "Fast", "Slow" };
    static SOC_ENUM_SINGLE_DECL(ramp_rate_enum, VOL_CTRL1, RAMP_RATE,
    ramp_rate_txt);
    static const char * const dacr_in_txt[] = { "Right", "Left" };
    static SOC_ENUM_SINGLE_DECL(dacr_in_enum, BLOCK_EN, DACR_SRC, dacr_in_txt);
    static const char * const dacl_in_txt[] = { "Left", "Right" };
    static SOC_ENUM_SINGLE_DECL(dacl_in_enum, BLOCK_EN, DACL_SRC, dacl_in_txt);
    static const char * const mono_txt[] = { "Stereo", "Mono"};
    static SOC_ENUM_SINGLE_DECL(mono_enum, VOL_CTRL1, DAC_MONO, mono_txt);
    static const struct snd_kcontrol_new t9015_snd_controls[] = {
// Volume Controls
    SOC_ENUM("Playback Channel Mode", mono_enum),
    SOC_SINGLE("Playback Switch", VOL_CTRL1, DAC_SOFT_MUTE, 1, 1),
    SOC_DOUBLE_TLV("Playback Volume", VOL_CTRL1, DACL_VC, DACR_VC,
    0xff, 0, dac_vol_tlv),
// Ramp Controls
    SOC_ENUM("Ramp Rate", ramp_rate_enum),
    SOC_SINGLE("Volume Ramp Switch", VOL_CTRL1, VC_RAMP_MODE, 1, 0),
    SOC_SINGLE("Mute Ramp Switch", VOL_CTRL1, MUTE_MODE, 1, 0),
    SOC_SINGLE("Unmute Ramp Switch", VOL_CTRL1, UNMUTE_MODE, 1, 0),
    };
    static const struct snd_kcontrol_new t9015_right_dac_mux =
    SOC_DAPM_ENUM("Right DAC Source", dacr_in_enum);
    static const struct snd_kcontrol_new t9015_left_dac_mux =
    SOC_DAPM_ENUM("Left DAC Source", dacl_in_enum);
    static const struct snd_soc_dapm_widget t9015_dapm_widgets[] = {
    SND_SOC_DAPM_AIF_IN("Right IN", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("Left IN", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX("Right DAC Sel", SND_SOC_NOPM, 0, 0,
    &t9015_right_dac_mux),
    SND_SOC_DAPM_MUX("Left DAC Sel", SND_SOC_NOPM, 0, 0,
    &t9015_left_dac_mux),
    SND_SOC_DAPM_DAC("Right DAC", core::ptr::null_mut(), BLOCK_EN, DACR_EN, 0),
    SND_SOC_DAPM_DAC("Left DAC",  core::ptr::null_mut(), BLOCK_EN, DACL_EN, 0),
    SND_SOC_DAPM_OUT_DRV("Right- Driver", BLOCK_EN, LORN_EN, 0,
    core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV("Right+ Driver", BLOCK_EN, LORP_EN, 0,
    core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV("Left- Driver",  BLOCK_EN, LOLN_EN, 0,
    core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV("Left+ Driver",  BLOCK_EN, LOLP_EN, 0,
    core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("LORN"),
    SND_SOC_DAPM_OUTPUT("LORP"),
    SND_SOC_DAPM_OUTPUT("LOLN"),
    SND_SOC_DAPM_OUTPUT("LOLP"),
    };
    static const struct snd_soc_dapm_route t9015_dapm_routes[] = {
    { "Right IN", core::ptr::null_mut(), "Playback" },
    { "Left IN",  core::ptr::null_mut(), "Playback" },
    { "Right DAC Sel", "Right", "Right IN" },
    { "Right DAC Sel", "Left",  "Left IN" },
    { "Left DAC Sel",  "Right", "Right IN" },
    { "Left DAC Sel",  "Left",  "Left IN" },
    { "Right DAC", core::ptr::null_mut(), "Right DAC Sel" },
    { "Left DAC",  core::ptr::null_mut(), "Left DAC Sel" },
    { "Right- Driver", core::ptr::null_mut(), "Right DAC" },
    { "Right+ Driver", core::ptr::null_mut(), "Right DAC" },
    { "Left- Driver",  core::ptr::null_mut(), "Left DAC"  },
    { "Left+ Driver",  core::ptr::null_mut(), "Left DAC"  },
    { "LORN", core::ptr::null_mut(), "Right- Driver", },
    { "LORP", core::ptr::null_mut(), "Right+ Driver", },
    { "LOLN", core::ptr::null_mut(), "Left- Driver",  },
    { "LOLP", core::ptr::null_mut(), "Left+ Driver",  },
    };
    static int t9015_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    struct t9015 *priv = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    let mut now: enum snd_soc_bias_level = snd_soc_dapm_get_bias_level(dapm);
    int ret;
    switch (level) {
    case SND_SOC_BIAS_ON:
    snd_soc_component_update_bits(component, BLOCK_EN,
    BIAS_CURRENT_EN,
    BIAS_CURRENT_EN);
    break;
    case SND_SOC_BIAS_PREPARE:
    snd_soc_component_update_bits(component, BLOCK_EN,
    BIAS_CURRENT_EN,
    0);
    break;
    case SND_SOC_BIAS_STANDBY:
    ret = regulator_enable(priv.avdd);
    if (ret) {
    dev_err(component.dev, "AVDD enable failed\n");
    return ret;
    }
    if (now == SND_SOC_BIAS_OFF) {
    snd_soc_component_update_bits(component, BLOCK_EN,
    VMID_GEN_EN | VMID_GEN_FAST | REFP_BUF_EN,
    VMID_GEN_EN | VMID_GEN_FAST | REFP_BUF_EN);
    mdelay(200);
    snd_soc_component_update_bits(component, BLOCK_EN,
    VMID_GEN_FAST,
    0);
    }
    break;
    case SND_SOC_BIAS_OFF:
    snd_soc_component_update_bits(component, BLOCK_EN,
    VMID_GEN_EN | VMID_GEN_FAST | REFP_BUF_EN,
    0);
    regulator_disable(priv.avdd);
    break;
    }
    return 0;
    }
    static const struct snd_soc_component_driver t9015_codec_driver = {
    .set_bias_level		= t9015_set_bias_level,
    .controls		= t9015_snd_controls,
    .num_controls		= ARRAY_SIZE(t9015_snd_controls),
    .dapm_widgets		= t9015_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(t9015_dapm_widgets),
    .dapm_routes		= t9015_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(t9015_dapm_routes),
    .suspend_bias_off	= 1,
    .endianness		= 1,
    };
    static const struct regmap_config t9015_regmap_config = {
    .reg_bits		= 32,
    .reg_stride		= 4,
    .val_bits		= 32,
    .max_register		= POWER_CFG,
    };
#[no_mangle]
unsafe extern "C" fn t9015_probe(pdev: *mut platform_device) -> c_int {
    static int t9015_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct t9015 *priv;
    void __iomem *regs;
    struct regmap *regmap;
    struct clk *pclk;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    pclk = devm_clk_get_enabled(dev, "pclk");
    if (IS_ERR(pclk))
    return dev_err_probe(dev, PTR_ERR(pclk), "failed to get core clock\n");
    priv.avdd = devm_regulator_get(dev, "AVDD");
    if (IS_ERR(priv.avdd))
    return dev_err_probe(dev, PTR_ERR(priv.avdd), "failed to AVDD\n");
    ret = device_reset(dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to reset device\n");
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs)) {
    dev_err(dev, "register map failed\n");
    return PTR_ERR(regs);
    }
    regmap = devm_regmap_init_mmio(dev, regs, &t9015_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(dev, "regmap init failed\n");
    return PTR_ERR(regmap);
    }
//
// Initialize output polarity:
// ATM the output polarity is fixed but in the future it might useful
// to add DT property to set this depending on the platform needs
//
    regmap_write(regmap, LINEOUT_CFG, 0x1111);
    return devm_snd_soc_register_component(dev, &t9015_codec_driver,
    &t9015_dai, 1);
    }
    static const struct of_device_id t9015_ids[] __maybe_unused = {
    { .compatible = "amlogic,t9015", },
    { }
    };
    MODULE_DEVICE_TABLE(of, t9015_ids);
    static struct platform_driver t9015_driver = {
    .driver = {
    .name = "t9015-codec",
    .of_match_table = of_match_ptr(t9015_ids),
    },
    .probe = t9015_probe,
    };
    module_platform_driver(t9015_driver);
    MODULE_DESCRIPTION("ASoC Amlogic T9015 codec driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL");
