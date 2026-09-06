//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ssm4567.c
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
// SSM4567 amplifier audio driver
//
// Copyright 2014 Google Chromium project.
// Author: Anatol Pomozov <anatol@chromium.org>
//
// Based on code copyright/by:
// Copyright 2013 Analog Devices Inc.
//

pub const SSM4567_REG_POWER_CTRL: c_uint = 0x00;
pub const SSM4567_REG_AMP_SNS_CTRL: c_uint = 0x01;
pub const SSM4567_REG_DAC_CTRL: c_uint = 0x02;
pub const SSM4567_REG_DAC_VOLUME: c_uint = 0x03;
pub const SSM4567_REG_SAI_CTRL_1: c_uint = 0x04;
pub const SSM4567_REG_SAI_CTRL_2: c_uint = 0x05;
pub const SSM4567_REG_SAI_PLACEMENT_1: c_uint = 0x06;
pub const SSM4567_REG_SAI_PLACEMENT_2: c_uint = 0x07;
pub const SSM4567_REG_SAI_PLACEMENT_3: c_uint = 0x08;
pub const SSM4567_REG_SAI_PLACEMENT_4: c_uint = 0x09;
pub const SSM4567_REG_SAI_PLACEMENT_5: c_uint = 0x0a;
pub const SSM4567_REG_SAI_PLACEMENT_6: c_uint = 0x0b;
pub const SSM4567_REG_BATTERY_V_OUT: c_uint = 0x0c;
pub const SSM4567_REG_LIMITER_CTRL_1: c_uint = 0x0d;
pub const SSM4567_REG_LIMITER_CTRL_2: c_uint = 0x0e;
pub const SSM4567_REG_LIMITER_CTRL_3: c_uint = 0x0f;
pub const SSM4567_REG_STATUS_1: c_uint = 0x10;
pub const SSM4567_REG_STATUS_2: c_uint = 0x11;
pub const SSM4567_REG_FAULT_CTRL: c_uint = 0x12;
pub const SSM4567_REG_PDM_CTRL: c_uint = 0x13;
pub const SSM4567_REG_MCLK_RATIO: c_uint = 0x14;
pub const SSM4567_REG_BOOST_CTRL_1: c_uint = 0x15;
pub const SSM4567_REG_BOOST_CTRL_2: c_uint = 0x16;
pub const SSM4567_REG_SOFT_RESET: c_uint = 0xff;
// POWER_CTRL

// DAC_CTRL

pub const SSM4567_DAC_FS_MASK: c_uint = 0x7;
pub const SSM4567_DAC_FS_8000_12000: c_uint = 0x0;
pub const SSM4567_DAC_FS_16000_24000: c_uint = 0x1;
pub const SSM4567_DAC_FS_32000_48000: c_uint = 0x2;
pub const SSM4567_DAC_FS_64000_96000: c_uint = 0x3;
pub const SSM4567_DAC_FS_128000_192000: c_uint = 0x4;
// SAI_CTRL_1

// SAI_CTRL_2

pub const SSM4567_SAI_CTRL_2_TDM_SLOT_MASK: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssm4567 {
    pub regmap: *mut regmap,
}

    static const struct reg_default ssm4567_reg_defaults[] = {
    { SSM4567_REG_POWER_CTRL,	0x81 },
    { SSM4567_REG_AMP_SNS_CTRL, 0x09 },
    { SSM4567_REG_DAC_CTRL, 0x32 },
    { SSM4567_REG_DAC_VOLUME, 0x40 },
    { SSM4567_REG_SAI_CTRL_1, 0x00 },
    { SSM4567_REG_SAI_CTRL_2, 0x08 },
    { SSM4567_REG_SAI_PLACEMENT_1, 0x01 },
    { SSM4567_REG_SAI_PLACEMENT_2, 0x20 },
    { SSM4567_REG_SAI_PLACEMENT_3, 0x32 },
    { SSM4567_REG_SAI_PLACEMENT_4, 0x07 },
    { SSM4567_REG_SAI_PLACEMENT_5, 0x07 },
    { SSM4567_REG_SAI_PLACEMENT_6, 0x07 },
    { SSM4567_REG_BATTERY_V_OUT, 0x00 },
    { SSM4567_REG_LIMITER_CTRL_1, 0xa4 },
    { SSM4567_REG_LIMITER_CTRL_2, 0x73 },
    { SSM4567_REG_LIMITER_CTRL_3, 0x00 },
    { SSM4567_REG_STATUS_1, 0x00 },
    { SSM4567_REG_STATUS_2, 0x00 },
    { SSM4567_REG_FAULT_CTRL, 0x30 },
    { SSM4567_REG_PDM_CTRL, 0x40 },
    { SSM4567_REG_MCLK_RATIO, 0x11 },
    { SSM4567_REG_BOOST_CTRL_1, 0x03 },
    { SSM4567_REG_BOOST_CTRL_2, 0x00 },
    { SSM4567_REG_SOFT_RESET, 0x00 },
    };
