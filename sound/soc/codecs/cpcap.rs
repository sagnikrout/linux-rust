//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cpcap.c
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
// ALSA SoC CPCAP codec driver
//
// Copyright (C) 2017 - 2018 Sebastian Reichel <sre@kernel.org>
//
// Very loosely based on original driver from Motorola:
// Copyright (C) 2007 - 2009 Motorola, Inc.
//

// Register 8 - CPCAP_REG_INTS1  --- Interrupt Sense 1

// Register 9 - CPCAP_REG_INTS2   --- Interrupt Sense 2

// Register 513 CPCAP_REG_CC     --- CODEC
pub const CPCAP_BIT_CDC_CLK2: c_int = 15;
pub const CPCAP_BIT_CDC_CLK1: c_int = 14;
pub const CPCAP_BIT_CDC_CLK0: c_int = 13;
pub const CPCAP_BIT_CDC_SR3: c_int = 12;
pub const CPCAP_BIT_CDC_SR2: c_int = 11;
pub const CPCAP_BIT_CDC_SR1: c_int = 10;
pub const CPCAP_BIT_CDC_SR0: c_int = 9;
pub const CPCAP_BIT_CDC_CLOCK_TREE_RESET: c_int = 8;
pub const CPCAP_BIT_MIC2_CDC_EN: c_int = 7;
pub const CPCAP_BIT_CDC_EN_RX: c_int = 6;
pub const CPCAP_BIT_DF_RESET: c_int = 5;
pub const CPCAP_BIT_MIC1_CDC_EN: c_int = 4;
pub const CPCAP_BIT_AUDOHPF_1: c_int = 3;
pub const CPCAP_BIT_AUDOHPF_0: c_int = 2;
pub const CPCAP_BIT_AUDIHPF_1: c_int = 1;
pub const CPCAP_BIT_AUDIHPF_0: c_int = 0;
// Register 514 CPCAP_REG_CDI    --- CODEC Digital Audio Interface
pub const CPCAP_BIT_CDC_PLL_SEL: c_int = 15;
pub const CPCAP_BIT_CLK_IN_SEL: c_int = 13;
pub const CPCAP_BIT_DIG_AUD_IN: c_int = 12;
pub const CPCAP_BIT_CDC_CLK_EN: c_int = 11;
pub const CPCAP_BIT_CDC_DIG_AUD_FS1: c_int = 10;
pub const CPCAP_BIT_CDC_DIG_AUD_FS0: c_int = 9;
pub const CPCAP_BIT_MIC2_TIMESLOT2: c_int = 8;
pub const CPCAP_BIT_MIC2_TIMESLOT1: c_int = 7;
pub const CPCAP_BIT_MIC2_TIMESLOT0: c_int = 6;
pub const CPCAP_BIT_MIC1_RX_TIMESLOT2: c_int = 5;
pub const CPCAP_BIT_MIC1_RX_TIMESLOT1: c_int = 4;
pub const CPCAP_BIT_MIC1_RX_TIMESLOT0: c_int = 3;
pub const CPCAP_BIT_FS_INV: c_int = 2;
pub const CPCAP_BIT_CLK_INV: c_int = 1;
pub const CPCAP_BIT_SMB_CDC: c_int = 0;
// Register 515 CPCAP_REG_SDAC   --- Stereo DAC
pub const CPCAP_BIT_FSYNC_CLK_IN_COMMON: c_int = 11;
pub const CPCAP_BIT_SLAVE_PLL_CLK_INPUT: c_int = 10;
pub const CPCAP_BIT_ST_CLOCK_TREE_RESET: c_int = 9;
pub const CPCAP_BIT_DF_RESET_ST_DAC: c_int = 8;
pub const CPCAP_BIT_ST_SR3: c_int = 7;
pub const CPCAP_BIT_ST_SR2: c_int = 6;
pub const CPCAP_BIT_ST_SR1: c_int = 5;
pub const CPCAP_BIT_ST_SR0: c_int = 4;
pub const CPCAP_BIT_ST_DAC_CLK2: c_int = 3;
pub const CPCAP_BIT_ST_DAC_CLK1: c_int = 2;
pub const CPCAP_BIT_ST_DAC_CLK0: c_int = 1;
pub const CPCAP_BIT_ST_DAC_EN: c_int = 0;
// Register 516 CPCAP_REG_SDACDI --- Stereo DAC Digital Audio Interface
pub const CPCAP_BIT_ST_L_TIMESLOT2: c_int = 13;
pub const CPCAP_BIT_ST_L_TIMESLOT1: c_int = 12;
pub const CPCAP_BIT_ST_L_TIMESLOT0: c_int = 11;
pub const CPCAP_BIT_ST_R_TIMESLOT2: c_int = 10;
pub const CPCAP_BIT_ST_R_TIMESLOT1: c_int = 9;
pub const CPCAP_BIT_ST_R_TIMESLOT0: c_int = 8;
pub const CPCAP_BIT_ST_DAC_CLK_IN_SEL: c_int = 7;
pub const CPCAP_BIT_ST_FS_INV: c_int = 6;
pub const CPCAP_BIT_ST_CLK_INV: c_int = 5;
pub const CPCAP_BIT_ST_DIG_AUD_FS1: c_int = 4;
pub const CPCAP_BIT_ST_DIG_AUD_FS0: c_int = 3;
pub const CPCAP_BIT_DIG_AUD_IN_ST_DAC: c_int = 2;
pub const CPCAP_BIT_ST_CLK_EN: c_int = 1;
pub const CPCAP_BIT_SMB_ST_DAC: c_int = 0;
// Register 517 CPCAP_REG_TXI    --- TX Interface
pub const CPCAP_BIT_PTT_TH: c_int = 15;
pub const CPCAP_BIT_PTT_CMP_EN: c_int = 14;
pub const CPCAP_BIT_HS_ID_TX: c_int = 13;
pub const CPCAP_BIT_MB_ON2: c_int = 12;
pub const CPCAP_BIT_MB_ON1L: c_int = 11;
pub const CPCAP_BIT_MB_ON1R: c_int = 10;
pub const CPCAP_BIT_RX_L_ENCODE: c_int = 9;
pub const CPCAP_BIT_RX_R_ENCODE: c_int = 8;
pub const CPCAP_BIT_MIC2_MUX: c_int = 7;
pub const CPCAP_BIT_MIC2_PGA_EN: c_int = 6;
pub const CPCAP_BIT_CDET_DIS: c_int = 5;
pub const CPCAP_BIT_EMU_MIC_MUX: c_int = 4;
pub const CPCAP_BIT_HS_MIC_MUX: c_int = 3;
pub const CPCAP_BIT_MIC1_MUX: c_int = 2;
pub const CPCAP_BIT_MIC1_PGA_EN: c_int = 1;
pub const CPCAP_BIT_DLM: c_int = 0;
// Register 518 CPCAP_REG_TXMP   --- Mic Gain
pub const CPCAP_BIT_MB_BIAS_R1: c_int = 11;
pub const CPCAP_BIT_MB_BIAS_R0: c_int = 10;
pub const CPCAP_BIT_MIC2_GAIN_4: c_int = 9;
pub const CPCAP_BIT_MIC2_GAIN_3: c_int = 8;
pub const CPCAP_BIT_MIC2_GAIN_2: c_int = 7;
pub const CPCAP_BIT_MIC2_GAIN_1: c_int = 6;
pub const CPCAP_BIT_MIC2_GAIN_0: c_int = 5;
pub const CPCAP_BIT_MIC1_GAIN_4: c_int = 4;
pub const CPCAP_BIT_MIC1_GAIN_3: c_int = 3;
pub const CPCAP_BIT_MIC1_GAIN_2: c_int = 2;
pub const CPCAP_BIT_MIC1_GAIN_1: c_int = 1;
pub const CPCAP_BIT_MIC1_GAIN_0: c_int = 0;
// Register 519 CPCAP_REG_RXOA   --- RX Output Amplifier
pub const CPCAP_BIT_UNUSED_519_15: c_int = 15;
pub const CPCAP_BIT_UNUSED_519_14: c_int = 14;
pub const CPCAP_BIT_UNUSED_519_13: c_int = 13;
pub const CPCAP_BIT_STDAC_LOW_PWR_DISABLE: c_int = 12;
pub const CPCAP_BIT_HS_LOW_PWR: c_int = 11;
pub const CPCAP_BIT_HS_ID_RX: c_int = 10;
pub const CPCAP_BIT_ST_HS_CP_EN: c_int = 9;
pub const CPCAP_BIT_EMU_SPKR_R_EN: c_int = 8;
pub const CPCAP_BIT_EMU_SPKR_L_EN: c_int = 7;
pub const CPCAP_BIT_HS_L_EN: c_int = 6;
pub const CPCAP_BIT_HS_R_EN: c_int = 5;
pub const CPCAP_BIT_A4_LINEOUT_L_EN: c_int = 4;
pub const CPCAP_BIT_A4_LINEOUT_R_EN: c_int = 3;
pub const CPCAP_BIT_A2_LDSP_L_EN: c_int = 2;
pub const CPCAP_BIT_A2_LDSP_R_EN: c_int = 1;
pub const CPCAP_BIT_A1_EAR_EN: c_int = 0;
// Register 520 CPCAP_REG_RXVC   --- RX Volume Control
pub const CPCAP_BIT_VOL_EXT3: c_int = 15;
pub const CPCAP_BIT_VOL_EXT2: c_int = 14;
pub const CPCAP_BIT_VOL_EXT1: c_int = 13;
pub const CPCAP_BIT_VOL_EXT0: c_int = 12;
pub const CPCAP_BIT_VOL_DAC3: c_int = 11;
pub const CPCAP_BIT_VOL_DAC2: c_int = 10;
pub const CPCAP_BIT_VOL_DAC1: c_int = 9;
pub const CPCAP_BIT_VOL_DAC0: c_int = 8;
pub const CPCAP_BIT_VOL_DAC_LSB_1dB1: c_int = 7;
pub const CPCAP_BIT_VOL_DAC_LSB_1dB0: c_int = 6;
pub const CPCAP_BIT_VOL_CDC3: c_int = 5;
pub const CPCAP_BIT_VOL_CDC2: c_int = 4;
pub const CPCAP_BIT_VOL_CDC1: c_int = 3;
pub const CPCAP_BIT_VOL_CDC0: c_int = 2;
pub const CPCAP_BIT_VOL_CDC_LSB_1dB1: c_int = 1;
pub const CPCAP_BIT_VOL_CDC_LSB_1dB0: c_int = 0;
// Register 521 CPCAP_REG_RXCOA  --- Codec to Output Amp Switches
pub const CPCAP_BIT_PGA_CDC_EN: c_int = 10;
pub const CPCAP_BIT_CDC_SW: c_int = 9;
pub const CPCAP_BIT_PGA_OUTR_USBDP_CDC_SW: c_int = 8;
pub const CPCAP_BIT_PGA_OUTL_USBDN_CDC_SW: c_int = 7;
pub const CPCAP_BIT_ALEFT_HS_CDC_SW: c_int = 6;
pub const CPCAP_BIT_ARIGHT_HS_CDC_SW: c_int = 5;
pub const CPCAP_BIT_A4_LINEOUT_L_CDC_SW: c_int = 4;
pub const CPCAP_BIT_A4_LINEOUT_R_CDC_SW: c_int = 3;
pub const CPCAP_BIT_A2_LDSP_L_CDC_SW: c_int = 2;
pub const CPCAP_BIT_A2_LDSP_R_CDC_SW: c_int = 1;
pub const CPCAP_BIT_A1_EAR_CDC_SW: c_int = 0;
// Register 522 CPCAP_REG_RXSDOA --- RX Stereo DAC to Output Amp Switches
pub const CPCAP_BIT_PGA_DAC_EN: c_int = 12;
pub const CPCAP_BIT_ST_DAC_SW: c_int = 11;
pub const CPCAP_BIT_MONO_DAC1: c_int = 10;
pub const CPCAP_BIT_MONO_DAC0: c_int = 9;
pub const CPCAP_BIT_PGA_OUTR_USBDP_DAC_SW: c_int = 8;
pub const CPCAP_BIT_PGA_OUTL_USBDN_DAC_SW: c_int = 7;
pub const CPCAP_BIT_ALEFT_HS_DAC_SW: c_int = 6;
pub const CPCAP_BIT_ARIGHT_HS_DAC_SW: c_int = 5;
pub const CPCAP_BIT_A4_LINEOUT_L_DAC_SW: c_int = 4;
pub const CPCAP_BIT_A4_LINEOUT_R_DAC_SW: c_int = 3;
pub const CPCAP_BIT_A2_LDSP_L_DAC_SW: c_int = 2;
pub const CPCAP_BIT_A2_LDSP_R_DAC_SW: c_int = 1;
pub const CPCAP_BIT_A1_EAR_DAC_SW: c_int = 0;
// Register 523 CPCAP_REG_RXEPOA --- RX External PGA to Output Amp Switches
pub const CPCAP_BIT_PGA_EXT_L_EN: c_int = 14;
pub const CPCAP_BIT_PGA_EXT_R_EN: c_int = 13;
pub const CPCAP_BIT_PGA_IN_L_SW: c_int = 12;
pub const CPCAP_BIT_PGA_IN_R_SW: c_int = 11;
pub const CPCAP_BIT_MONO_EXT1: c_int = 10;
pub const CPCAP_BIT_MONO_EXT0: c_int = 9;
pub const CPCAP_BIT_PGA_OUTR_USBDP_EXT_SW: c_int = 8;
pub const CPCAP_BIT_PGA_OUTL_USBDN_EXT_SW: c_int = 7;
pub const CPCAP_BIT_ALEFT_HS_EXT_SW: c_int = 6;
pub const CPCAP_BIT_ARIGHT_HS_EXT_SW: c_int = 5;
pub const CPCAP_BIT_A4_LINEOUT_L_EXT_SW: c_int = 4;
pub const CPCAP_BIT_A4_LINEOUT_R_EXT_SW: c_int = 3;
pub const CPCAP_BIT_A2_LDSP_L_EXT_SW: c_int = 2;
pub const CPCAP_BIT_A2_LDSP_R_EXT_SW: c_int = 1;
pub const CPCAP_BIT_A1_EAR_EXT_SW: c_int = 0;
// Register 525 CPCAP_REG_A2LA --- SPK Amplifier and Clock Config for Headset
pub const CPCAP_BIT_NCP_CLK_SYNC: c_int = 7;
pub const CPCAP_BIT_A2_CLK_SYNC: c_int = 6;
pub const CPCAP_BIT_A2_FREE_RUN: c_int = 5;
pub const CPCAP_BIT_A2_CLK2: c_int = 4;
pub const CPCAP_BIT_A2_CLK1: c_int = 3;
pub const CPCAP_BIT_A2_CLK0: c_int = 2;
pub const CPCAP_BIT_A2_CLK_IN: c_int = 1;
pub const CPCAP_BIT_A2_CONFIG: c_int = 0;
pub const SLEEP_ACTIVATE_POWER: c_int = 2;
pub const CLOCK_TREE_RESET_TIME: c_int = 1;
// constants for ST delay workaround
pub const STM_STDAC_ACTIVATE_RAMP_TIME: c_int = 1;
pub const STM_STDAC_EN_TEST_PRE: c_uint = 0x090C;
pub const STM_STDAC_EN_TEST_POST: c_uint = 0x0000;
pub const STM_STDAC_EN_ST_TEST1_PRE: c_uint = 0x2400;
pub const STM_STDAC_EN_ST_TEST1_POST: c_uint = 0x0400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_reg_info {
    pub reg: u16,
    pub mask: u16,
    pub val: u16,
}

    static const struct cpcap_reg_info cpcap_default_regs[] = {
    { CPCAP_REG_CC, 0xFFFF, 0x0000 },
    { CPCAP_REG_CC, 0xFFFF, 0x0000 },
    { CPCAP_REG_CDI, 0xBFFF, 0x0000 },
    { CPCAP_REG_SDAC, 0x0FFF, 0x0000 },
    { CPCAP_REG_SDACDI, 0x3FFF, 0x0000 },
    { CPCAP_REG_TXI, 0x0FDF, 0x0000 },
    { CPCAP_REG_TXMP, 0x0FFF, 0x0400 },
    { CPCAP_REG_RXOA, 0x01FF, 0x0000 },
    { CPCAP_REG_RXVC, 0xFF3C, 0x0000 },
    { CPCAP_REG_RXCOA, 0x07FF, 0x0000 },
    { CPCAP_REG_RXSDOA, 0x1FFF, 0x0000 },
    { CPCAP_REG_RXEPOA, 0x7FFF, 0x0000 },
    { CPCAP_REG_A2LA, BIT(CPCAP_BIT_A2_FREE_RUN),
    BIT(CPCAP_BIT_A2_FREE_RUN) },
    };
    enum cpcap_dai {
    CPCAP_DAI_HIFI,
    CPCAP_DAI_VOICE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_audio {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub vendor: u16,
    pub codec_clk_id: c_int,
    pub codec_freq: c_int,
    pub codec_format: c_int,
    pub vaudio: *mut regulator,
    pub hsirq: c_int,
    pub mb2irq: c_int,
    pub jack: snd_soc_jack,
}

    static int cpcap_st_workaround(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    let mut err: c_int = 0;
// Only CPCAP from ST requires workaround
    if (cpcap.vendor != CPCAP_VENDOR_ST)
    return 0;
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    err = regmap_write(cpcap.regmap, CPCAP_REG_TEST,
    STM_STDAC_EN_TEST_PRE);
    if (err)
    return err;
    err = regmap_write(cpcap.regmap, CPCAP_REG_ST_TEST1,
    STM_STDAC_EN_ST_TEST1_PRE);
    break;
    case SND_SOC_DAPM_POST_PMU:
    msleep(STM_STDAC_ACTIVATE_RAMP_TIME);
    err = regmap_write(cpcap.regmap, CPCAP_REG_ST_TEST1,
    STM_STDAC_EN_ST_TEST1_POST);
    if (err)
    return err;
    err = regmap_write(cpcap.regmap, CPCAP_REG_TEST,
    STM_STDAC_EN_TEST_POST);
    break;
    default:
    break;
    }
    return err;
    }
