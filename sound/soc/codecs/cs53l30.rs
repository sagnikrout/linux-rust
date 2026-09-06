//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs53l30.h
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
// ALSA SoC CS53L30 codec driver
//
// Copyright 2015 Cirrus Logic, Inc.
//
// Author: Paul Handrigan <Paul.Handrigan@cirrus.com>,
// Tim Howe <Tim.Howe@cirrus.com>
//
// I2C Registers
pub const CS53L30_DEVID_AB: c_uint = 0x01	 /* Device ID A & B [RO]. */;
pub const CS53L30_DEVID_CD: c_uint = 0x02     /* Device ID C & D [RO]. */;
pub const CS53L30_DEVID_E: c_uint = 0x03     /* Device ID E [RO]. */;
pub const CS53L30_REVID: c_uint = 0x05     /* Revision ID [RO]. */;
pub const CS53L30_PWRCTL: c_uint = 0x06     /* Power Control. */;
pub const CS53L30_MCLKCTL: c_uint = 0x07     /* MCLK Control. */;
pub const CS53L30_INT_SR_CTL: c_uint = 0x08     /* Internal Sample Rate Control. */;
pub const CS53L30_MICBIAS_CTL: c_uint = 0x0A     /* Mic Bias Control. */;
pub const CS53L30_ASPCFG_CTL: c_uint = 0x0C     /* ASP Config Control. */;
pub const CS53L30_ASP_CTL1: c_uint = 0x0D     /* ASP1 Control. */;
pub const CS53L30_ASP_TDMTX_CTL1: c_uint = 0x0E     /* ASP1 TDM TX Control 1 */;
pub const CS53L30_ASP_TDMTX_CTL2: c_uint = 0x0F     /* ASP1 TDM TX Control 2 */;
pub const CS53L30_ASP_TDMTX_CTL3: c_uint = 0x10     /* ASP1 TDM TX Control 3 */;
pub const CS53L30_ASP_TDMTX_CTL4: c_uint = 0x11     /* ASP1 TDM TX Control 4 */;
pub const CS53L30_ASP_TDMTX_EN1: c_uint = 0x12     /* ASP1 TDM TX Enable 1 */;
pub const CS53L30_ASP_TDMTX_EN2: c_uint = 0x13     /* ASP1 TDM TX Enable 2 */;
pub const CS53L30_ASP_TDMTX_EN3: c_uint = 0x14     /* ASP1 TDM TX Enable 3 */;
pub const CS53L30_ASP_TDMTX_EN4: c_uint = 0x15     /* ASP1 TDM TX Enable 4 */;
pub const CS53L30_ASP_TDMTX_EN5: c_uint = 0x16     /* ASP1 TDM TX Enable 5 */;
pub const CS53L30_ASP_TDMTX_EN6: c_uint = 0x17     /* ASP1 TDM TX Enable 6 */;
pub const CS53L30_ASP_CTL2: c_uint = 0x18     /* ASP2 Control. */;
pub const CS53L30_SFT_RAMP: c_uint = 0x1A     /* Soft Ramp Control. */;
pub const CS53L30_LRCK_CTL1: c_uint = 0x1B     /* LRCK Control 1. */;
pub const CS53L30_LRCK_CTL2: c_uint = 0x1C     /* LRCK Control 2. */;
pub const CS53L30_MUTEP_CTL1: c_uint = 0x1F     /* Mute Pin Control 1. */;
pub const CS53L30_MUTEP_CTL2: c_uint = 0x20     /* Mute Pin Control 2. */;
pub const CS53L30_INBIAS_CTL1: c_uint = 0x21     /* Input Bias Control 1. */;
pub const CS53L30_INBIAS_CTL2: c_uint = 0x22     /* Input Bias Control 2. */;
pub const CS53L30_DMIC1_STR_CTL: c_uint = 0x23     /* DMIC1 Stereo Control. */;
pub const CS53L30_DMIC2_STR_CTL: c_uint = 0x24     /* DMIC2 Stereo Control. */;
pub const CS53L30_ADCDMIC1_CTL1: c_uint = 0x25     /* ADC1/DMIC1 Control 1. */;
pub const CS53L30_ADCDMIC1_CTL2: c_uint = 0x26     /* ADC1/DMIC1 Control 2. */;
pub const CS53L30_ADC1_CTL3: c_uint = 0x27     /* ADC1 Control 3. */;
pub const CS53L30_ADC1_NG_CTL: c_uint = 0x28     /* ADC1 Noise Gate Control. */;
pub const CS53L30_ADC1A_AFE_CTL: c_uint = 0x29     /* ADC1A AFE Control. */;
pub const CS53L30_ADC1B_AFE_CTL: c_uint = 0x2A     /* ADC1B AFE Control. */;
pub const CS53L30_ADC1A_DIG_VOL: c_uint = 0x2B     /* ADC1A Digital Volume. */;
pub const CS53L30_ADC1B_DIG_VOL: c_uint = 0x2C     /* ADC1B Digital Volume. */;
pub const CS53L30_ADCDMIC2_CTL1: c_uint = 0x2D     /* ADC2/DMIC2 Control 1. */;
pub const CS53L30_ADCDMIC2_CTL2: c_uint = 0x2E     /* ADC2/DMIC2 Control 2. */;
pub const CS53L30_ADC2_CTL3: c_uint = 0x2F     /* ADC2 Control 3. */;
pub const CS53L30_ADC2_NG_CTL: c_uint = 0x30     /* ADC2 Noise Gate Control. */;
pub const CS53L30_ADC2A_AFE_CTL: c_uint = 0x31     /* ADC2A AFE Control. */;
pub const CS53L30_ADC2B_AFE_CTL: c_uint = 0x32     /* ADC2B AFE Control. */;
pub const CS53L30_ADC2A_DIG_VOL: c_uint = 0x33     /* ADC2A Digital Volume. */;
pub const CS53L30_ADC2B_DIG_VOL: c_uint = 0x34     /* ADC2B Digital Volume. */;
pub const CS53L30_INT_MASK: c_uint = 0x35     /* Interrupt Mask. */;
pub const CS53L30_IS: c_uint = 0x36     /* Interrupt Status. */;
pub const CS53L30_MAX_REGISTER: c_uint = 0x36;
pub const CS53L30_TDM_SLOT_MAX: c_int = 4;

