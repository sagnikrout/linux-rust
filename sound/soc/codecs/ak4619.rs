//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ak4619.c
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
// ak4619.c -- Asahi Kasei ALSA SoC Audio driver
//
// Copyright (C) 2023 Renesas Electronics Corporation
// Khanh Le <khanh.le.xr@renesas.com>
//
// Based on ak4613.c by Kuninori Morimoto
// Based on da7213.c by Adam Thomson
// Based on ak4641.c by Harald Welte
//

//
// Registers
//
pub const PWR_MGMT: c_uint = 0x00	/* Power Management */;
pub const AU_IFF1: c_uint = 0x01	/* Audio I/F Format */;
pub const AU_IFF2: c_uint = 0x02	/* Audio I/F Format (Extended) */;
pub const SYS_CLK: c_uint = 0x03	/* System Clock Setting */;
pub const MIC_AMP1: c_uint = 0x04	/* MIC AMP Gain 1 */;
pub const MIC_AMP2: c_uint = 0x05	/* MIC AMP Gain 2 */;
pub const LADC1: c_uint = 0x06	/* ADC1 Lch Digital Volume */;
pub const RADC1: c_uint = 0x07	/* ADC1 Rch Digital Volume */;
pub const LADC2: c_uint = 0x08	/* ADC2 Lch Digital Volume */;
pub const RADC2: c_uint = 0x09	/* ADC2 Rch Digital Volume */;
pub const ADC_DF: c_uint = 0x0a	/* ADC Digital Filter Setting */;
pub const ADC_AI: c_uint = 0x0b	/* ADC Analog Input Setting */;
pub const ADC_MHPF: c_uint = 0x0D	/* ADC Mute & HPF Control */;
pub const LDAC1: c_uint = 0x0E	/* DAC1 Lch Digital Volume */;
pub const RDAC1: c_uint = 0x0F	/* DAC1 Rch Digital Volume */;
pub const LDAC2: c_uint = 0x10	/* DAC2 Lch Digital Volume */;
pub const RDAC2: c_uint = 0x11	/* DAC2 Rch Digital Volume */;
pub const DAC_IS: c_uint = 0x12	/* DAC Input Select Setting */;
pub const DAC_DEMP: c_uint = 0x13	/* DAC De-Emphasis Setting */;
pub const DAC_MF: c_uint = 0x14	/* DAC Mute & Filter Setting */;
//
// Bit fields
//
// Power Management

// Audio_I/F Format

// Audio_I/F Format (Extended)

// System Clock Setting
pub const FS_MASK: c_uint = 0x7;
// MIC AMP Gain
pub const MGNL_SHIFT: c_int = 4;
pub const MGNR_SHIFT: c_int = 0;
pub const MGN_MAX: c_uint = 0xB;
// ADC Digital Volume
pub const VOLAD_SHIFT: c_int = 0;
pub const VOLAD_MAX: c_uint = 0xFF;
// ADC Digital Filter Setting
pub const AD1SL_SHIFT: c_int = 0;
pub const AD2SL_SHIFT: c_int = 4;
// Analog Input Select
pub const AD1LSEL_SHIFT: c_int = 6;
pub const AD1RSEL_SHIFT: c_int = 4;
pub const AD2LSEL_SHIFT: c_int = 2;
pub const AD2RSEL_SHIFT: c_int = 0;
// ADC Mute & HPF Control
pub const ATSPAD_SHIFT: c_int = 7;
pub const AD1MUTE_SHIFT: c_int = 5;
pub const AD2MUTE_SHIFT: c_int = 6;
pub const AD1MUTE_MAX: c_int = 1;
pub const AD2MUTE_MAX: c_int = 1;

pub const AD1HPFN_SHIFT: c_int = 1;
pub const AD1HPFN_MAX: c_int = 1;
pub const AD2HPFN_SHIFT: c_int = 2;
pub const AD2HPFN_MAX: c_int = 1;
// DAC Digital Volume
pub const VOLDA_SHIFT: c_int = 0;
pub const VOLDA_MAX: c_uint = 0xFF;
// DAC Input Select Setting
pub const DAC1SEL_SHIFT: c_int = 0;
pub const DAC2SEL_SHIFT: c_int = 2;
// DAC De-Emphasis Setting

