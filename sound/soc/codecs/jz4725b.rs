//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/jz4725b.c
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
// JZ4725B CODEC driver
//
// Copyright (C) 2019, Paul Cercueil <paul@crapouillou.net>

pub const ICDC_RGADW_OFFSET: c_uint = 0x00;
pub const ICDC_RGDATA_OFFSET: c_uint = 0x04;
// ICDC internal register access control register(RGADW)

pub const ICDC_RGADW_RGADDR_OFFSET: c_int = 8;

pub const ICDC_RGADW_RGDIN_OFFSET: c_int = 0;

// ICDC internal register data output register (RGDATA)

pub const ICDC_RGDATA_RGDOUT_OFFSET: c_int = 0;

// JZ internal register space
    enum {
    JZ4725B_CODEC_REG_AICR,
    JZ4725B_CODEC_REG_CR1,
    JZ4725B_CODEC_REG_CR2,
    JZ4725B_CODEC_REG_CCR1,
    JZ4725B_CODEC_REG_CCR2,
    JZ4725B_CODEC_REG_PMR1,
    JZ4725B_CODEC_REG_PMR2,
    JZ4725B_CODEC_REG_CRR,
    JZ4725B_CODEC_REG_ICR,
    JZ4725B_CODEC_REG_IFR,
    JZ4725B_CODEC_REG_CGR1,
    JZ4725B_CODEC_REG_CGR2,
    JZ4725B_CODEC_REG_CGR3,
    JZ4725B_CODEC_REG_CGR4,
    JZ4725B_CODEC_REG_CGR5,
    JZ4725B_CODEC_REG_CGR6,
    JZ4725B_CODEC_REG_CGR7,
    JZ4725B_CODEC_REG_CGR8,
    JZ4725B_CODEC_REG_CGR9,
    JZ4725B_CODEC_REG_CGR10,
    JZ4725B_CODEC_REG_TR1,
    JZ4725B_CODEC_REG_TR2,
    JZ4725B_CODEC_REG_CR3,
    JZ4725B_CODEC_REG_AGC1,
    JZ4725B_CODEC_REG_AGC2,
    JZ4725B_CODEC_REG_AGC3,
    JZ4725B_CODEC_REG_AGC4,
    JZ4725B_CODEC_REG_AGC5,
    };
pub const REG_AICR_CONFIG1_OFFSET: c_int = 0;

pub const REG_CR1_SB_MICBIAS_OFFSET: c_int = 7;
pub const REG_CR1_MONO_OFFSET: c_int = 6;
pub const REG_CR1_DAC_MUTE_OFFSET: c_int = 5;
pub const REG_CR1_HP_DIS_OFFSET: c_int = 4;
pub const REG_CR1_DACSEL_OFFSET: c_int = 3;
pub const REG_CR1_BYPASS_OFFSET: c_int = 2;
pub const REG_CR2_DAC_DEEMP_OFFSET: c_int = 7;
pub const REG_CR2_DAC_ADWL_OFFSET: c_int = 5;

pub const REG_CR2_ADC_ADWL_OFFSET: c_int = 3;

pub const REG_CR2_ADC_HPF_OFFSET: c_int = 2;
pub const REG_CR3_SB_MIC1_OFFSET: c_int = 7;
pub const REG_CR3_SB_MIC2_OFFSET: c_int = 6;
pub const REG_CR3_SIDETONE1_OFFSET: c_int = 5;
pub const REG_CR3_SIDETONE2_OFFSET: c_int = 4;
pub const REG_CR3_MICDIFF_OFFSET: c_int = 3;
pub const REG_CR3_MICSTEREO_OFFSET: c_int = 2;
pub const REG_CR3_INSEL_OFFSET: c_int = 0;

pub const REG_CCR1_CONFIG4_OFFSET: c_int = 0;

pub const REG_CCR2_DFREQ_OFFSET: c_int = 4;

pub const REG_CCR2_AFREQ_OFFSET: c_int = 0;

