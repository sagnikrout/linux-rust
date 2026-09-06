//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tda7419.c
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
// TDA7419 audio processor driver
//
// Copyright 2018 Konsulko Group
//
// Author: Matt Porter <mporter@konsulko.com>
//

pub const TDA7419_MAIN_SRC_REG: c_uint = 0x00;
pub const TDA7419_LOUDNESS_REG: c_uint = 0x01;
pub const TDA7419_MUTE_CLK_REG: c_uint = 0x02;
pub const TDA7419_VOLUME_REG: c_uint = 0x03;
pub const TDA7419_TREBLE_REG: c_uint = 0x04;
pub const TDA7419_MIDDLE_REG: c_uint = 0x05;
pub const TDA7419_BASS_REG: c_uint = 0x06;
pub const TDA7419_SECOND_SRC_REG: c_uint = 0x07;
pub const TDA7419_SUB_MID_BASS_REG: c_uint = 0x08;
pub const TDA7419_MIXING_GAIN_REG: c_uint = 0x09;
pub const TDA7419_ATTENUATOR_LF_REG: c_uint = 0x0a;
pub const TDA7419_ATTENUATOR_RF_REG: c_uint = 0x0b;
pub const TDA7419_ATTENUATOR_LR_REG: c_uint = 0x0c;
pub const TDA7419_ATTENUATOR_RR_REG: c_uint = 0x0d;
pub const TDA7419_MIXING_LEVEL_REG: c_uint = 0x0e;
pub const TDA7419_ATTENUATOR_SUB_REG: c_uint = 0x0f;
pub const TDA7419_SA_CLK_AC_REG: c_uint = 0x10;
pub const TDA7419_TESTING_REG: c_uint = 0x11;
pub const TDA7419_MAIN_SRC_SEL: c_int = 0;
pub const TDA7419_MAIN_SRC_GAIN: c_int = 3;
pub const TDA7419_MAIN_SRC_AUTOZERO: c_int = 7;
pub const TDA7419_LOUDNESS_ATTEN: c_int = 0;
pub const TDA7419_LOUDNESS_CENTER_FREQ: c_int = 4;
pub const TDA7419_LOUDNESS_BOOST: c_int = 6;
pub const TDA7419_LOUDNESS_SOFT_STEP: c_int = 7;
pub const TDA7419_VOLUME_SOFT_STEP: c_int = 7;
pub const TDA7419_SOFT_MUTE: c_int = 0;
pub const TDA7419_MUTE_INFLUENCE: c_int = 1;
pub const TDA7419_SOFT_MUTE_TIME: c_int = 2;
pub const TDA7419_SOFT_STEP_TIME: c_int = 4;
pub const TDA7419_CLK_FAST_MODE: c_int = 7;
pub const TDA7419_TREBLE_CENTER_FREQ: c_int = 5;
pub const TDA7419_REF_OUT_SELECT: c_int = 7;
pub const TDA7419_MIDDLE_Q_FACTOR: c_int = 5;
pub const TDA7419_MIDDLE_SOFT_STEP: c_int = 7;
pub const TDA7419_BASS_Q_FACTOR: c_int = 5;
pub const TDA7419_BASS_SOFT_STEP: c_int = 7;
pub const TDA7419_SECOND_SRC_SEL: c_int = 0;
pub const TDA7419_SECOND_SRC_GAIN: c_int = 3;
pub const TDA7419_REAR_SPKR_SRC: c_int = 7;
pub const TDA7419_SUB_CUT_OFF_FREQ: c_int = 0;
pub const TDA7419_MIDDLE_CENTER_FREQ: c_int = 2;
pub const TDA7419_BASS_CENTER_FREQ: c_int = 4;
pub const TDA7419_BASS_DC_MODE: c_int = 6;
pub const TDA7419_SMOOTHING_FILTER: c_int = 7;
pub const TDA7419_MIX_LF: c_int = 0;
pub const TDA7419_MIX_RF: c_int = 1;
pub const TDA7419_MIX_ENABLE: c_int = 2;
pub const TDA7419_SUB_ENABLE: c_int = 3;
pub const TDA7419_HPF_GAIN: c_int = 4;
pub const TDA7419_SA_Q_FACTOR: c_int = 0;
pub const TDA7419_RESET_MODE: c_int = 1;
pub const TDA7419_SA_SOURCE: c_int = 2;
pub const TDA7419_SA_RUN: c_int = 3;
pub const TDA7419_RESET: c_int = 4;
pub const TDA7419_CLK_SOURCE: c_int = 5;
pub const TDA7419_COUPLING_MODE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda7419_data {
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn tda7419_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tda7419_readable_reg(struct device *dev, unsigned int reg)
    {
    return false;
    }
    static const struct reg_default tda7419_regmap_defaults[] = {
    { TDA7419_MAIN_SRC_REG,	0xfe },
    { TDA7419_LOUDNESS_REG, 0xfe },
    { TDA7419_MUTE_CLK_REG, 0xfe },
    { TDA7419_VOLUME_REG, 0xfe },
    { TDA7419_TREBLE_REG, 0xfe },
    { TDA7419_MIDDLE_REG, 0xfe },
    { TDA7419_BASS_REG, 0xfe },
    { TDA7419_SECOND_SRC_REG, 0xfe },
    { TDA7419_SUB_MID_BASS_REG, 0xfe },
    { TDA7419_MIXING_GAIN_REG, 0xfe },
    { TDA7419_ATTENUATOR_LF_REG, 0xfe },
    { TDA7419_ATTENUATOR_RF_REG, 0xfe },
    { TDA7419_ATTENUATOR_LR_REG, 0xfe },
    { TDA7419_ATTENUATOR_RR_REG, 0xfe },
    { TDA7419_MIXING_LEVEL_REG, 0xfe },
    { TDA7419_ATTENUATOR_SUB_REG, 0xfe },
    { TDA7419_SA_CLK_AC_REG, 0xfe },
    { TDA7419_TESTING_REG, 0xfe },
    };
    static const struct regmap_config tda7419_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = TDA7419_TESTING_REG,
    .cache_type = REGCACHE_RBTREE,
    .readable_reg = tda7419_readable_reg,
    .reg_defaults = tda7419_regmap_defaults,
    .num_reg_defaults = ARRAY_SIZE(tda7419_regmap_defaults),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda7419_vol_control {
    pub max: int min,,
    pub thresh: unsigned int reg, rreg, mask,,
    pub invert:1: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn tda7419_vol_is_stereo(tvc: *mut tda7419_vol_control) -> bool {
    static inline bool tda7419_vol_is_stereo(struct tda7419_vol_control *tvc)
    {
    if (tvc.reg == tvc.rreg)
    return false;
    return true;
    }
    static int tda7419_vol_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct tda7419_vol_control *tvc =
    (struct tda7419_vol_control *)kcontrol.private_value;
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = tda7419_vol_is_stereo(tvc) ? 2 : 1;
    uinfo.value.integer.min = tvc.min;
    uinfo.value.integer.max = tvc.max;
    return 0;
    }
    static inline int tda7419_vol_get_value(int val, unsigned int mask,
    int min, int thresh,
    unsigned int invert)
    {
    val &= mask;
    if (val < thresh) {
    if (invert)
    val = 0 - val;
    } else if (val > thresh) {
    if (invert)
    val = val - thresh;
    else
    val = thresh - val;
    }
    if (val < min)
    val = min;
    return val;
    }
    static int tda7419_vol_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct tda7419_vol_control *tvc =
    (struct tda7419_vol_control *)kcontrol.private_value;
    let mut reg: c_uint = tvc.reg;
    let mut rreg: c_uint = tvc.rreg;
    let mut mask: c_uint = tvc.mask;
    let mut min: c_int = tvc.min;
    let mut thresh: c_int = tvc.thresh;
    let mut invert: c_uint = tvc.invert;
    int val;
    val = snd_soc_component_read(component, reg);
    ucontrol.value.integer.value[0] =
    tda7419_vol_get_value(val, mask, min, thresh, invert);
    if (tda7419_vol_is_stereo(tvc)) {
    val = snd_soc_component_read(component, rreg);
    ucontrol.value.integer.value[1] =
    tda7419_vol_get_value(val, mask, min, thresh, invert);
    }
    return 0;
    }
    static inline int tda7419_vol_put_value(int val, int thresh,
    unsigned int invert)
    {
    if (val < 0) {
    if (invert)
    val = abs(val);
    else
    val = thresh - val;
    } else if ((val > 0) && invert) {
    val += thresh;
    }
    return val;
    }
    static int tda7419_vol_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component =
    snd_kcontrol_chip(kcontrol);
    struct tda7419_vol_control *tvc =
    (struct tda7419_vol_control *)kcontrol.private_value;
    let mut reg: c_uint = tvc.reg;
    let mut rreg: c_uint = tvc.rreg;
    let mut mask: c_uint = tvc.mask;
    let mut thresh: c_int = tvc.thresh;
    let mut invert: c_uint = tvc.invert;
    int val;
    int ret;
    val = tda7419_vol_put_value(ucontrol.value.integer.value[0],
    thresh, invert);
    ret = snd_soc_component_update_bits(component, reg,
    mask, val);
    if (ret < 0)
    return ret;
    if (tda7419_vol_is_stereo(tvc)) {
    val = tda7419_vol_put_value(ucontrol.value.integer.value[1],
    thresh, invert);
    ret = snd_soc_component_update_bits(component, rreg,
    mask, val);
    }
    return ret;
    }

    ((unsigned long)&(struct tda7419_vol_control) \
    {.reg = xreg, .rreg = xreg, .mask = xmask, .min = xmin, \
    .max = xmax, .thresh = xthresh, .invert = xinvert})

    xinvert) \
    ((unsigned long)&(struct tda7419_vol_control) \
    {.reg = xregl, .rreg = xregr, .mask = xmask, .min = xmin, \
    .max = xmax, .thresh = xthresh, .invert = xinvert})

    xinvert, xtlv_array) \
    {	.iface = SNDRV_CTL_ELEM_IFACE_MIXER, \
    .name = xname, \
    .access = SNDRV_CTL_ELEM_ACCESS_TLV_READ | \
    SNDRV_CTL_ELEM_ACCESS_READWRITE, \
    .tlv.p = (xtlv_array), \
    .info = tda7419_vol_info, \
    .get = tda7419_vol_get, \
    .put = tda7419_vol_put, \
    .private_value = TDA7419_SINGLE_VALUE(xreg, xmask, xmin, \
    xmax, xthresh, xinvert), \
    }

    xthresh, xinvert, xtlv_array) \
    {	.iface = SNDRV_CTL_ELEM_IFACE_MIXER, \
    .name = xname, \
    .access = SNDRV_CTL_ELEM_ACCESS_TLV_READ | \
    SNDRV_CTL_ELEM_ACCESS_READWRITE, \
    .tlv.p = (xtlv_array), \
    .info = tda7419_vol_info, \
    .get = tda7419_vol_get, \
    .put = tda7419_vol_put, \
    .private_value = TDA7419_DOUBLE_R_VALUE(xregl, xregr, xmask, \
    xmin, xmax, xthresh, \
    xinvert), \
    }
    static const char * const enum_src_sel[] = {
    "QD", "SE1", "SE2", "SE3", "SE", "Mute", "Mute", "Mute"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_main_src_sel,
    TDA7419_MAIN_SRC_REG, TDA7419_MAIN_SRC_SEL, enum_src_sel);
    static const struct snd_kcontrol_new soc_mux_main_src_sel =
    SOC_DAPM_ENUM("Main Source Select", soc_enum_main_src_sel);
    static DECLARE_TLV_DB_SCALE(tlv_src_gain, 0, 100, 0);
    static DECLARE_TLV_DB_SCALE(tlv_loudness_atten, -1500, 100, 0);
    static const char * const enum_loudness_center_freq[] = {
    "Flat", "400 Hz", "800 Hz", "2400 Hz"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_loudness_center_freq,
    TDA7419_LOUDNESS_REG, TDA7419_LOUDNESS_CENTER_FREQ,
    enum_loudness_center_freq);
    static const char * const enum_mute_influence[] = {
    "Pin and IIC", "IIC"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_mute_influence,
    TDA7419_MUTE_CLK_REG, TDA7419_MUTE_INFLUENCE, enum_mute_influence);
    static const char * const enum_soft_mute_time[] = {
    "0.48 ms", "0.96 ms", "123 ms", "123 ms"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_soft_mute_time,
    TDA7419_MUTE_CLK_REG, TDA7419_SOFT_MUTE_TIME, enum_soft_mute_time);
    static const char * const enum_soft_step_time[] = {
    "0.160 ms", "0.321 ms", "0.642 ms", "1.28 ms",
    "2.56 ms", "5.12 ms", "10.24 ms", "20.48 ms"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_soft_step_time,
    TDA7419_MUTE_CLK_REG, TDA7419_SOFT_STEP_TIME, enum_soft_step_time);
    static DECLARE_TLV_DB_SCALE(tlv_volume, -8000, 100, 1);
    static const char * const enum_treble_center_freq[] = {
    "10.0 kHz", "12.5 kHz", "15.0 kHz", "17.5 kHz"};
    static DECLARE_TLV_DB_SCALE(tlv_filter, -1500, 100, 0);
    static SOC_ENUM_SINGLE_DECL(soc_enum_treble_center_freq,
    TDA7419_TREBLE_REG, TDA7419_TREBLE_CENTER_FREQ,
    enum_treble_center_freq);
    static const char * const enum_ref_out_select[] = {
    "External Vref (4 V)", "Internal Vref (3.3 V)"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_ref_out_select,
    TDA7419_TREBLE_REG, TDA7419_REF_OUT_SELECT, enum_ref_out_select);
    static const char * const enum_middle_q_factor[] = {
    "0.5", "0.75", "1.0", "1.25"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_middle_q_factor,
    TDA7419_MIDDLE_REG, TDA7419_MIDDLE_Q_FACTOR, enum_middle_q_factor);
    static const char * const enum_bass_q_factor[] = {
    "1.0", "1.25", "1.5", "2.0"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_bass_q_factor,
    TDA7419_BASS_REG, TDA7419_BASS_Q_FACTOR, enum_bass_q_factor);
    static SOC_ENUM_SINGLE_DECL(soc_enum_second_src_sel,
    TDA7419_SECOND_SRC_REG, TDA7419_SECOND_SRC_SEL, enum_src_sel);
    static const struct snd_kcontrol_new soc_mux_second_src_sel =
    SOC_DAPM_ENUM("Second Source Select", soc_enum_second_src_sel);
    static const char * const enum_rear_spkr_src[] = {
    "Main", "Second"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_rear_spkr_src,
    TDA7419_SECOND_SRC_REG, TDA7419_REAR_SPKR_SRC, enum_rear_spkr_src);
    static const struct snd_kcontrol_new soc_mux_rear_spkr_src =
    SOC_DAPM_ENUM("Rear Speaker Source", soc_enum_rear_spkr_src);
    static const char * const enum_sub_cut_off_freq[] = {
    "Flat", "80 Hz", "120 Hz", "160 Hz"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_sub_cut_off_freq,
    TDA7419_SUB_MID_BASS_REG, TDA7419_SUB_CUT_OFF_FREQ,
    enum_sub_cut_off_freq);
    static const char * const enum_middle_center_freq[] = {
    "500 Hz", "1000 Hz", "1500 Hz", "2500 Hz"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_middle_center_freq,
    TDA7419_SUB_MID_BASS_REG, TDA7419_MIDDLE_CENTER_FREQ,
    enum_middle_center_freq);
    static const char * const enum_bass_center_freq[] = {
    "60 Hz", "80 Hz", "100 Hz", "200 Hz"};
    static SOC_ENUM_SINGLE_DECL(soc_enum_bass_center_freq,
    TDA7419_SUB_MID_BASS_REG, TDA7419_BASS_CENTER_FREQ,
    enum_bass_center_freq);
    static const char * const enum_sa_q_factor[] = {
    "3.5", "1.75" };
    static SOC_ENUM_SINGLE_DECL(soc_enum_sa_q_factor,
    TDA7419_SA_CLK_AC_REG, TDA7419_SA_Q_FACTOR, enum_sa_q_factor);
    static const char * const enum_reset_mode[] = {
    "IIC", "Auto" };
    static SOC_ENUM_SINGLE_DECL(soc_enum_reset_mode,
    TDA7419_SA_CLK_AC_REG, TDA7419_RESET_MODE, enum_reset_mode);
    static const char * const enum_sa_src[] = {
    "Bass", "In Gain" };
    static SOC_ENUM_SINGLE_DECL(soc_enum_sa_src,
    TDA7419_SA_CLK_AC_REG, TDA7419_SA_SOURCE, enum_sa_src);
    static const char * const enum_clk_src[] = {
    "Internal", "External" };
    static SOC_ENUM_SINGLE_DECL(soc_enum_clk_src,
    TDA7419_SA_CLK_AC_REG, TDA7419_CLK_SOURCE, enum_clk_src);
    static const char * const enum_coupling_mode[] = {
    "DC Coupling (without HPF)", "AC Coupling after In Gain",
    "DC Coupling (with HPF)", "AC Coupling after Bass" };
    static SOC_ENUM_SINGLE_DECL(soc_enum_coupling_mode,
    TDA7419_SA_CLK_AC_REG, TDA7419_COUPLING_MODE, enum_coupling_mode);
