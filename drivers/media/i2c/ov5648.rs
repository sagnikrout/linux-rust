//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov5648.c
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
// Copyright (C) 2020 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

// Clock rate
pub const OV5648_XVCLK_RATE: c_int = 24000000;
// Register definitions
// System
pub const OV5648_SW_STANDBY_REG: c_uint = 0x100;

pub const OV5648_SW_RESET_REG: c_uint = 0x103;

pub const OV5648_PAD_OEN0_REG: c_uint = 0x3000;
pub const OV5648_PAD_OEN1_REG: c_uint = 0x3001;
pub const OV5648_PAD_OEN2_REG: c_uint = 0x3002;
pub const OV5648_PAD_OUT0_REG: c_uint = 0x3008;
pub const OV5648_PAD_OUT1_REG: c_uint = 0x3009;
pub const OV5648_CHIP_ID_H_REG: c_uint = 0x300a;
pub const OV5648_CHIP_ID_H_VALUE: c_uint = 0x56;
pub const OV5648_CHIP_ID_L_REG: c_uint = 0x300b;
pub const OV5648_CHIP_ID_L_VALUE: c_uint = 0x48;
pub const OV5648_PAD_OUT2_REG: c_uint = 0x300d;
pub const OV5648_PAD_SEL0_REG: c_uint = 0x300e;
pub const OV5648_PAD_SEL1_REG: c_uint = 0x300f;
pub const OV5648_PAD_SEL2_REG: c_uint = 0x3010;
pub const OV5648_PAD_PK_REG: c_uint = 0x3011;

pub const OV5648_A_PWC_PK_O0_REG: c_uint = 0x3013;

pub const OV5648_A_PWC_PK_O1_REG: c_uint = 0x3014;
pub const OV5648_MIPI_PHY0_REG: c_uint = 0x3016;
pub const OV5648_MIPI_PHY1_REG: c_uint = 0x3017;
pub const OV5648_MIPI_SC_CTRL0_REG: c_uint = 0x3018;

pub const OV5648_MIPI_SC_CTRL1_REG: c_uint = 0x3019;
pub const OV5648_MISC_CTRL0_REG: c_uint = 0x3021;
pub const OV5648_MIPI_SC_CTRL2_REG: c_uint = 0x3022;
pub const OV5648_SUB_ID_REG: c_uint = 0x302a;
pub const OV5648_PLL_CTRL0_REG: c_uint = 0x3034;

pub const OV5648_PLL_CTRL1_REG: c_uint = 0x3035;

pub const OV5648_PLL_MUL_REG: c_uint = 0x3036;

pub const OV5648_PLL_DIV_REG: c_uint = 0x3037;

pub const OV5648_PLL_DEBUG_REG: c_uint = 0x3038;
pub const OV5648_PLL_BYPASS_REG: c_uint = 0x3039;
pub const OV5648_PLLS_BYPASS_REG: c_uint = 0x303a;
pub const OV5648_PLLS_MUL_REG: c_uint = 0x303b;

pub const OV5648_PLLS_CTRL_REG: c_uint = 0x303c;

pub const OV5648_PLLS_DIV_REG: c_uint = 0x303d;

pub const OV5648_SRB_CTRL_REG: c_uint = 0x3106;

// Group Hold
pub const OV5648_GROUP_ADR0_REG: c_uint = 0x3200;
pub const OV5648_GROUP_ADR1_REG: c_uint = 0x3201;
pub const OV5648_GROUP_ADR2_REG: c_uint = 0x3202;
pub const OV5648_GROUP_ADR3_REG: c_uint = 0x3203;
pub const OV5648_GROUP_LEN0_REG: c_uint = 0x3204;
pub const OV5648_GROUP_LEN1_REG: c_uint = 0x3205;
pub const OV5648_GROUP_LEN2_REG: c_uint = 0x3206;
pub const OV5648_GROUP_LEN3_REG: c_uint = 0x3207;
pub const OV5648_GROUP_ACCESS_REG: c_uint = 0x3208;
// Exposure/gain/banding
pub const OV5648_EXPOSURE_CTRL_HH_REG: c_uint = 0x3500;

pub const OV5648_EXPOSURE_CTRL_H_REG: c_uint = 0x3501;

pub const OV5648_EXPOSURE_CTRL_L_REG: c_uint = 0x3502;

pub const OV5648_MANUAL_CTRL_REG: c_uint = 0x3503;

pub const OV5648_GAIN_CTRL_H_REG: c_uint = 0x350a;

pub const OV5648_GAIN_CTRL_L_REG: c_uint = 0x350b;

pub const OV5648_ANALOG_CTRL0_REG_BASE: c_uint = 0x3600;
pub const OV5648_ANALOG_CTRL1_REG_BASE: c_uint = 0x3700;
pub const OV5648_AEC_CTRL0_REG: c_uint = 0x3a00;

pub const OV5648_EXPOSURE_MIN_REG: c_uint = 0x3a01;
pub const OV5648_EXPOSURE_MAX_60_H_REG: c_uint = 0x3a02;
pub const OV5648_EXPOSURE_MAX_60_L_REG: c_uint = 0x3a03;
pub const OV5648_AEC_CTRL5_REG: c_uint = 0x3a05;
pub const OV5648_AEC_CTRL6_REG: c_uint = 0x3a06;
pub const OV5648_AEC_CTRL7_REG: c_uint = 0x3a07;
pub const OV5648_BANDING_STEP_50_H_REG: c_uint = 0x3a08;
pub const OV5648_BANDING_STEP_50_L_REG: c_uint = 0x3a09;
pub const OV5648_BANDING_STEP_60_H_REG: c_uint = 0x3a0a;
pub const OV5648_BANDING_STEP_60_L_REG: c_uint = 0x3a0b;
pub const OV5648_AEC_CTRLC_REG: c_uint = 0x3a0c;
pub const OV5648_BANDING_MAX_60_REG: c_uint = 0x3a0d;
pub const OV5648_BANDING_MAX_50_REG: c_uint = 0x3a0e;
pub const OV5648_WPT_REG: c_uint = 0x3a0f;
pub const OV5648_BPT_REG: c_uint = 0x3a10;
pub const OV5648_VPT_HIGH_REG: c_uint = 0x3a11;
pub const OV5648_AVG_MANUAL_REG: c_uint = 0x3a12;
pub const OV5648_PRE_GAIN_REG: c_uint = 0x3a13;
pub const OV5648_EXPOSURE_MAX_50_H_REG: c_uint = 0x3a14;
pub const OV5648_EXPOSURE_MAX_50_L_REG: c_uint = 0x3a15;
pub const OV5648_GAIN_BASE_NIGHT_REG: c_uint = 0x3a17;
pub const OV5648_AEC_GAIN_CEILING_H_REG: c_uint = 0x3a18;
pub const OV5648_AEC_GAIN_CEILING_L_REG: c_uint = 0x3a19;
pub const OV5648_DIFF_MAX_REG: c_uint = 0x3a1a;
pub const OV5648_WPT2_REG: c_uint = 0x3a1b;
pub const OV5648_LED_ADD_ROW_H_REG: c_uint = 0x3a1c;
pub const OV5648_LED_ADD_ROW_L_REG: c_uint = 0x3a1d;
pub const OV5648_BPT2_REG: c_uint = 0x3a1e;
pub const OV5648_VPT_LOW_REG: c_uint = 0x3a1f;
pub const OV5648_AEC_CTRL20_REG: c_uint = 0x3a20;
pub const OV5648_AEC_CTRL21_REG: c_uint = 0x3a21;
pub const OV5648_AVG_START_X_H_REG: c_uint = 0x5680;
pub const OV5648_AVG_START_X_L_REG: c_uint = 0x5681;
pub const OV5648_AVG_START_Y_H_REG: c_uint = 0x5682;
pub const OV5648_AVG_START_Y_L_REG: c_uint = 0x5683;
pub const OV5648_AVG_WINDOW_X_H_REG: c_uint = 0x5684;
pub const OV5648_AVG_WINDOW_X_L_REG: c_uint = 0x5685;
pub const OV5648_AVG_WINDOW_Y_H_REG: c_uint = 0x5686;
pub const OV5648_AVG_WINDOW_Y_L_REG: c_uint = 0x5687;
pub const OV5648_AVG_WEIGHT00_REG: c_uint = 0x5688;
pub const OV5648_AVG_WEIGHT01_REG: c_uint = 0x5689;
pub const OV5648_AVG_WEIGHT02_REG: c_uint = 0x568a;
pub const OV5648_AVG_WEIGHT03_REG: c_uint = 0x568b;
pub const OV5648_AVG_WEIGHT04_REG: c_uint = 0x568c;
pub const OV5648_AVG_WEIGHT05_REG: c_uint = 0x568d;
pub const OV5648_AVG_WEIGHT06_REG: c_uint = 0x568e;
pub const OV5648_AVG_WEIGHT07_REG: c_uint = 0x568f;
pub const OV5648_AVG_CTRL10_REG: c_uint = 0x5690;
pub const OV5648_AVG_WEIGHT_SUM_REG: c_uint = 0x5691;
pub const OV5648_AVG_READOUT_REG: c_uint = 0x5693;
pub const OV5648_DIG_CTRL0_REG: c_uint = 0x5a00;
pub const OV5648_DIG_COMP_MAN_H_REG: c_uint = 0x5a02;
pub const OV5648_DIG_COMP_MAN_L_REG: c_uint = 0x5a03;
pub const OV5648_GAINC_MAN_H_REG: c_uint = 0x5a20;
pub const OV5648_GAINC_MAN_L_REG: c_uint = 0x5a21;
pub const OV5648_GAINC_DGC_MAN_H_REG: c_uint = 0x5a22;
pub const OV5648_GAINC_DGC_MAN_L_REG: c_uint = 0x5a23;
pub const OV5648_GAINC_CTRL0_REG: c_uint = 0x5a24;
pub const OV5648_GAINF_ANA_NUM_REG: c_uint = 0x5a40;
pub const OV5648_GAINF_DIG_GAIN_REG: c_uint = 0x5a41;
// Timing
pub const OV5648_CROP_START_X_H_REG: c_uint = 0x3800;

pub const OV5648_CROP_START_X_L_REG: c_uint = 0x3801;

pub const OV5648_CROP_START_Y_H_REG: c_uint = 0x3802;

pub const OV5648_CROP_START_Y_L_REG: c_uint = 0x3803;

pub const OV5648_CROP_END_X_H_REG: c_uint = 0x3804;

pub const OV5648_CROP_END_X_L_REG: c_uint = 0x3805;

pub const OV5648_CROP_END_Y_H_REG: c_uint = 0x3806;

pub const OV5648_CROP_END_Y_L_REG: c_uint = 0x3807;

pub const OV5648_OUTPUT_SIZE_X_H_REG: c_uint = 0x3808;