pub const REG_PMR1_SB_DAC_OFFSET: c_int = 7;
pub const REG_PMR1_SB_OUT_OFFSET: c_int = 6;
pub const REG_PMR1_SB_MIX_OFFSET: c_int = 5;
pub const REG_PMR1_SB_ADC_OFFSET: c_int = 4;
pub const REG_PMR1_SB_LIN_OFFSET: c_int = 3;
pub const REG_PMR1_SB_IND_OFFSET: c_int = 0;
pub const REG_PMR2_LRGI_OFFSET: c_int = 7;
pub const REG_PMR2_RLGI_OFFSET: c_int = 6;
pub const REG_PMR2_LRGOD_OFFSET: c_int = 5;
pub const REG_PMR2_RLGOD_OFFSET: c_int = 4;
pub const REG_PMR2_GIM_OFFSET: c_int = 3;
pub const REG_PMR2_SB_MC_OFFSET: c_int = 2;
pub const REG_PMR2_SB_OFFSET: c_int = 1;
pub const REG_PMR2_SB_SLEEP_OFFSET: c_int = 0;
pub const REG_IFR_RAMP_UP_DONE_OFFSET: c_int = 3;
pub const REG_IFR_RAMP_DOWN_DONE_OFFSET: c_int = 2;
pub const REG_CGR1_GODL_OFFSET: c_int = 4;

pub const REG_CGR1_GODR_OFFSET: c_int = 0;

pub const REG_CGR2_GO1R_OFFSET: c_int = 0;

pub const REG_CGR3_GO1L_OFFSET: c_int = 0;

pub const REG_CGR4_GO2R_OFFSET: c_int = 0;

pub const REG_CGR5_GO2L_OFFSET: c_int = 0;

pub const REG_CGR6_GO3R_OFFSET: c_int = 0;

pub const REG_CGR7_GO3L_OFFSET: c_int = 0;

pub const REG_CGR8_GOR_OFFSET: c_int = 0;

pub const REG_CGR9_GOL_OFFSET: c_int = 0;