// Capture Gain Control: 0dB to 31dB in 1dB steps
    static const DECLARE_TLV_DB_SCALE(mic_gain_tlv, 0, 100, 0);
// Playback Gain Control: -33dB to 12dB in 3dB steps
    static const DECLARE_TLV_DB_SCALE(vol_tlv, -3300, 300, 0);
    static const struct snd_kcontrol_new cpcap_snd_controls[] = {
// Playback Gain
    SOC_SINGLE_TLV("HiFi Playback Volume",
    CPCAP_REG_RXVC, CPCAP_BIT_VOL_DAC0, 0xF, 0, vol_tlv),
    SOC_SINGLE_TLV("Voice Playback Volume",
    CPCAP_REG_RXVC, CPCAP_BIT_VOL_CDC0, 0xF, 0, vol_tlv),
    SOC_SINGLE_TLV("Ext Playback Volume",
    CPCAP_REG_RXVC, CPCAP_BIT_VOL_EXT0, 0xF, 0, vol_tlv),
// Capture Gain
    SOC_SINGLE_TLV("Mic1 Capture Volume",
    CPCAP_REG_TXMP, CPCAP_BIT_MIC1_GAIN_0, 0x1F, 0, mic_gain_tlv),
    SOC_SINGLE_TLV("Mic2 Capture Volume",
    CPCAP_REG_TXMP, CPCAP_BIT_MIC2_GAIN_0, 0x1F, 0, mic_gain_tlv),
// Phase Invert
    SOC_SINGLE("Hifi Left Phase Invert Switch",
    CPCAP_REG_RXSDOA, CPCAP_BIT_MONO_DAC0, 1, 0),
    SOC_SINGLE("Ext Left Phase Invert Switch",
    CPCAP_REG_RXEPOA, CPCAP_BIT_MONO_EXT0, 1, 0),
    };
    static const char * const cpcap_out_mux_texts[] = {
    "Off", "Voice", "HiFi", "Ext"
    };
    static const char * const cpcap_in_right_mux_texts[] = {
    "Off", "Mic 1", "Headset Mic", "EMU Mic", "Ext Right"
    };
    static const char * const cpcap_in_left_mux_texts[] = {
    "Off", "Mic 2", "Ext Left"
    };
