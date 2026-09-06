//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/rk817_codec.c
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
// rk817 ALSA SoC Audio driver
//
// Copyright (c) 2018, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk817_codec_priv {
    pub component: *mut snd_soc_component,
    pub rk808: *mut rk808,
    pub mclk: *mut clk,
    pub stereo_sysclk: c_uint,
    pub mic_in_differential: bool,
}

//
// This sets the codec up with the values defined in the default implementation including the APLL
// from the Rockchip vendor kernel. I do not know if these values are universal despite differing
// from the default values defined above and taken from the datasheet, or implementation specific.
// I don't have another implementation to compare from the Rockchip sources. Hard-coding for now.
// Additionally, I do not know according to the documentation the units accepted for the clock
// values, so for the moment those are left unvalidated.
//
#[no_mangle]
unsafe extern "C" fn rk817_init(component: *mut snd_soc_component) -> c_int {
    static int rk817_init(struct snd_soc_component *component)
    {
    struct rk817_codec_priv *rk817 = snd_soc_component_get_drvdata(component);
    snd_soc_component_write(component, RK817_CODEC_DDAC_POPD_DACST, 0x02);
    snd_soc_component_write(component, RK817_CODEC_DDAC_SR_LMT0, 0x02);
    snd_soc_component_write(component, RK817_CODEC_DADC_SR_ACL0, 0x02);
    snd_soc_component_write(component, RK817_CODEC_DTOP_VUCTIME, 0xf4);
    if (rk817.mic_in_differential) {
    snd_soc_component_update_bits(component, RK817_CODEC_AMIC_CFG0, MIC_DIFF_MASK,
    MIC_DIFF_EN);
    }
    return 0;
    }
    static int rk817_set_component_pll(struct snd_soc_component *component,
    int pll_id, int source, unsigned int freq_in,
    unsigned int freq_out)
    {
// Set resistor value and charge pump current for PLL.
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG1, 0x58);
// Set the PLL feedback clock divide value (values not documented).
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG2, 0x2d);
// Set the PLL pre-divide value (values not documented).
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG3, 0x0c);
// Set the PLL VCO output clock divide and PLL divided ratio of PLL High Clk (values not
// documented).
//
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG4, 0xa5);
    return 0;
    }
//
// DDAC/DADC L/R volume setting
// 0db~-95db, 0.375db/step, for example:
// 0x00: 0dB
// 0xff: -95dB
//
    static const DECLARE_TLV_DB_MINMAX(rk817_vol_tlv, -9500, 0);
//
// PGA GAIN L/R volume setting
// 27db~-18db, 3db/step, for example:
// 0x0: -18dB
// 0xf: 27dB
//
    static const DECLARE_TLV_DB_MINMAX(rk817_gain_tlv, -1800, 2700);
    static const struct snd_kcontrol_new rk817_volume_controls[] = {
    SOC_DOUBLE_R_RANGE_TLV("Master Playback Volume", RK817_CODEC_DDAC_VOLL,
    RK817_CODEC_DDAC_VOLR, 0, 0x00, 0xff, 1, rk817_vol_tlv),
    SOC_DOUBLE_R_RANGE_TLV("Master Capture Volume", RK817_CODEC_DADC_VOLL,
    RK817_CODEC_DADC_VOLR, 0, 0x00, 0xff, 1, rk817_vol_tlv),
    SOC_DOUBLE_TLV("Mic Capture Gain", RK817_CODEC_DMIC_PGA_GAIN, 4, 0, 0xf, 0,
    rk817_gain_tlv),
    };