// x : index for registers; n : index for slot; 8 slots per register

pub const CS53L30_ASP_TDMTX_ENx_MAX: c_int = 6;
// Device ID
pub const CS53L30_DEVID: c_uint = 0x53A30;
// PDN_DONE Poll Maximum
// If soft ramp is set it will take much longer to power down
// the system.
//
pub const CS53L30_PDN_POLL_MAX: c_int = 90;
// Bitfield Definitions
// R6 (0x06) CS53L30_PWRCTL - Power Control
pub const CS53L30_PDN_ULP_SHIFT: c_int = 7;

pub const CS53L30_PDN_LP_SHIFT: c_int = 6;

pub const CS53L30_DISCHARGE_FILT_SHIFT: c_int = 5;

pub const CS53L30_THMS_PDN_SHIFT: c_int = 4;

// R7 (0x07) CS53L30_MCLKCTL - MCLK Control
pub const CS53L30_MCLK_DIS_SHIFT: c_int = 7;

pub const CS53L30_MCLK_INT_SCALE_SHIFT: c_int = 6;

pub const CS53L30_DMIC_DRIVE_SHIFT: c_int = 5;

pub const CS53L30_MCLK_DIV_SHIFT: c_int = 2;
pub const CS53L30_MCLK_DIV_WIDTH: c_int = 2;

pub const CS53L30_SYNC_EN_SHIFT: c_int = 1;

// R8 (0x08) CS53L30_INT_SR_CTL - Internal Sample Rate Control
pub const CS53L30_INTRNL_FS_RATIO_SHIFT: c_int = 4;

pub const CS53L30_MCLK_19MHZ_EN_SHIFT: c_int = 0;

// 0x6 << 1 is reserved bits

// R10 (0x0A) CS53L30_MICBIAS_CTL - Mic Bias Control
pub const CS53L30_MIC4_BIAS_PDN_SHIFT: c_int = 7;

pub const CS53L30_MIC3_BIAS_PDN_SHIFT: c_int = 6;

pub const CS53L30_MIC2_BIAS_PDN_SHIFT: c_int = 5;

pub const CS53L30_MIC1_BIAS_PDN_SHIFT: c_int = 4;

pub const CS53L30_VP_MIN_SHIFT: c_int = 2;

pub const CS53L30_MIC_BIAS_CTRL_SHIFT: c_int = 0;
pub const CS53L30_MIC_BIAS_CTRL_WIDTH: c_int = 2;

