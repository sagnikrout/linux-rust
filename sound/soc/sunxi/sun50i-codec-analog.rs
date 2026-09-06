//! Automatically rewritten from C to Rust
//! Source: sound/soc/sunxi/sun50i-codec-analog.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// This driver supports the analog controls for the internal codec
// found in Allwinner's A64 SoC.
//
// Copyright (C) 2016 Chen-Yu Tsai <wens@csie.org>
// Copyright (C) 2017 Marcus Cooper <codekipper@gmail.com>
// Copyright (C) 2018 Vasily Khoruzhick <anarsoul@gmail.com>
//
// Based on sun8i-codec-analog.c
//

// Codec analog control register offsets and bit fields
pub const SUN50I_ADDA_HP_CTRL: c_uint = 0x00;
pub const SUN50I_ADDA_HP_CTRL_PA_CLK_GATE: c_int = 7;
pub const SUN50I_ADDA_HP_CTRL_HPPA_EN: c_int = 6;
pub const SUN50I_ADDA_HP_CTRL_HPVOL: c_int = 0;
pub const SUN50I_ADDA_OL_MIX_CTRL: c_uint = 0x01;
pub const SUN50I_ADDA_OL_MIX_CTRL_MIC1: c_int = 6;
pub const SUN50I_ADDA_OL_MIX_CTRL_MIC2: c_int = 5;
pub const SUN50I_ADDA_OL_MIX_CTRL_PHONE: c_int = 4;
pub const SUN50I_ADDA_OL_MIX_CTRL_PHONEN: c_int = 3;
pub const SUN50I_ADDA_OL_MIX_CTRL_LINEINL: c_int = 2;
pub const SUN50I_ADDA_OL_MIX_CTRL_DACL: c_int = 1;
pub const SUN50I_ADDA_OL_MIX_CTRL_DACR: c_int = 0;
pub const SUN50I_ADDA_OR_MIX_CTRL: c_uint = 0x02;
pub const SUN50I_ADDA_OR_MIX_CTRL_MIC1: c_int = 6;
pub const SUN50I_ADDA_OR_MIX_CTRL_MIC2: c_int = 5;
pub const SUN50I_ADDA_OR_MIX_CTRL_PHONE: c_int = 4;
pub const SUN50I_ADDA_OR_MIX_CTRL_PHONEP: c_int = 3;
pub const SUN50I_ADDA_OR_MIX_CTRL_LINEINR: c_int = 2;
pub const SUN50I_ADDA_OR_MIX_CTRL_DACR: c_int = 1;
pub const SUN50I_ADDA_OR_MIX_CTRL_DACL: c_int = 0;
pub const SUN50I_ADDA_EARPIECE_CTRL0: c_uint = 0x03;
pub const SUN50I_ADDA_EARPIECE_CTRL0_EAR_RAMP_TIME: c_int = 4;
pub const SUN50I_ADDA_EARPIECE_CTRL0_ESPSR: c_int = 0;
pub const SUN50I_ADDA_EARPIECE_CTRL1: c_uint = 0x04;
pub const SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_EN: c_int = 7;
pub const SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_MUTE: c_int = 6;
pub const SUN50I_ADDA_EARPIECE_CTRL1_ESP_VOL: c_int = 0;
pub const SUN50I_ADDA_LINEOUT_CTRL0: c_uint = 0x05;
pub const SUN50I_ADDA_LINEOUT_CTRL0_LEN: c_int = 7;
pub const SUN50I_ADDA_LINEOUT_CTRL0_REN: c_int = 6;
pub const SUN50I_ADDA_LINEOUT_CTRL0_LSRC_SEL: c_int = 5;
pub const SUN50I_ADDA_LINEOUT_CTRL0_RSRC_SEL: c_int = 4;
pub const SUN50I_ADDA_LINEOUT_CTRL1: c_uint = 0x06;
pub const SUN50I_ADDA_LINEOUT_CTRL1_VOL: c_int = 0;
pub const SUN50I_ADDA_MIC1_CTRL: c_uint = 0x07;
pub const SUN50I_ADDA_MIC1_CTRL_MIC1G: c_int = 4;
pub const SUN50I_ADDA_MIC1_CTRL_MIC1AMPEN: c_int = 3;
pub const SUN50I_ADDA_MIC1_CTRL_MIC1BOOST: c_int = 0;
pub const SUN50I_ADDA_MIC2_CTRL: c_uint = 0x08;
pub const SUN50I_ADDA_MIC2_CTRL_MIC2G: c_int = 4;
pub const SUN50I_ADDA_MIC2_CTRL_MIC2AMPEN: c_int = 3;
pub const SUN50I_ADDA_MIC2_CTRL_MIC2BOOST: c_int = 0;
pub const SUN50I_ADDA_LINEIN_CTRL: c_uint = 0x09;
pub const SUN50I_ADDA_LINEIN_CTRL_LINEING: c_int = 0;
pub const SUN50I_ADDA_MIX_DAC_CTRL: c_uint = 0x0a;
pub const SUN50I_ADDA_MIX_DAC_CTRL_DACAREN: c_int = 7;
pub const SUN50I_ADDA_MIX_DAC_CTRL_DACALEN: c_int = 6;
pub const SUN50I_ADDA_MIX_DAC_CTRL_RMIXEN: c_int = 5;
pub const SUN50I_ADDA_MIX_DAC_CTRL_LMIXEN: c_int = 4;
pub const SUN50I_ADDA_MIX_DAC_CTRL_RHPPAMUTE: c_int = 3;
pub const SUN50I_ADDA_MIX_DAC_CTRL_LHPPAMUTE: c_int = 2;
pub const SUN50I_ADDA_MIX_DAC_CTRL_RHPIS: c_int = 1;
pub const SUN50I_ADDA_MIX_DAC_CTRL_LHPIS: c_int = 0;
pub const SUN50I_ADDA_L_ADCMIX_SRC: c_uint = 0x0b;
pub const SUN50I_ADDA_L_ADCMIX_SRC_MIC1: c_int = 6;
pub const SUN50I_ADDA_L_ADCMIX_SRC_MIC2: c_int = 5;
pub const SUN50I_ADDA_L_ADCMIX_SRC_PHONE: c_int = 4;
pub const SUN50I_ADDA_L_ADCMIX_SRC_PHONEN: c_int = 3;
pub const SUN50I_ADDA_L_ADCMIX_SRC_LINEINL: c_int = 2;
pub const SUN50I_ADDA_L_ADCMIX_SRC_OMIXRL: c_int = 1;
pub const SUN50I_ADDA_L_ADCMIX_SRC_OMIXRR: c_int = 0;
pub const SUN50I_ADDA_R_ADCMIX_SRC: c_uint = 0x0c;
pub const SUN50I_ADDA_R_ADCMIX_SRC_MIC1: c_int = 6;
pub const SUN50I_ADDA_R_ADCMIX_SRC_MIC2: c_int = 5;
pub const SUN50I_ADDA_R_ADCMIX_SRC_PHONE: c_int = 4;
pub const SUN50I_ADDA_R_ADCMIX_SRC_PHONEP: c_int = 3;
pub const SUN50I_ADDA_R_ADCMIX_SRC_LINEINR: c_int = 2;
pub const SUN50I_ADDA_R_ADCMIX_SRC_OMIXR: c_int = 1;
pub const SUN50I_ADDA_R_ADCMIX_SRC_OMIXL: c_int = 0;
pub const SUN50I_ADDA_ADC_CTRL: c_uint = 0x0d;
pub const SUN50I_ADDA_ADC_CTRL_ADCREN: c_int = 7;
pub const SUN50I_ADDA_ADC_CTRL_ADCLEN: c_int = 6;
pub const SUN50I_ADDA_ADC_CTRL_ADCG: c_int = 0;
pub const SUN50I_ADDA_HS_MBIAS_CTRL: c_uint = 0x0e;
pub const SUN50I_ADDA_HS_MBIAS_CTRL_MMICBIASEN: c_int = 7;
pub const SUN50I_ADDA_MDET_CTRL: c_uint = 0x1c;
pub const SUN50I_ADDA_MDET_CTRL_SELDETADC_FS: c_int = 4;
pub const SUN50I_ADDA_MDET_CTRL_SELDETADC_DB: c_int = 2;
pub const SUN50I_ADDA_MDET_CTRL_SELDETADC_BF: c_int = 0;
pub const SUN50I_ADDA_JACK_MIC_CTRL: c_uint = 0x1d;
pub const SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN: c_int = 7;
pub const SUN50I_ADDA_JACK_MIC_CTRL_INNERRESEN: c_int = 6;
pub const SUN50I_ADDA_JACK_MIC_CTRL_HMICBIASEN: c_int = 5;
pub const SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN: c_int = 4;
// mixer controls
    static const struct snd_kcontrol_new sun50i_a64_codec_mixer_controls[] = {
    SOC_DAPM_DOUBLE_R("Mic1 Playback Switch",
    SUN50I_ADDA_OL_MIX_CTRL,
    SUN50I_ADDA_OR_MIX_CTRL,
    SUN50I_ADDA_OL_MIX_CTRL_MIC1, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic2 Playback Switch",
    SUN50I_ADDA_OL_MIX_CTRL,
    SUN50I_ADDA_OR_MIX_CTRL,
    SUN50I_ADDA_OL_MIX_CTRL_MIC2, 1, 0),
    SOC_DAPM_DOUBLE_R("Line In Playback Switch",
    SUN50I_ADDA_OL_MIX_CTRL,
    SUN50I_ADDA_OR_MIX_CTRL,
    SUN50I_ADDA_OL_MIX_CTRL_LINEINL, 1, 0),
    SOC_DAPM_DOUBLE_R("DAC Playback Switch",
    SUN50I_ADDA_OL_MIX_CTRL,
    SUN50I_ADDA_OR_MIX_CTRL,
    SUN50I_ADDA_OL_MIX_CTRL_DACL, 1, 0),
    SOC_DAPM_DOUBLE_R("DAC Reversed Playback Switch",
    SUN50I_ADDA_OL_MIX_CTRL,
    SUN50I_ADDA_OR_MIX_CTRL,
    SUN50I_ADDA_OL_MIX_CTRL_DACR, 1, 0),
    };
