//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ak4613.c
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
// ak4613.c  --  Asahi Kasei ALSA Soc Audio driver
//
// Copyright (C) 2015 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// Based on ak4642.c by Kuninori Morimoto
// Based on wm8731.c by Richard Purdie
// Based on ak4535.c by Richard Purdie
// Based on wm8753.c by Liam Girdwood
//
// +-------+
// |AK4613	|
// SDTO1 <-|	|
// |	|
// SDTI1 ->|	|
// SDTI2 ->|	|
// SDTI3 ->|	|
// +-------+
//
// +---+
// clk	  |   |___________________________________________...
//
// [TDM512]
// SDTO1  [L1][R1][L2][R2]
// SDTI1  [L1][R1][L2][R2][L3][R3][L4][R4][L5][R5][L6][R6]
//
// [TDM256]
// SDTO1  [L1][R1][L2][R2]
// SDTI1  [L1][R1][L2][R2][L3][R3][L4][R4]
// SDTI2  [L5][R5][L6][R6]
//
// [TDM128]
// SDTO1  [L1][R1][L2][R2]
// SDTI1  [L1][R1][L2][R2]
// SDTI2  [L3][R3][L4][R4]
// SDTI3  [L5][R5][L6][R6]
//
// [STEREO]
// Playback  2ch : SDTI1
// Capture   2ch : SDTO1
//
// [TDM512]
// Playback 12ch : SDTI1
// Capture   4ch : SDTO1
//
// [TDM256]
// Playback 12ch : SDTI1 + SDTI2
// Playback  8ch : SDTI1
// Capture   4ch : SDTO1
//
// [TDM128]
// Playback 12ch : SDTI1 + SDTI2 + SDTI3
// Playback  8ch : SDTI1 + SDTI2
// Playback  4ch : SDTI1
// Capture   4ch : SDTO1
//
// !!! NOTE !!!
//
// Renesas is the only user of ak4613 on upstream so far,
// but the chip connection is like below.
// Thus, Renesas can't test all connection case.
// Tested TDM is very limited.
//
// +-----+	+-----------+
// | SoC |	|  AK4613   |
// |     |<-----|SDTO1	 IN1|<-- Mic
// |     |	|	 IN2|
// |     |	|	    |
// |     |----->|SDTI1	OUT1|--> Headphone
// +-----+	|SDTI2	OUT2|
// |SDTI3	OUT3|
// |	OUT4|
// |	OUT5|
// |	OUT6|
// +-----------+
//
// Renesas SoC can handle [2,  6,8]    channels.
// Ak4613      can handle [2,4,  8,12] channels.
//
// Because of above HW connection and available channels number,
// Renesas could test are ...
//
// [STEREO] Playback  2ch : SDTI1
// Capture   2ch : SDTO1
// [TDM256] Playback  8ch : SDTI1 (*)
//
// (*) it used 8ch data between SoC <-> AK4613 on TDM256 mode,
// but could confirm is only first 2ch because only 1
// Headphone is connected.
//
// see
// AK4613_ENABLE_TDM_TEST
//

pub const PW_MGMT1: c_uint = 0x00 /* Power Management 1 */;
pub const PW_MGMT2: c_uint = 0x01 /* Power Management 2 */;
pub const PW_MGMT3: c_uint = 0x02 /* Power Management 3 */;
pub const CTRL1: c_uint = 0x03 /* Control 1 */;
pub const CTRL2: c_uint = 0x04 /* Control 2 */;
pub const DEMP1: c_uint = 0x05 /* De-emphasis1 */;
pub const DEMP2: c_uint = 0x06 /* De-emphasis2 */;
pub const OFD: c_uint = 0x07 /* Overflow Detect */;
pub const ZRD: c_uint = 0x08 /* Zero Detect */;
pub const ICTRL: c_uint = 0x09 /* Input Control */;
pub const OCTRL: c_uint = 0x0a /* Output Control */;
pub const LOUT1: c_uint = 0x0b /* LOUT1 Volume Control */;
pub const ROUT1: c_uint = 0x0c /* ROUT1 Volume Control */;
pub const LOUT2: c_uint = 0x0d /* LOUT2 Volume Control */;
pub const ROUT2: c_uint = 0x0e /* ROUT2 Volume Control */;
pub const LOUT3: c_uint = 0x0f /* LOUT3 Volume Control */;
pub const ROUT3: c_uint = 0x10 /* ROUT3 Volume Control */;
pub const LOUT4: c_uint = 0x11 /* LOUT4 Volume Control */;
pub const ROUT4: c_uint = 0x12 /* ROUT4 Volume Control */;
pub const LOUT5: c_uint = 0x13 /* LOUT5 Volume Control */;
pub const ROUT5: c_uint = 0x14 /* ROUT5 Volume Control */;
pub const LOUT6: c_uint = 0x15 /* LOUT6 Volume Control */;
pub const ROUT6: c_uint = 0x16 /* ROUT6 Volume Control */;
// PW_MGMT1

