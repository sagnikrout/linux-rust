//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/audio.h
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
// audio.h  --  Audio Driver for Wolfson WM8350 PMIC
//
// Copyright 2007, 2008 Wolfson Microelectronics PLC
//

pub const WM8350_CLOCK_CONTROL_1: c_uint = 0x28;
pub const WM8350_CLOCK_CONTROL_2: c_uint = 0x29;
pub const WM8350_FLL_CONTROL_1: c_uint = 0x2A;
pub const WM8350_FLL_CONTROL_2: c_uint = 0x2B;
pub const WM8350_FLL_CONTROL_3: c_uint = 0x2C;
pub const WM8350_FLL_CONTROL_4: c_uint = 0x2D;
pub const WM8350_DAC_CONTROL: c_uint = 0x30;
pub const WM8350_DAC_DIGITAL_VOLUME_L: c_uint = 0x32;
pub const WM8350_DAC_DIGITAL_VOLUME_R: c_uint = 0x33;
pub const WM8350_DAC_LR_RATE: c_uint = 0x35;
pub const WM8350_DAC_CLOCK_CONTROL: c_uint = 0x36;
pub const WM8350_DAC_MUTE: c_uint = 0x3A;
pub const WM8350_DAC_MUTE_VOLUME: c_uint = 0x3B;
pub const WM8350_DAC_SIDE: c_uint = 0x3C;
pub const WM8350_ADC_CONTROL: c_uint = 0x40;
pub const WM8350_ADC_DIGITAL_VOLUME_L: c_uint = 0x42;
pub const WM8350_ADC_DIGITAL_VOLUME_R: c_uint = 0x43;
pub const WM8350_ADC_DIVIDER: c_uint = 0x44;
pub const WM8350_ADC_LR_RATE: c_uint = 0x46;
pub const WM8350_INPUT_CONTROL: c_uint = 0x48;
pub const WM8350_IN3_INPUT_CONTROL: c_uint = 0x49;
pub const WM8350_MIC_BIAS_CONTROL: c_uint = 0x4A;
pub const WM8350_OUTPUT_CONTROL: c_uint = 0x4C;
pub const WM8350_JACK_DETECT: c_uint = 0x4D;
pub const WM8350_ANTI_POP_CONTROL: c_uint = 0x4E;
pub const WM8350_LEFT_INPUT_VOLUME: c_uint = 0x50;
pub const WM8350_RIGHT_INPUT_VOLUME: c_uint = 0x51;
pub const WM8350_LEFT_MIXER_CONTROL: c_uint = 0x58;
pub const WM8350_RIGHT_MIXER_CONTROL: c_uint = 0x59;
pub const WM8350_OUT3_MIXER_CONTROL: c_uint = 0x5C;
pub const WM8350_OUT4_MIXER_CONTROL: c_uint = 0x5D;
pub const WM8350_OUTPUT_LEFT_MIXER_VOLUME: c_uint = 0x60;
pub const WM8350_OUTPUT_RIGHT_MIXER_VOLUME: c_uint = 0x61;
pub const WM8350_INPUT_MIXER_VOLUME_L: c_uint = 0x62;
pub const WM8350_INPUT_MIXER_VOLUME_R: c_uint = 0x63;
pub const WM8350_INPUT_MIXER_VOLUME: c_uint = 0x64;
pub const WM8350_LOUT1_VOLUME: c_uint = 0x68;
pub const WM8350_ROUT1_VOLUME: c_uint = 0x69;
pub const WM8350_LOUT2_VOLUME: c_uint = 0x6A;
pub const WM8350_ROUT2_VOLUME: c_uint = 0x6B;
pub const WM8350_BEEP_VOLUME: c_uint = 0x6F;
pub const WM8350_AI_FORMATING: c_uint = 0x70;
pub const WM8350_ADC_DAC_COMP: c_uint = 0x71;
pub const WM8350_AI_ADC_CONTROL: c_uint = 0x72;
pub const WM8350_AI_DAC_CONTROL: c_uint = 0x73;
pub const WM8350_AIF_TEST: c_uint = 0x74;
pub const WM8350_JACK_PIN_STATUS: c_uint = 0xE7;
// Bit values for R08 (0x08)

