//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8983.h
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
// wm8983.h  --  WM8983 ALSA SoC Audio driver
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
//
// Register values.
//
pub const WM8983_SOFTWARE_RESET: c_uint = 0x00;
pub const WM8983_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const WM8983_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const WM8983_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const WM8983_AUDIO_INTERFACE: c_uint = 0x04;
pub const WM8983_COMPANDING_CONTROL: c_uint = 0x05;
pub const WM8983_CLOCK_GEN_CONTROL: c_uint = 0x06;
pub const WM8983_ADDITIONAL_CONTROL: c_uint = 0x07;
pub const WM8983_GPIO_CONTROL: c_uint = 0x08;
pub const WM8983_JACK_DETECT_CONTROL_1: c_uint = 0x09;
pub const WM8983_DAC_CONTROL: c_uint = 0x0A;
pub const WM8983_LEFT_DAC_DIGITAL_VOL: c_uint = 0x0B;
pub const WM8983_RIGHT_DAC_DIGITAL_VOL: c_uint = 0x0C;
pub const WM8983_JACK_DETECT_CONTROL_2: c_uint = 0x0D;
pub const WM8983_ADC_CONTROL: c_uint = 0x0E;
pub const WM8983_LEFT_ADC_DIGITAL_VOL: c_uint = 0x0F;
pub const WM8983_RIGHT_ADC_DIGITAL_VOL: c_uint = 0x10;
pub const WM8983_EQ1_LOW_SHELF: c_uint = 0x12;
pub const WM8983_EQ2_PEAK_1: c_uint = 0x13;
pub const WM8983_EQ3_PEAK_2: c_uint = 0x14;
pub const WM8983_EQ4_PEAK_3: c_uint = 0x15;
pub const WM8983_EQ5_HIGH_SHELF: c_uint = 0x16;
pub const WM8983_DAC_LIMITER_1: c_uint = 0x18;
pub const WM8983_DAC_LIMITER_2: c_uint = 0x19;
pub const WM8983_NOTCH_FILTER_1: c_uint = 0x1B;
pub const WM8983_NOTCH_FILTER_2: c_uint = 0x1C;
pub const WM8983_NOTCH_FILTER_3: c_uint = 0x1D;
pub const WM8983_NOTCH_FILTER_4: c_uint = 0x1E;
pub const WM8983_ALC_CONTROL_1: c_uint = 0x20;
pub const WM8983_ALC_CONTROL_2: c_uint = 0x21;
pub const WM8983_ALC_CONTROL_3: c_uint = 0x22;
pub const WM8983_NOISE_GATE: c_uint = 0x23;
pub const WM8983_PLL_N: c_uint = 0x24;
pub const WM8983_PLL_K_1: c_uint = 0x25;
pub const WM8983_PLL_K_2: c_uint = 0x26;
pub const WM8983_PLL_K_3: c_uint = 0x27;
pub const WM8983_3D_CONTROL: c_uint = 0x29;
pub const WM8983_OUT4_TO_ADC: c_uint = 0x2A;
pub const WM8983_BEEP_CONTROL: c_uint = 0x2B;
pub const WM8983_INPUT_CTRL: c_uint = 0x2C;
pub const WM8983_LEFT_INP_PGA_GAIN_CTRL: c_uint = 0x2D;
pub const WM8983_RIGHT_INP_PGA_GAIN_CTRL: c_uint = 0x2E;
pub const WM8983_LEFT_ADC_BOOST_CTRL: c_uint = 0x2F;
pub const WM8983_RIGHT_ADC_BOOST_CTRL: c_uint = 0x30;
pub const WM8983_OUTPUT_CTRL: c_uint = 0x31;
pub const WM8983_LEFT_MIXER_CTRL: c_uint = 0x32;
pub const WM8983_RIGHT_MIXER_CTRL: c_uint = 0x33;
pub const WM8983_LOUT1_HP_VOLUME_CTRL: c_uint = 0x34;
pub const WM8983_ROUT1_HP_VOLUME_CTRL: c_uint = 0x35;
pub const WM8983_LOUT2_SPK_VOLUME_CTRL: c_uint = 0x36;
pub const WM8983_ROUT2_SPK_VOLUME_CTRL: c_uint = 0x37;
pub const WM8983_OUT3_MIXER_CTRL: c_uint = 0x38;
pub const WM8983_OUT4_MONO_MIX_CTRL: c_uint = 0x39;
pub const WM8983_BIAS_CTRL: c_uint = 0x3D;
pub const WM8983_REGISTER_COUNT: c_int = 59;
pub const WM8983_MAX_REGISTER: c_uint = 0x3F;
//
// Field Definitions.
//
// R0 (0x00) - Software Reset
//
pub const WM8983_SOFTWARE_RESET_MASK: c_uint = 0x01FF  /* SOFTWARE_RESET - [8:0] */;

//
// R1 (0x01) - Power management 1
//
pub const WM8983_BUFDCOPEN: c_uint = 0x0100  /* BUFDCOPEN */;
pub const WM8983_BUFDCOPEN_MASK: c_uint = 0x0100  /* BUFDCOPEN */;

pub const WM8983_OUT4MIXEN: c_uint = 0x0080  /* OUT4MIXEN */;
pub const WM8983_OUT4MIXEN_MASK: c_uint = 0x0080  /* OUT4MIXEN */;