pub const OV5648_OUTPUT_SIZE_X_L_REG: c_uint = 0x3809;

pub const OV5648_OUTPUT_SIZE_Y_H_REG: c_uint = 0x380a;

pub const OV5648_OUTPUT_SIZE_Y_L_REG: c_uint = 0x380b;

pub const OV5648_HTS_H_REG: c_uint = 0x380c;

pub const OV5648_HTS_L_REG: c_uint = 0x380d;

pub const OV5648_VTS_H_REG: c_uint = 0x380e;

pub const OV5648_VTS_L_REG: c_uint = 0x380f;

pub const OV5648_OFFSET_X_H_REG: c_uint = 0x3810;

pub const OV5648_OFFSET_X_L_REG: c_uint = 0x3811;

pub const OV5648_OFFSET_Y_H_REG: c_uint = 0x3812;

pub const OV5648_OFFSET_Y_L_REG: c_uint = 0x3813;

pub const OV5648_SUB_INC_X_REG: c_uint = 0x3814;

pub const OV5648_SUB_INC_Y_REG: c_uint = 0x3815;

pub const OV5648_HSYNCST_H_REG: c_uint = 0x3816;

pub const OV5648_HSYNCST_L_REG: c_uint = 0x3817;

pub const OV5648_HSYNCW_H_REG: c_uint = 0x3818;

pub const OV5648_HSYNCW_L_REG: c_uint = 0x3819;

pub const OV5648_TC20_REG: c_uint = 0x3820;

pub const OV5648_TC21_REG: c_uint = 0x3821;

// Strobe/exposure
pub const OV5648_STROBE_REG: c_uint = 0x3b00;
pub const OV5648_FREX_EXP_HH_REG: c_uint = 0x3b01;
pub const OV5648_SHUTTER_DLY_H_REG: c_uint = 0x3b02;
pub const OV5648_SHUTTER_DLY_L_REG: c_uint = 0x3b03;
pub const OV5648_FREX_EXP_H_REG: c_uint = 0x3b04;
pub const OV5648_FREX_EXP_L_REG: c_uint = 0x3b05;
pub const OV5648_FREX_CTRL_REG: c_uint = 0x3b06;
pub const OV5648_FREX_MODE_SEL_REG: c_uint = 0x3b07;

pub const OV5648_FREX_MODE_SEL_MODE1: c_uint = 0x0;
pub const OV5648_FREX_MODE_SEL_MODE2: c_uint = 0x1;
pub const OV5648_FREX_MODE_SEL_ROLLING: c_uint = 0x2;
pub const OV5648_FREX_EXP_REQ_REG: c_uint = 0x3b08;
pub const OV5648_FREX_SHUTTER_DLY_REG: c_uint = 0x3b09;
pub const OV5648_FREX_RST_LEN_REG: c_uint = 0x3b0a;
pub const OV5648_STROBE_WIDTH_HH_REG: c_uint = 0x3b0b;
pub const OV5648_STROBE_WIDTH_H_REG: c_uint = 0x3b0c;
// OTP
pub const OV5648_OTP_DATA_REG_BASE: c_uint = 0x3d00;
pub const OV5648_OTP_PROGRAM_CTRL_REG: c_uint = 0x3d80;
pub const OV5648_OTP_LOAD_CTRL_REG: c_uint = 0x3d81;
// PSRAM
pub const OV5648_PSRAM_CTRL1_REG: c_uint = 0x3f01;
pub const OV5648_PSRAM_CTRLF_REG: c_uint = 0x3f0f;
// Black Level
pub const OV5648_BLC_CTRL0_REG: c_uint = 0x4000;
pub const OV5648_BLC_CTRL1_REG: c_uint = 0x4001;

pub const OV5648_BLC_CTRL2_REG: c_uint = 0x4002;

pub const OV5648_BLC_CTRL3_REG: c_uint = 0x4003;
pub const OV5648_BLC_LINE_NUM_REG: c_uint = 0x4004;

pub const OV5648_BLC_CTRL5_REG: c_uint = 0x4005;

pub const OV5648_BLC_LEVEL_REG: c_uint = 0x4009;
// Frame
pub const OV5648_FRAME_CTRL_REG: c_uint = 0x4200;
pub const OV5648_FRAME_ON_NUM_REG: c_uint = 0x4201;
pub const OV5648_FRAME_OFF_NUM_REG: c_uint = 0x4202;
// MIPI CSI-2
pub const OV5648_MIPI_CTRL0_REG: c_uint = 0x4800;

pub const OV5648_MIPI_CTRL0_LANE_SELECT_LANE1: c_int = 0;

pub const OV5648_MIPI_CTRL0_IDLE_LP00: c_int = 0;

pub const OV5648_MIPI_CTRL1_REG: c_uint = 0x4801;
pub const OV5648_MIPI_CTRL2_REG: c_uint = 0x4802;
pub const OV5648_MIPI_CTRL3_REG: c_uint = 0x4803;
pub const OV5648_MIPI_CTRL4_REG: c_uint = 0x4804;
pub const OV5648_MIPI_CTRL5_REG: c_uint = 0x4805;
pub const OV5648_MIPI_MAX_FRAME_COUNT_H_REG: c_uint = 0x4810;
pub const OV5648_MIPI_MAX_FRAME_COUNT_L_REG: c_uint = 0x4811;
pub const OV5648_MIPI_CTRL14_REG: c_uint = 0x4814;
pub const OV5648_MIPI_DT_SPKT_REG: c_uint = 0x4815;
pub const OV5648_MIPI_HS_ZERO_MIN_H_REG: c_uint = 0x4818;
pub const OV5648_MIPI_HS_ZERO_MIN_L_REG: c_uint = 0x4819;
pub const OV5648_MIPI_HS_TRAIN_MIN_H_REG: c_uint = 0x481a;
pub const OV5648_MIPI_HS_TRAIN_MIN_L_REG: c_uint = 0x481b;
pub const OV5648_MIPI_CLK_ZERO_MIN_H_REG: c_uint = 0x481c;
pub const OV5648_MIPI_CLK_ZERO_MIN_L_REG: c_uint = 0x481d;
pub const OV5648_MIPI_CLK_PREPARE_MIN_H_REG: c_uint = 0x481e;
pub const OV5648_MIPI_CLK_PREPARE_MIN_L_REG: c_uint = 0x481f;
pub const OV5648_MIPI_CLK_POST_MIN_H_REG: c_uint = 0x4820;
pub const OV5648_MIPI_CLK_POST_MIN_L_REG: c_uint = 0x4821;
pub const OV5648_MIPI_CLK_TRAIL_MIN_H_REG: c_uint = 0x4822;
pub const OV5648_MIPI_CLK_TRAIL_MIN_L_REG: c_uint = 0x4823;
pub const OV5648_MIPI_LPX_P_MIN_H_REG: c_uint = 0x4824;
pub const OV5648_MIPI_LPX_P_MIN_L_REG: c_uint = 0x4825;
pub const OV5648_MIPI_HS_PREPARE_MIN_H_REG: c_uint = 0x4826;
pub const OV5648_MIPI_HS_PREPARE_MIN_L_REG: c_uint = 0x4827;
pub const OV5648_MIPI_HS_EXIT_MIN_H_REG: c_uint = 0x4828;
pub const OV5648_MIPI_HS_EXIT_MIN_L_REG: c_uint = 0x4829;
pub const OV5648_MIPI_HS_ZERO_MIN_UI_REG: c_uint = 0x482a;
pub const OV5648_MIPI_HS_TRAIL_MIN_UI_REG: c_uint = 0x482b;
pub const OV5648_MIPI_CLK_ZERO_MIN_UI_REG: c_uint = 0x482c;
pub const OV5648_MIPI_CLK_PREPARE_MIN_UI_REG: c_uint = 0x482d;
pub const OV5648_MIPI_CLK_POST_MIN_UI_REG: c_uint = 0x482e;
pub const OV5648_MIPI_CLK_TRAIL_MIN_UI_REG: c_uint = 0x482f;
pub const OV5648_MIPI_LPX_P_MIN_UI_REG: c_uint = 0x4830;
pub const OV5648_MIPI_HS_PREPARE_MIN_UI_REG: c_uint = 0x4831;
pub const OV5648_MIPI_HS_EXIT_MIN_UI_REG: c_uint = 0x4832;
pub const OV5648_MIPI_REG_MIN_H_REG: c_uint = 0x4833;
pub const OV5648_MIPI_REG_MIN_L_REG: c_uint = 0x4834;
pub const OV5648_MIPI_REG_MAX_H_REG: c_uint = 0x4835;
pub const OV5648_MIPI_REG_MAX_L_REG: c_uint = 0x4836;
pub const OV5648_MIPI_PCLK_PERIOD_REG: c_uint = 0x4837;
pub const OV5648_MIPI_WKUP_DLY_REG: c_uint = 0x4838;
pub const OV5648_MIPI_LP_GPIO_REG: c_uint = 0x483b;
pub const OV5648_MIPI_SNR_PCLK_DIV_REG: c_uint = 0x4843;
// ISP
pub const OV5648_ISP_CTRL0_REG: c_uint = 0x5000;

pub const OV5648_ISP_CTRL1_REG: c_uint = 0x5001;

pub const OV5648_ISP_CTRL2_REG: c_uint = 0x5002;

pub const OV5648_ISP_CTRL3_REG: c_uint = 0x5003;

pub const OV5648_ISP_CTRL4_REG: c_uint = 0x5004;
pub const OV5648_ISP_CTRL5_REG: c_uint = 0x5005;
pub const OV5648_ISP_CTRL6_REG: c_uint = 0x5006;
pub const OV5648_ISP_CTRL7_REG: c_uint = 0x5007;
pub const OV5648_ISP_MAN_OFFSET_X_H_REG: c_uint = 0x5008;
pub const OV5648_ISP_MAN_OFFSET_X_L_REG: c_uint = 0x5009;
pub const OV5648_ISP_MAN_OFFSET_Y_H_REG: c_uint = 0x500a;
pub const OV5648_ISP_MAN_OFFSET_Y_L_REG: c_uint = 0x500b;
pub const OV5648_ISP_MAN_WIN_OFFSET_X_H_REG: c_uint = 0x500c;
pub const OV5648_ISP_MAN_WIN_OFFSET_X_L_REG: c_uint = 0x500d;
pub const OV5648_ISP_MAN_WIN_OFFSET_Y_H_REG: c_uint = 0x500e;
pub const OV5648_ISP_MAN_WIN_OFFSET_Y_L_REG: c_uint = 0x500f;
pub const OV5648_ISP_MAN_WIN_OUTPUT_X_H_REG: c_uint = 0x5010;
pub const OV5648_ISP_MAN_WIN_OUTPUT_X_L_REG: c_uint = 0x5011;
pub const OV5648_ISP_MAN_WIN_OUTPUT_Y_H_REG: c_uint = 0x5012;
pub const OV5648_ISP_MAN_WIN_OUTPUT_Y_L_REG: c_uint = 0x5013;
pub const OV5648_ISP_MAN_INPUT_X_H_REG: c_uint = 0x5014;
pub const OV5648_ISP_MAN_INPUT_X_L_REG: c_uint = 0x5015;
pub const OV5648_ISP_MAN_INPUT_Y_H_REG: c_uint = 0x5016;
pub const OV5648_ISP_MAN_INPUT_Y_L_REG: c_uint = 0x5017;
pub const OV5648_ISP_CTRL18_REG: c_uint = 0x5018;
pub const OV5648_ISP_CTRL19_REG: c_uint = 0x5019;
pub const OV5648_ISP_CTRL1A_REG: c_uint = 0x501a;
pub const OV5648_ISP_CTRL1D_REG: c_uint = 0x501d;
pub const OV5648_ISP_CTRL1F_REG: c_uint = 0x501f;
pub const OV5648_ISP_CTRL1F_OUTPUT_EN: c_int = 3;
pub const OV5648_ISP_CTRL25_REG: c_uint = 0x5025;
pub const OV5648_ISP_CTRL3D_REG: c_uint = 0x503d;

