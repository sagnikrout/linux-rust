//! Automatically rewritten from C to Rust
//! Source: sound/soc/sunxi/sun8i-codec-analog.c
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
// This driver supports the analog controls for the internal codec
// found in Allwinner's A31s, A23, A33 and H3 SoCs.
//
// Copyright 2016 Chen-Yu Tsai <wens@csie.org>
//

// Codec analog control register offsets and bit fields
pub const SUN8I_ADDA_HP_VOLC: c_uint = 0x00;
pub const SUN8I_ADDA_HP_VOLC_PA_CLK_GATE: c_int = 7;
pub const SUN8I_ADDA_HP_VOLC_HP_VOL: c_int = 0;
pub const SUN8I_ADDA_LOMIXSC: c_uint = 0x01;
pub const SUN8I_ADDA_LOMIXSC_MIC1: c_int = 6;
pub const SUN8I_ADDA_LOMIXSC_MIC2: c_int = 5;
pub const SUN8I_ADDA_LOMIXSC_PHONE: c_int = 4;
pub const SUN8I_ADDA_LOMIXSC_PHONEN: c_int = 3;
pub const SUN8I_ADDA_LOMIXSC_LINEINL: c_int = 2;
pub const SUN8I_ADDA_LOMIXSC_DACL: c_int = 1;
pub const SUN8I_ADDA_LOMIXSC_DACR: c_int = 0;
pub const SUN8I_ADDA_ROMIXSC: c_uint = 0x02;
pub const SUN8I_ADDA_ROMIXSC_MIC1: c_int = 6;
pub const SUN8I_ADDA_ROMIXSC_MIC2: c_int = 5;
pub const SUN8I_ADDA_ROMIXSC_PHONE: c_int = 4;
pub const SUN8I_ADDA_ROMIXSC_PHONEP: c_int = 3;
pub const SUN8I_ADDA_ROMIXSC_LINEINR: c_int = 2;
pub const SUN8I_ADDA_ROMIXSC_DACR: c_int = 1;
pub const SUN8I_ADDA_ROMIXSC_DACL: c_int = 0;
pub const SUN8I_ADDA_DAC_PA_SRC: c_uint = 0x03;
pub const SUN8I_ADDA_DAC_PA_SRC_DACAREN: c_int = 7;
pub const SUN8I_ADDA_DAC_PA_SRC_DACALEN: c_int = 6;
pub const SUN8I_ADDA_DAC_PA_SRC_RMIXEN: c_int = 5;
pub const SUN8I_ADDA_DAC_PA_SRC_LMIXEN: c_int = 4;
pub const SUN8I_ADDA_DAC_PA_SRC_RHPPAMUTE: c_int = 3;
pub const SUN8I_ADDA_DAC_PA_SRC_LHPPAMUTE: c_int = 2;
pub const SUN8I_ADDA_DAC_PA_SRC_RHPIS: c_int = 1;
pub const SUN8I_ADDA_DAC_PA_SRC_LHPIS: c_int = 0;
pub const SUN8I_ADDA_PHONEIN_GCTRL: c_uint = 0x04;
pub const SUN8I_ADDA_PHONEIN_GCTRL_PHONEPG: c_int = 4;
pub const SUN8I_ADDA_PHONEIN_GCTRL_PHONENG: c_int = 0;
pub const SUN8I_ADDA_LINEIN_GCTRL: c_uint = 0x05;
pub const SUN8I_ADDA_LINEIN_GCTRL_LINEING: c_int = 4;
pub const SUN8I_ADDA_LINEIN_GCTRL_PHONEG: c_int = 0;
pub const SUN8I_ADDA_MICIN_GCTRL: c_uint = 0x06;
pub const SUN8I_ADDA_MICIN_GCTRL_MIC1G: c_int = 4;
pub const SUN8I_ADDA_MICIN_GCTRL_MIC2G: c_int = 0;
pub const SUN8I_ADDA_PAEN_HP_CTRL: c_uint = 0x07;
pub const SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN: c_int = 7;