pub const WM8983_OUT3MIXEN: c_uint = 0x0040  /* OUT3MIXEN */;
pub const WM8983_OUT3MIXEN_MASK: c_uint = 0x0040  /* OUT3MIXEN */;

pub const WM8983_PLLEN: c_uint = 0x0020  /* PLLEN */;
pub const WM8983_PLLEN_MASK: c_uint = 0x0020  /* PLLEN */;

pub const WM8983_MICBEN: c_uint = 0x0010  /* MICBEN */;
pub const WM8983_MICBEN_MASK: c_uint = 0x0010  /* MICBEN */;

pub const WM8983_BIASEN: c_uint = 0x0008  /* BIASEN */;
pub const WM8983_BIASEN_MASK: c_uint = 0x0008  /* BIASEN */;

pub const WM8983_BUFIOEN: c_uint = 0x0004  /* BUFIOEN */;
pub const WM8983_BUFIOEN_MASK: c_uint = 0x0004  /* BUFIOEN */;

pub const WM8983_VMIDSEL_MASK: c_uint = 0x0003  /* VMIDSEL - [1:0] */;

//
// R2 (0x02) - Power management 2
//
pub const WM8983_ROUT1EN: c_uint = 0x0100  /* ROUT1EN */;
pub const WM8983_ROUT1EN_MASK: c_uint = 0x0100  /* ROUT1EN */;

pub const WM8983_LOUT1EN: c_uint = 0x0080  /* LOUT1EN */;
pub const WM8983_LOUT1EN_MASK: c_uint = 0x0080  /* LOUT1EN */;

pub const WM8983_SLEEP: c_uint = 0x0040  /* SLEEP */;
pub const WM8983_SLEEP_MASK: c_uint = 0x0040  /* SLEEP */;

pub const WM8983_BOOSTENR: c_uint = 0x0020  /* BOOSTENR */;
pub const WM8983_BOOSTENR_MASK: c_uint = 0x0020  /* BOOSTENR */;

pub const WM8983_BOOSTENL: c_uint = 0x0010  /* BOOSTENL */;
pub const WM8983_BOOSTENL_MASK: c_uint = 0x0010  /* BOOSTENL */;

pub const WM8983_INPGAENR: c_uint = 0x0008  /* INPGAENR */;
pub const WM8983_INPGAENR_MASK: c_uint = 0x0008  /* INPGAENR */;

pub const WM8983_INPPGAENL: c_uint = 0x0004  /* INPPGAENL */;
pub const WM8983_INPPGAENL_MASK: c_uint = 0x0004  /* INPPGAENL */;

pub const WM8983_ADCENR: c_uint = 0x0002  /* ADCENR */;
pub const WM8983_ADCENR_MASK: c_uint = 0x0002  /* ADCENR */;

pub const WM8983_ADCENL: c_uint = 0x0001  /* ADCENL */;
pub const WM8983_ADCENL_MASK: c_uint = 0x0001  /* ADCENL */;

//
// R3 (0x03) - Power management 3
//
pub const WM8983_OUT4EN: c_uint = 0x0100  /* OUT4EN */;
pub const WM8983_OUT4EN_MASK: c_uint = 0x0100  /* OUT4EN */;

pub const WM8983_OUT3EN: c_uint = 0x0080  /* OUT3EN */;
pub const WM8983_OUT3EN_MASK: c_uint = 0x0080  /* OUT3EN */;

pub const WM8983_LOUT2EN: c_uint = 0x0040  /* LOUT2EN */;
pub const WM8983_LOUT2EN_MASK: c_uint = 0x0040  /* LOUT2EN */;

pub const WM8983_ROUT2EN: c_uint = 0x0020  /* ROUT2EN */;
pub const WM8983_ROUT2EN_MASK: c_uint = 0x0020  /* ROUT2EN */;

pub const WM8983_RMIXEN: c_uint = 0x0008  /* RMIXEN */;
pub const WM8983_RMIXEN_MASK: c_uint = 0x0008  /* RMIXEN */;

pub const WM8983_LMIXEN: c_uint = 0x0004  /* LMIXEN */;
pub const WM8983_LMIXEN_MASK: c_uint = 0x0004  /* LMIXEN */;

pub const WM8983_DACENR: c_uint = 0x0002  /* DACENR */;
pub const WM8983_DACENR_MASK: c_uint = 0x0002  /* DACENR */;

pub const WM8983_DACENL: c_uint = 0x0001  /* DACENL */;
pub const WM8983_DACENL_MASK: c_uint = 0x0001  /* DACENL */;

//
// R4 (0x04) - Audio Interface
//
pub const WM8983_BCP: c_uint = 0x0100  /* BCP */;
pub const WM8983_BCP_MASK: c_uint = 0x0100  /* BCP */;

pub const WM8983_LRCP: c_uint = 0x0080  /* LRCP */;
pub const WM8983_LRCP_MASK: c_uint = 0x0080  /* LRCP */;

pub const WM8983_WL_MASK: c_uint = 0x0060  /* WL - [6:5] */;

pub const WM8983_FMT_MASK: c_uint = 0x0018  /* FMT - [4:3] */;

pub const WM8983_DLRSWAP: c_uint = 0x0004  /* DLRSWAP */;
pub const WM8983_DLRSWAP_MASK: c_uint = 0x0004  /* DLRSWAP */;

pub const WM8983_ALRSWAP: c_uint = 0x0002  /* ALRSWAP */;
pub const WM8983_ALRSWAP_MASK: c_uint = 0x0002  /* ALRSWAP */;