pub const OV5648_ISP_CTRL3D_PATTERN_COLOR_BARS: c_int = 0;
pub const OV5648_ISP_CTRL3D_PATTERN_RANDOM_DATA: c_int = 1;
pub const OV5648_ISP_CTRL3D_PATTERN_COLOR_SQUARES: c_int = 2;
pub const OV5648_ISP_CTRL3D_PATTERN_INPUT: c_int = 3;
pub const OV5648_ISP_CTRL3E_REG: c_uint = 0x503e;
pub const OV5648_ISP_CTRL4B_REG: c_uint = 0x504b;

pub const OV5648_ISP_CTRL4C_REG: c_uint = 0x504c;
pub const OV5648_ISP_CTRL57_REG: c_uint = 0x5057;
pub const OV5648_ISP_CTRL58_REG: c_uint = 0x5058;
pub const OV5648_ISP_CTRL59_REG: c_uint = 0x5059;
pub const OV5648_ISP_WINDOW_START_X_H_REG: c_uint = 0x5980;
pub const OV5648_ISP_WINDOW_START_X_L_REG: c_uint = 0x5981;
pub const OV5648_ISP_WINDOW_START_Y_H_REG: c_uint = 0x5982;
pub const OV5648_ISP_WINDOW_START_Y_L_REG: c_uint = 0x5983;
pub const OV5648_ISP_WINDOW_WIN_X_H_REG: c_uint = 0x5984;
pub const OV5648_ISP_WINDOW_WIN_X_L_REG: c_uint = 0x5985;
pub const OV5648_ISP_WINDOW_WIN_Y_H_REG: c_uint = 0x5986;
pub const OV5648_ISP_WINDOW_WIN_Y_L_REG: c_uint = 0x5987;
pub const OV5648_ISP_WINDOW_MAN_REG: c_uint = 0x5988;
// White Balance
pub const OV5648_AWB_CTRL_REG: c_uint = 0x5180;

pub const OV5648_AWB_DELTA_REG: c_uint = 0x5181;
pub const OV5648_AWB_STABLE_RANGE_REG: c_uint = 0x5182;
pub const OV5648_AWB_STABLE_RANGE_WIDE_REG: c_uint = 0x5183;
pub const OV5648_HSIZE_MAN_REG: c_uint = 0x5185;
pub const OV5648_GAIN_RED_MAN_H_REG: c_uint = 0x5186;

pub const OV5648_GAIN_RED_MAN_L_REG: c_uint = 0x5187;

pub const OV5648_GAIN_GREEN_MAN_H_REG: c_uint = 0x5188;

pub const OV5648_GAIN_GREEN_MAN_L_REG: c_uint = 0x5189;

pub const OV5648_GAIN_BLUE_MAN_H_REG: c_uint = 0x518a;

pub const OV5648_GAIN_BLUE_MAN_L_REG: c_uint = 0x518b;

pub const OV5648_GAIN_RED_LIMIT_REG: c_uint = 0x518c;
pub const OV5648_GAIN_GREEN_LIMIT_REG: c_uint = 0x518d;
pub const OV5648_GAIN_BLUE_LIMIT_REG: c_uint = 0x518e;
pub const OV5648_AWB_FRAME_COUNT_REG: c_uint = 0x518f;
pub const OV5648_AWB_BASE_MAN_REG: c_uint = 0x51df;
// Macros

    container_of(s, struct ov5648_sensor, subdev)

    (&container_of((c).handler, struct ov5648_sensor, \
    ctrls.handler).subdev)
// Data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_register_value {
    pub address: u16,
    pub value: u8,
    pub delay_ms: c_uint,
}

//
// PLL1 Clock Tree:
//
// +-< XVCLK
// |
// +-+ pll_pre_div (0x3037 [3:0], special values: 5: 1.5, 7: 2.5)
// |
// +-+ pll_mul (0x3036 [7:0])
// |
// +-+ sys_div (0x3035 [7:4])
// |
// +-+ mipi_div (0x3035 [3:0])
// | |
// | +-> MIPI_SCLK
// | |
// | +-+ mipi_phy_div (2)
// |   |
// |   +-> MIPI_CLK
// |
// +-+ root_div (0x3037 [4])
// |
// +-+ bit_div (0x3034 [3:0], 8 bits: 2, 10 bits: 2.5, other: 1)
// |
// +-+ sclk_div (0x3106 [3:2])
// |
// +-> SCLK
// |
// +-+ mipi_div (0x3035, 1: PCLK = SCLK)
// |
// +-> PCLK
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_pll1_config {
    pub pll_pre_div: c_uint,
    pub pll_mul: c_uint,
    pub sys_div: c_uint,
    pub root_div: c_uint,
    pub sclk_div: c_uint,
    pub mipi_div: c_uint,
}

//
// PLL2 Clock Tree:
//
// +-< XVCLK
// |
// +-+ plls_pre_div (0x303d [5:4], special values: 0: 1, 1: 1.5)
// |
// +-+ plls_div_r (0x303d [2])
// |
// +-+ plls_mul (0x303b [4:0])
// |
// +-+ sys_div (0x303c [3:0])
// |
// +-+ sel_div (0x303d [1:0], special values: 0: 1, 3: 2.5)
// |
// +-> ADCLK
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_pll2_config {
    pub plls_pre_div: c_uint,
    pub plls_div_r: c_uint,
    pub plls_mul: c_uint,
    pub sys_div: c_uint,
    pub sel_div: c_uint,
}

//
// General formulas for (array-centered) mode calculation:
// - photo_array_width = 2624
// - crop_start_x = (photo_array_width - output_size_x) / 2
// - crop_end_x = crop_start_x + offset_x + output_size_x - 1
//
// - photo_array_height = 1956
// - crop_start_y = (photo_array_height - output_size_y) / 2
// - crop_end_y = crop_start_y + offset_y + output_size_y - 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_mode {
    pub crop_start_x: c_uint,
    pub offset_x: c_uint,
    pub output_size_x: c_uint,
    pub crop_end_x: c_uint,
    pub hts: c_uint,
    pub crop_start_y: c_uint,
    pub offset_y: c_uint,
    pub output_size_y: c_uint,
    pub crop_end_y: c_uint,
    pub vts: c_uint,
    pub binning_x: bool,
    pub binning_y: bool,
    pub inc_x_odd: c_uint,
    pub inc_x_even: c_uint,
    pub inc_y_odd: c_uint,
    pub inc_y_even: c_uint,
// 8-bit frame interval followed by 10-bit frame interval.
    pub frame_interval: [v4l2_fract; 2],