pub const WM8350_VMID_OFF: c_int = 0;
pub const WM8350_VMID_300K: c_int = 1;
pub const WM8350_VMID_50K: c_int = 2;
pub const WM8350_VMID_5K: c_int = 3;
//
// R40 (0x28) - Clock Control 1
//
pub const WM8350_TOCLK_RATE: c_uint = 0x4000;
pub const WM8350_MCLK_SEL: c_uint = 0x0800;
pub const WM8350_MCLK_DIV_MASK: c_uint = 0x0100;
pub const WM8350_BCLK_DIV_MASK: c_uint = 0x00F0;
pub const WM8350_OPCLK_DIV_MASK: c_uint = 0x0007;
//
// R41 (0x29) - Clock Control 2
//
pub const WM8350_LRC_ADC_SEL: c_uint = 0x8000;
pub const WM8350_MCLK_DIR: c_uint = 0x0001;
//
// R42 (0x2A) - FLL Control 1
//
pub const WM8350_FLL_DITHER_WIDTH_MASK: c_uint = 0x3000;
pub const WM8350_FLL_DITHER_HP: c_uint = 0x0800;
pub const WM8350_FLL_OUTDIV_MASK: c_uint = 0x0700;
pub const WM8350_FLL_RSP_RATE_MASK: c_uint = 0x00F0;
pub const WM8350_FLL_RATE_MASK: c_uint = 0x0007;
//
// R43 (0x2B) - FLL Control 2
//
pub const WM8350_FLL_RATIO_MASK: c_uint = 0xF800;
pub const WM8350_FLL_N_MASK: c_uint = 0x03FF;
//
// R44 (0x2C) - FLL Control 3
//
pub const WM8350_FLL_K_MASK: c_uint = 0xFFFF;
//
// R45 (0x2D) - FLL Control 4
//
pub const WM8350_FLL_FRAC: c_uint = 0x0020;
pub const WM8350_FLL_SLOW_LOCK_REF: c_uint = 0x0010;
pub const WM8350_FLL_CLK_SRC_MASK: c_uint = 0x0003;
//
// R48 (0x30) - DAC Control
//
pub const WM8350_DAC_MONO: c_uint = 0x2000;
pub const WM8350_AIF_LRCLKRATE: c_uint = 0x1000;
pub const WM8350_DEEMP_MASK: c_uint = 0x0030;
pub const WM8350_DACL_DATINV: c_uint = 0x0002;
pub const WM8350_DACR_DATINV: c_uint = 0x0001;
//
// R50 (0x32) - DAC Digital Volume L
//
pub const WM8350_DAC_VU: c_uint = 0x0100;
pub const WM8350_DACL_VOL_MASK: c_uint = 0x00FF;
//
// R51 (0x33) - DAC Digital Volume R
//
pub const WM8350_DAC_VU: c_uint = 0x0100;
pub const WM8350_DACR_VOL_MASK: c_uint = 0x00FF;
//
// R53 (0x35) - DAC LR Rate
//
pub const WM8350_DACLRC_ENA: c_uint = 0x0800;
pub const WM8350_DACLRC_RATE_MASK: c_uint = 0x07FF;
//
// R54 (0x36) - DAC Clock Control
//
pub const WM8350_DACCLK_POL: c_uint = 0x0010;
pub const WM8350_DAC_CLKDIV_MASK: c_uint = 0x0007;
//
// R58 (0x3A) - DAC Mute
//
pub const WM8350_DAC_MUTE_ENA: c_uint = 0x4000;
//
// R59 (0x3B) - DAC Mute Volume
//
pub const WM8350_DAC_MUTEMODE: c_uint = 0x4000;
pub const WM8350_DAC_MUTERATE: c_uint = 0x2000;
pub const WM8350_DAC_SB_FILT: c_uint = 0x1000;
//
// R60 (0x3C) - DAC Side
//
pub const WM8350_ADC_TO_DACL_MASK: c_uint = 0x3000;
pub const WM8350_ADC_TO_DACR_MASK: c_uint = 0x0C00;
//
// R64 (0x40) - ADC Control
//
pub const WM8350_ADC_HPF_CUT_MASK: c_uint = 0x0300;
pub const WM8350_ADCL_DATINV: c_uint = 0x0002;
pub const WM8350_ADCR_DATINV: c_uint = 0x0001;
//
// R66 (0x42) - ADC Digital Volume L
//
pub const WM8350_ADC_VU: c_uint = 0x0100;
pub const WM8350_ADCL_VOL_MASK: c_uint = 0x00FF;
//
// R67 (0x43) - ADC Digital Volume R
//
pub const WM8350_ADC_VU: c_uint = 0x0100;
pub const WM8350_ADCR_VOL_MASK: c_uint = 0x00FF;
//
// R68 (0x44) - ADC Divider
//
pub const WM8350_ADCL_DAC_SVOL_MASK: c_uint = 0x0F00;
pub const WM8350_ADCR_DAC_SVOL_MASK: c_uint = 0x00F0;
pub const WM8350_ADCCLK_POL: c_uint = 0x0008;
pub const WM8350_ADC_CLKDIV_MASK: c_uint = 0x0007;
//
// R70 (0x46) - ADC LR Rate
//
pub const WM8350_ADCLRC_ENA: c_uint = 0x0800;
pub const WM8350_ADCLRC_RATE_MASK: c_uint = 0x07FF;
//
// R72 (0x48) - Input Control
//
pub const WM8350_IN2R_ENA: c_uint = 0x0400;
pub const WM8350_IN1RN_ENA: c_uint = 0x0200;
pub const WM8350_IN1RP_ENA: c_uint = 0x0100;
pub const WM8350_IN2L_ENA: c_uint = 0x0004;
pub const WM8350_IN1LN_ENA: c_uint = 0x0002;
pub const WM8350_IN1LP_ENA: c_uint = 0x0001;
//
// R73 (0x49) - IN3 Input Control
//
pub const WM8350_IN3R_SHORT: c_uint = 0x4000;
pub const WM8350_IN3L_SHORT: c_uint = 0x0040;
//
// R74 (0x4A) - Mic Bias Control
//
pub const WM8350_MICBSEL: c_uint = 0x4000;
pub const WM8350_MCDTHR_MASK: c_uint = 0x001C;
pub const WM8350_MCDSCTHR_MASK: c_uint = 0x0003;
//
// R76 (0x4C) - Output Control
//
pub const WM8350_OUT4_VROI: c_uint = 0x0800;
pub const WM8350_OUT3_VROI: c_uint = 0x0400;
pub const WM8350_OUT2_VROI: c_uint = 0x0200;
pub const WM8350_OUT1_VROI: c_uint = 0x0100;
pub const WM8350_OUT2_FB: c_uint = 0x0004;
pub const WM8350_OUT1_FB: c_uint = 0x0001;
//
// R77 (0x4D) - Jack Detect
//
pub const WM8350_JDL_ENA: c_uint = 0x8000;
pub const WM8350_JDR_ENA: c_uint = 0x4000;
//
// R78 (0x4E) - Anti Pop Control
//
pub const WM8350_ANTI_POP_MASK: c_uint = 0x0300;
pub const WM8350_DIS_OP_LN4_MASK: c_uint = 0x00C0;
pub const WM8350_DIS_OP_LN3_MASK: c_uint = 0x0030;
pub const WM8350_DIS_OP_OUT2_MASK: c_uint = 0x000C;
pub const WM8350_DIS_OP_OUT1_MASK: c_uint = 0x0003;
//
// R80 (0x50) - Left Input Volume
//
pub const WM8350_INL_MUTE: c_uint = 0x4000;
pub const WM8350_INL_ZC: c_uint = 0x2000;
pub const WM8350_IN_VU: c_uint = 0x0100;
pub const WM8350_INL_VOL_MASK: c_uint = 0x00FC;
//
// R81 (0x51) - Right Input Volume
//
pub const WM8350_INR_MUTE: c_uint = 0x4000;
pub const WM8350_INR_ZC: c_uint = 0x2000;
pub const WM8350_IN_VU: c_uint = 0x0100;
pub const WM8350_INR_VOL_MASK: c_uint = 0x00FC;
//
// R88 (0x58) - Left Mixer Control
//
pub const WM8350_DACR_TO_MIXOUTL: c_uint = 0x1000;
pub const WM8350_DACL_TO_MIXOUTL: c_uint = 0x0800;
pub const WM8350_IN3L_TO_MIXOUTL: c_uint = 0x0004;
pub const WM8350_INR_TO_MIXOUTL: c_uint = 0x0002;
pub const WM8350_INL_TO_MIXOUTL: c_uint = 0x0001;
//
// R89 (0x59) - Right Mixer Control
//
pub const WM8350_DACR_TO_MIXOUTR: c_uint = 0x1000;
pub const WM8350_DACL_TO_MIXOUTR: c_uint = 0x0800;
pub const WM8350_IN3R_TO_MIXOUTR: c_uint = 0x0008;
pub const WM8350_INR_TO_MIXOUTR: c_uint = 0x0002;
pub const WM8350_INL_TO_MIXOUTR: c_uint = 0x0001;
//
// R92 (0x5C) - OUT3 Mixer Control
//
pub const WM8350_DACL_TO_OUT3: c_uint = 0x0800;
pub const WM8350_MIXINL_TO_OUT3: c_uint = 0x0100;
pub const WM8350_OUT4_TO_OUT3: c_uint = 0x0008;
pub const WM8350_MIXOUTL_TO_OUT3: c_uint = 0x0001;
//
// R93 (0x5D) - OUT4 Mixer Control
//
pub const WM8350_DACR_TO_OUT4: c_uint = 0x1000;
pub const WM8350_DACL_TO_OUT4: c_uint = 0x0800;
pub const WM8350_OUT4_ATTN: c_uint = 0x0400;
pub const WM8350_MIXINR_TO_OUT4: c_uint = 0x0200;
pub const WM8350_OUT3_TO_OUT4: c_uint = 0x0004;
pub const WM8350_MIXOUTR_TO_OUT4: c_uint = 0x0002;
pub const WM8350_MIXOUTL_TO_OUT4: c_uint = 0x0001;
//
// R96 (0x60) - Output Left Mixer Volume
//
pub const WM8350_IN3L_MIXOUTL_VOL_MASK: c_uint = 0x0E00;
pub const WM8350_IN3L_MIXOUTL_VOL_SHIFT: c_int = 9;
pub const WM8350_INR_MIXOUTL_VOL_MASK: c_uint = 0x00E0;
pub const WM8350_INR_MIXOUTL_VOL_SHIFT: c_int = 5;
pub const WM8350_INL_MIXOUTL_VOL_MASK: c_uint = 0x000E;
pub const WM8350_INL_MIXOUTL_VOL_SHIFT: c_int = 1;
// Bit values for R96 (0x60)
pub const WM8350_IN3L_MIXOUTL_VOL_OFF: c_int = 0;
pub const WM8350_IN3L_MIXOUTL_VOL_M12DB: c_int = 1;
pub const WM8350_IN3L_MIXOUTL_VOL_M9DB: c_int = 2;
pub const WM8350_IN3L_MIXOUTL_VOL_M6DB: c_int = 3;
pub const WM8350_IN3L_MIXOUTL_VOL_M3DB: c_int = 4;
pub const WM8350_IN3L_MIXOUTL_VOL_0DB: c_int = 5;
pub const WM8350_IN3L_MIXOUTL_VOL_3DB: c_int = 6;
pub const WM8350_IN3L_MIXOUTL_VOL_6DB: c_int = 7;
pub const WM8350_INR_MIXOUTL_VOL_OFF: c_int = 0;
pub const WM8350_INR_MIXOUTL_VOL_M12DB: c_int = 1;
pub const WM8350_INR_MIXOUTL_VOL_M9DB: c_int = 2;
pub const WM8350_INR_MIXOUTL_VOL_M6DB: c_int = 3;
pub const WM8350_INR_MIXOUTL_VOL_M3DB: c_int = 4;
pub const WM8350_INR_MIXOUTL_VOL_0DB: c_int = 5;
pub const WM8350_INR_MIXOUTL_VOL_3DB: c_int = 6;
pub const WM8350_INR_MIXOUTL_VOL_6DB: c_int = 7;
pub const WM8350_INL_MIXOUTL_VOL_OFF: c_int = 0;
pub const WM8350_INL_MIXOUTL_VOL_M12DB: c_int = 1;
pub const WM8350_INL_MIXOUTL_VOL_M9DB: c_int = 2;
pub const WM8350_INL_MIXOUTL_VOL_M6DB: c_int = 3;
pub const WM8350_INL_MIXOUTL_VOL_M3DB: c_int = 4;
pub const WM8350_INL_MIXOUTL_VOL_0DB: c_int = 5;
pub const WM8350_INL_MIXOUTL_VOL_3DB: c_int = 6;
pub const WM8350_INL_MIXOUTL_VOL_6DB: c_int = 7;
//
// R97 (0x61) - Output Right Mixer Volume
//
pub const WM8350_IN3R_MIXOUTR_VOL_MASK: c_uint = 0xE000;
pub const WM8350_IN3R_MIXOUTR_VOL_SHIFT: c_int = 13;
pub const WM8350_INR_MIXOUTR_VOL_MASK: c_uint = 0x00E0;
pub const WM8350_INR_MIXOUTR_VOL_SHIFT: c_int = 5;
pub const WM8350_INL_MIXOUTR_VOL_MASK: c_uint = 0x000E;
pub const WM8350_INL_MIXOUTR_VOL_SHIFT: c_int = 1;
// Bit values for R96 (0x60)
pub const WM8350_IN3R_MIXOUTR_VOL_OFF: c_int = 0;
pub const WM8350_IN3R_MIXOUTR_VOL_M12DB: c_int = 1;
pub const WM8350_IN3R_MIXOUTR_VOL_M9DB: c_int = 2;
pub const WM8350_IN3R_MIXOUTR_VOL_M6DB: c_int = 3;
pub const WM8350_IN3R_MIXOUTR_VOL_M3DB: c_int = 4;
pub const WM8350_IN3R_MIXOUTR_VOL_0DB: c_int = 5;
pub const WM8350_IN3R_MIXOUTR_VOL_3DB: c_int = 6;
pub const WM8350_IN3R_MIXOUTR_VOL_6DB: c_int = 7;
pub const WM8350_INR_MIXOUTR_VOL_OFF: c_int = 0;
pub const WM8350_INR_MIXOUTR_VOL_M12DB: c_int = 1;
pub const WM8350_INR_MIXOUTR_VOL_M9DB: c_int = 2;
pub const WM8350_INR_MIXOUTR_VOL_M6DB: c_int = 3;
pub const WM8350_INR_MIXOUTR_VOL_M3DB: c_int = 4;
pub const WM8350_INR_MIXOUTR_VOL_0DB: c_int = 5;
pub const WM8350_INR_MIXOUTR_VOL_3DB: c_int = 6;
pub const WM8350_INR_MIXOUTR_VOL_6DB: c_int = 7;
pub const WM8350_INL_MIXOUTR_VOL_OFF: c_int = 0;
pub const WM8350_INL_MIXOUTR_VOL_M12DB: c_int = 1;
pub const WM8350_INL_MIXOUTR_VOL_M9DB: c_int = 2;
pub const WM8350_INL_MIXOUTR_VOL_M6DB: c_int = 3;
pub const WM8350_INL_MIXOUTR_VOL_M3DB: c_int = 4;
pub const WM8350_INL_MIXOUTR_VOL_0DB: c_int = 5;
pub const WM8350_INL_MIXOUTR_VOL_3DB: c_int = 6;
pub const WM8350_INL_MIXOUTR_VOL_6DB: c_int = 7;
//
// R98 (0x62) - Input Mixer Volume L
//
pub const WM8350_IN3L_MIXINL_VOL_MASK: c_uint = 0x0E00;
pub const WM8350_IN2L_MIXINL_VOL_MASK: c_uint = 0x000E;
pub const WM8350_INL_MIXINL_VOL: c_uint = 0x0001;
//
// R99 (0x63) - Input Mixer Volume R
//
pub const WM8350_IN3R_MIXINR_VOL_MASK: c_uint = 0xE000;
pub const WM8350_IN2R_MIXINR_VOL_MASK: c_uint = 0x00E0;
pub const WM8350_INR_MIXINR_VOL: c_uint = 0x0001;
//
// R100 (0x64) - Input Mixer Volume
//
pub const WM8350_OUT4_MIXIN_DST: c_uint = 0x8000;
pub const WM8350_OUT4_MIXIN_VOL_MASK: c_uint = 0x000E;
//
// R104 (0x68) - LOUT1 Volume
//
pub const WM8350_OUT1L_MUTE: c_uint = 0x4000;
pub const WM8350_OUT1L_ZC: c_uint = 0x2000;
pub const WM8350_OUT1_VU: c_uint = 0x0100;
pub const WM8350_OUT1L_VOL_MASK: c_uint = 0x00FC;
pub const WM8350_OUT1L_VOL_SHIFT: c_int = 2;
//
// R105 (0x69) - ROUT1 Volume
//
pub const WM8350_OUT1R_MUTE: c_uint = 0x4000;
pub const WM8350_OUT1R_ZC: c_uint = 0x2000;
pub const WM8350_OUT1_VU: c_uint = 0x0100;
pub const WM8350_OUT1R_VOL_MASK: c_uint = 0x00FC;
pub const WM8350_OUT1R_VOL_SHIFT: c_int = 2;
//
// R106 (0x6A) - LOUT2 Volume
//
pub const WM8350_OUT2L_MUTE: c_uint = 0x4000;
pub const WM8350_OUT2L_ZC: c_uint = 0x2000;
pub const WM8350_OUT2_VU: c_uint = 0x0100;
pub const WM8350_OUT2L_VOL_MASK: c_uint = 0x00FC;
//
// R107 (0x6B) - ROUT2 Volume
//
pub const WM8350_OUT2R_MUTE: c_uint = 0x4000;
pub const WM8350_OUT2R_ZC: c_uint = 0x2000;
pub const WM8350_OUT2R_INV: c_uint = 0x0400;
pub const WM8350_OUT2R_INV_MUTE: c_uint = 0x0200;
pub const WM8350_OUT2_VU: c_uint = 0x0100;
pub const WM8350_OUT2R_VOL_MASK: c_uint = 0x00FC;
//
// R111 (0x6F) - BEEP Volume
//
pub const WM8350_IN3R_OUT2R_VOL_MASK: c_uint = 0x00E0;
//
// R112 (0x70) - AI Formating
//
pub const WM8350_AIF_BCLK_INV: c_uint = 0x8000;
pub const WM8350_AIF_TRI: c_uint = 0x2000;
pub const WM8350_AIF_LRCLK_INV: c_uint = 0x1000;
pub const WM8350_AIF_WL_MASK: c_uint = 0x0C00;
pub const WM8350_AIF_FMT_MASK: c_uint = 0x0300;
//
// R113 (0x71) - ADC DAC COMP
//
pub const WM8350_DAC_COMP: c_uint = 0x0080;
pub const WM8350_DAC_COMPMODE: c_uint = 0x0040;
pub const WM8350_ADC_COMP: c_uint = 0x0020;
pub const WM8350_ADC_COMPMODE: c_uint = 0x0010;
pub const WM8350_LOOPBACK: c_uint = 0x0001;
//
// R114 (0x72) - AI ADC Control
//
pub const WM8350_AIFADC_PD: c_uint = 0x0080;
pub const WM8350_AIFADCL_SRC: c_uint = 0x0040;
pub const WM8350_AIFADCR_SRC: c_uint = 0x0020;
pub const WM8350_AIFADC_TDM_CHAN: c_uint = 0x0010;
pub const WM8350_AIFADC_TDM: c_uint = 0x0008;
//
// R115 (0x73) - AI DAC Control
//
pub const WM8350_BCLK_MSTR: c_uint = 0x4000;
pub const WM8350_AIFDAC_PD: c_uint = 0x0080;
pub const WM8350_DACL_SRC: c_uint = 0x0040;
pub const WM8350_DACR_SRC: c_uint = 0x0020;
pub const WM8350_AIFDAC_TDM_CHAN: c_uint = 0x0010;
pub const WM8350_AIFDAC_TDM: c_uint = 0x0008;
pub const WM8350_DAC_BOOST_MASK: c_uint = 0x0003;
//
// R116 (0x74) - AIF Test
//
pub const WM8350_CODEC_BYP: c_uint = 0x4000;
pub const WM8350_AIFADC_WR_TST: c_uint = 0x2000;
pub const WM8350_AIFADC_RD_TST: c_uint = 0x1000;
pub const WM8350_AIFDAC_WR_TST: c_uint = 0x0800;
pub const WM8350_AIFDAC_RD_TST: c_uint = 0x0400;
pub const WM8350_AIFADC_ASYN: c_uint = 0x0020;
pub const WM8350_AIFDAC_ASYN: c_uint = 0x0010;
//
// R231 (0xE7) - Jack Status
//
pub const WM8350_JACK_L_LVL: c_uint = 0x0800;
pub const WM8350_JACK_R_LVL: c_uint = 0x0400;
pub const WM8350_JACK_MICSCD_LVL: c_uint = 0x0200;
pub const WM8350_JACK_MICSD_LVL: c_uint = 0x0100;
//
// WM8350 Platform setup
//
pub const WM8350_S_CURVE_NONE: c_uint = 0x0;
pub const WM8350_S_CURVE_FAST: c_uint = 0x1;
pub const WM8350_S_CURVE_MEDIUM: c_uint = 0x2;
pub const WM8350_S_CURVE_SLOW: c_uint = 0x3;
pub const WM8350_DISCHARGE_OFF: c_uint = 0x0;
pub const WM8350_DISCHARGE_FAST: c_uint = 0x1;
pub const WM8350_DISCHARGE_MEDIUM: c_uint = 0x2;
pub const WM8350_DISCHARGE_SLOW: c_uint = 0x3;
pub const WM8350_TIE_OFF_500R: c_uint = 0x0;
pub const WM8350_TIE_OFF_30K: c_uint = 0x1;
//
// Clock sources & directions
//
pub const WM8350_SYSCLK: c_int = 0;
pub const WM8350_MCLK_SEL_PLL_MCLK: c_int = 0;
pub const WM8350_MCLK_SEL_PLL_DAC: c_int = 1;
pub const WM8350_MCLK_SEL_PLL_ADC: c_int = 2;
pub const WM8350_MCLK_SEL_PLL_32K: c_int = 3;
pub const WM8350_MCLK_SEL_MCLK: c_int = 5;
// clock divider id's
pub const WM8350_ADC_CLKDIV: c_int = 0;
pub const WM8350_DAC_CLKDIV: c_int = 1;
pub const WM8350_BCLK_CLKDIV: c_int = 2;
pub const WM8350_OPCLK_CLKDIV: c_int = 3;
pub const WM8350_TO_CLKDIV: c_int = 4;
pub const WM8350_SYS_CLKDIV: c_int = 5;
pub const WM8350_DACLR_CLKDIV: c_int = 6;
pub const WM8350_ADCLR_CLKDIV: c_int = 7;
// ADC clock dividers
pub const WM8350_ADCDIV_1: c_uint = 0x0;
pub const WM8350_ADCDIV_1_5: c_uint = 0x1;
pub const WM8350_ADCDIV_2: c_uint = 0x2;
pub const WM8350_ADCDIV_3: c_uint = 0x3;
pub const WM8350_ADCDIV_4: c_uint = 0x4;
pub const WM8350_ADCDIV_5_5: c_uint = 0x5;
pub const WM8350_ADCDIV_6: c_uint = 0x6;
// ADC clock dividers
pub const WM8350_DACDIV_1: c_uint = 0x0;
pub const WM8350_DACDIV_1_5: c_uint = 0x1;
pub const WM8350_DACDIV_2: c_uint = 0x2;
pub const WM8350_DACDIV_3: c_uint = 0x3;
pub const WM8350_DACDIV_4: c_uint = 0x4;
pub const WM8350_DACDIV_5_5: c_uint = 0x5;
pub const WM8350_DACDIV_6: c_uint = 0x6;
// BCLK clock dividers

