//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5631.h
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
pub const RT5631_RESET: c_uint = 0x00;
pub const RT5631_SPK_OUT_VOL: c_uint = 0x02;
pub const RT5631_HP_OUT_VOL: c_uint = 0x04;
pub const RT5631_MONO_AXO_1_2_VOL: c_uint = 0x06;
pub const RT5631_AUX_IN_VOL: c_uint = 0x0A;
pub const RT5631_STEREO_DAC_VOL_1: c_uint = 0x0C;
pub const RT5631_MIC_CTRL_1: c_uint = 0x0E;
pub const RT5631_STEREO_DAC_VOL_2: c_uint = 0x10;
pub const RT5631_ADC_CTRL_1: c_uint = 0x12;
pub const RT5631_ADC_REC_MIXER: c_uint = 0x14;
pub const RT5631_ADC_CTRL_2: c_uint = 0x16;
pub const RT5631_VDAC_DIG_VOL: c_uint = 0x18;
pub const RT5631_OUTMIXER_L_CTRL: c_uint = 0x1A;
pub const RT5631_OUTMIXER_R_CTRL: c_uint = 0x1C;
pub const RT5631_AXO1MIXER_CTRL: c_uint = 0x1E;
pub const RT5631_AXO2MIXER_CTRL: c_uint = 0x20;
pub const RT5631_MIC_CTRL_2: c_uint = 0x22;
pub const RT5631_DIG_MIC_CTRL: c_uint = 0x24;
pub const RT5631_MONO_INPUT_VOL: c_uint = 0x26;
pub const RT5631_SPK_MIXER_CTRL: c_uint = 0x28;
pub const RT5631_SPK_MONO_OUT_CTRL: c_uint = 0x2A;
pub const RT5631_SPK_MONO_HP_OUT_CTRL: c_uint = 0x2C;
pub const RT5631_SDP_CTRL: c_uint = 0x34;
pub const RT5631_MONO_SDP_CTRL: c_uint = 0x36;
pub const RT5631_STEREO_AD_DA_CLK_CTRL: c_uint = 0x38;
pub const RT5631_PWR_MANAG_ADD1: c_uint = 0x3A;
pub const RT5631_PWR_MANAG_ADD2: c_uint = 0x3B;
pub const RT5631_PWR_MANAG_ADD3: c_uint = 0x3C;
pub const RT5631_PWR_MANAG_ADD4: c_uint = 0x3E;
pub const RT5631_GEN_PUR_CTRL_REG: c_uint = 0x40;
pub const RT5631_GLOBAL_CLK_CTRL: c_uint = 0x42;
pub const RT5631_PLL_CTRL: c_uint = 0x44;
pub const RT5631_INT_ST_IRQ_CTRL_1: c_uint = 0x48;
pub const RT5631_INT_ST_IRQ_CTRL_2: c_uint = 0x4A;
pub const RT5631_GPIO_CTRL: c_uint = 0x4C;
pub const RT5631_MISC_CTRL: c_uint = 0x52;
pub const RT5631_DEPOP_FUN_CTRL_1: c_uint = 0x54;
pub const RT5631_DEPOP_FUN_CTRL_2: c_uint = 0x56;
pub const RT5631_JACK_DET_CTRL: c_uint = 0x5A;
pub const RT5631_SOFT_VOL_CTRL: c_uint = 0x5C;
pub const RT5631_ALC_CTRL_1: c_uint = 0x64;
pub const RT5631_ALC_CTRL_2: c_uint = 0x65;
pub const RT5631_ALC_CTRL_3: c_uint = 0x66;
pub const RT5631_PSEUDO_SPATL_CTRL: c_uint = 0x68;
pub const RT5631_INDEX_ADD: c_uint = 0x6A;
pub const RT5631_INDEX_DATA: c_uint = 0x6C;
pub const RT5631_EQ_CTRL: c_uint = 0x6E;
pub const RT5631_VENDOR_ID: c_uint = 0x7A;
pub const RT5631_VENDOR_ID1: c_uint = 0x7C;
pub const RT5631_VENDOR_ID2: c_uint = 0x7E;
// Index of Codec Private Register definition
pub const RT5631_EQ_BW_LOP: c_uint = 0x00;
pub const RT5631_EQ_GAIN_LOP: c_uint = 0x01;
pub const RT5631_EQ_FC_BP1: c_uint = 0x02;
pub const RT5631_EQ_BW_BP1: c_uint = 0x03;
pub const RT5631_EQ_GAIN_BP1: c_uint = 0x04;
pub const RT5631_EQ_FC_BP2: c_uint = 0x05;
pub const RT5631_EQ_BW_BP2: c_uint = 0x06;
pub const RT5631_EQ_GAIN_BP2: c_uint = 0x07;
pub const RT5631_EQ_FC_BP3: c_uint = 0x08;
pub const RT5631_EQ_BW_BP3: c_uint = 0x09;
pub const RT5631_EQ_GAIN_BP3: c_uint = 0x0a;
pub const RT5631_EQ_BW_HIP: c_uint = 0x0b;
pub const RT5631_EQ_GAIN_HIP: c_uint = 0x0c;
pub const RT5631_EQ_HPF_A1: c_uint = 0x0d;
pub const RT5631_EQ_HPF_A2: c_uint = 0x0e;
pub const RT5631_EQ_HPF_GAIN: c_uint = 0x0f;
pub const RT5631_EQ_PRE_VOL_CTRL: c_uint = 0x11;
pub const RT5631_EQ_POST_VOL_CTRL: c_uint = 0x12;
pub const RT5631_TEST_MODE_CTRL: c_uint = 0x39;
pub const RT5631_CP_INTL_REG2: c_uint = 0x45;
pub const RT5631_ADDA_MIXER_INTL_REG3: c_uint = 0x52;
pub const RT5631_SPK_INTL_CTRL: c_uint = 0x56;
// global definition

