//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov8865.c
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
// Copyright 2020 Kévin L'hôpital <kevin.lhopital@bootlin.com>
// Copyright 2020 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

// Register definitions
// System
pub const OV8865_SW_STANDBY_REG: c_uint = 0x100;

pub const OV8865_SW_RESET_REG: c_uint = 0x103;

pub const OV8865_PLL_CTRL0_REG: c_uint = 0x300;

pub const OV8865_PLL_CTRL1_REG: c_uint = 0x301;

pub const OV8865_PLL_CTRL2_REG: c_uint = 0x302;

pub const OV8865_PLL_CTRL3_REG: c_uint = 0x303;

pub const OV8865_PLL_CTRL4_REG: c_uint = 0x304;

pub const OV8865_PLL_CTRL5_REG: c_uint = 0x305;

pub const OV8865_PLL_CTRL6_REG: c_uint = 0x306;

pub const OV8865_PLL_CTRL8_REG: c_uint = 0x308;
pub const OV8865_PLL_CTRL9_REG: c_uint = 0x309;
pub const OV8865_PLL_CTRLA_REG: c_uint = 0x30a;

pub const OV8865_PLL_CTRLB_REG: c_uint = 0x30b;

pub const OV8865_PLL_CTRLC_REG: c_uint = 0x30c;

pub const OV8865_PLL_CTRLD_REG: c_uint = 0x30d;

pub const OV8865_PLL_CTRLE_REG: c_uint = 0x30e;

pub const OV8865_PLL_CTRLF_REG: c_uint = 0x30f;

pub const OV8865_PLL_CTRL10_REG: c_uint = 0x310;
pub const OV8865_PLL_CTRL11_REG: c_uint = 0x311;
pub const OV8865_PLL_CTRL12_REG: c_uint = 0x312;

pub const OV8865_PLL_CTRL1B_REG: c_uint = 0x31b;
pub const OV8865_PLL_CTRL1C_REG: c_uint = 0x31c;
pub const OV8865_PLL_CTRL1E_REG: c_uint = 0x31e;

pub const OV8865_PAD_OEN0_REG: c_uint = 0x3000;
pub const OV8865_PAD_OEN2_REG: c_uint = 0x3002;
pub const OV8865_CLK_RST5_REG: c_uint = 0x3005;
pub const OV8865_CHIP_ID_HH_REG: c_uint = 0x300a;
pub const OV8865_CHIP_ID_HH_VALUE: c_uint = 0x00;
pub const OV8865_CHIP_ID_H_REG: c_uint = 0x300b;
pub const OV8865_CHIP_ID_H_VALUE: c_uint = 0x88;
pub const OV8865_CHIP_ID_L_REG: c_uint = 0x300c;
pub const OV8865_CHIP_ID_L_VALUE: c_uint = 0x65;
pub const OV8865_PAD_OUT2_REG: c_uint = 0x300d;
pub const OV8865_PAD_SEL2_REG: c_uint = 0x3010;
pub const OV8865_PAD_PK_REG: c_uint = 0x3011;

pub const OV8865_PUMP_CLK_DIV_REG: c_uint = 0x3015;

pub const OV8865_MIPI_SC_CTRL0_REG: c_uint = 0x3018;

    GENMASK(7, 5))

pub const OV8865_MIPI_SC_CTRL1_REG: c_uint = 0x3019;
pub const OV8865_CLK_RST0_REG: c_uint = 0x301a;
pub const OV8865_CLK_RST1_REG: c_uint = 0x301b;
pub const OV8865_CLK_RST2_REG: c_uint = 0x301c;
pub const OV8865_CLK_RST3_REG: c_uint = 0x301d;
pub const OV8865_CLK_RST4_REG: c_uint = 0x301e;
pub const OV8865_PCLK_SEL_REG: c_uint = 0x3020;

pub const OV8865_MISC_CTRL_REG: c_uint = 0x3021;
pub const OV8865_MIPI_SC_CTRL2_REG: c_uint = 0x3022;

pub const OV8865_MIPI_BIT_SEL_REG: c_uint = 0x3031;

pub const OV8865_CLK_SEL0_REG: c_uint = 0x3032;

pub const OV8865_CLK_SEL1_REG: c_uint = 0x3033;

pub const OV8865_SCLK_CTRL_REG: c_uint = 0x3106;

// Exposure/gain
pub const OV8865_EXPOSURE_CTRL_HH_REG: c_uint = 0x3500;

pub const OV8865_EXPOSURE_CTRL_H_REG: c_uint = 0x3501;

pub const OV8865_EXPOSURE_CTRL_L_REG: c_uint = 0x3502;

pub const OV8865_EXPOSURE_GAIN_MANUAL_REG: c_uint = 0x3503;
pub const OV8865_INTEGRATION_TIME_MARGIN: c_int = 8;
pub const OV8865_GAIN_CTRL_H_REG: c_uint = 0x3508;

pub const OV8865_GAIN_CTRL_L_REG: c_uint = 0x3509;

// Timing
pub const OV8865_CROP_START_X_H_REG: c_uint = 0x3800;

pub const OV8865_CROP_START_X_L_REG: c_uint = 0x3801;

pub const OV8865_CROP_START_Y_H_REG: c_uint = 0x3802;

pub const OV8865_CROP_START_Y_L_REG: c_uint = 0x3803;

pub const OV8865_CROP_END_X_H_REG: c_uint = 0x3804;

pub const OV8865_CROP_END_X_L_REG: c_uint = 0x3805;

pub const OV8865_CROP_END_Y_H_REG: c_uint = 0x3806;

pub const OV8865_CROP_END_Y_L_REG: c_uint = 0x3807;

pub const OV8865_OUTPUT_SIZE_X_H_REG: c_uint = 0x3808;

pub const OV8865_OUTPUT_SIZE_X_L_REG: c_uint = 0x3809;

pub const OV8865_OUTPUT_SIZE_Y_H_REG: c_uint = 0x380a;

pub const OV8865_OUTPUT_SIZE_Y_L_REG: c_uint = 0x380b;

pub const OV8865_HTS_H_REG: c_uint = 0x380c;

pub const OV8865_HTS_L_REG: c_uint = 0x380d;

pub const OV8865_VTS_H_REG: c_uint = 0x380e;

pub const OV8865_VTS_L_REG: c_uint = 0x380f;

pub const OV8865_TIMING_MAX_VTS: c_uint = 0xffff;
pub const OV8865_TIMING_MIN_VTS: c_uint = 0x04;
pub const OV8865_OFFSET_X_H_REG: c_uint = 0x3810;

pub const OV8865_OFFSET_X_L_REG: c_uint = 0x3811;

pub const OV8865_OFFSET_Y_H_REG: c_uint = 0x3812;

pub const OV8865_OFFSET_Y_L_REG: c_uint = 0x3813;

pub const OV8865_INC_X_ODD_REG: c_uint = 0x3814;

pub const OV8865_INC_X_EVEN_REG: c_uint = 0x3815;

pub const OV8865_VSYNC_START_H_REG: c_uint = 0x3816;

pub const OV8865_VSYNC_START_L_REG: c_uint = 0x3817;

pub const OV8865_VSYNC_END_H_REG: c_uint = 0x3818;

pub const OV8865_VSYNC_END_L_REG: c_uint = 0x3819;

pub const OV8865_HSYNC_FIRST_H_REG: c_uint = 0x381a;

pub const OV8865_HSYNC_FIRST_L_REG: c_uint = 0x381b;

pub const OV8865_FORMAT1_REG: c_uint = 0x3820;

pub const OV8865_FORMAT2_REG: c_uint = 0x3821;

pub const OV8865_INC_Y_ODD_REG: c_uint = 0x382a;

pub const OV8865_INC_Y_EVEN_REG: c_uint = 0x382b;

pub const OV8865_ABLC_NUM_REG: c_uint = 0x3830;

pub const OV8865_ZLINE_NUM_REG: c_uint = 0x3836;

pub const OV8865_AUTO_SIZE_CTRL_REG: c_uint = 0x3841;

pub const OV8865_AUTO_SIZE_X_OFFSET_H_REG: c_uint = 0x3842;
pub const OV8865_AUTO_SIZE_X_OFFSET_L_REG: c_uint = 0x3843;
pub const OV8865_AUTO_SIZE_Y_OFFSET_H_REG: c_uint = 0x3844;
pub const OV8865_AUTO_SIZE_Y_OFFSET_L_REG: c_uint = 0x3845;
pub const OV8865_AUTO_SIZE_BOUNDARIES_REG: c_uint = 0x3846;

// PSRAM
pub const OV8865_PSRAM_CTRL8_REG: c_uint = 0x3f08;
// Black Level
pub const OV8865_BLC_CTRL0_REG: c_uint = 0x4000;

pub const OV8865_BLC_CTRL1_REG: c_uint = 0x4001;

pub const OV8865_BLC_CTRL2_REG: c_uint = 0x4002;
pub const OV8865_BLC_CTRL3_REG: c_uint = 0x4003;
pub const OV8865_BLC_CTRL4_REG: c_uint = 0x4004;
pub const OV8865_BLC_CTRL5_REG: c_uint = 0x4005;
pub const OV8865_BLC_CTRL6_REG: c_uint = 0x4006;
pub const OV8865_BLC_CTRL7_REG: c_uint = 0x4007;
pub const OV8865_BLC_CTRL8_REG: c_uint = 0x4008;
pub const OV8865_BLC_CTRL9_REG: c_uint = 0x4009;
pub const OV8865_BLC_CTRLA_REG: c_uint = 0x400a;
pub const OV8865_BLC_CTRLB_REG: c_uint = 0x400b;
pub const OV8865_BLC_CTRLC_REG: c_uint = 0x400c;
pub const OV8865_BLC_CTRLD_REG: c_uint = 0x400d;

pub const OV8865_BLC_CTRL1F_REG: c_uint = 0x401f;

pub const OV8865_BLC_ANCHOR_LEFT_START_H_REG: c_uint = 0x4020;

pub const OV8865_BLC_ANCHOR_LEFT_START_L_REG: c_uint = 0x4021;

pub const OV8865_BLC_ANCHOR_LEFT_END_H_REG: c_uint = 0x4022;

pub const OV8865_BLC_ANCHOR_LEFT_END_L_REG: c_uint = 0x4023;

pub const OV8865_BLC_ANCHOR_RIGHT_START_H_REG: c_uint = 0x4024;

pub const OV8865_BLC_ANCHOR_RIGHT_START_L_REG: c_uint = 0x4025;

pub const OV8865_BLC_ANCHOR_RIGHT_END_H_REG: c_uint = 0x4026;

pub const OV8865_BLC_ANCHOR_RIGHT_END_L_REG: c_uint = 0x4027;

pub const OV8865_BLC_TOP_ZLINE_START_REG: c_uint = 0x4028;

pub const OV8865_BLC_TOP_ZLINE_NUM_REG: c_uint = 0x4029;

pub const OV8865_BLC_TOP_BLKLINE_START_REG: c_uint = 0x402a;

pub const OV8865_BLC_TOP_BLKLINE_NUM_REG: c_uint = 0x402b;

pub const OV8865_BLC_BOT_ZLINE_START_REG: c_uint = 0x402c;

pub const OV8865_BLC_BOT_ZLINE_NUM_REG: c_uint = 0x402d;

pub const OV8865_BLC_BOT_BLKLINE_START_REG: c_uint = 0x402e;

pub const OV8865_BLC_BOT_BLKLINE_NUM_REG: c_uint = 0x402f;

pub const OV8865_BLC_OFFSET_LIMIT_REG: c_uint = 0x4034;

// VFIFO
pub const OV8865_VFIFO_READ_START_H_REG: c_uint = 0x4600;

pub const OV8865_VFIFO_READ_START_L_REG: c_uint = 0x4601;