// Sys (MCLK) clock dividers

// OP clock dividers
pub const WM8350_OPCLK_DIV_1: c_uint = 0x0;
pub const WM8350_OPCLK_DIV_2: c_uint = 0x1;
pub const WM8350_OPCLK_DIV_3: c_uint = 0x2;
pub const WM8350_OPCLK_DIV_4: c_uint = 0x3;
pub const WM8350_OPCLK_DIV_5_5: c_uint = 0x4;
pub const WM8350_OPCLK_DIV_6: c_uint = 0x5;
// DAI ID
pub const WM8350_HIFI_DAI: c_int = 0;
//
// Audio interrupts.
//
pub const WM8350_IRQ_CODEC_JCK_DET_L: c_int = 39;
pub const WM8350_IRQ_CODEC_JCK_DET_R: c_int = 40;
pub const WM8350_IRQ_CODEC_MICSCD: c_int = 41;
pub const WM8350_IRQ_CODEC_MICD: c_int = 42;
//
// WM8350 Platform data.
//
// This must be initialised per platform for best audio performance.
// Please see WM8350 datasheet for information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_audio_platform_data {
    pub /: *mut *mut int vmid_discharge_msecs; / VMID --> OFF discharge time,
    pub /: *mut *mut int drain_msecs; / OFF drain time,
    pub /: *mut *mut int cap_discharge_msecs; / Cap ON (from OFF) discharge time,
    pub /: *mut *mut int vmid_charge_msecs; / vmid power up time,
    pub /: *mut *mut u32 vmid_s_curve:2; / vmid enable s curve speed,
    pub /: *mut *mut u32 dis_out4:2; / out4 discharge speed,
    pub /: *mut *mut u32 dis_out3:2; / out3 discharge speed,
    pub /: *mut *mut u32 dis_out2:2; / out2 discharge speed,
    pub /: *mut *mut u32 dis_out1:2; / out1 discharge speed,
    pub /: *mut *mut u32 vroi_out4:1; / out4 tie off,
    pub /: *mut *mut u32 vroi_out3:1; / out3 tie off,
    pub /: *mut *mut u32 vroi_out2:1; / out2 tie off,
    pub /: *mut *mut u32 vroi_out1:1; / out1 tie off,
    pub /: *mut *mut u32 vroi_enable:1; / enable tie off,
    pub /: *mut *mut u32 codec_current_on:2; / current level ON,
    pub /: *mut *mut u32 codec_current_standby:2; / current level STANDBY,
    pub /: *mut *mut u32 codec_current_charge:2; / codec current @ vmid charge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_codec {
    pub pdev: *mut platform_device,
    pub platform_data: *mut wm8350_audio_platform_data,
}