pub const RT5631_L_MUTE_SHIFT: c_int = 15;

pub const RT5631_L_EN_SHIFT: c_int = 14;

pub const RT5631_R_MUTE_SHIFT: c_int = 7;

pub const RT5631_R_EN_SHIFT: c_int = 6;
pub const RT5631_VOL_MASK: c_uint = 0x1f;
pub const RT5631_L_VOL_SHIFT: c_int = 8;
pub const RT5631_R_VOL_SHIFT: c_int = 0;
// Speaker Output Control(0x02)

// Headphone Output Control(0x04)

// Output Control for AUXOUT/MONO(0x06)

pub const RT5631_MUTE_MONO_SHIFT: c_int = 13;

// Microphone Input Control 1(0x0E)

pub const RT5631_MIC1_DIFF_INPUT_SHIFT: c_int = 15;

pub const RT5631_MIC2_DIFF_INPUT_SHIFT: c_int = 7;
// Stereo DAC Digital Volume2(0x10)
pub const RT5631_DAC_VOL_MASK: c_uint = 0xff;
// ADC Recording Mixer Control(0x14)

pub const RT5631_M_OUTMIXL_RECMIXL_BIT: c_int = 15;

pub const RT5631_M_MIC1_RECMIXL_BIT: c_int = 14;

pub const RT5631_M_AXIL_RECMIXL_BIT: c_int = 13;

pub const RT5631_M_MONO_IN_RECMIXL_BIT: c_int = 12;

pub const RT5631_M_OUTMIXR_RECMIXR_BIT: c_int = 7;

pub const RT5631_M_MIC2_RECMIXR_BIT: c_int = 6;

pub const RT5631_M_AXIR_RECMIXR_BIT: c_int = 5;

pub const RT5631_M_MONO_IN_RECMIXR_BIT: c_int = 4;
// Left Output Mixer Control(0x1A)

pub const RT5631_M_RECMIXL_OUTMIXL_BIT: c_int = 15;

pub const RT5631_M_RECMIXR_OUTMIXL_BIT: c_int = 14;

pub const RT5631_M_DACL_OUTMIXL_BIT: c_int = 13;

pub const RT5631_M_MIC1_OUTMIXL_BIT: c_int = 12;

pub const RT5631_M_MIC2_OUTMIXL_BIT: c_int = 11;

pub const RT5631_M_MONO_INP_OUTMIXL_BIT: c_int = 10;

pub const RT5631_M_AXIL_OUTMIXL_BIT: c_int = 9;

pub const RT5631_M_AXIR_OUTMIXL_BIT: c_int = 8;

pub const RT5631_M_VDAC_OUTMIXL_BIT: c_int = 7;
// Right Output Mixer Control(0x1C)

pub const RT5631_M_RECMIXL_OUTMIXR_BIT: c_int = 15;

pub const RT5631_M_RECMIXR_OUTMIXR_BIT: c_int = 14;

pub const RT5631_M_DACR_OUTMIXR_BIT: c_int = 13;

pub const RT5631_M_MIC1_OUTMIXR_BIT: c_int = 12;

pub const RT5631_M_MIC2_OUTMIXR_BIT: c_int = 11;

pub const RT5631_M_MONO_INN_OUTMIXR_BIT: c_int = 10;

pub const RT5631_M_AXIL_OUTMIXR_BIT: c_int = 9;

pub const RT5631_M_AXIR_OUTMIXR_BIT: c_int = 8;

pub const RT5631_M_VDAC_OUTMIXR_BIT: c_int = 7;
// Lout Mixer Control(0x1E)

pub const RT5631_M_MIC1_AXO1MIX_BIT: c_int = 15;