// MIPI
pub const OV8865_MIPI_CTRL0_REG: c_uint = 0x4800;
pub const OV8865_MIPI_CTRL1_REG: c_uint = 0x4801;
pub const OV8865_MIPI_CTRL2_REG: c_uint = 0x4802;
pub const OV8865_MIPI_CTRL3_REG: c_uint = 0x4803;
pub const OV8865_MIPI_CTRL4_REG: c_uint = 0x4804;
pub const OV8865_MIPI_CTRL5_REG: c_uint = 0x4805;
pub const OV8865_MIPI_CTRL6_REG: c_uint = 0x4806;
pub const OV8865_MIPI_CTRL7_REG: c_uint = 0x4807;
pub const OV8865_MIPI_CTRL8_REG: c_uint = 0x4808;
pub const OV8865_MIPI_FCNT_MAX_H_REG: c_uint = 0x4810;
pub const OV8865_MIPI_FCNT_MAX_L_REG: c_uint = 0x4811;
pub const OV8865_MIPI_CTRL13_REG: c_uint = 0x4813;
pub const OV8865_MIPI_CTRL14_REG: c_uint = 0x4814;
pub const OV8865_MIPI_CTRL15_REG: c_uint = 0x4815;
pub const OV8865_MIPI_EMBEDDED_DT_REG: c_uint = 0x4816;
pub const OV8865_MIPI_HS_ZERO_MIN_H_REG: c_uint = 0x4818;
pub const OV8865_MIPI_HS_ZERO_MIN_L_REG: c_uint = 0x4819;
pub const OV8865_MIPI_HS_TRAIL_MIN_H_REG: c_uint = 0x481a;
pub const OV8865_MIPI_HS_TRAIL_MIN_L_REG: c_uint = 0x481b;
pub const OV8865_MIPI_CLK_ZERO_MIN_H_REG: c_uint = 0x481c;
pub const OV8865_MIPI_CLK_ZERO_MIN_L_REG: c_uint = 0x481d;
pub const OV8865_MIPI_CLK_PREPARE_MAX_REG: c_uint = 0x481e;
pub const OV8865_MIPI_CLK_PREPARE_MIN_REG: c_uint = 0x481f;
pub const OV8865_MIPI_CLK_POST_MIN_H_REG: c_uint = 0x4820;
pub const OV8865_MIPI_CLK_POST_MIN_L_REG: c_uint = 0x4821;
pub const OV8865_MIPI_CLK_TRAIL_MIN_H_REG: c_uint = 0x4822;
pub const OV8865_MIPI_CLK_TRAIL_MIN_L_REG: c_uint = 0x4823;
pub const OV8865_MIPI_LPX_P_MIN_H_REG: c_uint = 0x4824;
pub const OV8865_MIPI_LPX_P_MIN_L_REG: c_uint = 0x4825;
pub const OV8865_MIPI_HS_PREPARE_MIN_REG: c_uint = 0x4826;
pub const OV8865_MIPI_HS_PREPARE_MAX_REG: c_uint = 0x4827;
pub const OV8865_MIPI_HS_EXIT_MIN_H_REG: c_uint = 0x4828;
pub const OV8865_MIPI_HS_EXIT_MIN_L_REG: c_uint = 0x4829;
pub const OV8865_MIPI_UI_HS_ZERO_MIN_REG: c_uint = 0x482a;
pub const OV8865_MIPI_UI_HS_TRAIL_MIN_REG: c_uint = 0x482b;
pub const OV8865_MIPI_UI_CLK_ZERO_MIN_REG: c_uint = 0x482c;
pub const OV8865_MIPI_UI_CLK_PREPARE_REG: c_uint = 0x482d;
pub const OV8865_MIPI_UI_CLK_POST_MIN_REG: c_uint = 0x482e;
pub const OV8865_MIPI_UI_CLK_TRAIL_MIN_REG: c_uint = 0x482f;
pub const OV8865_MIPI_UI_LPX_P_MIN_REG: c_uint = 0x4830;
pub const OV8865_MIPI_UI_HS_PREPARE_REG: c_uint = 0x4831;
pub const OV8865_MIPI_UI_HS_EXIT_MIN_REG: c_uint = 0x4832;
pub const OV8865_MIPI_PKT_START_SIZE_REG: c_uint = 0x4833;
pub const OV8865_MIPI_PCLK_PERIOD_REG: c_uint = 0x4837;
pub const OV8865_MIPI_LP_GPIO0_REG: c_uint = 0x4838;
pub const OV8865_MIPI_LP_GPIO1_REG: c_uint = 0x4839;
pub const OV8865_MIPI_CTRL3C_REG: c_uint = 0x483c;
pub const OV8865_MIPI_LP_GPIO4_REG: c_uint = 0x483d;
pub const OV8865_MIPI_CTRL4A_REG: c_uint = 0x484a;
pub const OV8865_MIPI_CTRL4B_REG: c_uint = 0x484b;
pub const OV8865_MIPI_CTRL4C_REG: c_uint = 0x484c;
pub const OV8865_MIPI_LANE_TEST_PATTERN_REG: c_uint = 0x484d;
pub const OV8865_MIPI_FRAME_END_DELAY_REG: c_uint = 0x484e;
pub const OV8865_MIPI_CLOCK_TEST_PATTERN_REG: c_uint = 0x484f;
pub const OV8865_MIPI_LANE_SEL01_REG: c_uint = 0x4850;

pub const OV8865_MIPI_LANE_SEL23_REG: c_uint = 0x4851;

// ISP
pub const OV8865_ISP_CTRL0_REG: c_uint = 0x5000;

pub const OV8865_ISP_CTRL1_REG: c_uint = 0x5001;

pub const OV8865_ISP_CTRL2_REG: c_uint = 0x5002;

pub const OV8865_ISP_CTRL3_REG: c_uint = 0x5003;
pub const OV8865_ISP_GAIN_RED_H_REG: c_uint = 0x5018;

pub const OV8865_ISP_GAIN_RED_L_REG: c_uint = 0x5019;

pub const OV8865_ISP_GAIN_GREEN_H_REG: c_uint = 0x501a;

pub const OV8865_ISP_GAIN_GREEN_L_REG: c_uint = 0x501b;

pub const OV8865_ISP_GAIN_BLUE_H_REG: c_uint = 0x501c;

pub const OV8865_ISP_GAIN_BLUE_L_REG: c_uint = 0x501d;

// VarioPixel
pub const OV8865_VAP_CTRL0_REG: c_uint = 0x5900;
pub const OV8865_VAP_CTRL1_REG: c_uint = 0x5901;

    GENMASK(3, 2))

// Pre-DSP
pub const OV8865_PRE_CTRL0_REG: c_uint = 0x5e00;

pub const OV8865_PRE_CTRL0_PATTERN_COLOR_BARS: c_int = 0;
pub const OV8865_PRE_CTRL0_PATTERN_RANDOM_DATA: c_int = 1;
pub const OV8865_PRE_CTRL0_PATTERN_COLOR_SQUARES: c_int = 2;
pub const OV8865_PRE_CTRL0_PATTERN_BLACK: c_int = 3;
// Pixel Array
pub const OV8865_NATIVE_WIDTH: c_int = 3296;
pub const OV8865_NATIVE_HEIGHT: c_int = 2528;
pub const OV8865_ACTIVE_START_LEFT: c_int = 16;
pub const OV8865_ACTIVE_START_TOP: c_int = 40;
pub const OV8865_ACTIVE_WIDTH: c_int = 3264;
pub const OV8865_ACTIVE_HEIGHT: c_int = 2448;
// Macros

    container_of(s, struct ov8865_sensor, subdev)

    (&container_of((c).handler, struct ov8865_sensor, \
    ctrls.handler).subdev)
// Data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_register_value {
    pub address: u16,
    pub value: u8,
    pub delay_ms: c_uint,
}

//
// PLL1 Clock Tree:
//
// +-< EXTCLK
// |
// +-+ pll_pre_div_half (0x30a [0])
// |
// +-+ pll_pre_div (0x300 [2:0], special values:
// |              0: 1, 1: 1.5, 3: 2.5, 4: 3, 5: 4, 7: 8)
// +-+ pll_mul (0x301 [1:0], 0x302 [7:0])
// |
// +-+ m_div (0x303 [3:0])
// | |
// | +-> PHY_SCLK
// | |
// | +-+ mipi_div (0x304 [1:0], special values: 0: 4, 1: 5, 2: 6, 3: 8)
// |   |
// |   +-+ pclk_div (0x3020 [3])
// |     |
// |     +-> PCLK
// |
// +-+ sys_pre_div (0x305 [1:0], special values: 0: 3, 1: 4, 2: 5, 3: 6)
// |
// +-+ sys_div (0x306 [0])
// |
// +-+ sys_sel (0x3032 [7], 0: PLL1, 1: PLL2)
// |
// +-+ sclk_sel (0x3033 [1], 0: sys_sel, 1: PLL2 DAC_CLK)
// |
// +-+ sclk_pre_div (0x3106 [3:2], special values:
// |               0: 1, 1: 2, 2: 4, 3: 1)
// |
// +-+ sclk_div (0x3106 [7:4], special values: 0: 1)
// |
// +-> SCLK
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_pll1_config {
    pub pll_pre_div_half: c_uint,
    pub pll_pre_div: c_uint,
    pub pll_mul: c_uint,
    pub m_div: c_uint,
    pub mipi_div: c_uint,
    pub pclk_div: c_uint,
    pub sys_pre_div: c_uint,
    pub sys_div: c_uint,
}

