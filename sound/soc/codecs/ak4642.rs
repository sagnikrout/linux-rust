//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ak4642.c
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
// ak4642.c  --  AK4642/AK4643 ALSA Soc Audio driver
//
// Copyright (C) 2009 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Based on wm8731.c by Richard Purdie
// Based on ak4535.c by Richard Purdie
// Based on wm8753.c by Liam Girdwood
// ** CAUTION
//
// This is very simple driver.
// It can use headphone output / stereo input only
//
// AK4642 is tested.
// AK4643 is tested.
// AK4648 is tested.
//

pub const PW_MGMT1: c_uint = 0x00;
pub const PW_MGMT2: c_uint = 0x01;
pub const SG_SL1: c_uint = 0x02;
pub const SG_SL2: c_uint = 0x03;
pub const MD_CTL1: c_uint = 0x04;
pub const MD_CTL2: c_uint = 0x05;
pub const TIMER: c_uint = 0x06;
pub const ALC_CTL1: c_uint = 0x07;
pub const ALC_CTL2: c_uint = 0x08;
pub const L_IVC: c_uint = 0x09;
pub const L_DVC: c_uint = 0x0a;
pub const ALC_CTL3: c_uint = 0x0b;
pub const R_IVC: c_uint = 0x0c;
pub const R_DVC: c_uint = 0x0d;
pub const MD_CTL3: c_uint = 0x0e;
pub const MD_CTL4: c_uint = 0x0f;
pub const PW_MGMT3: c_uint = 0x10;
pub const DF_S: c_uint = 0x11;
pub const FIL3_0: c_uint = 0x12;
pub const FIL3_1: c_uint = 0x13;
pub const FIL3_2: c_uint = 0x14;
pub const FIL3_3: c_uint = 0x15;
pub const EQ_0: c_uint = 0x16;
pub const EQ_1: c_uint = 0x17;
pub const EQ_2: c_uint = 0x18;
pub const EQ_3: c_uint = 0x19;
pub const EQ_4: c_uint = 0x1a;
pub const EQ_5: c_uint = 0x1b;
pub const FIL1_0: c_uint = 0x1c;
pub const FIL1_1: c_uint = 0x1d;
pub const FIL1_2: c_uint = 0x1e;
pub const FIL1_3: c_uint = 0x1f	/* The maximum valid register for ak4642 */;
pub const PW_MGMT4: c_uint = 0x20;
pub const MD_CTL5: c_uint = 0x21;
pub const LO_MS: c_uint = 0x22;
pub const HP_MS: c_uint = 0x23;
pub const SPK_MS: c_uint = 0x24	/* The maximum valid register for ak4643 */;
pub const EQ_FBEQAB: c_uint = 0x25;
pub const EQ_FBEQCD: c_uint = 0x26;
pub const EQ_FBEQE: c_uint = 0x27	/* The maximum valid register for ak4648 */;
// PW_MGMT1

// PW_MGMT2

// PW_MGMT3

// SG_SL1

// SG_SL2

// TIMER

// ALC_CTL1

// MD_CTL1

// MD_CTL2

// MD_CTL3

