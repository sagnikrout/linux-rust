//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov2640.c
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
// ov2640 Camera Driver
//
// Copyright (C) 2010 Alberto Panizzo <maramaopercheseimorto@gmail.com>
//
// Based on ov772x, ov9640 drivers and previous non merged implementations.
//
// Copyright 2005-2009 Freescale Semiconductor, Inc. All Rights Reserved.
// Copyright (C) 2006, OmniVision
//

    ((((x) >> rshift) & mask) << lshift)
//
// DSP registers
// register offset for BANK_SEL == BANK_SEL_DSP
//
pub const R_BYPASS: c_uint = 0x05 /* Bypass DSP */;
pub const R_BYPASS_DSP_BYPAS: c_uint = 0x01 /* Bypass DSP, sensor out directly */;
pub const R_BYPASS_USE_DSP: c_uint = 0x00 /* Use the internal DSP */;
pub const QS: c_uint = 0x44 /* Quantization Scale Factor */;
pub const CTRLI: c_uint = 0x50;
pub const CTRLI_LP_DP: c_uint = 0x80;
pub const CTRLI_ROUND: c_uint = 0x40;

pub const HSIZE: c_uint = 0x51 /* H_SIZE[7:0] (real/4) */;

pub const VSIZE: c_uint = 0x52 /* V_SIZE[7:0] (real/4) */;

pub const XOFFL: c_uint = 0x53 /* OFFSET_X[7:0] */;

pub const YOFFL: c_uint = 0x54 /* OFFSET_Y[7:0] */;

pub const VHYX: c_uint = 0x55 /* Offset and size completion */;

pub const DPRP: c_uint = 0x56;
pub const TEST: c_uint = 0x57 /* Horizontal size completion */;

pub const ZMOW: c_uint = 0x5A /* Zoom: Out Width  OUTW[7:0] (real/4) */;

pub const ZMOH: c_uint = 0x5B /* Zoom: Out Height OUTH[7:0] (real/4) */;

pub const ZMHH: c_uint = 0x5C /* Zoom: Speed and H&W completion */;

pub const BPADDR: c_uint = 0x7C /* SDE Indirect Register Access: Address */;
pub const BPDATA: c_uint = 0x7D /* SDE Indirect Register Access: Data */;
pub const CTRL2: c_uint = 0x86 /* DSP Module enable 2 */;
pub const CTRL2_DCW_EN: c_uint = 0x20;
pub const CTRL2_SDE_EN: c_uint = 0x10;
pub const CTRL2_UV_ADJ_EN: c_uint = 0x08;
pub const CTRL2_UV_AVG_EN: c_uint = 0x04;
pub const CTRL2_CMX_EN: c_uint = 0x01;
pub const CTRL3: c_uint = 0x87 /* DSP Module enable 3 */;
pub const CTRL3_BPC_EN: c_uint = 0x80;
pub const CTRL3_WPC_EN: c_uint = 0x40;
pub const SIZEL: c_uint = 0x8C /* Image Size Completion */;

pub const HSIZE8: c_uint = 0xC0 /* Image Horizontal Size HSIZE[10:3] */;

pub const VSIZE8: c_uint = 0xC1 /* Image Vertical Size VSIZE[10:3] */;

pub const CTRL0: c_uint = 0xC2 /* DSP Module enable 0 */;
pub const CTRL0_AEC_EN: c_uint = 0x80;
pub const CTRL0_AEC_SEL: c_uint = 0x40;
pub const CTRL0_STAT_SEL: c_uint = 0x20;
pub const CTRL0_VFIRST: c_uint = 0x10;
pub const CTRL0_YUV422: c_uint = 0x08;
pub const CTRL0_YUV_EN: c_uint = 0x04;
pub const CTRL0_RGB_EN: c_uint = 0x02;
pub const CTRL0_RAW_EN: c_uint = 0x01;
pub const CTRL1: c_uint = 0xC3 /* DSP Module enable 1 */;
pub const CTRL1_CIP: c_uint = 0x80;
pub const CTRL1_DMY: c_uint = 0x40;
pub const CTRL1_RAW_GMA: c_uint = 0x20;
pub const CTRL1_DG: c_uint = 0x10;
pub const CTRL1_AWB: c_uint = 0x08;
pub const CTRL1_AWB_GAIN: c_uint = 0x04;
pub const CTRL1_LENC: c_uint = 0x02;
pub const CTRL1_PRE: c_uint = 0x01;
// REG 0xC7 (unknown name): affects Auto White Balance (AWB)
// AWB_OFF            0x40
// AWB_SIMPLE         0x10
// AWB_ON             0x00	(Advanced AWB ?)
pub const R_DVP_SP: c_uint = 0xD3 /* DVP output speed control */;
pub const R_DVP_SP_AUTO_MODE: c_uint = 0x80;
pub const R_DVP_SP_DVP_MASK: c_uint = 0x3F /* DVP PCLK = sysclk (48)/[6:0] (YUV0);;
// = sysclk (48)/(2*[6:0]) (RAW);
pub const IMAGE_MODE: c_uint = 0xDA /* Image Output Format Select */;
pub const IMAGE_MODE_Y8_DVP_EN: c_uint = 0x40;
pub const IMAGE_MODE_JPEG_EN: c_uint = 0x10;
pub const IMAGE_MODE_YUV422: c_uint = 0x00;
pub const IMAGE_MODE_RAW10: c_uint = 0x04 /* (DVP) */;
pub const IMAGE_MODE_RGB565: c_uint = 0x08;
pub const IMAGE_MODE_HREF_VSYNC: c_uint = 0x02 /* HREF timing select in DVP JPEG output;
// mode (0 for HREF is same as sensor)
pub const IMAGE_MODE_LBYTE_FIRST: c_uint = 0x01 /* Byte swap enable for DVP;
// 1: Low byte first UYVY (C2[4] =0)
// VYUY (C2[4] =1)
// 0: High byte first YUYV (C2[4]=0)
// YVYU (C2[4] = 1)
pub const RESET: c_uint = 0xE0 /* Reset */;
pub const RESET_MICROC: c_uint = 0x40;
pub const RESET_SCCB: c_uint = 0x20;
pub const RESET_JPEG: c_uint = 0x10;
pub const RESET_DVP: c_uint = 0x04;
pub const RESET_IPU: c_uint = 0x02;
pub const RESET_CIF: c_uint = 0x01;
pub const REGED: c_uint = 0xED /* Register ED */;
pub const REGED_CLK_OUT_DIS: c_uint = 0x10;
pub const MS_SP: c_uint = 0xF0 /* SCCB Master Speed */;
pub const SS_ID: c_uint = 0xF7 /* SCCB Slave ID */;
pub const SS_CTRL: c_uint = 0xF8 /* SCCB Slave Control */;
pub const SS_CTRL_ADD_AUTO_INC: c_uint = 0x20;
pub const SS_CTRL_EN: c_uint = 0x08;
pub const SS_CTRL_DELAY_CLK: c_uint = 0x04;
pub const SS_CTRL_ACC_EN: c_uint = 0x02;
pub const SS_CTRL_SEN_PASS_THR: c_uint = 0x01;
pub const MC_BIST: c_uint = 0xF9 /* Microcontroller misc register */;
pub const MC_BIST_RESET: c_uint = 0x80 /* Microcontroller Reset */;
pub const MC_BIST_BOOT_ROM_SEL: c_uint = 0x40;
pub const MC_BIST_12KB_SEL: c_uint = 0x20;
pub const MC_BIST_12KB_MASK: c_uint = 0x30;
pub const MC_BIST_512KB_SEL: c_uint = 0x08;
pub const MC_BIST_512KB_MASK: c_uint = 0x0C;
pub const MC_BIST_BUSY_BIT_R: c_uint = 0x02;
pub const MC_BIST_MC_RES_ONE_SH_W: c_uint = 0x02;
pub const MC_BIST_LAUNCH: c_uint = 0x01;
pub const BANK_SEL: c_uint = 0xFF /* Register Bank Select */;
pub const BANK_SEL_DSP: c_uint = 0x00;
pub const BANK_SEL_SENS: c_uint = 0x01;
//
// Sensor registers
// register offset for BANK_SEL == BANK_SEL_SENS
//
pub const GAIN: c_uint = 0x00 /* AGC - Gain control gain setting */;
pub const COM1: c_uint = 0x03 /* Common control 1 */;
pub const COM1_1_DUMMY_FR: c_uint = 0x40;
pub const COM1_3_DUMMY_FR: c_uint = 0x80;
pub const COM1_7_DUMMY_FR: c_uint = 0xC0;
pub const COM1_VWIN_LSB_UXGA: c_uint = 0x0F;
pub const COM1_VWIN_LSB_SVGA: c_uint = 0x0A;
pub const COM1_VWIN_LSB_CIF: c_uint = 0x06;
pub const REG04: c_uint = 0x04 /* Register 04 */;
pub const REG04_DEF: c_uint = 0x20 /* Always set */;
pub const REG04_HFLIP_IMG: c_uint = 0x80 /* Horizontal mirror image ON/OFF */;
pub const REG04_VFLIP_IMG: c_uint = 0x40 /* Vertical flip image ON/OFF */;
pub const REG04_VREF_EN: c_uint = 0x10;
pub const REG04_HREF_EN: c_uint = 0x08;