//
// PLL2 Clock Tree:
//
// +-< EXTCLK
// |
// +-+ pll_pre_div_half (0x312 [4])
// |
// +-+ pll_pre_div (0x30b [2:0], special values:
// |              0: 1, 1: 1.5, 3: 2.5, 4: 3, 5: 4, 7: 8)
// +-+ pll_mul (0x30c [1:0], 0x30d [7:0])
// |
// +-+ dac_div (0x312 [3:0])
// | |
// | +-> DAC_CLK
// |
// +-+ sys_pre_div (0x30f [3:0])
// |
// +-+ sys_div (0x30e [2:0], special values:
// |          0: 1, 1: 1.5, 3: 2.5, 4: 3, 5: 3.5, 6: 4, 7:5)
// |
// +-+ sys_sel (0x3032 [7], 0: PLL1, 1: PLL2)
// |
// +-+ sclk_sel (0x3033 [1], 0: sys_sel, 1: PLL2 DAC_CLK)
// |
// +-+ sclk_pre_div (0x3106 [3:2], special values:
// |               0: 1, 1: 2, 2: 4, 3: 1)
// |
// +-+ sclk_div (0x3106 [7:4], special values: 0: 1)
// |
// +-> SCLK
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_pll2_config {
    pub pll_pre_div_half: c_uint,
    pub pll_pre_div: c_uint,
    pub pll_mul: c_uint,
    pub dac_div: c_uint,
    pub sys_pre_div: c_uint,
    pub sys_div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_sclk_config {
    pub sys_sel: c_uint,
    pub sclk_sel: c_uint,
    pub sclk_pre_div: c_uint,
    pub sclk_div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_pll_configs {
    pub pll1_config: *const ov8865_pll1_config,
    pub pll2_config_native: *const ov8865_pll2_config,
    pub pll2_config_binning: *const ov8865_pll2_config,
}

// Clock rate
    enum extclk_rate {
    OV8865_19_2_MHZ,
    OV8865_24_MHZ,
    OV8865_NUM_SUPPORTED_RATES
    };
    static const unsigned long supported_extclk_rates[] = {
    [OV8865_19_2_MHZ] = 19200000,
    [OV8865_24_MHZ] = 24000000,
    };
//
// General formulas for (array-centered) mode calculation:
// - photo_array_width = 3296
// - crop_start_x = (photo_array_width - output_size_x) / 2
// - crop_end_x = crop_start_x + offset_x + output_size_x - 1
//
// - photo_array_height = 2480
// - crop_start_y = (photo_array_height - output_size_y) / 2
// - crop_end_y = crop_start_y + offset_y + output_size_y - 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_mode {
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
// With auto size, only output and total sizes need to be set.
    pub size_auto: bool,
    pub size_auto_boundary_x: c_uint,
    pub size_auto_boundary_y: c_uint,
    pub binning_x: bool,
    pub binning_y: bool,
    pub variopixel: bool,
    pub variopixel_hsub_coef: c_uint,
    pub variopixel_vsub_coef: c_uint,
// Bits for the format register, used for binning.
    pub sync_hbin: bool,
    pub horz_var2: bool,
    pub inc_x_odd: c_uint,
    pub inc_x_even: c_uint,
    pub inc_y_odd: c_uint,
    pub inc_y_even: c_uint,
    pub vfifo_read_start: c_uint,
    pub ablc_num: c_uint,
    pub zline_num: c_uint,
    pub blc_top_zero_line_start: c_uint,
    pub blc_top_zero_line_num: c_uint,
    pub blc_top_black_line_start: c_uint,
    pub blc_top_black_line_num: c_uint,
    pub blc_bottom_zero_line_start: c_uint,
    pub blc_bottom_zero_line_num: c_uint,
    pub blc_bottom_black_line_start: c_uint,
    pub blc_bottom_black_line_num: c_uint,
    pub blc_col_shift_mask: u8,
    pub blc_anchor_left_start: c_uint,
    pub blc_anchor_left_end: c_uint,
    pub blc_anchor_right_start: c_uint,
    pub blc_anchor_right_end: c_uint,
    pub pll2_binning: bool,
    pub register_values: *const ov8865_register_value,
    pub register_values_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_state {
    pub mode: *const ov8865_mode,
    pub mbus_code: u32,
    pub streaming: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_ctrls {
    pub link_freq: *mut v4l2_ctrl,
    pub pixel_rate: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub handler: v4l2_ctrl_handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov8865_sensor {
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub reset: *mut gpio_desc,
    pub powerdown: *mut gpio_desc,
    pub avdd: *mut regulator,
    pub dvdd: *mut regulator,
    pub dovdd: *mut regulator,
    pub extclk_rate: c_ulong,
    pub pll_configs: *const ov8865_pll_configs,
    pub extclk: *mut clk,
    pub endpoint: v4l2_fwnode_endpoint,
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub mutex: mutex,
    pub state: ov8865_state,
    pub ctrls: ov8865_ctrls,
}

// Static definitions
//
// PHY_SCLK = 720 MHz
// MIPI_PCLK = 90 MHz
//
    static const struct ov8865_pll1_config ov8865_pll1_config_native_19_2mhz = {
    .pll_pre_div_half	= 1,
    .pll_pre_div		= 2,
    .pll_mul		= 75,
    .m_div			= 1,
    .mipi_div		= 3,
    .pclk_div		= 1,
    .sys_pre_div		= 1,
    .sys_div		= 2,
    };
    static const struct ov8865_pll1_config ov8865_pll1_config_native_24mhz = {
    .pll_pre_div_half	= 1,
    .pll_pre_div		= 0,
    .pll_mul		= 30,
    .m_div			= 1,
    .mipi_div		= 3,
    .pclk_div		= 1,
    .sys_pre_div		= 1,
    .sys_div		= 2,
    };
//
// DAC_CLK = 360 MHz
// SCLK = 144 MHz
//
    static const struct ov8865_pll2_config ov8865_pll2_config_native_19_2mhz = {
    .pll_pre_div_half	= 1,
    .pll_pre_div		= 5,
    .pll_mul		= 75,
    .dac_div		= 1,
    .sys_pre_div		= 1,
    .sys_div		= 3,
    };
    static const struct ov8865_pll2_config ov8865_pll2_config_native_24mhz = {
    .pll_pre_div_half	= 1,
    .pll_pre_div		= 0,
    .pll_mul		= 30,
    .dac_div		= 2,
    .sys_pre_div		= 5,
    .sys_div		= 0,
    };
//
// DAC_CLK = 360 MHz
// SCLK = 72 MHz
//
    static const struct ov8865_pll2_config ov8865_pll2_config_binning_19_2mhz = {
    .pll_pre_div_half	= 1,
    .pll_pre_div		= 2,
    .pll_mul		= 75,
    .dac_div		= 2,
    .sys_pre_div		= 10,
    .sys_div		= 0,
    };
    static const struct ov8865_pll2_config ov8865_pll2_config_binning_24mhz = {
    .pll_pre_div_half	= 1,
    .pll_pre_div		= 0,
    .pll_mul		= 30,
    .dac_div		= 2,
    .sys_pre_div		= 10,
    .sys_div		= 0,
    };
    static const struct ov8865_pll_configs ov8865_pll_configs_19_2mhz = {
    .pll1_config = &ov8865_pll1_config_native_19_2mhz,
    .pll2_config_native = &ov8865_pll2_config_native_19_2mhz,
    .pll2_config_binning = &ov8865_pll2_config_binning_19_2mhz,
    };
    static const struct ov8865_pll_configs ov8865_pll_configs_24mhz = {
    .pll1_config = &ov8865_pll1_config_native_24mhz,
    .pll2_config_native = &ov8865_pll2_config_native_24mhz,
    .pll2_config_binning = &ov8865_pll2_config_binning_24mhz,
    };
    static const struct ov8865_pll_configs *ov8865_pll_configs[] = {
    &ov8865_pll_configs_19_2mhz,
    &ov8865_pll_configs_24mhz,
    };
    static const struct ov8865_sclk_config ov8865_sclk_config_native = {
    .sys_sel		= 1,
    .sclk_sel		= 0,
    .sclk_pre_div		= 0,
    .sclk_div		= 0,
    };
    static const struct ov8865_register_value ov8865_register_values_native[] = {
// Sensor
    { 0x3700, 0x48 },
    { 0x3701, 0x18 },
    { 0x3702, 0x50 },
    { 0x3703, 0x32 },
    { 0x3704, 0x28 },
    { 0x3706, 0x70 },
    { 0x3707, 0x08 },
    { 0x3708, 0x48 },
    { 0x3709, 0x80 },
    { 0x370a, 0x01 },
    { 0x370b, 0x70 },
    { 0x370c, 0x07 },
    { 0x3718, 0x14 },
    { 0x3712, 0x44 },
    { 0x371e, 0x31 },
    { 0x371f, 0x7f },
    { 0x3720, 0x0a },
    { 0x3721, 0x0a },
    { 0x3724, 0x04 },
    { 0x3725, 0x04 },
    { 0x3726, 0x0c },
    { 0x3728, 0x0a },
    { 0x3729, 0x03 },
    { 0x372a, 0x06 },
    { 0x372b, 0xa6 },
    { 0x372c, 0xa6 },
    { 0x372d, 0xa6 },
    { 0x372e, 0x0c },
    { 0x372f, 0x20 },
    { 0x3730, 0x02 },
    { 0x3731, 0x0c },
    { 0x3732, 0x28 },
    { 0x3736, 0x30 },
    { 0x373a, 0x04 },
    { 0x373b, 0x18 },
    { 0x373c, 0x14 },
    { 0x373e, 0x06 },
    { 0x375a, 0x0c },
    { 0x375b, 0x26 },
    { 0x375d, 0x04 },
    { 0x375f, 0x28 },
    { 0x3767, 0x1e },
    { 0x3772, 0x46 },
    { 0x3773, 0x04 },
    { 0x3774, 0x2c },
    { 0x3775, 0x13 },
    { 0x3776, 0x10 },
    { 0x37a0, 0x88 },
    { 0x37a1, 0x7a },
    { 0x37a2, 0x7a },
    { 0x37a3, 0x02 },
    { 0x37a5, 0x09 },
    { 0x37a7, 0x88 },
    { 0x37a8, 0xb0 },
    { 0x37a9, 0xb0 },
    { 0x37aa, 0x88 },
    { 0x37ab, 0x5c },
    { 0x37ac, 0x5c },
    { 0x37ad, 0x55 },
    { 0x37ae, 0x19 },
    { 0x37af, 0x19 },
    { 0x37b3, 0x84 },
    { 0x37b4, 0x84 },
    { 0x37b5, 0x66 },
// PSRAM
    { OV8865_PSRAM_CTRL8_REG, 0x16 },
// ADC Sync
    { 0x4500, 0x68 },
    };
    static const struct ov8865_register_value ov8865_register_values_binning[] = {
// Sensor
    { 0x3700, 0x24 },
    { 0x3701, 0x0c },
    { 0x3702, 0x28 },
    { 0x3703, 0x19 },
    { 0x3704, 0x14 },
    { 0x3706, 0x38 },
    { 0x3707, 0x04 },
    { 0x3708, 0x24 },
    { 0x3709, 0x40 },
    { 0x370a, 0x00 },
    { 0x370b, 0xb8 },
    { 0x370c, 0x04 },
    { 0x3718, 0x12 },
    { 0x3712, 0x42 },
    { 0x371e, 0x19 },
    { 0x371f, 0x40 },
    { 0x3720, 0x05 },
    { 0x3721, 0x05 },
    { 0x3724, 0x02 },
    { 0x3725, 0x02 },
    { 0x3726, 0x06 },
    { 0x3728, 0x05 },
    { 0x3729, 0x02 },
    { 0x372a, 0x03 },
    { 0x372b, 0x53 },
    { 0x372c, 0xa3 },
    { 0x372d, 0x53 },
    { 0x372e, 0x06 },
    { 0x372f, 0x10 },
    { 0x3730, 0x01 },
    { 0x3731, 0x06 },
    { 0x3732, 0x14 },
    { 0x3736, 0x20 },
    { 0x373a, 0x02 },
    { 0x373b, 0x0c },
    { 0x373c, 0x0a },
    { 0x373e, 0x03 },
    { 0x375a, 0x06 },
    { 0x375b, 0x13 },
    { 0x375d, 0x02 },
    { 0x375f, 0x14 },
    { 0x3767, 0x1c },
    { 0x3772, 0x23 },
    { 0x3773, 0x02 },
    { 0x3774, 0x16 },
    { 0x3775, 0x12 },
    { 0x3776, 0x08 },
    { 0x37a0, 0x44 },
    { 0x37a1, 0x3d },
    { 0x37a2, 0x3d },
    { 0x37a3, 0x01 },
    { 0x37a5, 0x08 },
    { 0x37a7, 0x44 },
    { 0x37a8, 0x58 },
    { 0x37a9, 0x58 },
    { 0x37aa, 0x44 },
    { 0x37ab, 0x2e },
    { 0x37ac, 0x2e },
    { 0x37ad, 0x33 },
    { 0x37ae, 0x0d },
    { 0x37af, 0x0d },
    { 0x37b3, 0x42 },
    { 0x37b4, 0x42 },
    { 0x37b5, 0x33 },
// PSRAM
    { OV8865_PSRAM_CTRL8_REG, 0x0b },
// ADC Sync
    { 0x4500, 0x40 },
    };
    static const struct ov8865_mode ov8865_modes[] = {
// 3264x2448
    {
// Horizontal
    .output_size_x			= 3264,
    .hts				= 3888,
// Vertical
    .output_size_y			= 2448,
    .vts				= 2470,
    .size_auto			= true,
    .size_auto_boundary_x		= 8,
    .size_auto_boundary_y		= 4,
// Subsample increase
    .inc_x_odd			= 1,
    .inc_x_even			= 1,
    .inc_y_odd			= 1,
    .inc_y_even			= 1,
// VFIFO
    .vfifo_read_start		= 16,
    .ablc_num			= 4,
    .zline_num			= 1,
// Black Level
    .blc_top_zero_line_start	= 0,
    .blc_top_zero_line_num		= 2,
    .blc_top_black_line_start	= 4,
    .blc_top_black_line_num		= 4,
    .blc_bottom_zero_line_start	= 2,
    .blc_bottom_zero_line_num	= 2,
    .blc_bottom_black_line_start	= 8,
    .blc_bottom_black_line_num	= 2,
    .blc_anchor_left_start		= 576,
    .blc_anchor_left_end		= 831,
    .blc_anchor_right_start		= 1984,
    .blc_anchor_right_end		= 2239,
// PLL
    .pll2_binning			= false,
// Registers
    .register_values	= ov8865_register_values_native,
    .register_values_count	=
    ARRAY_SIZE(ov8865_register_values_native),
    },
// 3264x1836
    {
// Horizontal
    .output_size_x			= 3264,
    .hts				= 3888,
// Vertical
    .output_size_y			= 1836,
    .vts				= 2470,
    .size_auto			= true,
    .size_auto_boundary_x		= 8,
    .size_auto_boundary_y		= 4,
// Subsample increase
    .inc_x_odd			= 1,
    .inc_x_even			= 1,
    .inc_y_odd			= 1,
    .inc_y_even			= 1,
// VFIFO
    .vfifo_read_start		= 16,
    .ablc_num			= 4,
    .zline_num			= 1,
// Black Level
    .blc_top_zero_line_start	= 0,
    .blc_top_zero_line_num		= 2,
    .blc_top_black_line_start	= 4,
    .blc_top_black_line_num		= 4,
    .blc_bottom_zero_line_start	= 2,
    .blc_bottom_zero_line_num	= 2,
    .blc_bottom_black_line_start	= 8,
    .blc_bottom_black_line_num	= 2,
    .blc_anchor_left_start		= 576,
    .blc_anchor_left_end		= 831,
    .blc_anchor_right_start		= 1984,
    .blc_anchor_right_end		= 2239,
// PLL
    .pll2_binning			= false,
// Registers
    .register_values	= ov8865_register_values_native,
    .register_values_count	=
    ARRAY_SIZE(ov8865_register_values_native),
    },
// 1632x1224
    {
// Horizontal
    .output_size_x			= 1632,
    .hts				= 1923,
// Vertical
    .output_size_y			= 1224,
    .vts				= 1248,
    .size_auto			= true,
    .size_auto_boundary_x		= 8,
    .size_auto_boundary_y		= 8,
// Subsample increase
    .inc_x_odd			= 3,
    .inc_x_even			= 1,
    .inc_y_odd			= 3,
    .inc_y_even			= 1,
// Binning
    .binning_y			= true,
    .sync_hbin			= true,
// VFIFO
    .vfifo_read_start		= 116,
    .ablc_num			= 8,
    .zline_num			= 2,
// Black Level
    .blc_top_zero_line_start	= 0,
    .blc_top_zero_line_num		= 2,
    .blc_top_black_line_start	= 4,
    .blc_top_black_line_num		= 4,
    .blc_bottom_zero_line_start	= 2,
    .blc_bottom_zero_line_num	= 2,
    .blc_bottom_black_line_start	= 8,
    .blc_bottom_black_line_num	= 2,
    .blc_anchor_left_start		= 288,
    .blc_anchor_left_end		= 415,
    .blc_anchor_right_start		= 992,
    .blc_anchor_right_end		= 1119,
// PLL
    .pll2_binning			= true,
// Registers
    .register_values	= ov8865_register_values_binning,
    .register_values_count	=
    ARRAY_SIZE(ov8865_register_values_binning),
    },
// 800x600 (SVGA)
    {
// Horizontal
    .output_size_x			= 800,
    .hts				= 1250,
// Vertical
    .output_size_y			= 600,
    .vts				= 640,
    .size_auto			= true,
    .size_auto_boundary_x		= 8,
    .size_auto_boundary_y		= 8,
// Subsample increase
    .inc_x_odd			= 3,
    .inc_x_even			= 1,
    .inc_y_odd			= 5,
    .inc_y_even			= 3,
// Binning
    .binning_y			= true,
    .variopixel			= true,
    .variopixel_hsub_coef		= 2,
    .variopixel_vsub_coef		= 1,
    .sync_hbin			= true,
    .horz_var2			= true,
// VFIFO
    .vfifo_read_start		= 80,
    .ablc_num			= 8,
    .zline_num			= 2,
// Black Level
    .blc_top_zero_line_start	= 0,
    .blc_top_zero_line_num		= 2,
    .blc_top_black_line_start	= 2,
    .blc_top_black_line_num		= 2,
    .blc_bottom_zero_line_start	= 0,
    .blc_bottom_zero_line_num	= 0,
    .blc_bottom_black_line_start	= 4,
    .blc_bottom_black_line_num	= 2,
    .blc_col_shift_mask	= OV8865_BLC_CTRL1_COL_SHIFT_128,
    .blc_anchor_left_start		= 288,
    .blc_anchor_left_end		= 415,
    .blc_anchor_right_start		= 992,
    .blc_anchor_right_end		= 1119,
// PLL
    .pll2_binning			= true,
// Registers
    .register_values	= ov8865_register_values_binning,
    .register_values_count	=
    ARRAY_SIZE(ov8865_register_values_binning),
    },
    };
    static const u32 ov8865_mbus_codes[] = {
    MEDIA_BUS_FMT_SBGGR10_1X10,
    };
    static const struct ov8865_register_value ov8865_init_sequence[] = {
// Analog
    { 0x3604, 0x04 },
    { 0x3602, 0x30 },
    { 0x3605, 0x00 },
    { 0x3607, 0x20 },
    { 0x3608, 0x11 },
    { 0x3609, 0x68 },
    { 0x360a, 0x40 },
    { 0x360c, 0xdd },
    { 0x360e, 0x0c },
    { 0x3610, 0x07 },
    { 0x3612, 0x86 },
    { 0x3613, 0x58 },
    { 0x3614, 0x28 },
    { 0x3617, 0x40 },
    { 0x3618, 0x5a },
    { 0x3619, 0x9b },
    { 0x361c, 0x00 },
    { 0x361d, 0x60 },
    { 0x3631, 0x60 },
    { 0x3633, 0x10 },
    { 0x3634, 0x10 },
    { 0x3635, 0x10 },
    { 0x3636, 0x10 },
    { 0x3638, 0xff },
    { 0x3641, 0x55 },
    { 0x3646, 0x86 },
    { 0x3647, 0x27 },
    { 0x364a, 0x1b },
// Sensor
    { 0x3700, 0x24 },
    { 0x3701, 0x0c },
    { 0x3702, 0x28 },
    { 0x3703, 0x19 },
    { 0x3704, 0x14 },
    { 0x3705, 0x00 },
    { 0x3706, 0x38 },
    { 0x3707, 0x04 },
    { 0x3708, 0x24 },
    { 0x3709, 0x40 },
    { 0x370a, 0x00 },
    { 0x370b, 0xb8 },
    { 0x370c, 0x04 },
    { 0x3718, 0x12 },
    { 0x3719, 0x31 },
    { 0x3712, 0x42 },
    { 0x3714, 0x12 },
    { 0x371e, 0x19 },
    { 0x371f, 0x40 },
    { 0x3720, 0x05 },
    { 0x3721, 0x05 },
    { 0x3724, 0x02 },
    { 0x3725, 0x02 },
    { 0x3726, 0x06 },
    { 0x3728, 0x05 },
    { 0x3729, 0x02 },
    { 0x372a, 0x03 },
    { 0x372b, 0x53 },
    { 0x372c, 0xa3 },
    { 0x372d, 0x53 },
    { 0x372e, 0x06 },
    { 0x372f, 0x10 },
    { 0x3730, 0x01 },
    { 0x3731, 0x06 },
    { 0x3732, 0x14 },
    { 0x3733, 0x10 },
    { 0x3734, 0x40 },
    { 0x3736, 0x20 },
    { 0x373a, 0x02 },
    { 0x373b, 0x0c },
    { 0x373c, 0x0a },
    { 0x373e, 0x03 },
    { 0x3755, 0x40 },
    { 0x3758, 0x00 },
    { 0x3759, 0x4c },
    { 0x375a, 0x06 },
    { 0x375b, 0x13 },
    { 0x375c, 0x40 },
    { 0x375d, 0x02 },
    { 0x375e, 0x00 },
    { 0x375f, 0x14 },
    { 0x3767, 0x1c },
    { 0x3768, 0x04 },
    { 0x3769, 0x20 },
    { 0x376c, 0xc0 },
    { 0x376d, 0xc0 },
    { 0x376a, 0x08 },
    { 0x3761, 0x00 },
    { 0x3762, 0x00 },
    { 0x3763, 0x00 },
    { 0x3766, 0xff },
    { 0x376b, 0x42 },
    { 0x3772, 0x23 },
    { 0x3773, 0x02 },
    { 0x3774, 0x16 },
    { 0x3775, 0x12 },
    { 0x3776, 0x08 },
    { 0x37a0, 0x44 },
    { 0x37a1, 0x3d },
    { 0x37a2, 0x3d },
    { 0x37a3, 0x01 },
    { 0x37a4, 0x00 },
    { 0x37a5, 0x08 },
    { 0x37a6, 0x00 },
    { 0x37a7, 0x44 },
    { 0x37a8, 0x58 },
    { 0x37a9, 0x58 },
    { 0x3760, 0x00 },
    { 0x376f, 0x01 },
    { 0x37aa, 0x44 },
    { 0x37ab, 0x2e },
    { 0x37ac, 0x2e },
    { 0x37ad, 0x33 },
    { 0x37ae, 0x0d },
    { 0x37af, 0x0d },
    { 0x37b0, 0x00 },
    { 0x37b1, 0x00 },
    { 0x37b2, 0x00 },
    { 0x37b3, 0x42 },
    { 0x37b4, 0x42 },
    { 0x37b5, 0x33 },
    { 0x37b6, 0x00 },
    { 0x37b7, 0x00 },
    { 0x37b8, 0x00 },
    { 0x37b9, 0xff },
// ADC Sync
    { 0x4503, 0x10 },
    };
    static const s64 ov8865_link_freq_menu[] = {
    360000000,
    };
    static const char *const ov8865_test_pattern_menu[] = {
    "Disabled",
    "Random data",
    "Color bars",
    "Color bars with rolling bar",
    "Color squares",
    "Color squares with rolling bar"
    };
    static const u8 ov8865_test_pattern_bits[] = {
    0,
    OV8865_PRE_CTRL0_PATTERN_EN | OV8865_PRE_CTRL0_PATTERN_RANDOM_DATA,
    OV8865_PRE_CTRL0_PATTERN_EN | OV8865_PRE_CTRL0_PATTERN_COLOR_BARS,
    OV8865_PRE_CTRL0_PATTERN_EN | OV8865_PRE_CTRL0_ROLLING_BAR_EN |
    OV8865_PRE_CTRL0_PATTERN_COLOR_BARS,
    OV8865_PRE_CTRL0_PATTERN_EN | OV8865_PRE_CTRL0_PATTERN_COLOR_SQUARES,
    OV8865_PRE_CTRL0_PATTERN_EN | OV8865_PRE_CTRL0_ROLLING_BAR_EN |
    OV8865_PRE_CTRL0_PATTERN_COLOR_SQUARES,
    };
// Input/Output
#[no_mangle]
unsafe extern "C" fn ov8865_read(sensor: *mut ov8865_sensor, address: u16, value: *mut u8) -> c_int {
    static int ov8865_read(struct ov8865_sensor *sensor, u16 address, u8 *value)
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
unsafe extern "C" fn ov8865_write(sensor: *mut ov8865_sensor, address: u16, value: u8) -> c_int {
    static int ov8865_write(struct ov8865_sensor *sensor, u16 address, u8 value)
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
    static int ov8865_write_sequence(struct ov8865_sensor *sensor,
    const struct ov8865_register_value *sequence,
    unsigned int sequence_count)
    {
    unsigned int i;
    let mut ret: c_int = 0;
    for (i = 0; i < sequence_count; i++) {
    ret = ov8865_write(sensor, sequence[i].address,
    sequence[i].value);
    if (ret)
    break;
    if (sequence[i].delay_ms)
    msleep(sequence[i].delay_ms);
    }
    return ret;
    }
    static int ov8865_update_bits(struct ov8865_sensor *sensor, u16 address,
    u8 mask, u8 bits)
    {
    let mut value: u8 = 0;
    int ret;
    ret = ov8865_read(sensor, address, &value);
    if (ret)
    return ret;
    value &= ~mask;
    value |= bits;
    return ov8865_write(sensor, address, value);
    }
// Sensor
#[no_mangle]
unsafe extern "C" fn ov8865_sw_reset(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_sw_reset(struct ov8865_sensor *sensor)
    {
    return ov8865_write(sensor, OV8865_SW_RESET_REG, OV8865_SW_RESET_RESET);
    }
#[no_mangle]
unsafe extern "C" fn ov8865_sw_standby(sensor: *mut ov8865_sensor, standby: c_int) -> c_int {
    static int ov8865_sw_standby(struct ov8865_sensor *sensor, int standby)
    {
    let mut value: u8 = 0;
    if (!standby)
    value = OV8865_SW_STANDBY_STREAM_ON;
    return ov8865_write(sensor, OV8865_SW_STANDBY_REG, value);
    }
#[no_mangle]
unsafe extern "C" fn ov8865_chip_id_check(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_chip_id_check(struct ov8865_sensor *sensor)
    {
    u16 regs[] = { OV8865_CHIP_ID_HH_REG, OV8865_CHIP_ID_H_REG,
    OV8865_CHIP_ID_L_REG };
    u8 values[] = { OV8865_CHIP_ID_HH_VALUE, OV8865_CHIP_ID_H_VALUE,
    OV8865_CHIP_ID_L_VALUE };
    unsigned int i;
    u8 value;
    int ret;
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    ret = ov8865_read(sensor, regs[i], &value);
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
unsafe extern "C" fn ov8865_charge_pump_configure(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_charge_pump_configure(struct ov8865_sensor *sensor)
    {
    return ov8865_write(sensor, OV8865_PUMP_CLK_DIV_REG,
    OV8865_PUMP_CLK_DIV_PUMP_P(1));
    }
#[no_mangle]
unsafe extern "C" fn ov8865_mipi_configure(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_mipi_configure(struct ov8865_sensor *sensor)
    {
    struct v4l2_mbus_config_mipi_csi2 *bus_mipi_csi2 =
    &sensor.endpoint.bus.mipi_csi2;
    let mut lanes_count: c_uint = bus_mipi_csi2.num_data_lanes;
    int ret;
    ret = ov8865_write(sensor, OV8865_MIPI_SC_CTRL0_REG,
    OV8865_MIPI_SC_CTRL0_LANES(lanes_count) |
    OV8865_MIPI_SC_CTRL0_MIPI_EN |
    OV8865_MIPI_SC_CTRL0_UNKNOWN);
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_MIPI_SC_CTRL2_REG,
    OV8865_MIPI_SC_CTRL2_PD_MIPI_RST_SYNC);
    if (ret)
    return ret;
    if (lanes_count >= 2) {
    ret = ov8865_write(sensor, OV8865_MIPI_LANE_SEL01_REG,
    OV8865_MIPI_LANE_SEL01_LANE0(0) |
    OV8865_MIPI_LANE_SEL01_LANE1(1));
    if (ret)
    return ret;
    }
    if (lanes_count >= 4) {
    ret = ov8865_write(sensor, OV8865_MIPI_LANE_SEL23_REG,
    OV8865_MIPI_LANE_SEL23_LANE2(2) |
    OV8865_MIPI_LANE_SEL23_LANE3(3));
    if (ret)
    return ret;
    }
    ret = ov8865_update_bits(sensor, OV8865_CLK_SEL1_REG,
    OV8865_CLK_SEL1_MIPI_EOF,
    OV8865_CLK_SEL1_MIPI_EOF);
    if (ret)
    return ret;
//
// This value might need to change depending on PCLK rate,
// but it's unclear how. This value seems to generally work
// while the default value was found to cause transmission errors.
//
    return ov8865_write(sensor, OV8865_MIPI_PCLK_PERIOD_REG, 0x16);
    }
#[no_mangle]
unsafe extern "C" fn ov8865_black_level_configure(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_black_level_configure(struct ov8865_sensor *sensor)
    {
    int ret;
// Trigger BLC on relevant events and enable filter.
    ret = ov8865_write(sensor, OV8865_BLC_CTRL0_REG,
    OV8865_BLC_CTRL0_TRIG_RANGE_EN |
    OV8865_BLC_CTRL0_TRIG_FORMAT_EN |
    OV8865_BLC_CTRL0_TRIG_GAIN_EN |
    OV8865_BLC_CTRL0_TRIG_EXPOSURE_EN |
    OV8865_BLC_CTRL0_FILTER_EN);
    if (ret)
    return ret;
// Lower BLC offset trigger threshold.
    ret = ov8865_write(sensor, OV8865_BLC_CTRLD_REG,
    OV8865_BLC_CTRLD_OFFSET_TRIGGER(16));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_CTRL1F_REG, 0);
    if (ret)
    return ret;
// Increase BLC offset maximum limit.
    return ov8865_write(sensor, OV8865_BLC_OFFSET_LIMIT_REG,
    OV8865_BLC_OFFSET_LIMIT(63));
    }
#[no_mangle]
unsafe extern "C" fn ov8865_isp_configure(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_isp_configure(struct ov8865_sensor *sensor)
    {
    int ret;
// Disable lens correction.
    ret = ov8865_write(sensor, OV8865_ISP_CTRL0_REG,
    OV8865_ISP_CTRL0_WHITE_BALANCE_EN |
    OV8865_ISP_CTRL0_DPC_BLACK_EN |
    OV8865_ISP_CTRL0_DPC_WHITE_EN);
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_ISP_CTRL1_REG,
    OV8865_ISP_CTRL1_BLC_EN);
    }
    static unsigned long ov8865_mode_pll1_rate(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode)
    {
    const struct ov8865_pll1_config *config;
    unsigned long pll1_rate;
    config = sensor.pll_configs.pll1_config;
    pll1_rate = sensor.extclk_rate * config.pll_mul / config.pll_pre_div_half;
    switch (config.pll_pre_div) {
    case 0:
    break;
    case 1:
    pll1_rate *= 3;
    pll1_rate /= 2;
    break;
    case 3:
    pll1_rate *= 5;
    pll1_rate /= 2;
    break;
    case 4:
    pll1_rate /= 3;
    break;
    case 5:
    pll1_rate /= 4;
    break;
    case 7:
    pll1_rate /= 8;
    break;
    default:
    pll1_rate /= config.pll_pre_div;
    break;
    }
    return pll1_rate;
    }
    static int ov8865_mode_pll1_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode,
    u32 mbus_code)
    {
    const struct ov8865_pll1_config *config;
    u8 value;
    int ret;
    config = sensor.pll_configs.pll1_config;
    switch (mbus_code) {
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    value = OV8865_MIPI_BIT_SEL(10);
    break;
    default:
    return -EINVAL;
    }
    ret = ov8865_write(sensor, OV8865_MIPI_BIT_SEL_REG, value);
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRLA_REG,
    OV8865_PLL_CTRLA_PRE_DIV_HALF(config.pll_pre_div_half));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL0_REG,
    OV8865_PLL_CTRL0_PRE_DIV(config.pll_pre_div));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL1_REG,
    OV8865_PLL_CTRL1_MUL_H(config.pll_mul));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL2_REG,
    OV8865_PLL_CTRL2_MUL_L(config.pll_mul));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL3_REG,
    OV8865_PLL_CTRL3_M_DIV(config.m_div));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL4_REG,
    OV8865_PLL_CTRL4_MIPI_DIV(config.mipi_div));
    if (ret)
    return ret;
    ret = ov8865_update_bits(sensor, OV8865_PCLK_SEL_REG,
    OV8865_PCLK_SEL_PCLK_DIV_MASK,
    OV8865_PCLK_SEL_PCLK_DIV(config.pclk_div));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL5_REG,
    OV8865_PLL_CTRL5_SYS_PRE_DIV(config.sys_pre_div));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL6_REG,
    OV8865_PLL_CTRL6_SYS_DIV(config.sys_div));
    if (ret)
    return ret;
    return ov8865_update_bits(sensor, OV8865_PLL_CTRL1E_REG,
    OV8865_PLL_CTRL1E_PLL1_NO_LAT,
    OV8865_PLL_CTRL1E_PLL1_NO_LAT);
    }
    static int ov8865_mode_pll2_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode)
    {
    const struct ov8865_pll2_config *config;
    int ret;
    config = mode.pll2_binning ? sensor.pll_configs.pll2_config_binning :
    sensor.pll_configs.pll2_config_native;
    ret = ov8865_write(sensor, OV8865_PLL_CTRL12_REG,
    OV8865_PLL_CTRL12_PRE_DIV_HALF(config.pll_pre_div_half) |
    OV8865_PLL_CTRL12_DAC_DIV(config.dac_div));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRLB_REG,
    OV8865_PLL_CTRLB_PRE_DIV(config.pll_pre_div));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRLC_REG,
    OV8865_PLL_CTRLC_MUL_H(config.pll_mul));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRLD_REG,
    OV8865_PLL_CTRLD_MUL_L(config.pll_mul));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_PLL_CTRLF_REG,
    OV8865_PLL_CTRLF_SYS_PRE_DIV(config.sys_pre_div));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_PLL_CTRLE_REG,
    OV8865_PLL_CTRLE_SYS_DIV(config.sys_div));
    }
    static int ov8865_mode_sclk_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode)
    {
    const struct ov8865_sclk_config *config = &ov8865_sclk_config_native;
    int ret;
    ret = ov8865_write(sensor, OV8865_CLK_SEL0_REG,
    OV8865_CLK_SEL0_PLL1_SYS_SEL(config.sys_sel));
    if (ret)
    return ret;
    ret = ov8865_update_bits(sensor, OV8865_CLK_SEL1_REG,
    OV8865_CLK_SEL1_PLL_SCLK_SEL_MASK,
    OV8865_CLK_SEL1_PLL_SCLK_SEL(config.sclk_sel));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_SCLK_CTRL_REG,
    OV8865_SCLK_CTRL_UNKNOWN |
    OV8865_SCLK_CTRL_SCLK_DIV(config.sclk_div) |
    OV8865_SCLK_CTRL_SCLK_PRE_DIV(config.sclk_pre_div));
    }
    static int ov8865_mode_binning_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode)
    {
    unsigned int variopixel_hsub_coef, variopixel_vsub_coef;
    u8 value;
    int ret;
    ret = ov8865_write(sensor, OV8865_FORMAT1_REG, 0);
    if (ret)
    return ret;
    value = OV8865_FORMAT2_HSYNC_EN;
    if (mode.binning_x)
    value |= OV8865_FORMAT2_FST_HBIN_EN;
    if (mode.binning_y)
    value |= OV8865_FORMAT2_FST_VBIN_EN;
    if (mode.sync_hbin)
    value |= OV8865_FORMAT2_SYNC_HBIN_EN;
    if (mode.horz_var2)
    value |= OV8865_FORMAT2_ISP_HORZ_VAR2_EN;
    ret = ov8865_write(sensor, OV8865_FORMAT2_REG, value);
    if (ret)
    return ret;
    ret = ov8865_update_bits(sensor, OV8865_ISP_CTRL2_REG,
    OV8865_ISP_CTRL2_VARIOPIXEL_EN,
    mode.variopixel ?
    OV8865_ISP_CTRL2_VARIOPIXEL_EN : 0);
    if (ret)
    return ret;
    if (mode.variopixel) {
// VarioPixel coefs needs to be > 1.
    variopixel_hsub_coef = mode.variopixel_hsub_coef;
    variopixel_vsub_coef = mode.variopixel_vsub_coef;
    } else {
    variopixel_hsub_coef = 1;
    variopixel_vsub_coef = 1;
    }
    ret = ov8865_write(sensor, OV8865_VAP_CTRL1_REG,
    OV8865_VAP_CTRL1_HSUB_COEF(variopixel_hsub_coef) |
    OV8865_VAP_CTRL1_VSUB_COEF(variopixel_vsub_coef));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_INC_X_ODD_REG,
    OV8865_INC_X_ODD(mode.inc_x_odd));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_INC_X_EVEN_REG,
    OV8865_INC_X_EVEN(mode.inc_x_even));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_INC_Y_ODD_REG,
    OV8865_INC_Y_ODD(mode.inc_y_odd));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_INC_Y_EVEN_REG,
    OV8865_INC_Y_EVEN(mode.inc_y_even));
    }
    static int ov8865_mode_black_level_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode)
    {
    int ret;
// Note that a zero value for blc_col_shift_mask is the default 256.
    ret = ov8865_write(sensor, OV8865_BLC_CTRL1_REG,
    mode.blc_col_shift_mask |
    OV8865_BLC_CTRL1_OFFSET_LIMIT_EN);
    if (ret)
    return ret;
// BLC top zero line
    ret = ov8865_write(sensor, OV8865_BLC_TOP_ZLINE_START_REG,
    OV8865_BLC_TOP_ZLINE_START(mode.blc_top_zero_line_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_TOP_ZLINE_NUM_REG,
    OV8865_BLC_TOP_ZLINE_NUM(mode.blc_top_zero_line_num));
    if (ret)
    return ret;
// BLC top black line
    ret = ov8865_write(sensor, OV8865_BLC_TOP_BLKLINE_START_REG,
    OV8865_BLC_TOP_BLKLINE_START(mode.blc_top_black_line_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_TOP_BLKLINE_NUM_REG,
    OV8865_BLC_TOP_BLKLINE_NUM(mode.blc_top_black_line_num));
    if (ret)
    return ret;
// BLC bottom zero line
    ret = ov8865_write(sensor, OV8865_BLC_BOT_ZLINE_START_REG,
    OV8865_BLC_BOT_ZLINE_START(mode.blc_bottom_zero_line_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_BOT_ZLINE_NUM_REG,
    OV8865_BLC_BOT_ZLINE_NUM(mode.blc_bottom_zero_line_num));
    if (ret)
    return ret;
// BLC bottom black line
    ret = ov8865_write(sensor, OV8865_BLC_BOT_BLKLINE_START_REG,
    OV8865_BLC_BOT_BLKLINE_START(mode.blc_bottom_black_line_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_BOT_BLKLINE_NUM_REG,
    OV8865_BLC_BOT_BLKLINE_NUM(mode.blc_bottom_black_line_num));
    if (ret)
    return ret;
// BLC anchor
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_LEFT_START_H_REG,
    OV8865_BLC_ANCHOR_LEFT_START_H(mode.blc_anchor_left_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_LEFT_START_L_REG,
    OV8865_BLC_ANCHOR_LEFT_START_L(mode.blc_anchor_left_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_LEFT_END_H_REG,
    OV8865_BLC_ANCHOR_LEFT_END_H(mode.blc_anchor_left_end));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_LEFT_END_L_REG,
    OV8865_BLC_ANCHOR_LEFT_END_L(mode.blc_anchor_left_end));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_RIGHT_START_H_REG,
    OV8865_BLC_ANCHOR_RIGHT_START_H(mode.blc_anchor_right_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_RIGHT_START_L_REG,
    OV8865_BLC_ANCHOR_RIGHT_START_L(mode.blc_anchor_right_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_BLC_ANCHOR_RIGHT_END_H_REG,
    OV8865_BLC_ANCHOR_RIGHT_END_H(mode.blc_anchor_right_end));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_BLC_ANCHOR_RIGHT_END_L_REG,
    OV8865_BLC_ANCHOR_RIGHT_END_L(mode.blc_anchor_right_end));
    }
    static int ov8865_mode_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode, u32 mbus_code)
    {
    int ret;
// Output Size X
    ret = ov8865_write(sensor, OV8865_OUTPUT_SIZE_X_H_REG,
    OV8865_OUTPUT_SIZE_X_H(mode.output_size_x));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_OUTPUT_SIZE_X_L_REG,
    OV8865_OUTPUT_SIZE_X_L(mode.output_size_x));
    if (ret)
    return ret;
// Horizontal Total Size
    ret = ov8865_write(sensor, OV8865_HTS_H_REG, OV8865_HTS_H(mode.hts));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_HTS_L_REG, OV8865_HTS_L(mode.hts));
    if (ret)
    return ret;
// Output Size Y
    ret = ov8865_write(sensor, OV8865_OUTPUT_SIZE_Y_H_REG,
    OV8865_OUTPUT_SIZE_Y_H(mode.output_size_y));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_OUTPUT_SIZE_Y_L_REG,
    OV8865_OUTPUT_SIZE_Y_L(mode.output_size_y));
    if (ret)
    return ret;
// Vertical Total Size
    ret = ov8865_write(sensor, OV8865_VTS_H_REG, OV8865_VTS_H(mode.vts));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_VTS_L_REG, OV8865_VTS_L(mode.vts));
    if (ret)
    return ret;
    if (mode.size_auto) {
// Auto Size
    ret = ov8865_write(sensor, OV8865_AUTO_SIZE_CTRL_REG,
    OV8865_AUTO_SIZE_CTRL_OFFSET_Y_REG |
    OV8865_AUTO_SIZE_CTRL_OFFSET_X_REG |
    OV8865_AUTO_SIZE_CTRL_CROP_END_Y_REG |
    OV8865_AUTO_SIZE_CTRL_CROP_END_X_REG |
    OV8865_AUTO_SIZE_CTRL_CROP_START_Y_REG |
    OV8865_AUTO_SIZE_CTRL_CROP_START_X_REG);
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_AUTO_SIZE_BOUNDARIES_REG,
    OV8865_AUTO_SIZE_BOUNDARIES_Y(mode.size_auto_boundary_y) |
    OV8865_AUTO_SIZE_BOUNDARIES_X(mode.size_auto_boundary_x));
    if (ret)
    return ret;
    } else {
// Crop Start X
    ret = ov8865_write(sensor, OV8865_CROP_START_X_H_REG,
    OV8865_CROP_START_X_H(mode.crop_start_x));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_CROP_START_X_L_REG,
    OV8865_CROP_START_X_L(mode.crop_start_x));
    if (ret)
    return ret;