// MD_CTL4

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4642_drvdata {
    pub regmap_config: *const regmap_config,
    pub extended_frequencies: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4642_priv {
    pub drvdata: *const ak4642_drvdata,
    pub mcko: *mut clk,
}

//
// Playback Volume (table 39)
//
// max : 0x00 : +12.0 dB
// ( 0.5 dB step )
// min : 0xFE : -115.0 dB
// mute: 0xFF
//
    static const DECLARE_TLV_DB_SCALE(out_tlv, -11550, 50, 1);
    static const struct snd_kcontrol_new ak4642_snd_controls[] = {
    SOC_DOUBLE_R_TLV("Digital Playback Volume", L_DVC, R_DVC,
    0, 0xFF, 1, out_tlv),
    SOC_SINGLE("ALC Capture Switch", ALC_CTL1, 5, 1, 0),
    SOC_SINGLE("ALC Capture ZC Switch", ALC_CTL1, 4, 1, 1),
    };
    static const struct snd_kcontrol_new ak4642_headphone_control =
    SOC_DAPM_SINGLE("Switch", PW_MGMT2, 6, 1, 0);
    static const struct snd_kcontrol_new ak4642_lout_mixer_controls[] = {
    SOC_DAPM_SINGLE("DACL", SG_SL1, 4, 1, 0),
    };
// event handlers
    static int ak4642_lout_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMD:
    case SND_SOC_DAPM_PRE_PMU:
// Power save mode ON
    snd_soc_component_update_bits(component, SG_SL2, LOPS, LOPS);
    break;
    case SND_SOC_DAPM_POST_PMU:
    case SND_SOC_DAPM_POST_PMD:
// Power save mode OFF
    msleep(300);
    snd_soc_component_update_bits(component, SG_SL2, LOPS, 0);
    break;
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget ak4642_dapm_widgets[] = {
// Outputs
    SND_SOC_DAPM_OUTPUT("HPOUTL"),
    SND_SOC_DAPM_OUTPUT("HPOUTR"),
    SND_SOC_DAPM_OUTPUT("LINEOUT"),
    SND_SOC_DAPM_PGA("HPL Out", PW_MGMT2, 5, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("HPR Out", PW_MGMT2, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SWITCH("Headphone Enable", SND_SOC_NOPM, 0, 0,
    &ak4642_headphone_control),
    SND_SOC_DAPM_PGA("DACH", MD_CTL4, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER_E("LINEOUT Mixer", PW_MGMT1, 3, 0,
    &ak4642_lout_mixer_controls[0],
    ARRAY_SIZE(ak4642_lout_mixer_controls),
    ak4642_lout_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
// DAC
    SND_SOC_DAPM_DAC("DAC", core::ptr::null_mut(), PW_MGMT1, 2, 0),
    };
    static const struct snd_soc_dapm_route ak4642_intercon[] = {
// Outputs
    {"HPOUTL", core::ptr::null_mut(), "HPL Out"},
    {"HPOUTR", core::ptr::null_mut(), "HPR Out"},
    {"LINEOUT", core::ptr::null_mut(), "LINEOUT Mixer"},
    {"HPL Out", core::ptr::null_mut(), "Headphone Enable"},
    {"HPR Out", core::ptr::null_mut(), "Headphone Enable"},
    {"Headphone Enable", "Switch", "DACH"},
    {"DACH", core::ptr::null_mut(), "DAC"},
    {"LINEOUT Mixer", "DACL", "DAC"},
    { "DAC", core::ptr::null_mut(), "Playback" },
    };
//
// ak4642 register cache
//
    static const struct reg_default ak4643_reg[] = {
    {  0, 0x00 }, {  1, 0x00 }, {  2, 0x01 }, {  3, 0x00 },
    {  4, 0x02 }, {  5, 0x00 }, {  6, 0x00 }, {  7, 0x00 },
    {  8, 0xe1 }, {  9, 0xe1 }, { 10, 0x18 }, { 11, 0x00 },
    { 12, 0xe1 }, { 13, 0x18 }, { 14, 0x11 }, { 15, 0x08 },
    { 16, 0x00 }, { 17, 0x00 }, { 18, 0x00 }, { 19, 0x00 },
    { 20, 0x00 }, { 21, 0x00 }, { 22, 0x00 }, { 23, 0x00 },
    { 24, 0x00 }, { 25, 0x00 }, { 26, 0x00 }, { 27, 0x00 },
    { 28, 0x00 }, { 29, 0x00 }, { 30, 0x00 }, { 31, 0x00 },
    { 32, 0x00 }, { 33, 0x00 }, { 34, 0x00 }, { 35, 0x00 },
    { 36, 0x00 },
    };
// The default settings for 0x0 ~ 0x1f registers are the same for ak4642
    and ak4643. So we reuse the ak4643 reg_default for ak4642.
    The valid registers for ak4642 are 0x0 ~ 0x1f which is a subset of ak4643,
    so define NUM_AK4642_REG_DEFAULTS for ak4642.
//

    static const struct reg_default ak4648_reg[] = {
    {  0, 0x00 }, {  1, 0x00 }, {  2, 0x01 }, {  3, 0x00 },
    {  4, 0x02 }, {  5, 0x00 }, {  6, 0x00 }, {  7, 0x00 },
    {  8, 0xe1 }, {  9, 0xe1 }, { 10, 0x18 }, { 11, 0x00 },
    { 12, 0xe1 }, { 13, 0x18 }, { 14, 0x11 }, { 15, 0xb8 },
    { 16, 0x00 }, { 17, 0x00 }, { 18, 0x00 }, { 19, 0x00 },
    { 20, 0x00 }, { 21, 0x00 }, { 22, 0x00 }, { 23, 0x00 },
    { 24, 0x00 }, { 25, 0x00 }, { 26, 0x00 }, { 27, 0x00 },
    { 28, 0x00 }, { 29, 0x00 }, { 30, 0x00 }, { 31, 0x00 },
    { 32, 0x00 }, { 33, 0x00 }, { 34, 0x00 }, { 35, 0x00 },
    { 36, 0x00 }, { 37, 0x88 }, { 38, 0x88 }, { 39, 0x08 },
    };
    static int ak4642_dai_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    let mut is_play: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK;
    struct snd_soc_component *component = dai.component;
    if (is_play) {
//
// start headphone output
//
// PLL, Master Mode
// Audio I/F Format :MSB justified (ADC & DAC)
// Bass Boost Level : Middle
//
// This operation came from example code of
// "ASAHI KASEI AK4642" (japanese) manual p97.
//
    snd_soc_component_write(component, L_IVC, 0x91); /* volume */
    snd_soc_component_write(component, R_IVC, 0x91); /* volume */
    } else {
//
// start stereo input
//
// PLL Master Mode
// Audio I/F Format:MSB justified (ADC & DAC)
// Pre MIC AMP:+20dB
// MIC Power On
// ALC setting:Refer to Table 35
// ALC bit=“1”
//
// This operation came from example code of
// "ASAHI KASEI AK4642" (japanese) manual p94.
//
    snd_soc_component_update_bits(component, SG_SL1, PMMP | MGAIN0, PMMP | MGAIN0);
    snd_soc_component_write(component, TIMER, ZTM(0x3) | WTM(0x3));
    snd_soc_component_write(component, ALC_CTL1, ALC | LMTH0);
    snd_soc_component_update_bits(component, PW_MGMT1, PMADL, PMADL);
    snd_soc_component_update_bits(component, PW_MGMT3, PMADR, PMADR);
    }
    return 0;
    }
    static void ak4642_dai_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    let mut is_play: c_int = substream.stream == SNDRV_PCM_STREAM_PLAYBACK;
    struct snd_soc_component *component = dai.component;
    if (is_play) {
    } else {
// stop stereo input
    snd_soc_component_update_bits(component, PW_MGMT1, PMADL, 0);
    snd_soc_component_update_bits(component, PW_MGMT3, PMADR, 0);
    snd_soc_component_update_bits(component, ALC_CTL1, ALC, 0);
    }
    }
    static int ak4642_dai_set_sysclk(struct snd_soc_dai *codec_dai,
    int clk_id, unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct ak4642_priv *priv = snd_soc_component_get_drvdata(component);
    u8 pll;
    let mut extended_freq: c_int = 0;
    switch (freq) {
    case 11289600:
    pll = PLL2;
    break;
    case 12288000:
    pll = PLL2 | PLL0;
    break;
    case 12000000:
    pll = PLL2 | PLL1;
    break;
    case 24000000:
    pll = PLL2 | PLL1 | PLL0;
    break;
    case 13500000:
    pll = PLL3 | PLL2;
    break;
    case 27000000:
    pll = PLL3 | PLL2 | PLL0;
    break;
    case 19200000:
    pll = PLL3;
    extended_freq = 1;
    break;
    case 13000000:
    pll = PLL3 | PLL2 | PLL1;
    extended_freq = 1;
    break;
    case 26000000:
    pll = PLL3 | PLL2 | PLL1 | PLL0;
    extended_freq = 1;
    break;
    default:
    return -EINVAL;
    }
    if (extended_freq && !priv.drvdata.extended_frequencies)
    return -EINVAL;
    snd_soc_component_update_bits(component, MD_CTL1, PLL_MASK, pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4642_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int ak4642_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct snd_soc_component *component = dai.component;
    u8 data;
    u8 bcko;
    data = MCKO | PMPLL; /* use MCKO */
    bcko = 0;
// set clocking for audio interface
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    data |= MS;
    bcko = BCKO_64;
    break;
    case SND_SOC_DAIFMT_CBC_CFC:
    break;
    default:
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, PW_MGMT2, MS | MCKO | PMPLL, data);
    snd_soc_component_update_bits(component, MD_CTL1, BCKO_MASK, bcko);
// format type
    data = 0;
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_LEFT_J:
    data = LEFT_J;
    break;
    case SND_SOC_DAIFMT_I2S:
    data = I2S;
    break;
// FIXME
// Please add RIGHT_J / DSP support here
//
    default:
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, MD_CTL1, DIF_MASK, data);
    return 0;
    }
    static int ak4642_set_mcko(struct snd_soc_component *component,
    u32 frequency)
    {
    static const u32 fs_list[] = {
    [0] = 8000,
    [1] = 12000,
    [2] = 16000,
    [3] = 24000,
    [4] = 7350,
    [5] = 11025,
    [6] = 14700,
    [7] = 22050,
    [10] = 32000,
    [11] = 48000,
    [14] = 29400,
    [15] = 44100,
    };
    static const u32 ps_list[] = {
    [0] = 256,
    [1] = 128,
    [2] = 64,
    [3] = 32
    };
    int ps, fs;
    for (ps = 0; ps < ARRAY_SIZE(ps_list); ps++) {
    for (fs = 0; fs < ARRAY_SIZE(fs_list); fs++) {
    if (frequency == ps_list[ps] * fs_list[fs]) {
    snd_soc_component_write(component, MD_CTL2,
    PSs(ps) | FSs(fs));
    return 0;
    }
    }
    }
    return 0;
    }
    static int ak4642_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4642_priv *priv = snd_soc_component_get_drvdata(component);
    let mut rate: u32 = clk_get_rate(priv.mcko);
    if (!rate)
    rate = params_rate(params) * 256;
    return ak4642_set_mcko(component, rate);
    }
    static int ak4642_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    switch (level) {
    case SND_SOC_BIAS_OFF:
    snd_soc_component_write(component, PW_MGMT1, 0x00);
    break;
    default:
    snd_soc_component_update_bits(component, PW_MGMT1, PMVCM, PMVCM);
    break;
    }
    return 0;
    }
    static const struct snd_soc_dai_ops ak4642_dai_ops = {
    .startup	= ak4642_dai_startup,
    .shutdown	= ak4642_dai_shutdown,
    .set_sysclk	= ak4642_dai_set_sysclk,
    .set_fmt	= ak4642_dai_set_fmt,
    .hw_params	= ak4642_dai_hw_params,
    };
    static struct snd_soc_dai_driver ak4642_dai = {
    .name = "ak4642-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE },
    .ops = &ak4642_dai_ops,
    .symmetric_rate = 1,
    };