pub const WM8983_MONO: c_uint = 0x0001  /* MONO */;
pub const WM8983_MONO_MASK: c_uint = 0x0001  /* MONO */;

//
// R5 (0x05) - Companding control
//
pub const WM8983_WL8: c_uint = 0x0020  /* WL8 */;
pub const WM8983_WL8_MASK: c_uint = 0x0020  /* WL8 */;

pub const WM8983_DAC_COMP_MASK: c_uint = 0x0018  /* DAC_COMP - [4:3] */;

pub const WM8983_ADC_COMP_MASK: c_uint = 0x0006  /* ADC_COMP - [2:1] */;

pub const WM8983_LOOPBACK: c_uint = 0x0001  /* LOOPBACK */;
pub const WM8983_LOOPBACK_MASK: c_uint = 0x0001  /* LOOPBACK */;

//
// R6 (0x06) - Clock Gen control
//
pub const WM8983_CLKSEL: c_uint = 0x0100  /* CLKSEL */;
pub const WM8983_CLKSEL_MASK: c_uint = 0x0100  /* CLKSEL */;

pub const WM8983_MCLKDIV_MASK: c_uint = 0x00E0  /* MCLKDIV - [7:5] */;

pub const WM8983_BCLKDIV_MASK: c_uint = 0x001C  /* BCLKDIV - [4:2] */;

pub const WM8983_MS: c_uint = 0x0001  /* MS */;
pub const WM8983_MS_MASK: c_uint = 0x0001  /* MS */;

//
// R7 (0x07) - Additional control
//
pub const WM8983_SR_MASK: c_uint = 0x000E  /* SR - [3:1] */;

pub const WM8983_SLOWCLKEN: c_uint = 0x0001  /* SLOWCLKEN */;
pub const WM8983_SLOWCLKEN_MASK: c_uint = 0x0001  /* SLOWCLKEN */;

//
// R8 (0x08) - GPIO Control
//
pub const WM8983_OPCLKDIV_MASK: c_uint = 0x0030  /* OPCLKDIV - [5:4] */;

pub const WM8983_GPIO1POL: c_uint = 0x0008  /* GPIO1POL */;
pub const WM8983_GPIO1POL_MASK: c_uint = 0x0008  /* GPIO1POL */;

pub const WM8983_GPIO1SEL_MASK: c_uint = 0x0007  /* GPIO1SEL - [2:0] */;

//
// R9 (0x09) - Jack Detect Control 1
//
pub const WM8983_JD_VMID1: c_uint = 0x0100  /* JD_VMID1 */;
pub const WM8983_JD_VMID1_MASK: c_uint = 0x0100  /* JD_VMID1 */;

pub const WM8983_JD_VMID0: c_uint = 0x0080  /* JD_VMID0 */;
pub const WM8983_JD_VMID0_MASK: c_uint = 0x0080  /* JD_VMID0 */;

pub const WM8983_JD_EN: c_uint = 0x0040  /* JD_EN */;
pub const WM8983_JD_EN_MASK: c_uint = 0x0040  /* JD_EN */;

pub const WM8983_JD_SEL_MASK: c_uint = 0x0030  /* JD_SEL - [5:4] */;

//
// R10 (0x0A) - DAC Control
//
pub const WM8983_SOFTMUTE: c_uint = 0x0040  /* SOFTMUTE */;
pub const WM8983_SOFTMUTE_MASK: c_uint = 0x0040  /* SOFTMUTE */;

pub const WM8983_DACOSR128: c_uint = 0x0008  /* DACOSR128 */;
pub const WM8983_DACOSR128_MASK: c_uint = 0x0008  /* DACOSR128 */;

pub const WM8983_AMUTE: c_uint = 0x0004  /* AMUTE */;
pub const WM8983_AMUTE_MASK: c_uint = 0x0004  /* AMUTE */;

pub const WM8983_DACRPOL: c_uint = 0x0002  /* DACRPOL */;
pub const WM8983_DACRPOL_MASK: c_uint = 0x0002  /* DACRPOL */;

pub const WM8983_DACLPOL: c_uint = 0x0001  /* DACLPOL */;
pub const WM8983_DACLPOL_MASK: c_uint = 0x0001  /* DACLPOL */;

//
// R11 (0x0B) - Left DAC digital Vol
//
pub const WM8983_DACVU: c_uint = 0x0100  /* DACVU */;
pub const WM8983_DACVU_MASK: c_uint = 0x0100  /* DACVU */;

pub const WM8983_DACLVOL_MASK: c_uint = 0x00FF  /* DACLVOL - [7:0] */;

//
// R12 (0x0C) - Right DAC digital vol
//
pub const WM8983_DACVU: c_uint = 0x0100  /* DACVU */;
pub const WM8983_DACVU_MASK: c_uint = 0x0100  /* DACVU */;

pub const WM8983_DACRVOL_MASK: c_uint = 0x00FF  /* DACRVOL - [7:0] */;

//
// R13 (0x0D) - Jack Detect Control 2
//
pub const WM8983_JD_EN1_MASK: c_uint = 0x00F0  /* JD_EN1 - [7:4] */;

pub const WM8983_JD_EN0_MASK: c_uint = 0x000F  /* JD_EN0 - [3:0] */;

