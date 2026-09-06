//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ntp8918.c
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
// Driver for the NTP8918 Audio Amplifier
//
// Copyright (c) 2024, SaluteDevices. All Rights Reserved.
//
// Author: Igor Prusov <ivprusov@salutedevices.com>
//

    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000)

    SNDRV_PCM_FMTBIT_S20_3LE | \
    SNDRV_PCM_FMTBIT_S24_LE | \
    SNDRV_PCM_FMTBIT_S32_LE)
pub const NTP8918_INPUT_FMT: c_uint = 0x0;

pub const NTP8918_GSA_FMT: c_uint = 0x1;

pub const NTP8918_MCLK_FREQ_CTRL: c_uint = 0x2;

pub const NTP8918_MASTER_VOL: c_uint = 0x0C;
pub const NTP8918_CHNL_A_VOL: c_uint = 0x17;
pub const NTP8918_CHNL_B_VOL: c_uint = 0x18;
pub const NTP8918_SOFT_MUTE: c_uint = 0x33;

pub const NTP8918_PWM_SWITCH: c_uint = 0x34;
pub const NTP8918_PWM_MASK_CTRL0: c_uint = 0x35;

pub const NTP8918_FW_MAGIC: c_uint = 0x38393138	/* "8918" */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntp8918_priv {
    pub i2c: *mut i2c_client,
    pub bck: *mut clk,
    pub reset: *mut reset_control,
    pub format: c_uint,
}

    static const DECLARE_TLV_DB_SCALE(ntp8918_master_vol_scale, -12550, 50, 0);
    static const struct snd_kcontrol_new ntp8918_vol_control[] = {
    SOC_SINGLE_RANGE_TLV("Playback Volume", NTP8918_MASTER_VOL, 0,
    0x04, 0xff, 0, ntp8918_master_vol_scale),
    SOC_SINGLE("Playback Switch", NTP8918_PWM_MASK_CTRL0, 1, 1, 1),
    };