pub const REG_CGR10_GIL_OFFSET: c_int = 0;
pub const REG_CGR10_GIR_OFFSET: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz_icdc {
    pub regmap: *mut regmap,
    pub base: *mut void __iomem,
}

    static const SNDRV_CTL_TLVD_DECLARE_DB_SCALE(jz4725b_adc_tlv,     0, 150, 0);
    static const SNDRV_CTL_TLVD_DECLARE_DB_SCALE(jz4725b_dac_tlv, -2250, 150, 0);
    static const SNDRV_CTL_TLVD_DECLARE_DB_RANGE(jz4725b_mix_tlv,
    0, 11, TLV_DB_SCALE_ITEM(-2250,   0, 0),
    12, 31, TLV_DB_SCALE_ITEM(-2250, 150, 0),
    );
    static const SNDRV_CTL_TLVD_DECLARE_DB_RANGE(jz4725b_out_tlv,
    0, 11, TLV_DB_SCALE_ITEM(-3350, 200, 0),
    12, 23, TLV_DB_SCALE_ITEM(-1050, 100, 0),
    24, 31, TLV_DB_SCALE_ITEM(  100,  50, 0),
    );
    static const SNDRV_CTL_TLVD_DECLARE_DB_SCALE(jz4725b_mic_boost_tlv, 0, 2000, 0);
    static const char * const jz4725b_mic_mode_texts[] = {
    "Single Ended", "Differential",
    };
    static const struct soc_enum jz4725b_mic_mode_enum =
    SOC_ENUM_SINGLE(JZ4725B_CODEC_REG_CR3, REG_CR3_MICDIFF_OFFSET,
    2, jz4725b_mic_mode_texts);
    static const struct snd_kcontrol_new jz4725b_codec_controls[] = {
    SOC_DOUBLE_TLV("DAC Playback Volume",
    JZ4725B_CODEC_REG_CGR1,
    REG_CGR1_GODL_OFFSET,
    REG_CGR1_GODR_OFFSET,
    0xf, 1, jz4725b_dac_tlv),
    SOC_DOUBLE_TLV("Master Capture Volume",
    JZ4725B_CODEC_REG_CGR10,
    REG_CGR10_GIL_OFFSET,
    REG_CGR10_GIR_OFFSET,
    0xf, 0, jz4725b_adc_tlv),
    SOC_DOUBLE_R_TLV("Mixer Line In Bypass Playback Volume",
    JZ4725B_CODEC_REG_CGR3,
    JZ4725B_CODEC_REG_CGR2,
    REG_CGR2_GO1R_OFFSET,
    0x1f, 1, jz4725b_mix_tlv),
    SOC_DOUBLE_R_TLV("Mixer Mic 1 Bypass Playback Volume",
    JZ4725B_CODEC_REG_CGR5,
    JZ4725B_CODEC_REG_CGR4,
    REG_CGR4_GO2R_OFFSET,
    0x1f, 1, jz4725b_mix_tlv),
    SOC_DOUBLE_R_TLV("Mixer Mic 2 Bypass Playback Volume",
    JZ4725B_CODEC_REG_CGR7,
    JZ4725B_CODEC_REG_CGR6,
    REG_CGR6_GO3R_OFFSET,
    0x1f, 1, jz4725b_mix_tlv),
    SOC_DOUBLE_R_TLV("Master Playback Volume",
    JZ4725B_CODEC_REG_CGR9,
    JZ4725B_CODEC_REG_CGR8,
    REG_CGR8_GOR_OFFSET,
    0x1f, 1, jz4725b_out_tlv),
    SOC_SINGLE("DAC Playback Switch", JZ4725B_CODEC_REG_CR1,
    REG_CR1_DAC_MUTE_OFFSET, 1, 1),
    SOC_SINGLE("Deemphasize Filter Playback Switch",
    JZ4725B_CODEC_REG_CR2,
    REG_CR2_DAC_DEEMP_OFFSET, 1, 0),
    SOC_SINGLE("High-Pass Filter Capture Switch",
    JZ4725B_CODEC_REG_CR2,
    REG_CR2_ADC_HPF_OFFSET, 1, 0),
    SOC_ENUM("Mic Mode Capture Switch", jz4725b_mic_mode_enum),
    SOC_SINGLE_TLV("Mic1 Boost Capture Volume",
    JZ4725B_CODEC_REG_PMR2,
    REG_PMR2_GIM_OFFSET,
    1, 0, jz4725b_mic_boost_tlv),
    };
    static const char * const jz4725b_codec_adc_src_texts[] = {
    "Mic 1", "Mic 2", "Line In", "Mixer",
    };
    static const unsigned int jz4725b_codec_adc_src_values[] = { 0, 1, 2, 3, };
    static SOC_VALUE_ENUM_SINGLE_DECL(jz4725b_codec_adc_src_enum,
    JZ4725B_CODEC_REG_CR3,
    REG_CR3_INSEL_OFFSET,
    REG_CR3_INSEL_MASK,
    jz4725b_codec_adc_src_texts,
    jz4725b_codec_adc_src_values);
    static const struct snd_kcontrol_new jz4725b_codec_adc_src_ctrl =
    SOC_DAPM_ENUM("ADC Source Capture Route", jz4725b_codec_adc_src_enum);
    static const struct snd_kcontrol_new jz4725b_codec_mixer_controls[] = {
    SOC_DAPM_SINGLE("Line In Bypass Playback Switch", JZ4725B_CODEC_REG_CR1,
    REG_CR1_BYPASS_OFFSET, 1, 0),
    SOC_DAPM_SINGLE("Mic 1 Bypass Playback Switch", JZ4725B_CODEC_REG_CR3,
    REG_CR3_SIDETONE1_OFFSET, 1, 0),
    SOC_DAPM_SINGLE("Mic 2 Bypass Playback Switch", JZ4725B_CODEC_REG_CR3,
    REG_CR3_SIDETONE2_OFFSET, 1, 0),
    };
    static int jz4725b_out_stage_enable(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *codec = snd_soc_dapm_to_component(w.dapm);
    struct jz_icdc *icdc = snd_soc_component_get_drvdata(codec);
    struct regmap *map = icdc.regmap;
    unsigned int val;
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    return regmap_clear_bits(map, JZ4725B_CODEC_REG_IFR,
    BIT(REG_IFR_RAMP_UP_DONE_OFFSET));
    case SND_SOC_DAPM_POST_PMU:
    return regmap_read_poll_timeout(map, JZ4725B_CODEC_REG_IFR,
    val, val & BIT(REG_IFR_RAMP_UP_DONE_OFFSET),
    100000, 500000);
    case SND_SOC_DAPM_PRE_PMD:
    return regmap_clear_bits(map, JZ4725B_CODEC_REG_IFR,
    BIT(REG_IFR_RAMP_DOWN_DONE_OFFSET));
    case SND_SOC_DAPM_POST_PMD:
    return regmap_read_poll_timeout(map, JZ4725B_CODEC_REG_IFR,
    val, val & BIT(REG_IFR_RAMP_DOWN_DONE_OFFSET),
    100000, 500000);
    default:
    return -EINVAL;
    }
    }
    static const struct snd_soc_dapm_widget jz4725b_codec_dapm_widgets[] = {
// DAC
    SND_SOC_DAPM_DAC("DAC", "Playback",
    JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_DAC_OFFSET, 1),
// ADC
    SND_SOC_DAPM_ADC("ADC", "Capture",
    JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_ADC_OFFSET, 1),
    SND_SOC_DAPM_MUX("ADC Source Capture Route", SND_SOC_NOPM, 0, 0,
    &jz4725b_codec_adc_src_ctrl),
// Mixer
    SND_SOC_DAPM_MIXER("Mixer", JZ4725B_CODEC_REG_PMR1,
    REG_PMR1_SB_MIX_OFFSET, 1,
    jz4725b_codec_mixer_controls,
    ARRAY_SIZE(jz4725b_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("DAC to Mixer", JZ4725B_CODEC_REG_CR1,
    REG_CR1_DACSEL_OFFSET, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("Line In", JZ4725B_CODEC_REG_PMR1,
    REG_PMR1_SB_LIN_OFFSET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("HP Out", JZ4725B_CODEC_REG_CR1,
    REG_CR1_HP_DIS_OFFSET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("Mic 1", JZ4725B_CODEC_REG_CR3,
    REG_CR3_SB_MIC1_OFFSET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("Mic 2", JZ4725B_CODEC_REG_CR3,
    REG_CR3_SB_MIC2_OFFSET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER_E("Out Stage", JZ4725B_CODEC_REG_PMR1,
    REG_PMR1_SB_OUT_OFFSET, 1, core::ptr::null_mut(), 0,
    jz4725b_out_stage_enable,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER("Mixer to ADC", JZ4725B_CODEC_REG_PMR1,
    REG_PMR1_SB_IND_OFFSET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Mic Bias", JZ4725B_CODEC_REG_CR1,
    REG_CR1_SB_MICBIAS_OFFSET, 1, core::ptr::null_mut(), 0),
// Pins
    SND_SOC_DAPM_INPUT("MIC1P"),
    SND_SOC_DAPM_INPUT("MIC1N"),
    SND_SOC_DAPM_INPUT("MIC2P"),
    SND_SOC_DAPM_INPUT("MIC2N"),
    SND_SOC_DAPM_INPUT("LLINEIN"),
    SND_SOC_DAPM_INPUT("RLINEIN"),
    SND_SOC_DAPM_OUTPUT("LHPOUT"),
    SND_SOC_DAPM_OUTPUT("RHPOUT"),
    };
    static const struct snd_soc_dapm_route jz4725b_codec_dapm_routes[] = {
    {"Mic 1", core::ptr::null_mut(), "MIC1P"},
    {"Mic 1", core::ptr::null_mut(), "MIC1N"},
    {"Mic 2", core::ptr::null_mut(), "MIC2P"},
    {"Mic 2", core::ptr::null_mut(), "MIC2N"},
    {"Line In", core::ptr::null_mut(), "LLINEIN"},
    {"Line In", core::ptr::null_mut(), "RLINEIN"},
    {"Mixer", "Mic 1 Bypass Playback Switch", "Mic 1"},
    {"Mixer", "Mic 2 Bypass Playback Switch", "Mic 2"},
    {"Mixer", "Line In Bypass Playback Switch", "Line In"},
    {"DAC to Mixer", core::ptr::null_mut(), "DAC"},
    {"Mixer", core::ptr::null_mut(), "DAC to Mixer"},
    {"Mixer to ADC", core::ptr::null_mut(), "Mixer"},
    {"ADC Source Capture Route", "Mixer", "Mixer to ADC"},
    {"ADC Source Capture Route", "Line In", "Line In"},
    {"ADC Source Capture Route", "Mic 1", "Mic 1"},
    {"ADC Source Capture Route", "Mic 2", "Mic 2"},
    {"ADC", core::ptr::null_mut(), "ADC Source Capture Route"},
    {"Out Stage", core::ptr::null_mut(), "Mixer"},
    {"HP Out", core::ptr::null_mut(), "Out Stage"},
    {"LHPOUT", core::ptr::null_mut(), "HP Out"},
    {"RHPOUT", core::ptr::null_mut(), "HP Out"},
    };
    static int jz4725b_codec_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    struct jz_icdc *icdc = snd_soc_component_get_drvdata(component);
    struct regmap *map = icdc.regmap;
    switch (level) {
    case SND_SOC_BIAS_ON:
    regmap_clear_bits(map, JZ4725B_CODEC_REG_PMR2,
    BIT(REG_PMR2_SB_SLEEP_OFFSET));
    break;
    case SND_SOC_BIAS_PREPARE:
// Enable sound hardware
    regmap_clear_bits(map, JZ4725B_CODEC_REG_PMR2,
    BIT(REG_PMR2_SB_OFFSET));
    msleep(224);
    break;
    case SND_SOC_BIAS_STANDBY:
    regmap_set_bits(map, JZ4725B_CODEC_REG_PMR2,
    BIT(REG_PMR2_SB_SLEEP_OFFSET));
    break;
    case SND_SOC_BIAS_OFF:
    regmap_set_bits(map, JZ4725B_CODEC_REG_PMR2,
    BIT(REG_PMR2_SB_OFFSET));
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4725b_codec_dev_probe(component: *mut snd_soc_component) -> c_int {
    static int jz4725b_codec_dev_probe(struct snd_soc_component *component)
    {
    struct jz_icdc *icdc = snd_soc_component_get_drvdata(component);
    struct regmap *map = icdc.regmap;
// Write CONFIGn (n=1 to 8) bits.
// The value 0x0f is specified in the datasheet as a requirement.
//
    regmap_write(map, JZ4725B_CODEC_REG_AICR,
    0xf << REG_AICR_CONFIG1_OFFSET);
    regmap_write(map, JZ4725B_CODEC_REG_CCR1,
    0x0 << REG_CCR1_CONFIG4_OFFSET);
    return 0;
    }
    static const struct snd_soc_component_driver jz4725b_codec = {
    .probe			= jz4725b_codec_dev_probe,
    .set_bias_level		= jz4725b_codec_set_bias_level,
    .controls		= jz4725b_codec_controls,
    .num_controls		= ARRAY_SIZE(jz4725b_codec_controls),
    .dapm_widgets		= jz4725b_codec_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(jz4725b_codec_dapm_widgets),
    .dapm_routes		= jz4725b_codec_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(jz4725b_codec_dapm_routes),
    .suspend_bias_off	= 1,
    .use_pmdown_time	= 1,
    };
    static const unsigned int jz4725b_codec_sample_rates[] = {
    96000, 48000, 44100, 32000,
    24000, 22050, 16000, 12000,
    11025, 9600, 8000,
    };
    static int jz4725b_codec_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct jz_icdc *icdc = snd_soc_component_get_drvdata(dai.component);
    unsigned int rate, bit_width;
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    bit_width = 0;
    break;
    case SNDRV_PCM_FORMAT_S18_3LE:
    bit_width = 1;
    break;
    case SNDRV_PCM_FORMAT_S20_3LE:
    bit_width = 2;
    break;
    case SNDRV_PCM_FORMAT_S24_3LE:
    bit_width = 3;
    break;
    default:
    return -EINVAL;
    }
    for (rate = 0; rate < ARRAY_SIZE(jz4725b_codec_sample_rates); rate++) {
    if (jz4725b_codec_sample_rates[rate] == params_rate(params))
    break;
    }
    if (rate == ARRAY_SIZE(jz4725b_codec_sample_rates))
    return -EINVAL;
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    regmap_update_bits(icdc.regmap,
    JZ4725B_CODEC_REG_CR2,
    REG_CR2_DAC_ADWL_MASK,
    bit_width << REG_CR2_DAC_ADWL_OFFSET);
    regmap_update_bits(icdc.regmap,
    JZ4725B_CODEC_REG_CCR2,
    REG_CCR2_DFREQ_MASK,
    rate << REG_CCR2_DFREQ_OFFSET);
    } else {
    regmap_update_bits(icdc.regmap,
    JZ4725B_CODEC_REG_CR2,
    REG_CR2_ADC_ADWL_MASK,
    bit_width << REG_CR2_ADC_ADWL_OFFSET);
    regmap_update_bits(icdc.regmap,
    JZ4725B_CODEC_REG_CCR2,
    REG_CCR2_AFREQ_MASK,
    rate << REG_CCR2_AFREQ_OFFSET);
    }
    return 0;
    }
    static const struct snd_soc_dai_ops jz4725b_codec_dai_ops = {
    .hw_params = jz4725b_codec_hw_params,
    };

    SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE)
    static struct snd_soc_dai_driver jz4725b_codec_dai = {
    .name = "jz4725b-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = JZ_ICDC_FORMATS,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = JZ_ICDC_FORMATS,
    },
    .ops = &jz4725b_codec_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn jz4725b_codec_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool jz4725b_codec_volatile(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = JZ4725B_CODEC_REG_IFR;
    }
#[no_mangle]
unsafe extern "C" fn jz4725b_codec_can_access_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool jz4725b_codec_can_access_reg(struct device *dev, unsigned int reg)
    {
    return (reg != JZ4725B_CODEC_REG_TR1) && (reg != JZ4725B_CODEC_REG_TR2);
    }
#[no_mangle]
unsafe extern "C" fn jz4725b_codec_io_wait(icdc: *mut jz_icdc) -> c_int {
    static int jz4725b_codec_io_wait(struct jz_icdc *icdc)
    {
    u32 reg;
    return readl_poll_timeout(icdc.base + ICDC_RGADW_OFFSET, reg,
    !(reg & ICDC_RGADW_RGWR), 1000, 10000);
    }
    static int jz4725b_codec_reg_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct jz_icdc *icdc = context;
    unsigned int i;
    u32 tmp;
    int ret;
    ret = jz4725b_codec_io_wait(icdc);
    if (ret)
    return ret;
    tmp = readl(icdc.base + ICDC_RGADW_OFFSET);
    tmp = (tmp & ~ICDC_RGADW_RGADDR_MASK)
    | (reg << ICDC_RGADW_RGADDR_OFFSET);
    writel(tmp, icdc.base + ICDC_RGADW_OFFSET);
// wait 6+ cycles
    for (i = 0; i < 6; i++)
// val = readl(icdc->base + ICDC_RGDATA_OFFSET) &
    ICDC_RGDATA_RGDOUT_MASK;
    return 0;
    }
    static int jz4725b_codec_reg_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct jz_icdc *icdc = context;
    int ret;
    ret = jz4725b_codec_io_wait(icdc);
    if (ret)
    return ret;
    writel(ICDC_RGADW_RGWR | (reg << ICDC_RGADW_RGADDR_OFFSET) | val,
    icdc.base + ICDC_RGADW_OFFSET);
    ret = jz4725b_codec_io_wait(icdc);
    if (ret)
    return ret;
    return 0;
    }
    static const u8 jz4725b_codec_reg_defaults[] = {
    0x0c, 0xaa, 0x78, 0x00, 0x00, 0xff, 0x03, 0x51,
    0x3f, 0x00, 0x00, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x0a, 0x0a, 0x00, 0x00, 0x00, 0xc0, 0x34,
    0x07, 0x44, 0x1f, 0x00,
    };
    static const struct regmap_config jz4725b_codec_regmap_config = {
    .reg_bits = 7,
    .val_bits = 8,
    .max_register = JZ4725B_CODEC_REG_AGC5,
    .volatile_reg = jz4725b_codec_volatile,
    .readable_reg = jz4725b_codec_can_access_reg,
    .writeable_reg = jz4725b_codec_can_access_reg,
    .reg_read = jz4725b_codec_reg_read,
    .reg_write = jz4725b_codec_reg_write,
    .reg_defaults_raw = jz4725b_codec_reg_defaults,
    .num_reg_defaults_raw = ARRAY_SIZE(jz4725b_codec_reg_defaults),
    .cache_type = REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn jz4725b_codec_probe(pdev: *mut platform_device) -> c_int {
    static int jz4725b_codec_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct jz_icdc *icdc;
    struct clk *clk;
    int ret;
    icdc = devm_kzalloc(dev, sizeof(*icdc), GFP_KERNEL);
    if (!icdc)
    return -ENOMEM;
    icdc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(icdc.base))
    return PTR_ERR(icdc.base);
    icdc.regmap = devm_regmap_init(dev, core::ptr::null_mut(), icdc,
    &jz4725b_codec_regmap_config);
    if (IS_ERR(icdc.regmap))
    return PTR_ERR(icdc.regmap);
    clk = devm_clk_get_enabled(dev, "aic");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    platform_set_drvdata(pdev, icdc);
    ret = devm_snd_soc_register_component(dev, &jz4725b_codec,
    &jz4725b_codec_dai, 1);
    if (ret)
    dev_err(dev, "Failed to register codec\n");
    return ret;
    }
    static const struct of_device_id jz4725b_codec_of_matches[] = {
    { .compatible = "ingenic,jz4725b-codec", },
    { }
    };
    MODULE_DEVICE_TABLE(of, jz4725b_codec_of_matches);
    static struct platform_driver jz4725b_codec_driver = {
    .probe = jz4725b_codec_probe,
    .driver = {
    .name = "jz4725b-codec",
    .of_match_table = jz4725b_codec_of_matches,
    },
    };
    module_platform_driver(jz4725b_codec_driver);
    MODULE_DESCRIPTION("JZ4725B SoC internal codec driver");
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_LICENSE("GPL v2");
