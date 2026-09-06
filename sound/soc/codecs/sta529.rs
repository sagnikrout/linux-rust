//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/sta529.c
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
// ASoC codec driver for spear platform
//
// sound/soc/codecs/sta529.c -- spear ALSA Soc codec driver
//
// Copyright (C) 2012 ST Microelectronics
// Rajeev Kumar <rajeevkumar.linux@gmail.com>
//

// STA529 Register offsets
pub const STA529_FFXCFG0: c_uint = 0x00;
pub const STA529_FFXCFG1: c_uint = 0x01;
pub const STA529_MVOL: c_uint = 0x02;
pub const STA529_LVOL: c_uint = 0x03;
pub const STA529_RVOL: c_uint = 0x04;
pub const STA529_TTF0: c_uint = 0x05;
pub const STA529_TTF1: c_uint = 0x06;
pub const STA529_TTP0: c_uint = 0x07;
pub const STA529_TTP1: c_uint = 0x08;
pub const STA529_S2PCFG0: c_uint = 0x0A;
pub const STA529_S2PCFG1: c_uint = 0x0B;
pub const STA529_P2SCFG0: c_uint = 0x0C;
pub const STA529_P2SCFG1: c_uint = 0x0D;
pub const STA529_PLLCFG0: c_uint = 0x14;
pub const STA529_PLLCFG1: c_uint = 0x15;
pub const STA529_PLLCFG2: c_uint = 0x16;
pub const STA529_PLLCFG3: c_uint = 0x17;
pub const STA529_PLLPFE: c_uint = 0x18;
pub const STA529_PLLST: c_uint = 0x19;
pub const STA529_ADCCFG: c_uint = 0x1E /*mic_select*/;
pub const STA529_CKOCFG: c_uint = 0x1F;
pub const STA529_MISC: c_uint = 0x20;
pub const STA529_PADST0: c_uint = 0x21;
pub const STA529_PADST1: c_uint = 0x22;
pub const STA529_FFXST: c_uint = 0x23;
pub const STA529_PWMIN1: c_uint = 0x2D;
pub const STA529_PWMIN2: c_uint = 0x2E;
pub const STA529_POWST: c_uint = 0x32;
pub const STA529_MAX_REGISTER: c_uint = 0x32;

    SNDRV_PCM_RATE_11025 | \
    SNDRV_PCM_RATE_16000 | \
    SNDRV_PCM_RATE_22050 | \
    SNDRV_PCM_RATE_32000 | \
    SNDRV_PCM_RATE_44100 | \
    SNDRV_PCM_RATE_48000)

    SNDRV_PCM_FMTBIT_S24_LE | \
    SNDRV_PCM_FMTBIT_S32_LE)