//
// R14 (0x0E) - ADC Control
//
pub const WM8983_HPFEN: c_uint = 0x0100  /* HPFEN */;
pub const WM8983_HPFEN_MASK: c_uint = 0x0100  /* HPFEN */;

pub const WM8983_HPFAPP: c_uint = 0x0080  /* HPFAPP */;
pub const WM8983_HPFAPP_MASK: c_uint = 0x0080  /* HPFAPP */;

pub const WM8983_HPFCUT_MASK: c_uint = 0x0070  /* HPFCUT - [6:4] */;

pub const WM8983_ADCOSR128: c_uint = 0x0008  /* ADCOSR128 */;
pub const WM8983_ADCOSR128_MASK: c_uint = 0x0008  /* ADCOSR128 */;

pub const WM8983_ADCRPOL: c_uint = 0x0002  /* ADCRPOL */;
pub const WM8983_ADCRPOL_MASK: c_uint = 0x0002  /* ADCRPOL */;

pub const WM8983_ADCLPOL: c_uint = 0x0001  /* ADCLPOL */;
pub const WM8983_ADCLPOL_MASK: c_uint = 0x0001  /* ADCLPOL */;

//
// R15 (0x0F) - Left ADC Digital Vol
//
pub const WM8983_ADCVU: c_uint = 0x0100  /* ADCVU */;
pub const WM8983_ADCVU_MASK: c_uint = 0x0100  /* ADCVU */;

pub const WM8983_ADCLVOL_MASK: c_uint = 0x00FF  /* ADCLVOL - [7:0] */;

//
// R16 (0x10) - Right ADC Digital Vol
//
pub const WM8983_ADCVU: c_uint = 0x0100  /* ADCVU */;
pub const WM8983_ADCVU_MASK: c_uint = 0x0100  /* ADCVU */;

pub const WM8983_ADCRVOL_MASK: c_uint = 0x00FF  /* ADCRVOL - [7:0] */;

//
// R18 (0x12) - EQ1 - low shelf
//
pub const WM8983_EQ3DMODE: c_uint = 0x0100  /* EQ3DMODE */;
pub const WM8983_EQ3DMODE_MASK: c_uint = 0x0100  /* EQ3DMODE */;

pub const WM8983_EQ1C_MASK: c_uint = 0x0060  /* EQ1C - [6:5] */;

pub const WM8983_EQ1G_MASK: c_uint = 0x001F  /* EQ1G - [4:0] */;

//
// R19 (0x13) - EQ2 - peak 1
//
pub const WM8983_EQ2BW: c_uint = 0x0100  /* EQ2BW */;
pub const WM8983_EQ2BW_MASK: c_uint = 0x0100  /* EQ2BW */;

pub const WM8983_EQ2C_MASK: c_uint = 0x0060  /* EQ2C - [6:5] */;

pub const WM8983_EQ2G_MASK: c_uint = 0x001F  /* EQ2G - [4:0] */;

//
// R20 (0x14) - EQ3 - peak 2
//
pub const WM8983_EQ3BW: c_uint = 0x0100  /* EQ3BW */;
pub const WM8983_EQ3BW_MASK: c_uint = 0x0100  /* EQ3BW */;

pub const WM8983_EQ3C_MASK: c_uint = 0x0060  /* EQ3C - [6:5] */;

pub const WM8983_EQ3G_MASK: c_uint = 0x001F  /* EQ3G - [4:0] */;

//
// R21 (0x15) - EQ4 - peak 3
//
pub const WM8983_EQ4BW: c_uint = 0x0100  /* EQ4BW */;
pub const WM8983_EQ4BW_MASK: c_uint = 0x0100  /* EQ4BW */;

pub const WM8983_EQ4C_MASK: c_uint = 0x0060  /* EQ4C - [6:5] */;

pub const WM8983_EQ4G_MASK: c_uint = 0x001F  /* EQ4G - [4:0] */;

//
// R22 (0x16) - EQ5 - high shelf
//
pub const WM8983_EQ5C_MASK: c_uint = 0x0060  /* EQ5C - [6:5] */;

pub const WM8983_EQ5G_MASK: c_uint = 0x001F  /* EQ5G - [4:0] */;

//
// R24 (0x18) - DAC Limiter 1
//
pub const WM8983_LIMEN: c_uint = 0x0100  /* LIMEN */;
pub const WM8983_LIMEN_MASK: c_uint = 0x0100  /* LIMEN */;

pub const WM8983_LIMDCY_MASK: c_uint = 0x00F0  /* LIMDCY - [7:4] */;

pub const WM8983_LIMATK_MASK: c_uint = 0x000F  /* LIMATK - [3:0] */;

//
// R25 (0x19) - DAC Limiter 2
//
pub const WM8983_LIMLVL_MASK: c_uint = 0x0070  /* LIMLVL - [6:4] */;

pub const WM8983_LIMBOOST_MASK: c_uint = 0x000F  /* LIMBOOST - [3:0] */;

//
// R27 (0x1B) - Notch Filter 1
//
pub const WM8983_NFU: c_uint = 0x0100  /* NFU */;
pub const WM8983_NFU_MASK: c_uint = 0x0100  /* NFU */;

pub const WM8983_NFEN: c_uint = 0x0080  /* NFEN */;
pub const WM8983_NFEN_MASK: c_uint = 0x0080  /* NFEN */;

pub const WM8983_NFA0_13_7_MASK: c_uint = 0x007F  /* NFA0(13:7) - [6:0] */;