pub const DEM1_SHIFT: c_int = 0;
pub const DEM2_SHIFT: c_int = 2;
// DAC Mute & Filter Setting
pub const DA1MUTE_SHIFT: c_int = 4;
pub const DA1MUTE_MAX: c_int = 1;
pub const DA2MUTE_SHIFT: c_int = 5;
pub const DA2MUTE_MAX: c_int = 1;

pub const ATSPDA_SHIFT: c_int = 7;
pub const DA1SL_SHIFT: c_int = 0;
pub const DA2SL_SHIFT: c_int = 2;
// Codec private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4619_priv {
    pub regmap: *mut regmap,
    pub constraint: snd_pcm_hw_constraint_list,
    pub deemph_en: c_int,
    pub playback_rate: c_uint,
    pub sysclk: c_uint,
}

//
// DAC Volume
//
// max : 0x00 : +12.0 dB
// ( 0.5 dB step )
// min : 0xFE : -115.0 dB
// mute: 0xFF
//
    static const DECLARE_TLV_DB_SCALE(dac_tlv, -11550, 50, 1);
//
// MIC Volume
//
// max : 0x0B : +27.0 dB
// ( 3 dB step )
// min: 0x00 : -6.0 dB
//
    static const DECLARE_TLV_DB_SCALE(mic_tlv, -600, 300, 0);
//
// ADC Volume
//
// max : 0x00 : +24.0 dB
// ( 0.5 dB step )
// min : 0xFE : -103.0 dB
// mute: 0xFF
//
    static const DECLARE_TLV_DB_SCALE(adc_tlv, -10350, 50, 1);
// ADC & DAC Volume Level Transition Time select
    static const char * const ak4619_vol_trans_txt[] = {
    "4/fs", "16/fs"
    };
    static SOC_ENUM_SINGLE_DECL(ak4619_adc_vol_trans, ADC_MHPF, ATSPAD_SHIFT, ak4619_vol_trans_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_vol_trans, DAC_MF,   ATSPDA_SHIFT, ak4619_vol_trans_txt);
// ADC Digital Filter select
    static const char * const ak4619_adc_digi_fil_txt[] = {
    "Sharp Roll-Off Filter",
    "Slow Roll-Off Filter",
    "Short Delay Sharp Roll-Off Filter",
    "Short Delay Slow Roll-Off Filter",
    "Voice Filter"
    };
    static SOC_ENUM_SINGLE_DECL(ak4619_adc_1_digi_fil, ADC_DF, AD1SL_SHIFT, ak4619_adc_digi_fil_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_adc_2_digi_fil, ADC_DF, AD2SL_SHIFT, ak4619_adc_digi_fil_txt);