// 8-bit config followed by 10-bit config.
    pub pll1_config: [*const ov5648_pll1_config; 2],
    pub pll2_config: *const ov5648_pll2_config,
    pub register_values: *const ov5648_register_value,
    pub register_values_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_state {
    pub mode: *const ov5648_mode,
    pub mbus_code: u32,
    pub streaming: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_ctrls {
    pub exposure_auto: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub gain_auto: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
    pub white_balance_auto: *mut v4l2_ctrl,
    pub red_balance: *mut v4l2_ctrl,
    pub blue_balance: *mut v4l2_ctrl,
    pub link_freq: *mut v4l2_ctrl,
    pub pixel_rate: *mut v4l2_ctrl,
    pub handler: v4l2_ctrl_handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5648_sensor {
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub reset: *mut gpio_desc,
    pub powerdown: *mut gpio_desc,
    pub avdd: *mut regulator,
    pub dvdd: *mut regulator,
    pub dovdd: *mut regulator,
    pub xvclk: *mut clk,
    pub endpoint: v4l2_fwnode_endpoint,
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub mutex: mutex,
    pub state: ov5648_state,
    pub ctrls: ov5648_ctrls,
}

// Static definitions
//
// XVCLK = 24 MHz
// SCLK  = 84 MHz
// PCLK  = 84 MHz
//
    static const struct ov5648_pll1_config ov5648_pll1_config_native_8_bits = {
    .pll_pre_div	= 3,
    .pll_mul	= 84,
    .sys_div	= 2,
    .root_div	= 1,
    .sclk_div	= 1,
    .mipi_div	= 1,
    };
//
// XVCLK = 24 MHz
// SCLK  = 84 MHz
// PCLK  = 84 MHz
//
    static const struct ov5648_pll1_config ov5648_pll1_config_native_10_bits = {
    .pll_pre_div	= 3,
    .pll_mul	= 105,
    .sys_div	= 2,
    .root_div	= 1,
    .sclk_div	= 1,
    .mipi_div	= 1,
    };
//
// XVCLK = 24 MHz
// ADCLK = 200 MHz
//
    static const struct ov5648_pll2_config ov5648_pll2_config_native = {
    .plls_pre_div	= 3,
    .plls_div_r	= 1,
    .plls_mul	= 25,
    .sys_div	= 1,
    .sel_div	= 1,
    };
    static const struct ov5648_mode ov5648_modes[] = {
// 2592x1944
    {
// Horizontal
    .crop_start_x	= 16,
    .offset_x	= 0,
    .output_size_x	= 2592,
    .crop_end_x	= 2607,
    .hts		= 2816,
// Vertical
    .crop_start_y	= 6,
    .offset_y	= 0,
    .output_size_y	= 1944,
    .crop_end_y	= 1949,
    .vts		= 1984,
// Subsample increase
    .inc_x_odd	= 1,
    .inc_x_even	= 1,
    .inc_y_odd	= 1,
    .inc_y_even	= 1,
// Frame Interval
    .frame_interval	= {
    { 1,	15 },
    { 1,	15 },
    },
// PLL
    .pll1_config	= {
    &ov5648_pll1_config_native_8_bits,
    &ov5648_pll1_config_native_10_bits,
    },
    .pll2_config	= &ov5648_pll2_config_native,
    },
// 1600x1200 (UXGA)
    {
// Horizontal
    .crop_start_x	= 512,
    .offset_x	= 0,
    .output_size_x	= 1600,
    .crop_end_x	= 2111,
    .hts		= 2816,
// Vertical
    .crop_start_y	= 378,
    .offset_y	= 0,
    .output_size_y	= 1200,
    .crop_end_y	= 1577,
    .vts		= 1984,
// Subsample increase
    .inc_x_odd	= 1,
    .inc_x_even	= 1,
    .inc_y_odd	= 1,
    .inc_y_even	= 1,
// Frame Interval
    .frame_interval	= {
    { 1,	15 },
    { 1,	15 },
    },
// PLL
    .pll1_config	= {
    &ov5648_pll1_config_native_8_bits,
    &ov5648_pll1_config_native_10_bits,
    },
    .pll2_config	= &ov5648_pll2_config_native,
    },
// 1920x1080 (Full HD)
    {
// Horizontal
    .crop_start_x	= 352,
    .offset_x	= 0,
    .output_size_x	= 1920,
    .crop_end_x	= 2271,
    .hts		= 2816,
// Vertical
    .crop_start_y	= 438,
    .offset_y	= 0,
    .output_size_y	= 1080,
    .crop_end_y	= 1517,
    .vts		= 1984,
// Subsample increase
    .inc_x_odd	= 1,
    .inc_x_even	= 1,
    .inc_y_odd	= 1,
    .inc_y_even	= 1,
// Frame Interval
    .frame_interval	= {
    { 1,	15 },
    { 1,	15 },
    },
// PLL
    .pll1_config	= {
    &ov5648_pll1_config_native_8_bits,
    &ov5648_pll1_config_native_10_bits,
    },
    .pll2_config	= &ov5648_pll2_config_native,
    },
// 1280x960
    {
// Horizontal
    .crop_start_x	= 16,
    .offset_x	= 8,
    .output_size_x	= 1280,
    .crop_end_x	= 2607,
    .hts		= 1912,
// Vertical
    .crop_start_y	= 6,
    .offset_y	= 6,
    .output_size_y	= 960,
    .crop_end_y	= 1949,
    .vts		= 1496,
// Binning
    .binning_x	= true,
// Subsample increase
    .inc_x_odd	= 3,
    .inc_x_even	= 1,
    .inc_y_odd	= 3,
    .inc_y_even	= 1,
// Frame Interval
    .frame_interval	= {
    { 1,	30 },
    { 1,	30 },
    },
// PLL
    .pll1_config	= {
    &ov5648_pll1_config_native_8_bits,
    &ov5648_pll1_config_native_10_bits,
    },
    .pll2_config	= &ov5648_pll2_config_native,
    },
// 1280x720 (HD)
    {
// Horizontal
    .crop_start_x	= 16,
    .offset_x	= 8,
    .output_size_x	= 1280,
    .crop_end_x	= 2607,
    .hts		= 1912,
// Vertical
    .crop_start_y	= 254,
    .offset_y	= 2,
    .output_size_y	= 720,
    .crop_end_y	= 1701,
    .vts		= 1496,
// Binning
    .binning_x	= true,
// Subsample increase
    .inc_x_odd	= 3,
    .inc_x_even	= 1,
    .inc_y_odd	= 3,
    .inc_y_even	= 1,
// Frame Interval
    .frame_interval	= {
    { 1,	30 },
    { 1,	30 },
    },
// PLL
    .pll1_config	= {
    &ov5648_pll1_config_native_8_bits,
    &ov5648_pll1_config_native_10_bits,
    },
    .pll2_config	= &ov5648_pll2_config_native,
    },
// 640x480 (VGA)
    {
// Horizontal
    .crop_start_x	= 0,
    .offset_x	= 8,
    .output_size_x	= 640,
    .crop_end_x	= 2623,
    .hts		= 1896,
// Vertical
    .crop_start_y	= 0,
    .offset_y	= 2,
    .output_size_y	= 480,
    .crop_end_y	= 1953,
    .vts		= 984,
// Binning
    .binning_x	= true,
// Subsample increase
    .inc_x_odd	= 7,
    .inc_x_even	= 1,
    .inc_y_odd	= 7,
    .inc_y_even	= 1,
// Frame Interval
    .frame_interval	= {
    { 1,	30 },
    { 1,	30 },
    },
// PLL
    .pll1_config	= {
    &ov5648_pll1_config_native_8_bits,
    &ov5648_pll1_config_native_10_bits,
    },
    .pll2_config	= &ov5648_pll2_config_native,
    },
    };
    static const u32 ov5648_mbus_codes[] = {
    MEDIA_BUS_FMT_SBGGR8_1X8,
    MEDIA_BUS_FMT_SBGGR10_1X10,
    };
    static const struct ov5648_register_value ov5648_init_sequence[] = {
// PSRAM
    { OV5648_PSRAM_CTRL1_REG, 0x0d },
    { OV5648_PSRAM_CTRLF_REG, 0xf5 },
    };
    static const s64 ov5648_link_freq_menu[] = {
    210000000,
    168000000,
    };
    static const char *const ov5648_test_pattern_menu[] = {
    "Disabled",
    "Random data",
    "Color bars",
    "Color bars with rolling bar",
    "Color squares",
    "Color squares with rolling bar"
    };
    static const u8 ov5648_test_pattern_bits[] = {
    0,
    OV5648_ISP_CTRL3D_PATTERN_EN | OV5648_ISP_CTRL3D_PATTERN_RANDOM_DATA,
    OV5648_ISP_CTRL3D_PATTERN_EN | OV5648_ISP_CTRL3D_PATTERN_COLOR_BARS,
    OV5648_ISP_CTRL3D_PATTERN_EN | OV5648_ISP_CTRL3D_ROLLING_BAR_EN |
    OV5648_ISP_CTRL3D_PATTERN_COLOR_BARS,
    OV5648_ISP_CTRL3D_PATTERN_EN | OV5648_ISP_CTRL3D_PATTERN_COLOR_SQUARES,
    OV5648_ISP_CTRL3D_PATTERN_EN | OV5648_ISP_CTRL3D_ROLLING_BAR_EN |
    OV5648_ISP_CTRL3D_PATTERN_COLOR_SQUARES,
    };
// Input/Output
#[no_mangle]
unsafe extern "C" fn ov5648_read(sensor: *mut ov5648_sensor, address: u16, value: *mut u8) -> c_int {
    static int ov5648_read(struct ov5648_sensor *sensor, u16 address, u8 *value)
    {
    unsigned char data[2] = { address >> 8, address & 0xff };
    struct i2c_client *client = sensor.i2c_client;
    int ret;
    ret = i2c_master_send(client, data, sizeof(data));
    if (ret < 0) {
    dev_dbg(&client.dev, "i2c send error at address %#04x\n",
    address);
    return ret;
    }
    ret = i2c_master_recv(client, value, 1);
    if (ret < 0) {
    dev_dbg(&client.dev, "i2c recv error at address %#04x\n",
    address);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_write(sensor: *mut ov5648_sensor, address: u16, value: u8) -> c_int {
    static int ov5648_write(struct ov5648_sensor *sensor, u16 address, u8 value)
    {
    unsigned char data[3] = { address >> 8, address & 0xff, value };
    struct i2c_client *client = sensor.i2c_client;
    int ret;
    ret = i2c_master_send(client, data, sizeof(data));
    if (ret < 0) {
    dev_dbg(&client.dev, "i2c send error at address %#04x\n",
    address);
    return ret;
    }
    return 0;
    }
    static int ov5648_write_sequence(struct ov5648_sensor *sensor,
    const struct ov5648_register_value *sequence,
    unsigned int sequence_count)
    {
    unsigned int i;
    let mut ret: c_int = 0;
    for (i = 0; i < sequence_count; i++) {
    ret = ov5648_write(sensor, sequence[i].address,
    sequence[i].value);
    if (ret)
    break;
    if (sequence[i].delay_ms)
    msleep(sequence[i].delay_ms);
    }
    return ret;
    }
    static int ov5648_update_bits(struct ov5648_sensor *sensor, u16 address,
    u8 mask, u8 bits)
    {
    let mut value: u8 = 0;
    int ret;
    ret = ov5648_read(sensor, address, &value);
    if (ret)
    return ret;
    value &= ~mask;
    value |= bits;
    ret = ov5648_write(sensor, address, value);
    if (ret)
    return ret;
    return 0;
    }
// Sensor
#[no_mangle]
unsafe extern "C" fn ov5648_sw_reset(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_sw_reset(struct ov5648_sensor *sensor)
    {
    return ov5648_write(sensor, OV5648_SW_RESET_REG, OV5648_SW_RESET_RESET);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_sw_standby(sensor: *mut ov5648_sensor, standby: c_int) -> c_int {
    static int ov5648_sw_standby(struct ov5648_sensor *sensor, int standby)
    {
    let mut value: u8 = 0;
    if (!standby)
    value = OV5648_SW_STANDBY_STREAM_ON;
    return ov5648_write(sensor, OV5648_SW_STANDBY_REG, value);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_chip_id_check(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_chip_id_check(struct ov5648_sensor *sensor)
    {
    static const u16 regs[] = { OV5648_CHIP_ID_H_REG, OV5648_CHIP_ID_L_REG };
    static const u8 values[] = { OV5648_CHIP_ID_H_VALUE, OV5648_CHIP_ID_L_VALUE };
    unsigned int i;
    u8 value;
    int ret;
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    ret = ov5648_read(sensor, regs[i], &value);
    if (ret < 0)
    return ret;
    if (value != values[i]) {
    dev_err(sensor.dev,
    "chip id value mismatch: %#x instead of %#x\n",
    value, values[i]);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_avdd_internal_power(sensor: *mut ov5648_sensor, on: c_int) -> c_int {
    static int ov5648_avdd_internal_power(struct ov5648_sensor *sensor, int on)
    {
    return ov5648_write(sensor, OV5648_A_PWC_PK_O0_REG,
    on ? 0 : OV5648_A_PWC_PK_O0_BP_REGULATOR_N);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_pad_configure(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_pad_configure(struct ov5648_sensor *sensor)
    {
    int ret;
// Configure pads as input.
    ret = ov5648_write(sensor, OV5648_PAD_OEN1_REG, 0);
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_PAD_OEN2_REG, 0);
    if (ret)
    return ret;
// Disable FREX pin.
    return ov5648_write(sensor, OV5648_PAD_PK_REG,
    OV5648_PAD_PK_DRIVE_STRENGTH_1X |
    OV5648_PAD_PK_FREX_N);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_mipi_configure(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_mipi_configure(struct ov5648_sensor *sensor)
    {
    struct v4l2_mbus_config_mipi_csi2 *bus_mipi_csi2 =
    &sensor.endpoint.bus.mipi_csi2;
    let mut lanes_count: c_uint = bus_mipi_csi2.num_data_lanes;
    int ret;
    ret = ov5648_write(sensor, OV5648_MIPI_CTRL0_REG,
    OV5648_MIPI_CTRL0_CLK_LANE_AUTOGATE |
    OV5648_MIPI_CTRL0_LANE_SELECT_LANE1 |
    OV5648_MIPI_CTRL0_IDLE_LP11);
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_MIPI_SC_CTRL0_REG,
    OV5648_MIPI_SC_CTRL0_MIPI_LANES(lanes_count) |
    OV5648_MIPI_SC_CTRL0_PHY_LP_RX_PD |
    OV5648_MIPI_SC_CTRL0_MIPI_EN);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_black_level_configure(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_black_level_configure(struct ov5648_sensor *sensor)
    {
    int ret;
// Up to 6 lines are available for black level calibration.
    ret = ov5648_write(sensor, OV5648_BLC_CTRL1_REG,
    OV5648_BLC_CTRL1_START_LINE(2));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_BLC_CTRL2_REG,
    OV5648_BLC_CTRL2_AUTO_EN |
    OV5648_BLC_CTRL2_RESET_FRAME_NUM(5));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_BLC_LINE_NUM_REG,
    OV5648_BLC_LINE_NUM(4));
    if (ret)
    return ret;
    return ov5648_update_bits(sensor, OV5648_BLC_CTRL5_REG,
    OV5648_BLC_CTRL5_UPDATE_EN,
    OV5648_BLC_CTRL5_UPDATE_EN);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_isp_configure(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_isp_configure(struct ov5648_sensor *sensor)
    {
    u8 bits;
    int ret;
// Enable black and white level correction.
    bits = OV5648_ISP_CTRL0_BLACK_CORRECT_EN |
    OV5648_ISP_CTRL0_WHITE_CORRECT_EN;
    ret = ov5648_update_bits(sensor, OV5648_ISP_CTRL0_REG, bits, bits);
    if (ret)
    return ret;
// Enable AWB.
    ret = ov5648_write(sensor, OV5648_ISP_CTRL1_REG,
    OV5648_ISP_CTRL1_AWB_EN);
    if (ret)
    return ret;
// Enable AWB gain and windowing.
    ret = ov5648_write(sensor, OV5648_ISP_CTRL2_REG,
    OV5648_ISP_CTRL2_WIN_EN |
    OV5648_ISP_CTRL2_AWB_GAIN_EN);
    if (ret)
    return ret;
// Enable buffering and auto-binning.
    ret = ov5648_write(sensor, OV5648_ISP_CTRL3_REG,
    OV5648_ISP_CTRL3_BUF_EN |
    OV5648_ISP_CTRL3_BIN_AUTO_EN);
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_ISP_CTRL4_REG, 0);
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_ISP_CTRL1F_REG,
    OV5648_ISP_CTRL1F_OUTPUT_EN);
    if (ret)
    return ret;
// Enable post-binning filters.
    ret = ov5648_write(sensor, OV5648_ISP_CTRL4B_REG,
    OV5648_ISP_CTRL4B_POST_BIN_H_EN |
    OV5648_ISP_CTRL4B_POST_BIN_V_EN);
    if (ret)
    return ret;
// Disable debanding and night mode. Debug bit seems necessary.
    ret = ov5648_write(sensor, OV5648_AEC_CTRL0_REG,
    OV5648_AEC_CTRL0_DEBUG |
    OV5648_AEC_CTRL0_START_SEL_EN);
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_MANUAL_CTRL_REG,
    OV5648_MANUAL_CTRL_FRAME_DELAY(1));
    }
    static unsigned long ov5648_mode_pll1_rate(struct ov5648_sensor *sensor,
    const struct ov5648_pll1_config *config)
    {
    unsigned long xvclk_rate;
    unsigned long pll1_rate;
    xvclk_rate = clk_get_rate(sensor.xvclk);
    pll1_rate = xvclk_rate * config.pll_mul;
    switch (config.pll_pre_div) {
    case 5:
    pll1_rate *= 3;
    pll1_rate /= 2;
    break;
    case 7:
    pll1_rate *= 5;
    pll1_rate /= 2;
    break;
    default:
    pll1_rate /= config.pll_pre_div;
    break;
    }
    return pll1_rate;
    }
    static int ov5648_mode_pll1_configure(struct ov5648_sensor *sensor,
    const struct ov5648_mode *mode,
    u32 mbus_code)
    {
    const struct ov5648_pll1_config *config;
    u8 value;
    int ret;
    value = OV5648_PLL_CTRL0_PLL_CHARGE_PUMP(1);
    switch (mbus_code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    config = mode.pll1_config[0];
    value |= OV5648_PLL_CTRL0_BITS(8);
    break;
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    config = mode.pll1_config[1];
    value |= OV5648_PLL_CTRL0_BITS(10);
    break;
    default:
    return -EINVAL;
    }
    ret = ov5648_write(sensor, OV5648_PLL_CTRL0_REG, value);
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_PLL_DIV_REG,
    OV5648_PLL_DIV_ROOT_DIV(config.root_div) |
    OV5648_PLL_DIV_PLL_PRE_DIV(config.pll_pre_div));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_PLL_MUL_REG,
    OV5648_PLL_MUL(config.pll_mul));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_PLL_CTRL1_REG,
    OV5648_PLL_CTRL1_SYS_DIV(config.sys_div) |
    OV5648_PLL_CTRL1_MIPI_DIV(config.mipi_div));
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_SRB_CTRL_REG,
    OV5648_SRB_CTRL_SCLK_DIV(config.sclk_div) |
    OV5648_SRB_CTRL_SCLK_ARBITER_EN);
    }
    static int ov5648_mode_pll2_configure(struct ov5648_sensor *sensor,
    const struct ov5648_mode *mode)
    {
    const struct ov5648_pll2_config *config = mode.pll2_config;
    int ret;
    ret = ov5648_write(sensor, OV5648_PLLS_DIV_REG,
    OV5648_PLLS_DIV_PLLS_PRE_DIV(config.plls_pre_div) |
    OV5648_PLLS_DIV_PLLS_DIV_R(config.plls_div_r) |
    OV5648_PLLS_DIV_PLLS_SEL_DIV(config.sel_div));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_PLLS_MUL_REG,
    OV5648_PLLS_MUL(config.plls_mul));
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_PLLS_CTRL_REG,
    OV5648_PLLS_CTRL_PLL_CHARGE_PUMP(1) |
    OV5648_PLLS_CTRL_SYS_DIV(config.sys_div));
    }
    static int ov5648_mode_configure(struct ov5648_sensor *sensor,
    const struct ov5648_mode *mode, u32 mbus_code)
    {
    int ret;
// Crop Start X
    ret = ov5648_write(sensor, OV5648_CROP_START_X_H_REG,
    OV5648_CROP_START_X_H(mode.crop_start_x));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_CROP_START_X_L_REG,
    OV5648_CROP_START_X_L(mode.crop_start_x));
    if (ret)
    return ret;
// Offset X
    ret = ov5648_write(sensor, OV5648_OFFSET_X_H_REG,
    OV5648_OFFSET_X_H(mode.offset_x));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_OFFSET_X_L_REG,
    OV5648_OFFSET_X_L(mode.offset_x));
    if (ret)
    return ret;
// Output Size X
    ret = ov5648_write(sensor, OV5648_OUTPUT_SIZE_X_H_REG,
    OV5648_OUTPUT_SIZE_X_H(mode.output_size_x));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_OUTPUT_SIZE_X_L_REG,
    OV5648_OUTPUT_SIZE_X_L(mode.output_size_x));
    if (ret)
    return ret;
// Crop End X
    ret = ov5648_write(sensor, OV5648_CROP_END_X_H_REG,
    OV5648_CROP_END_X_H(mode.crop_end_x));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_CROP_END_X_L_REG,
    OV5648_CROP_END_X_L(mode.crop_end_x));
    if (ret)
    return ret;
// Horizontal Total Size
    ret = ov5648_write(sensor, OV5648_HTS_H_REG, OV5648_HTS_H(mode.hts));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_HTS_L_REG, OV5648_HTS_L(mode.hts));
    if (ret)
    return ret;
// Crop Start Y
    ret = ov5648_write(sensor, OV5648_CROP_START_Y_H_REG,
    OV5648_CROP_START_Y_H(mode.crop_start_y));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_CROP_START_Y_L_REG,
    OV5648_CROP_START_Y_L(mode.crop_start_y));
    if (ret)
    return ret;
// Offset Y
    ret = ov5648_write(sensor, OV5648_OFFSET_Y_H_REG,
    OV5648_OFFSET_Y_H(mode.offset_y));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_OFFSET_Y_L_REG,
    OV5648_OFFSET_Y_L(mode.offset_y));
    if (ret)
    return ret;
// Output Size Y
    ret = ov5648_write(sensor, OV5648_OUTPUT_SIZE_Y_H_REG,
    OV5648_OUTPUT_SIZE_Y_H(mode.output_size_y));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_OUTPUT_SIZE_Y_L_REG,
    OV5648_OUTPUT_SIZE_Y_L(mode.output_size_y));
    if (ret)
    return ret;
// Crop End Y
    ret = ov5648_write(sensor, OV5648_CROP_END_Y_H_REG,
    OV5648_CROP_END_Y_H(mode.crop_end_y));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_CROP_END_Y_L_REG,
    OV5648_CROP_END_Y_L(mode.crop_end_y));
    if (ret)
    return ret;
// Vertical Total Size
    ret = ov5648_write(sensor, OV5648_VTS_H_REG, OV5648_VTS_H(mode.vts));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_VTS_L_REG, OV5648_VTS_L(mode.vts));
    if (ret)
    return ret;
