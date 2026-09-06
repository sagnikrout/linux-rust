//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm1681.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PCM1681 ASoC codec driver
//
// Copyright (c) StreamUnlimited GmbH 2013
// Marek Belisko <marek.belisko@streamunlimited.com>
//

    SNDRV_PCM_FMTBIT_S24_LE)

    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100  | \
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200  | \
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000)
pub const PCM1681_SOFT_MUTE_ALL: c_uint = 0xff;
pub const PCM1681_DEEMPH_RATE_MASK: c_uint = 0x18;
pub const PCM1681_DEEMPH_MASK: c_uint = 0x01;

pub const PCM1681_SOFT_MUTE: c_uint = 0x07	/* Soft mute control register */;
pub const PCM1681_DAC_CONTROL: c_uint = 0x08	/* DAC operation control */;
pub const PCM1681_FMT_CONTROL: c_uint = 0x09	/* Audio interface data format */;
pub const PCM1681_DEEMPH_CONTROL: c_uint = 0x0a	/* De-emphasis control */;
pub const PCM1681_ZERO_DETECT_STATUS: c_uint = 0x0e	/* Zero detect status reg */;
    static const struct reg_default pcm1681_reg_defaults[] = {
    { 0x01,	0xff },
    { 0x02,	0xff },
    { 0x03,	0xff },
    { 0x04,	0xff },
    { 0x05,	0xff },
    { 0x06,	0xff },
    { 0x07,	0x00 },
    { 0x08,	0x00 },
    { 0x09,	0x06 },
    { 0x0A,	0x00 },
    { 0x0B,	0xff },
    { 0x0C,	0x0f },
    { 0x0D,	0x00 },
    { 0x10,	0xff },
    { 0x11,	0xff },
    { 0x12,	0x00 },
    { 0x13,	0x00 },
    };
#[no_mangle]
unsafe extern "C" fn pcm1681_accessible_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool pcm1681_accessible_reg(struct device *dev, unsigned int reg)
    {
    return !((reg == 0x00) || (reg == 0x0f));
    }
