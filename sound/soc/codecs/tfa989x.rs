//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tfa989x.c
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
// Copyright (C) 2021 Stephan Gerhold
//
// Register definitions/sequences taken from various tfa98xx kernel drivers:
// Copyright (C) 2014-2020 NXP Semiconductors, All Rights Reserved.
// Copyright (C) 2013 Sony Mobile Communications Inc.
//

pub const TFA989X_STATUSREG: c_uint = 0x00;
pub const TFA989X_BATTERYVOLTAGE: c_uint = 0x01;
pub const TFA989X_TEMPERATURE: c_uint = 0x02;
pub const TFA989X_REVISIONNUMBER: c_uint = 0x03;

pub const TFA989X_I2SREG: c_uint = 0x04;

pub const TFA989X_BAT_PROT: c_uint = 0x05;
pub const TFA989X_AUDIO_CTR: c_uint = 0x06;
pub const TFA989X_DCDCBOOST: c_uint = 0x07;
pub const TFA989X_SPKR_CALIBRATION: c_uint = 0x08;
pub const TFA989X_SYS_CTRL: c_uint = 0x09;

pub const TFA989X_I2S_SEL_REG: c_uint = 0x0a;

pub const TFA989X_HIDE_UNHIDE_KEY: c_uint = 0x40;
pub const TFA989X_PWM_CONTROL: c_uint = 0x41;
pub const TFA989X_CURRENTSENSE1: c_uint = 0x46;
pub const TFA989X_CURRENTSENSE2: c_uint = 0x47;
pub const TFA989X_CURRENTSENSE3: c_uint = 0x48;
pub const TFA989X_CURRENTSENSE4: c_uint = 0x49;
pub const TFA9890_REVISION: c_uint = 0x80;
pub const TFA9895_REVISION: c_uint = 0x12;
pub const TFA9897_REVISION: c_uint = 0x97;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfa989x_rev {
    pub rev: c_uint,
    pub regmap): *mut *mut int (init)(struct regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfa989x {
    pub rev: *const tfa989x_rev,
    pub vddd_supply: *mut regulator,
    pub rcv_gpiod: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn tfa989x_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tfa989x_writeable_reg(struct device *dev, unsigned int reg)
    {
    return reg > TFA989X_REVISIONNUMBER;
    }
#[no_mangle]
unsafe extern "C" fn tfa989x_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tfa989x_volatile_reg(struct device *dev, unsigned int reg)
    {
    return reg < TFA989X_REVISIONNUMBER;
    }
    static const struct regmap_config tfa989x_regmap = {
    .reg_bits = 8,
    .val_bits = 16,
    .writeable_reg	= tfa989x_writeable_reg,
    .volatile_reg	= tfa989x_volatile_reg,
    .cache_type	= REGCACHE_RBTREE,
    };
    static const char * const chsa_text[] = { "Left", "Right", /* "DSP" */ };
    static SOC_ENUM_SINGLE_DECL(chsa_enum, TFA989X_I2SREG, TFA989X_I2SREG_CHSA, chsa_text);
    let mut chsa_mux: static struct snd_kcontrol_new = SOC_DAPM_ENUM("Amp Input", chsa_enum);
    static const struct snd_soc_dapm_widget tfa989x_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("OUT"),
    SND_SOC_DAPM_SUPPLY("POWER", TFA989X_SYS_CTRL, TFA989X_SYS_CTRL_PWDN, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV("AMPE", TFA989X_SYS_CTRL, TFA989X_SYS_CTRL_AMPE, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MUX("Amp Input", SND_SOC_NOPM, 0, 0, &chsa_mux),
    SND_SOC_DAPM_AIF_IN("AIFINL", "HiFi Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("AIFINR", "HiFi Playback", 1, SND_SOC_NOPM, 0, 0),
    };
    static const struct snd_soc_dapm_route tfa989x_dapm_routes[] = {
    {"OUT", core::ptr::null_mut(), "AMPE"},
    {"AMPE", core::ptr::null_mut(), "POWER"},
    {"AMPE", core::ptr::null_mut(), "Amp Input"},
    {"Amp Input", "Left", "AIFINL"},
    {"Amp Input", "Right", "AIFINR"},
    };
#[no_mangle]
unsafe extern "C" fn tfa989x_put_mode(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static int tfa989x_put_mode(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct tfa989x *tfa989x = snd_soc_component_get_drvdata(component);
    gpiod_set_value_cansleep(tfa989x.rcv_gpiod, ucontrol.value.enumerated.item[0]);
    return snd_soc_put_enum_double(kcontrol, ucontrol);
    }
    static const char * const mode_text[] = { "Speaker", "Receiver" };
    static SOC_ENUM_SINGLE_DECL(mode_enum, TFA989X_I2SREG, TFA989X_I2SREG_RCV, mode_text);
    static const struct snd_kcontrol_new tfa989x_mode_controls[] = {
    SOC_ENUM_EXT("Mode", mode_enum, snd_soc_get_enum_double, tfa989x_put_mode),
    };
#[no_mangle]
unsafe extern "C" fn tfa989x_probe(component: *mut snd_soc_component) -> c_int {
    static int tfa989x_probe(struct snd_soc_component *component)
    {
    struct tfa989x *tfa989x = snd_soc_component_get_drvdata(component);
    if (tfa989x.rev.rev == TFA9897_REVISION)
    return snd_soc_add_component_controls(component, tfa989x_mode_controls,
    ARRAY_SIZE(tfa989x_mode_controls));
    return 0;
    }
    static const struct snd_soc_component_driver tfa989x_component = {
    .probe			= tfa989x_probe,
    .dapm_widgets		= tfa989x_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(tfa989x_dapm_widgets),
    .dapm_routes		= tfa989x_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(tfa989x_dapm_routes),
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const unsigned int tfa989x_rates[] = {
    8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000
    };
#[no_mangle]
unsafe extern "C" fn tfa989x_find_sample_rate(rate: c_uint) -> c_int {
    static int tfa989x_find_sample_rate(unsigned int rate)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(tfa989x_rates); ++i)
    if (tfa989x_rates[i] == rate)
    return i;
    return -EINVAL;
    }
    static int tfa989x_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    int sr;
    sr = tfa989x_find_sample_rate(params_rate(params));
    if (sr < 0)
    return sr;
    return snd_soc_component_update_bits(component, TFA989X_I2SREG,
    TFA989X_I2SREG_I2SSR_MSK,
    sr << TFA989X_I2SREG_I2SSR);
    }
    static const struct snd_soc_dai_ops tfa989x_dai_ops = {
    .hw_params = tfa989x_hw_params,
    };
    static struct snd_soc_dai_driver tfa989x_dai = {
    .name = "tfa989x-hifi",
    .playback = {
    .stream_name	= "HiFi Playback",
    .formats	= SNDRV_PCM_FMTBIT_S16_LE,
    .rates		= SNDRV_PCM_RATE_8000_48000,
    .rate_min	= 8000,
    .rate_max	= 48000,
    .channels_min	= 1,
    .channels_max	= 2,
    },
    .ops = &tfa989x_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn tfa9890_init(regmap: *mut regmap) -> c_int {
    static int tfa9890_init(struct regmap *regmap)
    {
    int ret;
// temporarily allow access to hidden registers
    ret = regmap_write(regmap, TFA989X_HIDE_UNHIDE_KEY, 0x5a6b);
    if (ret)
    return ret;
// update PLL registers
    ret = regmap_set_bits(regmap, 0x59, 0x3);
    if (ret)
    return ret;
// hide registers again
    ret = regmap_write(regmap, TFA989X_HIDE_UNHIDE_KEY, 0x0000);
    if (ret)
    return ret;
    return regmap_write(regmap, TFA989X_CURRENTSENSE2, 0x7BE1);
    }
    static const struct tfa989x_rev tfa9890_rev = {
    .rev	= TFA9890_REVISION,
    .init	= tfa9890_init,
    };
    static const struct reg_sequence tfa9895_reg_init[] = {
// some other registers must be set for optimal amplifier behaviour
    { TFA989X_BAT_PROT, 0x13ab },
    { TFA989X_AUDIO_CTR, 0x001f },
// peak voltage protection is always on, but may be written
    { TFA989X_SPKR_CALIBRATION, 0x3c4e },
// TFA989X_SYSCTRL_DCA = 0
    { TFA989X_SYS_CTRL, 0x024d },
    { TFA989X_PWM_CONTROL, 0x0308 },
    { TFA989X_CURRENTSENSE4, 0x0e82 },
    };
#[no_mangle]
unsafe extern "C" fn tfa9895_init(regmap: *mut regmap) -> c_int {
    static int tfa9895_init(struct regmap *regmap)
    {
    return regmap_multi_reg_write(regmap, tfa9895_reg_init,
    ARRAY_SIZE(tfa9895_reg_init));
    }
    static const struct tfa989x_rev tfa9895_rev = {
    .rev	= TFA9895_REVISION,
    .init	= tfa9895_init,
    };
#[no_mangle]
unsafe extern "C" fn tfa9897_init(regmap: *mut regmap) -> c_int {
    static int tfa9897_init(struct regmap *regmap)
    {
    int ret;
// Reduce slewrate by clearing iddqtestbst to avoid booster damage
    ret = regmap_write(regmap, TFA989X_CURRENTSENSE3, 0x0300);
    if (ret)
    return ret;
// Enable clipping
    ret = regmap_clear_bits(regmap, TFA989X_CURRENTSENSE4, 0x1);
    if (ret)
    return ret;
// Set required TDM configuration
    return regmap_write(regmap, 0x14, 0x0);
    }
    static const struct tfa989x_rev tfa9897_rev = {
    .rev	= TFA9897_REVISION,
    .init	= tfa9897_init,
    };
//
// Note: At the moment this driver bypasses the "CoolFlux DSP" built into the
// TFA989X amplifiers. Unfortunately, there seems to be absolutely
// no documentation for it - the public "short datasheets" do not provide
// any information about the DSP or available registers.
//
// Usually the TFA989X amplifiers are configured through proprietary userspace
// libraries. There are also some (rather complex) kernel drivers but even those
// rely on obscure firmware blobs for configuration (so-called "containers").
// They seem to contain different "profiles" with tuned speaker settings, sample
// rates and volume steps (which would be better exposed as separate ALSA mixers).
//
// Bypassing the DSP disables volume control (and perhaps some speaker
// optimization?), but at least allows using the speaker without obscure
// kernel drivers and firmware.
//
// Ideally NXP (or now Goodix) should release proper documentation for these
// amplifiers so that support for the "CoolFlux DSP" can be implemented properly.
//
#[no_mangle]
unsafe extern "C" fn tfa989x_dsp_bypass(regmap: *mut regmap) -> c_int {
    static int tfa989x_dsp_bypass(struct regmap *regmap)
    {
    int ret;
// Clear CHSA to bypass DSP and take input from I2S 1 left channel
    ret = regmap_clear_bits(regmap, TFA989X_I2SREG, TFA989X_I2SREG_CHSA_MSK);
    if (ret)
    return ret;
// Set DCDC compensation to off and speaker impedance to 8 ohm
    ret = regmap_update_bits(regmap, TFA989X_I2S_SEL_REG,
    TFA989X_I2S_SEL_REG_DCFG_MSK |
    TFA989X_I2S_SEL_REG_SPKR_MSK,
    TFA989X_I2S_SEL_REG_SPKR_MSK);
    if (ret)
    return ret;
// Set DCDC to follower mode and disable CoolFlux DSP
    return regmap_clear_bits(regmap, TFA989X_SYS_CTRL,
    BIT(TFA989X_SYS_CTRL_DCA) |
    BIT(TFA989X_SYS_CTRL_CFE) |
    BIT(TFA989X_SYS_CTRL_AMPC));
    }
#[no_mangle]
unsafe extern "C" fn tfa989x_regulator_disable(data: *mut c_void) {
    static void tfa989x_regulator_disable(void *data)
    {
    struct tfa989x *tfa989x = data;
    regulator_disable(tfa989x.vddd_supply);
    }
#[no_mangle]
unsafe extern "C" fn tfa989x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int tfa989x_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    const struct tfa989x_rev *rev;
    struct tfa989x *tfa989x;
    struct regmap *regmap;
    unsigned int val;
    int ret;
    rev = device_get_match_data(dev);
    if (!rev) {
    dev_err(dev, "unknown device revision\n");
    return -ENODEV;
    }
    tfa989x = devm_kzalloc(dev, sizeof(*tfa989x), GFP_KERNEL);
    if (!tfa989x)
    return -ENOMEM;
    tfa989x.rev = rev;
    i2c_set_clientdata(i2c, tfa989x);
    tfa989x.vddd_supply = devm_regulator_get(dev, "vddd");
    if (IS_ERR(tfa989x.vddd_supply))
    return dev_err_probe(dev, PTR_ERR(tfa989x.vddd_supply),
    "Failed to get vddd regulator\n");
    if (tfa989x.rev.rev == TFA9897_REVISION) {
    tfa989x.rcv_gpiod = devm_gpiod_get_optional(dev, "rcv", GPIOD_OUT_LOW);
    if (IS_ERR(tfa989x.rcv_gpiod))
    return PTR_ERR(tfa989x.rcv_gpiod);
    }
    regmap = devm_regmap_init_i2c(i2c, &tfa989x_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ret = regulator_enable(tfa989x.vddd_supply);
    if (ret) {
    dev_err(dev, "Failed to enable vddd regulator: %d\n", ret);
    return ret;
    }
    ret = devm_add_action_or_reset(dev, tfa989x_regulator_disable, tfa989x);
    if (ret)
    return ret;
// Bypass regcache for reset and init sequence
    regcache_cache_bypass(regmap, true);
// Dummy read to generate i2c clocks, required on some devices
    regmap_read(regmap, TFA989X_REVISIONNUMBER, &val);
    ret = regmap_read(regmap, TFA989X_REVISIONNUMBER, &val);
    if (ret) {
    dev_err(dev, "failed to read revision number: %d\n", ret);
    return ret;
    }
    val &= TFA989X_REVISIONNUMBER_REV_MSK;
    if (val != rev.rev) {
    dev_err(dev, "invalid revision number, expected %#x, got %#x\n",
    rev.rev, val);
    return -ENODEV;
    }
    ret = regmap_write(regmap, TFA989X_SYS_CTRL, BIT(TFA989X_SYS_CTRL_I2CR));
    if (ret) {
    dev_err(dev, "failed to reset I2C registers: %d\n", ret);
    return ret;
    }
    ret = rev.init(regmap);
    if (ret) {
    dev_err(dev, "failed to initialize registers: %d\n", ret);
    return ret;
    }
    ret = tfa989x_dsp_bypass(regmap);
    if (ret) {
    dev_err(dev, "failed to enable DSP bypass: %d\n", ret);
    return ret;
    }
    regcache_cache_bypass(regmap, false);
    return devm_snd_soc_register_component(dev, &tfa989x_component,
    &tfa989x_dai, 1);
    }
    static const struct of_device_id tfa989x_of_match[] = {
    { .compatible = "nxp,tfa9890", .data = &tfa9890_rev },
    { .compatible = "nxp,tfa9895", .data = &tfa9895_rev },
    { .compatible = "nxp,tfa9897", .data = &tfa9897_rev },
    { }
    };
    MODULE_DEVICE_TABLE(of, tfa989x_of_match);
    static struct i2c_driver tfa989x_i2c_driver = {
    .driver = {
    .name = "tfa989x",
    .of_match_table = tfa989x_of_match,
    },
    .probe = tfa989x_i2c_probe,
    };
    module_i2c_driver(tfa989x_i2c_driver);
    MODULE_DESCRIPTION("ASoC NXP/Goodix TFA989X (TFA1) driver");
    MODULE_AUTHOR("Stephan Gerhold <stephan@gerhold.net>");
    MODULE_LICENSE("GPL");