pub const S2PC_VALUE: c_uint = 0x98;
pub const CLOCK_OUT: c_uint = 0x60;
pub const DATA_FORMAT_MSK: c_uint = 0x0E;
pub const LEFT_J_DATA_FORMAT: c_uint = 0x00;
pub const I2S_DATA_FORMAT: c_uint = 0x02;
pub const RIGHT_J_DATA_FORMAT: c_uint = 0x04;
pub const CODEC_MUTE_VAL: c_uint = 0x80;
pub const POWER_CNTLMSAK: c_uint = 0x40;
pub const POWER_STDBY: c_uint = 0x40;
pub const FFX_MASK: c_uint = 0x80;
pub const FFX_OFF: c_uint = 0x80;
pub const POWER_UP: c_uint = 0x00;
pub const FFX_CLK_ENB: c_uint = 0x01;
pub const FFX_CLK_DIS: c_uint = 0x00;
pub const FFX_CLK_MSK: c_uint = 0x01;
pub const PLAY_FREQ_RANGE_MSK: c_uint = 0x70;
pub const CAP_FREQ_RANGE_MSK: c_uint = 0x0C;
pub const PDATA_LEN_MSK: c_uint = 0xC0;
pub const BCLK_TO_FS_MSK: c_uint = 0x30;
pub const AUDIO_MUTE_MSK: c_uint = 0x80;
    static const struct reg_default sta529_reg_defaults[] = {
    { 0,  0x35 },     /* R0   - FFX Configuration reg 0 */
    { 1,  0xc8 },     /* R1   - FFX Configuration reg 1 */
    { 2,  0x50 },     /* R2   - Master Volume */
    { 3,  0x00 },     /* R3   - Left Volume */
    { 4,  0x00 },     /* R4  -  Right Volume */
    { 10, 0xb2 },     /* R10  - S2P Config Reg 0 */
    { 11, 0x41 },     /* R11  - S2P Config Reg 1 */
    { 12, 0x92 },     /* R12  - P2S Config Reg 0 */
    { 13, 0x41 },     /* R13  - P2S Config Reg 1 */
    { 30, 0xd2 },     /* R30  - ADC Config Reg */
    { 31, 0x40 },     /* R31  - clock Out Reg */
    { 32, 0x21 },     /* R32  - Misc Register */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta529 {
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn sta529_readable(dev: *mut device, reg: c_uint) -> bool {
    static bool sta529_readable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STA529_FFXCFG0:
    case STA529_FFXCFG1:
    case STA529_MVOL:
    case STA529_LVOL:
    case STA529_RVOL:
    case STA529_S2PCFG0:
    case STA529_S2PCFG1:
    case STA529_P2SCFG0:
    case STA529_P2SCFG1:
    case STA529_ADCCFG:
    case STA529_CKOCFG:
    case STA529_MISC:
    return true;
    default:
    return false;
    }
    }
    static const char *pwm_mode_text[] = { "Binary", "Headphone", "Ternary",
    "Phase-shift"};
    static const DECLARE_TLV_DB_SCALE(out_gain_tlv, -9150, 50, 0);
    static const DECLARE_TLV_DB_SCALE(master_vol_tlv, -12750, 50, 0);
    static SOC_ENUM_SINGLE_DECL(pwm_src, STA529_FFXCFG1, 4, pwm_mode_text);
    static const struct snd_kcontrol_new sta529_snd_controls[] = {
    SOC_DOUBLE_R_TLV("Digital Playback Volume", STA529_LVOL, STA529_RVOL, 0,
    127, 0, out_gain_tlv),
    SOC_SINGLE_TLV("Master Playback Volume", STA529_MVOL, 0, 127, 1,
    master_vol_tlv),
    SOC_ENUM("PWM Select", pwm_src),
    };
    static int sta529_set_bias_level(struct snd_soc_component *component, enum
    snd_soc_bias_level level)
    {
    struct sta529 *sta529 = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    switch (level) {
    case SND_SOC_BIAS_ON:
    case SND_SOC_BIAS_PREPARE:
    snd_soc_component_update_bits(component, STA529_FFXCFG0, POWER_CNTLMSAK,
    POWER_UP);
    snd_soc_component_update_bits(component, STA529_MISC,	FFX_CLK_MSK,
    FFX_CLK_ENB);
    break;
    case SND_SOC_BIAS_STANDBY:
    if (snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF)
    regcache_sync(sta529.regmap);
    snd_soc_component_update_bits(component, STA529_FFXCFG0,
    POWER_CNTLMSAK, POWER_STDBY);
// Making FFX output to zero
    snd_soc_component_update_bits(component, STA529_FFXCFG0, FFX_MASK,
    FFX_OFF);
    snd_soc_component_update_bits(component, STA529_MISC, FFX_CLK_MSK,
    FFX_CLK_DIS);
    break;
    case SND_SOC_BIAS_OFF:
    break;
    }
    return 0;
    }
    static int sta529_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    int pdata, play_freq_val, record_freq_val;
    int bclk_to_fs_ratio;
    switch (params_width(params)) {
    case 16:
    pdata = 1;
    bclk_to_fs_ratio = 0;
    break;
    case 24:
    pdata = 2;
    bclk_to_fs_ratio = 1;
    break;
    case 32:
    pdata = 3;
    bclk_to_fs_ratio = 2;
    break;
    default:
    dev_err(component.dev, "Unsupported format\n");
    return -EINVAL;
    }
    switch (params_rate(params)) {
    case 8000:
    case 11025:
    play_freq_val = 0;
    record_freq_val = 2;
    break;
    case 16000:
    case 22050:
    play_freq_val = 1;
    record_freq_val = 0;
    break;
    case 32000:
    case 44100:
    case 48000:
    play_freq_val = 2;
    record_freq_val = 0;
    break;
    default:
    dev_err(component.dev, "Unsupported rate\n");
    return -EINVAL;
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    snd_soc_component_update_bits(component, STA529_S2PCFG1, PDATA_LEN_MSK,
    pdata << 6);
    snd_soc_component_update_bits(component, STA529_S2PCFG1, BCLK_TO_FS_MSK,
    bclk_to_fs_ratio << 4);
    snd_soc_component_update_bits(component, STA529_MISC, PLAY_FREQ_RANGE_MSK,
    play_freq_val << 4);
    } else {
    snd_soc_component_update_bits(component, STA529_P2SCFG1, PDATA_LEN_MSK,
    pdata << 6);
    snd_soc_component_update_bits(component, STA529_P2SCFG1, BCLK_TO_FS_MSK,
    bclk_to_fs_ratio << 4);
    snd_soc_component_update_bits(component, STA529_MISC, CAP_FREQ_RANGE_MSK,
    record_freq_val << 2);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sta529_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    static int sta529_mute(struct snd_soc_dai *dai, int mute, int direction)
    {
    let mut val: u8 = 0;
    if (mute)
    val |= CODEC_MUTE_VAL;
    snd_soc_component_update_bits(dai.component, STA529_FFXCFG0, AUDIO_MUTE_MSK, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sta529_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    static int sta529_set_dai_fmt(struct snd_soc_dai *codec_dai, u32 fmt)
    {
    struct snd_soc_component *component = codec_dai.component;
    let mut mode: u8 = 0;
// interface format
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_LEFT_J:
    mode = LEFT_J_DATA_FORMAT;
    break;
    case SND_SOC_DAIFMT_I2S:
    mode = I2S_DATA_FORMAT;
    break;
    case SND_SOC_DAIFMT_RIGHT_J:
    mode = RIGHT_J_DATA_FORMAT;
    break;
    default:
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, STA529_S2PCFG0, DATA_FORMAT_MSK, mode);
    return 0;
    }
    static const struct snd_soc_dai_ops sta529_dai_ops = {
    .hw_params	=	sta529_hw_params,
    .set_fmt	=	sta529_set_dai_fmt,
    .mute_stream	=	sta529_mute,
    .no_capture_mute = 1,
    };
    static struct snd_soc_dai_driver sta529_dai = {
    .name = "sta529-audio",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = STA529_RATES,
    .formats = STA529_FORMAT,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = STA529_RATES,
    .formats = STA529_FORMAT,
    },
    .ops	= &sta529_dai_ops,
    };
    static const struct snd_soc_component_driver sta529_component_driver = {
    .set_bias_level		= sta529_set_bias_level,
    .controls		= sta529_snd_controls,
    .num_controls		= ARRAY_SIZE(sta529_snd_controls),
    .suspend_bias_off	= 1,
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct regmap_config sta529_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = STA529_MAX_REGISTER,
    .readable_reg = sta529_readable,
    .cache_type = REGCACHE_MAPLE,
    .reg_defaults = sta529_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(sta529_reg_defaults),
    };
#[no_mangle]
unsafe extern "C" fn sta529_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int sta529_i2c_probe(struct i2c_client *i2c)
    {
    struct sta529 *sta529;
    int ret;
    sta529 = devm_kzalloc(&i2c.dev, sizeof(struct sta529), GFP_KERNEL);
    if (!sta529)
    return -ENOMEM;
    sta529.regmap = devm_regmap_init_i2c(i2c, &sta529_regmap);
    if (IS_ERR(sta529.regmap)) {
    ret = PTR_ERR(sta529.regmap);
    dev_err(&i2c.dev, "Failed to allocate regmap: %d\n", ret);
    return ret;
    }
    i2c_set_clientdata(i2c, sta529);
    ret = devm_snd_soc_register_component(&i2c.dev,
    &sta529_component_driver, &sta529_dai, 1);
    if (ret != 0)
    dev_err(&i2c.dev, "Failed to register CODEC: %d\n", ret);
    return ret;
    }
    static const struct i2c_device_id sta529_i2c_id[] = {
    { .name = "sta529" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sta529_i2c_id);
    static const struct of_device_id sta529_of_match[] = {
    { .compatible = "st,sta529", },
    { }
    };
    MODULE_DEVICE_TABLE(of, sta529_of_match);
    static struct i2c_driver sta529_i2c_driver = {
    .driver = {
    .name = "sta529",
    .of_match_table = sta529_of_match,
    },
    .probe		= sta529_i2c_probe,
    .id_table	= sta529_i2c_id,
    };
    module_i2c_driver(sta529_i2c_driver);
    MODULE_DESCRIPTION("ASoC STA529 codec driver");
    MODULE_AUTHOR("Rajeev Kumar <rajeevkumar.linux@gmail.com>");
    MODULE_LICENSE("GPL");