// PW_MGMT2
pub const PMAD_ALL: c_uint = 0x7;
// PW_MGMT3
pub const PMDA_ALL: c_uint = 0x3f;
// CTRL1

// CTRL2

// ICTRL

// OCTRL

//
// configs
//
// 0x000000BA
//
// B : AK4613_CONFIG_SDTI_x
// A : AK4613_CONFIG_MODE_x
//

//
// AK4613_CONFIG_SDTI_x
//
// It indicates how many SDTIx is connected.
//

//
// AK4613_CONFIG_MODE_x
//
// Same as Ctrl1 :: TDM1/TDM0
// No shift is requested
// see
// AK4613_CTRL1_TO_MODE()
// Table 11/12/13/14
//

//
// !!!! FIXME !!!!
//
// Because of testable HW limitation, TDM256 8ch TDM was only tested.
// This driver uses AK4613_ENABLE_TDM_TEST instead of new DT property so far.
// Don't hesitate to update driver, you don't need to care compatible
// with Renesas.
//
// #define AK4613_ENABLE_TDM_TEST
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4613_interface {
    pub width: c_uint,
    pub fmt: c_uint,
    pub dif: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4613_priv {
    pub lock: mutex,
    pub constraint_rates: snd_pcm_hw_constraint_list,
    pub constraint_channels: snd_pcm_hw_constraint_list,
    pub dummy_write_work: work_struct,
    pub component: *mut snd_soc_component,
    pub rate: c_uint,
    pub sysclk: c_uint,
    pub fmt: c_uint,
    pub configs: c_uint,
    pub cnt: c_int,
    pub ctrl1: u8,
    pub oc: u8,
    pub ic: u8,
}

//
// Playback Volume
//
// max : 0x00 : 0 dB
// ( 0.5 dB step )
// min : 0xFE : -127.0 dB
// mute: 0xFF
//
    static const DECLARE_TLV_DB_SCALE(out_tlv, -12750, 50, 1);
    static const struct snd_kcontrol_new ak4613_snd_controls[] = {
    SOC_DOUBLE_R_TLV("Digital Playback Volume1", LOUT1, ROUT1,
    0, 0xFF, 1, out_tlv),
    SOC_DOUBLE_R_TLV("Digital Playback Volume2", LOUT2, ROUT2,
    0, 0xFF, 1, out_tlv),
    SOC_DOUBLE_R_TLV("Digital Playback Volume3", LOUT3, ROUT3,
    0, 0xFF, 1, out_tlv),
    SOC_DOUBLE_R_TLV("Digital Playback Volume4", LOUT4, ROUT4,
    0, 0xFF, 1, out_tlv),
    SOC_DOUBLE_R_TLV("Digital Playback Volume5", LOUT5, ROUT5,
    0, 0xFF, 1, out_tlv),
    SOC_DOUBLE_R_TLV("Digital Playback Volume6", LOUT6, ROUT6,
    0, 0xFF, 1, out_tlv),
    };
    static const struct reg_default ak4613_reg[] = {
    { 0x0,  0x0f }, { 0x1,  0x07 }, { 0x2,  0x3f }, { 0x3,  0x20 },
    { 0x4,  0x20 }, { 0x5,  0x55 }, { 0x6,  0x05 }, { 0x7,  0x07 },
    { 0x8,  0x0f }, { 0x9,  0x07 }, { 0xa,  0x3f }, { 0xb,  0x00 },
    { 0xc,  0x00 }, { 0xd,  0x00 }, { 0xe,  0x00 }, { 0xf,  0x00 },
    { 0x10, 0x00 }, { 0x11, 0x00 }, { 0x12, 0x00 }, { 0x13, 0x00 },
    { 0x14, 0x00 }, { 0x15, 0x00 }, { 0x16, 0x00 },
    };
//
// CTRL1 register
// see
// Table 11/12/13/14
//

    {					\
    .dif	= _dif,			\
    .width	= _width,		\
    .fmt	= SND_SOC_DAIFMT_##_fmt,\
    }
    static const struct ak4613_interface ak4613_iface[] = {
// It doesn't support asymmetric format
    AUDIO_IFACE(0x03, 24, LEFT_J),
    AUDIO_IFACE(0x04, 24, I2S),
    };

    static const struct regmap_config ak4613_regmap_cfg = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= 0x16,
    .reg_defaults		= ak4613_reg,
    .num_reg_defaults	= ARRAY_SIZE(ak4613_reg),
    .cache_type		= REGCACHE_RBTREE,
    };
    static const struct of_device_id ak4613_of_match[] = {
    { .compatible = "asahi-kasei,ak4613",	.data = &ak4613_regmap_cfg },
    {},
    };
    MODULE_DEVICE_TABLE(of, ak4613_of_match);
    static const struct i2c_device_id ak4613_i2c_id[] = {
    { .name = "ak4613", .driver_data = (kernel_ulong_t)&ak4613_regmap_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ak4613_i2c_id);
    static const struct snd_soc_dapm_widget ak4613_dapm_widgets[] = {
// Outputs
    SND_SOC_DAPM_OUTPUT("LOUT1"),
    SND_SOC_DAPM_OUTPUT("LOUT2"),
    SND_SOC_DAPM_OUTPUT("LOUT3"),
    SND_SOC_DAPM_OUTPUT("LOUT4"),
    SND_SOC_DAPM_OUTPUT("LOUT5"),
    SND_SOC_DAPM_OUTPUT("LOUT6"),
    SND_SOC_DAPM_OUTPUT("ROUT1"),
    SND_SOC_DAPM_OUTPUT("ROUT2"),
    SND_SOC_DAPM_OUTPUT("ROUT3"),
    SND_SOC_DAPM_OUTPUT("ROUT4"),
    SND_SOC_DAPM_OUTPUT("ROUT5"),
    SND_SOC_DAPM_OUTPUT("ROUT6"),
// Inputs
    SND_SOC_DAPM_INPUT("LIN1"),
    SND_SOC_DAPM_INPUT("LIN2"),
    SND_SOC_DAPM_INPUT("RIN1"),
    SND_SOC_DAPM_INPUT("RIN2"),
// DAC
    SND_SOC_DAPM_DAC("DAC1", core::ptr::null_mut(), PW_MGMT3, 0, 0),
    SND_SOC_DAPM_DAC("DAC2", core::ptr::null_mut(), PW_MGMT3, 1, 0),
    SND_SOC_DAPM_DAC("DAC3", core::ptr::null_mut(), PW_MGMT3, 2, 0),
    SND_SOC_DAPM_DAC("DAC4", core::ptr::null_mut(), PW_MGMT3, 3, 0),
    SND_SOC_DAPM_DAC("DAC5", core::ptr::null_mut(), PW_MGMT3, 4, 0),
    SND_SOC_DAPM_DAC("DAC6", core::ptr::null_mut(), PW_MGMT3, 5, 0),
// ADC
    SND_SOC_DAPM_ADC("ADC1", core::ptr::null_mut(), PW_MGMT2, 0, 0),
    SND_SOC_DAPM_ADC("ADC2", core::ptr::null_mut(), PW_MGMT2, 1, 0),
    };
    static const struct snd_soc_dapm_route ak4613_intercon[] = {
    {"LOUT1", core::ptr::null_mut(), "DAC1"},
    {"LOUT2", core::ptr::null_mut(), "DAC2"},
    {"LOUT3", core::ptr::null_mut(), "DAC3"},
    {"LOUT4", core::ptr::null_mut(), "DAC4"},
    {"LOUT5", core::ptr::null_mut(), "DAC5"},
    {"LOUT6", core::ptr::null_mut(), "DAC6"},
    {"ROUT1", core::ptr::null_mut(), "DAC1"},
    {"ROUT2", core::ptr::null_mut(), "DAC2"},
    {"ROUT3", core::ptr::null_mut(), "DAC3"},
    {"ROUT4", core::ptr::null_mut(), "DAC4"},
    {"ROUT5", core::ptr::null_mut(), "DAC5"},
    {"ROUT6", core::ptr::null_mut(), "DAC6"},
    {"DAC1", core::ptr::null_mut(), "Playback"},
    {"DAC2", core::ptr::null_mut(), "Playback"},
    {"DAC3", core::ptr::null_mut(), "Playback"},
    {"DAC4", core::ptr::null_mut(), "Playback"},
    {"DAC5", core::ptr::null_mut(), "Playback"},
    {"DAC6", core::ptr::null_mut(), "Playback"},
    {"Capture", core::ptr::null_mut(), "ADC1"},
    {"Capture", core::ptr::null_mut(), "ADC2"},
    {"ADC1", core::ptr::null_mut(), "LIN1"},
    {"ADC2", core::ptr::null_mut(), "LIN2"},
    {"ADC1", core::ptr::null_mut(), "RIN1"},
    {"ADC2", core::ptr::null_mut(), "RIN2"},
    };
    static void ak4613_dai_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4613_priv *priv = snd_soc_component_get_drvdata(component);
    struct device *dev = component.dev;
    guard(mutex)(&priv.lock);
    priv.cnt--;
    if (priv.cnt < 0) {
    dev_err(dev, "unexpected counter error\n");
    priv.cnt = 0;
    }
    if (!priv.cnt)
    priv.ctrl1 = 0;
    }
    static void ak4613_hw_constraints(struct ak4613_priv *priv,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    static const unsigned int ak4613_rates[] = {
    32000,
    44100,
    48000,
    64000,
    88200,
    96000,
    176400,
    192000,
    };
pub const AK4613_CHANNEL_2: c_int = 0;
pub const AK4613_CHANNEL_4: c_int = 1;
pub const AK4613_CHANNEL_8: c_int = 2;
pub const AK4613_CHANNEL_12: c_int = 3;

    static const unsigned int ak4613_channels[] = {
    [AK4613_CHANNEL_2]  =  2,
    [AK4613_CHANNEL_4]  =  4,
    [AK4613_CHANNEL_8]  =  8,
    [AK4613_CHANNEL_12] = 12,
    };
pub const MODE_MAX: c_int = 4;
pub const SDTx_MAX: c_int = 4;

    static const int mask_list[MODE_MAX][SDTx_MAX] = {
// SDTO	 SDTIx1    SDTIx2		SDTIx3
    [AK4613_CONFIG_MODE_STEREO] = { MASK(2), MASK(2),  MASK(2),		MASK(2)},
    [AK4613_CONFIG_MODE_TDM512] = { MASK(4), MASK(12), MASK(12),		MASK(12)},
    [AK4613_CONFIG_MODE_TDM256] = { MASK(4), MASK(8),  MASK(8)|MASK(12),	MASK(8)|MASK(12)},
    [AK4613_CONFIG_MODE_TDM128] = { MASK(4), MASK(4),  MASK(4)|MASK(8),	MASK(4)|MASK(8)|MASK(12)},
    };
    struct snd_pcm_hw_constraint_list *constraint;
    unsigned int mask;
    unsigned int mode;
    unsigned int fs;
    let mut is_play: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK;
    int sdti_num;
    int i;
    constraint		= &priv.constraint_rates;
    constraint.list	= ak4613_rates;
    constraint.mask	= 0;
    constraint.count	= 0;
//
// Slave Mode
// Normal: [32kHz, 48kHz] : 256fs,384fs or 512fs
// Double: [64kHz, 96kHz] : 256fs
// Quad  : [128kHz,192kHz]: 128fs
//
// Master mode
// Normal: [32kHz, 48kHz] : 256fs or 512fs
// Double: [64kHz, 96kHz] : 256fs
// Quad  : [128kHz,192kHz]: 128fs
//
    for (i = 0; i < ARRAY_SIZE(ak4613_rates); i++) {
// minimum fs on each range
    fs = (ak4613_rates[i] <= 96000) ? 256 : 128;
    if (priv.sysclk >= ak4613_rates[i] * fs)
    constraint.count = i + 1;
    }
    snd_pcm_hw_constraint_list(runtime, 0,
    SNDRV_PCM_HW_PARAM_RATE, constraint);
    sdti_num = AK4613_CONFIG_SDTI_get(priv);
    if (WARN_ON(sdti_num >= SDTx_MAX))
    return;
    if (priv.cnt) {
//
// If it was already working,
// the constraint is same as working mode.
//
    mode = AK4613_CTRL1_TO_MODE(priv);
    mask = 0; /* no default */
    } else {
//
// It is not yet working,
// the constraint is based on board configs.
// STEREO mask is default
//
    mode = AK4613_CONFIG_GET(priv, MODE);
    mask = mask_list[AK4613_CONFIG_MODE_STEREO][is_play * sdti_num];
    }
    if (WARN_ON(mode >= MODE_MAX))
    return;
// add each mode mask
    mask |= mask_list[mode][is_play * sdti_num];
    constraint		= &priv.constraint_channels;
    constraint.list	= ak4613_channels;
    constraint.mask	= mask;
    constraint.count	= sizeof(ak4613_channels);
    snd_pcm_hw_constraint_list(runtime, 0,
    SNDRV_PCM_HW_PARAM_CHANNELS, constraint);
    }
    static int ak4613_dai_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4613_priv *priv = snd_soc_component_get_drvdata(component);
    guard(mutex)(&priv.lock);
    ak4613_hw_constraints(priv, substream);
    priv.cnt++;
    return 0;
    }
    static int ak4613_dai_set_sysclk(struct snd_soc_dai *codec_dai,
    int clk_id, unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct ak4613_priv *priv = snd_soc_component_get_drvdata(component);
    priv.sysclk = freq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4613_dai_set_fmt(dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    static int ak4613_dai_set_fmt(struct snd_soc_dai *dai, unsigned int format)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4613_priv *priv = snd_soc_component_get_drvdata(component);
    unsigned int fmt;
    fmt = format & SND_SOC_DAIFMT_FORMAT_MASK;
    switch (fmt) {
    case SND_SOC_DAIFMT_LEFT_J:
    case SND_SOC_DAIFMT_I2S:
    priv.fmt = fmt;
    break;
    default:
    return -EINVAL;
    }
    fmt = format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    switch (fmt) {
    case SND_SOC_DAIFMT_CBC_CFC:
    break;
    default:
//
// SUPPORTME
//
// "clock provider" is not yet supperted
//
    return -EINVAL;
    }
    return 0;
    }
    static int ak4613_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4613_priv *priv = snd_soc_component_get_drvdata(component);
    struct device *dev = component.dev;
    let mut width: c_uint = params_width(params);
    let mut fmt: c_uint = priv.fmt;
    unsigned int rate;
    int i, ret;
    u8 ctrl2;
    rate = params_rate(params);
    switch (rate) {
    case 32000:
    case 44100:
    case 48000:
    ctrl2 = DFS_NORMAL_SPEED;
    break;
    case 64000:
    case 88200:
    case 96000:
    ctrl2 = DFS_DOUBLE_SPEED;
    break;
    case 176400:
    case 192000:
    ctrl2 = DFS_QUAD_SPEED;
    break;
    default:
    return -EINVAL;
    }
    priv.rate = rate;
//
// FIXME
//
// It doesn't have full TDM suppert yet
//
    ret = -EINVAL;
    scoped_guard(mutex, &priv.lock) {
    if (priv.cnt > 1) {
//
// If it was already working, use current priv->ctrl1
//
    ret = 0;
    } else {
//
// It is not yet working,
//
    let mut channel: c_uint = params_channels(params);
    u8 tdm;
// STEREO or TDM
    if (channel == 2)
    tdm = AK4613_CONFIG_MODE_STEREO;
    else
    tdm = AK4613_CONFIG_GET(priv, MODE);
    for (i = ARRAY_SIZE(ak4613_iface) - 1; i >= 0; i--) {
    const struct ak4613_interface *iface = ak4613_iface + i;
    if (iface.fmt == fmt && iface.width == width) {
//
// Ctrl1
// | D7 | D6 | D5 | D4 | D3 | D2 | D1 | D0  |
// |TDM1|TDM0|DIF2|DIF1|DIF0|ATS1|ATS0|SMUTE|
// <  tdm  > < iface->dif >
//
    priv.ctrl1 = (tdm << 6) | (iface.dif << 3);
    ret = 0;
    break;
    }
    }
    }
    }
    if (ret < 0) {
    dev_warn(dev, "unsupported data width/format combination\n");
    return ret;
    }
    snd_soc_component_update_bits(component, CTRL1, FMT_MASK, priv.ctrl1);
    snd_soc_component_update_bits(component, CTRL2, DFS_MASK, ctrl2);
    snd_soc_component_update_bits(component, ICTRL, ICTRL_MASK, priv.ic);
    snd_soc_component_update_bits(component, OCTRL, OCTRL_MASK, priv.oc);
    return ret;
    }
    static int ak4613_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    let mut mgmt1: u8 = 0;
    switch (level) {
    case SND_SOC_BIAS_ON:
    mgmt1 |= RSTN;
    fallthrough;
    case SND_SOC_BIAS_PREPARE:
    mgmt1 |= PMADC | PMDAC;
    fallthrough;
    case SND_SOC_BIAS_STANDBY:
    mgmt1 |= PMVR;
    fallthrough;
    case SND_SOC_BIAS_OFF:
    default:
    break;
    }
    snd_soc_component_write(component, PW_MGMT1, mgmt1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4613_dummy_write(work: *mut work_struct) {
    static void ak4613_dummy_write(struct work_struct *work)
    {
    struct ak4613_priv *priv = container_of(work,
    struct ak4613_priv,
    dummy_write_work);
    struct snd_soc_component *component = priv.component;
    unsigned int mgmt1;
    unsigned int mgmt3;
//
// PW_MGMT1 / PW_MGMT3 needs dummy write at least after 5 LR clocks
//
// Note
//
// To avoid extra delay, we want to avoid preemption here,
// but we can't. Because it uses I2C access which is using IRQ
// and sleep. Thus, delay might be more than 5 LR clocks
// see also
// ak4613_dai_trigger()
//
    udelay(5000000 / priv.rate);
    mgmt1 = snd_soc_component_read(component, PW_MGMT1);
    mgmt3 = snd_soc_component_read(component, PW_MGMT3);
    snd_soc_component_write(component, PW_MGMT1, mgmt1);
    snd_soc_component_write(component, PW_MGMT3, mgmt3);
    }
    static int ak4613_dai_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4613_priv *priv = snd_soc_component_get_drvdata(component);
//
// FIXME
//
// PW_MGMT1 / PW_MGMT3 needs dummy write at least after 5 LR clocks
// from Power Down Release. Otherwise, Playback volume will be 0dB.
// To avoid complex multiple delay/dummy_write method from
// ak4613_set_bias_level() / SND_SOC_DAPM_DAC_E("DACx", ...),
// call it once here.
//
// But, unfortunately, we can't "write" here because here is atomic
// context (It uses I2C access for writing).
// Thus, use schedule_work() to switching to normal context
// immediately.
//
// Note
//
// Calling ak4613_dummy_write() function might be delayed.
// In such case, ak4613 volume might be temporarily 0dB when
// beggining of playback.
// see also
// ak4613_dummy_write()
//
    if ((cmd != SNDRV_PCM_TRIGGER_START) &&
    (cmd != SNDRV_PCM_TRIGGER_RESUME))
    return 0;
    if (substream.stream != SNDRV_PCM_STREAM_PLAYBACK)
    return  0;
    priv.component = component;
    schedule_work(&priv.dummy_write_work);
    return 0;
    }
    static const u64 ak4613_dai_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J;
    static const struct snd_soc_dai_ops ak4613_dai_ops = {
    .startup	= ak4613_dai_startup,
    .shutdown	= ak4613_dai_shutdown,
    .set_sysclk	= ak4613_dai_set_sysclk,
    .set_fmt	= ak4613_dai_set_fmt,
    .trigger	= ak4613_dai_trigger,
    .hw_params	= ak4613_dai_hw_params,
    .auto_selectable_formats	= &ak4613_dai_formats,
    .num_auto_selectable_formats	= 1,
    };

    SNDRV_PCM_RATE_44100  |\
    SNDRV_PCM_RATE_48000  |\
    SNDRV_PCM_RATE_64000  |\
    SNDRV_PCM_RATE_88200  |\
    SNDRV_PCM_RATE_96000  |\
    SNDRV_PCM_RATE_176400 |\
    SNDRV_PCM_RATE_192000)

    static struct snd_soc_dai_driver ak4613_dai = {
    .name = "ak4613-hifi",
    .playback = {
    .stream_name	= "Playback",
    .channels_min	= 2,
    .channels_max	= 12,
    .rates		= AK4613_PCM_RATE,
    .formats	= AK4613_PCM_FMTBIT,
    },
    .capture = {
    .stream_name	= "Capture",
    .channels_min	= 2,
    .channels_max	= 4,
    .rates		= AK4613_PCM_RATE,
    .formats	= AK4613_PCM_FMTBIT,
    },
    .ops = &ak4613_dai_ops,
    .symmetric_rate = 1,
    };