// ADC mixer controls
    static const struct snd_kcontrol_new sun50i_codec_adc_mixer_controls[] = {
    SOC_DAPM_DOUBLE_R("Mic1 Capture Switch",
    SUN50I_ADDA_L_ADCMIX_SRC,
    SUN50I_ADDA_R_ADCMIX_SRC,
    SUN50I_ADDA_L_ADCMIX_SRC_MIC1, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic2 Capture Switch",
    SUN50I_ADDA_L_ADCMIX_SRC,
    SUN50I_ADDA_R_ADCMIX_SRC,
    SUN50I_ADDA_L_ADCMIX_SRC_MIC2, 1, 0),
    SOC_DAPM_DOUBLE_R("Line In Capture Switch",
    SUN50I_ADDA_L_ADCMIX_SRC,
    SUN50I_ADDA_R_ADCMIX_SRC,
    SUN50I_ADDA_L_ADCMIX_SRC_LINEINL, 1, 0),
    SOC_DAPM_DOUBLE_R("Mixer Capture Switch",
    SUN50I_ADDA_L_ADCMIX_SRC,
    SUN50I_ADDA_R_ADCMIX_SRC,
    SUN50I_ADDA_L_ADCMIX_SRC_OMIXRL, 1, 0),
    SOC_DAPM_DOUBLE_R("Mixer Reversed Capture Switch",
    SUN50I_ADDA_L_ADCMIX_SRC,
    SUN50I_ADDA_R_ADCMIX_SRC,
    SUN50I_ADDA_L_ADCMIX_SRC_OMIXRR, 1, 0),
    };
    static const DECLARE_TLV_DB_SCALE(sun50i_codec_out_mixer_pregain_scale,
    -450, 150, 0);
    static const DECLARE_TLV_DB_RANGE(sun50i_codec_mic_gain_scale,
    0, 0, TLV_DB_SCALE_ITEM(0, 0, 0),
    1, 7, TLV_DB_SCALE_ITEM(2400, 300, 0),
    );
    static const DECLARE_TLV_DB_SCALE(sun50i_codec_hp_vol_scale, -6300, 100, 1);
    static const DECLARE_TLV_DB_RANGE(sun50i_codec_lineout_vol_scale,
    0, 1, TLV_DB_SCALE_ITEM(TLV_DB_GAIN_MUTE, 0, 1),
    2, 31, TLV_DB_SCALE_ITEM(-4350, 150, 0),
    );
    static const DECLARE_TLV_DB_RANGE(sun50i_codec_earpiece_vol_scale,
    0, 1, TLV_DB_SCALE_ITEM(TLV_DB_GAIN_MUTE, 0, 1),
    2, 31, TLV_DB_SCALE_ITEM(-4350, 150, 0),
    );