// Offset X
    ret = ov8865_write(sensor, OV8865_OFFSET_X_H_REG,
    OV8865_OFFSET_X_H(mode.offset_x));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_OFFSET_X_L_REG,
    OV8865_OFFSET_X_L(mode.offset_x));
    if (ret)
    return ret;
// Crop End X
    ret = ov8865_write(sensor, OV8865_CROP_END_X_H_REG,
    OV8865_CROP_END_X_H(mode.crop_end_x));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_CROP_END_X_L_REG,
    OV8865_CROP_END_X_L(mode.crop_end_x));
    if (ret)
    return ret;
// Crop Start Y
    ret = ov8865_write(sensor, OV8865_CROP_START_Y_H_REG,
    OV8865_CROP_START_Y_H(mode.crop_start_y));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_CROP_START_Y_L_REG,
    OV8865_CROP_START_Y_L(mode.crop_start_y));
    if (ret)
    return ret;
// Offset Y
    ret = ov8865_write(sensor, OV8865_OFFSET_Y_H_REG,
    OV8865_OFFSET_Y_H(mode.offset_y));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_OFFSET_Y_L_REG,
    OV8865_OFFSET_Y_L(mode.offset_y));
    if (ret)
    return ret;
// Crop End Y
    ret = ov8865_write(sensor, OV8865_CROP_END_Y_H_REG,
    OV8865_CROP_END_Y_H(mode.crop_end_y));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_CROP_END_Y_L_REG,
    OV8865_CROP_END_Y_L(mode.crop_end_y));
    if (ret)
    return ret;
    }