// DAC De-Emphasis Filter select
    static const char * const ak4619_dac_de_emp_txt[] = {
    "44.1kHz", "OFF", "48kHz", "32kHz"
    };
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_1_de_emp, DAC_DEMP, DEM1_SHIFT, ak4619_dac_de_emp_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_2_de_emp, DAC_DEMP, DEM2_SHIFT, ak4619_dac_de_emp_txt);
// DAC Digital Filter select
    static const char * const ak4619_dac_digi_fil_txt[] = {
    "Sharp Roll-Off Filter",
    "Slow Roll-Off Filter",
    "Short Delay Sharp Roll-Off Filter",
    "Short Delay Slow Roll-Off Filter"
    };
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_1_digi_fil, DAC_MF, DA1SL_SHIFT, ak4619_dac_digi_fil_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_2_digi_fil, DAC_MF, DA2SL_SHIFT, ak4619_dac_digi_fil_txt);
//
// Control functions
//
#[no_mangle]
unsafe extern "C" fn ak4619_set_deemph(component: *mut snd_soc_component) {
    static void ak4619_set_deemph(struct snd_soc_component *component)
    {
    struct ak4619_priv *ak4619 = snd_soc_component_get_drvdata(component);
    let mut dem: u8 = 0;
    if (!ak4619.deemph_en)
    return;
    switch (ak4619.playback_rate) {
    case 32000:
    dem |= DEM1_32000 | DEM2_32000;
    break;
    case 44100:
    dem |= DEM1_44100 | DEM2_44100;
    break;
    case 48000:
    dem |= DEM1_48000 | DEM2_48000;
    break;
    default:
    dem |= DEM1_OFF | DEM2_OFF;
    break;
    }
    snd_soc_component_update_bits(component, DAC_DEMP, DEM1_MASK | DEM2_MASK, dem);
    }
    static int ak4619_put_deemph(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct ak4619_priv *ak4619 = snd_soc_component_get_drvdata(component);
    let mut deemph_en: c_int = ucontrol.value.integer.value[0];
    let mut ret: c_int = 0;
    switch (deemph_en) {
    case 0:
    case 1:
    break;
    default:
    return -EINVAL;
    }
    if (ak4619.deemph_en != deemph_en)
    ret = 1; /* The value changed */
    ak4619.deemph_en = deemph_en;
    ak4619_set_deemph(component);
    return ret;
    }
    static int ak4619_get_deemph(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct ak4619_priv *ak4619 = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = ak4619.deemph_en;
    return 0;
    };
//
// KControls
//
    static const struct snd_kcontrol_new ak4619_snd_controls[] = {
// Volume controls
    SOC_DOUBLE_R_TLV("DAC 1 Volume", LDAC1, RDAC1, VOLDA_SHIFT, VOLDA_MAX, 1, dac_tlv),
    SOC_DOUBLE_R_TLV("DAC 2 Volume", LDAC2, RDAC2, VOLDA_SHIFT, VOLDA_MAX, 1, dac_tlv),
    SOC_DOUBLE_R_TLV("ADC 1 Volume", LADC1, RADC1, VOLAD_SHIFT, VOLAD_MAX, 1, adc_tlv),
    SOC_DOUBLE_R_TLV("ADC 2 Volume", LADC2, RADC2, VOLAD_SHIFT, VOLAD_MAX, 1, adc_tlv),
    SOC_DOUBLE_TLV("Mic 1 Volume", MIC_AMP1, MGNL_SHIFT, MGNR_SHIFT, MGN_MAX, 0, mic_tlv),
    SOC_DOUBLE_TLV("Mic 2 Volume", MIC_AMP2, MGNL_SHIFT, MGNR_SHIFT, MGN_MAX, 0, mic_tlv),
// Volume Level Transition Time controls
    SOC_ENUM("ADC Volume Level Transition Time", ak4619_adc_vol_trans),
    SOC_ENUM("DAC Volume Level Transition Time", ak4619_dac_vol_trans),
// Mute controls
    SOC_SINGLE("DAC 1 Switch", DAC_MF, DA1MUTE_SHIFT, DA1MUTE_MAX, 1),
    SOC_SINGLE("DAC 2 Switch", DAC_MF, DA2MUTE_SHIFT, DA2MUTE_MAX, 1),
    SOC_SINGLE("ADC 1 Switch", ADC_MHPF, AD1MUTE_SHIFT, AD1MUTE_MAX, 1),
    SOC_SINGLE("ADC 2 Switch", ADC_MHPF, AD2MUTE_SHIFT, AD2MUTE_MAX, 1),
// Filter controls
    SOC_ENUM("ADC 1 Digital Filter", ak4619_adc_1_digi_fil),
    SOC_ENUM("ADC 2 Digital Filter", ak4619_adc_2_digi_fil),
    SOC_SINGLE("ADC 1 HPF", ADC_MHPF, AD1HPFN_SHIFT, AD1HPFN_MAX, 1),
    SOC_SINGLE("ADC 2 HPF", ADC_MHPF, AD2HPFN_SHIFT, AD2HPFN_MAX, 1),
    SOC_ENUM("DAC 1 De-Emphasis Filter", ak4619_dac_1_de_emp),
    SOC_ENUM("DAC 2 De-Emphasis Filter", ak4619_dac_2_de_emp),
    SOC_ENUM("DAC 1 Digital Filter", ak4619_dac_1_digi_fil),
    SOC_ENUM("DAC 2 Digital Filter", ak4619_dac_2_digi_fil),
    SOC_SINGLE_BOOL_EXT("Playback De-Emphasis Switch", 0, ak4619_get_deemph, ak4619_put_deemph),
    };
//
// DAPM
//
// Analog input mode
    static const char * const ak4619_analog_in_txt[] = {
    "Differential", "Single-Ended1", "Single-Ended2", "Pseudo Differential"
    };
    static SOC_ENUM_SINGLE_DECL(ak4619_ad_1_left_in,  ADC_AI, AD1LSEL_SHIFT, ak4619_analog_in_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_ad_1_right_in, ADC_AI, AD1RSEL_SHIFT, ak4619_analog_in_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_ad_2_left_in,  ADC_AI, AD2LSEL_SHIFT, ak4619_analog_in_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_ad_2_right_in, ADC_AI, AD2RSEL_SHIFT, ak4619_analog_in_txt);
    static const struct snd_kcontrol_new ak4619_ad_1_left_in_mux =
    SOC_DAPM_ENUM("Analog Input 1 Left MUX",  ak4619_ad_1_left_in);
    static const struct snd_kcontrol_new ak4619_ad_1_right_in_mux =
    SOC_DAPM_ENUM("Analog Input 1 Right MUX", ak4619_ad_1_right_in);
    static const struct snd_kcontrol_new ak4619_ad_2_left_in_mux =
    SOC_DAPM_ENUM("Analog Input 2 Left MUX",  ak4619_ad_2_left_in);
    static const struct snd_kcontrol_new ak4619_ad_2_right_in_mux =
    SOC_DAPM_ENUM("Analog Input 2 Right MUX", ak4619_ad_2_right_in);
// DAC source mux
    static const char * const ak4619_dac_in_txt[] = {
    "SDIN1", "SDIN2", "SDOUT1", "SDOUT2"
    };
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_1_in, DAC_IS, DAC1SEL_SHIFT, ak4619_dac_in_txt);
    static SOC_ENUM_SINGLE_DECL(ak4619_dac_2_in, DAC_IS, DAC2SEL_SHIFT, ak4619_dac_in_txt);
    static const struct snd_kcontrol_new ak4619_dac_1_in_mux =
    SOC_DAPM_ENUM("DAC 1 Source MUX", ak4619_dac_1_in);
    static const struct snd_kcontrol_new ak4619_dac_2_in_mux =
    SOC_DAPM_ENUM("DAC 2 Source MUX", ak4619_dac_2_in);
    static const struct snd_soc_dapm_widget ak4619_dapm_widgets[] = {
// DACs
    SND_SOC_DAPM_DAC("DAC1", core::ptr::null_mut(), PWR_MGMT, 1, 0),
    SND_SOC_DAPM_DAC("DAC2", core::ptr::null_mut(), PWR_MGMT, 2, 0),
// ADCs
    SND_SOC_DAPM_ADC("ADC1", core::ptr::null_mut(), PWR_MGMT, 4, 0),
    SND_SOC_DAPM_ADC("ADC2", core::ptr::null_mut(), PWR_MGMT, 5, 0),
// Outputs
    SND_SOC_DAPM_OUTPUT("AOUT1L"),
    SND_SOC_DAPM_OUTPUT("AOUT2L"),
    SND_SOC_DAPM_OUTPUT("AOUT1R"),
    SND_SOC_DAPM_OUTPUT("AOUT2R"),
// Inputs
    SND_SOC_DAPM_INPUT("AIN1L"),
    SND_SOC_DAPM_INPUT("AIN2L"),
    SND_SOC_DAPM_INPUT("AIN4L"),
    SND_SOC_DAPM_INPUT("AIN5L"),
    SND_SOC_DAPM_INPUT("AIN1R"),
    SND_SOC_DAPM_INPUT("AIN2R"),
    SND_SOC_DAPM_INPUT("AIN4R"),
    SND_SOC_DAPM_INPUT("AIN5R"),
    SND_SOC_DAPM_INPUT("MIC1L"),
    SND_SOC_DAPM_INPUT("MIC1R"),
    SND_SOC_DAPM_INPUT("MIC2L"),
    SND_SOC_DAPM_INPUT("MIC2R"),
// DAI
    SND_SOC_DAPM_AIF_IN("SDIN1",  "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("SDIN2",  "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("SDOUT1", "Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("SDOUT2", "Capture", 0, SND_SOC_NOPM, 0, 0),
// MUXs for Mic PGA source selection
    SND_SOC_DAPM_MUX("Analog Input 1 Left MUX",  SND_SOC_NOPM, 0, 0, &ak4619_ad_1_left_in_mux),
    SND_SOC_DAPM_MUX("Analog Input 1 Right MUX", SND_SOC_NOPM, 0, 0, &ak4619_ad_1_right_in_mux),
    SND_SOC_DAPM_MUX("Analog Input 2 Left MUX",  SND_SOC_NOPM, 0, 0, &ak4619_ad_2_left_in_mux),
    SND_SOC_DAPM_MUX("Analog Input 2 Right MUX", SND_SOC_NOPM, 0, 0, &ak4619_ad_2_right_in_mux),
// MUXs for DAC source selection
    SND_SOC_DAPM_MUX("DAC 1 Source MUX", SND_SOC_NOPM, 0, 0, &ak4619_dac_1_in_mux),
    SND_SOC_DAPM_MUX("DAC 2 Source MUX", SND_SOC_NOPM, 0, 0, &ak4619_dac_2_in_mux),
    };
    static const struct snd_soc_dapm_route ak4619_intercon[] = {
// Dest       Connecting Widget    Source
// Output path
    {"AOUT1L", core::ptr::null_mut(), "DAC1"},
    {"AOUT2L", core::ptr::null_mut(), "DAC2"},
    {"AOUT1R", core::ptr::null_mut(), "DAC1"},
    {"AOUT2R", core::ptr::null_mut(), "DAC2"},
    {"DAC1", core::ptr::null_mut(), "DAC 1 Source MUX"},
    {"DAC2", core::ptr::null_mut(), "DAC 2 Source MUX"},
    {"DAC 1 Source MUX", "SDIN1",  "SDIN1"},
    {"DAC 1 Source MUX", "SDIN2",  "SDIN2"},
    {"DAC 1 Source MUX", "SDOUT1", "SDOUT1"},
    {"DAC 1 Source MUX", "SDOUT2", "SDOUT2"},
    {"DAC 2 Source MUX", "SDIN1",  "SDIN1"},
    {"DAC 2 Source MUX", "SDIN2",  "SDIN2"},
    {"DAC 2 Source MUX", "SDOUT1", "SDOUT1"},
    {"DAC 2 Source MUX", "SDOUT2", "SDOUT2"},
// Input path
    {"SDOUT1", core::ptr::null_mut(), "ADC1"},
    {"SDOUT2", core::ptr::null_mut(), "ADC2"},
    {"ADC1", core::ptr::null_mut(), "Analog Input 1 Left MUX"},
    {"ADC1", core::ptr::null_mut(), "Analog Input 1 Right MUX"},
    {"ADC2", core::ptr::null_mut(), "Analog Input 2 Left MUX"},
    {"ADC2", core::ptr::null_mut(), "Analog Input 2 Right MUX"},
    {"Analog Input 1 Left MUX", "Differential",		"MIC1L"},
    {"Analog Input 1 Left MUX", "Single-Ended1",		"MIC1L"},
    {"Analog Input 1 Left MUX", "Single-Ended2",		"MIC1L"},
    {"Analog Input 1 Left MUX", "Pseudo Differential",	"MIC1L"},
    {"Analog Input 1 Right MUX", "Differential",		"MIC1R"},
    {"Analog Input 1 Right MUX", "Single-Ended1",		"MIC1R"},
    {"Analog Input 1 Right MUX", "Single-Ended2",		"MIC1R"},
    {"Analog Input 1 Right MUX", "Pseudo Differential",	"MIC1R"},
    {"Analog Input 2 Left MUX", "Differential",		"MIC2L"},
    {"Analog Input 2 Left MUX", "Single-Ended1",		"MIC2L"},
    {"Analog Input 2 Left MUX", "Single-Ended2",		"MIC2L"},
    {"Analog Input 2 Left MUX", "Pseudo Differential",	"MIC2L"},
    {"Analog Input 2 Right MUX", "Differential",		"MIC2R"},
    {"Analog Input 2 Right MUX", "Single-Ended1",		"MIC2R"},
    {"Analog Input 2 Right MUX", "Single-Ended2",		"MIC2R"},
    {"Analog Input 2 Right MUX", "Pseudo Differential",	"MIC2R"},
    {"MIC1L", core::ptr::null_mut(), "AIN1L"},
    {"MIC1L", core::ptr::null_mut(), "AIN2L"},
    {"MIC1R", core::ptr::null_mut(), "AIN1R"},
    {"MIC1R", core::ptr::null_mut(), "AIN2R"},
    {"MIC2L", core::ptr::null_mut(), "AIN4L"},
    {"MIC2L", core::ptr::null_mut(), "AIN5L"},
    {"MIC2R", core::ptr::null_mut(), "AIN4R"},
    {"MIC2R", core::ptr::null_mut(), "AIN5R"},
    };
    static const struct reg_default ak4619_reg_defaults[] = {
    { PWR_MGMT,	0x00 },
    { AU_IFF1,	0x0C },
    { AU_IFF2,	0x0C },
    { SYS_CLK,	0x00 },
    { MIC_AMP1,	0x22 },
    { MIC_AMP2,	0x22 },
    { LADC1,	0x30 },
    { RADC1,	0x30 },
    { LADC2,	0x30 },
    { RADC2,	0x30 },
    { ADC_DF,	0x00 },
    { ADC_AI,	0x00 },
    { ADC_MHPF,	0x00 },
    { LDAC1,	0x18 },
    { RDAC1,	0x18 },
    { LDAC2,	0x18 },
    { RDAC2,	0x18 },
    { DAC_IS,	0x04 },
    { DAC_DEMP,	0x05 },
    { DAC_MF,	0x0A },
    };
    static int ak4619_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    let mut pwr_ctrl: u8 = 0;
    switch (level) {
    case SND_SOC_BIAS_ON:
    pwr_ctrl |= RSTN;
    fallthrough;
    case SND_SOC_BIAS_PREPARE:
    pwr_ctrl |= PMAD1 | PMAD2 | PMDA1 | PMDA2;
    fallthrough;
    case SND_SOC_BIAS_STANDBY:
    case SND_SOC_BIAS_OFF:
    default:
    break;
    }
    snd_soc_component_write(component, PWR_MGMT, pwr_ctrl);
    return 0;
    }
    static int ak4619_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4619_priv *ak4619 = snd_soc_component_get_drvdata(component);
    unsigned int width;
    unsigned int rate;
    unsigned int fs;
    let mut is_play: bool = substream.stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut dai_ctrl: u8 = 0;
    let mut clk_mode: u8 = 0;
    width = params_width(params);
    switch (width) {
    case 16:
    dai_ctrl |= is_play ? DIDL_16 : DODL_16;
    break;
    case 20:
    dai_ctrl |= is_play ? DIDL_20 : DODL_20;
    break;
    case 24:
    dai_ctrl |= is_play ? DIDL_24 : DODL_24;
    break;
    case 32:
    if (is_play)
    dai_ctrl |= DIDL_32;
    else
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    rate = params_rate(params);
    if (rate)
    fs = ak4619.sysclk / rate;
    else
    return -EINVAL;
    switch (rate) {
    case  8000:
    case 11025:
    case 12000:
    case 16000:
    case 22050:
    case 24000:
    case 32000:
    case 44100:
    case 48000:
    switch (fs) {
    case 256:
    clk_mode |= (0x0 << 0);
    break;
    case 384:
    clk_mode |= (0x2 << 0);
    break;
    case 512:
    clk_mode |= (0x3 << 0);
    break;
    default:
    return -EINVAL;
    }
    break;
    case 64000:
    case 88200:
    case 96000:
    if (fs == 256)
    clk_mode |= (0x1 << 0);
    else
    return -EINVAL;
    break;
    case 176400:
    case 192000:
    if (fs == 128)
    clk_mode |= (0x4 << 0);
    else
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    snd_soc_component_update_bits(component, SYS_CLK, FS_MASK, clk_mode);
    snd_soc_component_update_bits(component, AU_IFF2,
    is_play ? DIDL_MASK : DODL_MASK, dai_ctrl);
    if (is_play) {
    ak4619.playback_rate = rate;
    ak4619_set_deemph(component);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4619_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int ak4619_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct snd_soc_component *component = dai.component;
    let mut dai_fmt1: u8 = 0;
    let mut dai_fmt2: u8 = 0;
// Set clock normal/inverted
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    break;
    case SND_SOC_DAIFMT_IB_NF:
    dai_fmt1 |= BCKP;
    break;
    case SND_SOC_DAIFMT_NB_IF:
    case SND_SOC_DAIFMT_IB_IF:
    default:
    return -EINVAL;
    }
// Only Stereo modes are supported
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    dai_fmt1 |= DCF_STEREO_I2S;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    dai_fmt1 |= DCF_STEREO_MSB;
    break;
    case SND_SOC_DAIFMT_DSP_A: /* L data MSB after FRM LRC */
    dai_fmt1 |= DCF_PCM_SF;
    dai_fmt2 |= SLOT;
    break;
    case SND_SOC_DAIFMT_DSP_B: /* L data MSB during FRM LRC */
    dai_fmt1 |= DCF_PCM_LF;
    dai_fmt2 |= SLOT;
    break;
    default:
    return -EINVAL;
    }
// Only slave mode is support
    switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
    case SND_SOC_DAIFMT_CBC_CFC:
    break;
    default:
    return -EINVAL;
    }
// By default only 64 BICK per LRCLK is supported
    dai_fmt1 |= DSL_32;
    snd_soc_component_update_bits(component, AU_IFF1, DCF_MASK |
    DSL_MASK | BCKP, dai_fmt1);
    snd_soc_component_update_bits(component, AU_IFF2, SLOT, dai_fmt2);
    return 0;
    }
    static int ak4619_dai_set_sysclk(struct snd_soc_dai *codec_dai,
    int clk_id, unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct ak4619_priv *ak4619 = snd_soc_component_get_drvdata(component);
    ak4619.sysclk = freq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4619_dai_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    static int ak4619_dai_mute(struct snd_soc_dai *dai, int mute, int direction)
    {
    struct snd_soc_component *component = dai.component;
    snd_soc_component_update_bits(component, DAC_MF, DA1MUTE_EN, mute ? DA1MUTE_EN : 0);
    snd_soc_component_update_bits(component, DAC_MF, DA2MUTE_EN, mute ? DA2MUTE_EN : 0);
    return 0;
    }
    static void ak4619_hw_constraints(struct ak4619_priv *ak4619,
    struct snd_pcm_runtime *runtime)
    {
    struct snd_pcm_hw_constraint_list *constraint = &ak4619.constraint;
    let mut ak4619_rate_mask: c_int = 0;
    unsigned int fs;
    int i;
    static const unsigned int ak4619_sr[] = {
    8000,
    11025,
    12000,
    16000,
    22050,
    24000,
    32000,
    44100,
    48000,
    64000,
    88200,
    96000,
    176400,
    192000,
    };
//
// [8kHz - 48kHz]		: 256fs, 384fs or 512fs
// [64kHz - 96kHz]		: 256fs
// [176.4kHz, 192kHz]	: 128fs
//
    for (i = 0; i < ARRAY_SIZE(ak4619_sr); i++) {
    fs = ak4619.sysclk / ak4619_sr[i];
    switch (fs) {
    case 512:
    case 384:
    case 256:
    ak4619_rate_mask |= (1 << i);
    break;
    case 128:
    switch (i) {
    case (ARRAY_SIZE(ak4619_sr) - 1):
    case (ARRAY_SIZE(ak4619_sr) - 2):
    ak4619_rate_mask |= (1 << i);
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    }
    constraint.list	= ak4619_sr;
    constraint.mask	= ak4619_rate_mask;
    constraint.count	= ARRAY_SIZE(ak4619_sr);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, constraint);
    };
pub const PLAYBACK_MODE: c_int = 0;
pub const CAPTURE_MODE: c_int = 1;
    static int ak4619_dai_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4619_priv *ak4619 = snd_soc_component_get_drvdata(component);
    ak4619_hw_constraints(ak4619, substream.runtime);
    return 0;
    }
    static u64 ak4619_dai_formats[] = {
// First Priority
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J,
// Second Priority
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_A	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_B,
    };
    static const struct snd_soc_dai_ops ak4619_dai_ops = {
    .startup			= ak4619_dai_startup,
    .set_sysclk			= ak4619_dai_set_sysclk,
    .set_fmt			= ak4619_dai_set_fmt,
    .hw_params			= ak4619_dai_hw_params,
    .mute_stream			= ak4619_dai_mute,
    .auto_selectable_formats	= ak4619_dai_formats,
    .num_auto_selectable_formats	= ARRAY_SIZE(ak4619_dai_formats),
    };
    static const struct snd_soc_component_driver soc_component_dev_ak4619 = {
    .set_bias_level		= ak4619_set_bias_level,
    .controls		= ak4619_snd_controls,
    .num_controls		= ARRAY_SIZE(ak4619_snd_controls),
    .dapm_widgets		= ak4619_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ak4619_dapm_widgets),
    .dapm_routes		= ak4619_intercon,
    .num_dapm_routes	= ARRAY_SIZE(ak4619_intercon),
    .idle_bias_on		= 1,
    .endianness		= 1,
    };
    static const struct regmap_config ak4619_regmap_cfg = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= 0x14,
    .reg_defaults		= ak4619_reg_defaults,
    .num_reg_defaults	= ARRAY_SIZE(ak4619_reg_defaults),
    .cache_type		= REGCACHE_MAPLE,
    };
    static const struct of_device_id ak4619_of_match[] = {
    { .compatible = "asahi-kasei,ak4619",	.data = &ak4619_regmap_cfg },
    {},
    };
    MODULE_DEVICE_TABLE(of, ak4619_of_match);
    static const struct i2c_device_id ak4619_i2c_id[] = {
    { .name = "ak4619", .driver_data = (kernel_ulong_t)&ak4619_regmap_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ak4619_i2c_id);

    SNDRV_PCM_FMTBIT_S20_LE |\
    SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S32_LE)

    SNDRV_PCM_FMTBIT_S20_LE |\
    SNDRV_PCM_FMTBIT_S24_LE)
    static struct snd_soc_dai_driver ak4619_dai = {
    .name = "ak4619-hifi",
    .playback = {
    .stream_name	= "Playback",
    .channels_min	= 1,
    .channels_max	= 2,
    .rates		= AK4619_RATES,
    .formats	= AK4619_DAC_FORMATS,
    },
    .capture = {
    .stream_name	= "Capture",
    .channels_min	= 1,
    .channels_max	= 2,
    .rates		= AK4619_RATES,
    .formats	= AK4619_ADC_FORMATS,
    },
    .ops			= &ak4619_dai_ops,
    .symmetric_rate		= 1,
    };