pub const RT5631_M_MIC2_AXO1MIX_BIT: c_int = 11;

pub const RT5631_M_OUTMIXL_AXO1MIX_BIT: c_int = 7;

pub const RT5631_M_OUTMIXR_AXO1MIX_BIT: c_int = 6;
// Rout Mixer Control(0x20)

pub const RT5631_M_MIC1_AXO2MIX_BIT: c_int = 15;

pub const RT5631_M_MIC2_AXO2MIX_BIT: c_int = 11;

pub const RT5631_M_OUTMIXL_AXO2MIX_BIT: c_int = 7;

pub const RT5631_M_OUTMIXR_AXO2MIX_BIT: c_int = 6;
// Micphone Input Control 2(0x22)
pub const RT5631_MIC_BIAS_90_PRECNET_AVDD: c_int = 1;
pub const RT5631_MIC_BIAS_75_PRECNET_AVDD: c_int = 2;

pub const RT5631_MIC1_BOOST_SHIFT: c_int = 12;

pub const RT5631_MIC2_BOOST_SHIFT: c_int = 8;

// Digital Microphone Control(0x24)

pub const RT5631_DMIC_ENA_SHIFT: c_int = 15;
// DMIC_ENA: DMIC to ADC Digital filter

// DMIC_DIS: ADC mixer to ADC Digital filter

pub const RT5631_DMIC_L_CH_MUTE_SHIFT: c_int = 13;

pub const RT5631_DMIC_R_CH_MUTE_SHIFT: c_int = 12;

// Microphone Input Volume(0x26)
pub const RT5631_MONO_DIFF_INPUT_SHIFT: c_int = 15;
// Speaker Mixer Control(0x28)

pub const RT5631_M_RECMIXL_SPKMIXL_BIT: c_int = 15;

pub const RT5631_M_MIC1P_SPKMIXL_BIT: c_int = 14;

pub const RT5631_M_DACL_SPKMIXL_BIT: c_int = 13;

pub const RT5631_M_OUTMIXL_SPKMIXL_BIT: c_int = 12;

pub const RT5631_M_RECMIXR_SPKMIXR_BIT: c_int = 7;

pub const RT5631_M_MIC2P_SPKMIXR_BIT: c_int = 6;

pub const RT5631_M_DACR_SPKMIXR_BIT: c_int = 5;

pub const RT5631_M_OUTMIXR_SPKMIXR_BIT: c_int = 4;
// Speaker/Mono Output Control(0x2A)

pub const RT5631_M_SPKVOLL_SPOLMIX_BIT: c_int = 15;

pub const RT5631_M_SPKVOLR_SPOLMIX_BIT: c_int = 14;

pub const RT5631_M_SPKVOLL_SPORMIX_BIT: c_int = 13;

pub const RT5631_M_SPKVOLR_SPORMIX_BIT: c_int = 12;

pub const RT5631_M_OUTVOLL_MONOMIX_BIT: c_int = 11;

pub const RT5631_M_OUTVOLR_MONOMIX_BIT: c_int = 10;
// Speaker/Mono/HP Output Control(0x2C)

pub const RT5631_SPK_L_MUX_SEL_SHIFT: c_int = 14;

pub const RT5631_SPK_R_MUX_SEL_SHIFT: c_int = 10;

pub const RT5631_MONO_MUX_SEL_SHIFT: c_int = 6;

pub const RT5631_HP_L_MUX_SEL_SHIFT: c_int = 3;

pub const RT5631_HP_R_MUX_SEL_SHIFT: c_int = 2;
// Stereo I2S Serial Data Port Control(0x34)

// 0:Normal 1:Invert

// 0:Normal 1:Invert

// 0:ADC data appear at left phase of LRCK
// 1:ADC data appear at right phase of LRCK
//

// 0:DAC data appear at left phase of LRCK
// 1:DAC data appear at right phase of LRCK
//

// Data Length Slection

// PCM Data Format Selection

// Stereo AD/DA Clock Control(0x38h)

// CLOCK RELATIVE OF BCLK AND LCRK

// Power managment addition 1 (0x3A)

pub const RT5631_PWR_MAIN_I2S_BIT: c_int = 15;

pub const RT5631_PWR_CLASS_D_BIT: c_int = 12;

pub const RT5631_PWR_ADC_L_CLK_BIT: c_int = 11;

pub const RT5631_PWR_ADC_R_CLK_BIT: c_int = 10;

pub const RT5631_PWR_DAC_L_CLK_BIT: c_int = 9;

pub const RT5631_PWR_DAC_R_CLK_BIT: c_int = 8;

pub const RT5631_PWR_DAC_REF_BIT: c_int = 7;

pub const RT5631_PWR_DAC_L_TO_MIXER_BIT: c_int = 6;

pub const RT5631_PWR_DAC_R_TO_MIXER_BIT: c_int = 5;
// Power managment addition 2 (0x3B)