// VFIFO
    ret = ov8865_write(sensor, OV8865_VFIFO_READ_START_H_REG,
    OV8865_VFIFO_READ_START_H(mode.vfifo_read_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_VFIFO_READ_START_L_REG,
    OV8865_VFIFO_READ_START_L(mode.vfifo_read_start));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_ABLC_NUM_REG,
    OV8865_ABLC_NUM(mode.ablc_num));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_ZLINE_NUM_REG,
    OV8865_ZLINE_NUM(mode.zline_num));
    if (ret)
    return ret;
// Binning
    ret = ov8865_mode_binning_configure(sensor, mode);
    if (ret)
    return ret;
// Black Level
    ret = ov8865_mode_black_level_configure(sensor, mode);
    if (ret)
    return ret;
// PLLs
    ret = ov8865_mode_pll1_configure(sensor, mode, mbus_code);
    if (ret)
    return ret;
    ret = ov8865_mode_pll2_configure(sensor, mode);
    if (ret)
    return ret;
    ret = ov8865_mode_sclk_configure(sensor, mode);
    if (ret)
    return ret;
// Extra registers
    if (mode.register_values) {
    ret = ov8865_write_sequence(sensor, mode.register_values,
    mode.register_values_count);
    if (ret)
    return ret;
    }
    return 0;
    }
    static unsigned long ov8865_mode_mipi_clk_rate(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode)
    {
    const struct ov8865_pll1_config *config;
    unsigned long pll1_rate;
    config = sensor.pll_configs.pll1_config;
    pll1_rate = ov8865_mode_pll1_rate(sensor, mode);
    return pll1_rate / config.m_div / 2;
    }