pub const REG08: c_uint = 0x08 /* Frame Exposure One-pin Control Pre-charge Row Num */;
pub const COM2: c_uint = 0x09 /* Common control 2 */;
pub const COM2_SOFT_SLEEP_MODE: c_uint = 0x10 /* Soft sleep mode */;
// Output drive capability

pub const PID: c_uint = 0x0A /* Product ID Number MSB */;
pub const VER: c_uint = 0x0B /* Product ID Number LSB */;
pub const COM3: c_uint = 0x0C /* Common control 3 */;
pub const COM3_BAND_50H: c_uint = 0x04 /* 0 For Banding at 60H */;
pub const COM3_BAND_AUTO: c_uint = 0x02 /* Auto Banding */;
pub const COM3_SING_FR_SNAPSH: c_uint = 0x01 /* 0 For enable live video output after the;
// snapshot sequence
pub const AEC: c_uint = 0x10 /* AEC[9:2] Exposure Value */;
pub const CLKRC: c_uint = 0x11 /* Internal clock */;
pub const CLKRC_EN: c_uint = 0x80;

pub const COM7: c_uint = 0x12 /* Common control 7 */;
pub const COM7_SRST: c_uint = 0x80 /* Initiates system reset. All registers are;
// set to factory default values after which
// the chip resumes normal operation
pub const COM7_RES_UXGA: c_uint = 0x00 /* Resolution selectors for UXGA */;
pub const COM7_RES_SVGA: c_uint = 0x40 /* SVGA */;
pub const COM7_RES_CIF: c_uint = 0x20 /* CIF */;
pub const COM7_ZOOM_EN: c_uint = 0x04 /* Enable Zoom mode */;
pub const COM7_COLOR_BAR_TEST: c_uint = 0x02 /* Enable Color Bar Test Pattern */;
pub const COM8: c_uint = 0x13 /* Common control 8 */;
pub const COM8_DEF: c_uint = 0xC0;
pub const COM8_BNDF_EN: c_uint = 0x20 /* Banding filter ON/OFF */;
pub const COM8_AGC_EN: c_uint = 0x04 /* AGC Auto/Manual control selection */;
pub const COM8_AEC_EN: c_uint = 0x01 /* Auto/Manual Exposure control */;
pub const COM9: c_uint = 0x14 /* Common control 9;
// Automatic gain ceiling - maximum AGC value [7:5]
pub const COM9_AGC_GAIN_2x: c_uint = 0x00 /* 000 :   2x */;
pub const COM9_AGC_GAIN_4x: c_uint = 0x20 /* 001 :   4x */;
pub const COM9_AGC_GAIN_8x: c_uint = 0x40 /* 010 :   8x */;
pub const COM9_AGC_GAIN_16x: c_uint = 0x60 /* 011 :  16x */;
pub const COM9_AGC_GAIN_32x: c_uint = 0x80 /* 100 :  32x */;
pub const COM9_AGC_GAIN_64x: c_uint = 0xA0 /* 101 :  64x */;
pub const COM9_AGC_GAIN_128x: c_uint = 0xC0 /* 110 : 128x */;
pub const COM10: c_uint = 0x15 /* Common control 10 */;
pub const COM10_PCLK_HREF: c_uint = 0x20 /* PCLK output qualified by HREF */;
pub const COM10_PCLK_RISE: c_uint = 0x10 /* Data is updated at the rising edge of;
// PCLK (user can latch data at the next
// falling edge of PCLK).
// 0 otherwise.
pub const COM10_HREF_INV: c_uint = 0x08 /* Invert HREF polarity:;
// HREF negative for valid data
pub const COM10_VSINC_INV: c_uint = 0x02 /* Invert VSYNC polarity */;
pub const HSTART: c_uint = 0x17 /* Horizontal Window start MSB 8 bit */;
pub const HEND: c_uint = 0x18 /* Horizontal Window end MSB 8 bit */;
pub const VSTART: c_uint = 0x19 /* Vertical Window start MSB 8 bit */;
pub const VEND: c_uint = 0x1A /* Vertical Window end MSB 8 bit */;
pub const MIDH: c_uint = 0x1C /* Manufacturer ID byte - high */;
pub const MIDL: c_uint = 0x1D /* Manufacturer ID byte - low  */;
pub const AEW: c_uint = 0x24 /* AGC/AEC - Stable operating region (upper limit) */;
pub const AEB: c_uint = 0x25 /* AGC/AEC - Stable operating region (lower limit) */;
pub const VV: c_uint = 0x26 /* AGC/AEC Fast mode operating region */;