//
// R28 (0x1C) - Notch Filter 2
//
pub const WM8983_NFU: c_uint = 0x0100  /* NFU */;
pub const WM8983_NFU_MASK: c_uint = 0x0100  /* NFU */;

pub const WM8983_NFA0_6_0_MASK: c_uint = 0x007F  /* NFA0(6:0) - [6:0] */;

//
// R29 (0x1D) - Notch Filter 3
//
pub const WM8983_NFU: c_uint = 0x0100  /* NFU */;
pub const WM8983_NFU_MASK: c_uint = 0x0100  /* NFU */;

pub const WM8983_NFA1_13_7_MASK: c_uint = 0x007F  /* NFA1(13:7) - [6:0] */;

//
// R30 (0x1E) - Notch Filter 4
//
pub const WM8983_NFU: c_uint = 0x0100  /* NFU */;
pub const WM8983_NFU_MASK: c_uint = 0x0100  /* NFU */;

pub const WM8983_NFA1_6_0_MASK: c_uint = 0x007F  /* NFA1(6:0) - [6:0] */;

//
// R32 (0x20) - ALC control 1
//
pub const WM8983_ALCSEL_MASK: c_uint = 0x0180  /* ALCSEL - [8:7] */;

pub const WM8983_ALCMAX_MASK: c_uint = 0x0038  /* ALCMAX - [5:3] */;

pub const WM8983_ALCMIN_MASK: c_uint = 0x0007  /* ALCMIN - [2:0] */;

//
// R33 (0x21) - ALC control 2
//
pub const WM8983_ALCHLD_MASK: c_uint = 0x00F0  /* ALCHLD - [7:4] */;

pub const WM8983_ALCLVL_MASK: c_uint = 0x000F  /* ALCLVL - [3:0] */;

//
// R34 (0x22) - ALC control 3
//
pub const WM8983_ALCMODE: c_uint = 0x0100  /* ALCMODE */;
pub const WM8983_ALCMODE_MASK: c_uint = 0x0100  /* ALCMODE */;

pub const WM8983_ALCDCY_MASK: c_uint = 0x00F0  /* ALCDCY - [7:4] */;

pub const WM8983_ALCATK_MASK: c_uint = 0x000F  /* ALCATK - [3:0] */;

//
// R35 (0x23) - Noise Gate
//
pub const WM8983_NGEN: c_uint = 0x0008  /* NGEN */;
pub const WM8983_NGEN_MASK: c_uint = 0x0008  /* NGEN */;

pub const WM8983_NGTH_MASK: c_uint = 0x0007  /* NGTH - [2:0] */;

//
// R36 (0x24) - PLL N
//
pub const WM8983_PLL_PRESCALE: c_uint = 0x0010  /* PLL_PRESCALE */;
pub const WM8983_PLL_PRESCALE_MASK: c_uint = 0x0010  /* PLL_PRESCALE */;

pub const WM8983_PLLN_MASK: c_uint = 0x000F  /* PLLN - [3:0] */;

//
// R37 (0x25) - PLL K 1
//
pub const WM8983_PLLK_23_18_MASK: c_uint = 0x003F  /* PLLK(23:18) - [5:0] */;

//
// R38 (0x26) - PLL K 2
//
pub const WM8983_PLLK_17_9_MASK: c_uint = 0x01FF  /* PLLK(17:9) - [8:0] */;

//
// R39 (0x27) - PLL K 3
//
pub const WM8983_PLLK_8_0_MASK: c_uint = 0x01FF  /* PLLK(8:0) - [8:0] */;

//
// R41 (0x29) - 3D control
//
pub const WM8983_DEPTH3D_MASK: c_uint = 0x000F  /* DEPTH3D - [3:0] */;

//
// R42 (0x2A) - OUT4 to ADC
//
pub const WM8983_OUT4_2ADCVOL_MASK: c_uint = 0x01C0  /* OUT4_2ADCVOL - [8:6] */;

pub const WM8983_OUT4_2LNR: c_uint = 0x0020  /* OUT4_2LNR */;
pub const WM8983_OUT4_2LNR_MASK: c_uint = 0x0020  /* OUT4_2LNR */;

pub const WM8983_POBCTRL: c_uint = 0x0004  /* POBCTRL */;
pub const WM8983_POBCTRL_MASK: c_uint = 0x0004  /* POBCTRL */;

pub const WM8983_DELEN: c_uint = 0x0002  /* DELEN */;
pub const WM8983_DELEN_MASK: c_uint = 0x0002  /* DELEN */;

pub const WM8983_OUT1DEL: c_uint = 0x0001  /* OUT1DEL */;
pub const WM8983_OUT1DEL_MASK: c_uint = 0x0001  /* OUT1DEL */;

//
// R43 (0x2B) - Beep control
//
pub const WM8983_BYPL2RMIX: c_uint = 0x0100  /* BYPL2RMIX */;
pub const WM8983_BYPL2RMIX_MASK: c_uint = 0x0100  /* BYPL2RMIX */;

pub const WM8983_BYPR2LMIX: c_uint = 0x0080  /* BYPR2LMIX */;
pub const WM8983_BYPR2LMIX_MASK: c_uint = 0x0080  /* BYPR2LMIX */;

pub const WM8983_MUTERPGA2INV: c_uint = 0x0020  /* MUTERPGA2INV */;
pub const WM8983_MUTERPGA2INV_MASK: c_uint = 0x0020  /* MUTERPGA2INV */;