#[no_mangle]
unsafe extern "C" fn ak4619_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ak4619_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct ak4619_priv *ak4619;
    int ret;
    ak4619 = devm_kzalloc(dev, sizeof(*ak4619), GFP_KERNEL);
    if (!ak4619)
    return -ENOMEM;
    i2c_set_clientdata(i2c, ak4619);
    ak4619.regmap = devm_regmap_init_i2c(i2c, &ak4619_regmap_cfg);
    if (IS_ERR(ak4619.regmap)) {
    ret = PTR_ERR(ak4619.regmap);
    dev_err(dev, "regmap_init() failed: %d\n", ret);
    return ret;
    }
    ret = devm_snd_soc_register_component(dev, &soc_component_dev_ak4619,
    &ak4619_dai, 1);
    if (ret < 0) {
    dev_err(dev, "Failed to register ak4619 component: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static struct i2c_driver ak4619_i2c_driver = {
    .driver = {
    .name = "ak4619-codec",
    .of_match_table = ak4619_of_match,
    },
    .probe		= ak4619_i2c_probe,
    .id_table	= ak4619_i2c_id,
    };
    module_i2c_driver(ak4619_i2c_driver);
    MODULE_DESCRIPTION("SoC AK4619 driver");
    MODULE_AUTHOR("Khanh Le <khanh.le.xr@renesas.com>");
    MODULE_LICENSE("GPL v2");