pub const REG2A: c_uint = 0x2A /* Dummy pixel insert MSB */;
pub const FRARL: c_uint = 0x2B /* Dummy pixel insert LSB */;
pub const ADDVFL: c_uint = 0x2D /* LSB of insert dummy lines in Vertical direction */;
pub const ADDVFH: c_uint = 0x2E /* MSB of insert dummy lines in Vertical direction */;
pub const YAVG: c_uint = 0x2F /* Y/G Channel Average value */;
pub const REG32: c_uint = 0x32 /* Common Control 32 */;
pub const REG32_PCLK_DIV_2: c_uint = 0x80 /* PCLK freq divided by 2 */;
pub const REG32_PCLK_DIV_4: c_uint = 0xC0 /* PCLK freq divided by 4 */;
pub const ARCOM2: c_uint = 0x34 /* Zoom: Horizontal start point */;
pub const REG45: c_uint = 0x45 /* Register 45 */;
pub const FLL: c_uint = 0x46 /* Frame Length Adjustment LSBs */;
pub const FLH: c_uint = 0x47 /* Frame Length Adjustment MSBs */;
pub const COM19: c_uint = 0x48 /* Zoom: Vertical start point */;
pub const ZOOMS: c_uint = 0x49 /* Zoom: Vertical start point */;
pub const COM22: c_uint = 0x4B /* Flash light control */;
pub const COM25: c_uint = 0x4E /* For Banding operations */;
pub const COM25_50HZ_BANDING_AEC_MSBS_MASK: c_uint = 0xC0 /* 50Hz Bd. AEC 2 MSBs */;
pub const COM25_60HZ_BANDING_AEC_MSBS_MASK: c_uint = 0x30 /* 60Hz Bd. AEC 2 MSBs */;

pub const BD50: c_uint = 0x4F /* 50Hz Banding AEC 8 LSBs */;

pub const BD60: c_uint = 0x50 /* 60Hz Banding AEC 8 LSBs */;

pub const REG5A: c_uint = 0x5A /* 50/60Hz Banding Maximum AEC Step */;
pub const BD50_MAX_AEC_STEP_MASK: c_uint = 0xF0 /* 50Hz Banding Max. AEC Step */;
pub const BD60_MAX_AEC_STEP_MASK: c_uint = 0x0F /* 60Hz Banding Max. AEC Step */;

pub const REG5D: c_uint = 0x5D /* AVGsel[7:0],   16-zone average weight option */;
pub const REG5E: c_uint = 0x5E /* AVGsel[15:8],  16-zone average weight option */;
pub const REG5F: c_uint = 0x5F /* AVGsel[23:16], 16-zone average weight option */;
pub const REG60: c_uint = 0x60 /* AVGsel[31:24], 16-zone average weight option */;
pub const HISTO_LOW: c_uint = 0x61 /* Histogram Algorithm Low Level */;
pub const HISTO_HIGH: c_uint = 0x62 /* Histogram Algorithm High Level */;
//
// ID
//
pub const MANUFACTURER_ID: c_uint = 0x7FA2;
pub const PID_OV2640: c_uint = 0x2642;