pub const WM8983_INVROUT2: c_uint = 0x0010  /* INVROUT2 */;
pub const WM8983_INVROUT2_MASK: c_uint = 0x0010  /* INVROUT2 */;

pub const WM8983_BEEPVOL_MASK: c_uint = 0x000E  /* BEEPVOL - [3:1] */;

pub const WM8983_BEEPEN: c_uint = 0x0001  /* BEEPEN */;
pub const WM8983_BEEPEN_MASK: c_uint = 0x0001  /* BEEPEN */;

//
// R44 (0x2C) - Input ctrl
//
pub const WM8983_MBVSEL: c_uint = 0x0100  /* MBVSEL */;
pub const WM8983_MBVSEL_MASK: c_uint = 0x0100  /* MBVSEL */;

pub const WM8983_R2_2INPPGA: c_uint = 0x0040  /* R2_2INPPGA */;
pub const WM8983_R2_2INPPGA_MASK: c_uint = 0x0040  /* R2_2INPPGA */;

pub const WM8983_RIN2INPPGA: c_uint = 0x0020  /* RIN2INPPGA */;
pub const WM8983_RIN2INPPGA_MASK: c_uint = 0x0020  /* RIN2INPPGA */;

pub const WM8983_RIP2INPPGA: c_uint = 0x0010  /* RIP2INPPGA */;
pub const WM8983_RIP2INPPGA_MASK: c_uint = 0x0010  /* RIP2INPPGA */;

pub const WM8983_L2_2INPPGA: c_uint = 0x0004  /* L2_2INPPGA */;
pub const WM8983_L2_2INPPGA_MASK: c_uint = 0x0004  /* L2_2INPPGA */;

pub const WM8983_LIN2INPPGA: c_uint = 0x0002  /* LIN2INPPGA */;
pub const WM8983_LIN2INPPGA_MASK: c_uint = 0x0002  /* LIN2INPPGA */;

pub const WM8983_LIP2INPPGA: c_uint = 0x0001  /* LIP2INPPGA */;
pub const WM8983_LIP2INPPGA_MASK: c_uint = 0x0001  /* LIP2INPPGA */;

//
// R45 (0x2D) - Left INP PGA gain ctrl
//
pub const WM8983_INPGAVU: c_uint = 0x0100  /* INPGAVU */;
pub const WM8983_INPGAVU_MASK: c_uint = 0x0100  /* INPGAVU */;

pub const WM8983_INPPGAZCL: c_uint = 0x0080  /* INPPGAZCL */;
pub const WM8983_INPPGAZCL_MASK: c_uint = 0x0080  /* INPPGAZCL */;

pub const WM8983_INPPGAMUTEL: c_uint = 0x0040  /* INPPGAMUTEL */;
pub const WM8983_INPPGAMUTEL_MASK: c_uint = 0x0040  /* INPPGAMUTEL */;

pub const WM8983_INPPGAVOLL_MASK: c_uint = 0x003F  /* INPPGAVOLL - [5:0] */;

//
// R46 (0x2E) - Right INP PGA gain ctrl
//
pub const WM8983_INPGAVU: c_uint = 0x0100  /* INPGAVU */;
pub const WM8983_INPGAVU_MASK: c_uint = 0x0100  /* INPGAVU */;

pub const WM8983_INPPGAZCR: c_uint = 0x0080  /* INPPGAZCR */;
pub const WM8983_INPPGAZCR_MASK: c_uint = 0x0080  /* INPPGAZCR */;

pub const WM8983_INPPGAMUTER: c_uint = 0x0040  /* INPPGAMUTER */;
pub const WM8983_INPPGAMUTER_MASK: c_uint = 0x0040  /* INPPGAMUTER */;

pub const WM8983_INPPGAVOLR_MASK: c_uint = 0x003F  /* INPPGAVOLR - [5:0] */;

//
// R47 (0x2F) - Left ADC BOOST ctrl
//
pub const WM8983_PGABOOSTL: c_uint = 0x0100  /* PGABOOSTL */;
pub const WM8983_PGABOOSTL_MASK: c_uint = 0x0100  /* PGABOOSTL */;

pub const WM8983_L2_2BOOSTVOL_MASK: c_uint = 0x0070  /* L2_2BOOSTVOL - [6:4] */;

pub const WM8983_AUXL2BOOSTVOL_MASK: c_uint = 0x0007  /* AUXL2BOOSTVOL - [2:0] */;

//
// R48 (0x30) - Right ADC BOOST ctrl
//
pub const WM8983_PGABOOSTR: c_uint = 0x0100  /* PGABOOSTR */;
pub const WM8983_PGABOOSTR_MASK: c_uint = 0x0100  /* PGABOOSTR */;

pub const WM8983_R2_2BOOSTVOL_MASK: c_uint = 0x0070  /* R2_2BOOSTVOL - [6:4] */;

pub const WM8983_AUXR2BOOSTVOL_MASK: c_uint = 0x0007  /* AUXR2BOOSTVOL - [2:0] */;

//
// R49 (0x31) - Output ctrl
//
pub const WM8983_DACL2RMIX: c_uint = 0x0040  /* DACL2RMIX */;
pub const WM8983_DACL2RMIX_MASK: c_uint = 0x0040  /* DACL2RMIX */;