//
// input muxes use unusual register layout, so that we need to use custom
// getter/setter methods
//
    static SOC_ENUM_SINGLE_EXT_DECL(cpcap_input_left_mux_enum,
    cpcap_in_left_mux_texts);
    static SOC_ENUM_SINGLE_EXT_DECL(cpcap_input_right_mux_enum,
    cpcap_in_right_mux_texts);
//
// mux uses same bit in CPCAP_REG_RXCOA, CPCAP_REG_RXSDOA & CPCAP_REG_RXEPOA;
// even though the register layout makes it look like a mixer, this is a mux.
// Enabling multiple inputs will result in no audio being forwarded.
//
    static SOC_ENUM_SINGLE_DECL(cpcap_earpiece_mux_enum, 0, 0, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_spkr_r_mux_enum, 0, 1, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_spkr_l_mux_enum, 0, 2, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_line_r_mux_enum, 0, 3, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_line_l_mux_enum, 0, 4, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_hs_r_mux_enum, 0, 5, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_hs_l_mux_enum, 0, 6, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_emu_l_mux_enum, 0, 7, cpcap_out_mux_texts);
    static SOC_ENUM_SINGLE_DECL(cpcap_emu_r_mux_enum, 0, 8, cpcap_out_mux_texts);
    static int cpcap_output_mux_get_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    let mut shift: c_uint = e.shift_l;
    int reg_voice, reg_hifi, reg_ext, status;
    int err;
    err = regmap_read(cpcap.regmap, CPCAP_REG_RXCOA, &reg_voice);
    if (err)
    return err;
    err = regmap_read(cpcap.regmap, CPCAP_REG_RXSDOA, &reg_hifi);
    if (err)
    return err;
    err = regmap_read(cpcap.regmap, CPCAP_REG_RXEPOA, &reg_ext);
    if (err)
    return err;
    reg_voice = (reg_voice >> shift) & 1;
    reg_hifi = (reg_hifi >> shift) & 1;
    reg_ext = (reg_ext >> shift) & 1;
    status = reg_ext << 2 | reg_hifi << 1 | reg_voice;
    switch (status) {
    case 0x04:
    ucontrol.value.enumerated.item[0] = 3;
    break;
    case 0x02:
    ucontrol.value.enumerated.item[0] = 2;
    break;
    case 0x01:
    ucontrol.value.enumerated.item[0] = 1;
    break;
    default:
    ucontrol.value.enumerated.item[0] = 0;
    break;
    }
    return 0;
    }
    static int cpcap_output_mux_put_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    let mut muxval: c_uint = ucontrol.value.enumerated.item[0];
    let mut mask: c_uint = BIT(e.shift_l);
    let mut reg_voice: u16 = 0x00, reg_hifi = 0x00, reg_ext = 0x00;
    int err;
    switch (muxval) {
    case 1:
    reg_voice = mask;
    break;
    case 2:
    reg_hifi = mask;
    break;
    case 3:
    reg_ext = mask;
    break;
    default:
    break;
    }
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_RXCOA,
    mask, reg_voice);
    if (err)
    return err;
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_RXSDOA,
    mask, reg_hifi);
    if (err)
    return err;
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_RXEPOA,
    mask, reg_ext);
    if (err)
    return err;
    snd_soc_dapm_mux_update_power(dapm, kcontrol, muxval, e, core::ptr::null_mut());
    return 0;
    }
    static int cpcap_input_right_mux_get_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    int regval, mask;
    int err;
    err = regmap_read(cpcap.regmap, CPCAP_REG_TXI, &regval);
    if (err)
    return err;
    mask = 0;
    mask |= BIT(CPCAP_BIT_MIC1_MUX);
    mask |= BIT(CPCAP_BIT_HS_MIC_MUX);
    mask |= BIT(CPCAP_BIT_EMU_MIC_MUX);
    mask |= BIT(CPCAP_BIT_RX_R_ENCODE);
    switch (regval & mask) {
    case BIT(CPCAP_BIT_RX_R_ENCODE):
    ucontrol.value.enumerated.item[0] = 4;
    break;
    case BIT(CPCAP_BIT_EMU_MIC_MUX):
    ucontrol.value.enumerated.item[0] = 3;
    break;
    case BIT(CPCAP_BIT_HS_MIC_MUX):
    ucontrol.value.enumerated.item[0] = 2;
    break;
    case BIT(CPCAP_BIT_MIC1_MUX):
    ucontrol.value.enumerated.item[0] = 1;
    break;
    default:
    ucontrol.value.enumerated.item[0] = 0;
    break;
    }
    return 0;
    }
    static int cpcap_input_right_mux_put_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    let mut muxval: c_uint = ucontrol.value.enumerated.item[0];
    let mut regval: c_int = 0, mask;
    int err;
    mask = 0;
    mask |= BIT(CPCAP_BIT_MIC1_MUX);
    mask |= BIT(CPCAP_BIT_HS_MIC_MUX);
    mask |= BIT(CPCAP_BIT_EMU_MIC_MUX);
    mask |= BIT(CPCAP_BIT_RX_R_ENCODE);
    switch (muxval) {
    case 1:
    regval = BIT(CPCAP_BIT_MIC1_MUX);
    break;
    case 2:
    regval = BIT(CPCAP_BIT_HS_MIC_MUX);
    break;
    case 3:
    regval = BIT(CPCAP_BIT_EMU_MIC_MUX);
    break;
    case 4:
    regval = BIT(CPCAP_BIT_RX_R_ENCODE);
    break;
    default:
    break;
    }
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_TXI,
    mask, regval);
    if (err)
    return err;
    snd_soc_dapm_mux_update_power(dapm, kcontrol, muxval, e, core::ptr::null_mut());
    return 0;
    }
    static int cpcap_input_left_mux_get_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    int regval, mask;
    int err;
    err = regmap_read(cpcap.regmap, CPCAP_REG_TXI, &regval);
    if (err)
    return err;
    mask = 0;
    mask |= BIT(CPCAP_BIT_MIC2_MUX);
    mask |= BIT(CPCAP_BIT_RX_L_ENCODE);
    switch (regval & mask) {
    case BIT(CPCAP_BIT_RX_L_ENCODE):
    ucontrol.value.enumerated.item[0] = 2;
    break;
    case BIT(CPCAP_BIT_MIC2_MUX):
    ucontrol.value.enumerated.item[0] = 1;
    break;
    default:
    ucontrol.value.enumerated.item[0] = 0;
    break;
    }
    return 0;
    }
    static int cpcap_input_left_mux_put_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    let mut muxval: c_uint = ucontrol.value.enumerated.item[0];
    let mut regval: c_int = 0, mask;
    int err;
    mask = 0;
    mask |= BIT(CPCAP_BIT_MIC2_MUX);
    mask |= BIT(CPCAP_BIT_RX_L_ENCODE);
    switch (muxval) {
    case 1:
    regval = BIT(CPCAP_BIT_MIC2_MUX);
    break;
    case 2:
    regval = BIT(CPCAP_BIT_RX_L_ENCODE);
    break;
    default:
    break;
    }
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_TXI,
    mask, regval);
    if (err)
    return err;
    snd_soc_dapm_mux_update_power(dapm, kcontrol, muxval, e, core::ptr::null_mut());
    return 0;
    }
    static const struct snd_kcontrol_new cpcap_input_left_mux =
    SOC_DAPM_ENUM_EXT("Input Left", cpcap_input_left_mux_enum,
    cpcap_input_left_mux_get_enum,
    cpcap_input_left_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_input_right_mux =
    SOC_DAPM_ENUM_EXT("Input Right", cpcap_input_right_mux_enum,
    cpcap_input_right_mux_get_enum,
    cpcap_input_right_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_emu_left_mux =
    SOC_DAPM_ENUM_EXT("EMU Left", cpcap_emu_l_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_emu_right_mux =
    SOC_DAPM_ENUM_EXT("EMU Right", cpcap_emu_r_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_hs_left_mux =
    SOC_DAPM_ENUM_EXT("Headset Left", cpcap_hs_l_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_hs_right_mux =
    SOC_DAPM_ENUM_EXT("Headset Right", cpcap_hs_r_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_line_left_mux =
    SOC_DAPM_ENUM_EXT("Line Left", cpcap_line_l_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_line_right_mux =
    SOC_DAPM_ENUM_EXT("Line Right", cpcap_line_r_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_speaker_left_mux =
    SOC_DAPM_ENUM_EXT("Speaker Left", cpcap_spkr_l_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_speaker_right_mux =
    SOC_DAPM_ENUM_EXT("Speaker Right", cpcap_spkr_r_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_earpiece_mux =
    SOC_DAPM_ENUM_EXT("Earpiece", cpcap_earpiece_mux_enum,
    cpcap_output_mux_get_enum, cpcap_output_mux_put_enum);
    static const struct snd_kcontrol_new cpcap_hifi_mono_mixer_controls[] = {
    SOC_DAPM_SINGLE("HiFi Mono Playback Switch",
    CPCAP_REG_RXSDOA, CPCAP_BIT_MONO_DAC1, 1, 0),
    };
    static const struct snd_kcontrol_new cpcap_ext_mono_mixer_controls[] = {
    SOC_DAPM_SINGLE("Ext Mono Playback Switch",
    CPCAP_REG_RXEPOA, CPCAP_BIT_MONO_EXT0, 1, 0),
    };
    static const struct snd_kcontrol_new cpcap_extr_mute_control =
    SOC_DAPM_SINGLE("Switch",
    CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_IN_R_SW, 1, 0);
    static const struct snd_kcontrol_new cpcap_extl_mute_control =
    SOC_DAPM_SINGLE("Switch",
    CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_IN_L_SW, 1, 0);
    static const struct snd_kcontrol_new cpcap_voice_loopback =
    SOC_DAPM_SINGLE("Switch",
    CPCAP_REG_TXI, CPCAP_BIT_DLM, 1, 0);
    static const struct snd_soc_dapm_widget cpcap_dapm_widgets[] = {
// DAIs
    SND_SOC_DAPM_AIF_IN("HiFi RX", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("Voice RX", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("Voice TX", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
// Power Supply
    SND_SOC_DAPM_REGULATOR_SUPPLY("VAUDIO", SLEEP_ACTIVATE_POWER, 0),
// Highpass Filters
    SND_SOC_DAPM_REG(snd_soc_dapm_pga, "Highpass Filter RX",
    CPCAP_REG_CC, CPCAP_BIT_AUDIHPF_0, 0x3, 0x3, 0x0),
    SND_SOC_DAPM_REG(snd_soc_dapm_pga, "Highpass Filter TX",
    CPCAP_REG_CC, CPCAP_BIT_AUDOHPF_0, 0x3, 0x3, 0x0),
// Clocks
    SND_SOC_DAPM_SUPPLY("HiFi DAI Clock",
    CPCAP_REG_SDACDI, CPCAP_BIT_ST_CLK_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("Voice DAI Clock",
    CPCAP_REG_CDI, CPCAP_BIT_CDC_CLK_EN, 0, core::ptr::null_mut(), 0),
// Microphone Bias
    SND_SOC_DAPM_SUPPLY("MIC1R Bias",
    CPCAP_REG_TXI, CPCAP_BIT_MB_ON1R, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("MIC1L Bias",
    CPCAP_REG_TXI, CPCAP_BIT_MB_ON1L, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("MIC2 Bias",
    CPCAP_REG_TXI, CPCAP_BIT_MB_ON2, 0, core::ptr::null_mut(), 0),
// Inputs
    SND_SOC_DAPM_INPUT("MICR"),
    SND_SOC_DAPM_INPUT("HSMIC"),
    SND_SOC_DAPM_INPUT("EMUMIC"),
    SND_SOC_DAPM_INPUT("MICL"),
    SND_SOC_DAPM_INPUT("EXTR"),
    SND_SOC_DAPM_INPUT("EXTL"),
// Capture Route
    SND_SOC_DAPM_MUX("Right Capture Route",
    SND_SOC_NOPM, 0, 0, &cpcap_input_right_mux),
    SND_SOC_DAPM_MUX("Left Capture Route",
    SND_SOC_NOPM, 0, 0, &cpcap_input_left_mux),
// Capture PGAs
    SND_SOC_DAPM_PGA("Microphone 1 PGA",
    CPCAP_REG_TXI, CPCAP_BIT_MIC1_PGA_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Microphone 2 PGA",
    CPCAP_REG_TXI, CPCAP_BIT_MIC2_PGA_EN, 0, core::ptr::null_mut(), 0),
// ADC
    SND_SOC_DAPM_ADC("ADC Right", core::ptr::null_mut(),
    CPCAP_REG_CC, CPCAP_BIT_MIC1_CDC_EN, 0),
    SND_SOC_DAPM_ADC("ADC Left", core::ptr::null_mut(),
    CPCAP_REG_CC, CPCAP_BIT_MIC2_CDC_EN, 0),
// DAC
    SND_SOC_DAPM_DAC_E("DAC HiFi", core::ptr::null_mut(),
    CPCAP_REG_SDAC, CPCAP_BIT_ST_DAC_EN, 0,
    cpcap_st_workaround,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_DAC_E("DAC Voice", core::ptr::null_mut(),
    CPCAP_REG_CC, CPCAP_BIT_CDC_EN_RX, 0,
    cpcap_st_workaround,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// Playback PGA
    SND_SOC_DAPM_PGA("HiFi PGA",
    CPCAP_REG_RXSDOA, CPCAP_BIT_PGA_DAC_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Voice PGA",
    CPCAP_REG_RXCOA, CPCAP_BIT_PGA_CDC_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA_E("Ext Right PGA",
    CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_EXT_R_EN, 0,
    core::ptr::null_mut(), 0,
    cpcap_st_workaround,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_E("Ext Left PGA",
    CPCAP_REG_RXEPOA, CPCAP_BIT_PGA_EXT_L_EN, 0,
    core::ptr::null_mut(), 0,
    cpcap_st_workaround,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// Playback Switch
    SND_SOC_DAPM_SWITCH("Ext Right Enable", SND_SOC_NOPM, 0, 0,
    &cpcap_extr_mute_control),
    SND_SOC_DAPM_SWITCH("Ext Left Enable", SND_SOC_NOPM, 0, 0,
    &cpcap_extl_mute_control),
// Loopback Switch
    SND_SOC_DAPM_SWITCH("Voice Loopback", SND_SOC_NOPM, 0, 0,
    &cpcap_voice_loopback),
// Mono Mixer
    SOC_MIXER_ARRAY("HiFi Mono Left Mixer", SND_SOC_NOPM, 0, 0,
    cpcap_hifi_mono_mixer_controls),
    SOC_MIXER_ARRAY("HiFi Mono Right Mixer", SND_SOC_NOPM, 0, 0,
    cpcap_hifi_mono_mixer_controls),
    SOC_MIXER_ARRAY("Ext Mono Left Mixer", SND_SOC_NOPM, 0, 0,
    cpcap_ext_mono_mixer_controls),
    SOC_MIXER_ARRAY("Ext Mono Right Mixer", SND_SOC_NOPM, 0, 0,
    cpcap_ext_mono_mixer_controls),
// Output Routes
    SND_SOC_DAPM_MUX("Earpiece Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_earpiece_mux),
    SND_SOC_DAPM_MUX("Speaker Right Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_speaker_right_mux),
    SND_SOC_DAPM_MUX("Speaker Left Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_speaker_left_mux),
    SND_SOC_DAPM_MUX("Lineout Right Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_line_right_mux),
    SND_SOC_DAPM_MUX("Lineout Left Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_line_left_mux),
    SND_SOC_DAPM_MUX("Headset Right Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_hs_right_mux),
    SND_SOC_DAPM_MUX("Headset Left Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_hs_left_mux),
    SND_SOC_DAPM_MUX("EMU Right Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_emu_right_mux),
    SND_SOC_DAPM_MUX("EMU Left Playback Route", SND_SOC_NOPM, 0, 0,
    &cpcap_emu_left_mux),
// Output Amplifier
    SND_SOC_DAPM_PGA("Earpiece PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_A1_EAR_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Speaker Right PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_A2_LDSP_R_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Speaker Left PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_A2_LDSP_L_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Lineout Right PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_A4_LINEOUT_R_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Lineout Left PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_A4_LINEOUT_L_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Headset Right PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_HS_R_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("Headset Left PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_HS_L_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("EMU Right PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_EMU_SPKR_R_EN, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA("EMU Left PGA",
    CPCAP_REG_RXOA, CPCAP_BIT_EMU_SPKR_L_EN, 0, core::ptr::null_mut(), 0),
// Headet Charge Pump
    SND_SOC_DAPM_SUPPLY("Headset Charge Pump",
    CPCAP_REG_RXOA, CPCAP_BIT_ST_HS_CP_EN, 0, core::ptr::null_mut(), 0),
// Outputs
    SND_SOC_DAPM_OUTPUT("EP"),
    SND_SOC_DAPM_OUTPUT("SPKR"),
    SND_SOC_DAPM_OUTPUT("SPKL"),
    SND_SOC_DAPM_OUTPUT("LINER"),
    SND_SOC_DAPM_OUTPUT("LINEL"),
    SND_SOC_DAPM_OUTPUT("HSR"),
    SND_SOC_DAPM_OUTPUT("HSL"),
    SND_SOC_DAPM_OUTPUT("EMUR"),
    SND_SOC_DAPM_OUTPUT("EMUL"),
    };
    static const struct snd_soc_dapm_route intercon[] = {
// Power Supply
    {"HiFi PGA", core::ptr::null_mut(), "VAUDIO"},
    {"Voice PGA", core::ptr::null_mut(), "VAUDIO"},
    {"Ext Right PGA", core::ptr::null_mut(), "VAUDIO"},
    {"Ext Left PGA", core::ptr::null_mut(), "VAUDIO"},
    {"Microphone 1 PGA", core::ptr::null_mut(), "VAUDIO"},
    {"Microphone 2 PGA", core::ptr::null_mut(), "VAUDIO"},
// Stream -> AIF
    {"HiFi RX", core::ptr::null_mut(), "HiFi Playback"},
    {"Voice RX", core::ptr::null_mut(), "Voice Playback"},
    {"Voice Capture", core::ptr::null_mut(), "Voice TX"},
// AIF clocks
    {"HiFi RX", core::ptr::null_mut(), "HiFi DAI Clock"},
    {"Voice RX", core::ptr::null_mut(), "Voice DAI Clock"},
    {"Voice TX", core::ptr::null_mut(), "Voice DAI Clock"},
// Digital Loopback
    {"Voice Loopback", "Switch", "Voice TX"},
    {"Voice RX", core::ptr::null_mut(), "Voice Loopback"},
// Highpass Filters
    {"Highpass Filter RX", core::ptr::null_mut(), "Voice RX"},
    {"Voice TX", core::ptr::null_mut(), "Highpass Filter TX"},
// AIF -> DAC mapping
    {"DAC HiFi", core::ptr::null_mut(), "HiFi RX"},
    {"DAC Voice", core::ptr::null_mut(), "Highpass Filter RX"},
// DAC -> PGA
    {"HiFi PGA", core::ptr::null_mut(), "DAC HiFi"},
    {"Voice PGA", core::ptr::null_mut(), "DAC Voice"},
// Ext Input -> PGA
    {"Ext Right PGA", core::ptr::null_mut(), "EXTR"},
    {"Ext Left PGA", core::ptr::null_mut(), "EXTL"},
// Ext PGA -> Ext Playback Switch
    {"Ext Right Enable", "Switch", "Ext Right PGA"},
    {"Ext Left Enable", "Switch", "Ext Left PGA"},
// HiFi PGA -> Mono Mixer
    {"HiFi Mono Left Mixer", core::ptr::null_mut(), "HiFi PGA"},
    {"HiFi Mono Left Mixer", "HiFi Mono Playback Switch", "HiFi PGA"},
    {"HiFi Mono Right Mixer", core::ptr::null_mut(), "HiFi PGA"},
    {"HiFi Mono Right Mixer", "HiFi Mono Playback Switch", "HiFi PGA"},
// Ext Playback Switch -> Ext Mono Mixer
    {"Ext Mono Right Mixer", core::ptr::null_mut(), "Ext Right Enable"},
    {"Ext Mono Right Mixer", "Ext Mono Playback Switch", "Ext Left Enable"},
    {"Ext Mono Left Mixer", core::ptr::null_mut(), "Ext Left Enable"},
    {"Ext Mono Left Mixer", "Ext Mono Playback Switch", "Ext Right Enable"},
// HiFi Mono Mixer -> Output Route
    {"Earpiece Playback Route", "HiFi", "HiFi Mono Right Mixer"},
    {"Speaker Right Playback Route", "HiFi", "HiFi Mono Right Mixer"},
    {"Speaker Left Playback Route", "HiFi", "HiFi Mono Left Mixer"},
    {"Lineout Right Playback Route", "HiFi", "HiFi Mono Right Mixer"},
    {"Lineout Left Playback Route", "HiFi", "HiFi Mono Left Mixer"},
    {"Headset Right Playback Route", "HiFi", "HiFi Mono Right Mixer"},
    {"Headset Left Playback Route", "HiFi", "HiFi Mono Left Mixer"},
    {"EMU Right Playback Route", "HiFi", "HiFi Mono Right Mixer"},
    {"EMU Left Playback Route", "HiFi", "HiFi Mono Left Mixer"},
// Voice PGA -> Output Route
    {"Earpiece Playback Route", "Voice", "Voice PGA"},
    {"Speaker Right Playback Route", "Voice", "Voice PGA"},
    {"Speaker Left Playback Route", "Voice", "Voice PGA"},
    {"Lineout Right Playback Route", "Voice", "Voice PGA"},
    {"Lineout Left Playback Route", "Voice", "Voice PGA"},
    {"Headset Right Playback Route", "Voice", "Voice PGA"},
    {"Headset Left Playback Route", "Voice", "Voice PGA"},
    {"EMU Right Playback Route", "Voice", "Voice PGA"},
    {"EMU Left Playback Route", "Voice", "Voice PGA"},
// Ext Mono Mixer -> Output Route
    {"Earpiece Playback Route", "Ext", "Ext Mono Right Mixer"},
    {"Speaker Right Playback Route", "Ext", "Ext Mono Right Mixer"},
    {"Speaker Left Playback Route", "Ext", "Ext Mono Left Mixer"},
    {"Lineout Right Playback Route", "Ext", "Ext Mono Right Mixer"},
    {"Lineout Left Playback Route", "Ext", "Ext Mono Left Mixer"},
    {"Headset Right Playback Route", "Ext", "Ext Mono Right Mixer"},
    {"Headset Left Playback Route", "Ext", "Ext Mono Left Mixer"},
    {"EMU Right Playback Route", "Ext", "Ext Mono Right Mixer"},
    {"EMU Left Playback Route", "Ext", "Ext Mono Left Mixer"},
// Output Route -> Output Amplifier
    {"Earpiece PGA", core::ptr::null_mut(), "Earpiece Playback Route"},
    {"Speaker Right PGA", core::ptr::null_mut(), "Speaker Right Playback Route"},
    {"Speaker Left PGA", core::ptr::null_mut(), "Speaker Left Playback Route"},
    {"Lineout Right PGA", core::ptr::null_mut(), "Lineout Right Playback Route"},
    {"Lineout Left PGA", core::ptr::null_mut(), "Lineout Left Playback Route"},
    {"Headset Right PGA", core::ptr::null_mut(), "Headset Right Playback Route"},
    {"Headset Left PGA", core::ptr::null_mut(), "Headset Left Playback Route"},
    {"EMU Right PGA", core::ptr::null_mut(), "EMU Right Playback Route"},
    {"EMU Left PGA", core::ptr::null_mut(), "EMU Left Playback Route"},
// Output Amplifier -> Output
    {"EP", core::ptr::null_mut(), "Earpiece PGA"},
    {"SPKR", core::ptr::null_mut(), "Speaker Right PGA"},
    {"SPKL", core::ptr::null_mut(), "Speaker Left PGA"},
    {"LINER", core::ptr::null_mut(), "Lineout Right PGA"},
    {"LINEL", core::ptr::null_mut(), "Lineout Left PGA"},
    {"HSR", core::ptr::null_mut(), "Headset Right PGA"},
    {"HSL", core::ptr::null_mut(), "Headset Left PGA"},
    {"EMUR", core::ptr::null_mut(), "EMU Right PGA"},
    {"EMUL", core::ptr::null_mut(), "EMU Left PGA"},
// Headset Charge Pump -> Headset
    {"HSR", core::ptr::null_mut(), "Headset Charge Pump"},
    {"HSL", core::ptr::null_mut(), "Headset Charge Pump"},
// Mic -> Mic Route
    {"Right Capture Route", "Mic 1", "MICR"},
    {"Right Capture Route", "Headset Mic", "HSMIC"},
    {"Right Capture Route", "EMU Mic", "EMUMIC"},
    {"Right Capture Route", "Ext Right", "EXTR"},
    {"Left Capture Route", "Mic 2", "MICL"},
    {"Left Capture Route", "Ext Left", "EXTL"},
// Input Route -> Microphone PGA
    {"Microphone 1 PGA", core::ptr::null_mut(), "Right Capture Route"},
    {"Microphone 2 PGA", core::ptr::null_mut(), "Left Capture Route"},
// Microphone PGA -> ADC
    {"ADC Right", core::ptr::null_mut(), "Microphone 1 PGA"},
    {"ADC Left", core::ptr::null_mut(), "Microphone 2 PGA"},
// ADC -> Stream
    {"Highpass Filter TX", core::ptr::null_mut(), "ADC Right"},
    {"Highpass Filter TX", core::ptr::null_mut(), "ADC Left"},
// Mic Bias
    {"MICL", core::ptr::null_mut(), "MIC1L Bias"},
    {"MICR", core::ptr::null_mut(), "MIC1R Bias"},
    };
    static int cpcap_set_sysclk(struct cpcap_audio *cpcap, enum cpcap_dai dai,
    int clk_id, int freq)
    {
    u16 clkfreqreg, clkfreqshift;
    u16 clkfreqmask, clkfreqval;
    u16 clkidreg, clkidshift;
    u16 mask, val;
    int err;
    switch (dai) {
    case CPCAP_DAI_HIFI:
    clkfreqreg = CPCAP_REG_SDAC;
    clkfreqshift = CPCAP_BIT_ST_DAC_CLK0;
    clkidreg = CPCAP_REG_SDACDI;
    clkidshift = CPCAP_BIT_ST_DAC_CLK_IN_SEL;
    break;
    case CPCAP_DAI_VOICE:
    clkfreqreg = CPCAP_REG_CC;
    clkfreqshift = CPCAP_BIT_CDC_CLK0;
    clkidreg = CPCAP_REG_CDI;
    clkidshift = CPCAP_BIT_CLK_IN_SEL;
    break;
    default:
    dev_err(cpcap.component.dev, "invalid DAI: %d", dai);
    return -EINVAL;
    }
// setup clk id
    if (clk_id < 0 || clk_id > 1) {
    dev_err(cpcap.component.dev, "invalid clk id %d", clk_id);
    return -EINVAL;
    }
    err = regmap_update_bits(cpcap.regmap, clkidreg, BIT(clkidshift),
    clk_id ? BIT(clkidshift) : 0);
    if (err)
    return err;
// enable PLL for Voice DAI
    if (dai == CPCAP_DAI_VOICE) {
    mask = BIT(CPCAP_BIT_CDC_PLL_SEL);
    val = BIT(CPCAP_BIT_CDC_PLL_SEL);
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_CDI,
    mask, val);
    if (err)
    return err;
    }
// setup frequency
    clkfreqmask = 0x7 << clkfreqshift;
    switch (freq) {
    case 15360000:
    clkfreqval = 0x01 << clkfreqshift;
    break;
    case 16800000:
    clkfreqval = 0x02 << clkfreqshift;
    break;
    case 19200000:
    clkfreqval = 0x03 << clkfreqshift;
    break;
    case 26000000:
    clkfreqval = 0x04 << clkfreqshift;
    break;
    case 33600000:
    clkfreqval = 0x05 << clkfreqshift;
    break;
    case 38400000:
    clkfreqval = 0x06 << clkfreqshift;
    break;
    default:
    dev_err(cpcap.component.dev, "unsupported freq %u", freq);
    return -EINVAL;
    }
    err = regmap_update_bits(cpcap.regmap, clkfreqreg,
    clkfreqmask, clkfreqval);
    if (err)
    return err;
    if (dai == CPCAP_DAI_VOICE) {
    cpcap.codec_clk_id = clk_id;
    cpcap.codec_freq = freq;
    }
    return 0;
    }
    static int cpcap_set_samprate(struct cpcap_audio *cpcap, enum cpcap_dai dai,
    int samplerate)
    {
    struct snd_soc_component *component = cpcap.component;
    u16 sampreg, sampmask, sampshift, sampval, sampreset;
    int err, sampreadval;
    switch (dai) {
    case CPCAP_DAI_HIFI:
    sampreg = CPCAP_REG_SDAC;
    sampshift = CPCAP_BIT_ST_SR0;
    sampreset = BIT(CPCAP_BIT_DF_RESET_ST_DAC) |
    BIT(CPCAP_BIT_ST_CLOCK_TREE_RESET);
    break;
    case CPCAP_DAI_VOICE:
    sampreg = CPCAP_REG_CC;
    sampshift = CPCAP_BIT_CDC_SR0;
    sampreset = BIT(CPCAP_BIT_DF_RESET) |
    BIT(CPCAP_BIT_CDC_CLOCK_TREE_RESET);
    break;
    default:
    dev_err(component.dev, "invalid DAI: %d", dai);
    return -EINVAL;
    }
    sampmask = 0xF << sampshift | sampreset;
    switch (samplerate) {
    case 48000:
    sampval = 0x8 << sampshift;
    break;
    case 44100:
    sampval = 0x7 << sampshift;
    break;
    case 32000:
    sampval = 0x6 << sampshift;
    break;
    case 24000:
    sampval = 0x5 << sampshift;
    break;
    case 22050:
    sampval = 0x4 << sampshift;
    break;
    case 16000:
    sampval = 0x3 << sampshift;
    break;
    case 12000:
    sampval = 0x2 << sampshift;
    break;
    case 11025:
    sampval = 0x1 << sampshift;
    break;
    case 8000:
    sampval = 0x0 << sampshift;
    break;
    default:
    dev_err(component.dev, "unsupported samplerate %d", samplerate);
    return -EINVAL;
    }
    err = regmap_update_bits(cpcap.regmap, sampreg,
    sampmask, sampval | sampreset);
    if (err)
    return err;
// Wait for clock tree reset to complete
    mdelay(CLOCK_TREE_RESET_TIME);
    err = regmap_read(cpcap.regmap, sampreg, &sampreadval);
    if (err)
    return err;
    if (sampreadval & sampreset) {
    dev_err(component.dev, "reset self-clear failed: %04x",
    sampreadval);
    return -EIO;
    }
    return 0;
    }
    static int cpcap_hifi_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    let mut rate: c_int = params_rate(params);
    dev_dbg(component.dev, "HiFi setup HW params: rate=%d", rate);
    return cpcap_set_samprate(cpcap, CPCAP_DAI_HIFI, rate);
    }
    static int cpcap_hifi_set_dai_sysclk(struct snd_soc_dai *codec_dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct device *dev = component.dev;
    dev_dbg(dev, "HiFi setup sysclk: clk_id=%u, freq=%u", clk_id, freq);
    return cpcap_set_sysclk(cpcap, CPCAP_DAI_HIFI, clk_id, freq);
    }
    static int cpcap_hifi_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int fmt)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct device *dev = component.dev;
    let mut reg: static u16 = CPCAP_REG_SDACDI;
    static const u16 mask =
    BIT(CPCAP_BIT_SMB_ST_DAC) |
    BIT(CPCAP_BIT_ST_CLK_INV) |
    BIT(CPCAP_BIT_ST_FS_INV) |
    BIT(CPCAP_BIT_ST_DIG_AUD_FS0) |
    BIT(CPCAP_BIT_ST_DIG_AUD_FS1) |
    BIT(CPCAP_BIT_ST_L_TIMESLOT0) |
    BIT(CPCAP_BIT_ST_L_TIMESLOT1) |
    BIT(CPCAP_BIT_ST_L_TIMESLOT2) |
    BIT(CPCAP_BIT_ST_R_TIMESLOT0) |
    BIT(CPCAP_BIT_ST_R_TIMESLOT1) |
    BIT(CPCAP_BIT_ST_R_TIMESLOT2);
    let mut val: u16 = 0x0000;
    dev_dbg(dev, "HiFi setup dai format (%08x)", fmt);
//
// "HiFi Playback" should always be configured as
// SND_SOC_DAIFMT_CBP_CFP - codec clk & frm provider
// SND_SOC_DAIFMT_I2S - I2S mode
//
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    val &= ~BIT(CPCAP_BIT_SMB_ST_DAC);
    break;
    default:
    dev_err(dev, "HiFi dai fmt failed: CPCAP should be provider");
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_IB_IF:
    val |= BIT(CPCAP_BIT_ST_FS_INV);
    val |= BIT(CPCAP_BIT_ST_CLK_INV);
    break;
    case SND_SOC_DAIFMT_IB_NF:
    val &= ~BIT(CPCAP_BIT_ST_FS_INV);
    val |= BIT(CPCAP_BIT_ST_CLK_INV);
    break;
    case SND_SOC_DAIFMT_NB_IF:
    val |= BIT(CPCAP_BIT_ST_FS_INV);
    val &= ~BIT(CPCAP_BIT_ST_CLK_INV);
    break;
    case SND_SOC_DAIFMT_NB_NF:
    val &= ~BIT(CPCAP_BIT_ST_FS_INV);
    val &= ~BIT(CPCAP_BIT_ST_CLK_INV);
    break;
    default:
    dev_err(dev, "HiFi dai fmt failed: unsupported clock invert mode");
    return -EINVAL;
    }
    if (val & BIT(CPCAP_BIT_ST_CLK_INV))
    val &= ~BIT(CPCAP_BIT_ST_CLK_INV);
    else
    val |= BIT(CPCAP_BIT_ST_CLK_INV);
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    val |= BIT(CPCAP_BIT_ST_DIG_AUD_FS0);
    val |= BIT(CPCAP_BIT_ST_DIG_AUD_FS1);
    break;
    default:
// 01 - 4 slots network mode
    val |= BIT(CPCAP_BIT_ST_DIG_AUD_FS0);
    val &= ~BIT(CPCAP_BIT_ST_DIG_AUD_FS1);
// L on slot 1
    val |= BIT(CPCAP_BIT_ST_L_TIMESLOT0);
    break;
    }
    dev_dbg(dev, "HiFi dai format: val=%04x", val);
    return regmap_update_bits(cpcap.regmap, reg, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_hifi_set_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    static int cpcap_hifi_set_mute(struct snd_soc_dai *dai, int mute, int direction)
    {
    struct snd_soc_component *component = dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    let mut reg: static u16 = CPCAP_REG_RXSDOA;
    let mut mask: static u16 = BIT(CPCAP_BIT_ST_DAC_SW);
    u16 val;
    if (mute)
    val = 0;
    else
    val = BIT(CPCAP_BIT_ST_DAC_SW);
    dev_dbg(component.dev, "HiFi mute: %d", mute);
    return regmap_update_bits(cpcap.regmap, reg, mask, val);
    }
    static const struct snd_soc_dai_ops cpcap_dai_hifi_ops = {
    .hw_params	= cpcap_hifi_hw_params,
    .set_sysclk	= cpcap_hifi_set_dai_sysclk,
    .set_fmt	= cpcap_hifi_set_dai_fmt,
    .mute_stream	= cpcap_hifi_set_mute,
    .no_capture_mute = 1,
    };
    static int cpcap_voice_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_component *component = dai.component;
    struct device *dev = component.dev;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    let mut reg_cdi: static u16 = CPCAP_REG_CDI;
    let mut rate: c_int = params_rate(params);
    let mut channels: c_int = params_channels(params);
    let mut direction: c_int = substream.stream;
    u16 val, mask;
    int err;
    dev_dbg(dev, "Voice setup HW params: rate=%d, direction=%d, chan=%d",
    rate, direction, channels);
    err = cpcap_set_samprate(cpcap, CPCAP_DAI_VOICE, rate);
    if (err)
    return err;
    if (direction == SNDRV_PCM_STREAM_CAPTURE) {
    mask = 0x0000;
    mask |= BIT(CPCAP_BIT_MIC1_RX_TIMESLOT0);
    mask |= BIT(CPCAP_BIT_MIC1_RX_TIMESLOT1);
    mask |= BIT(CPCAP_BIT_MIC1_RX_TIMESLOT2);
    mask |= BIT(CPCAP_BIT_MIC2_TIMESLOT0);
    mask |= BIT(CPCAP_BIT_MIC2_TIMESLOT1);
    mask |= BIT(CPCAP_BIT_MIC2_TIMESLOT2);
    val = 0x0000;
    if (channels >= 2)
    val = BIT(CPCAP_BIT_MIC1_RX_TIMESLOT0);
    err = regmap_update_bits(cpcap.regmap, reg_cdi, mask, val);
    if (err)
    return err;
    }
    return snd_soc_runtime_set_dai_fmt(rtd, rtd.dai_link.dai_fmt);
    }
    static int cpcap_voice_set_dai_sysclk(struct snd_soc_dai *codec_dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    dev_dbg(component.dev, "Voice setup sysclk: clk_id=%u, freq=%u",
    clk_id, freq);
    return cpcap_set_sysclk(cpcap, CPCAP_DAI_VOICE, clk_id, freq);
    }
    static int cpcap_voice_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int fmt)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    static const u16 mask = BIT(CPCAP_BIT_SMB_CDC) |
    BIT(CPCAP_BIT_CLK_INV) |
    BIT(CPCAP_BIT_FS_INV) |
    BIT(CPCAP_BIT_CDC_DIG_AUD_FS0) |
    BIT(CPCAP_BIT_CDC_DIG_AUD_FS1);
    let mut val: u16 = 0x0000;
    int err;
    dev_dbg(component.dev, "Voice setup dai format (%08x)", fmt);
//
// "Voice Playback" and "Voice Capture" should always be
// configured as SND_SOC_DAIFMT_CBP_CFP - codec clk & frm
// provider
//
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBP_CFP:
    val &= ~BIT(CPCAP_BIT_SMB_CDC);
    break;
    default:
    dev_err(component.dev, "Voice dai fmt failed: CPCAP should be the provider");
    val &= ~BIT(CPCAP_BIT_SMB_CDC);
    break;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_IB_IF:
    val |= BIT(CPCAP_BIT_CLK_INV);
    val |= BIT(CPCAP_BIT_FS_INV);
    break;
    case SND_SOC_DAIFMT_IB_NF:
    val |= BIT(CPCAP_BIT_CLK_INV);
    val &= ~BIT(CPCAP_BIT_FS_INV);
    break;
    case SND_SOC_DAIFMT_NB_IF:
    val &= ~BIT(CPCAP_BIT_CLK_INV);
    val |= BIT(CPCAP_BIT_FS_INV);
    break;
    case SND_SOC_DAIFMT_NB_NF:
    val &= ~BIT(CPCAP_BIT_CLK_INV);
    val &= ~BIT(CPCAP_BIT_FS_INV);
    break;
    default:
    dev_err(component.dev, "Voice dai fmt failed: unsupported clock invert mode");
    break;
    }
    if (val & BIT(CPCAP_BIT_CLK_INV))
    val &= ~BIT(CPCAP_BIT_CLK_INV);
    else
    val |= BIT(CPCAP_BIT_CLK_INV);
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
// 11 - true I2S mode
    val |= BIT(CPCAP_BIT_CDC_DIG_AUD_FS0);
    val |= BIT(CPCAP_BIT_CDC_DIG_AUD_FS1);
    break;
    default:
// 4 timeslots network mode
    val |= BIT(CPCAP_BIT_CDC_DIG_AUD_FS0);
    val &= ~BIT(CPCAP_BIT_CDC_DIG_AUD_FS1);
    break;
    }
    dev_dbg(component.dev, "Voice dai format: val=%04x", val);
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_CDI, mask, val);
    if (err)
    return err;
    cpcap.codec_format = val;
    return 0;
    }
    static int cpcap_voice_set_mute(struct snd_soc_dai *dai,
    int mute, int direction)
    {
    struct snd_soc_component *component = dai.component;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    let mut reg: static u16 = CPCAP_REG_RXCOA;
    let mut mask: static u16 = BIT(CPCAP_BIT_CDC_SW);
    u16 val;
    if (mute)
    val = 0;
    else
    val = BIT(CPCAP_BIT_CDC_SW);
    dev_dbg(component.dev, "Voice mute: %d", mute);
    return regmap_update_bits(cpcap.regmap, reg, mask, val);
    };
    static const struct snd_soc_dai_ops cpcap_dai_voice_ops = {
    .hw_params	= cpcap_voice_hw_params,
    .set_sysclk	= cpcap_voice_set_dai_sysclk,
    .set_fmt	= cpcap_voice_set_dai_fmt,
    .mute_stream	= cpcap_voice_set_mute,
    .no_capture_mute = 1,
    };
    static struct snd_soc_dai_driver cpcap_dai[] = {
    {
    .id = 0,
    .name = "cpcap-hifi",
    .playback = {
    .stream_name = "HiFi Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FORMAT_S24_LE,
    },
    .ops = &cpcap_dai_hifi_ops,
    },
    {
    .id = 1,
    .name = "cpcap-voice",
    .playback = {
    .stream_name = "Voice Playback",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "Voice Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .ops = &cpcap_dai_voice_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn cpcap_dai_mux(cpcap: *mut cpcap_audio, swap_dai_configuration: bool) -> c_int {
    static int cpcap_dai_mux(struct cpcap_audio *cpcap, bool swap_dai_configuration)
    {
    u16 hifi_val, voice_val;
    let mut hifi_mask: u16 = BIT(CPCAP_BIT_DIG_AUD_IN_ST_DAC);
    let mut voice_mask: u16 = BIT(CPCAP_BIT_DIG_AUD_IN);
    int err;
    if (!swap_dai_configuration) {
// Codec on DAI0, HiFi on DAI1
    voice_val = 0;
    hifi_val = hifi_mask;
    } else {
// Codec on DAI1, HiFi on DAI0
    voice_val = voice_mask;
    hifi_val = 0;
    }
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_CDI,
    voice_mask, voice_val);
    if (err)
    return err;
    err = regmap_update_bits(cpcap.regmap, CPCAP_REG_SDACDI,
    hifi_mask, hifi_val);
    if (err)
    return err;
    return 0;
    }
    static int cpcap_audio_reset(struct snd_soc_component *component,
    bool swap_dai_configuration)
    {
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    int i, err = 0;
    dev_dbg(component.dev, "init audio codec");
    for (i = 0; i < ARRAY_SIZE(cpcap_default_regs); i++) {
    err = regmap_update_bits(cpcap.regmap,
    cpcap_default_regs[i].reg,
    cpcap_default_regs[i].mask,
    cpcap_default_regs[i].val);
    if (err)
    return err;
    }
// setup default settings
    err = cpcap_dai_mux(cpcap, swap_dai_configuration);
    if (err)
    return err;
    err = cpcap_set_sysclk(cpcap, CPCAP_DAI_HIFI, 0, 26000000);
    if (err)
    return err;
    err = cpcap_set_sysclk(cpcap, CPCAP_DAI_VOICE, 0, 26000000);
    if (err)
    return err;
    err = cpcap_set_samprate(cpcap, CPCAP_DAI_HIFI, 48000);
    if (err)
    return err;
    err = cpcap_set_samprate(cpcap, CPCAP_DAI_VOICE, 48000);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_hs_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cpcap_hs_irq_thread(int irq, void *data)
    {
    struct snd_soc_component *component = data;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct regmap *regmap = cpcap.regmap;
    let mut status: c_int = 0;
    let mut mask: c_int = SND_JACK_HEADSET;
    int val;
    if (!regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_HS_S))) {
    val = BIT(CPCAP_BIT_MB_ON2) | BIT(CPCAP_BIT_PTT_CMP_EN);
    regmap_update_bits(regmap, CPCAP_REG_TXI, val, val);
    val = BIT(CPCAP_BIT_ST_HS_CP_EN);
    regmap_update_bits(regmap, CPCAP_REG_RXOA, val, val);
    regulator_set_mode(cpcap.vaudio, REGULATOR_MODE_NORMAL);
// Give PTTS time to settle
    msleep(20);
    if (!regmap_test_bits(regmap, CPCAP_REG_INTS2,
    BIT(CPCAP_BIT_PTT_S))) {
// Headphones detected. (May also be a headset with the
// MFB pressed.)
//
    status = SND_JACK_HEADPHONE;
    dev_info(component.dev, "HP plugged in\n");
    } else if (regmap_test_bits(regmap, CPCAP_REG_INTS1,
    BIT(CPCAP_BIT_MB2_S)) == 1) {
    status = SND_JACK_HEADSET;
    dev_info(component.dev, "HS plugged in\n");
    } else
    dev_info(component.dev, "Unsupported HS plugged in\n");
    } else {
    let mut mic: bool = cpcap.jack.status & SND_JACK_MICROPHONE;
    dev_info(component.dev, "H%s disconnect\n", mic ? "S" : "P");
    val = BIT(CPCAP_BIT_MB_ON2) | BIT(CPCAP_BIT_PTT_CMP_EN);
    regmap_update_bits(cpcap.regmap, CPCAP_REG_TXI, val, 0);
    val = BIT(CPCAP_BIT_ST_HS_CP_EN);
    regmap_update_bits(cpcap.regmap, CPCAP_REG_RXOA, val, 0);
    regulator_set_mode(cpcap.vaudio, REGULATOR_MODE_STANDBY);
    mask |= SND_JACK_BTN_0;
    }
    snd_soc_jack_report(&cpcap.jack, status, mask);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_mb2_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cpcap_mb2_irq_thread(int irq, void *data)
    {
    struct snd_soc_component *component = data;
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    struct regmap *regmap = cpcap.regmap;
    let mut status: c_int = 0;
    int mb2;
    int ptt;
    if (regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_HS_S)) == 1)
    return IRQ_HANDLED;
    mb2 = regmap_test_bits(regmap, CPCAP_REG_INTS1, BIT(CPCAP_BIT_MB2_S));
    ptt = regmap_test_bits(regmap, CPCAP_REG_INTS2, BIT(CPCAP_BIT_PTT_S));
// Initial detection might have been with MFB pressed
    if (!(cpcap.jack.status & SND_JACK_MICROPHONE)) {
    if (ptt == 1 && mb2 == 1) {
    dev_info(component.dev, "MIC plugged in\n");
    snd_soc_jack_report(&cpcap.jack, SND_JACK_MICROPHONE,
    SND_JACK_MICROPHONE);
    }
    return IRQ_HANDLED;
    }
    if (!mb2 || !ptt)
    status = SND_JACK_BTN_0;
    snd_soc_jack_report(&cpcap.jack, status, SND_JACK_BTN_0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_soc_probe(component: *mut snd_soc_component) -> c_int {
    static int cpcap_soc_probe(struct snd_soc_component *component)
    {
    struct platform_device *pdev = to_platform_device(component.dev);
    struct snd_soc_card *card = component.card;
    struct cpcap_audio *cpcap;
    int err;
    cpcap = devm_kzalloc(component.dev, sizeof(*cpcap), GFP_KERNEL);
    if (!cpcap)
    return -ENOMEM;
    snd_soc_component_set_drvdata(component, cpcap);
    cpcap.component = component;
    cpcap.vaudio = devm_regulator_get(component.dev, "VAUDIO");
    if (IS_ERR(cpcap.vaudio))
    return dev_err_probe(component.dev, PTR_ERR(cpcap.vaudio),
    "Cannot get VAUDIO regulator\n");
    err = snd_soc_card_jack_new(card, "Headphones",
    SND_JACK_HEADSET | SND_JACK_BTN_0,
    &cpcap.jack);
    if (err < 0) {
    dev_err(component.dev, "Cannot create HS jack: %i\n", err);
    return err;
    }
    snd_jack_set_key(cpcap.jack.jack, SND_JACK_BTN_0, KEY_MEDIA);
    cpcap.regmap = dev_get_regmap(component.dev.parent, core::ptr::null_mut());
    if (!cpcap.regmap)
    return -ENODEV;
    snd_soc_component_init_regmap(component, cpcap.regmap);
    err = cpcap_get_vendor(component.dev, cpcap.regmap, &cpcap.vendor);
    if (err)
    return err;
    cpcap.hsirq = platform_get_irq_byname(pdev, "hs");
    if (cpcap.hsirq < 0)
    return cpcap.hsirq;
    err = devm_request_threaded_irq(component.dev, cpcap.hsirq, core::ptr::null_mut(),
    cpcap_hs_irq_thread,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    "cpcap-codec-hs",
    component);
    if (err) {
    dev_warn(component.dev, "no HS irq%i: %i\n",
    cpcap.hsirq, err);
    return err;
    }
    cpcap.mb2irq = platform_get_irq_byname(pdev, "mb2");
    if (cpcap.mb2irq < 0)
    return cpcap.mb2irq;
    err = devm_request_threaded_irq(component.dev, cpcap.mb2irq, core::ptr::null_mut(),
    cpcap_mb2_irq_thread,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    "cpcap-codec-mb2",
    component);
    if (err) {
    dev_warn(component.dev, "no MB2 irq%i: %i\n",
    cpcap.mb2irq, err);
    return err;
    }
    err = cpcap_audio_reset(component, false);
    if (err)
    return err;
    cpcap_hs_irq_thread(cpcap.hsirq, component);
    enable_irq_wake(cpcap.hsirq);
    enable_irq_wake(cpcap.mb2irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_soc_remove(component: *mut snd_soc_component) {
    static void cpcap_soc_remove(struct snd_soc_component *component)
    {
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
    disable_irq_wake(cpcap.hsirq);
    disable_irq_wake(cpcap.mb2irq);
    }
    static int cpcap_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    struct cpcap_audio *cpcap = snd_soc_component_get_drvdata(component);
// VAIDIO should be kept in normal mode in order MIC/PTT to work
    if (cpcap.jack.status & SND_JACK_MICROPHONE)
    return 0;
    switch (level) {
    case SND_SOC_BIAS_OFF:
    break;
    case SND_SOC_BIAS_PREPARE:
    regulator_set_mode(cpcap.vaudio, REGULATOR_MODE_NORMAL);
    break;
    case SND_SOC_BIAS_STANDBY:
    regulator_set_mode(cpcap.vaudio, REGULATOR_MODE_STANDBY);
    break;
    case SND_SOC_BIAS_ON:
    break;
    }
    return 0;
    }
    static const struct snd_soc_component_driver soc_codec_dev_cpcap = {
    .probe			= cpcap_soc_probe,
    .remove			= cpcap_soc_remove,
    .controls		= cpcap_snd_controls,
    .num_controls		= ARRAY_SIZE(cpcap_snd_controls),
    .dapm_widgets		= cpcap_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(cpcap_dapm_widgets),
    .dapm_routes		= intercon,
    .num_dapm_routes	= ARRAY_SIZE(intercon),
    .set_bias_level		= cpcap_set_bias_level,
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn cpcap_codec_probe(pdev: *mut platform_device) -> c_int {
    static int cpcap_codec_probe(struct platform_device *pdev)
    {
    struct device_node *codec_node =
    of_get_child_by_name(pdev.dev.parent.of_node, "audio-codec");
    if (!codec_node)
    return -ENODEV;
    pdev.dev.of_node = codec_node;
    return devm_snd_soc_register_component(&pdev.dev, &soc_codec_dev_cpcap,
    cpcap_dai, ARRAY_SIZE(cpcap_dai));
    }
    static struct platform_driver cpcap_codec_driver = {
    .probe		= cpcap_codec_probe,
    .driver		= {
    .name	= "cpcap-codec",
    },
    };
    module_platform_driver(cpcap_codec_driver);
    MODULE_ALIAS("platform:cpcap-codec");
    MODULE_DESCRIPTION("ASoC CPCAP codec driver");
    MODULE_AUTHOR("Sebastian Reichel");
    MODULE_LICENSE("GPL v2");