//
// Struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regval_list {
    pub reg_num: u8,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov2640_win_size {
    pub name: *mut c_char,
    pub width: u32,
    pub height: u32,
    pub regs: *const regval_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov2640_priv {
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub hdl: v4l2_ctrl_handler,
    pub cfmt_code: u32,
    pub clk: *mut clk,
    pub win: *const ov2640_win_size,
    pub resetb_gpio: *mut gpio_desc,
    pub pwdn_gpio: *mut gpio_desc,
    pub /: *mut *mut mutex lock; / lock to protect streaming and power_count,
    pub streaming: bool,
    pub power_count: c_int,
}

//
// Registers settings
//

    static const struct regval_list ov2640_init_regs[] = {
    { BANK_SEL, BANK_SEL_DSP },
    { 0x2c,   0xff },
    { 0x2e,   0xdf },
    { BANK_SEL, BANK_SEL_SENS },
    { 0x3c,   0x32 },
    { CLKRC,  CLKRC_DIV_SET(1) },
    { COM2,   COM2_OCAP_Nx_SET(3) },
    { REG04,  REG04_DEF | REG04_HREF_EN },
    { COM8,   COM8_DEF | COM8_BNDF_EN | COM8_AGC_EN | COM8_AEC_EN },
    { COM9,   COM9_AGC_GAIN_8x | 0x08},
    { 0x2c,   0x0c },
    { 0x33,   0x78 },
    { 0x3a,   0x33 },
    { 0x3b,   0xfb },
    { 0x3e,   0x00 },
    { 0x43,   0x11 },
    { 0x16,   0x10 },
    { 0x39,   0x02 },
    { 0x35,   0x88 },
    { 0x22,   0x0a },
    { 0x37,   0x40 },
    { 0x23,   0x00 },
    { ARCOM2, 0xa0 },
    { 0x06,   0x02 },
    { 0x06,   0x88 },
    { 0x07,   0xc0 },
    { 0x0d,   0xb7 },
    { 0x0e,   0x01 },
    { 0x4c,   0x00 },
    { 0x4a,   0x81 },
    { 0x21,   0x99 },
    { AEW,    0x40 },
    { AEB,    0x38 },
    { VV,     VV_HIGH_TH_SET(0x08) | VV_LOW_TH_SET(0x02) },
    { 0x5c,   0x00 },
    { 0x63,   0x00 },
    { FLL,    0x22 },
    { COM3,   0x38 | COM3_BAND_AUTO },
    { REG5D,  0x55 },
    { REG5E,  0x7d },
    { REG5F,  0x7d },
    { REG60,  0x55 },
    { HISTO_LOW,   0x70 },
    { HISTO_HIGH,  0x80 },
    { 0x7c,   0x05 },
    { 0x20,   0x80 },
    { 0x28,   0x30 },
    { 0x6c,   0x00 },
    { 0x6d,   0x80 },
    { 0x6e,   0x00 },
    { 0x70,   0x02 },
    { 0x71,   0x94 },
    { 0x73,   0xc1 },
    { 0x3d,   0x34 },
    { COM7,   COM7_RES_UXGA | COM7_ZOOM_EN },
    { REG5A,  BD50_MAX_AEC_STEP_SET(6)
    | BD60_MAX_AEC_STEP_SET(8) },		/* 0x57 */
    { COM25,  COM25_50HZ_BANDING_AEC_MSBS_SET(0x0bb)
    | COM25_60HZ_BANDING_AEC_MSBS_SET(0x09c) },	/* 0x00 */
    { BD50,   BD50_50HZ_BANDING_AEC_LSBS_SET(0x0bb) },	/* 0xbb */
    { BD60,   BD60_60HZ_BANDING_AEC_LSBS_SET(0x09c) },	/* 0x9c */
    { BANK_SEL,  BANK_SEL_DSP },
    { 0xe5,   0x7f },
    { MC_BIST,  MC_BIST_RESET | MC_BIST_BOOT_ROM_SEL },
    { 0x41,   0x24 },
    { RESET,  RESET_JPEG | RESET_DVP },
    { 0x76,   0xff },
    { 0x33,   0xa0 },
    { 0x42,   0x20 },
    { 0x43,   0x18 },
    { 0x4c,   0x00 },
    { CTRL3,  CTRL3_BPC_EN | CTRL3_WPC_EN | 0x10 },
    { 0x88,   0x3f },
    { 0xd7,   0x03 },
    { 0xd9,   0x10 },
    { R_DVP_SP,  R_DVP_SP_AUTO_MODE | 0x2 },
    { 0xc8,   0x08 },
    { 0xc9,   0x80 },
    { BPADDR, 0x00 },
    { BPDATA, 0x00 },
    { BPADDR, 0x03 },
    { BPDATA, 0x48 },
    { BPDATA, 0x48 },
    { BPADDR, 0x08 },
    { BPDATA, 0x20 },
    { BPDATA, 0x10 },
    { BPDATA, 0x0e },
    { 0x90,   0x00 },
    { 0x91,   0x0e },
    { 0x91,   0x1a },
    { 0x91,   0x31 },
    { 0x91,   0x5a },
    { 0x91,   0x69 },
    { 0x91,   0x75 },
    { 0x91,   0x7e },
    { 0x91,   0x88 },
    { 0x91,   0x8f },
    { 0x91,   0x96 },
    { 0x91,   0xa3 },
    { 0x91,   0xaf },
    { 0x91,   0xc4 },
    { 0x91,   0xd7 },
    { 0x91,   0xe8 },
    { 0x91,   0x20 },
    { 0x92,   0x00 },
    { 0x93,   0x06 },
    { 0x93,   0xe3 },
    { 0x93,   0x03 },
    { 0x93,   0x03 },
    { 0x93,   0x00 },
    { 0x93,   0x02 },
    { 0x93,   0x00 },
    { 0x93,   0x00 },
    { 0x93,   0x00 },
    { 0x93,   0x00 },
    { 0x93,   0x00 },
    { 0x93,   0x00 },
    { 0x93,   0x00 },
    { 0x96,   0x00 },
    { 0x97,   0x08 },
    { 0x97,   0x19 },
    { 0x97,   0x02 },
    { 0x97,   0x0c },
    { 0x97,   0x24 },
    { 0x97,   0x30 },
    { 0x97,   0x28 },
    { 0x97,   0x26 },
    { 0x97,   0x02 },
    { 0x97,   0x98 },
    { 0x97,   0x80 },
    { 0x97,   0x00 },
    { 0x97,   0x00 },
    { 0xa4,   0x00 },
    { 0xa8,   0x00 },
    { 0xc5,   0x11 },
    { 0xc6,   0x51 },
    { 0xbf,   0x80 },
    { 0xc7,   0x10 },	/* simple AWB */
    { 0xb6,   0x66 },
    { 0xb8,   0xA5 },
    { 0xb7,   0x64 },
    { 0xb9,   0x7C },
    { 0xb3,   0xaf },
    { 0xb4,   0x97 },
    { 0xb5,   0xFF },
    { 0xb0,   0xC5 },
    { 0xb1,   0x94 },
    { 0xb2,   0x0f },
    { 0xc4,   0x5c },
    { 0xa6,   0x00 },
    { 0xa7,   0x20 },
    { 0xa7,   0xd8 },
    { 0xa7,   0x1b },
    { 0xa7,   0x31 },
    { 0xa7,   0x00 },
    { 0xa7,   0x18 },
    { 0xa7,   0x20 },
    { 0xa7,   0xd8 },
    { 0xa7,   0x19 },
    { 0xa7,   0x31 },
    { 0xa7,   0x00 },
    { 0xa7,   0x18 },
    { 0xa7,   0x20 },
    { 0xa7,   0xd8 },
    { 0xa7,   0x19 },
    { 0xa7,   0x31 },
    { 0xa7,   0x00 },
    { 0xa7,   0x18 },
    { 0x7f,   0x00 },
    { 0xe5,   0x1f },
    { 0xe1,   0x77 },
    { 0xdd,   0x7f },
    { CTRL0,  CTRL0_YUV422 | CTRL0_YUV_EN | CTRL0_RGB_EN },
    ENDMARKER,
    };
//
// Register settings for window size
// The preamble, setup the internal DSP to input an UXGA (1600x1200) image.
// Then the different zooming configurations will setup the output image size.
//
    static const struct regval_list ov2640_size_change_preamble_regs[] = {
    { BANK_SEL, BANK_SEL_DSP },
    { RESET, RESET_DVP },
    { SIZEL, SIZEL_HSIZE8_11_SET(UXGA_WIDTH) |
    SIZEL_HSIZE8_SET(UXGA_WIDTH) |
    SIZEL_VSIZE8_SET(UXGA_HEIGHT) },
    { HSIZE8, HSIZE8_SET(UXGA_WIDTH) },
    { VSIZE8, VSIZE8_SET(UXGA_HEIGHT) },
    { CTRL2, CTRL2_DCW_EN | CTRL2_SDE_EN |
    CTRL2_UV_AVG_EN | CTRL2_CMX_EN | CTRL2_UV_ADJ_EN },
    { HSIZE, HSIZE_SET(UXGA_WIDTH) },
    { VSIZE, VSIZE_SET(UXGA_HEIGHT) },
    { XOFFL, XOFFL_SET(0) },
    { YOFFL, YOFFL_SET(0) },
    { VHYX, VHYX_HSIZE_SET(UXGA_WIDTH) | VHYX_VSIZE_SET(UXGA_HEIGHT) |
    VHYX_XOFF_SET(0) | VHYX_YOFF_SET(0)},
    { TEST, TEST_HSIZE_SET(UXGA_WIDTH) },
    ENDMARKER,
    };

    { CTRLI, CTRLI_LP_DP | CTRLI_V_DIV_SET(v_div) |	\
    CTRLI_H_DIV_SET(h_div)},		\
    { ZMOW, ZMOW_OUTW_SET(x) },			\
    { ZMOH, ZMOH_OUTH_SET(y) },			\
    { ZMHH, ZMHH_OUTW_SET(x) | ZMHH_OUTH_SET(y) },	\
    { R_DVP_SP, pclk_div },				\
    { RESET, 0x00}
    static const struct regval_list ov2640_qcif_regs[] = {
    PER_SIZE_REG_SEQ(QCIF_WIDTH, QCIF_HEIGHT, 3, 3, 4),
    ENDMARKER,
    };
    static const struct regval_list ov2640_qvga_regs[] = {
    PER_SIZE_REG_SEQ(QVGA_WIDTH, QVGA_HEIGHT, 2, 2, 4),
    ENDMARKER,
    };
    static const struct regval_list ov2640_cif_regs[] = {
    PER_SIZE_REG_SEQ(CIF_WIDTH, CIF_HEIGHT, 2, 2, 8),
    ENDMARKER,
    };
    static const struct regval_list ov2640_vga_regs[] = {
    PER_SIZE_REG_SEQ(VGA_WIDTH, VGA_HEIGHT, 0, 0, 2),
    ENDMARKER,
    };
    static const struct regval_list ov2640_svga_regs[] = {
    PER_SIZE_REG_SEQ(SVGA_WIDTH, SVGA_HEIGHT, 1, 1, 2),
    ENDMARKER,
    };
    static const struct regval_list ov2640_xga_regs[] = {
    PER_SIZE_REG_SEQ(XGA_WIDTH, XGA_HEIGHT, 0, 0, 2),
    { CTRLI,    0x00},
    ENDMARKER,
    };
    static const struct regval_list ov2640_sxga_regs[] = {
    PER_SIZE_REG_SEQ(SXGA_WIDTH, SXGA_HEIGHT, 0, 0, 2),
    { CTRLI,    0x00},
    { R_DVP_SP, 2 | R_DVP_SP_AUTO_MODE },
    ENDMARKER,
    };
    static const struct regval_list ov2640_uxga_regs[] = {
    PER_SIZE_REG_SEQ(UXGA_WIDTH, UXGA_HEIGHT, 0, 0, 0),
    { CTRLI,    0x00},
    { R_DVP_SP, 0 | R_DVP_SP_AUTO_MODE },
    ENDMARKER,
    };

    {.name = n, .width = w , .height = h, .regs = r }
    static const struct ov2640_win_size ov2640_supported_win_sizes[] = {
    OV2640_SIZE("QCIF", QCIF_WIDTH, QCIF_HEIGHT, ov2640_qcif_regs),
    OV2640_SIZE("QVGA", QVGA_WIDTH, QVGA_HEIGHT, ov2640_qvga_regs),
    OV2640_SIZE("CIF", CIF_WIDTH, CIF_HEIGHT, ov2640_cif_regs),
    OV2640_SIZE("VGA", VGA_WIDTH, VGA_HEIGHT, ov2640_vga_regs),
    OV2640_SIZE("SVGA", SVGA_WIDTH, SVGA_HEIGHT, ov2640_svga_regs),
    OV2640_SIZE("XGA", XGA_WIDTH, XGA_HEIGHT, ov2640_xga_regs),
    OV2640_SIZE("SXGA", SXGA_WIDTH, SXGA_HEIGHT, ov2640_sxga_regs),
    OV2640_SIZE("UXGA", UXGA_WIDTH, UXGA_HEIGHT, ov2640_uxga_regs),
    };
//
// Register settings for pixel formats
//
    static const struct regval_list ov2640_format_change_preamble_regs[] = {
    { BANK_SEL, BANK_SEL_DSP },
    { R_BYPASS, R_BYPASS_USE_DSP },
    ENDMARKER,
    };
    static const struct regval_list ov2640_yuyv_regs[] = {
    { IMAGE_MODE, IMAGE_MODE_YUV422 },
    { 0xd7, 0x03 },
    { 0x33, 0xa0 },
    { 0xe5, 0x1f },
    { 0xe1, 0x67 },
    { RESET,  0x00 },
    { R_BYPASS, R_BYPASS_USE_DSP },
    ENDMARKER,
    };
    static const struct regval_list ov2640_uyvy_regs[] = {
    { IMAGE_MODE, IMAGE_MODE_LBYTE_FIRST | IMAGE_MODE_YUV422 },
    { 0xd7, 0x01 },
    { 0x33, 0xa0 },
    { 0xe1, 0x67 },
    { RESET,  0x00 },
    { R_BYPASS, R_BYPASS_USE_DSP },
    ENDMARKER,
    };
    static const struct regval_list ov2640_rgb565_be_regs[] = {
    { IMAGE_MODE, IMAGE_MODE_RGB565 },
    { 0xd7, 0x03 },
    { RESET,  0x00 },
    { R_BYPASS, R_BYPASS_USE_DSP },
    ENDMARKER,
    };
    static const struct regval_list ov2640_rgb565_le_regs[] = {
    { IMAGE_MODE, IMAGE_MODE_LBYTE_FIRST | IMAGE_MODE_RGB565 },
    { 0xd7, 0x03 },
    { RESET,  0x00 },
    { R_BYPASS, R_BYPASS_USE_DSP },
    ENDMARKER,
    };
    static u32 ov2640_codes[] = {
    MEDIA_BUS_FMT_YUYV8_2X8,
    MEDIA_BUS_FMT_UYVY8_2X8,
    MEDIA_BUS_FMT_YVYU8_2X8,
    MEDIA_BUS_FMT_VYUY8_2X8,
    MEDIA_BUS_FMT_RGB565_2X8_BE,
    MEDIA_BUS_FMT_RGB565_2X8_LE,
    };
//
// General functions
//
    static struct ov2640_priv *to_ov2640(const struct i2c_client *client)
    {
    return container_of(i2c_get_clientdata(client), struct ov2640_priv,
    subdev);
    }
    static int ov2640_write_array(struct i2c_client *client,
    const struct regval_list *vals)
    {
    int ret;
    while ((vals.reg_num != 0xff) || (vals.value != 0xff)) {
    ret = i2c_smbus_write_byte_data(client,
    vals.reg_num, vals.value);
    dev_vdbg(&client.dev, "array: 0x%02x, 0x%02x",
    vals.reg_num, vals.value);
    if (ret < 0)
    return ret;
    vals++;
    }
    return 0;
    }
    static int ov2640_mask_set(struct i2c_client *client,
    u8  reg, u8  mask, u8  set)
    {
    let mut val: i32 = i2c_smbus_read_byte_data(client, reg);
    if (val < 0)
    return val;
    val &= ~mask;
    val |= set & mask;
    dev_vdbg(&client.dev, "masks: 0x%02x, 0x%02x", reg, val);
    return i2c_smbus_write_byte_data(client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn ov2640_reset(client: *mut i2c_client) -> c_int {
    static int ov2640_reset(struct i2c_client *client)
    {
    int ret;
    static const struct regval_list reset_seq[] = {
    {BANK_SEL, BANK_SEL_SENS},
    {COM7, COM7_SRST},
    ENDMARKER,
    };
    ret = ov2640_write_array(client, reset_seq);
    if (ret)
    goto err;
    msleep(5);
    err:
    dev_dbg(&client.dev, "%s: (ret %d)", __func__, ret);
    return ret;
    }
    static const char * const ov2640_test_pattern_menu[] = {
    "Disabled",
    "Eight Vertical Colour Bars",
    };
//
// functions
//
#[no_mangle]
unsafe extern "C" fn ov2640_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov2640_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd =
    &container_of(ctrl.handler, struct ov2640_priv, hdl).subdev;
    struct i2c_client  *client = v4l2_get_subdevdata(sd);
    struct ov2640_priv *priv = to_ov2640(client);
    u8 val;
    int ret;
// v4l2_ctrl_lock() locks our own mutex
//
// If the device is not powered up by the host driver, do not apply any
// controls to H/W at this time. Instead the controls will be restored
// when the streaming is started.
//
    if (!priv.power_count)
    return 0;
    ret = i2c_smbus_write_byte_data(client, BANK_SEL, BANK_SEL_SENS);
    if (ret < 0)
    return ret;
    switch (ctrl.id) {
    case V4L2_CID_VFLIP:
    val = ctrl.val ? REG04_VFLIP_IMG | REG04_VREF_EN : 0x00;
    return ov2640_mask_set(client, REG04,
    REG04_VFLIP_IMG | REG04_VREF_EN, val);
// NOTE: REG04_VREF_EN: 1 line shift / even/odd line swap
    case V4L2_CID_HFLIP:
    val = ctrl.val ? REG04_HFLIP_IMG : 0x00;
    return ov2640_mask_set(client, REG04, REG04_HFLIP_IMG, val);
    case V4L2_CID_TEST_PATTERN:
    val = ctrl.val ? COM7_COLOR_BAR_TEST : 0x00;
    return ov2640_mask_set(client, COM7, COM7_COLOR_BAR_TEST, val);
    }
    return -EINVAL;
    }

    static int ov2640_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int ret;
    reg.size = 1;
    if (reg.reg > 0xff)
    return -EINVAL;
    ret = i2c_smbus_read_byte_data(client, reg.reg);
    if (ret < 0)
    return ret;
    reg.val = ret;
    return 0;
    }
    static int ov2640_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    if (reg.reg > 0xff ||
    reg.val > 0xff)
    return -EINVAL;
    return i2c_smbus_write_byte_data(client, reg.reg, reg.val);
    }

#[no_mangle]
unsafe extern "C" fn ov2640_set_power(priv: *mut ov2640_priv, on: c_int) {
    static void ov2640_set_power(struct ov2640_priv *priv, int on)
    {

    if (priv.pwdn_gpio)
    gpiod_direction_output(priv.pwdn_gpio, !on);
    if (on && priv.resetb_gpio) {
// Active the resetb pin to perform a reset pulse
    gpiod_direction_output(priv.resetb_gpio, 1);
    usleep_range(3000, 5000);
    gpiod_set_value(priv.resetb_gpio, 0);
    }

    }
#[no_mangle]
unsafe extern "C" fn ov2640_s_power(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int ov2640_s_power(struct v4l2_subdev *sd, int on)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ov2640_priv *priv = to_ov2640(client);
    mutex_lock(&priv.lock);
//
// If the power count is modified from 0 to != 0 or from != 0 to 0,
// update the power state.
//
    if (priv.power_count == !on)
    ov2640_set_power(priv, on);
    priv.power_count += on ? 1 : -1;
    WARN_ON(priv.power_count < 0);
    mutex_unlock(&priv.lock);
    return 0;
    }
// Select the nearest higher resolution for capture
    static const struct ov2640_win_size *ov2640_select_win(u32 width, u32 height)
    {
    int i, default_size = ARRAY_SIZE(ov2640_supported_win_sizes) - 1;
    for (i = 0; i < ARRAY_SIZE(ov2640_supported_win_sizes); i++) {
    if (ov2640_supported_win_sizes[i].width  >= width &&
    ov2640_supported_win_sizes[i].height >= height)
    return &ov2640_supported_win_sizes[i];
    }
    return &ov2640_supported_win_sizes[default_size];
    }
    static int ov2640_set_params(struct i2c_client *client,
    const struct ov2640_win_size *win, u32 code)
    {
    const struct regval_list *selected_cfmt_regs;
    u8 val;
    int ret;
    switch (code) {
    case MEDIA_BUS_FMT_RGB565_2X8_BE:
    dev_dbg(&client.dev, "%s: Selected cfmt RGB565 BE", __func__);
    selected_cfmt_regs = ov2640_rgb565_be_regs;
    break;
    case MEDIA_BUS_FMT_RGB565_2X8_LE:
    dev_dbg(&client.dev, "%s: Selected cfmt RGB565 LE", __func__);
    selected_cfmt_regs = ov2640_rgb565_le_regs;
    break;
    case MEDIA_BUS_FMT_YUYV8_2X8:
    dev_dbg(&client.dev, "%s: Selected cfmt YUYV (YUV422)", __func__);
    selected_cfmt_regs = ov2640_yuyv_regs;
    break;
    case MEDIA_BUS_FMT_UYVY8_2X8:
    default:
    dev_dbg(&client.dev, "%s: Selected cfmt UYVY", __func__);
    selected_cfmt_regs = ov2640_uyvy_regs;
    break;
    case MEDIA_BUS_FMT_YVYU8_2X8:
    dev_dbg(&client.dev, "%s: Selected cfmt YVYU", __func__);
    selected_cfmt_regs = ov2640_yuyv_regs;
    break;
    case MEDIA_BUS_FMT_VYUY8_2X8:
    dev_dbg(&client.dev, "%s: Selected cfmt VYUY", __func__);
    selected_cfmt_regs = ov2640_uyvy_regs;
    break;
    }
// reset hardware
    ov2640_reset(client);
// initialize the sensor with default data
    dev_dbg(&client.dev, "%s: Init default", __func__);
    ret = ov2640_write_array(client, ov2640_init_regs);
    if (ret < 0)
    goto err;
// select preamble
    dev_dbg(&client.dev, "%s: Set size to %s", __func__, win.name);
    ret = ov2640_write_array(client, ov2640_size_change_preamble_regs);
    if (ret < 0)
    goto err;
// set size win
    ret = ov2640_write_array(client, win.regs);
    if (ret < 0)
    goto err;
// cfmt preamble
    dev_dbg(&client.dev, "%s: Set cfmt", __func__);
    ret = ov2640_write_array(client, ov2640_format_change_preamble_regs);
    if (ret < 0)
    goto err;
// set cfmt
    ret = ov2640_write_array(client, selected_cfmt_regs);
    if (ret < 0)
    goto err;
    val = (code == MEDIA_BUS_FMT_YVYU8_2X8)
    || (code == MEDIA_BUS_FMT_VYUY8_2X8) ? CTRL0_VFIRST : 0x00;
    ret = ov2640_mask_set(client, CTRL0, CTRL0_VFIRST, val);
    if (ret < 0)
    goto err;
    return 0;
    err:
    dev_err(&client.dev, "%s: Error %d", __func__, ret);
    ov2640_reset(client);
    return ret;
    }
    static int ov2640_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct i2c_client  *client = v4l2_get_subdevdata(sd);
    struct ov2640_priv *priv = to_ov2640(client);
    if (format.pad)
    return -EINVAL;
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
    mf = v4l2_subdev_state_get_format(sd_state, 0);
    format.format = *mf;
    return 0;
    }
    mf.width	= priv.win.width;
    mf.height	= priv.win.height;
    mf.code	= priv.cfmt_code;
    mf.colorspace	= V4L2_COLORSPACE_SRGB;
    mf.field	= V4L2_FIELD_NONE;
    mf.ycbcr_enc	= V4L2_YCBCR_ENC_DEFAULT;
    mf.quantization = V4L2_QUANTIZATION_DEFAULT;
    mf.xfer_func	= V4L2_XFER_FUNC_DEFAULT;
    return 0;
    }
    static int ov2640_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ov2640_priv *priv = to_ov2640(client);
    const struct ov2640_win_size *win;
    let mut ret: c_int = 0;
    if (format.pad)
    return -EINVAL;
    mutex_lock(&priv.lock);