pub const WM8983_DACR2LMIX: c_uint = 0x0020  /* DACR2LMIX */;
pub const WM8983_DACR2LMIX_MASK: c_uint = 0x0020  /* DACR2LMIX */;

pub const WM8983_OUT4BOOST: c_uint = 0x0010  /* OUT4BOOST */;
pub const WM8983_OUT4BOOST_MASK: c_uint = 0x0010  /* OUT4BOOST */;

pub const WM8983_OUT3BOOST: c_uint = 0x0008  /* OUT3BOOST */;
pub const WM8983_OUT3BOOST_MASK: c_uint = 0x0008  /* OUT3BOOST */;

pub const WM8983_SPKBOOST: c_uint = 0x0004  /* SPKBOOST */;
pub const WM8983_SPKBOOST_MASK: c_uint = 0x0004  /* SPKBOOST */;

pub const WM8983_TSDEN: c_uint = 0x0002  /* TSDEN */;
pub const WM8983_TSDEN_MASK: c_uint = 0x0002  /* TSDEN */;

pub const WM8983_VROI: c_uint = 0x0001  /* VROI */;
pub const WM8983_VROI_MASK: c_uint = 0x0001  /* VROI */;

//
// R50 (0x32) - Left mixer ctrl
//
pub const WM8983_AUXLMIXVOL_MASK: c_uint = 0x01C0  /* AUXLMIXVOL - [8:6] */;

pub const WM8983_AUXL2LMIX: c_uint = 0x0020  /* AUXL2LMIX */;
pub const WM8983_AUXL2LMIX_MASK: c_uint = 0x0020  /* AUXL2LMIX */;

pub const WM8983_BYPLMIXVOL_MASK: c_uint = 0x001C  /* BYPLMIXVOL - [4:2] */;

pub const WM8983_BYPL2LMIX: c_uint = 0x0002  /* BYPL2LMIX */;
pub const WM8983_BYPL2LMIX_MASK: c_uint = 0x0002  /* BYPL2LMIX */;

pub const WM8983_DACL2LMIX: c_uint = 0x0001  /* DACL2LMIX */;
pub const WM8983_DACL2LMIX_MASK: c_uint = 0x0001  /* DACL2LMIX */;

//
// R51 (0x33) - Right mixer ctrl
//
pub const WM8983_AUXRMIXVOL_MASK: c_uint = 0x01C0  /* AUXRMIXVOL - [8:6] */;

pub const WM8983_AUXR2RMIX: c_uint = 0x0020  /* AUXR2RMIX */;
pub const WM8983_AUXR2RMIX_MASK: c_uint = 0x0020  /* AUXR2RMIX */;

pub const WM8983_BYPRMIXVOL_MASK: c_uint = 0x001C  /* BYPRMIXVOL - [4:2] */;

pub const WM8983_BYPR2RMIX: c_uint = 0x0002  /* BYPR2RMIX */;
pub const WM8983_BYPR2RMIX_MASK: c_uint = 0x0002  /* BYPR2RMIX */;

pub const WM8983_DACR2RMIX: c_uint = 0x0001  /* DACR2RMIX */;
pub const WM8983_DACR2RMIX_MASK: c_uint = 0x0001  /* DACR2RMIX */;

//
// R52 (0x34) - LOUT1 (HP) volume ctrl
//
pub const WM8983_OUT1VU: c_uint = 0x0100  /* OUT1VU */;
pub const WM8983_OUT1VU_MASK: c_uint = 0x0100  /* OUT1VU */;

pub const WM8983_LOUT1ZC: c_uint = 0x0080  /* LOUT1ZC */;
pub const WM8983_LOUT1ZC_MASK: c_uint = 0x0080  /* LOUT1ZC */;

pub const WM8983_LOUT1MUTE: c_uint = 0x0040  /* LOUT1MUTE */;
pub const WM8983_LOUT1MUTE_MASK: c_uint = 0x0040  /* LOUT1MUTE */;

pub const WM8983_LOUT1VOL_MASK: c_uint = 0x003F  /* LOUT1VOL - [5:0] */;

//
// R53 (0x35) - ROUT1 (HP) volume ctrl
//
pub const WM8983_OUT1VU: c_uint = 0x0100  /* OUT1VU */;
pub const WM8983_OUT1VU_MASK: c_uint = 0x0100  /* OUT1VU */;

pub const WM8983_ROUT1ZC: c_uint = 0x0080  /* ROUT1ZC */;
pub const WM8983_ROUT1ZC_MASK: c_uint = 0x0080  /* ROUT1ZC */;

pub const WM8983_ROUT1MUTE: c_uint = 0x0040  /* ROUT1MUTE */;
pub const WM8983_ROUT1MUTE_MASK: c_uint = 0x0040  /* ROUT1MUTE */;

pub const WM8983_ROUT1VOL_MASK: c_uint = 0x003F  /* ROUT1VOL - [5:0] */;

//
// R54 (0x36) - LOUT2 (SPK) volume ctrl
//
pub const WM8983_OUT2VU: c_uint = 0x0100  /* OUT2VU */;
pub const WM8983_OUT2VU_MASK: c_uint = 0x0100  /* OUT2VU */;

pub const WM8983_LOUT2ZC: c_uint = 0x0080  /* LOUT2ZC */;
pub const WM8983_LOUT2ZC_MASK: c_uint = 0x0080  /* LOUT2ZC */;