// Flip/Mirror/Binning
//
// A debug bit is enabled by default and needs to be cleared for
// subsampling to work.
//
    ret = ov5648_update_bits(sensor, OV5648_TC20_REG,
    OV5648_TC20_DEBUG |
    OV5648_TC20_BINNING_VERT_EN,
    mode.binning_y ? OV5648_TC20_BINNING_VERT_EN :
    0);
    if (ret)
    return ret;
    ret = ov5648_update_bits(sensor, OV5648_TC21_REG,
    OV5648_TC21_BINNING_HORZ_EN,
    mode.binning_x ? OV5648_TC21_BINNING_HORZ_EN :
    0);
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_SUB_INC_X_REG,
    OV5648_SUB_INC_X_ODD(mode.inc_x_odd) |
    OV5648_SUB_INC_X_EVEN(mode.inc_x_even));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_SUB_INC_Y_REG,
    OV5648_SUB_INC_Y_ODD(mode.inc_y_odd) |
    OV5648_SUB_INC_Y_EVEN(mode.inc_y_even));
    if (ret)
    return ret;
// PLLs
    ret = ov5648_mode_pll1_configure(sensor, mode, mbus_code);
    if (ret)
    return ret;
    ret = ov5648_mode_pll2_configure(sensor, mode);
    if (ret)
    return ret;