#[no_mangle]
unsafe extern "C" fn ntp8918_reset_gpio(ntp8918: *mut ntp8918_priv) {
    static void ntp8918_reset_gpio(struct ntp8918_priv *ntp8918)
    {
//
// Proper initialization sequence for NTP8918 amplifier requires driving
// /RESET signal low during power up for at least 0.1us. The sequence is,
// according to NTP8918 datasheet, 6.2 Timing Sequence 1:
// Deassert for T2 >= 1ms...
//
    reset_control_deassert(ntp8918.reset);
    fsleep(1000);
// ...Assert for T3 >= 0.1us...
    reset_control_assert(ntp8918.reset);
    fsleep(1);
// ...Deassert, and wait for T4 >= 0.5ms before sound on sequence.
    reset_control_deassert(ntp8918.reset);
    fsleep(500);
    }
    static const struct reg_sequence ntp8918_sound_off[] = {
    { NTP8918_MASTER_VOL, 0 },
    };
    static const struct reg_sequence ntp8918_sound_on[] = {
    { NTP8918_MASTER_VOL, 0b11 },
    };
#[no_mangle]
unsafe extern "C" fn ntp8918_load_firmware(ntp8918: *mut ntp8918_priv) -> c_int {
    static int ntp8918_load_firmware(struct ntp8918_priv *ntp8918)
    {
    int ret;
    ret = ntpfw_load(ntp8918.i2c, NTP8918_FW_NAME, NTP8918_FW_MAGIC);
    if (ret == -ENOENT) {
    dev_warn_once(&ntp8918.i2c.dev, "Could not find firmware %s\n",
    NTP8918_FW_NAME);
    return 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntp8918_snd_suspend(component: *mut snd_soc_component) -> c_int {
    static int ntp8918_snd_suspend(struct snd_soc_component *component)
    {
    struct ntp8918_priv *ntp8918 = snd_soc_component_get_drvdata(component);
    regcache_cache_only(component.regmap, true);
    regmap_multi_reg_write_bypassed(component.regmap,
    ntp8918_sound_off,
    ARRAY_SIZE(ntp8918_sound_off));
//
// According to NTP8918 datasheet, 6.2 Timing Sequence 1:
// wait after sound off for T6 >= 0.5ms
//
    fsleep(500);
    reset_control_assert(ntp8918.reset);
    regcache_mark_dirty(component.regmap);
    clk_disable_unprepare(ntp8918.bck);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntp8918_snd_resume(component: *mut snd_soc_component) -> c_int {
    static int ntp8918_snd_resume(struct snd_soc_component *component)
    {
    struct ntp8918_priv *ntp8918 = snd_soc_component_get_drvdata(component);
    int ret;
    ret = clk_prepare_enable(ntp8918.bck);
    if (ret)
    return ret;
    ntp8918_reset_gpio(ntp8918);
    regmap_multi_reg_write_bypassed(component.regmap,
    ntp8918_sound_on,
    ARRAY_SIZE(ntp8918_sound_on));
    ret = ntp8918_load_firmware(ntp8918);
    if (ret) {
    dev_err(&ntp8918.i2c.dev, "Failed to load firmware\n");
    return ret;
    }
    regcache_cache_only(component.regmap, false);
    snd_soc_component_cache_sync(component);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntp8918_probe(component: *mut snd_soc_component) -> c_int {
    static int ntp8918_probe(struct snd_soc_component *component)
    {
    int ret;
    struct ntp8918_priv *ntp8918 = snd_soc_component_get_drvdata(component);
    struct device *dev = component.dev;
    ret = snd_soc_add_component_controls(component, ntp8918_vol_control,
    ARRAY_SIZE(ntp8918_vol_control));
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add controls\n");
    ret = ntp8918_load_firmware(ntp8918);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to load firmware\n");
    return 0;
    }
    static const struct snd_soc_dapm_widget ntp8918_dapm_widgets[] = {
    SND_SOC_DAPM_DAC("AIFIN", "Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT("OUT1"),
    SND_SOC_DAPM_OUTPUT("OUT2"),
    };
    static const struct snd_soc_dapm_route ntp8918_dapm_routes[] = {
    { "OUT1", core::ptr::null_mut(), "AIFIN" },
    { "OUT2", core::ptr::null_mut(), "AIFIN" },
    };
    static const struct snd_soc_component_driver soc_component_ntp8918 = {
    .probe = ntp8918_probe,
    .suspend = ntp8918_snd_suspend,
    .resume = ntp8918_snd_resume,
    .dapm_widgets = ntp8918_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(ntp8918_dapm_widgets),
    .dapm_routes = ntp8918_dapm_routes,
    .num_dapm_routes = ARRAY_SIZE(ntp8918_dapm_routes),
    };
    static int ntp8918_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ntp8918_priv *ntp8918 = snd_soc_component_get_drvdata(component);
    let mut input_fmt: c_uint = 0;
    let mut gsa_fmt: c_uint = 0;
    unsigned int gsa_fmt_mask;
    unsigned int mcf;
    int bclk;
    int ret;
    bclk = snd_soc_params_to_bclk(params);
    switch (bclk) {
    case 3072000:
    case 2822400:
    mcf = 0;
    break;
    case 6144000:
    mcf = 1;
    break;
    case 2048000:
    mcf = 2;
    break;
    default:
    return -EINVAL;
    }
    ret = snd_soc_component_update_bits(component, NTP8918_MCLK_FREQ_CTRL,
    NTP8918_MCLK_FREQ_MCF, mcf);
    if (ret)
    return ret;
    switch (ntp8918.format) {
    case SND_SOC_DAIFMT_I2S:
    break;
    case SND_SOC_DAIFMT_RIGHT_J:
    input_fmt |= NTP8918_INPUT_FMT_GSA_MODE;
    gsa_fmt |= NTP8918_GSA_RIGHT_J;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    input_fmt |= NTP8918_INPUT_FMT_GSA_MODE;
    break;
    }
    ret = snd_soc_component_update_bits(component, NTP8918_INPUT_FMT,
    NTP8918_INPUT_FMT_MASTER_MODE |
    NTP8918_INPUT_FMT_GSA_MODE,
    input_fmt);
    if (!(input_fmt & NTP8918_INPUT_FMT_GSA_MODE) || ret < 0)
    return ret;
    switch (params_width(params)) {
    case 24:
    gsa_fmt |= NTP8918_GSA_BS(0);
    break;
    case 20:
    gsa_fmt |= NTP8918_GSA_BS(1);
    break;
    case 18:
    gsa_fmt |= NTP8918_GSA_BS(2);
    break;
    case 16:
    gsa_fmt |= NTP8918_GSA_BS(3);
    break;
    default:
    return -EINVAL;
    }
    gsa_fmt_mask = NTP8918_GSA_BS_MASK |
    NTP8918_GSA_RIGHT_J |
    NTP8918_GSA_LSB;
    return snd_soc_component_update_bits(component, NTP8918_GSA_FMT,
    gsa_fmt_mask, gsa_fmt);
    }
#[no_mangle]
unsafe extern "C" fn ntp8918_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int ntp8918_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct snd_soc_component *component = dai.component;
    struct ntp8918_priv *ntp8918 = snd_soc_component_get_drvdata(component);
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    case SND_SOC_DAIFMT_RIGHT_J:
    case SND_SOC_DAIFMT_LEFT_J:
    ntp8918.format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntp8918_digital_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int ntp8918_digital_mute(struct snd_soc_dai *dai, int mute, int stream)
    {
    unsigned int mute_mask = NTP8918_SOFT_MUTE_SM1 |
    NTP8918_SOFT_MUTE_SM2;
    return snd_soc_component_update_bits(dai.component, NTP8918_SOFT_MUTE,
    mute_mask, mute ? mute_mask : 0);
    }
    static const struct snd_soc_dai_ops ntp8918_dai_ops = {
    .hw_params = ntp8918_hw_params,
    .set_fmt = ntp8918_set_fmt,
    .mute_stream = ntp8918_digital_mute,
    };
    static struct snd_soc_dai_driver ntp8918_dai = {
    .name = "ntp8918-amplifier",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = NTP8918_RATES,
    .formats = NTP8918_FORMATS,
    },
    .ops = &ntp8918_dai_ops,
    };
    static const struct regmap_config ntp8918_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_MAX,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn ntp8918_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ntp8918_i2c_probe(struct i2c_client *i2c)
    {
    struct ntp8918_priv *ntp8918;
    int ret;
    struct regmap *regmap;
    ntp8918 = devm_kzalloc(&i2c.dev, sizeof(*ntp8918), GFP_KERNEL);
    if (!ntp8918)
    return -ENOMEM;
    ntp8918.i2c = i2c;
    ntp8918.reset = devm_reset_control_get_shared(&i2c.dev, core::ptr::null_mut());
    if (IS_ERR(ntp8918.reset))
    return dev_err_probe(&i2c.dev, PTR_ERR(ntp8918.reset), "Failed to get reset\n");
    dev_set_drvdata(&i2c.dev, ntp8918);
    ntp8918_reset_gpio(ntp8918);
    regmap = devm_regmap_init_i2c(i2c, &ntp8918_regmap);
    if (IS_ERR(regmap))
    return dev_err_probe(&i2c.dev, PTR_ERR(regmap),
    "Failed to allocate regmap\n");
    ret = devm_snd_soc_register_component(&i2c.dev, &soc_component_ntp8918,
    &ntp8918_dai, 1);
    if (ret)
    return dev_err_probe(&i2c.dev, ret,
    "Failed to register component\n");
    ntp8918.bck = devm_clk_get_enabled(&i2c.dev, "bck");
    if (IS_ERR(ntp8918.bck))
    return dev_err_probe(&i2c.dev, PTR_ERR(ntp8918.bck), "failed to get bck clock\n");
    return 0;
    }
    static const struct i2c_device_id ntp8918_i2c_id[] = {
    { .name = "ntp8918" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ntp8918_i2c_id);
    static const struct of_device_id ntp8918_of_match[] = {
    {.compatible = "neofidelity,ntp8918"},
    {}
    };
    MODULE_DEVICE_TABLE(of, ntp8918_of_match);
    static struct i2c_driver ntp8918_i2c_driver = {
    .probe = ntp8918_i2c_probe,
    .id_table = ntp8918_i2c_id,
    .driver = {
    .name = "ntp8918",
    .of_match_table = ntp8918_of_match,
    },
    };
    module_i2c_driver(ntp8918_i2c_driver);
    MODULE_AUTHOR("Igor Prusov <ivprusov@salutedevices.com>");
    MODULE_DESCRIPTION("NTP8918 Audio Amplifier Driver");
    MODULE_LICENSE("GPL");