pub const WM8983_LOUT2MUTE: c_uint = 0x0040  /* LOUT2MUTE */;
pub const WM8983_LOUT2MUTE_MASK: c_uint = 0x0040  /* LOUT2MUTE */;

pub const WM8983_LOUT2VOL_MASK: c_uint = 0x003F  /* LOUT2VOL - [5:0] */;

//
// R55 (0x37) - ROUT2 (SPK) volume ctrl
//
pub const WM8983_OUT2VU: c_uint = 0x0100  /* OUT2VU */;
pub const WM8983_OUT2VU_MASK: c_uint = 0x0100  /* OUT2VU */;

pub const WM8983_ROUT2ZC: c_uint = 0x0080  /* ROUT2ZC */;
pub const WM8983_ROUT2ZC_MASK: c_uint = 0x0080  /* ROUT2ZC */;

pub const WM8983_ROUT2MUTE: c_uint = 0x0040  /* ROUT2MUTE */;
pub const WM8983_ROUT2MUTE_MASK: c_uint = 0x0040  /* ROUT2MUTE */;

pub const WM8983_ROUT2VOL_MASK: c_uint = 0x003F  /* ROUT2VOL - [5:0] */;

//
// R56 (0x38) - OUT3 mixer ctrl
//
pub const WM8983_OUT3MUTE: c_uint = 0x0040  /* OUT3MUTE */;
pub const WM8983_OUT3MUTE_MASK: c_uint = 0x0040  /* OUT3MUTE */;

pub const WM8983_OUT4_2OUT3: c_uint = 0x0008  /* OUT4_2OUT3 */;
pub const WM8983_OUT4_2OUT3_MASK: c_uint = 0x0008  /* OUT4_2OUT3 */;

pub const WM8983_BYPL2OUT3: c_uint = 0x0004  /* BYPL2OUT3 */;
pub const WM8983_BYPL2OUT3_MASK: c_uint = 0x0004  /* BYPL2OUT3 */;

pub const WM8983_LMIX2OUT3: c_uint = 0x0002  /* LMIX2OUT3 */;
pub const WM8983_LMIX2OUT3_MASK: c_uint = 0x0002  /* LMIX2OUT3 */;

pub const WM8983_LDAC2OUT3: c_uint = 0x0001  /* LDAC2OUT3 */;
pub const WM8983_LDAC2OUT3_MASK: c_uint = 0x0001  /* LDAC2OUT3 */;

//
// R57 (0x39) - OUT4 (MONO) mix ctrl
//
pub const WM8983_OUT3_2OUT4: c_uint = 0x0080  /* OUT3_2OUT4 */;
pub const WM8983_OUT3_2OUT4_MASK: c_uint = 0x0080  /* OUT3_2OUT4 */;

pub const WM8983_OUT4MUTE: c_uint = 0x0040  /* OUT4MUTE */;
pub const WM8983_OUT4MUTE_MASK: c_uint = 0x0040  /* OUT4MUTE */;

pub const WM8983_OUT4ATTN: c_uint = 0x0020  /* OUT4ATTN */;
pub const WM8983_OUT4ATTN_MASK: c_uint = 0x0020  /* OUT4ATTN */;

pub const WM8983_LMIX2OUT4: c_uint = 0x0010  /* LMIX2OUT4 */;
pub const WM8983_LMIX2OUT4_MASK: c_uint = 0x0010  /* LMIX2OUT4 */;

pub const WM8983_LDAC2OUT4: c_uint = 0x0008  /* LDAC2OUT4 */;
pub const WM8983_LDAC2OUT4_MASK: c_uint = 0x0008  /* LDAC2OUT4 */;

pub const WM8983_BYPR2OUT4: c_uint = 0x0004  /* BYPR2OUT4 */;
pub const WM8983_BYPR2OUT4_MASK: c_uint = 0x0004  /* BYPR2OUT4 */;

pub const WM8983_RMIX2OUT4: c_uint = 0x0002  /* RMIX2OUT4 */;
pub const WM8983_RMIX2OUT4_MASK: c_uint = 0x0002  /* RMIX2OUT4 */;

pub const WM8983_RDAC2OUT4: c_uint = 0x0001  /* RDAC2OUT4 */;
pub const WM8983_RDAC2OUT4_MASK: c_uint = 0x0001  /* RDAC2OUT4 */;

//
// R61 (0x3D) - BIAS CTRL
//
pub const WM8983_BIASCUT: c_uint = 0x0100  /* BIASCUT */;
pub const WM8983_BIASCUT_MASK: c_uint = 0x0100  /* BIASCUT */;

pub const WM8983_HALFIPBIAS: c_uint = 0x0080  /* HALFIPBIAS */;
pub const WM8983_HALFIPBIAS_MASK: c_uint = 0x0080  /* HALFIPBIAS */;

pub const WM8983_VBBIASTST_MASK: c_uint = 0x0060  /* VBBIASTST - [6:5] */;

pub const WM8983_BUFBIAS_MASK: c_uint = 0x0018  /* BUFBIAS - [4:3] */;

pub const WM8983_ADCBIAS_MASK: c_uint = 0x0006  /* ADCBIAS - [2:1] */;

pub const WM8983_HALFOPBIAS: c_uint = 0x0001  /* HALFOPBIAS */;
pub const WM8983_HALFOPBIAS_MASK: c_uint = 0x0001  /* HALFOPBIAS */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_src {
    WM8983_CLKSRC_MCLK,
    WM8983_CLKSRC_PLL
}