#[no_mangle]
unsafe extern "C" fn pcm1681_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool pcm1681_writeable_reg(struct device *dev, unsigned int reg)
    {
    return pcm1681_accessible_reg(dev, reg) &&
    (reg != PCM1681_ZERO_DETECT_STATUS);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm1681_private {
    pub regmap: *mut regmap,
    pub format: c_uint,
// Current deemphasis status
    pub deemph: c_uint,
// Current rate for deemphasis control
    pub rate: c_uint,
}

    static const int pcm1681_deemph[] = { 44100, 48000, 32000 };
#[no_mangle]
unsafe extern "C" fn pcm1681_set_deemph(component: *mut snd_soc_component) -> c_int {
    static int pcm1681_set_deemph(struct snd_soc_component *component)
    {
    struct pcm1681_private *priv = snd_soc_component_get_drvdata(component);
    int i, val = -1, enable = 0;
    if (priv.deemph) {
    for (i = 0; i < ARRAY_SIZE(pcm1681_deemph); i++) {
    if (pcm1681_deemph[i] == priv.rate) {
    val = i;
    break;
    }
    }
    }
    if (val != -1) {
    regmap_update_bits(priv.regmap, PCM1681_DEEMPH_CONTROL,
    PCM1681_DEEMPH_RATE_MASK, val << 3);
    enable = 1;
    } else {
    enable = 0;
    }
// enable/disable deemphasis functionality
    return regmap_update_bits(priv.regmap, PCM1681_DEEMPH_CONTROL,
    PCM1681_DEEMPH_MASK, enable);
    }
    static int pcm1681_get_deemph(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct pcm1681_private *priv = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = priv.deemph;
    return 0;
    }
    static int pcm1681_put_deemph(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct pcm1681_private *priv = snd_soc_component_get_drvdata(component);
    priv.deemph = ucontrol.value.integer.value[0];
    return pcm1681_set_deemph(component);
    }
    static int pcm1681_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int format)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct pcm1681_private *priv = snd_soc_component_get_drvdata(component);
// The PCM1681 can only be consumer to all clocks
    if ((format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC) {
    dev_err(component.dev, "Invalid clocking mode\n");
    return -EINVAL;
    }
    priv.format = format;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm1681_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    static int pcm1681_mute(struct snd_soc_dai *dai, int mute, int direction)
    {
    struct snd_soc_component *component = dai.component;
    struct pcm1681_private *priv = snd_soc_component_get_drvdata(component);
    int val;
    if (mute)
    val = PCM1681_SOFT_MUTE_ALL;
    else
    val = 0;
    return regmap_write(priv.regmap, PCM1681_SOFT_MUTE, val);
    }
    static int pcm1681_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct pcm1681_private *priv = snd_soc_component_get_drvdata(component);
    let mut val: c_int = 0, ret;
    priv.rate = params_rate(params);
    switch (priv.format & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_RIGHT_J:
    switch (params_width(params)) {
    case 24:
    val = 0;
    break;
    case 16:
    val = 3;
    break;
    default:
    return -EINVAL;
    }
    break;
    case SND_SOC_DAIFMT_I2S:
    val = 0x04;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    val = 0x05;
    break;
    default:
    dev_err(component.dev, "Invalid DAI format\n");
    return -EINVAL;
    }
    ret = regmap_update_bits(priv.regmap, PCM1681_FMT_CONTROL, 0x0f, val);
    if (ret < 0)
    return ret;
    return pcm1681_set_deemph(component);
    }
    static const u64 pcm1681_selectable_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J;
    static const struct snd_soc_dai_ops pcm1681_dai_ops = {
    .set_fmt	= pcm1681_set_dai_fmt,
    .hw_params	= pcm1681_hw_params,
    .mute_stream	= pcm1681_mute,
    .auto_selectable_formats	= &pcm1681_selectable_formats,
    .num_auto_selectable_formats	= 1,
    .no_capture_mute = 1,
    };
    static const struct snd_soc_dapm_widget pcm1681_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("VOUT1"),
    SND_SOC_DAPM_OUTPUT("VOUT2"),
    SND_SOC_DAPM_OUTPUT("VOUT3"),
    SND_SOC_DAPM_OUTPUT("VOUT4"),
    SND_SOC_DAPM_OUTPUT("VOUT5"),
    SND_SOC_DAPM_OUTPUT("VOUT6"),
    SND_SOC_DAPM_OUTPUT("VOUT7"),
    SND_SOC_DAPM_OUTPUT("VOUT8"),
    };
    static const struct snd_soc_dapm_route pcm1681_dapm_routes[] = {
    { "VOUT1", core::ptr::null_mut(), "Playback" },
    { "VOUT2", core::ptr::null_mut(), "Playback" },
    { "VOUT3", core::ptr::null_mut(), "Playback" },
    { "VOUT4", core::ptr::null_mut(), "Playback" },
    { "VOUT5", core::ptr::null_mut(), "Playback" },
    { "VOUT6", core::ptr::null_mut(), "Playback" },
    { "VOUT7", core::ptr::null_mut(), "Playback" },
    { "VOUT8", core::ptr::null_mut(), "Playback" },
    };
    static const DECLARE_TLV_DB_SCALE(pcm1681_dac_tlv, -6350, 50, 1);
    static const struct snd_kcontrol_new pcm1681_controls[] = {
    SOC_DOUBLE_R_TLV("Channel 1/2 Playback Volume",
    PCM1681_ATT_CONTROL(1), PCM1681_ATT_CONTROL(2), 0,
    0x7f, 0, pcm1681_dac_tlv),
    SOC_DOUBLE_R_TLV("Channel 3/4 Playback Volume",
    PCM1681_ATT_CONTROL(3), PCM1681_ATT_CONTROL(4), 0,
    0x7f, 0, pcm1681_dac_tlv),
    SOC_DOUBLE_R_TLV("Channel 5/6 Playback Volume",
    PCM1681_ATT_CONTROL(5), PCM1681_ATT_CONTROL(6), 0,
    0x7f, 0, pcm1681_dac_tlv),
    SOC_DOUBLE_R_TLV("Channel 7/8 Playback Volume",
    PCM1681_ATT_CONTROL(7), PCM1681_ATT_CONTROL(8), 0,
    0x7f, 0, pcm1681_dac_tlv),
    SOC_SINGLE_BOOL_EXT("De-emphasis Switch", 0,
    pcm1681_get_deemph, pcm1681_put_deemph),
    };
    static struct snd_soc_dai_driver pcm1681_dai = {
    .name = "pcm1681-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 8,
    .rates = PCM1681_PCM_RATES,
    .formats = PCM1681_PCM_FORMATS,
    },
    .ops = &pcm1681_dai_ops,
    };

    static const struct of_device_id pcm1681_dt_ids[] = {
    { .compatible = "ti,pcm1681", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm1681_dt_ids);

    static const struct regmap_config pcm1681_regmap = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= 0x13,
    .reg_defaults		= pcm1681_reg_defaults,
    .num_reg_defaults	= ARRAY_SIZE(pcm1681_reg_defaults),
    .writeable_reg		= pcm1681_writeable_reg,
    .readable_reg		= pcm1681_accessible_reg,
    };
    static const struct snd_soc_component_driver soc_component_dev_pcm1681 = {
    .controls		= pcm1681_controls,
    .num_controls		= ARRAY_SIZE(pcm1681_controls),
    .dapm_widgets		= pcm1681_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(pcm1681_dapm_widgets),
    .dapm_routes		= pcm1681_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(pcm1681_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct i2c_device_id pcm1681_i2c_id[] = {
    { .name = "pcm1681" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcm1681_i2c_id);
#[no_mangle]
unsafe extern "C" fn pcm1681_i2c_probe(client: *mut i2c_client) -> c_int {
    static int pcm1681_i2c_probe(struct i2c_client *client)
    {
    int ret;
    struct pcm1681_private *priv;
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = devm_regmap_init_i2c(client, &pcm1681_regmap);
    if (IS_ERR(priv.regmap)) {
    ret = PTR_ERR(priv.regmap);
    dev_err(&client.dev, "Failed to create regmap: %d\n", ret);
    return ret;
    }
    i2c_set_clientdata(client, priv);
    return devm_snd_soc_register_component(&client.dev,
    &soc_component_dev_pcm1681,
    &pcm1681_dai, 1);
    }
    static struct i2c_driver pcm1681_i2c_driver = {
    .driver = {
    .name	= "pcm1681",
    .of_match_table = of_match_ptr(pcm1681_dt_ids),
    },
    .id_table	= pcm1681_i2c_id,
    .probe		= pcm1681_i2c_probe,
    };
    module_i2c_driver(pcm1681_i2c_driver);
    MODULE_DESCRIPTION("Texas Instruments PCM1681 ALSA SoC Codec Driver");
    MODULE_AUTHOR("Marek Belisko <marek.belisko@streamunlimited.com>");
    MODULE_LICENSE("GPL");