pub const RT5631_PWR_OUTMIXER_L_BIT: c_int = 15;

pub const RT5631_PWR_OUTMIXER_R_BIT: c_int = 14;

pub const RT5631_PWR_SPKMIXER_L_BIT: c_int = 13;

pub const RT5631_PWR_SPKMIXER_R_BIT: c_int = 12;

pub const RT5631_PWR_RECMIXER_L_BIT: c_int = 11;

pub const RT5631_PWR_RECMIXER_R_BIT: c_int = 10;

pub const RT5631_PWR_MIC1_BOOT_GAIN_BIT: c_int = 5;

pub const RT5631_PWR_MIC2_BOOT_GAIN_BIT: c_int = 4;

pub const RT5631_PWR_MICBIAS1_VOL_BIT: c_int = 3;

pub const RT5631_PWR_MICBIAS2_VOL_BIT: c_int = 2;

pub const RT5631_PWR_PLL1_BIT: c_int = 1;

pub const RT5631_PWR_PLL2_BIT: c_int = 0;
// Power managment addition 3(0x3C)

pub const RT5631_PWR_VREF_BIT: c_int = 15;

pub const RT5631_PWR_FAST_VREF_CTRL_BIT: c_int = 14;

pub const RT5631_PWR_MAIN_BIAS_BIT: c_int = 13;

pub const RT5631_PWR_AXO1MIXER_BIT: c_int = 11;

pub const RT5631_PWR_AXO2MIXER_BIT: c_int = 10;

pub const RT5631_PWR_MONOMIXER_BIT: c_int = 9;

pub const RT5631_PWR_MONO_DEPOP_DIS_BIT: c_int = 8;

pub const RT5631_PWR_MONO_AMP_EN_BIT: c_int = 7;

pub const RT5631_PWR_CHARGE_PUMP_BIT: c_int = 4;

pub const RT5631_PWR_HP_L_AMP_BIT: c_int = 3;

pub const RT5631_PWR_HP_R_AMP_BIT: c_int = 2;

pub const RT5631_PWR_HP_DEPOP_DIS_BIT: c_int = 1;

pub const RT5631_PWR_HP_AMP_DRIVING_BIT: c_int = 0;
// Power managment addition 4(0x3E)

pub const RT5631_PWR_SPK_L_VOL_BIT: c_int = 15;

pub const RT5631_PWR_SPK_R_VOL_BIT: c_int = 14;

pub const RT5631_PWR_LOUT_VOL_BIT: c_int = 13;

pub const RT5631_PWR_ROUT_VOL_BIT: c_int = 12;

pub const RT5631_PWR_HP_L_OUT_VOL_BIT: c_int = 11;

pub const RT5631_PWR_HP_R_OUT_VOL_BIT: c_int = 10;

pub const RT5631_PWR_AXIL_IN_VOL_BIT: c_int = 9;

pub const RT5631_PWR_AXIR_IN_VOL_BIT: c_int = 8;

pub const RT5631_PWR_MONO_IN_P_VOL_BIT: c_int = 7;

pub const RT5631_PWR_MONO_IN_N_VOL_BIT: c_int = 6;
// General Purpose Control Register(0x40)

pub const RT5631_SPK_AMP_RATIO_CTRL_SHIFT: c_int = 12;

// Select ADC Wind Filter Clock type

// SelectADC Wind Filter Corner Frequency

// Global Clock Control Register(0x42)

// PLL Control(0x44)

// Internal Status and IRQ Control2(0x4A)

pub const RT5631_ADC_DATA_SEL_MIC1_SHIFT: c_int = 14;

pub const RT5631_ADC_DATA_SEL_MIC2_SHIFT: c_int = 15;

pub const RT5631_ADC_DATA_SEL_SHIFT: c_int = 14;
// GPIO Pin Configuration(0x4C)

// De-POP function Control 1(0x54)

// Power Down HPAMP_L Starts Up Signal

// Power Down HPAMP_R Starts Up Signal

// Enable left HP mute/unmute depop

// Enable right HP mute/unmute depop

// De-POP Fnction Control(0x56)

// Jack Detect Control Register(0x5A)

// JD trigger enable for HP

// JD trigger enable for speaker LP/LN

// JD trigger enable for speaker RP/RN

// JD trigger enable for monoout

// JD trigger enable for Lout

// JD trigger enable for Rout

// ALC CONTROL 1(0x64)

// ALC CONTROL 2(0x65)
// select Compensation gain for Noise gate function

// ALC CONTROL 3(0x66)

// ALC noise gate hold data function

// Psedueo Stereo & Spatial Effect Block Control(0x68)

// 3D gain parameter

// 3D ratio parameter

// select samplerate for all pass filter

// EQ CONTROL 1(0x6E)