#[no_mangle]
unsafe extern "C" fn ssm4567_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ssm4567_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SSM4567_REG_POWER_CTRL ... SSM4567_REG_BOOST_CTRL_2:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn ssm4567_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ssm4567_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SSM4567_REG_POWER_CTRL ... SSM4567_REG_SAI_PLACEMENT_6:
    case SSM4567_REG_LIMITER_CTRL_1 ... SSM4567_REG_LIMITER_CTRL_3:
    case SSM4567_REG_FAULT_CTRL ... SSM4567_REG_BOOST_CTRL_2:
// The datasheet states that soft reset register is read-only,
// but logically it is write-only.
    case SSM4567_REG_SOFT_RESET:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn ssm4567_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ssm4567_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SSM4567_REG_BATTERY_V_OUT:
    case SSM4567_REG_STATUS_1 ... SSM4567_REG_STATUS_2:
    case SSM4567_REG_SOFT_RESET:
    return true;
    default:
    return false;
    }
    }
    static const DECLARE_TLV_DB_MINMAX_MUTE(ssm4567_vol_tlv, -7125, 2400);
    static const struct snd_kcontrol_new ssm4567_snd_controls[] = {
    SOC_SINGLE_TLV("Master Playback Volume", SSM4567_REG_DAC_VOLUME, 0,
    0xff, 1, ssm4567_vol_tlv),
    SOC_SINGLE("DAC Low Power Mode Switch", SSM4567_REG_DAC_CTRL, 4, 1, 0),
    SOC_SINGLE("DAC High Pass Filter Switch", SSM4567_REG_DAC_CTRL,
    5, 1, 0),
    };
    static const struct snd_kcontrol_new ssm4567_amplifier_boost_control =
    SOC_DAPM_SINGLE("Switch", SSM4567_REG_POWER_CTRL, 1, 1, 1);
    static const struct snd_soc_dapm_widget ssm4567_dapm_widgets[] = {
    SND_SOC_DAPM_DAC("DAC", "HiFi Playback", SSM4567_REG_POWER_CTRL, 2, 1),
    SND_SOC_DAPM_SWITCH("Amplifier Boost", SSM4567_REG_POWER_CTRL, 3, 1,
    &ssm4567_amplifier_boost_control),
    SND_SOC_DAPM_SIGGEN("Sense"),
    SND_SOC_DAPM_PGA("Current Sense", SSM4567_REG_POWER_CTRL, 4, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Voltage Sense", SSM4567_REG_POWER_CTRL, 5, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("VBAT Sense", SSM4567_REG_POWER_CTRL, 6, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("OUT"),
    };
    static const struct snd_soc_dapm_route ssm4567_routes[] = {
    { "OUT", core::ptr::null_mut(), "Amplifier Boost" },
    { "Amplifier Boost", "Switch", "DAC" },
    { "OUT", core::ptr::null_mut(), "DAC" },
    { "Current Sense", core::ptr::null_mut(), "Sense" },
    { "Voltage Sense", core::ptr::null_mut(), "Sense" },
    { "VBAT Sense", core::ptr::null_mut(), "Sense" },
    { "Capture Sense", core::ptr::null_mut(), "Current Sense" },
    { "Capture Sense", core::ptr::null_mut(), "Voltage Sense" },
    { "Capture Sense", core::ptr::null_mut(), "VBAT Sense" },
    };
    static int ssm4567_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ssm4567 *ssm4567 = snd_soc_component_get_drvdata(component);
    let mut rate: c_uint = params_rate(params);
    unsigned int dacfs;
    if (rate >= 8000 && rate <= 12000)
    dacfs = SSM4567_DAC_FS_8000_12000;
#[no_mangle]
pub unsafe extern "C" fn if(24000: rate >= 16000 && rate <=) -> else {
    else if (rate >= 16000 && rate <= 24000)
    dacfs = SSM4567_DAC_FS_16000_24000;
#[no_mangle]
pub unsafe extern "C" fn if(48000: rate >= 32000 && rate <=) -> else {
    else if (rate >= 32000 && rate <= 48000)
    dacfs = SSM4567_DAC_FS_32000_48000;
#[no_mangle]
pub unsafe extern "C" fn if(96000: rate >= 64000 && rate <=) -> else {
    else if (rate >= 64000 && rate <= 96000)
    dacfs = SSM4567_DAC_FS_64000_96000;
#[no_mangle]
pub unsafe extern "C" fn if(192000: rate >= 128000 && rate <=) -> else {
    else if (rate >= 128000 && rate <= 192000)
    dacfs = SSM4567_DAC_FS_128000_192000;
    else
    return -EINVAL;
    return regmap_update_bits(ssm4567.regmap, SSM4567_REG_DAC_CTRL,
    SSM4567_DAC_FS_MASK, dacfs);
    }
#[no_mangle]
unsafe extern "C" fn ssm4567_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    static int ssm4567_mute(struct snd_soc_dai *dai, int mute, int direction)
    {
    struct ssm4567 *ssm4567 = snd_soc_component_get_drvdata(dai.component);
    unsigned int val;
    val = mute ? SSM4567_DAC_MUTE : 0;
    return regmap_update_bits(ssm4567.regmap, SSM4567_REG_DAC_CTRL,
    SSM4567_DAC_MUTE, val);
    }
    static int ssm4567_set_tdm_slot(struct snd_soc_dai *dai, unsigned int tx_mask,
    unsigned int rx_mask, int slots, int width)
    {
    struct ssm4567 *ssm4567 = snd_soc_dai_get_drvdata(dai);
    unsigned int blcks;
    int slot;
    int ret;
    if (tx_mask == 0)
    return -EINVAL;
    if (rx_mask && rx_mask != tx_mask)
    return -EINVAL;
    slot = __ffs(tx_mask);
    if (tx_mask != BIT(slot))
    return -EINVAL;
    switch (width) {
    case 32:
    blcks = SSM4567_SAI_CTRL_1_TDM_BLCKS_32;
    break;
    case 48:
    blcks = SSM4567_SAI_CTRL_1_TDM_BLCKS_48;
    break;
    case 64:
    blcks = SSM4567_SAI_CTRL_1_TDM_BLCKS_64;
    break;
    default:
    return -EINVAL;
    }
    ret = regmap_update_bits(ssm4567.regmap, SSM4567_REG_SAI_CTRL_2,
    SSM4567_SAI_CTRL_2_AUTO_SLOT | SSM4567_SAI_CTRL_2_TDM_SLOT_MASK,
    SSM4567_SAI_CTRL_2_TDM_SLOT(slot));
    if (ret)
    return ret;
    return regmap_update_bits(ssm4567.regmap, SSM4567_REG_SAI_CTRL_1,
    SSM4567_SAI_CTRL_1_TDM_BLCKS_MASK, blcks);
    }
#[no_mangle]
unsafe extern "C" fn ssm4567_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int ssm4567_set_dai_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct ssm4567 *ssm4567 = snd_soc_dai_get_drvdata(dai);
    let mut ctrl1: c_uint = 0;
    bool invert_fclk;
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBC_CFC:
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    invert_fclk = false;
    break;
    case SND_SOC_DAIFMT_IB_NF:
    ctrl1 |= SSM4567_SAI_CTRL_1_BCLK;
    invert_fclk = false;
    break;
    case SND_SOC_DAIFMT_NB_IF:
    ctrl1 |= SSM4567_SAI_CTRL_1_FSYNC;
    invert_fclk = true;
    break;
    case SND_SOC_DAIFMT_IB_IF:
    ctrl1 |= SSM4567_SAI_CTRL_1_BCLK;
    invert_fclk = true;
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    ctrl1 |= SSM4567_SAI_CTRL_1_LJ;
    invert_fclk = !invert_fclk;
    break;
    case SND_SOC_DAIFMT_DSP_A:
    ctrl1 |= SSM4567_SAI_CTRL_1_TDM;
    break;
    case SND_SOC_DAIFMT_DSP_B:
    ctrl1 |= SSM4567_SAI_CTRL_1_TDM | SSM4567_SAI_CTRL_1_LJ;
    break;
    case SND_SOC_DAIFMT_PDM:
    ctrl1 |= SSM4567_SAI_CTRL_1_PDM;
    break;
    default:
    return -EINVAL;
    }
    if (invert_fclk)
    ctrl1 |= SSM4567_SAI_CTRL_1_FSYNC;
    return regmap_update_bits(ssm4567.regmap, SSM4567_REG_SAI_CTRL_1,
    SSM4567_SAI_CTRL_1_BCLK |
    SSM4567_SAI_CTRL_1_FSYNC |
    SSM4567_SAI_CTRL_1_LJ |
    SSM4567_SAI_CTRL_1_TDM |
    SSM4567_SAI_CTRL_1_PDM,
    ctrl1);
    }
#[no_mangle]
unsafe extern "C" fn ssm4567_set_power(ssm4567: *mut ssm4567, enable: bool) -> c_int {
    static int ssm4567_set_power(struct ssm4567 *ssm4567, bool enable)
    {
    let mut ret: c_int = 0;
    if (!enable) {
    ret = regmap_update_bits(ssm4567.regmap,
    SSM4567_REG_POWER_CTRL,
    SSM4567_POWER_SPWDN, SSM4567_POWER_SPWDN);
    regcache_mark_dirty(ssm4567.regmap);
    }
    regcache_cache_only(ssm4567.regmap, !enable);
    if (enable) {
    ret = regmap_write(ssm4567.regmap, SSM4567_REG_SOFT_RESET,
    0x00);
    if (ret)
    return ret;
    ret = regmap_update_bits(ssm4567.regmap,
    SSM4567_REG_POWER_CTRL,
    SSM4567_POWER_SPWDN, 0x00);
    regcache_sync(ssm4567.regmap);
    }
    return ret;
    }
    static int ssm4567_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    struct ssm4567 *ssm4567 = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;
    switch (level) {
    case SND_SOC_BIAS_ON:
    break;
    case SND_SOC_BIAS_PREPARE:
    break;
    case SND_SOC_BIAS_STANDBY:
    if (snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF)
    ret = ssm4567_set_power(ssm4567, true);
    break;
    case SND_SOC_BIAS_OFF:
    ret = ssm4567_set_power(ssm4567, false);
    break;
    }
    return ret;
    }
    static const struct snd_soc_dai_ops ssm4567_dai_ops = {
    .hw_params	= ssm4567_hw_params,
    .mute_stream	= ssm4567_mute,
    .set_fmt	= ssm4567_set_dai_fmt,
    .set_tdm_slot	= ssm4567_set_tdm_slot,
    .no_capture_mute = 1,
    };
    static struct snd_soc_dai_driver ssm4567_dai = {
    .name = "ssm4567-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32,
    },
    .capture = {
    .stream_name = "Capture Sense",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32,
    },
    .ops = &ssm4567_dai_ops,
    };
    static const struct snd_soc_component_driver ssm4567_component_driver = {
    .set_bias_level		= ssm4567_set_bias_level,
    .controls		= ssm4567_snd_controls,
    .num_controls		= ARRAY_SIZE(ssm4567_snd_controls),
    .dapm_widgets		= ssm4567_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ssm4567_dapm_widgets),
    .dapm_routes		= ssm4567_routes,
    .num_dapm_routes	= ARRAY_SIZE(ssm4567_routes),
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct regmap_config ssm4567_regmap_config = {
    .val_bits = 8,
    .reg_bits = 8,
    .max_register = SSM4567_REG_SOFT_RESET,
    .readable_reg = ssm4567_readable_reg,
    .writeable_reg = ssm4567_writeable_reg,
    .volatile_reg = ssm4567_volatile_reg,
    .cache_type = REGCACHE_RBTREE,
    .reg_defaults = ssm4567_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(ssm4567_reg_defaults),
    };
#[no_mangle]
unsafe extern "C" fn ssm4567_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ssm4567_i2c_probe(struct i2c_client *i2c)
    {
    struct ssm4567 *ssm4567;
    int ret;
    ssm4567 = devm_kzalloc(&i2c.dev, sizeof(*ssm4567), GFP_KERNEL);
    if (ssm4567 == core::ptr::null_mut())
    return -ENOMEM;
    i2c_set_clientdata(i2c, ssm4567);
    ssm4567.regmap = devm_regmap_init_i2c(i2c, &ssm4567_regmap_config);
    if (IS_ERR(ssm4567.regmap))
    return PTR_ERR(ssm4567.regmap);
    ret = regmap_write(ssm4567.regmap, SSM4567_REG_SOFT_RESET, 0x00);
    if (ret)
    return ret;
    ret = ssm4567_set_power(ssm4567, false);
    if (ret)
    return ret;
    return devm_snd_soc_register_component(&i2c.dev, &ssm4567_component_driver,
    &ssm4567_dai, 1);
    }
    static const struct i2c_device_id ssm4567_i2c_ids[] = {
    { .name = "ssm4567" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ssm4567_i2c_ids);

    static const struct of_device_id ssm4567_of_match[] = {
    { .compatible = "adi,ssm4567", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ssm4567_of_match);

    static const struct acpi_device_id ssm4567_acpi_match[] = {
    { "INT343B", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, ssm4567_acpi_match);

    static struct i2c_driver ssm4567_driver = {
    .driver = {
    .name = "ssm4567",
    .of_match_table = of_match_ptr(ssm4567_of_match),
    .acpi_match_table = ACPI_PTR(ssm4567_acpi_match),
    },
    .probe = ssm4567_i2c_probe,
    .id_table = ssm4567_i2c_ids,
    };
    module_i2c_driver(ssm4567_driver);
    MODULE_DESCRIPTION("ASoC SSM4567 driver");
    MODULE_AUTHOR("Anatol Pomozov <anatol@chromium.org>");
    MODULE_LICENSE("GPL");