// Since the speaker output and L headphone pin are internally the same, make audio path mutually
// exclusive with a mux.
//
    static const char *dac_mux_text[] = {
    "HP",
    "SPK",
    };
    static SOC_ENUM_SINGLE_VIRT_DECL(dac_enum, dac_mux_text);
    static const struct snd_kcontrol_new dac_mux =
    SOC_DAPM_ENUM("Playback Mux", dac_enum);
    static const struct snd_soc_dapm_widget rk817_dapm_widgets[] = {
// capture/playback common
    SND_SOC_DAPM_SUPPLY("LDO Regulator", RK817_CODEC_AREF_RTCFG1, 6, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("IBIAS Block", RK817_CODEC_AREF_RTCFG1, 2, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("VAvg Buffer", RK817_CODEC_AREF_RTCFG1, 1, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("PLL Power", RK817_CODEC_APLL_CFG5, 0, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S TX1 Transfer Start", RK817_CODEC_DI2S_RXCMD_TSD, 5, 0, core::ptr::null_mut(), 0),
// capture path common
    SND_SOC_DAPM_SUPPLY("ADC Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 7, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S TX Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 6, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("ADC Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 5, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S TX Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("MIC Power On", RK817_CODEC_AMIC_CFG0, 6, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S TX3 Transfer Start", RK817_CODEC_DI2S_TXCR3_TXCMD, 7, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S TX3 Right Justified", RK817_CODEC_DI2S_TXCR3_TXCMD, 3, 0, core::ptr::null_mut(), 0),
// capture path L
    SND_SOC_DAPM_ADC("ADC L", "Capture", RK817_CODEC_AADC_CFG0, 7, 1),
    SND_SOC_DAPM_SUPPLY("PGA L Power On", RK817_CODEC_AMIC_CFG0, 5, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Mic Boost L1", RK817_CODEC_AMIC_CFG0, 3, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Mic Boost L2", RK817_CODEC_AMIC_CFG0, 2, 0, core::ptr::null_mut(), 0),
// capture path R
    SND_SOC_DAPM_ADC("ADC R", "Capture", RK817_CODEC_AADC_CFG0, 6, 1),
    SND_SOC_DAPM_SUPPLY("PGA R Power On", RK817_CODEC_AMIC_CFG0, 4, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Mic Boost R1", RK817_CODEC_AMIC_CFG0, 3, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Mic Boost R2", RK817_CODEC_AMIC_CFG0, 3, 0, core::ptr::null_mut(), 0),
// playback path common
    SND_SOC_DAPM_SUPPLY("DAC Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 3, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S RX Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 2, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("DAC Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 1, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("I2S RX Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("DAC Bias", RK817_CODEC_ADAC_CFG1, 3, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("DAC Mute Off", RK817_CODEC_DDAC_MUTE_MIXCTL, 0, 1, core::ptr::null_mut(), 0),
// playback path speaker
    SND_SOC_DAPM_SUPPLY("Class D Mode", RK817_CODEC_DDAC_MUTE_MIXCTL, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("High Pass Filter", RK817_CODEC_DDAC_MUTE_MIXCTL, 7, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_DAC("SPK DAC", "Playback", RK817_CODEC_ADAC_CFG1, 2, 1),
    SND_SOC_DAPM_SUPPLY("Enable Class D", RK817_CODEC_ACLASSD_CFG1, 7, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Disable Class D Mute Ramp", RK817_CODEC_ACLASSD_CFG1, 6, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Class D Mute Rate 1", RK817_CODEC_ACLASSD_CFG1, 3, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Class D Mute Rate 2", RK817_CODEC_ACLASSD_CFG1, 2, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Class D OCPP 2", RK817_CODEC_ACLASSD_CFG2, 5, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Class D OCPP 3", RK817_CODEC_ACLASSD_CFG2, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Class D OCPN 2", RK817_CODEC_ACLASSD_CFG2, 1, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Class D OCPN 3", RK817_CODEC_ACLASSD_CFG2, 0, 0, core::ptr::null_mut(), 0),
// playback path headphones
    SND_SOC_DAPM_SUPPLY("Headphone Charge Pump", RK817_CODEC_AHP_CP, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Headphone CP Discharge LDO", RK817_CODEC_AHP_CP, 3, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Headphone OStage", RK817_CODEC_AHP_CFG0, 6, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Headphone Pre Amp", RK817_CODEC_AHP_CFG0, 5, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_DAC("DAC L", "Playback", RK817_CODEC_ADAC_CFG1, 1, 1),
    SND_SOC_DAPM_DAC("DAC R", "Playback", RK817_CODEC_ADAC_CFG1, 0, 1),
// Mux for input/output path selection
    SND_SOC_DAPM_MUX("Playback Mux", SND_SOC_NOPM, 1, 0, &dac_mux),
// Pins for Simple Card Bindings
    SND_SOC_DAPM_INPUT("MICL"),
    SND_SOC_DAPM_INPUT("MICR"),
    SND_SOC_DAPM_OUTPUT("HPOL"),
    SND_SOC_DAPM_OUTPUT("HPOR"),
    SND_SOC_DAPM_OUTPUT("SPKO"),
    };
    static const struct snd_soc_dapm_route rk817_dapm_routes[] = {
// capture path
// left mic
    {"ADC L", core::ptr::null_mut(), "LDO Regulator"},
    {"ADC L", core::ptr::null_mut(), "IBIAS Block"},
    {"ADC L", core::ptr::null_mut(), "VAvg Buffer"},
    {"ADC L", core::ptr::null_mut(), "PLL Power"},
    {"ADC L", core::ptr::null_mut(), "ADC Clock"},
    {"ADC L", core::ptr::null_mut(), "I2S TX Clock"},
    {"ADC L", core::ptr::null_mut(), "ADC Channel Enable"},
    {"ADC L", core::ptr::null_mut(), "I2S TX Channel Enable"},
    {"ADC L", core::ptr::null_mut(), "I2S TX1 Transfer Start"},
    {"MICL", core::ptr::null_mut(), "MIC Power On"},
    {"MICL", core::ptr::null_mut(), "PGA L Power On"},
    {"MICL", core::ptr::null_mut(), "Mic Boost L1"},
    {"MICL", core::ptr::null_mut(), "Mic Boost L2"},
    {"MICL", core::ptr::null_mut(), "I2S TX3 Transfer Start"},
    {"MICL", core::ptr::null_mut(), "I2S TX3 Right Justified"},
    {"ADC L", core::ptr::null_mut(), "MICL"},
// right mic
    {"ADC R", core::ptr::null_mut(), "LDO Regulator"},
    {"ADC R", core::ptr::null_mut(), "IBIAS Block"},
    {"ADC R", core::ptr::null_mut(), "VAvg Buffer"},
    {"ADC R", core::ptr::null_mut(), "PLL Power"},
    {"ADC R", core::ptr::null_mut(), "ADC Clock"},
    {"ADC R", core::ptr::null_mut(), "I2S TX Clock"},
    {"ADC R", core::ptr::null_mut(), "ADC Channel Enable"},
    {"ADC R", core::ptr::null_mut(), "I2S TX Channel Enable"},
    {"ADC R", core::ptr::null_mut(), "I2S TX1 Transfer Start"},
    {"MICR", core::ptr::null_mut(), "MIC Power On"},
    {"MICR", core::ptr::null_mut(), "PGA R Power On"},
    {"MICR", core::ptr::null_mut(), "Mic Boost R1"},
    {"MICR", core::ptr::null_mut(), "Mic Boost R2"},
    {"MICR", core::ptr::null_mut(), "I2S TX3 Transfer Start"},
    {"MICR", core::ptr::null_mut(), "I2S TX3 Right Justified"},
    {"ADC R", core::ptr::null_mut(), "MICR"},
// playback path
// speaker path
    {"SPK DAC", core::ptr::null_mut(), "LDO Regulator"},
    {"SPK DAC", core::ptr::null_mut(), "IBIAS Block"},
    {"SPK DAC", core::ptr::null_mut(), "VAvg Buffer"},
    {"SPK DAC", core::ptr::null_mut(), "PLL Power"},
    {"SPK DAC", core::ptr::null_mut(), "I2S TX1 Transfer Start"},
    {"SPK DAC", core::ptr::null_mut(), "DAC Clock"},
    {"SPK DAC", core::ptr::null_mut(), "I2S RX Clock"},
    {"SPK DAC", core::ptr::null_mut(), "DAC Channel Enable"},
    {"SPK DAC", core::ptr::null_mut(), "I2S RX Channel Enable"},
    {"SPK DAC", core::ptr::null_mut(), "Class D Mode"},
    {"SPK DAC", core::ptr::null_mut(), "DAC Bias"},
    {"SPK DAC", core::ptr::null_mut(), "DAC Mute Off"},
    {"SPK DAC", core::ptr::null_mut(), "Enable Class D"},
    {"SPK DAC", core::ptr::null_mut(), "Disable Class D Mute Ramp"},
    {"SPK DAC", core::ptr::null_mut(), "Class D Mute Rate 1"},
    {"SPK DAC", core::ptr::null_mut(), "Class D Mute Rate 2"},
    {"SPK DAC", core::ptr::null_mut(), "Class D OCPP 2"},
    {"SPK DAC", core::ptr::null_mut(), "Class D OCPP 3"},
    {"SPK DAC", core::ptr::null_mut(), "Class D OCPN 2"},
    {"SPK DAC", core::ptr::null_mut(), "Class D OCPN 3"},
    {"SPK DAC", core::ptr::null_mut(), "High Pass Filter"},
// headphone path L
    {"DAC L", core::ptr::null_mut(), "LDO Regulator"},
    {"DAC L", core::ptr::null_mut(), "IBIAS Block"},
    {"DAC L", core::ptr::null_mut(), "VAvg Buffer"},
    {"DAC L", core::ptr::null_mut(), "PLL Power"},
    {"DAC L", core::ptr::null_mut(), "I2S TX1 Transfer Start"},
    {"DAC L", core::ptr::null_mut(), "DAC Clock"},
    {"DAC L", core::ptr::null_mut(), "I2S RX Clock"},
    {"DAC L", core::ptr::null_mut(), "DAC Channel Enable"},
    {"DAC L", core::ptr::null_mut(), "I2S RX Channel Enable"},
    {"DAC L", core::ptr::null_mut(), "DAC Bias"},
    {"DAC L", core::ptr::null_mut(), "DAC Mute Off"},
    {"DAC L", core::ptr::null_mut(), "Headphone Charge Pump"},
    {"DAC L", core::ptr::null_mut(), "Headphone CP Discharge LDO"},
    {"DAC L", core::ptr::null_mut(), "Headphone OStage"},
    {"DAC L", core::ptr::null_mut(), "Headphone Pre Amp"},
// headphone path R
    {"DAC R", core::ptr::null_mut(), "LDO Regulator"},
    {"DAC R", core::ptr::null_mut(), "IBIAS Block"},
    {"DAC R", core::ptr::null_mut(), "VAvg Buffer"},
    {"DAC R", core::ptr::null_mut(), "PLL Power"},
    {"DAC R", core::ptr::null_mut(), "I2S TX1 Transfer Start"},
    {"DAC R", core::ptr::null_mut(), "DAC Clock"},
    {"DAC R", core::ptr::null_mut(), "I2S RX Clock"},
    {"DAC R", core::ptr::null_mut(), "DAC Channel Enable"},
    {"DAC R", core::ptr::null_mut(), "I2S RX Channel Enable"},
    {"DAC R", core::ptr::null_mut(), "DAC Bias"},
    {"DAC R", core::ptr::null_mut(), "DAC Mute Off"},
    {"DAC R", core::ptr::null_mut(), "Headphone Charge Pump"},
    {"DAC R", core::ptr::null_mut(), "Headphone CP Discharge LDO"},
    {"DAC R", core::ptr::null_mut(), "Headphone OStage"},
    {"DAC R", core::ptr::null_mut(), "Headphone Pre Amp"},
// mux path for output selection
    {"Playback Mux", "HP", "DAC L"},
    {"Playback Mux", "HP", "DAC R"},
    {"Playback Mux", "SPK", "SPK DAC"},
    {"SPKO", core::ptr::null_mut(), "Playback Mux"},
    {"HPOL", core::ptr::null_mut(), "Playback Mux"},
    {"HPOR", core::ptr::null_mut(), "Playback Mux"},
    };
    static int rk817_set_dai_sysclk(struct snd_soc_dai *codec_dai,
    int clk_id, unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct rk817_codec_priv *rk817 = snd_soc_component_get_drvdata(component);
    rk817.stereo_sysclk = freq;
    return 0;
    }
    static int rk817_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int fmt)
    {
    struct snd_soc_component *component = codec_dai.component;
    let mut i2s_mst: c_uint = 0;
    switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
    case SND_SOC_DAIFMT_CBC_CFC:
    i2s_mst |= RK817_I2S_MODE_SLV;
    break;
    case SND_SOC_DAIFMT_CBP_CFP:
    i2s_mst |= RK817_I2S_MODE_MST;
    break;
    default:
    dev_err(component.dev, "%s : set master mask failed!\n", __func__);
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, RK817_CODEC_DI2S_CKM,
    RK817_I2S_MODE_MASK, i2s_mst);
    return 0;
    }
    static int rk817_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    snd_soc_component_write(component, RK817_CODEC_DI2S_RXCR2,
    VDW_RX_16BITS);
    snd_soc_component_write(component, RK817_CODEC_DI2S_TXCR2,
    VDW_TX_16BITS);
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    case SNDRV_PCM_FORMAT_S32_LE:
    snd_soc_component_write(component, RK817_CODEC_DI2S_RXCR2,
    VDW_RX_24BITS);
    snd_soc_component_write(component, RK817_CODEC_DI2S_TXCR2,
    VDW_TX_24BITS);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk817_digital_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int rk817_digital_mute(struct snd_soc_dai *dai, int mute, int stream)
    {
    struct snd_soc_component *component = dai.component;
    if (mute)
    snd_soc_component_update_bits(component,
    RK817_CODEC_DDAC_MUTE_MIXCTL,
    DACMT_MASK, DACMT_ENABLE);
    else
    snd_soc_component_update_bits(component,
    RK817_CODEC_DDAC_MUTE_MIXCTL,
    DACMT_MASK, DACMT_DISABLE);
    return 0;
    }

    SNDRV_PCM_RATE_16000 |	\
    SNDRV_PCM_RATE_32000 |	\
    SNDRV_PCM_RATE_44100 |	\
    SNDRV_PCM_RATE_48000 |	\
    SNDRV_PCM_RATE_96000)

    SNDRV_PCM_RATE_16000 |	\
    SNDRV_PCM_RATE_32000 |	\
    SNDRV_PCM_RATE_44100 |	\
    SNDRV_PCM_RATE_48000 |	\
    SNDRV_PCM_RATE_96000)

    SNDRV_PCM_FMTBIT_S20_3LE |\
    SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S32_LE)
    static const struct snd_soc_dai_ops rk817_dai_ops = {
    .hw_params	= rk817_hw_params,
    .set_fmt	= rk817_set_dai_fmt,
    .set_sysclk	= rk817_set_dai_sysclk,
    .mute_stream	= rk817_digital_mute,
    .no_capture_mute	= 1,
    };
    static struct snd_soc_dai_driver rk817_dai[] = {
    {
    .name = "rk817-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 8,
    .rates = RK817_PLAYBACK_RATES,
    .formats = RK817_FORMATS,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = RK817_CAPTURE_RATES,
    .formats = RK817_FORMATS,
    },
    .ops = &rk817_dai_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn rk817_probe(component: *mut snd_soc_component) -> c_int {
    static int rk817_probe(struct snd_soc_component *component)
    {
    struct rk817_codec_priv *rk817 = snd_soc_component_get_drvdata(component);
    struct rk808 *rk808 = dev_get_drvdata(component.dev.parent);
    snd_soc_component_init_regmap(component, rk808.regmap);
    rk817.component = component;
    snd_soc_component_write(component, RK817_CODEC_DTOP_LPT_SRST, 0x40);
    rk817_init(component);
// setting initial pll values so that we can continue to leverage simple-audio-card.
// The values aren't important since no parameters are used.
//
    snd_soc_component_set_pll(component, 0, 0, 0, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk817_remove(component: *mut snd_soc_component) {
    static void rk817_remove(struct snd_soc_component *component)
    {
    snd_soc_component_exit_regmap(component);
    }
    static const struct snd_soc_component_driver soc_codec_dev_rk817 = {
    .probe = rk817_probe,
    .remove = rk817_remove,
    .idle_bias_on = 1,
    .use_pmdown_time = 1,
    .endianness = 1,
    .controls = rk817_volume_controls,
    .num_controls = ARRAY_SIZE(rk817_volume_controls),
    .dapm_routes = rk817_dapm_routes,
    .num_dapm_routes = ARRAY_SIZE(rk817_dapm_routes),
    .dapm_widgets = rk817_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(rk817_dapm_widgets),
    .set_pll = rk817_set_component_pll,
    };
    static void rk817_codec_parse_dt_property(struct device *dev,
    struct rk817_codec_priv *rk817)
    {
    struct device_node *node;
    node = of_get_child_by_name(dev.parent.of_node, "codec");
    if (!node) {
    dev_dbg(dev, "%s() Can not get child: codec\n",
    __func__);
    }
    rk817.mic_in_differential =
    of_property_read_bool(node, "rockchip,mic-in-differential");
    of_node_put(node);
    }
#[no_mangle]
unsafe extern "C" fn rk817_platform_probe(pdev: *mut platform_device) -> c_int {
    static int rk817_platform_probe(struct platform_device *pdev)
    {
    struct rk808 *rk808 = dev_get_drvdata(pdev.dev.parent);
    struct rk817_codec_priv *rk817_codec_data;
    int ret;
    rk817_codec_data = devm_kzalloc(&pdev.dev,
    sizeof(struct rk817_codec_priv),
    GFP_KERNEL);
    if (!rk817_codec_data)
    return -ENOMEM;
    platform_set_drvdata(pdev, rk817_codec_data);
    rk817_codec_data.rk808 = rk808;
    rk817_codec_parse_dt_property(&pdev.dev, rk817_codec_data);
    rk817_codec_data.mclk = devm_clk_get(pdev.dev.parent, "mclk");
    if (IS_ERR(rk817_codec_data.mclk)) {
    dev_dbg(&pdev.dev, "Unable to get mclk\n");
    ret = -ENXIO;
    goto err_;
    }
    ret = clk_prepare_enable(rk817_codec_data.mclk);
    if (ret < 0) {
    dev_err(&pdev.dev, "%s() clock prepare error %d\n",
    __func__, ret);
    goto err_;
    }
    ret = devm_snd_soc_register_component(&pdev.dev, &soc_codec_dev_rk817,
    rk817_dai, ARRAY_SIZE(rk817_dai));
    if (ret < 0) {
    dev_err(&pdev.dev, "%s() register codec error %d\n",
    __func__, ret);
    goto err_clk;
    }
    return 0;
    err_clk:
    clk_disable_unprepare(rk817_codec_data.mclk);
    err_:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rk817_platform_remove(pdev: *mut platform_device) {
    static void rk817_platform_remove(struct platform_device *pdev)
    {
    struct rk817_codec_priv *rk817 = platform_get_drvdata(pdev);
    clk_disable_unprepare(rk817.mclk);
    }
    static struct platform_driver rk817_codec_driver = {
    .driver = {
    .name = "rk817-codec",
    },
    .probe = rk817_platform_probe,
    .remove = rk817_platform_remove,
    };
    module_platform_driver(rk817_codec_driver);
    MODULE_DESCRIPTION("ASoC RK817 codec driver");
    MODULE_AUTHOR("binyuan <kevan.lan@rock-chips.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:rk817-codec");