// Exposure
#[no_mangle]
unsafe extern "C" fn ov8865_exposure_configure(sensor: *mut ov8865_sensor, exposure: u32) -> c_int {
    static int ov8865_exposure_configure(struct ov8865_sensor *sensor, u32 exposure)
    {
    int ret;
// The sensor stores exposure in units of 1/16th of a line
    exposure *= 16;
    ret = ov8865_write(sensor, OV8865_EXPOSURE_CTRL_HH_REG,
    OV8865_EXPOSURE_CTRL_HH(exposure));
    if (ret)
    return ret;
    ret = ov8865_write(sensor, OV8865_EXPOSURE_CTRL_H_REG,
    OV8865_EXPOSURE_CTRL_H(exposure));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_EXPOSURE_CTRL_L_REG,
    OV8865_EXPOSURE_CTRL_L(exposure));
    }
// Gain
#[no_mangle]
unsafe extern "C" fn ov8865_analog_gain_configure(sensor: *mut ov8865_sensor, gain: u32) -> c_int {
    static int ov8865_analog_gain_configure(struct ov8865_sensor *sensor, u32 gain)
    {
    int ret;
    ret = ov8865_write(sensor, OV8865_GAIN_CTRL_H_REG,
    OV8865_GAIN_CTRL_H(gain));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_GAIN_CTRL_L_REG,
    OV8865_GAIN_CTRL_L(gain));
    }
// White Balance
    static int ov8865_red_balance_configure(struct ov8865_sensor *sensor,
    u32 red_balance)
    {
    int ret;
    ret = ov8865_write(sensor, OV8865_ISP_GAIN_RED_H_REG,
    OV8865_ISP_GAIN_RED_H(red_balance));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_ISP_GAIN_RED_L_REG,
    OV8865_ISP_GAIN_RED_L(red_balance));
    }
    static int ov8865_blue_balance_configure(struct ov8865_sensor *sensor,
    u32 blue_balance)
    {
    int ret;
    ret = ov8865_write(sensor, OV8865_ISP_GAIN_BLUE_H_REG,
    OV8865_ISP_GAIN_BLUE_H(blue_balance));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_ISP_GAIN_BLUE_L_REG,
    OV8865_ISP_GAIN_BLUE_L(blue_balance));
    }