// Extra registers
    if (mode.register_values) {
    ret = ov5648_write_sequence(sensor, mode.register_values,
    mode.register_values_count);
    if (ret)
    return ret;
    }
    return 0;
    }
    static unsigned long ov5648_mode_mipi_clk_rate(struct ov5648_sensor *sensor,
    const struct ov5648_mode *mode,
    u32 mbus_code)
    {
    const struct ov5648_pll1_config *config;
    unsigned long pll1_rate;
    switch (mbus_code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    config = mode.pll1_config[0];
    break;
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    config = mode.pll1_config[1];
    break;
    default:
    return 0;
    }
    pll1_rate = ov5648_mode_pll1_rate(sensor, config);
    return pll1_rate / config.sys_div / config.mipi_div / 2;
    }
// Exposure
    static int ov5648_exposure_auto_configure(struct ov5648_sensor *sensor,
    bool enable)
    {
    return ov5648_update_bits(sensor, OV5648_MANUAL_CTRL_REG,
    OV5648_MANUAL_CTRL_AEC_MANUAL_EN,
    enable ? 0 : OV5648_MANUAL_CTRL_AEC_MANUAL_EN);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_exposure_configure(sensor: *mut ov5648_sensor, exposure: u32) -> c_int {
    static int ov5648_exposure_configure(struct ov5648_sensor *sensor, u32 exposure)
    {
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    int ret;
    if (ctrls.exposure_auto.val != V4L2_EXPOSURE_MANUAL)
    return -EINVAL;
    ret = ov5648_write(sensor, OV5648_EXPOSURE_CTRL_HH_REG,
    OV5648_EXPOSURE_CTRL_HH(exposure));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_EXPOSURE_CTRL_H_REG,
    OV5648_EXPOSURE_CTRL_H(exposure));
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_EXPOSURE_CTRL_L_REG,
    OV5648_EXPOSURE_CTRL_L(exposure));
    }
    static int ov5648_exposure_value(struct ov5648_sensor *sensor,
    u32 *exposure)
    {
    let mut exposure_hh: u8 = 0, exposure_h = 0, exposure_l = 0;
    int ret;
    ret = ov5648_read(sensor, OV5648_EXPOSURE_CTRL_HH_REG, &exposure_hh);
    if (ret)
    return ret;
    ret = ov5648_read(sensor, OV5648_EXPOSURE_CTRL_H_REG, &exposure_h);
    if (ret)
    return ret;
    ret = ov5648_read(sensor, OV5648_EXPOSURE_CTRL_L_REG, &exposure_l);
    if (ret)
    return ret;
// exposure = OV5648_EXPOSURE_CTRL_HH_VALUE((u32)exposure_hh) |
    OV5648_EXPOSURE_CTRL_H_VALUE((u32)exposure_h) |
    OV5648_EXPOSURE_CTRL_L_VALUE((u32)exposure_l);
    return 0;
    }