// volume / mute controls
    static const struct snd_kcontrol_new sun50i_a64_codec_controls[] = {
    SOC_SINGLE_TLV("Headphone Playback Volume",
    SUN50I_ADDA_HP_CTRL,
    SUN50I_ADDA_HP_CTRL_HPVOL, 0x3f, 0,
    sun50i_codec_hp_vol_scale),
// Mixer pre-gain
    SOC_SINGLE_TLV("Mic1 Playback Volume", SUN50I_ADDA_MIC1_CTRL,
    SUN50I_ADDA_MIC1_CTRL_MIC1G,
    0x7, 0, sun50i_codec_out_mixer_pregain_scale),
// Microphone Amp boost gain
    SOC_SINGLE_TLV("Mic1 Boost Volume", SUN50I_ADDA_MIC1_CTRL,
    SUN50I_ADDA_MIC1_CTRL_MIC1BOOST, 0x7, 0,
    sun50i_codec_mic_gain_scale),
// Mixer pre-gain
    SOC_SINGLE_TLV("Mic2 Playback Volume",
    SUN50I_ADDA_MIC2_CTRL, SUN50I_ADDA_MIC2_CTRL_MIC2G,
    0x7, 0, sun50i_codec_out_mixer_pregain_scale),
// Microphone Amp boost gain
    SOC_SINGLE_TLV("Mic2 Boost Volume", SUN50I_ADDA_MIC2_CTRL,
    SUN50I_ADDA_MIC2_CTRL_MIC2BOOST, 0x7, 0,
    sun50i_codec_mic_gain_scale),
// ADC
    SOC_SINGLE_TLV("ADC Gain Capture Volume", SUN50I_ADDA_ADC_CTRL,
    SUN50I_ADDA_ADC_CTRL_ADCG, 0x7, 0,
    sun50i_codec_out_mixer_pregain_scale),
// Mixer pre-gain
    SOC_SINGLE_TLV("Line In Playback Volume", SUN50I_ADDA_LINEIN_CTRL,
    SUN50I_ADDA_LINEIN_CTRL_LINEING,
    0x7, 0, sun50i_codec_out_mixer_pregain_scale),
    SOC_SINGLE_TLV("Line Out Playback Volume",
    SUN50I_ADDA_LINEOUT_CTRL1,
    SUN50I_ADDA_LINEOUT_CTRL1_VOL, 0x1f, 0,
    sun50i_codec_lineout_vol_scale),
    SOC_SINGLE_TLV("Earpiece Playback Volume",
    SUN50I_ADDA_EARPIECE_CTRL1,
    SUN50I_ADDA_EARPIECE_CTRL1_ESP_VOL, 0x1f, 0,
    sun50i_codec_earpiece_vol_scale),
    };
    static const char * const sun50i_codec_hp_src_enum_text[] = {
    "DAC", "Mixer",
    };
    static SOC_ENUM_DOUBLE_DECL(sun50i_codec_hp_src_enum,
    SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_LHPIS,
    SUN50I_ADDA_MIX_DAC_CTRL_RHPIS,
    sun50i_codec_hp_src_enum_text);
    static const struct snd_kcontrol_new sun50i_codec_hp_src[] = {
    SOC_DAPM_ENUM("Headphone Source Playback Route",
    sun50i_codec_hp_src_enum),
    };
    static const struct snd_kcontrol_new sun50i_codec_hp_switch =
    SOC_DAPM_DOUBLE("Headphone Playback Switch",
    SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_LHPPAMUTE,
    SUN50I_ADDA_MIX_DAC_CTRL_RHPPAMUTE, 1, 0);
    static const char * const sun50i_codec_lineout_src_enum_text[] = {
    "Stereo", "Mono Differential",
    };
    static SOC_ENUM_DOUBLE_DECL(sun50i_codec_lineout_src_enum,
    SUN50I_ADDA_LINEOUT_CTRL0,
    SUN50I_ADDA_LINEOUT_CTRL0_LSRC_SEL,
    SUN50I_ADDA_LINEOUT_CTRL0_RSRC_SEL,
    sun50i_codec_lineout_src_enum_text);
    static const struct snd_kcontrol_new sun50i_codec_lineout_src[] = {
    SOC_DAPM_ENUM("Line Out Source Playback Route",
    sun50i_codec_lineout_src_enum),
    };
    static const struct snd_kcontrol_new sun50i_codec_lineout_switch =
    SOC_DAPM_DOUBLE("Line Out Playback Switch",
    SUN50I_ADDA_LINEOUT_CTRL0,
    SUN50I_ADDA_LINEOUT_CTRL0_LEN,
    SUN50I_ADDA_LINEOUT_CTRL0_REN, 1, 0);
    static const char * const sun50i_codec_earpiece_src_enum_text[] = {
    "DACR", "DACL", "Right Mixer", "Left Mixer",
    };
    static SOC_ENUM_SINGLE_DECL(sun50i_codec_earpiece_src_enum,
    SUN50I_ADDA_EARPIECE_CTRL0,
    SUN50I_ADDA_EARPIECE_CTRL0_ESPSR,
    sun50i_codec_earpiece_src_enum_text);
    static const struct snd_kcontrol_new sun50i_codec_earpiece_src[] = {
    SOC_DAPM_ENUM("Earpiece Source Playback Route",
    sun50i_codec_earpiece_src_enum),
    };
    static const struct snd_kcontrol_new sun50i_codec_earpiece_switch[] = {
    SOC_DAPM_SINGLE("Earpiece Playback Switch",
    SUN50I_ADDA_EARPIECE_CTRL1,
    SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_MUTE, 1, 0),
    };
    static int sun50i_codec_hbias_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    let mut value: u32 = !!SND_SOC_DAPM_EVENT_ON(event);
    regmap_update_bits(component.regmap, SUN50I_ADDA_JACK_MIC_CTRL,
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN),
    value << SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN);
    return 0;
    }
    static const struct snd_soc_dapm_widget sun50i_a64_codec_widgets[] = {
// DAC
    SND_SOC_DAPM_DAC("Left DAC", core::ptr::null_mut(), SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_DACALEN, 0),
    SND_SOC_DAPM_DAC("Right DAC", core::ptr::null_mut(), SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_DACAREN, 0),
// ADC
    SND_SOC_DAPM_ADC("Left ADC", core::ptr::null_mut(), SUN50I_ADDA_ADC_CTRL,
    SUN50I_ADDA_ADC_CTRL_ADCLEN, 0),
    SND_SOC_DAPM_ADC("Right ADC", core::ptr::null_mut(), SUN50I_ADDA_ADC_CTRL,
    SUN50I_ADDA_ADC_CTRL_ADCREN, 0),
//
// Due to this component and the codec belonging to separate DAPM
// contexts, we need to manually link the above widgets to their
// stream widgets at the card level.
//
    SND_SOC_DAPM_REGULATOR_SUPPLY("cpvdd", 0, 0),
    SND_SOC_DAPM_MUX("Left Headphone Source",
    SND_SOC_NOPM, 0, 0, sun50i_codec_hp_src),
    SND_SOC_DAPM_MUX("Right Headphone Source",
    SND_SOC_NOPM, 0, 0, sun50i_codec_hp_src),
    SND_SOC_DAPM_SWITCH("Left Headphone Switch",
    SND_SOC_NOPM, 0, 0, &sun50i_codec_hp_switch),
    SND_SOC_DAPM_SWITCH("Right Headphone Switch",
    SND_SOC_NOPM, 0, 0, &sun50i_codec_hp_switch),
    SND_SOC_DAPM_OUT_DRV("Left Headphone Amp",
    SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUT_DRV("Right Headphone Amp",
    SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Headphone Amp", SUN50I_ADDA_HP_CTRL,
    SUN50I_ADDA_HP_CTRL_HPPA_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("HP"),
    SND_SOC_DAPM_MUX("Left Line Out Source",
    SND_SOC_NOPM, 0, 0, sun50i_codec_lineout_src),
    SND_SOC_DAPM_MUX("Right Line Out Source",
    SND_SOC_NOPM, 0, 0, sun50i_codec_lineout_src),
    SND_SOC_DAPM_SWITCH("Left Line Out Switch",
    SND_SOC_NOPM, 0, 0, &sun50i_codec_lineout_switch),
    SND_SOC_DAPM_SWITCH("Right Line Out Switch",
    SND_SOC_NOPM, 0, 0, &sun50i_codec_lineout_switch),
    SND_SOC_DAPM_OUTPUT("LINEOUT"),
    SND_SOC_DAPM_MUX("Earpiece Source Playback Route",
    SND_SOC_NOPM, 0, 0, sun50i_codec_earpiece_src),
    SOC_MIXER_NAMED_CTL_ARRAY("Earpiece Switch",
    SND_SOC_NOPM, 0, 0,
    sun50i_codec_earpiece_switch),
    SND_SOC_DAPM_OUT_DRV("Earpiece Amp", SUN50I_ADDA_EARPIECE_CTRL1,
    SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("EARPIECE"),
// Microphone inputs
    SND_SOC_DAPM_INPUT("MIC1"),
// Microphone Bias
    SND_SOC_DAPM_SUPPLY("MBIAS", SUN50I_ADDA_HS_MBIAS_CTRL,
    SUN50I_ADDA_HS_MBIAS_CTRL_MMICBIASEN,
    0, core::ptr::null_mut(), 0),
// Mic input path
    SND_SOC_DAPM_PGA("Mic1 Amplifier", SUN50I_ADDA_MIC1_CTRL,
    SUN50I_ADDA_MIC1_CTRL_MIC1AMPEN, 0, core::ptr::null_mut(), 0),
// Microphone input
    SND_SOC_DAPM_INPUT("MIC2"),
// Microphone Bias
    SND_SOC_DAPM_SUPPLY("HBIAS", SUN50I_ADDA_JACK_MIC_CTRL,
    SUN50I_ADDA_JACK_MIC_CTRL_HMICBIASEN,
    0, sun50i_codec_hbias_event,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
// Mic input path
    SND_SOC_DAPM_PGA("Mic2 Amplifier", SUN50I_ADDA_MIC2_CTRL,
    SUN50I_ADDA_MIC2_CTRL_MIC2AMPEN, 0, core::ptr::null_mut(), 0),
// Line input
    SND_SOC_DAPM_INPUT("LINEIN"),
// Mixers
    SND_SOC_DAPM_MIXER("Left Mixer", SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_LMIXEN, 0,
    sun50i_a64_codec_mixer_controls,
    ARRAY_SIZE(sun50i_a64_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("Right Mixer", SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_RMIXEN, 0,
    sun50i_a64_codec_mixer_controls,
    ARRAY_SIZE(sun50i_a64_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("Left ADC Mixer", SND_SOC_NOPM, 0, 0,
    sun50i_codec_adc_mixer_controls,
    ARRAY_SIZE(sun50i_codec_adc_mixer_controls)),
    SND_SOC_DAPM_MIXER("Right ADC Mixer", SND_SOC_NOPM, 0, 0,
    sun50i_codec_adc_mixer_controls,
    ARRAY_SIZE(sun50i_codec_adc_mixer_controls)),
    };
    static const struct snd_soc_dapm_route sun50i_a64_codec_routes[] = {
// Left Mixer Routes
    { "Left Mixer", "Mic1 Playback Switch", "Mic1 Amplifier" },
    { "Left Mixer", "Mic2 Playback Switch", "Mic2 Amplifier" },
    { "Left Mixer", "Line In Playback Switch", "LINEIN" },
    { "Left Mixer", "DAC Playback Switch", "Left DAC" },
    { "Left Mixer", "DAC Reversed Playback Switch", "Right DAC" },
// Right Mixer Routes
    { "Right Mixer", "Mic1 Playback Switch", "Mic1 Amplifier" },
    { "Right Mixer", "Mic2 Playback Switch", "Mic2 Amplifier" },
    { "Right Mixer", "Line In Playback Switch", "LINEIN" },
    { "Right Mixer", "DAC Playback Switch", "Right DAC" },
    { "Right Mixer", "DAC Reversed Playback Switch", "Left DAC" },
// Left ADC Mixer Routes
    { "Left ADC Mixer", "Mic1 Capture Switch", "Mic1 Amplifier" },
    { "Left ADC Mixer", "Mic2 Capture Switch", "Mic2 Amplifier" },
    { "Left ADC Mixer", "Line In Capture Switch", "LINEIN" },
    { "Left ADC Mixer", "Mixer Capture Switch", "Left Mixer" },
    { "Left ADC Mixer", "Mixer Reversed Capture Switch", "Right Mixer" },
// Right ADC Mixer Routes
    { "Right ADC Mixer", "Mic1 Capture Switch", "Mic1 Amplifier" },
    { "Right ADC Mixer", "Mic2 Capture Switch", "Mic2 Amplifier" },
    { "Right ADC Mixer", "Line In Capture Switch", "LINEIN" },
    { "Right ADC Mixer", "Mixer Capture Switch", "Right Mixer" },
    { "Right ADC Mixer", "Mixer Reversed Capture Switch", "Left Mixer" },
// ADC Routes
    { "Left ADC", core::ptr::null_mut(), "Left ADC Mixer" },
    { "Right ADC", core::ptr::null_mut(), "Right ADC Mixer" },
// Headphone Routes
    { "Left Headphone Source", "DAC", "Left DAC" },
    { "Left Headphone Source", "Mixer", "Left Mixer" },
    { "Left Headphone Switch", "Headphone Playback Switch", "Left Headphone Source" },
    { "Left Headphone Amp", core::ptr::null_mut(), "Left Headphone Switch" },
    { "Left Headphone Amp", core::ptr::null_mut(), "Headphone Amp" },
    { "HP", core::ptr::null_mut(), "Left Headphone Amp" },
    { "Right Headphone Source", "DAC", "Right DAC" },
    { "Right Headphone Source", "Mixer", "Right Mixer" },
    { "Right Headphone Switch", "Headphone Playback Switch", "Right Headphone Source" },
    { "Right Headphone Amp", core::ptr::null_mut(), "Right Headphone Switch" },
    { "Right Headphone Amp", core::ptr::null_mut(), "Headphone Amp" },
    { "HP", core::ptr::null_mut(), "Right Headphone Amp" },
    { "Headphone Amp", core::ptr::null_mut(), "cpvdd" },
// Microphone Routes
    { "Mic1 Amplifier", core::ptr::null_mut(), "MIC1"},
// Microphone Routes
    { "Mic2 Amplifier", core::ptr::null_mut(), "MIC2"},
// Line-out Routes
    { "Left Line Out Source", "Stereo", "Left Mixer" },
    { "Left Line Out Source", "Mono Differential", "Left Mixer" },
    { "Left Line Out Source", "Mono Differential", "Right Mixer" },
    { "Left Line Out Switch", "Line Out Playback Switch", "Left Line Out Source" },
    { "LINEOUT", core::ptr::null_mut(), "Left Line Out Switch" },
    { "Right Line Out Switch", "Line Out Playback Switch", "Right Mixer" },
    { "Right Line Out Source", "Stereo", "Right Line Out Switch" },
    { "Right Line Out Source", "Mono Differential", "Left Line Out Switch" },
    { "LINEOUT", core::ptr::null_mut(), "Right Line Out Source" },
// Earpiece Routes
    { "Earpiece Source Playback Route", "DACL", "Left DAC" },
    { "Earpiece Source Playback Route", "DACR", "Right DAC" },
    { "Earpiece Source Playback Route", "Left Mixer", "Left Mixer" },
    { "Earpiece Source Playback Route", "Right Mixer", "Right Mixer" },
    { "Earpiece Switch", "Earpiece Playback Switch", "Earpiece Source Playback Route" },
    { "Earpiece Amp", core::ptr::null_mut(), "Earpiece Switch" },
    { "EARPIECE", core::ptr::null_mut(), "Earpiece Amp" },
    };
    static int sun50i_a64_codec_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    int hbias;
    switch (level) {
    case SND_SOC_BIAS_OFF:
    regmap_clear_bits(component.regmap, SUN50I_ADDA_JACK_MIC_CTRL,
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN) |
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN));
    regmap_set_bits(component.regmap, SUN50I_ADDA_HP_CTRL,
    BIT(SUN50I_ADDA_HP_CTRL_PA_CLK_GATE));
    break;
    case SND_SOC_BIAS_STANDBY:
    regmap_clear_bits(component.regmap, SUN50I_ADDA_HP_CTRL,
    BIT(SUN50I_ADDA_HP_CTRL_PA_CLK_GATE));
    hbias = snd_soc_dapm_get_pin_status(dapm, "HBIAS");
    regmap_update_bits(component.regmap, SUN50I_ADDA_JACK_MIC_CTRL,
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN) |
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN),
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN) |
    hbias << SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN);
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct snd_soc_component_driver sun50i_codec_analog_cmpnt_drv = {
    .controls		= sun50i_a64_codec_controls,
    .num_controls		= ARRAY_SIZE(sun50i_a64_codec_controls),
    .dapm_widgets		= sun50i_a64_codec_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(sun50i_a64_codec_widgets),
    .dapm_routes		= sun50i_a64_codec_routes,
    .num_dapm_routes	= ARRAY_SIZE(sun50i_a64_codec_routes),
    .set_bias_level		= sun50i_a64_codec_set_bias_level,
    .idle_bias_on		= true,
    .suspend_bias_off	= true,
    };
    static const struct of_device_id sun50i_codec_analog_of_match[] = {
    {
    .compatible = "allwinner,sun50i-a64-codec-analog",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, sun50i_codec_analog_of_match);
#[no_mangle]
unsafe extern "C" fn sun50i_codec_analog_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_codec_analog_probe(struct platform_device *pdev)
    {
    struct regmap *regmap;
    void __iomem *base;
    bool enable;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = sun8i_adda_pr_regmap_init(&pdev.dev, base);
    if (IS_ERR(regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(regmap),
    "Failed to create regmap\n");
    enable = device_property_read_bool(&pdev.dev,
    "allwinner,internal-bias-resistor");
    regmap_update_bits(regmap, SUN50I_ADDA_JACK_MIC_CTRL,
    BIT(SUN50I_ADDA_JACK_MIC_CTRL_INNERRESEN),
    enable << SUN50I_ADDA_JACK_MIC_CTRL_INNERRESEN);
// Select sample interval of the ADC sample to 16ms
    regmap_update_bits(regmap, SUN50I_ADDA_MDET_CTRL,
    0x7 << SUN50I_ADDA_MDET_CTRL_SELDETADC_FS |
    0x3 << SUN50I_ADDA_MDET_CTRL_SELDETADC_BF,
    0x3 << SUN50I_ADDA_MDET_CTRL_SELDETADC_FS |
    0x3 << SUN50I_ADDA_MDET_CTRL_SELDETADC_BF);
    return devm_snd_soc_register_component(&pdev.dev,
    &sun50i_codec_analog_cmpnt_drv,
    core::ptr::null_mut(), 0);
    }
    static struct platform_driver sun50i_codec_analog_driver = {
    .driver = {
    .name = "sun50i-codec-analog",
    .of_match_table = sun50i_codec_analog_of_match,
    },
    .probe = sun50i_codec_analog_probe,
    };
    module_platform_driver(sun50i_codec_analog_driver);
    MODULE_DESCRIPTION("Allwinner internal codec analog controls driver for A64");
    MODULE_AUTHOR("Vasily Khoruzhick <anarsoul@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sun50i-codec-analog");