// Flip
#[no_mangle]
unsafe extern "C" fn ov8865_flip_vert_configure(sensor: *mut ov8865_sensor, enable: bool) -> c_int {
    static int ov8865_flip_vert_configure(struct ov8865_sensor *sensor, bool enable)
    {
    u8 bits = OV8865_FORMAT1_FLIP_VERT_ISP_EN |
    OV8865_FORMAT1_FLIP_VERT_SENSOR_EN;
    return ov8865_update_bits(sensor, OV8865_FORMAT1_REG, bits,
    enable ? bits : 0);
    }
#[no_mangle]
unsafe extern "C" fn ov8865_flip_horz_configure(sensor: *mut ov8865_sensor, enable: bool) -> c_int {
    static int ov8865_flip_horz_configure(struct ov8865_sensor *sensor, bool enable)
    {
    u8 bits = OV8865_FORMAT2_FLIP_HORZ_ISP_EN |
    OV8865_FORMAT2_FLIP_HORZ_SENSOR_EN;
    return ov8865_update_bits(sensor, OV8865_FORMAT2_REG, bits,
    enable ? bits : 0);
    }
// Test Pattern
    static int ov8865_test_pattern_configure(struct ov8865_sensor *sensor,
    unsigned int index)
    {
    if (index >= ARRAY_SIZE(ov8865_test_pattern_bits))
    return -EINVAL;
    return ov8865_write(sensor, OV8865_PRE_CTRL0_REG,
    ov8865_test_pattern_bits[index]);
    }
// Blanking
#[no_mangle]
unsafe extern "C" fn ov8865_vts_configure(sensor: *mut ov8865_sensor, vblank: u32) -> c_int {
    static int ov8865_vts_configure(struct ov8865_sensor *sensor, u32 vblank)
    {
    let mut vts: u16 = sensor.state.mode.output_size_y + vblank;
    int ret;
    ret = ov8865_write(sensor, OV8865_VTS_H_REG, OV8865_VTS_H(vts));
    if (ret)
    return ret;
    return ov8865_write(sensor, OV8865_VTS_L_REG, OV8865_VTS_L(vts));
    }
// State
    static int ov8865_state_mipi_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode,
    u32 mbus_code)
    {
    struct ov8865_ctrls *ctrls = &sensor.ctrls;
    struct v4l2_mbus_config_mipi_csi2 *bus_mipi_csi2 =
    &sensor.endpoint.bus.mipi_csi2;
    unsigned long mipi_clk_rate;
    unsigned int bits_per_sample;
    unsigned int lanes_count;
    unsigned int i, j;
    s64 mipi_pixel_rate;
    mipi_clk_rate = ov8865_mode_mipi_clk_rate(sensor, mode);
    if (!mipi_clk_rate)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(ov8865_link_freq_menu); i++) {
    let mut freq: i64 = ov8865_link_freq_menu[i];
    if (freq == mipi_clk_rate)
    break;
    }
    for (j = 0; j < sensor.endpoint.nr_of_link_frequencies; j++) {
    let mut freq: u64 = sensor.endpoint.link_frequencies[j];
    if (freq == mipi_clk_rate)
    break;
    }
    if (i == ARRAY_SIZE(ov8865_link_freq_menu)) {
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
    static int ov8865_state_configure(struct ov8865_sensor *sensor,
    const struct ov8865_mode *mode,
    u32 mbus_code)
    {
    int ret;
    if (sensor.state.streaming)
    return -EBUSY;
    ret = ov8865_state_mipi_configure(sensor, mode, mbus_code);
    if (ret)
    return ret;
    sensor.state.mode = mode;
    sensor.state.mbus_code = mbus_code;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov8865_state_init(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_state_init(struct ov8865_sensor *sensor)
    {
    return ov8865_state_configure(sensor, &ov8865_modes[0],
    ov8865_mbus_codes[0]);
    }
// Sensor Base
#[no_mangle]
unsafe extern "C" fn ov8865_sensor_init(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_sensor_init(struct ov8865_sensor *sensor)
    {
    int ret;
    ret = ov8865_sw_reset(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to perform sw reset\n");
    return ret;
    }
    ret = ov8865_sw_standby(sensor, 1);
    if (ret) {
    dev_err(sensor.dev, "failed to set sensor standby\n");
    return ret;
    }
    ret = ov8865_chip_id_check(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to check sensor chip id\n");
    return ret;
    }
    ret = ov8865_write_sequence(sensor, ov8865_init_sequence,
    ARRAY_SIZE(ov8865_init_sequence));
    if (ret) {
    dev_err(sensor.dev, "failed to write init sequence\n");
    return ret;
    }
    ret = ov8865_charge_pump_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure pad\n");
    return ret;
    }
    ret = ov8865_mipi_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure MIPI\n");
    return ret;
    }
    ret = ov8865_isp_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure ISP\n");
    return ret;
    }
    ret = ov8865_black_level_configure(sensor);
    if (ret) {
    dev_err(sensor.dev, "failed to configure black level\n");
    return ret;
    }
// Configure current mode.
    ret = ov8865_mode_configure(sensor, sensor.state.mode,
    sensor.state.mbus_code);
    if (ret) {
    dev_err(sensor.dev, "failed to configure mode\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov8865_sensor_power(sensor: *mut ov8865_sensor, on: bool) -> c_int {
    static int ov8865_sensor_power(struct ov8865_sensor *sensor, bool on)
    {
// Keep initialized to zero for disable label.
    let mut ret: c_int = 0;
    if (on) {
    gpiod_set_value_cansleep(sensor.reset, 1);
    gpiod_set_value_cansleep(sensor.powerdown, 1);
    ret = regulator_enable(sensor.dovdd);
    if (ret) {
    dev_err(sensor.dev,
    "failed to enable DOVDD regulator\n");
    return ret;
    }
    ret = regulator_enable(sensor.avdd);
    if (ret) {
    dev_err(sensor.dev,
    "failed to enable AVDD regulator\n");
    goto disable_dovdd;
    }
    ret = regulator_enable(sensor.dvdd);
    if (ret) {
    dev_err(sensor.dev,
    "failed to enable DVDD regulator\n");
    goto disable_avdd;
    }
    ret = clk_prepare_enable(sensor.extclk);
    if (ret) {
    dev_err(sensor.dev, "failed to enable EXTCLK clock\n");
    goto disable_dvdd;
    }
    gpiod_set_value_cansleep(sensor.reset, 0);
    gpiod_set_value_cansleep(sensor.powerdown, 0);
// Time to enter streaming mode according to power timings.
    usleep_range(10000, 12000);
    } else {
    gpiod_set_value_cansleep(sensor.powerdown, 1);
    gpiod_set_value_cansleep(sensor.reset, 1);
    clk_disable_unprepare(sensor.extclk);
    disable_dvdd:
    regulator_disable(sensor.dvdd);
    disable_avdd:
    regulator_disable(sensor.avdd);
    disable_dovdd:
    regulator_disable(sensor.dovdd);
    }
    return ret;
    }
// Controls
#[no_mangle]
unsafe extern "C" fn ov8865_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov8865_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *subdev = ov8865_ctrl_subdev(ctrl);
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    unsigned int index;
    int ret;
// If VBLANK is altered we need to update exposure to compensate
    if (ctrl.id == V4L2_CID_VBLANK) {
    int exposure_max;
    exposure_max = sensor.state.mode.output_size_y + ctrl.val -
    OV8865_INTEGRATION_TIME_MARGIN;
    __v4l2_ctrl_modify_range(sensor.ctrls.exposure,
    sensor.ctrls.exposure.minimum,
    exposure_max,
    sensor.ctrls.exposure.step,
    min(sensor.ctrls.exposure.val,
    exposure_max));
    }
// Wait for the sensor to be on before setting controls.
    if (pm_runtime_suspended(sensor.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE:
    ret = ov8865_exposure_configure(sensor, ctrl.val);
    if (ret)
    return ret;
    break;
    case V4L2_CID_ANALOGUE_GAIN:
    ret = ov8865_analog_gain_configure(sensor, ctrl.val);
    if (ret)
    return ret;
    break;
    case V4L2_CID_RED_BALANCE:
    return ov8865_red_balance_configure(sensor, ctrl.val);
    case V4L2_CID_BLUE_BALANCE:
    return ov8865_blue_balance_configure(sensor, ctrl.val);
    case V4L2_CID_HFLIP:
    return ov8865_flip_horz_configure(sensor, !!ctrl.val);
    case V4L2_CID_VFLIP:
    return ov8865_flip_vert_configure(sensor, !!ctrl.val);
    case V4L2_CID_TEST_PATTERN:
    index = (unsigned int)ctrl.val;
    return ov8865_test_pattern_configure(sensor, index);
    case V4L2_CID_VBLANK:
    return ov8865_vts_configure(sensor, ctrl.val);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct v4l2_ctrl_ops ov8865_ctrl_ops = {
    .s_ctrl			= ov8865_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov8865_ctrls_init(sensor: *mut ov8865_sensor) -> c_int {
    static int ov8865_ctrls_init(struct ov8865_sensor *sensor)
    {
    struct ov8865_ctrls *ctrls = &sensor.ctrls;
    struct v4l2_ctrl_handler *handler = &ctrls.handler;
    const struct v4l2_ctrl_ops *ops = &ov8865_ctrl_ops;
    const struct ov8865_mode *mode = &ov8865_modes[0];
    struct v4l2_fwnode_device_properties props;
    unsigned int vblank_max, vblank_def;
    unsigned int hblank;
    int ret;
    v4l2_ctrl_handler_init(handler, 32);
// Use our mutex for ctrl locking.
    handler.lock = &sensor.mutex;
// Exposure
    ctrls.exposure = v4l2_ctrl_new_std(handler, ops, V4L2_CID_EXPOSURE, 2,
    65535, 1, 32);
// Gain
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_ANALOGUE_GAIN, 128, 2048, 128,
    128);
// White Balance
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_RED_BALANCE, 1, 32767, 1,
    1024);
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_BLUE_BALANCE, 1, 32767, 1,
    1024);
// Flip
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(handler, ops, V4L2_CID_VFLIP, 0, 1, 1, 0);
// Test Pattern
    v4l2_ctrl_new_std_menu_items(handler, ops, V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov8865_test_pattern_menu) - 1,
    0, 0, ov8865_test_pattern_menu);
// Blanking
    hblank = mode.hts - mode.output_size_x;
    ctrls.hblank = v4l2_ctrl_new_std(handler, ops, V4L2_CID_HBLANK, hblank,
    hblank, 1, hblank);
    if (ctrls.hblank)
    ctrls.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    vblank_max = OV8865_TIMING_MAX_VTS - mode.output_size_y;
    vblank_def = mode.vts - mode.output_size_y;
    ctrls.vblank = v4l2_ctrl_new_std(handler, ops, V4L2_CID_VBLANK,
    OV8865_TIMING_MIN_VTS, vblank_max, 1,
    vblank_def);
// MIPI CSI-2
    ctrls.link_freq =
    v4l2_ctrl_new_int_menu(handler, core::ptr::null_mut(), V4L2_CID_LINK_FREQ,
    ARRAY_SIZE(ov8865_link_freq_menu) - 1,
    0, ov8865_link_freq_menu);
    ctrls.pixel_rate =
    v4l2_ctrl_new_std(handler, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE, 1,
    INT_MAX, 1, 1);
// set properties from fwnode (e.g. rotation, orientation)
    ret = v4l2_fwnode_device_parse(sensor.dev, &props);
    if (ret)
    goto error_ctrls;
    ret = v4l2_ctrl_new_fwnode_properties(handler, ops, &props);
    if (ret)
    goto error_ctrls;
    if (handler.error) {
    ret = handler.error;
    goto error_ctrls;
    }
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
unsafe extern "C" fn ov8865_s_stream(subdev: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ov8865_s_stream(struct v4l2_subdev *subdev, int enable)
    {
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    struct ov8865_state *state = &sensor.state;
    let mut ret: c_int = 0;
    if (enable) {
    ret = pm_runtime_resume_and_get(sensor.dev);
    if (ret < 0)
    return ret;
    }
    mutex_lock(&sensor.mutex);
//
// The sensor may have been kept powered by something else (e.g. the
// VCM's runtime PM device link on IPU3 platforms), in which case
// runtime resume did not run and the hardware may still be
// configured for a previous mode. Always program the negotiated
// configuration on stream start.
//
    if (enable) {
    ret = ov8865_sensor_init(sensor);
    if (!ret)
    ret = __v4l2_ctrl_handler_setup(&sensor.ctrls.handler);
    }
    if (!ret)
    ret = ov8865_sw_standby(sensor, !enable);
    mutex_unlock(&sensor.mutex);
    if (ret || !enable)
    pm_runtime_put(sensor.dev);
    if (!ret)
    state.streaming = enable;
    return ret;
    }
    static const struct v4l2_subdev_video_ops ov8865_subdev_video_ops = {
    .s_stream		= ov8865_s_stream,
    };
// Subdev Pad Operations
    static int ov8865_enum_mbus_code(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code_enum)
    {
    if (code_enum.index >= ARRAY_SIZE(ov8865_mbus_codes))
    return -EINVAL;
    code_enum.code = ov8865_mbus_codes[code_enum.index];
    return 0;
    }
    static void ov8865_mbus_format_fill(struct v4l2_mbus_framefmt *mbus_format,
    u32 mbus_code,
    const struct ov8865_mode *mode)
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
    static int ov8865_get_fmt(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    struct v4l2_mbus_framefmt *mbus_format = &format.format;
    mutex_lock(&sensor.mutex);
    if (format.which == V4L2_SUBDEV_FORMAT_TRY)
// mbus_format = *v4l2_subdev_state_get_format(sd_state,
    format.pad);
    else
    ov8865_mbus_format_fill(mbus_format, sensor.state.mbus_code,
    sensor.state.mode);
    mutex_unlock(&sensor.mutex);
    return 0;
    }
    static int ov8865_set_fmt(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    struct v4l2_mbus_framefmt *mbus_format = &format.format;
    const struct ov8865_mode *mode;
    let mut mbus_code: u32 = 0;
    unsigned int hblank;
    unsigned int index;
    int exposure_max;
    let mut ret: c_int = 0;
    mutex_lock(&sensor.mutex);
    if (sensor.state.streaming) {
    ret = -EBUSY;
    goto complete;
    }
// Try to find requested mbus code.
    for (index = 0; index < ARRAY_SIZE(ov8865_mbus_codes); index++) {
    if (ov8865_mbus_codes[index] == mbus_format.code) {
    mbus_code = mbus_format.code;
    break;
    }
    }
// Fallback to default.
    if (!mbus_code)
    mbus_code = ov8865_mbus_codes[0];
// Find the mode with nearest dimensions.
    mode = v4l2_find_nearest_size(ov8865_modes, ARRAY_SIZE(ov8865_modes),
    output_size_x, output_size_y,
    mbus_format.width, mbus_format.height);
    if (!mode) {
    ret = -EINVAL;
    goto complete;
    }
    ov8865_mbus_format_fill(mbus_format, mbus_code, mode);
    if (format.which == V4L2_SUBDEV_FORMAT_TRY)
// v4l2_subdev_state_get_format(sd_state, format->pad) =
// mbus_format;
    else if (sensor.state.mode != mode ||
    sensor.state.mbus_code != mbus_code)
    ret = ov8865_state_configure(sensor, mode, mbus_code);
    __v4l2_ctrl_modify_range(sensor.ctrls.vblank, OV8865_TIMING_MIN_VTS,
    OV8865_TIMING_MAX_VTS - mode.output_size_y,
    1, mode.vts - mode.output_size_y);
    hblank = mode.hts - mode.output_size_x;
    __v4l2_ctrl_modify_range(sensor.ctrls.hblank, hblank, hblank, 1,
    hblank);
    exposure_max = mode.vts - OV8865_INTEGRATION_TIME_MARGIN;
    __v4l2_ctrl_modify_range(sensor.ctrls.exposure,
    sensor.ctrls.exposure.minimum, exposure_max,
    sensor.ctrls.exposure.step,
    min(sensor.ctrls.exposure.val,
    exposure_max));
    complete:
    mutex_unlock(&sensor.mutex);
    return ret;
    }
    static int ov8865_enum_frame_size(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *size_enum)
    {
    const struct ov8865_mode *mode;
    if (size_enum.index >= ARRAY_SIZE(ov8865_modes))
    return -EINVAL;
    mode = &ov8865_modes[size_enum.index];
    size_enum.min_width = size_enum.max_width = mode.output_size_x;
    size_enum.min_height = size_enum.max_height = mode.output_size_y;
    return 0;
    }
    static void
    __ov8865_get_pad_crop(struct ov8865_sensor *sensor,
    struct v4l2_subdev_state *state, unsigned int pad,
    enum v4l2_subdev_format_whence which, struct v4l2_rect *r)
    {
    const struct ov8865_mode *mode = sensor.state.mode;
    switch (which) {
    case V4L2_SUBDEV_FORMAT_TRY:
// r = *v4l2_subdev_state_get_crop(state, pad);
    break;
    case V4L2_SUBDEV_FORMAT_ACTIVE:
    r.height = mode.output_size_y;
    r.width = mode.output_size_x;
    r.top = (OV8865_NATIVE_HEIGHT - mode.output_size_y) / 2;
    r.left = (OV8865_NATIVE_WIDTH - mode.output_size_x) / 2;
    break;
    }
    }
    static int ov8865_get_selection(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    mutex_lock(&sensor.mutex);
    __ov8865_get_pad_crop(sensor, state, sel.pad,
    sel.which, &sel.r);
    mutex_unlock(&sensor.mutex);
    break;
    case V4L2_SEL_TGT_NATIVE_SIZE:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = OV8865_NATIVE_WIDTH;
    sel.r.height = OV8865_NATIVE_HEIGHT;
    break;
    case V4L2_SEL_TGT_CROP_BOUNDS:
    case V4L2_SEL_TGT_CROP_DEFAULT:
    sel.r.top = OV8865_ACTIVE_START_TOP;
    sel.r.left = OV8865_ACTIVE_START_LEFT;
    sel.r.width = OV8865_ACTIVE_WIDTH;
    sel.r.height = OV8865_ACTIVE_HEIGHT;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ov8865_get_frame_interval(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *interval)
    {
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    const struct ov8865_mode *mode;
    unsigned int framesize;
    unsigned int fps;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (interval.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    mutex_lock(&sensor.mutex);
    mode = sensor.state.mode;
    framesize = mode.hts * (mode.output_size_y +
    sensor.ctrls.vblank.val);
    fps = DIV_ROUND_CLOSEST(sensor.ctrls.pixel_rate.val, framesize);
    interval.interval.numerator = 1;
    interval.interval.denominator = fps;
    mutex_unlock(&sensor.mutex);
    return 0;
    }
    static const struct v4l2_subdev_pad_ops ov8865_subdev_pad_ops = {
    .enum_mbus_code		= ov8865_enum_mbus_code,
    .get_fmt		= ov8865_get_fmt,
    .set_fmt		= ov8865_set_fmt,
    .enum_frame_size	= ov8865_enum_frame_size,
    .get_selection		= ov8865_get_selection,
    .set_selection		= ov8865_get_selection,
    .get_frame_interval	= ov8865_get_frame_interval,
    .set_frame_interval	= ov8865_get_frame_interval,
    };
    static const struct v4l2_subdev_ops ov8865_subdev_ops = {
    .video		= &ov8865_subdev_video_ops,
    .pad		= &ov8865_subdev_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn ov8865_suspend(dev: *mut device) -> c_int {
    static int ov8865_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    struct ov8865_state *state = &sensor.state;
    let mut ret: c_int = 0;
    mutex_lock(&sensor.mutex);
    if (state.streaming) {
    ret = ov8865_sw_standby(sensor, true);
    if (ret)
    goto complete;
    }
    ret = ov8865_sensor_power(sensor, false);
    if (ret)
    ov8865_sw_standby(sensor, false);
    complete:
    mutex_unlock(&sensor.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov8865_resume(dev: *mut device) -> c_int {
    static int ov8865_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    struct ov8865_state *state = &sensor.state;
    let mut ret: c_int = 0;
    mutex_lock(&sensor.mutex);
    ret = ov8865_sensor_power(sensor, true);
    if (ret)
    goto complete;
    if (state.streaming) {
    ret = ov8865_sensor_init(sensor);
    if (ret)
    goto error_power;
    ret = __v4l2_ctrl_handler_setup(&sensor.ctrls.handler);
    if (ret)
    goto error_power;
    ret = ov8865_sw_standby(sensor, false);
    if (ret)
    goto error_power;
    }
    goto complete;
    error_power:
    ov8865_sensor_power(sensor, false);
    complete:
    mutex_unlock(&sensor.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov8865_probe(client: *mut i2c_client) -> c_int {
    static int ov8865_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct fwnode_handle *handle;
    struct ov8865_sensor *sensor;
    struct v4l2_subdev *subdev;
    struct media_pad *pad;
    unsigned int i;
    int ret;
    sensor = devm_kzalloc(dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.dev = dev;
    sensor.i2c_client = client;
// Regulators
// DVDD: digital core
    sensor.dvdd = devm_regulator_get(dev, "dvdd");
    if (IS_ERR(sensor.dvdd))
    return dev_err_probe(dev, PTR_ERR(sensor.dvdd),
    "cannot get DVDD regulator\n");
// DOVDD: digital I/O
    sensor.dovdd = devm_regulator_get(dev, "dovdd");
    if (IS_ERR(sensor.dovdd))
    return dev_err_probe(dev, PTR_ERR(sensor.dovdd),
    "cannot get DOVDD regulator\n");
// AVDD: analog
    sensor.avdd = devm_regulator_get(dev, "avdd");
    if (IS_ERR(sensor.avdd))
    return dev_err_probe(dev, PTR_ERR(sensor.avdd),
    "cannot get AVDD (analog) regulator\n");
// Graph Endpoint
    handle = fwnode_graph_get_next_endpoint(dev_fwnode(dev), core::ptr::null_mut());
    if (!handle)
    return dev_err_probe(dev, -EPROBE_DEFER,
    "waiting for fwnode graph endpoint\n");
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
// External Clock
    sensor.extclk = devm_v4l2_sensor_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(sensor.extclk)) {
    ret = dev_err_probe(dev, PTR_ERR(sensor.extclk),
    "failed to get external clock\n");
    goto error_endpoint;
    }
    sensor.extclk_rate = clk_get_rate(sensor.extclk);
    for (i = 0; i < ARRAY_SIZE(supported_extclk_rates); i++) {
    if (sensor.extclk_rate == supported_extclk_rates[i])
    break;
    }
    if (i == ARRAY_SIZE(supported_extclk_rates)) {
    dev_err(dev, "clock rate %lu Hz is unsupported\n",
    sensor.extclk_rate);
    ret = -EINVAL;
    goto error_endpoint;
    }
    sensor.pll_configs = ov8865_pll_configs[i];
// Subdev, entity and pad
    subdev = &sensor.subdev;
    v4l2_i2c_subdev_init(subdev, client, &ov8865_subdev_ops);
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
    ret = ov8865_ctrls_init(sensor);
    if (ret)
    goto error_mutex;
    mutex_lock(&sensor.mutex);
    ret = ov8865_state_init(sensor);
    mutex_unlock(&sensor.mutex);
    if (ret)
    goto error_ctrls;
// Runtime PM
    pm_runtime_set_suspended(sensor.dev);
    pm_runtime_enable(sensor.dev);
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
unsafe extern "C" fn ov8865_remove(client: *mut i2c_client) {
    static void ov8865_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ov8865_sensor *sensor = ov8865_subdev_sensor(subdev);
    v4l2_async_unregister_subdev(subdev);
    pm_runtime_disable(sensor.dev);
    v4l2_ctrl_handler_free(&sensor.ctrls.handler);
    mutex_destroy(&sensor.mutex);
    media_entity_cleanup(&subdev.entity);
    v4l2_fwnode_endpoint_free(&sensor.endpoint);
    }
    static const struct dev_pm_ops ov8865_pm_ops = {
    SET_RUNTIME_PM_OPS(ov8865_suspend, ov8865_resume, core::ptr::null_mut())
    };
    static const struct acpi_device_id ov8865_acpi_match[] = {
    {"INT347A"},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ov8865_acpi_match);
    static const struct of_device_id ov8865_of_match[] = {
    { .compatible = "ovti,ov8865" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ov8865_of_match);
    static struct i2c_driver ov8865_driver = {
    .driver = {
    .name = "ov8865",
    .of_match_table = ov8865_of_match,
    .acpi_match_table = ov8865_acpi_match,
    .pm = &ov8865_pm_ops,
    },
    .probe = ov8865_probe,
    .remove = ov8865_remove,
    };
    module_i2c_driver(ov8865_driver);
    MODULE_AUTHOR("Paul Kocialkowski <paul.kocialkowski@bootlin.com>");
    MODULE_DESCRIPTION("V4L2 driver for the OmniVision OV8865 image sensor");
    MODULE_LICENSE("GPL v2");