// Gain
#[no_mangle]
unsafe extern "C" fn ov5648_gain_auto_configure(sensor: *mut ov5648_sensor, enable: bool) -> c_int {
    static int ov5648_gain_auto_configure(struct ov5648_sensor *sensor, bool enable)
    {
    return ov5648_update_bits(sensor, OV5648_MANUAL_CTRL_REG,
    OV5648_MANUAL_CTRL_AGC_MANUAL_EN,
    enable ? 0 : OV5648_MANUAL_CTRL_AGC_MANUAL_EN);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_gain_configure(sensor: *mut ov5648_sensor, gain: u32) -> c_int {
    static int ov5648_gain_configure(struct ov5648_sensor *sensor, u32 gain)
    {
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    int ret;
    if (ctrls.gain_auto.val)
    return -EINVAL;
    ret = ov5648_write(sensor, OV5648_GAIN_CTRL_H_REG,
    OV5648_GAIN_CTRL_H(gain));
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_GAIN_CTRL_L_REG,
    OV5648_GAIN_CTRL_L(gain));
    }
#[no_mangle]
unsafe extern "C" fn ov5648_gain_value(sensor: *mut ov5648_sensor, gain: *mut u32) -> c_int {
    static int ov5648_gain_value(struct ov5648_sensor *sensor, u32 *gain)
    {
    let mut gain_h: u8 = 0, gain_l = 0;
    int ret;
    ret = ov5648_read(sensor, OV5648_GAIN_CTRL_H_REG, &gain_h);
    if (ret)
    return ret;
    ret = ov5648_read(sensor, OV5648_GAIN_CTRL_L_REG, &gain_l);
    if (ret)
    return ret;
// gain = OV5648_GAIN_CTRL_H_VALUE((u32)gain_h) |
    OV5648_GAIN_CTRL_L_VALUE((u32)gain_l);
    return 0;
    }
// White Balance
    static int ov5648_white_balance_auto_configure(struct ov5648_sensor *sensor,
    bool enable)
    {
    return ov5648_write(sensor, OV5648_AWB_CTRL_REG,
    enable ? 0 : OV5648_AWB_CTRL_GAIN_MANUAL_EN);
    }
    static int ov5648_white_balance_configure(struct ov5648_sensor *sensor,
    u32 red_balance, u32 blue_balance)
    {
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    int ret;
    if (ctrls.white_balance_auto.val)
    return -EINVAL;
    ret = ov5648_write(sensor, OV5648_GAIN_RED_MAN_H_REG,
    OV5648_GAIN_RED_MAN_H(red_balance));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_GAIN_RED_MAN_L_REG,
    OV5648_GAIN_RED_MAN_L(red_balance));
    if (ret)
    return ret;
    ret = ov5648_write(sensor, OV5648_GAIN_BLUE_MAN_H_REG,
    OV5648_GAIN_BLUE_MAN_H(blue_balance));
    if (ret)
    return ret;
    return ov5648_write(sensor, OV5648_GAIN_BLUE_MAN_L_REG,
    OV5648_GAIN_BLUE_MAN_L(blue_balance));
    }
// Flip
#[no_mangle]
unsafe extern "C" fn ov5648_flip_vert_configure(sensor: *mut ov5648_sensor, enable: bool) -> c_int {
    static int ov5648_flip_vert_configure(struct ov5648_sensor *sensor, bool enable)
    {
    u8 bits = OV5648_TC20_FLIP_VERT_ISP_EN |
    OV5648_TC20_FLIP_VERT_SENSOR_EN;
    return ov5648_update_bits(sensor, OV5648_TC20_REG, bits,
    enable ? bits : 0);
    }
#[no_mangle]
unsafe extern "C" fn ov5648_flip_horz_configure(sensor: *mut ov5648_sensor, enable: bool) -> c_int {
    static int ov5648_flip_horz_configure(struct ov5648_sensor *sensor, bool enable)
    {
    u8 bits = OV5648_TC21_FLIP_HORZ_ISP_EN |
    OV5648_TC21_FLIP_HORZ_SENSOR_EN;
    return ov5648_update_bits(sensor, OV5648_TC21_REG, bits,
    enable ? bits : 0);
    }
// Test Pattern
    static int ov5648_test_pattern_configure(struct ov5648_sensor *sensor,
    unsigned int index)
    {
    if (index >= ARRAY_SIZE(ov5648_test_pattern_bits))
    return -EINVAL;
    return ov5648_write(sensor, OV5648_ISP_CTRL3D_REG,
    ov5648_test_pattern_bits[index]);
    }
// State
    static int ov5648_state_mipi_configure(struct ov5648_sensor *sensor,
    const struct ov5648_mode *mode,
    u32 mbus_code)
    {
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    struct v4l2_mbus_config_mipi_csi2 *bus_mipi_csi2 =
    &sensor.endpoint.bus.mipi_csi2;
    unsigned long mipi_clk_rate;
    unsigned int bits_per_sample;
    unsigned int lanes_count;
    unsigned int i, j;
    s64 mipi_pixel_rate;
    mipi_clk_rate = ov5648_mode_mipi_clk_rate(sensor, mode, mbus_code);
    if (!mipi_clk_rate)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(ov5648_link_freq_menu); i++) {
    let mut freq: i64 = ov5648_link_freq_menu[i];
    if (freq == mipi_clk_rate)
    break;
    }
    for (j = 0; j < sensor.endpoint.nr_of_link_frequencies; j++) {
    let mut freq: u64 = sensor.endpoint.link_frequencies[j];
    if (freq == mipi_clk_rate)
    break;
    }
    if (i == ARRAY_SIZE(ov5648_link_freq_menu)) {
    dev_err(sensor.dev,
    "failed to find %lu clk rate in link freq\n",
    mipi_clk_rate);
    } else if (j == sensor.endpoint.nr_of_link_frequencies) {
    dev_err(sensor.dev,
    "failed to find %lu clk rate in endpoint link-frequencies\n",
    mipi_clk_rate);
    } else {
    __v4l2_ctrl_s_ctrl(ctrls.link_freq, i);
    }
    switch (mbus_code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    bits_per_sample = 8;
    break;
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    bits_per_sample = 10;
    break;
    default:
    return -EINVAL;
    }
    lanes_count = bus_mipi_csi2.num_data_lanes;
    mipi_pixel_rate = mipi_clk_rate * 2 * lanes_count / bits_per_sample;
    __v4l2_ctrl_s_ctrl_int64(ctrls.pixel_rate, mipi_pixel_rate);
    return 0;
    }
    static int ov5648_state_configure(struct ov5648_sensor *sensor,
    const struct ov5648_mode *mode,
    u32 mbus_code)
    {
    int ret;
    if (sensor.state.streaming)
    return -EBUSY;
// State will be configured at first power on otherwise.
    if (pm_runtime_enabled(sensor.dev) &&
    !pm_runtime_suspended(sensor.dev)) {
    ret = ov5648_mode_configure(sensor, mode, mbus_code);
    if (ret)
    return ret;
    }
    ret = ov5648_state_mipi_configure(sensor, mode, mbus_code);
    if (ret)
    return ret;
    sensor.state.mode = mode;
    sensor.state.mbus_code = mbus_code;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_state_init(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_state_init(struct ov5648_sensor *sensor)
    {
    int ret;
    mutex_lock(&sensor.mutex);
    ret = ov5648_state_configure(sensor, &ov5648_modes[0],
    ov5648_mbus_codes[0]);
    mutex_unlock(&sensor.mutex);
    return ret;
    }
// Sensor Base
#[no_mangle]
unsafe extern "C" fn ov5648_sensor_init(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_sensor_init(struct ov5648_sensor *sensor)
    {
    int ret;
    ret = ov5648_sw_reset(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to perform sw reset\n");
    return ret;
    }
    ret = ov5648_sw_standby(sensor, 1);
    if (ret) {
    dev_err(sensor.dev, "failed to set sensor standby\n");
    return ret;
    }
    ret = ov5648_chip_id_check(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to check sensor chip id\n");
    return ret;
    }
    ret = ov5648_avdd_internal_power(sensor, !sensor.avdd);
    if (ret) {
    dev_err(sensor.dev, "failed to set internal avdd power\n");
    return ret;
    }
    ret = ov5648_write_sequence(sensor, ov5648_init_sequence,
    ARRAY_SIZE(ov5648_init_sequence));
    if (ret) {
    dev_err(sensor.dev, "failed to write init sequence\n");
    return ret;
    }
    ret = ov5648_pad_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure pad\n");
    return ret;
    }
    ret = ov5648_mipi_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure MIPI\n");
    return ret;
    }
    ret = ov5648_isp_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure ISP\n");
    return ret;
    }
    ret = ov5648_black_level_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure black level\n");
    return ret;
    }
// Configure current mode.
    ret = ov5648_state_configure(sensor, sensor.state.mode,
    sensor.state.mbus_code);
    if (ret) {
    dev_err(sensor.dev, "failed to configure state\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_sensor_power(sensor: *mut ov5648_sensor, on: bool) -> c_int {
    static int ov5648_sensor_power(struct ov5648_sensor *sensor, bool on)
    {
// Keep initialized to zero for disable label.
    let mut ret: c_int = 0;
//
// General notes about the power sequence:
// - power-down GPIO must be active (low) during power-on;
// - reset GPIO state does not matter during power-on;
// - XVCLK must be provided 1 ms before register access;
// - 10 ms are needed between power-down deassert and register access.
//
// Note that regulator-and-GPIO-based power is untested.
    if (on) {
    gpiod_set_value_cansleep(sensor.reset, 1);
    gpiod_set_value_cansleep(sensor.powerdown, 1);
    ret = regulator_enable(sensor.dovdd);
    if (ret) {
    dev_err(sensor.dev,
    "failed to enable DOVDD regulator\n");
    goto disable;
    }
    if (sensor.avdd) {
    ret = regulator_enable(sensor.avdd);
    if (ret) {
    dev_err(sensor.dev,
    "failed to enable AVDD regulator\n");
    goto disable;
    }
    }
    ret = regulator_enable(sensor.dvdd);
    if (ret) {
    dev_err(sensor.dev,
    "failed to enable DVDD regulator\n");
    goto disable;
    }
// According to OV5648 power up diagram.
    usleep_range(5000, 10000);
    ret = clk_prepare_enable(sensor.xvclk);
    if (ret) {
    dev_err(sensor.dev, "failed to enable XVCLK clock\n");
    goto disable;
    }
    gpiod_set_value_cansleep(sensor.reset, 0);
    gpiod_set_value_cansleep(sensor.powerdown, 0);
    usleep_range(20000, 25000);
    } else {
    disable:
    gpiod_set_value_cansleep(sensor.powerdown, 1);
    gpiod_set_value_cansleep(sensor.reset, 1);
    clk_disable_unprepare(sensor.xvclk);
    regulator_disable(sensor.dvdd);
    if (sensor.avdd)
    regulator_disable(sensor.avdd);
    regulator_disable(sensor.dovdd);
    }
    return ret;
    }
// Controls
#[no_mangle]
unsafe extern "C" fn ov5648_g_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov5648_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *subdev = ov5648_ctrl_subdev(ctrl);
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    int ret;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE_AUTO:
    ret = ov5648_exposure_value(sensor, &ctrls.exposure.val);
    if (ret)
    return ret;
    break;
    case V4L2_CID_AUTOGAIN:
    ret = ov5648_gain_value(sensor, &ctrls.gain.val);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov5648_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *subdev = ov5648_ctrl_subdev(ctrl);
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    unsigned int index;
    bool enable;
    int ret;
// Wait for the sensor to be on before setting controls.
    if (pm_runtime_suspended(sensor.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE_AUTO:
    enable = ctrl.val == V4L2_EXPOSURE_AUTO;
    ret = ov5648_exposure_auto_configure(sensor, enable);
    if (ret)
    return ret;
    if (!enable && ctrls.exposure.is_new) {
    ret = ov5648_exposure_configure(sensor,
    ctrls.exposure.val);
    if (ret)
    return ret;
    }
    break;
    case V4L2_CID_AUTOGAIN:
    enable = !!ctrl.val;
    ret = ov5648_gain_auto_configure(sensor, enable);
    if (ret)
    return ret;
    if (!enable) {
    ret = ov5648_gain_configure(sensor, ctrls.gain.val);
    if (ret)
    return ret;
    }
    break;
    case V4L2_CID_AUTO_WHITE_BALANCE:
    enable = !!ctrl.val;
    ret = ov5648_white_balance_auto_configure(sensor, enable);
    if (ret)
    return ret;
    if (!enable) {
    ret = ov5648_white_balance_configure(sensor,
    ctrls.red_balance.val,
    ctrls.blue_balance.val);
    if (ret)
    return ret;
    }
    break;
    case V4L2_CID_HFLIP:
    enable = !!ctrl.val;
    return ov5648_flip_horz_configure(sensor, enable);
    case V4L2_CID_VFLIP:
    enable = !!ctrl.val;
    return ov5648_flip_vert_configure(sensor, enable);
    case V4L2_CID_TEST_PATTERN:
    index = (unsigned int)ctrl.val;
    return ov5648_test_pattern_configure(sensor, index);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct v4l2_ctrl_ops ov5648_ctrl_ops = {
    .g_volatile_ctrl	= ov5648_g_volatile_ctrl,
    .s_ctrl			= ov5648_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov5648_ctrls_init(sensor: *mut ov5648_sensor) -> c_int {
    static int ov5648_ctrls_init(struct ov5648_sensor *sensor)
    {
    struct ov5648_ctrls *ctrls = &sensor.ctrls;
    struct v4l2_ctrl_handler *handler = &ctrls.handler;
    const struct v4l2_ctrl_ops *ops = &ov5648_ctrl_ops;
    int ret;
    v4l2_ctrl_handler_init(handler, 32);
// Use our mutex for ctrl locking.
    handler.lock = &sensor.mutex;
// Exposure
    ctrls.exposure_auto = v4l2_ctrl_new_std_menu(handler, ops,
    V4L2_CID_EXPOSURE_AUTO,
    V4L2_EXPOSURE_MANUAL, 0,
    V4L2_EXPOSURE_AUTO);
    ctrls.exposure = v4l2_ctrl_new_std(handler, ops, V4L2_CID_EXPOSURE,
    16, 1048575, 16, 512);
    v4l2_ctrl_auto_cluster(2, &ctrls.exposure_auto, 1, true);
// Gain
    ctrls.gain_auto =
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_AUTOGAIN, 0, 1, 1, 1);
    ctrls.gain = v4l2_ctrl_new_std(handler, ops, V4L2_CID_GAIN, 16, 1023,
    16, 16);
    v4l2_ctrl_auto_cluster(2, &ctrls.gain_auto, 0, true);
// White Balance
    ctrls.white_balance_auto =
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_AUTO_WHITE_BALANCE, 0,
    1, 1, 1);
    ctrls.red_balance = v4l2_ctrl_new_std(handler, ops,
    V4L2_CID_RED_BALANCE, 0, 4095,
    1, 1024);
    ctrls.blue_balance = v4l2_ctrl_new_std(handler, ops,
    V4L2_CID_BLUE_BALANCE, 0, 4095,
    1, 1024);
    v4l2_ctrl_auto_cluster(3, &ctrls.white_balance_auto, 0, false);
// Flip
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_VFLIP, 0, 1, 1, 0);
// Test Pattern
    v4l2_ctrl_new_std_menu_items(handler, ops, V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov5648_test_pattern_menu) - 1,
    0, 0, ov5648_test_pattern_menu);
// MIPI CSI-2
    ctrls.link_freq =
    v4l2_ctrl_new_int_menu(handler, core::ptr::null_mut(), V4L2_CID_LINK_FREQ,
    ARRAY_SIZE(ov5648_link_freq_menu) - 1,
    0, ov5648_link_freq_menu);
    ctrls.pixel_rate =
    v4l2_ctrl_new_std(handler, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE, 1,
    INT_MAX, 1, 1);
    if (handler.error) {
    ret = handler.error;
    goto error_ctrls;
    }
    ctrls.exposure.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrls.gain.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrls.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    ctrls.pixel_rate.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    sensor.subdev.ctrl_handler = handler;
    return 0;
    error_ctrls:
    v4l2_ctrl_handler_free(handler);
    return ret;
    }
// Subdev Video Operations
#[no_mangle]
unsafe extern "C" fn ov5648_s_stream(subdev: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ov5648_s_stream(struct v4l2_subdev *subdev, int enable)
    {
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct ov5648_state *state = &sensor.state;
    int ret;
    if (enable) {
    ret = pm_runtime_resume_and_get(sensor.dev);
    if (ret < 0)
    return ret;
    }
    mutex_lock(&sensor.mutex);
    ret = ov5648_sw_standby(sensor, !enable);
    mutex_unlock(&sensor.mutex);
    if (ret)
    return ret;
    state.streaming = !!enable;
    if (!enable)
    pm_runtime_put(sensor.dev);
    return 0;
    }
    static const struct v4l2_subdev_video_ops ov5648_subdev_video_ops = {
    .s_stream		= ov5648_s_stream,
    };
// Subdev Pad Operations
    static int ov5648_enum_mbus_code(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code_enum)
    {
    if (code_enum.index >= ARRAY_SIZE(ov5648_mbus_codes))
    return -EINVAL;
    code_enum.code = ov5648_mbus_codes[code_enum.index];
    return 0;
    }
    static void ov5648_mbus_format_fill(struct v4l2_mbus_framefmt *mbus_format,
    u32 mbus_code,
    const struct ov5648_mode *mode)
    {
    mbus_format.width = mode.output_size_x;
    mbus_format.height = mode.output_size_y;
    mbus_format.code = mbus_code;
    mbus_format.field = V4L2_FIELD_NONE;
    mbus_format.colorspace = V4L2_COLORSPACE_RAW;
    mbus_format.ycbcr_enc =
    V4L2_MAP_YCBCR_ENC_DEFAULT(mbus_format.colorspace);
    mbus_format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    mbus_format.xfer_func =
    V4L2_MAP_XFER_FUNC_DEFAULT(mbus_format.colorspace);
    }
    static int ov5648_get_fmt(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct v4l2_mbus_framefmt *mbus_format = &format.format;
    mutex_lock(&sensor.mutex);
    if (format.which == V4L2_SUBDEV_FORMAT_TRY)
// mbus_format = *v4l2_subdev_state_get_format(sd_state,
    format.pad);
    else
    ov5648_mbus_format_fill(mbus_format, sensor.state.mbus_code,
    sensor.state.mode);
    mutex_unlock(&sensor.mutex);
    return 0;
    }
    static int ov5648_set_fmt(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct v4l2_mbus_framefmt *mbus_format = &format.format;
    const struct ov5648_mode *mode;
    let mut mbus_code: u32 = 0;
    unsigned int index;
    let mut ret: c_int = 0;
    mutex_lock(&sensor.mutex);
    if (sensor.state.streaming) {
    ret = -EBUSY;
    goto complete;
    }
// Try to find requested mbus code.
    for (index = 0; index < ARRAY_SIZE(ov5648_mbus_codes); index++) {
    if (ov5648_mbus_codes[index] == mbus_format.code) {
    mbus_code = mbus_format.code;
    break;
    }
    }
// Fallback to default.
    if (!mbus_code)
    mbus_code = ov5648_mbus_codes[0];
// Find the mode with nearest dimensions.
    mode = v4l2_find_nearest_size(ov5648_modes, ARRAY_SIZE(ov5648_modes),
    output_size_x, output_size_y,
    mbus_format.width, mbus_format.height);
    if (!mode) {
    ret = -EINVAL;
    goto complete;
    }
    ov5648_mbus_format_fill(mbus_format, mbus_code, mode);
    if (format.which == V4L2_SUBDEV_FORMAT_TRY)
// v4l2_subdev_state_get_format(sd_state, format->pad) =
// mbus_format;
    else if (sensor.state.mode != mode ||
    sensor.state.mbus_code != mbus_code)
    ret = ov5648_state_configure(sensor, mode, mbus_code);
    complete:
    mutex_unlock(&sensor.mutex);
    return ret;
    }
    static int ov5648_get_frame_interval(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *interval)
    {
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    const struct ov5648_mode *mode;
    let mut ret: c_int = 0;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (interval.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    mutex_lock(&sensor.mutex);
    mode = sensor.state.mode;
    switch (sensor.state.mbus_code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    interval.interval = mode.frame_interval[0];
    break;
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    interval.interval = mode.frame_interval[1];
    break;
    default:
    ret = -EINVAL;
    }
    mutex_unlock(&sensor.mutex);
    return ret;
    }
    static int ov5648_enum_frame_size(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *size_enum)
    {
    const struct ov5648_mode *mode;
    if (size_enum.index >= ARRAY_SIZE(ov5648_modes))
    return -EINVAL;
    mode = &ov5648_modes[size_enum.index];
    size_enum.min_width = size_enum.max_width = mode.output_size_x;
    size_enum.min_height = size_enum.max_height = mode.output_size_y;
    return 0;
    }
    static int ov5648_enum_frame_interval(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval_enum *interval_enum)
    {
    const struct ov5648_mode *mode = core::ptr::null_mut();
    unsigned int mode_index;
    unsigned int interval_index;
    if (interval_enum.index > 0)
    return -EINVAL;
//
// Multiple modes with the same dimensions may have different frame
// intervals, so look up each relevant mode.
//
    for (mode_index = 0, interval_index = 0;
    mode_index < ARRAY_SIZE(ov5648_modes); mode_index++) {
    mode = &ov5648_modes[mode_index];
    if (mode.output_size_x == interval_enum.width &&
    mode.output_size_y == interval_enum.height) {
    if (interval_index == interval_enum.index)
    break;
    interval_index++;
    }
    }
    if (mode_index == ARRAY_SIZE(ov5648_modes))
    return -EINVAL;
    switch (interval_enum.code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    interval_enum.interval = mode.frame_interval[0];
    break;
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    interval_enum.interval = mode.frame_interval[1];
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct v4l2_subdev_pad_ops ov5648_subdev_pad_ops = {
    .enum_mbus_code		= ov5648_enum_mbus_code,
    .get_fmt		= ov5648_get_fmt,
    .set_fmt		= ov5648_set_fmt,
    .get_frame_interval	= ov5648_get_frame_interval,
    .set_frame_interval	= ov5648_get_frame_interval,
    .enum_frame_size	= ov5648_enum_frame_size,
    .enum_frame_interval	= ov5648_enum_frame_interval,
    };
    static const struct v4l2_subdev_ops ov5648_subdev_ops = {
    .video		= &ov5648_subdev_video_ops,
    .pad		= &ov5648_subdev_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn ov5648_suspend(dev: *mut device) -> c_int {
    static int ov5648_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct ov5648_state *state = &sensor.state;
    let mut ret: c_int = 0;
    mutex_lock(&sensor.mutex);
    if (state.streaming) {
    ret = ov5648_sw_standby(sensor, true);
    if (ret)
    goto complete;
    }
    ret = ov5648_sensor_power(sensor, false);
    if (ret)
    ov5648_sw_standby(sensor, false);
    complete:
    mutex_unlock(&sensor.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_resume(dev: *mut device) -> c_int {
    static int ov5648_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    struct ov5648_state *state = &sensor.state;
    let mut ret: c_int = 0;
    mutex_lock(&sensor.mutex);
    ret = ov5648_sensor_power(sensor, true);
    if (ret)
    goto complete;
    ret = ov5648_sensor_init(sensor);
    if (ret)
    goto error_power;
    ret = __v4l2_ctrl_handler_setup(&sensor.ctrls.handler);
    if (ret)
    goto error_power;
    if (state.streaming) {
    ret = ov5648_sw_standby(sensor, false);
    if (ret)
    goto error_power;
    }
    goto complete;
    error_power:
    ov5648_sensor_power(sensor, false);
    complete:
    mutex_unlock(&sensor.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_probe(client: *mut i2c_client) -> c_int {
    static int ov5648_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct fwnode_handle *handle;
    struct ov5648_sensor *sensor;
    struct v4l2_subdev *subdev;
    struct media_pad *pad;
    unsigned long rate;
    int ret;
    sensor = devm_kzalloc(dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.dev = dev;
    sensor.i2c_client = client;
// Graph Endpoint
    handle = fwnode_graph_get_next_endpoint(dev_fwnode(dev), core::ptr::null_mut());
    if (!handle) {
    dev_err(dev, "unable to find endpoint node\n");
    return -EINVAL;
    }
    sensor.endpoint.bus_type = V4L2_MBUS_CSI2_DPHY;
    ret = v4l2_fwnode_endpoint_alloc_parse(handle, &sensor.endpoint);
    fwnode_handle_put(handle);
    if (ret) {
    dev_err(dev, "failed to parse endpoint node\n");
    return ret;
    }
// GPIOs
    sensor.powerdown = devm_gpiod_get_optional(dev, "powerdown",
    GPIOD_OUT_HIGH);
    if (IS_ERR(sensor.powerdown)) {
    ret = PTR_ERR(sensor.powerdown);
    goto error_endpoint;
    }
    sensor.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(sensor.reset)) {
    ret = PTR_ERR(sensor.reset);
    goto error_endpoint;
    }
// Regulators
// DVDD: digital core
    sensor.dvdd = devm_regulator_get(dev, "dvdd");
    if (IS_ERR(sensor.dvdd)) {
    dev_err(dev, "cannot get DVDD (digital core) regulator\n");
    ret = PTR_ERR(sensor.dvdd);
    goto error_endpoint;
    }
// DOVDD: digital I/O
    sensor.dovdd = devm_regulator_get(dev, "dovdd");
    if (IS_ERR(sensor.dovdd)) {
    dev_err(dev, "cannot get DOVDD (digital I/O) regulator\n");
    ret = PTR_ERR(sensor.dovdd);
    goto error_endpoint;
    }
// AVDD: analog
    sensor.avdd = devm_regulator_get_optional(dev, "avdd");
    if (IS_ERR(sensor.avdd)) {
    dev_info(dev, "no AVDD regulator provided, using internal\n");
    sensor.avdd = core::ptr::null_mut();
    }
// External Clock
    sensor.xvclk = devm_v4l2_sensor_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(sensor.xvclk)) {
    ret = dev_err_probe(dev, PTR_ERR(sensor.xvclk),
    "failed to get external clock\n");
    goto error_endpoint;
    }
    rate = clk_get_rate(sensor.xvclk);
    if (rate != OV5648_XVCLK_RATE) {
    dev_err(dev, "clock rate %lu Hz is unsupported\n", rate);
    ret = -EINVAL;
    goto error_endpoint;
    }
// Subdev, entity and pad
    subdev = &sensor.subdev;
    v4l2_i2c_subdev_init(subdev, client, &ov5648_subdev_ops);
    subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    pad = &sensor.pad;
    pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&subdev.entity, 1, pad);
    if (ret)
    goto error_entity;
// Mutex
    mutex_init(&sensor.mutex);
// Sensor
    ret = ov5648_ctrls_init(sensor);
    if (ret)
    goto error_mutex;
    ret = ov5648_state_init(sensor);
    if (ret)
    goto error_ctrls;
// Runtime PM
    pm_runtime_enable(sensor.dev);
    pm_runtime_set_suspended(sensor.dev);
// V4L2 subdev register
    ret = v4l2_async_register_subdev_sensor(subdev);
    if (ret)
    goto error_pm;
    return 0;
    error_pm:
    pm_runtime_disable(sensor.dev);
    error_ctrls:
    v4l2_ctrl_handler_free(&sensor.ctrls.handler);
    error_mutex:
    mutex_destroy(&sensor.mutex);
    error_entity:
    media_entity_cleanup(&sensor.subdev.entity);
    error_endpoint:
    v4l2_fwnode_endpoint_free(&sensor.endpoint);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5648_remove(client: *mut i2c_client) {
    static void ov5648_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ov5648_sensor *sensor = ov5648_subdev_sensor(subdev);
    v4l2_async_unregister_subdev(subdev);
    pm_runtime_disable(sensor.dev);
    v4l2_ctrl_handler_free(&sensor.ctrls.handler);
    mutex_destroy(&sensor.mutex);
    media_entity_cleanup(&subdev.entity);
    v4l2_fwnode_endpoint_free(&sensor.endpoint);
    }
    static const struct dev_pm_ops ov5648_pm_ops = {
    SET_RUNTIME_PM_OPS(ov5648_suspend, ov5648_resume, core::ptr::null_mut())
    };
    static const struct of_device_id ov5648_of_match[] = {
    { .compatible = "ovti,ov5648" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ov5648_of_match);
    static struct i2c_driver ov5648_driver = {
    .driver = {
    .name = "ov5648",
    .of_match_table = ov5648_of_match,
    .pm = &ov5648_pm_ops,
    },
    .probe = ov5648_probe,
    .remove = ov5648_remove,
    };
    module_i2c_driver(ov5648_driver);
    MODULE_AUTHOR("Paul Kocialkowski <paul.kocialkowski@bootlin.com>");
    MODULE_DESCRIPTION("V4L2 driver for the OmniVision OV5648 image sensor");
    MODULE_LICENSE("GPL v2");