pub const SUN8I_ADDA_PAEN_HP_CTRL_HPCOM_FC: c_int = 5;
pub const SUN8I_ADDA_PAEN_HP_CTRL_COMPTEN: c_int = 4;
pub const SUN8I_ADDA_PAEN_HP_CTRL_PA_ANTI_POP_CTRL: c_int = 2;
pub const SUN8I_ADDA_PAEN_HP_CTRL_LTRNMUTE: c_int = 1;
pub const SUN8I_ADDA_PAEN_HP_CTRL_RTLNMUTE: c_int = 0;
pub const SUN8I_ADDA_PHONEOUT_CTRL: c_uint = 0x08;
pub const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUTG: c_int = 5;
pub const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUTEN: c_int = 4;
pub const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_MIC1: c_int = 3;
pub const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_MIC2: c_int = 2;
pub const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_RMIX: c_int = 1;
pub const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_LMIX: c_int = 0;
pub const SUN8I_ADDA_PHONE_GAIN_CTRL: c_uint = 0x09;
pub const SUN8I_ADDA_PHONE_GAIN_CTRL_LINEOUT_VOL: c_int = 3;
pub const SUN8I_ADDA_PHONE_GAIN_CTRL_PHONEPREG: c_int = 0;
pub const SUN8I_ADDA_MIC2G_CTRL: c_uint = 0x0a;
pub const SUN8I_ADDA_MIC2G_CTRL_MIC2AMPEN: c_int = 7;
pub const SUN8I_ADDA_MIC2G_CTRL_MIC2BOOST: c_int = 4;
pub const SUN8I_ADDA_MIC2G_CTRL_LINEOUTLEN: c_int = 3;
pub const SUN8I_ADDA_MIC2G_CTRL_LINEOUTREN: c_int = 2;
pub const SUN8I_ADDA_MIC2G_CTRL_LINEOUTLSRC: c_int = 1;
pub const SUN8I_ADDA_MIC2G_CTRL_LINEOUTRSRC: c_int = 0;
pub const SUN8I_ADDA_MIC1G_MICBIAS_CTRL: c_uint = 0x0b;
pub const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_HMICBIASEN: c_int = 7;
pub const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MMICBIASEN: c_int = 6;
pub const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_HMICBIAS_MODE: c_int = 5;
pub const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1AMPEN: c_int = 3;
pub const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1BOOST: c_int = 0;
pub const SUN8I_ADDA_LADCMIXSC: c_uint = 0x0c;
pub const SUN8I_ADDA_LADCMIXSC_MIC1: c_int = 6;
pub const SUN8I_ADDA_LADCMIXSC_MIC2: c_int = 5;
pub const SUN8I_ADDA_LADCMIXSC_PHONE: c_int = 4;
pub const SUN8I_ADDA_LADCMIXSC_PHONEN: c_int = 3;
pub const SUN8I_ADDA_LADCMIXSC_LINEINL: c_int = 2;
pub const SUN8I_ADDA_LADCMIXSC_OMIXRL: c_int = 1;
pub const SUN8I_ADDA_LADCMIXSC_OMIXRR: c_int = 0;
pub const SUN8I_ADDA_RADCMIXSC: c_uint = 0x0d;
pub const SUN8I_ADDA_RADCMIXSC_MIC1: c_int = 6;
pub const SUN8I_ADDA_RADCMIXSC_MIC2: c_int = 5;
pub const SUN8I_ADDA_RADCMIXSC_PHONE: c_int = 4;
pub const SUN8I_ADDA_RADCMIXSC_PHONEP: c_int = 3;
pub const SUN8I_ADDA_RADCMIXSC_LINEINR: c_int = 2;
pub const SUN8I_ADDA_RADCMIXSC_OMIXR: c_int = 1;
pub const SUN8I_ADDA_RADCMIXSC_OMIXL: c_int = 0;
pub const SUN8I_ADDA_RES: c_uint = 0x0e;
pub const SUN8I_ADDA_RES_MMICBIAS_SEL: c_int = 4;
pub const SUN8I_ADDA_RES_PA_ANTI_POP_CTRL: c_int = 0;
pub const SUN8I_ADDA_ADC_AP_EN: c_uint = 0x0f;
pub const SUN8I_ADDA_ADC_AP_EN_ADCREN: c_int = 7;
pub const SUN8I_ADDA_ADC_AP_EN_ADCLEN: c_int = 6;
pub const SUN8I_ADDA_ADC_AP_EN_ADCG: c_int = 0;
// mixer controls
    static const struct snd_kcontrol_new sun8i_codec_mixer_controls[] = {
    SOC_DAPM_DOUBLE_R("DAC Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_DACL, 1, 0),
    SOC_DAPM_DOUBLE_R("DAC Reversed Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_DACR, 1, 0),
    SOC_DAPM_DOUBLE_R("Line In Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_LINEINL, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic1 Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_MIC1, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic2 Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_MIC2, 1, 0),
    };
// mixer controls
    static const struct snd_kcontrol_new sun8i_v3s_codec_mixer_controls[] = {
    SOC_DAPM_DOUBLE_R("DAC Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_DACL, 1, 0),
    SOC_DAPM_DOUBLE_R("DAC Reversed Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_DACR, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic1 Playback Switch",
    SUN8I_ADDA_LOMIXSC,
    SUN8I_ADDA_ROMIXSC,
    SUN8I_ADDA_LOMIXSC_MIC1, 1, 0),
    };
// ADC mixer controls
    static const struct snd_kcontrol_new sun8i_codec_adc_mixer_controls[] = {
    SOC_DAPM_DOUBLE_R("Mixer Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_OMIXRL, 1, 0),
    SOC_DAPM_DOUBLE_R("Mixer Reversed Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_OMIXRR, 1, 0),
    SOC_DAPM_DOUBLE_R("Line In Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_LINEINL, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic1 Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_MIC1, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic2 Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_MIC2, 1, 0),
    };
// ADC mixer controls
    static const struct snd_kcontrol_new sun8i_v3s_codec_adc_mixer_controls[] = {
    SOC_DAPM_DOUBLE_R("Mixer Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_OMIXRL, 1, 0),
    SOC_DAPM_DOUBLE_R("Mixer Reversed Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_OMIXRR, 1, 0),
    SOC_DAPM_DOUBLE_R("Mic1 Capture Switch",
    SUN8I_ADDA_LADCMIXSC,
    SUN8I_ADDA_RADCMIXSC,
    SUN8I_ADDA_LADCMIXSC_MIC1, 1, 0),
    };
// volume / mute controls
    static const DECLARE_TLV_DB_SCALE(sun8i_codec_out_mixer_pregain_scale,
    -450, 150, 0);
    static const DECLARE_TLV_DB_RANGE(sun8i_codec_mic_gain_scale,
    0, 0, TLV_DB_SCALE_ITEM(0, 0, 0),
    1, 7, TLV_DB_SCALE_ITEM(2400, 300, 0),
    );
    static const struct snd_kcontrol_new sun8i_codec_common_controls[] = {
// Mixer pre-gain
    SOC_SINGLE_TLV("Mic1 Playback Volume", SUN8I_ADDA_MICIN_GCTRL,
    SUN8I_ADDA_MICIN_GCTRL_MIC1G,
    0x7, 0, sun8i_codec_out_mixer_pregain_scale),
// Microphone Amp boost gain
    SOC_SINGLE_TLV("Mic1 Boost Volume", SUN8I_ADDA_MIC1G_MICBIAS_CTRL,
    SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1BOOST, 0x7, 0,
    sun8i_codec_mic_gain_scale),
// ADC
    SOC_SINGLE_TLV("ADC Gain Capture Volume", SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCG, 0x7, 0,
    sun8i_codec_out_mixer_pregain_scale),
    };
    static const struct snd_soc_dapm_widget sun8i_codec_common_widgets[] = {
// ADC
    SND_SOC_DAPM_ADC("Left ADC", core::ptr::null_mut(), SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCLEN, 0),
    SND_SOC_DAPM_ADC("Right ADC", core::ptr::null_mut(), SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCREN, 0),
// DAC
    SND_SOC_DAPM_DAC("Left DAC", core::ptr::null_mut(), SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_DACALEN, 0),
    SND_SOC_DAPM_DAC("Right DAC", core::ptr::null_mut(), SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_DACAREN, 0),
//
// Due to this component and the codec belonging to separate DAPM
// contexts, we need to manually link the above widgets to their
// stream widgets at the card level.
//
// Microphone input
    SND_SOC_DAPM_INPUT("MIC1"),
// Mic input path
    SND_SOC_DAPM_PGA("Mic1 Amplifier", SUN8I_ADDA_MIC1G_MICBIAS_CTRL,
    SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1AMPEN, 0, core::ptr::null_mut(), 0),
    };
    static const struct snd_soc_dapm_widget sun8i_codec_mixer_widgets[] = {
    SND_SOC_DAPM_MIXER("Left Mixer", SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_LMIXEN, 0,
    sun8i_codec_mixer_controls,
    ARRAY_SIZE(sun8i_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("Right Mixer", SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_RMIXEN, 0,
    sun8i_codec_mixer_controls,
    ARRAY_SIZE(sun8i_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("Left ADC Mixer", SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCLEN, 0,
    sun8i_codec_adc_mixer_controls,
    ARRAY_SIZE(sun8i_codec_adc_mixer_controls)),
    SND_SOC_DAPM_MIXER("Right ADC Mixer", SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCREN, 0,
    sun8i_codec_adc_mixer_controls,
    ARRAY_SIZE(sun8i_codec_adc_mixer_controls)),
    };
    static const struct snd_soc_dapm_widget sun8i_v3s_codec_mixer_widgets[] = {
    SND_SOC_DAPM_MIXER("Left Mixer", SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_LMIXEN, 0,
    sun8i_v3s_codec_mixer_controls,
    ARRAY_SIZE(sun8i_v3s_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("Right Mixer", SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_RMIXEN, 0,
    sun8i_v3s_codec_mixer_controls,
    ARRAY_SIZE(sun8i_v3s_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER("Left ADC Mixer", SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCLEN, 0,
    sun8i_v3s_codec_adc_mixer_controls,
    ARRAY_SIZE(sun8i_v3s_codec_adc_mixer_controls)),
    SND_SOC_DAPM_MIXER("Right ADC Mixer", SUN8I_ADDA_ADC_AP_EN,
    SUN8I_ADDA_ADC_AP_EN_ADCREN, 0,
    sun8i_v3s_codec_adc_mixer_controls,
    ARRAY_SIZE(sun8i_v3s_codec_adc_mixer_controls)),
    };
    static const struct snd_soc_dapm_route sun8i_codec_common_routes[] = {
// Microphone Routes
    { "Mic1 Amplifier", core::ptr::null_mut(), "MIC1"},
    };
    static const struct snd_soc_dapm_route sun8i_codec_mixer_routes[] = {
// Left Mixer Routes
    { "Left Mixer", "DAC Playback Switch", "Left DAC" },
    { "Left Mixer", "DAC Reversed Playback Switch", "Right DAC" },
    { "Left Mixer", "Mic1 Playback Switch", "Mic1 Amplifier" },
// Right Mixer Routes
    { "Right Mixer", "DAC Playback Switch", "Right DAC" },
    { "Right Mixer", "DAC Reversed Playback Switch", "Left DAC" },
    { "Right Mixer", "Mic1 Playback Switch", "Mic1 Amplifier" },
// Left ADC Mixer Routes
    { "Left ADC Mixer", "Mixer Capture Switch", "Left Mixer" },
    { "Left ADC Mixer", "Mixer Reversed Capture Switch", "Right Mixer" },
    { "Left ADC Mixer", "Mic1 Capture Switch", "Mic1 Amplifier" },
// Right ADC Mixer Routes
    { "Right ADC Mixer", "Mixer Capture Switch", "Right Mixer" },
    { "Right ADC Mixer", "Mixer Reversed Capture Switch", "Left Mixer" },
    { "Right ADC Mixer", "Mic1 Capture Switch", "Mic1 Amplifier" },
// ADC Routes
    { "Left ADC", core::ptr::null_mut(), "Left ADC Mixer" },
    { "Right ADC", core::ptr::null_mut(), "Right ADC Mixer" },
    };
// headphone specific controls, widgets, and routes
    static const DECLARE_TLV_DB_SCALE(sun8i_codec_hp_vol_scale, -6300, 100, 1);
    static const struct snd_kcontrol_new sun8i_codec_headphone_controls[] = {
    SOC_SINGLE_TLV("Headphone Playback Volume",
    SUN8I_ADDA_HP_VOLC,
    SUN8I_ADDA_HP_VOLC_HP_VOL, 0x3f, 0,
    sun8i_codec_hp_vol_scale),
    SOC_DOUBLE("Headphone Playback Switch",
    SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_LHPPAMUTE,
    SUN8I_ADDA_DAC_PA_SRC_RHPPAMUTE, 1, 0),
    };
    static const char * const sun8i_codec_hp_src_enum_text[] = {
    "DAC", "Mixer",
    };
    static SOC_ENUM_DOUBLE_DECL(sun8i_codec_hp_src_enum,
    SUN8I_ADDA_DAC_PA_SRC,
    SUN8I_ADDA_DAC_PA_SRC_LHPIS,
    SUN8I_ADDA_DAC_PA_SRC_RHPIS,
    sun8i_codec_hp_src_enum_text);
    static const struct snd_kcontrol_new sun8i_codec_hp_src[] = {
    SOC_DAPM_ENUM("Headphone Source Playback Route",
    sun8i_codec_hp_src_enum),
    };
    static int sun8i_headphone_amp_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *k, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    if (SND_SOC_DAPM_EVENT_ON(event)) {
    snd_soc_component_update_bits(component, SUN8I_ADDA_PAEN_HP_CTRL,
    BIT(SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN),
    BIT(SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN));
//
// Need a delay to have the amplifier up. 700ms seems the best
// compromise between the time to let the amplifier up and the
// time not to feel this delay while playing a sound.
//
    msleep(700);
    } else if (SND_SOC_DAPM_EVENT_OFF(event)) {
    snd_soc_component_update_bits(component, SUN8I_ADDA_PAEN_HP_CTRL,
    BIT(SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN),
    0x0);
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget sun8i_codec_headphone_widgets[] = {
    SND_SOC_DAPM_MUX("Headphone Source Playback Route",
    SND_SOC_NOPM, 0, 0, sun8i_codec_hp_src),
    SND_SOC_DAPM_OUT_DRV_E("Headphone Amp", SUN8I_ADDA_PAEN_HP_CTRL,
    SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN, 0, core::ptr::null_mut(), 0,
    sun8i_headphone_amp_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY("HPCOM Protection", SUN8I_ADDA_PAEN_HP_CTRL,
    SUN8I_ADDA_PAEN_HP_CTRL_COMPTEN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_REG(snd_soc_dapm_supply, "HPCOM", SUN8I_ADDA_PAEN_HP_CTRL,
    SUN8I_ADDA_PAEN_HP_CTRL_HPCOM_FC, 0x3, 0x3, 0),
    SND_SOC_DAPM_OUTPUT("HP"),
    };
    static const struct snd_soc_dapm_route sun8i_codec_headphone_routes[] = {
    { "Headphone Source Playback Route", "DAC", "Left DAC" },
    { "Headphone Source Playback Route", "DAC", "Right DAC" },
    { "Headphone Source Playback Route", "Mixer", "Left Mixer" },
    { "Headphone Source Playback Route", "Mixer", "Right Mixer" },
    { "Headphone Amp", core::ptr::null_mut(), "Headphone Source Playback Route" },
    { "HPCOM", core::ptr::null_mut(), "HPCOM Protection" },
    { "HP", core::ptr::null_mut(), "Headphone Amp" },
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_add_headphone(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_add_headphone(struct snd_soc_component *cmpnt)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    ret = snd_soc_add_component_controls(cmpnt,
    sun8i_codec_headphone_controls,
    ARRAY_SIZE(sun8i_codec_headphone_controls));
    if (ret) {
    dev_err(dev, "Failed to add Headphone controls: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_headphone_widgets,
    ARRAY_SIZE(sun8i_codec_headphone_widgets));
    if (ret) {
    dev_err(dev, "Failed to add Headphone DAPM widgets: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_headphone_routes,
    ARRAY_SIZE(sun8i_codec_headphone_routes));
    if (ret) {
    dev_err(dev, "Failed to add Headphone DAPM routes: %d\n", ret);
    return ret;
    }
    return 0;
    }
// mbias specific widget
    static const struct snd_soc_dapm_widget sun8i_codec_mbias_widgets[] = {
    SND_SOC_DAPM_SUPPLY("MBIAS", SUN8I_ADDA_MIC1G_MICBIAS_CTRL,
    SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MMICBIASEN,
    0, core::ptr::null_mut(), 0),
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_add_mbias(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_add_mbias(struct snd_soc_component *cmpnt)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_mbias_widgets,
    ARRAY_SIZE(sun8i_codec_mbias_widgets));
    if (ret)
    dev_err(dev, "Failed to add MBIAS DAPM widgets: %d\n", ret);
    return ret;
    }
// hmic specific widget
    static const struct snd_soc_dapm_widget sun8i_codec_hmic_widgets[] = {
    SND_SOC_DAPM_SUPPLY("HBIAS", SUN8I_ADDA_MIC1G_MICBIAS_CTRL,
    SUN8I_ADDA_MIC1G_MICBIAS_CTRL_HMICBIASEN,
    0, core::ptr::null_mut(), 0),
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_add_hmic(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_add_hmic(struct snd_soc_component *cmpnt)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_hmic_widgets,
    ARRAY_SIZE(sun8i_codec_hmic_widgets));
    if (ret)
    dev_err(dev, "Failed to add Mic3 DAPM widgets: %d\n", ret);
    return ret;
    }
// line in specific controls, widgets and rines
    static const struct snd_kcontrol_new sun8i_codec_linein_controls[] = {
// Mixer pre-gain
    SOC_SINGLE_TLV("Line In Playback Volume", SUN8I_ADDA_LINEIN_GCTRL,
    SUN8I_ADDA_LINEIN_GCTRL_LINEING,
    0x7, 0, sun8i_codec_out_mixer_pregain_scale),
    };
    static const struct snd_soc_dapm_widget sun8i_codec_linein_widgets[] = {
// Line input
    SND_SOC_DAPM_INPUT("LINEIN"),
    };
    static const struct snd_soc_dapm_route sun8i_codec_linein_routes[] = {
    { "Left Mixer", "Line In Playback Switch", "LINEIN" },
    { "Right Mixer", "Line In Playback Switch", "LINEIN" },
    { "Left ADC Mixer", "Line In Capture Switch", "LINEIN" },
    { "Right ADC Mixer", "Line In Capture Switch", "LINEIN" },
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_add_linein(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_add_linein(struct snd_soc_component *cmpnt)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    ret = snd_soc_add_component_controls(cmpnt,
    sun8i_codec_linein_controls,
    ARRAY_SIZE(sun8i_codec_linein_controls));
    if (ret) {
    dev_err(dev, "Failed to add Line In controls: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_linein_widgets,
    ARRAY_SIZE(sun8i_codec_linein_widgets));
    if (ret) {
    dev_err(dev, "Failed to add Line In DAPM widgets: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_linein_routes,
    ARRAY_SIZE(sun8i_codec_linein_routes));
    if (ret) {
    dev_err(dev, "Failed to add Line In DAPM routes: %d\n", ret);
    return ret;
    }
    return 0;
    }
// line out specific controls, widgets and routes
    static const DECLARE_TLV_DB_RANGE(sun8i_codec_lineout_vol_scale,
    0, 1, TLV_DB_SCALE_ITEM(TLV_DB_GAIN_MUTE, 0, 1),
    2, 31, TLV_DB_SCALE_ITEM(-4350, 150, 0),
    );
    static const struct snd_kcontrol_new sun8i_codec_lineout_controls[] = {
    SOC_SINGLE_TLV("Line Out Playback Volume",
    SUN8I_ADDA_PHONE_GAIN_CTRL,
    SUN8I_ADDA_PHONE_GAIN_CTRL_LINEOUT_VOL, 0x1f, 0,
    sun8i_codec_lineout_vol_scale),
    SOC_DOUBLE("Line Out Playback Switch",
    SUN8I_ADDA_MIC2G_CTRL,
    SUN8I_ADDA_MIC2G_CTRL_LINEOUTLEN,
    SUN8I_ADDA_MIC2G_CTRL_LINEOUTREN, 1, 0),
    };
    static const char * const sun8i_codec_lineout_src_enum_text[] = {
    "Stereo", "Mono Differential",
    };
    static SOC_ENUM_DOUBLE_DECL(sun8i_codec_lineout_src_enum,
    SUN8I_ADDA_MIC2G_CTRL,
    SUN8I_ADDA_MIC2G_CTRL_LINEOUTLSRC,
    SUN8I_ADDA_MIC2G_CTRL_LINEOUTRSRC,
    sun8i_codec_lineout_src_enum_text);
    static const struct snd_kcontrol_new sun8i_codec_lineout_src[] = {
    SOC_DAPM_ENUM("Line Out Source Playback Route",
    sun8i_codec_lineout_src_enum),
    };
    static const struct snd_soc_dapm_widget sun8i_codec_lineout_widgets[] = {
    SND_SOC_DAPM_MUX("Line Out Source Playback Route",
    SND_SOC_NOPM, 0, 0, sun8i_codec_lineout_src),
// It is unclear if this is a buffer or gate, model it as a supply
    SND_SOC_DAPM_SUPPLY("Line Out Enable", SUN8I_ADDA_PAEN_HP_CTRL,
    SUN8I_ADDA_PAEN_HP_CTRL_LINEOUTEN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("LINEOUT"),
    };
    static const struct snd_soc_dapm_route sun8i_codec_lineout_routes[] = {
    { "Line Out Source Playback Route", "Stereo", "Left Mixer" },
    { "Line Out Source Playback Route", "Stereo", "Right Mixer" },
    { "Line Out Source Playback Route", "Mono Differential", "Left Mixer" },
    { "Line Out Source Playback Route", "Mono Differential", "Right Mixer" },
    { "LINEOUT", core::ptr::null_mut(), "Line Out Source Playback Route" },
    { "LINEOUT", core::ptr::null_mut(), "Line Out Enable", },
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_add_lineout(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_add_lineout(struct snd_soc_component *cmpnt)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    ret = snd_soc_add_component_controls(cmpnt,
    sun8i_codec_lineout_controls,
    ARRAY_SIZE(sun8i_codec_lineout_controls));
    if (ret) {
    dev_err(dev, "Failed to add Line Out controls: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_lineout_widgets,
    ARRAY_SIZE(sun8i_codec_lineout_widgets));
    if (ret) {
    dev_err(dev, "Failed to add Line Out DAPM widgets: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_lineout_routes,
    ARRAY_SIZE(sun8i_codec_lineout_routes));
    if (ret) {
    dev_err(dev, "Failed to add Line Out DAPM routes: %d\n", ret);
    return ret;
    }
    return 0;
    }
// mic2 specific controls, widgets and routes
    static const struct snd_kcontrol_new sun8i_codec_mic2_controls[] = {
// Mixer pre-gain
    SOC_SINGLE_TLV("Mic2 Playback Volume",
    SUN8I_ADDA_MICIN_GCTRL, SUN8I_ADDA_MICIN_GCTRL_MIC2G,
    0x7, 0, sun8i_codec_out_mixer_pregain_scale),
// Microphone Amp boost gain
    SOC_SINGLE_TLV("Mic2 Boost Volume", SUN8I_ADDA_MIC2G_CTRL,
    SUN8I_ADDA_MIC2G_CTRL_MIC2BOOST, 0x7, 0,
    sun8i_codec_mic_gain_scale),
    };
    static const struct snd_soc_dapm_widget sun8i_codec_mic2_widgets[] = {
// Microphone input
    SND_SOC_DAPM_INPUT("MIC2"),
// Mic input path
    SND_SOC_DAPM_PGA("Mic2 Amplifier", SUN8I_ADDA_MIC2G_CTRL,
    SUN8I_ADDA_MIC2G_CTRL_MIC2AMPEN, 0, core::ptr::null_mut(), 0),
    };
    static const struct snd_soc_dapm_route sun8i_codec_mic2_routes[] = {
    { "Mic2 Amplifier", core::ptr::null_mut(), "MIC2"},
    { "Left Mixer", "Mic2 Playback Switch", "Mic2 Amplifier" },
    { "Right Mixer", "Mic2 Playback Switch", "Mic2 Amplifier" },
    { "Left ADC Mixer", "Mic2 Capture Switch", "Mic2 Amplifier" },
    { "Right ADC Mixer", "Mic2 Capture Switch", "Mic2 Amplifier" },
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_add_mic2(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_add_mic2(struct snd_soc_component *cmpnt)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    ret = snd_soc_add_component_controls(cmpnt,
    sun8i_codec_mic2_controls,
    ARRAY_SIZE(sun8i_codec_mic2_controls));
    if (ret) {
    dev_err(dev, "Failed to add MIC2 controls: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_mic2_widgets,
    ARRAY_SIZE(sun8i_codec_mic2_widgets));
    if (ret) {
    dev_err(dev, "Failed to add MIC2 DAPM widgets: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_mic2_routes,
    ARRAY_SIZE(sun8i_codec_mic2_routes));
    if (ret) {
    dev_err(dev, "Failed to add MIC2 DAPM routes: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_codec_analog_quirks {
    pub has_headphone: bool,
    pub has_hmic: bool,
    pub has_linein: bool,
    pub has_lineout: bool,
    pub has_mbias: bool,
    pub has_mic2: bool,
}

    static const struct sun8i_codec_analog_quirks sun8i_a23_quirks = {
    .has_headphone	= true,
    .has_hmic	= true,
    .has_linein	= true,
    .has_mbias	= true,
    .has_mic2	= true,
    };
    static const struct sun8i_codec_analog_quirks sun8i_h3_quirks = {
    .has_linein	= true,
    .has_lineout	= true,
    .has_mbias	= true,
    .has_mic2	= true,
    };
    static int sun8i_codec_analog_add_mixer(struct snd_soc_component *cmpnt,
    const struct sun8i_codec_analog_quirks *quirks)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cmpnt);
    struct device *dev = cmpnt.dev;
    int ret;
    if (!quirks.has_mic2 && !quirks.has_linein) {
//
// Apply the special widget set which has uses a control
// without MIC2 and Line In, for SoCs without these.
// TODO: not all special cases are supported now, this case
// is present because it's the case of V3s.
//
    ret = snd_soc_dapm_new_controls(dapm,
    sun8i_v3s_codec_mixer_widgets,
    ARRAY_SIZE(sun8i_v3s_codec_mixer_widgets));
    if (ret) {
    dev_err(dev, "Failed to add V3s Mixer DAPM widgets: %d\n", ret);
    return ret;
    }
    } else {
// Apply the generic mixer widget set.
    ret = snd_soc_dapm_new_controls(dapm,
    sun8i_codec_mixer_widgets,
    ARRAY_SIZE(sun8i_codec_mixer_widgets));
    if (ret) {
    dev_err(dev, "Failed to add Mixer DAPM widgets: %d\n", ret);
    return ret;
    }
    }
    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_mixer_routes,
    ARRAY_SIZE(sun8i_codec_mixer_routes));
    if (ret) {
    dev_err(dev, "Failed to add Mixer DAPM routes: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct sun8i_codec_analog_quirks sun8i_v3s_quirks = {
    .has_headphone	= true,
    .has_hmic	= true,
    };
#[no_mangle]
unsafe extern "C" fn sun8i_codec_analog_cmpnt_probe(cmpnt: *mut snd_soc_component) -> c_int {
    static int sun8i_codec_analog_cmpnt_probe(struct snd_soc_component *cmpnt)
    {
    struct device *dev = cmpnt.dev;
    const struct sun8i_codec_analog_quirks *quirks;
    int ret;
//
// This would never return NULL unless someone directly registers a
// platform device matching this driver's name, without specifying a
// device tree node.
//
    quirks = of_device_get_match_data(dev);
// Add controls, widgets, and routes for individual features
    ret = sun8i_codec_analog_add_mixer(cmpnt, quirks);
    if (ret)
    return ret;
    if (quirks.has_headphone) {
    ret = sun8i_codec_add_headphone(cmpnt);
    if (ret)
    return ret;
    }
    if (quirks.has_hmic) {
    ret = sun8i_codec_add_hmic(cmpnt);
    if (ret)
    return ret;
    }
    if (quirks.has_linein) {
    ret = sun8i_codec_add_linein(cmpnt);
    if (ret)
    return ret;
    }
    if (quirks.has_lineout) {
    ret = sun8i_codec_add_lineout(cmpnt);
    if (ret)
    return ret;
    }
    if (quirks.has_mbias) {
    ret = sun8i_codec_add_mbias(cmpnt);
    if (ret)
    return ret;
    }
    if (quirks.has_mic2) {
    ret = sun8i_codec_add_mic2(cmpnt);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_component_driver sun8i_codec_analog_cmpnt_drv = {
    .controls		= sun8i_codec_common_controls,
    .num_controls		= ARRAY_SIZE(sun8i_codec_common_controls),
    .dapm_widgets		= sun8i_codec_common_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(sun8i_codec_common_widgets),
    .dapm_routes		= sun8i_codec_common_routes,
    .num_dapm_routes	= ARRAY_SIZE(sun8i_codec_common_routes),
    .probe			= sun8i_codec_analog_cmpnt_probe,
    };
    static const struct of_device_id sun8i_codec_analog_of_match[] = {
    {
    .compatible = "allwinner,sun8i-a23-codec-analog",
    .data = &sun8i_a23_quirks,
    },
    {
    .compatible = "allwinner,sun8i-h3-codec-analog",
    .data = &sun8i_h3_quirks,
    },
    {
    .compatible = "allwinner,sun8i-v3s-codec-analog",
    .data = &sun8i_v3s_quirks,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, sun8i_codec_analog_of_match);
#[no_mangle]
unsafe extern "C" fn sun8i_codec_analog_probe(pdev: *mut platform_device) -> c_int {
    static int sun8i_codec_analog_probe(struct platform_device *pdev)
    {
    struct regmap *regmap;
    void __iomem *base;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = sun8i_adda_pr_regmap_init(&pdev.dev, base);
    if (IS_ERR(regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(regmap),
    "Failed to create regmap\n");
    return devm_snd_soc_register_component(&pdev.dev,
    &sun8i_codec_analog_cmpnt_drv,
    core::ptr::null_mut(), 0);
    }
    static struct platform_driver sun8i_codec_analog_driver = {
    .driver = {
    .name = "sun8i-codec-analog",
    .of_match_table = sun8i_codec_analog_of_match,
    },
    .probe = sun8i_codec_analog_probe,
    };
    module_platform_driver(sun8i_codec_analog_driver);
    MODULE_DESCRIPTION("Allwinner internal codec analog controls driver");
    MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sun8i-codec-analog");