#[no_mangle]
unsafe extern "C" fn ak4642_suspend(component: *mut snd_soc_component) -> c_int {
    static int ak4642_suspend(struct snd_soc_component *component)
    {
    struct regmap *regmap = dev_get_regmap(component.dev, core::ptr::null_mut());
    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4642_resume(component: *mut snd_soc_component) -> c_int {
    static int ak4642_resume(struct snd_soc_component *component)
    {
    struct regmap *regmap = dev_get_regmap(component.dev, core::ptr::null_mut());
    regcache_cache_only(regmap, false);
    regcache_sync(regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4642_probe(component: *mut snd_soc_component) -> c_int {
    static int ak4642_probe(struct snd_soc_component *component)
    {
    struct ak4642_priv *priv = snd_soc_component_get_drvdata(component);
    if (priv.mcko)
    ak4642_set_mcko(component, clk_get_rate(priv.mcko));
    return 0;
    }
    static const struct snd_soc_component_driver soc_component_dev_ak4642 = {
    .probe			= ak4642_probe,
    .suspend		= ak4642_suspend,
    .resume			= ak4642_resume,
    .set_bias_level		= ak4642_set_bias_level,
    .controls		= ak4642_snd_controls,
    .num_controls		= ARRAY_SIZE(ak4642_snd_controls),
    .dapm_widgets		= ak4642_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ak4642_dapm_widgets),
    .dapm_routes		= ak4642_intercon,
    .num_dapm_routes	= ARRAY_SIZE(ak4642_intercon),
    .idle_bias_on		= 1,
    .endianness		= 1,
    };
    static const struct regmap_config ak4642_regmap = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= FIL1_3,
    .reg_defaults		= ak4642_reg,
    .num_reg_defaults	= NUM_AK4642_REG_DEFAULTS,
    .cache_type		= REGCACHE_RBTREE,
    };
    static const struct regmap_config ak4643_regmap = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= SPK_MS,
    .reg_defaults		= ak4643_reg,
    .num_reg_defaults	= ARRAY_SIZE(ak4643_reg),
    .cache_type		= REGCACHE_RBTREE,
    };
    static const struct regmap_config ak4648_regmap = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= EQ_FBEQE,
    .reg_defaults		= ak4648_reg,
    .num_reg_defaults	= ARRAY_SIZE(ak4648_reg),
    .cache_type		= REGCACHE_RBTREE,
    };
    static const struct ak4642_drvdata ak4642_drvdata = {
    .regmap_config = &ak4642_regmap,
    };
    static const struct ak4642_drvdata ak4643_drvdata = {
    .regmap_config = &ak4643_regmap,
    };
    static const struct ak4642_drvdata ak4648_drvdata = {
    .regmap_config = &ak4648_regmap,
    .extended_frequencies = 1,
    };

    static struct clk *ak4642_of_parse_mcko(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct clk *clk;
    const char *clk_name = np.name;
    const char *parent_clk_name = core::ptr::null_mut();
    u32 rate;
    if (of_property_read_u32(np, "clock-frequency", &rate))
    return core::ptr::null_mut();
    if (of_property_read_bool(np, "clocks"))
    parent_clk_name = of_clk_get_parent_name(np, 0);
    of_property_read_string(np, "clock-output-names", &clk_name);
    clk = clk_register_fixed_rate(dev, clk_name, parent_clk_name, 0, rate);
    if (!IS_ERR(clk))
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    return clk;
    }

pub const ak4642_of_parse_mcko(d): c_int = 0;

#[no_mangle]
unsafe extern "C" fn ak4642_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ak4642_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    const struct ak4642_drvdata *drvdata;
    struct regmap *regmap;
    struct ak4642_priv *priv;
    struct clk *mcko = core::ptr::null_mut();
    if (dev_fwnode(dev)) {
    mcko = ak4642_of_parse_mcko(dev);
    if (IS_ERR(mcko))
    mcko = core::ptr::null_mut();
    }
    drvdata = i2c_get_match_data(i2c);
    if (!drvdata)
    return dev_err_probe(dev, -EINVAL, "Unknown device type\n");
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.drvdata = drvdata;
    priv.mcko = mcko;
    i2c_set_clientdata(i2c, priv);
    regmap = devm_regmap_init_i2c(i2c, drvdata.regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return devm_snd_soc_register_component(dev,
    &soc_component_dev_ak4642, &ak4642_dai, 1);
    }
    static const struct of_device_id ak4642_of_match[] = {
    { .compatible = "asahi-kasei,ak4642",	.data = &ak4642_drvdata},
    { .compatible = "asahi-kasei,ak4643",	.data = &ak4643_drvdata},
    { .compatible = "asahi-kasei,ak4648",	.data = &ak4648_drvdata},
    {}
    };
    MODULE_DEVICE_TABLE(of, ak4642_of_match);
    static const struct i2c_device_id ak4642_i2c_id[] = {
    { .name = "ak4642", .driver_data = (kernel_ulong_t)&ak4642_drvdata },
    { .name = "ak4643", .driver_data = (kernel_ulong_t)&ak4643_drvdata },
    { .name = "ak4648", .driver_data = (kernel_ulong_t)&ak4648_drvdata },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ak4642_i2c_id);
    static struct i2c_driver ak4642_i2c_driver = {
    .driver = {
    .name = "ak4642-codec",
    .of_match_table = ak4642_of_match,
    },
    .probe		= ak4642_i2c_probe,
    .id_table	= ak4642_i2c_id,
    };
    module_i2c_driver(ak4642_i2c_driver);
    MODULE_DESCRIPTION("Soc AK4642 driver");
    MODULE_AUTHOR("Kuninori Morimoto <morimoto.kuninori@renesas.com>");
    MODULE_LICENSE("GPL v2");