// select suitable win
    win = ov2640_select_win(mf.width, mf.height);
    mf.width	= win.width;
    mf.height	= win.height;
    mf.field	= V4L2_FIELD_NONE;
    mf.colorspace	= V4L2_COLORSPACE_SRGB;
    mf.ycbcr_enc	= V4L2_YCBCR_ENC_DEFAULT;
    mf.quantization = V4L2_QUANTIZATION_DEFAULT;
    mf.xfer_func	= V4L2_XFER_FUNC_DEFAULT;
    switch (mf.code) {
    case MEDIA_BUS_FMT_RGB565_2X8_BE:
    case MEDIA_BUS_FMT_RGB565_2X8_LE:
    case MEDIA_BUS_FMT_YUYV8_2X8:
    case MEDIA_BUS_FMT_UYVY8_2X8:
    case MEDIA_BUS_FMT_YVYU8_2X8:
    case MEDIA_BUS_FMT_VYUY8_2X8:
    break;
    default:
    mf.code = MEDIA_BUS_FMT_UYVY8_2X8;
    break;
    }
    if (format.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    struct ov2640_priv *priv = to_ov2640(client);
    if (priv.streaming) {
    ret = -EBUSY;
    goto out;
    }
// select win
    priv.win = win;
// select format
    priv.cfmt_code = mf.code;
    } else {
// v4l2_subdev_state_get_format(sd_state, 0) = *mf;
    }
    out:
    mutex_unlock(&priv.lock);
    return ret;
    }
    static int ov2640_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    struct v4l2_mbus_framefmt *try_fmt =
    v4l2_subdev_state_get_format(sd_state, 0);
    const struct ov2640_win_size *win =
    ov2640_select_win(SVGA_WIDTH, SVGA_HEIGHT);
    try_fmt.width = win.width;
    try_fmt.height = win.height;
    try_fmt.code = MEDIA_BUS_FMT_UYVY8_2X8;
    try_fmt.colorspace = V4L2_COLORSPACE_SRGB;
    try_fmt.field = V4L2_FIELD_NONE;
    try_fmt.ycbcr_enc = V4L2_YCBCR_ENC_DEFAULT;
    try_fmt.quantization = V4L2_QUANTIZATION_DEFAULT;
    try_fmt.xfer_func = V4L2_XFER_FUNC_DEFAULT;
    return 0;
    }
    static int ov2640_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index >= ARRAY_SIZE(ov2640_codes))
    return -EINVAL;
    code.code = ov2640_codes[code.index];
    return 0;
    }
    static int ov2640_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP_BOUNDS:
    case V4L2_SEL_TGT_CROP:
    sel.r.left = 0;
    sel.r.top = 0;
    sel.r.width = UXGA_WIDTH;
    sel.r.height = UXGA_HEIGHT;
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ov2640_s_stream(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int ov2640_s_stream(struct v4l2_subdev *sd, int on)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ov2640_priv *priv = to_ov2640(client);
    let mut ret: c_int = 0;
    mutex_lock(&priv.lock);
    if (priv.streaming == !on) {
    if (on) {
    ret = ov2640_set_params(client, priv.win,
    priv.cfmt_code);
    if (!ret)
    ret = __v4l2_ctrl_handler_setup(&priv.hdl);
    }
    }
    if (!ret)
    priv.streaming = on;
    mutex_unlock(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2640_video_probe(client: *mut i2c_client) -> c_int {
    static int ov2640_video_probe(struct i2c_client *client)
    {
    struct ov2640_priv *priv = to_ov2640(client);
    u8 pid, ver, midh, midl;
    const char *devname;
    int ret;
    ret = ov2640_s_power(&priv.subdev, 1);
    if (ret < 0)
    return ret;
//
// check and show product ID and manufacturer ID
//
    i2c_smbus_write_byte_data(client, BANK_SEL, BANK_SEL_SENS);
    pid  = i2c_smbus_read_byte_data(client, PID);
    ver  = i2c_smbus_read_byte_data(client, VER);
    midh = i2c_smbus_read_byte_data(client, MIDH);
    midl = i2c_smbus_read_byte_data(client, MIDL);
    switch (VERSION(pid, ver)) {
    case PID_OV2640:
    devname     = "ov2640";
    break;
    default:
    dev_err(&client.dev,
    "Product ID error %x:%x\n", pid, ver);
    ret = -ENODEV;
    goto done;
    }
    dev_info(&client.dev,
    "%s Product ID %0x:%0x Manufacturer ID %x:%x\n",
    devname, pid, ver, midh, midl);
    done:
    ov2640_s_power(&priv.subdev, 0);
    return ret;
    }
    static const struct v4l2_ctrl_ops ov2640_ctrl_ops = {
    .s_ctrl = ov2640_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops ov2640_subdev_core_ops = {
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,

    .g_register	= ov2640_g_register,
    .s_register	= ov2640_s_register,

    .s_power	= ov2640_s_power,
    };
    static const struct v4l2_subdev_pad_ops ov2640_subdev_pad_ops = {
    .enum_mbus_code = ov2640_enum_mbus_code,
    .get_selection	= ov2640_get_selection,
    .get_fmt	= ov2640_get_fmt,
    .set_fmt	= ov2640_set_fmt,
    };
    static const struct v4l2_subdev_video_ops ov2640_subdev_video_ops = {
    .s_stream = ov2640_s_stream,
    };
    static const struct v4l2_subdev_ops ov2640_subdev_ops = {
    .core	= &ov2640_subdev_core_ops,
    .pad	= &ov2640_subdev_pad_ops,
    .video	= &ov2640_subdev_video_ops,
    };
    static const struct v4l2_subdev_internal_ops ov2640_internal_ops = {
    .init_state	= ov2640_init_state,
    };
    static int ov2640_probe_dt(struct i2c_client *client,
    struct ov2640_priv *priv)
    {
    int ret;
// Request the reset GPIO deasserted
    priv.resetb_gpio = devm_gpiod_get_optional(&client.dev, "resetb",
    GPIOD_OUT_LOW);
    if (!priv.resetb_gpio)
    dev_dbg(&client.dev, "resetb gpio is not assigned!\n");
    ret = PTR_ERR_OR_ZERO(priv.resetb_gpio);
    if (ret && ret != -ENOSYS) {
    dev_dbg(&client.dev,
    "Error %d while getting resetb gpio\n", ret);
    return ret;
    }
// Request the power down GPIO asserted
    priv.pwdn_gpio = devm_gpiod_get_optional(&client.dev, "pwdn",
    GPIOD_OUT_HIGH);
    if (!priv.pwdn_gpio)
    dev_dbg(&client.dev, "pwdn gpio is not assigned!\n");
    ret = PTR_ERR_OR_ZERO(priv.pwdn_gpio);
    if (ret && ret != -ENOSYS) {
    dev_dbg(&client.dev,
    "Error %d while getting pwdn gpio\n", ret);
    return ret;
    }
    return 0;
    }
//
// i2c_driver functions
//
#[no_mangle]
unsafe extern "C" fn ov2640_probe(client: *mut i2c_client) -> c_int {
    static int ov2640_probe(struct i2c_client *client)
    {
    struct ov2640_priv	*priv;
    struct i2c_adapter	*adapter = client.adapter;
    int			ret;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(&adapter.dev,
    "OV2640: I2C-Adapter doesn't support SMBUS\n");
    return -EIO;
    }
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    if (client.dev.of_node) {
    priv.clk = devm_clk_get_enabled(&client.dev, "xvclk");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    }
    ret = ov2640_probe_dt(client, priv);
    if (ret)
    return ret;
    priv.win = ov2640_select_win(SVGA_WIDTH, SVGA_HEIGHT);
    priv.cfmt_code = MEDIA_BUS_FMT_UYVY8_2X8;
    v4l2_i2c_subdev_init(&priv.subdev, client, &ov2640_subdev_ops);
    priv.subdev.internal_ops = &ov2640_internal_ops;
    priv.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE |
    V4L2_SUBDEV_FL_HAS_EVENTS;
    mutex_init(&priv.lock);
    v4l2_ctrl_handler_init(&priv.hdl, 3);
    priv.hdl.lock = &priv.lock;
    v4l2_ctrl_new_std(&priv.hdl, &ov2640_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ov2640_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std_menu_items(&priv.hdl, &ov2640_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov2640_test_pattern_menu) - 1, 0, 0,
    ov2640_test_pattern_menu);
    priv.subdev.ctrl_handler = &priv.hdl;
    if (priv.hdl.error) {
    ret = priv.hdl.error;
    goto err_hdl;
    }
    priv.pad.flags = MEDIA_PAD_FL_SOURCE;
    priv.subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&priv.subdev.entity, 1, &priv.pad);
    if (ret < 0)
    goto err_hdl;
    ret = ov2640_video_probe(client);
    if (ret < 0)
    goto err_videoprobe;
    ret = v4l2_async_register_subdev(&priv.subdev);
    if (ret < 0)
    goto err_videoprobe;
    dev_info(&adapter.dev, "OV2640 Probed\n");
    return 0;
    err_videoprobe:
    media_entity_cleanup(&priv.subdev.entity);
    err_hdl:
    v4l2_ctrl_handler_free(&priv.hdl);
    mutex_destroy(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2640_remove(client: *mut i2c_client) {
    static void ov2640_remove(struct i2c_client *client)
    {
    struct ov2640_priv       *priv = to_ov2640(client);
    v4l2_async_unregister_subdev(&priv.subdev);
    v4l2_ctrl_handler_free(&priv.hdl);
    mutex_destroy(&priv.lock);
    media_entity_cleanup(&priv.subdev.entity);
    v4l2_device_unregister_subdev(&priv.subdev);
    }
    static const struct i2c_device_id ov2640_id[] = {
    { .name = "ov2640" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ov2640_id);
    static const struct of_device_id ov2640_of_match[] = {
    {.compatible = "ovti,ov2640", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ov2640_of_match);
    static struct i2c_driver ov2640_i2c_driver = {
    .driver = {
    .name = "ov2640",
    .of_match_table = ov2640_of_match,
    },
    .probe    = ov2640_probe,
    .remove   = ov2640_remove,
    .id_table = ov2640_id,
    };
    module_i2c_driver(ov2640_i2c_driver);
    MODULE_DESCRIPTION("Driver for Omni Vision 2640 sensor");
    MODULE_AUTHOR("Alberto Panizzo");
    MODULE_LICENSE("GPL v2");