// R12 (0x0C) CS53L30_ASPCFG_CTL - ASP Configuration Control
pub const CS53L30_ASP_MS_SHIFT: c_int = 7;

pub const CS53L30_ASP_SCLK_INV_SHIFT: c_int = 4;

pub const CS53L30_ASP_RATE_SHIFT: c_int = 0;
pub const CS53L30_ASP_RATE_WIDTH: c_int = 4;

// R13/R24 (0x0D/0x18) CS53L30_ASP_CTL1 & CS53L30_ASP_CTL2 - ASP Control 1~2
pub const CS53L30_ASP_TDM_PDN_SHIFT: c_int = 7;

pub const CS53L30_ASP_SDOUTx_PDN_SHIFT: c_int = 6;

pub const CS53L30_ASP_3ST_SHIFT: c_int = 5;

pub const CS53L30_SHIFT_LEFT_SHIFT: c_int = 4;

pub const CS53L30_ASP_SDOUTx_DRIVE_SHIFT: c_int = 0;

// R14 (0x0E) ~ R17 (0x11) CS53L30_ASP_TDMTX_CTLx - ASP TDM TX Control 1~4
pub const CS53L30_ASP_CHx_TX_STATE_SHIFT: c_int = 7;

pub const CS53L30_ASP_CHx_TX_LOC_SHIFT: c_int = 0;
pub const CS53L30_ASP_CHx_TX_LOC_WIDTH: c_int = 6;

// R18 (0x12) ~ R23 (0x17) CS53L30_ASP_TDMTX_ENx - ASP TDM TX Enable 1~6

// R26 (0x1A) CS53L30_SFT_RAMP - Soft Ramp Control
pub const CS53L30_DIGSFT_SHIFT: c_int = 5;

// R28 (0x1C) CS53L30_LRCK_CTL2 - LRCK Control 2
pub const CS53L30_LRCK_50_NPW_SHIFT: c_int = 3;

pub const CS53L30_LRCK_TPWH_SHIFT: c_int = 0;
pub const CS53L30_LRCK_TPWH_WIDTH: c_int = 3;

// R31 (0x1F) CS53L30_MUTEP_CTL1 - MUTE Pin Control 1
pub const CS53L30_MUTE_PDN_ULP_SHIFT: c_int = 7;

pub const CS53L30_MUTE_PDN_LP_SHIFT: c_int = 6;

pub const CS53L30_MUTE_M4B_PDN_SHIFT: c_int = 4;

pub const CS53L30_MUTE_M3B_PDN_SHIFT: c_int = 3;

pub const CS53L30_MUTE_M2B_PDN_SHIFT: c_int = 2;

pub const CS53L30_MUTE_M1B_PDN_SHIFT: c_int = 1;

// Note: be careful - x starts from 0

pub const CS53L30_MUTE_MB_ALL_PDN_SHIFT: c_int = 0;

// R32 (0x20) CS53L30_MUTEP_CTL2 - MUTE Pin Control 2
pub const CS53L30_MUTE_PIN_POLARITY_SHIFT: c_int = 7;

pub const CS53L30_MUTE_ASP_TDM_PDN_SHIFT: c_int = 6;

pub const CS53L30_MUTE_ASP_SDOUT2_PDN_SHIFT: c_int = 5;

pub const CS53L30_MUTE_ASP_SDOUT1_PDN_SHIFT: c_int = 4;

// Note: be careful - x starts from 0

pub const CS53L30_MUTE_ADC2B_PDN_SHIFT: c_int = 3;

pub const CS53L30_MUTE_ADC2A_PDN_SHIFT: c_int = 2;

pub const CS53L30_MUTE_ADC1B_PDN_SHIFT: c_int = 1;

pub const CS53L30_MUTE_ADC1A_PDN_SHIFT: c_int = 0;

// R33 (0x21) CS53L30_INBIAS_CTL1 - Input Bias Control 1
pub const CS53L30_IN4M_BIAS_SHIFT: c_int = 6;
pub const CS53L30_IN4M_BIAS_WIDTH: c_int = 2;

pub const CS53L30_IN4P_BIAS_SHIFT: c_int = 4;
pub const CS53L30_IN4P_BIAS_WIDTH: c_int = 2;

pub const CS53L30_IN3M_BIAS_SHIFT: c_int = 2;
pub const CS53L30_IN3M_BIAS_WIDTH: c_int = 2;

pub const CS53L30_IN3P_BIAS_SHIFT: c_int = 0;
pub const CS53L30_IN3P_BIAS_WIDTH: c_int = 2;