#[no_mangle]
unsafe extern "C" fn ak4613_suspend(component: *mut snd_soc_component) -> c_int {
    static int ak4613_suspend(struct snd_soc_component *component)
    {
    struct regmap *regmap = dev_get_regmap(component.dev, core::ptr::null_mut());
    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4613_resume(component: *mut snd_soc_component) -> c_int {
    static int ak4613_resume(struct snd_soc_component *component)
    {
    struct regmap *regmap = dev_get_regmap(component.dev, core::ptr::null_mut());
    regcache_cache_only(regmap, false);
    return regcache_sync(regmap);
    }
    static const struct snd_soc_component_driver soc_component_dev_ak4613 = {
    .suspend		= ak4613_suspend,
    .resume			= ak4613_resume,
    .set_bias_level		= ak4613_set_bias_level,
    .controls		= ak4613_snd_controls,
    .num_controls		= ARRAY_SIZE(ak4613_snd_controls),
    .dapm_widgets		= ak4613_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ak4613_dapm_widgets),
    .dapm_routes		= ak4613_intercon,
    .num_dapm_routes	= ARRAY_SIZE(ak4613_intercon),
    .idle_bias_on		= 1,
    .endianness		= 1,
    };
    static void ak4613_parse_of(struct ak4613_priv *priv,
    struct device *dev)
    {
    struct device_node *np = dev.of_node;
    char prop[32];
    int sdti_num;
    int i;
// Input 1 - 2
    for (i = 0; i < 2; i++) {
    snprintf(prop, sizeof(prop), "asahi-kasei,in%d-single-end", i + 1);
    if (!of_property_read_bool(np, prop))
    priv.ic |= 1 << i;
    }
// Output 1 - 6
    for (i = 0; i < 6; i++) {
    snprintf(prop, sizeof(prop), "asahi-kasei,out%d-single-end", i + 1);
    if (!of_property_read_bool(np, prop))
    priv.oc |= 1 << i;
    }
//
// enable TDM256 test
//
// !!! FIXME !!!
//
// It should be configured by DT or other way
// if it was full supported.
// But it is using ifdef style for now for test
// purpose.
//

    AK4613_CONFIG_SET(priv, MODE_TDM256);

//
// connected STDI
// TDM support is assuming it is probed via Audio-Graph-Card style here.
// Default is SDTIx1 if it was probed via Simple-Audio-Card for now.
//
    sdti_num = of_graph_get_endpoint_count(np);
    if ((sdti_num >= SDTx_MAX) || (sdti_num < 1))
    sdti_num = 1;
    AK4613_CONFIG_SDTI_set(priv, sdti_num);
    }
#[no_mangle]
unsafe extern "C" fn ak4613_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ak4613_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    const struct regmap_config *regmap_cfg;
    struct regmap *regmap;
    struct ak4613_priv *priv;
    regmap_cfg = i2c_get_match_data(i2c);
    if (!regmap_cfg)
    return -EINVAL;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ak4613_parse_of(priv, dev);
    priv.ctrl1		= 0;
    priv.cnt		= 0;
    priv.sysclk		= 0;
    INIT_WORK(&priv.dummy_write_work, ak4613_dummy_write);
    mutex_init(&priv.lock);
    i2c_set_clientdata(i2c, priv);
    regmap = devm_regmap_init_i2c(i2c, regmap_cfg);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return devm_snd_soc_register_component(dev, &soc_component_dev_ak4613,
    &ak4613_dai, 1);
    }
    static struct i2c_driver ak4613_i2c_driver = {
    .driver = {
    .name = "ak4613-codec",
    .of_match_table = ak4613_of_match,
    },
    .probe		= ak4613_i2c_probe,
    .id_table	= ak4613_i2c_id,
    };
    module_i2c_driver(ak4613_i2c_driver);
    MODULE_DESCRIPTION("Soc AK4613 driver");
    MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
    MODULE_LICENSE("GPL v2");