// ASoC Controls
    static struct snd_kcontrol_new tda7419_controls[] = {
    SOC_SINGLE_TLV("Main Source Capture Volume", TDA7419_MAIN_SRC_REG,
    TDA7419_MAIN_SRC_GAIN, 15, 0, tlv_src_gain),
    SOC_SINGLE("Main Source AutoZero Switch", TDA7419_MAIN_SRC_REG,
    TDA7419_MAIN_SRC_AUTOZERO, 1, 1),
    SOC_SINGLE_TLV("Loudness Playback Volume", TDA7419_LOUDNESS_REG,
    TDA7419_LOUDNESS_ATTEN, 15, 1, tlv_loudness_atten),
    SOC_ENUM("Loudness Center Frequency", soc_enum_loudness_center_freq),
    SOC_SINGLE("Loudness High Boost Switch", TDA7419_LOUDNESS_REG,
    TDA7419_LOUDNESS_BOOST, 1, 1),
    SOC_SINGLE("Loudness Soft Step Switch", TDA7419_LOUDNESS_REG,
    TDA7419_LOUDNESS_SOFT_STEP, 1, 1),
    SOC_SINGLE("Soft Mute Switch", TDA7419_MUTE_CLK_REG, TDA7419_SOFT_MUTE, 1, 1),
    SOC_ENUM("Mute Influence", soc_enum_mute_influence),
    SOC_ENUM("Soft Mute Time", soc_enum_soft_mute_time),
    SOC_ENUM("Soft Step Time", soc_enum_soft_step_time),
    SOC_SINGLE("Clock Fast Mode Switch", TDA7419_MUTE_CLK_REG,
    TDA7419_CLK_FAST_MODE, 1, 1),
    TDA7419_SINGLE_TLV("Master Playback Volume", TDA7419_VOLUME_REG,
    0x7f, -80, 15, 0x10, 0, tlv_volume),
    SOC_SINGLE("Volume Soft Step Switch", TDA7419_VOLUME_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    TDA7419_SINGLE_TLV("Treble Playback Volume", TDA7419_TREBLE_REG,
    0x1f, -15, 15, 0x10, 1, tlv_filter),
    SOC_ENUM("Treble Center Frequency", soc_enum_treble_center_freq),
    SOC_ENUM("Reference Output Select", soc_enum_ref_out_select),
    TDA7419_SINGLE_TLV("Middle Playback Volume", TDA7419_MIDDLE_REG,
    0x1f, -15, 15, 0x10, 1, tlv_filter),
    SOC_ENUM("Middle Q Factor", soc_enum_middle_q_factor),
    SOC_SINGLE("Middle Soft Step Switch", TDA7419_MIDDLE_REG,
    TDA7419_MIDDLE_SOFT_STEP, 1, 1),
    TDA7419_SINGLE_TLV("Bass Playback Volume", TDA7419_BASS_REG,
    0x1f, -15, 15, 0x10, 1, tlv_filter),
    SOC_ENUM("Bass Q Factor", soc_enum_bass_q_factor),
    SOC_SINGLE("Bass Soft Step Switch", TDA7419_BASS_REG,
    TDA7419_BASS_SOFT_STEP, 1, 1),
    SOC_SINGLE_TLV("Second Source Capture Volume", TDA7419_SECOND_SRC_REG,
    TDA7419_SECOND_SRC_GAIN, 15, 0, tlv_src_gain),
    SOC_ENUM("Subwoofer Cut-off Frequency", soc_enum_sub_cut_off_freq),
    SOC_ENUM("Middle Center Frequency", soc_enum_middle_center_freq),
    SOC_ENUM("Bass Center Frequency", soc_enum_bass_center_freq),
    SOC_SINGLE("Bass DC Mode Switch", TDA7419_SUB_MID_BASS_REG,
    TDA7419_BASS_DC_MODE, 1, 1),
    SOC_SINGLE("Smoothing Filter Switch", TDA7419_SUB_MID_BASS_REG,
    TDA7419_SMOOTHING_FILTER, 1, 1),
    TDA7419_DOUBLE_R_TLV("Front Speaker Playback Volume", TDA7419_ATTENUATOR_LF_REG,
    TDA7419_ATTENUATOR_RF_REG, 0x7f, -80, 15, 0x10, 0,
    tlv_volume),
    SOC_SINGLE("Left Front Soft Step Switch", TDA7419_ATTENUATOR_LF_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    SOC_SINGLE("Right Front Soft Step Switch", TDA7419_ATTENUATOR_RF_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    TDA7419_DOUBLE_R_TLV("Rear Speaker Playback Volume", TDA7419_ATTENUATOR_LR_REG,
    TDA7419_ATTENUATOR_RR_REG, 0x7f, -80, 15, 0x10, 0,
    tlv_volume),
    SOC_SINGLE("Left Rear Soft Step Switch", TDA7419_ATTENUATOR_LR_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    SOC_SINGLE("Right Rear Soft Step Switch", TDA7419_ATTENUATOR_RR_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    TDA7419_SINGLE_TLV("Mixing Capture Volume", TDA7419_MIXING_LEVEL_REG,
    0x7f, -80, 15, 0x10, 0, tlv_volume),
    SOC_SINGLE("Mixing Level Soft Step Switch", TDA7419_MIXING_LEVEL_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    TDA7419_SINGLE_TLV("Subwoofer Playback Volume", TDA7419_ATTENUATOR_SUB_REG,
    0x7f, -80, 15, 0x10, 0, tlv_volume),
    SOC_SINGLE("Subwoofer Soft Step Switch", TDA7419_ATTENUATOR_SUB_REG,
    TDA7419_VOLUME_SOFT_STEP, 1, 1),
    SOC_ENUM("Spectrum Analyzer Q Factor", soc_enum_sa_q_factor),
    SOC_ENUM("Spectrum Analyzer Reset Mode", soc_enum_reset_mode),
    SOC_ENUM("Spectrum Analyzer Source", soc_enum_sa_src),
    SOC_SINGLE("Spectrum Analyzer Run Switch", TDA7419_SA_CLK_AC_REG,
    TDA7419_SA_RUN, 1, 1),
    SOC_SINGLE("Spectrum Analyzer Reset Switch", TDA7419_SA_CLK_AC_REG,
    TDA7419_RESET, 1, 1),
    SOC_ENUM("Clock Source", soc_enum_clk_src),
    SOC_ENUM("Coupling Mode", soc_enum_coupling_mode),
    };
    static const struct snd_kcontrol_new soc_mixer_lf_output_controls[] = {
    SOC_DAPM_SINGLE("Mix to LF Speaker Switch",
    TDA7419_MIXING_GAIN_REG,
    TDA7419_MIX_LF, 1, 1),
    };
    static const struct snd_kcontrol_new soc_mixer_rf_output_controls[] = {
    SOC_DAPM_SINGLE("Mix to RF Speaker Switch",
    TDA7419_MIXING_GAIN_REG,
    TDA7419_MIX_RF, 1, 1),
    };
    static const struct snd_kcontrol_new soc_mix_enable_switch_controls[] = {
    SOC_DAPM_SINGLE("Switch", TDA7419_MIXING_GAIN_REG,
    TDA7419_MIX_ENABLE, 1, 1),
    };
    static const struct snd_kcontrol_new soc_sub_enable_switch_controls[] = {
    SOC_DAPM_SINGLE("Switch", TDA7419_MIXING_GAIN_REG,
    TDA7419_MIX_ENABLE, 1, 1),
    };
    static const struct snd_soc_dapm_widget tda7419_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("SE3L"),
    SND_SOC_DAPM_INPUT("SE3R"),
    SND_SOC_DAPM_INPUT("SE2L"),
    SND_SOC_DAPM_INPUT("SE2R"),
    SND_SOC_DAPM_INPUT("SE1L"),
    SND_SOC_DAPM_INPUT("SE1R"),
    SND_SOC_DAPM_INPUT("DIFFL"),
    SND_SOC_DAPM_INPUT("DIFFR"),
    SND_SOC_DAPM_INPUT("MIX"),
    SND_SOC_DAPM_MUX("Main Source Select", SND_SOC_NOPM,
    0, 0, &soc_mux_main_src_sel),
    SND_SOC_DAPM_MUX("Second Source Select", SND_SOC_NOPM,
    0, 0, &soc_mux_second_src_sel),
    SND_SOC_DAPM_MUX("Rear Speaker Source", SND_SOC_NOPM,
    0, 0, &soc_mux_rear_spkr_src),
    SND_SOC_DAPM_SWITCH("Mix Enable", SND_SOC_NOPM,
    0, 0, &soc_mix_enable_switch_controls[0]),
    SND_SOC_DAPM_MIXER_NAMED_CTL("LF Output Mixer", SND_SOC_NOPM,
    0, 0, &soc_mixer_lf_output_controls[0],
    ARRAY_SIZE(soc_mixer_lf_output_controls)),
    SND_SOC_DAPM_MIXER_NAMED_CTL("RF Output Mixer", SND_SOC_NOPM,
    0, 0, &soc_mixer_rf_output_controls[0],
    ARRAY_SIZE(soc_mixer_rf_output_controls)),
    SND_SOC_DAPM_SWITCH("Subwoofer Enable",
    SND_SOC_NOPM, 0, 0,
    &soc_sub_enable_switch_controls[0]),
    SND_SOC_DAPM_OUTPUT("OUTLF"),
    SND_SOC_DAPM_OUTPUT("OUTRF"),
    SND_SOC_DAPM_OUTPUT("OUTLR"),
    SND_SOC_DAPM_OUTPUT("OUTRR"),
    SND_SOC_DAPM_OUTPUT("OUTSW"),
    };
    static const struct snd_soc_dapm_route tda7419_dapm_routes[] = {
    {"Main Source Select", "SE3", "SE3L"},
    {"Main Source Select", "SE3", "SE3R"},
    {"Main Source Select", "SE2", "SE2L"},
    {"Main Source Select", "SE2", "SE2R"},
    {"Main Source Select", "SE1", "SE1L"},
    {"Main Source Select", "SE1", "SE1R"},
    {"Main Source Select", "SE", "DIFFL"},
    {"Main Source Select", "SE", "DIFFR"},
    {"Main Source Select", "QD", "DIFFL"},
    {"Main Source Select", "QD", "DIFFR"},
    {"Second Source Select", "SE3", "SE3L"},
    {"Second Source Select", "SE3", "SE3R"},
    {"Second Source Select", "SE2", "SE2L"},
    {"Second Source Select", "SE2", "SE2R"},
    {"Second Source Select", "SE1", "SE1L"},
    {"Second Source Select", "SE1", "SE1R"},
    {"Second Source Select", "SE", "DIFFL"},
    {"Second Source Select", "SE", "DIFFR"},
    {"Second Source Select", "QD", "DIFFL"},
    {"Second Source Select", "QD", "DIFFR"},
    {"Rear Speaker Source", "Main", "Main Source Select"},
    {"Rear Speaker Source", "Second", "Second Source Select"},
    {"Subwoofer Enable", "Switch", "Main Source Select"},
    {"Mix Enable", "Switch", "MIX"},
    {"LF Output Mixer", core::ptr::null_mut(), "Main Source Select"},
    {"LF Output Mixer", "Mix to LF Speaker Switch", "Mix Enable"},
    {"RF Output Mixer", core::ptr::null_mut(), "Main Source Select"},
    {"RF Output Mixer", "Mix to RF Speaker Switch", "Mix Enable"},
    {"OUTLF", core::ptr::null_mut(), "LF Output Mixer"},
    {"OUTRF", core::ptr::null_mut(), "RF Output Mixer"},
    {"OUTLR", core::ptr::null_mut(), "Rear Speaker Source"},
    {"OUTRR", core::ptr::null_mut(), "Rear Speaker Source"},
    {"OUTSW", core::ptr::null_mut(), "Subwoofer Enable"},
    };
    static const struct snd_soc_component_driver tda7419_component_driver = {
    .name			= "tda7419",
    .controls		= tda7419_controls,
    .num_controls		= ARRAY_SIZE(tda7419_controls),
    .dapm_widgets		= tda7419_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(tda7419_dapm_widgets),
    .dapm_routes		= tda7419_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(tda7419_dapm_routes),
    };
#[no_mangle]
unsafe extern "C" fn tda7419_probe(i2c: *mut i2c_client) -> c_int {
    static int tda7419_probe(struct i2c_client *i2c)
    {
    struct tda7419_data *tda7419;
    int i, ret;
    tda7419 = devm_kzalloc(&i2c.dev,
    sizeof(struct tda7419_data),
    GFP_KERNEL);
    if (tda7419 == core::ptr::null_mut())
    return -ENOMEM;
    i2c_set_clientdata(i2c, tda7419);
    tda7419.regmap = devm_regmap_init_i2c(i2c, &tda7419_regmap_config);
    if (IS_ERR(tda7419.regmap)) {
    ret = PTR_ERR(tda7419.regmap);
    dev_err(&i2c.dev, "error initializing regmap: %d\n",
    ret);
    return ret;
    }
//
// Reset registers to power-on defaults. The part does not provide a
// soft-reset function and the registers are not readable. This ensures
// that the cache matches register contents even if the registers have
// been previously initialized and not power cycled before probe.
//
    for (i = 0; i < ARRAY_SIZE(tda7419_regmap_defaults); i++)
    regmap_write(tda7419.regmap,
    tda7419_regmap_defaults[i].reg,
    tda7419_regmap_defaults[i].def);
    ret = devm_snd_soc_register_component(&i2c.dev,
    &tda7419_component_driver, core::ptr::null_mut(), 0);
    if (ret < 0) {
    dev_err(&i2c.dev, "error registering component: %d\n",
    ret);
    }
    return ret;
    }
    static const struct i2c_device_id tda7419_i2c_id[] = {
    { .name = "tda7419" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tda7419_i2c_id);
    static const struct of_device_id tda7419_of_match[] = {
    { .compatible = "st,tda7419" },
    { },
    };
    MODULE_DEVICE_TABLE(of, tda7419_of_match);
    static struct i2c_driver tda7419_driver = {
    .driver = {
    .name   = "tda7419",
    .of_match_table = tda7419_of_match,
    },
    .probe          = tda7419_probe,
    .id_table       = tda7419_i2c_id,
    };
    module_i2c_driver(tda7419_driver);
    MODULE_AUTHOR("Matt Porter <mporter@konsulko.com>");
    MODULE_DESCRIPTION("TDA7419 audio processor driver");
    MODULE_LICENSE("GPL");