// R34 (0x22) CS53L30_INBIAS_CTL2 - Input Bias Control 2
pub const CS53L30_IN2M_BIAS_SHIFT: c_int = 6;
pub const CS53L30_IN2M_BIAS_WIDTH: c_int = 2;

pub const CS53L30_IN2P_BIAS_SHIFT: c_int = 4;
pub const CS53L30_IN2P_BIAS_WIDTH: c_int = 2;

pub const CS53L30_IN1M_BIAS_SHIFT: c_int = 2;
pub const CS53L30_IN1M_BIAS_WIDTH: c_int = 2;

pub const CS53L30_IN1P_BIAS_SHIFT: c_int = 0;
pub const CS53L30_IN1P_BIAS_WIDTH: c_int = 2;

// R35 (0x23) & R36 (0x24) CS53L30_DMICx_STR_CTL - DMIC1 & DMIC2 Stereo Control
pub const CS53L30_DMICx_STEREO_ENB_SHIFT: c_int = 5;

// 0x88 and 0xCC are reserved bits

// R37/R45 (0x25/0x2D) CS53L30_ADCDMICx_CTL1 - ADC1/DMIC1 & ADC2/DMIC2 Control 1
pub const CS53L30_ADCxB_PDN_SHIFT: c_int = 7;

pub const CS53L30_ADCxA_PDN_SHIFT: c_int = 6;

pub const CS53L30_DMICx_PDN_SHIFT: c_int = 2;

pub const CS53L30_DMICx_SCLK_DIV_SHIFT: c_int = 1;

pub const CS53L30_CH_TYPE_SHIFT: c_int = 0;

pub const CS53L30_ADCDMICx_PDN_MASK: c_uint = 0xFF;

// R38/R46 (0x26/0x2E) CS53L30_ADCDMICx_CTL2 - ADC1/DMIC1 & ADC2/DMIC2 Control 2
pub const CS53L30_ADCx_NOTCH_DIS_SHIFT: c_int = 7;

pub const CS53L30_ADCxB_INV_SHIFT: c_int = 5;

pub const CS53L30_ADCxA_INV_SHIFT: c_int = 4;

pub const CS53L30_ADCxB_DIG_BOOST_SHIFT: c_int = 1;

pub const CS53L30_ADCxA_DIG_BOOST_SHIFT: c_int = 0;

// R39/R47 (0x27/0x2F) CS53L30_ADCx_CTL3 - ADC1/ADC2 Control 3
pub const CS53L30_ADCx_HPF_EN_SHIFT: c_int = 3;

pub const CS53L30_ADCx_HPF_CF_SHIFT: c_int = 1;
pub const CS53L30_ADCx_HPF_CF_WIDTH: c_int = 2;

pub const CS53L30_ADCx_NG_ALL_SHIFT: c_int = 0;

// R40/R48 (0x28/0x30) CS53L30_ADCx_NG_CTL - ADC1/ADC2 Noise Gate Control
pub const CS53L30_ADCxB_NG_SHIFT: c_int = 7;

pub const CS53L30_ADCxA_NG_SHIFT: c_int = 6;

pub const CS53L30_ADCx_NG_BOOST_SHIFT: c_int = 5;

pub const CS53L30_ADCx_NG_THRESH_SHIFT: c_int = 2;
pub const CS53L30_ADCx_NG_THRESH_WIDTH: c_int = 3;

pub const CS53L30_ADCx_NG_DELAY_SHIFT: c_int = 0;
pub const CS53L30_ADCx_NG_DELAY_WIDTH: c_int = 2;

// R41/R42/R49/R50 (0x29/0x2A/0x31/0x32) CS53L30_ADCxy_AFE_CTL - ADC1A/1B/2A/2B AFE Control
pub const CS53L30_ADCxy_PREAMP_SHIFT: c_int = 6;
pub const CS53L30_ADCxy_PREAMP_WIDTH: c_int = 2;

pub const CS53L30_ADCxy_PGA_VOL_SHIFT: c_int = 0;
pub const CS53L30_ADCxy_PGA_VOL_WIDTH: c_int = 6;

// R43/R44/R51/R52 (0x2B/0x2C/0x33/0x34) CS53L30_ADCxy_DIG_VOL - ADC1A/1B/2A/2B Digital Volume

// CS53L30_INT

pub const CS53L30_DEVICE_INT_MASK: c_uint = 0xFF;
