//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/saa711x_regs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// saa711x - Philips SAA711x video decoder register specifications
//
// Copyright (c) 2006 Mauro Carvalho Chehab <mchehab@kernel.org>
//
pub const R_00_CHIP_VERSION: c_uint = 0x00;
// Video Decoder
// Video Decoder - Frontend part
pub const R_01_INC_DELAY: c_uint = 0x01;
pub const R_02_INPUT_CNTL_1: c_uint = 0x02;
pub const R_03_INPUT_CNTL_2: c_uint = 0x03;
pub const R_04_INPUT_CNTL_3: c_uint = 0x04;
pub const R_05_INPUT_CNTL_4: c_uint = 0x05;
// Video Decoder - Decoder part
pub const R_06_H_SYNC_START: c_uint = 0x06;
pub const R_07_H_SYNC_STOP: c_uint = 0x07;
pub const R_08_SYNC_CNTL: c_uint = 0x08;
pub const R_09_LUMA_CNTL: c_uint = 0x09;
pub const R_0A_LUMA_BRIGHT_CNTL: c_uint = 0x0a;
pub const R_0B_LUMA_CONTRAST_CNTL: c_uint = 0x0b;
pub const R_0C_CHROMA_SAT_CNTL: c_uint = 0x0c;
pub const R_0D_CHROMA_HUE_CNTL: c_uint = 0x0d;
pub const R_0E_CHROMA_CNTL_1: c_uint = 0x0e;
pub const R_0F_CHROMA_GAIN_CNTL: c_uint = 0x0f;
pub const R_10_CHROMA_CNTL_2: c_uint = 0x10;
pub const R_11_MODE_DELAY_CNTL: c_uint = 0x11;
pub const R_12_RT_SIGNAL_CNTL: c_uint = 0x12;
pub const R_13_RT_X_PORT_OUT_CNTL: c_uint = 0x13;
pub const R_14_ANAL_ADC_COMPAT_CNTL: c_uint = 0x14;
pub const R_15_VGATE_START_FID_CHG: c_uint = 0x15;
pub const R_16_VGATE_STOP: c_uint = 0x16;
pub const R_17_MISC_VGATE_CONF_AND_MSB: c_uint = 0x17;
pub const R_18_RAW_DATA_GAIN_CNTL: c_uint = 0x18;
pub const R_19_RAW_DATA_OFF_CNTL: c_uint = 0x19;
pub const R_1A_COLOR_KILL_LVL_CNTL: c_uint = 0x1a;
pub const R_1B_MISC_TVVCRDET: c_uint = 0x1b;
pub const R_1C_ENHAN_COMB_CTRL1: c_uint = 0x1c;
pub const R_1D_ENHAN_COMB_CTRL2: c_uint = 0x1d;
pub const R_1E_STATUS_BYTE_1_VD_DEC: c_uint = 0x1e;
pub const R_1F_STATUS_BYTE_2_VD_DEC: c_uint = 0x1f;
// Component processing and interrupt masking part
pub const R_23_INPUT_CNTL_5: c_uint = 0x23;
pub const R_24_INPUT_CNTL_6: c_uint = 0x24;
pub const R_25_INPUT_CNTL_7: c_uint = 0x25;
pub const R_29_COMP_DELAY: c_uint = 0x29;
pub const R_2A_COMP_BRIGHT_CNTL: c_uint = 0x2a;
pub const R_2B_COMP_CONTRAST_CNTL: c_uint = 0x2b;
pub const R_2C_COMP_SAT_CNTL: c_uint = 0x2c;
pub const R_2D_INTERRUPT_MASK_1: c_uint = 0x2d;
pub const R_2E_INTERRUPT_MASK_2: c_uint = 0x2e;
pub const R_2F_INTERRUPT_MASK_3: c_uint = 0x2f;
// Audio clock generator part
pub const R_30_AUD_MAST_CLK_CYCLES_PER_FIELD: c_uint = 0x30;
pub const R_34_AUD_MAST_CLK_NOMINAL_INC: c_uint = 0x34;
pub const R_38_CLK_RATIO_AMXCLK_TO_ASCLK: c_uint = 0x38;
pub const R_39_CLK_RATIO_ASCLK_TO_ALRCLK: c_uint = 0x39;
pub const R_3A_AUD_CLK_GEN_BASIC_SETUP: c_uint = 0x3a;
// General purpose VBI data slicer part
pub const R_40_SLICER_CNTL_1: c_uint = 0x40;
pub const R_41_LCR_BASE: c_uint = 0x41;
pub const R_58_PROGRAM_FRAMING_CODE: c_uint = 0x58;
pub const R_59_H_OFF_FOR_SLICER: c_uint = 0x59;
pub const R_5A_V_OFF_FOR_SLICER: c_uint = 0x5a;
pub const R_5B_FLD_OFF_AND_MSB_FOR_H_AND_V_OFF: c_uint = 0x5b;
pub const R_5D_DID: c_uint = 0x5d;
pub const R_5E_SDID: c_uint = 0x5e;
pub const R_60_SLICER_STATUS_BYTE_0: c_uint = 0x60;
pub const R_61_SLICER_STATUS_BYTE_1: c_uint = 0x61;
pub const R_62_SLICER_STATUS_BYTE_2: c_uint = 0x62;
// X port, I port and the scaler part
// Task independent global settings
pub const R_80_GLOBAL_CNTL_1: c_uint = 0x80;
pub const R_81_V_SYNC_FLD_ID_SRC_SEL_AND_RETIMED_V_F: c_uint = 0x81;
pub const R_83_X_PORT_I_O_ENA_AND_OUT_CLK: c_uint = 0x83;
pub const R_84_I_PORT_SIGNAL_DEF: c_uint = 0x84;
pub const R_85_I_PORT_SIGNAL_POLAR: c_uint = 0x85;
pub const R_86_I_PORT_FIFO_FLAG_CNTL_AND_ARBIT: c_uint = 0x86;
pub const R_87_I_PORT_I_O_ENA_OUT_CLK_AND_GATED: c_uint = 0x87;
pub const R_88_POWER_SAVE_ADC_PORT_CNTL: c_uint = 0x88;
pub const R_8F_STATUS_INFO_SCALER: c_uint = 0x8f;
// Task A definition
// Basic settings and acquisition window definition
pub const R_90_A_TASK_HANDLING_CNTL: c_uint = 0x90;
pub const R_91_A_X_PORT_FORMATS_AND_CONF: c_uint = 0x91;
pub const R_92_A_X_PORT_INPUT_REFERENCE_SIGNAL: c_uint = 0x92;
pub const R_93_A_I_PORT_OUTPUT_FORMATS_AND_CONF: c_uint = 0x93;
pub const R_94_A_HORIZ_INPUT_WINDOW_START: c_uint = 0x94;
pub const R_95_A_HORIZ_INPUT_WINDOW_START_MSB: c_uint = 0x95;
pub const R_96_A_HORIZ_INPUT_WINDOW_LENGTH: c_uint = 0x96;
pub const R_97_A_HORIZ_INPUT_WINDOW_LENGTH_MSB: c_uint = 0x97;
pub const R_98_A_VERT_INPUT_WINDOW_START: c_uint = 0x98;
pub const R_99_A_VERT_INPUT_WINDOW_START_MSB: c_uint = 0x99;
pub const R_9A_A_VERT_INPUT_WINDOW_LENGTH: c_uint = 0x9a;
pub const R_9B_A_VERT_INPUT_WINDOW_LENGTH_MSB: c_uint = 0x9b;
pub const R_9C_A_HORIZ_OUTPUT_WINDOW_LENGTH: c_uint = 0x9c;
pub const R_9D_A_HORIZ_OUTPUT_WINDOW_LENGTH_MSB: c_uint = 0x9d;
pub const R_9E_A_VERT_OUTPUT_WINDOW_LENGTH: c_uint = 0x9e;
pub const R_9F_A_VERT_OUTPUT_WINDOW_LENGTH_MSB: c_uint = 0x9f;
// FIR filtering and prescaling
pub const R_A0_A_HORIZ_PRESCALING: c_uint = 0xa0;
pub const R_A1_A_ACCUMULATION_LENGTH: c_uint = 0xa1;
pub const R_A2_A_PRESCALER_DC_GAIN_AND_FIR_PREFILTER: c_uint = 0xa2;
pub const R_A4_A_LUMA_BRIGHTNESS_CNTL: c_uint = 0xa4;
pub const R_A5_A_LUMA_CONTRAST_CNTL: c_uint = 0xa5;
pub const R_A6_A_CHROMA_SATURATION_CNTL: c_uint = 0xa6;
// Horizontal phase scaling
pub const R_A8_A_HORIZ_LUMA_SCALING_INC: c_uint = 0xa8;
pub const R_A9_A_HORIZ_LUMA_SCALING_INC_MSB: c_uint = 0xa9;
pub const R_AA_A_HORIZ_LUMA_PHASE_OFF: c_uint = 0xaa;
pub const R_AC_A_HORIZ_CHROMA_SCALING_INC: c_uint = 0xac;
pub const R_AD_A_HORIZ_CHROMA_SCALING_INC_MSB: c_uint = 0xad;
pub const R_AE_A_HORIZ_CHROMA_PHASE_OFF: c_uint = 0xae;
pub const R_AF_A_HORIZ_CHROMA_PHASE_OFF_MSB: c_uint = 0xaf;
// Vertical scaling
pub const R_B0_A_VERT_LUMA_SCALING_INC: c_uint = 0xb0;
pub const R_B1_A_VERT_LUMA_SCALING_INC_MSB: c_uint = 0xb1;
pub const R_B2_A_VERT_CHROMA_SCALING_INC: c_uint = 0xb2;
pub const R_B3_A_VERT_CHROMA_SCALING_INC_MSB: c_uint = 0xb3;
pub const R_B4_A_VERT_SCALING_MODE_CNTL: c_uint = 0xb4;
pub const R_B8_A_VERT_CHROMA_PHASE_OFF_00: c_uint = 0xb8;
pub const R_B9_A_VERT_CHROMA_PHASE_OFF_01: c_uint = 0xb9;
pub const R_BA_A_VERT_CHROMA_PHASE_OFF_10: c_uint = 0xba;
pub const R_BB_A_VERT_CHROMA_PHASE_OFF_11: c_uint = 0xbb;
pub const R_BC_A_VERT_LUMA_PHASE_OFF_00: c_uint = 0xbc;
pub const R_BD_A_VERT_LUMA_PHASE_OFF_01: c_uint = 0xbd;
pub const R_BE_A_VERT_LUMA_PHASE_OFF_10: c_uint = 0xbe;
pub const R_BF_A_VERT_LUMA_PHASE_OFF_11: c_uint = 0xbf;
// Task B definition
// Basic settings and acquisition window definition
pub const R_C0_B_TASK_HANDLING_CNTL: c_uint = 0xc0;
pub const R_C1_B_X_PORT_FORMATS_AND_CONF: c_uint = 0xc1;
pub const R_C2_B_INPUT_REFERENCE_SIGNAL_DEFINITION: c_uint = 0xc2;
pub const R_C3_B_I_PORT_FORMATS_AND_CONF: c_uint = 0xc3;
pub const R_C4_B_HORIZ_INPUT_WINDOW_START: c_uint = 0xc4;
pub const R_C5_B_HORIZ_INPUT_WINDOW_START_MSB: c_uint = 0xc5;
pub const R_C6_B_HORIZ_INPUT_WINDOW_LENGTH: c_uint = 0xc6;
pub const R_C7_B_HORIZ_INPUT_WINDOW_LENGTH_MSB: c_uint = 0xc7;
pub const R_C8_B_VERT_INPUT_WINDOW_START: c_uint = 0xc8;
pub const R_C9_B_VERT_INPUT_WINDOW_START_MSB: c_uint = 0xc9;
pub const R_CA_B_VERT_INPUT_WINDOW_LENGTH: c_uint = 0xca;
pub const R_CB_B_VERT_INPUT_WINDOW_LENGTH_MSB: c_uint = 0xcb;
pub const R_CC_B_HORIZ_OUTPUT_WINDOW_LENGTH: c_uint = 0xcc;
pub const R_CD_B_HORIZ_OUTPUT_WINDOW_LENGTH_MSB: c_uint = 0xcd;
pub const R_CE_B_VERT_OUTPUT_WINDOW_LENGTH: c_uint = 0xce;
pub const R_CF_B_VERT_OUTPUT_WINDOW_LENGTH_MSB: c_uint = 0xcf;
// FIR filtering and prescaling
pub const R_D0_B_HORIZ_PRESCALING: c_uint = 0xd0;
pub const R_D1_B_ACCUMULATION_LENGTH: c_uint = 0xd1;
pub const R_D2_B_PRESCALER_DC_GAIN_AND_FIR_PREFILTER: c_uint = 0xd2;
pub const R_D4_B_LUMA_BRIGHTNESS_CNTL: c_uint = 0xd4;
pub const R_D5_B_LUMA_CONTRAST_CNTL: c_uint = 0xd5;
pub const R_D6_B_CHROMA_SATURATION_CNTL: c_uint = 0xd6;
// Horizontal phase scaling
pub const R_D8_B_HORIZ_LUMA_SCALING_INC: c_uint = 0xd8;
pub const R_D9_B_HORIZ_LUMA_SCALING_INC_MSB: c_uint = 0xd9;
pub const R_DA_B_HORIZ_LUMA_PHASE_OFF: c_uint = 0xda;
pub const R_DC_B_HORIZ_CHROMA_SCALING: c_uint = 0xdc;
pub const R_DD_B_HORIZ_CHROMA_SCALING_MSB: c_uint = 0xdd;
pub const R_DE_B_HORIZ_PHASE_OFFSET_CRHOMA: c_uint = 0xde;
// Vertical scaling
pub const R_E0_B_VERT_LUMA_SCALING_INC: c_uint = 0xe0;
pub const R_E1_B_VERT_LUMA_SCALING_INC_MSB: c_uint = 0xe1;
pub const R_E2_B_VERT_CHROMA_SCALING_INC: c_uint = 0xe2;
pub const R_E3_B_VERT_CHROMA_SCALING_INC_MSB: c_uint = 0xe3;
pub const R_E4_B_VERT_SCALING_MODE_CNTL: c_uint = 0xe4;
pub const R_E8_B_VERT_CHROMA_PHASE_OFF_00: c_uint = 0xe8;
pub const R_E9_B_VERT_CHROMA_PHASE_OFF_01: c_uint = 0xe9;
pub const R_EA_B_VERT_CHROMA_PHASE_OFF_10: c_uint = 0xea;
pub const R_EB_B_VERT_CHROMA_PHASE_OFF_11: c_uint = 0xeb;
pub const R_EC_B_VERT_LUMA_PHASE_OFF_00: c_uint = 0xec;
pub const R_ED_B_VERT_LUMA_PHASE_OFF_01: c_uint = 0xed;
pub const R_EE_B_VERT_LUMA_PHASE_OFF_10: c_uint = 0xee;
pub const R_EF_B_VERT_LUMA_PHASE_OFF_11: c_uint = 0xef;
// second PLL (PLL2) and Pulsegenerator Programming
pub const R_F0_LFCO_PER_LINE: c_uint = 0xf0;
pub const R_F1_P_I_PARAM_SELECT: c_uint = 0xf1;
pub const R_F2_NOMINAL_PLL2_DTO: c_uint = 0xf2;
pub const R_F3_PLL_INCREMENT: c_uint = 0xf3;
pub const R_F4_PLL2_STATUS: c_uint = 0xf4;
pub const R_F5_PULSGEN_LINE_LENGTH: c_uint = 0xf5;
pub const R_F6_PULSE_A_POS_LSB_AND_PULSEGEN_CONFIG: c_uint = 0xf6;
pub const R_F7_PULSE_A_POS_MSB: c_uint = 0xf7;
pub const R_F8_PULSE_B_POS: c_uint = 0xf8;
pub const R_F9_PULSE_B_POS_MSB: c_uint = 0xf9;
pub const R_FA_PULSE_C_POS: c_uint = 0xfa;
pub const R_FB_PULSE_C_POS_MSB: c_uint = 0xfb;
pub const R_FF_S_PLL_MAX_PHASE_ERR_THRESH_NUM_LINES: c_uint = 0xff;
// SAA7113 bit-masks
pub const SAA7113_R_08_HTC_OFFSET: c_int = 3;

pub const SAA7113_R_08_FSEL: c_uint = 0x40;
pub const SAA7113_R_08_AUFD: c_uint = 0x80;
pub const SAA7113_R_10_VRLN_OFFSET: c_int = 3;

pub const SAA7113_R_10_OFTS_OFFSET: c_int = 6;

pub const SAA7113_R_12_RTS0_OFFSET: c_int = 0;

pub const SAA7113_R_12_RTS1_OFFSET: c_int = 4;

pub const SAA7113_R_13_ADLSB_OFFSET: c_int = 7;

// Those structs will be used in the future for debug purposes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa711x_reg_descr {
    pub reg: u8,
    pub count: c_int,
    pub name: *mut c_char,
}

// REG COUNT NAME
// Video Decoder: R_01_INC_DELAY to R_1F_STATUS_BYTE_2_VD_DEC
// Video Decoder - Frontend part: R_01_INC_DELAY to R_05_INPUT_CNTL_4
// Video Decoder - Decoder part: R_06_H_SYNC_START to R_1F_STATUS_BYTE_2_VD_DEC
// Component processing and interrupt masking part:  0x20h to R_2F_INTERRUPT_MASK_3
// 0x20 to 0x22 - Reserved
// 0x26 to 0x28 - Reserved
// Audio clock generator part: R_30_AUD_MAST_CLK_CYCLES_PER_FIELD to 0x3f
// 0x33 - Reserved
// 0x37 - Reserved
// 0x3b-0x3f - Reserved
// General purpose VBI data slicer part: R_40_SLICER_CNTL_1 to 0x7f
// 0x63-0x7f - Reserved
// X port, I port and the scaler part: R_80_GLOBAL_CNTL_1 to R_EF_B_VERT_LUMA_PHASE_OFF_11
// Task independent global settings: R_80_GLOBAL_CNTL_1 to R_8F_STATUS_INFO_SCALER
// 0x82 - Reserved
// 089-0x8e - Reserved
// Task A definition: R_90_A_TASK_HANDLING_CNTL to R_BF_A_VERT_LUMA_PHASE_OFF_11
// Task A: Basic settings and acquisition window definition
// Task A: FIR filtering and prescaling
// 0xa3 - Reserved
// 0xa7 - Reserved
// Task A: Horizontal phase scaling
// 0xab - Reserved
// 0xaf - Reserved
// Task A: Vertical scaling
// 0xb5-0xb7 - Reserved
// Task B definition: R_C0_B_TASK_HANDLING_CNTL to R_EF_B_VERT_LUMA_PHASE_OFF_11
// Task B: Basic settings and acquisition window definition
// Task B: FIR filtering and prescaling
// 0xd3 - Reserved
// 0xd7 - Reserved
// Task B: Horizontal phase scaling
// 0xdb - Reserved
// 0xdf - Reserved
// Task B: Vertical scaling
// 0xe5-0xe7 - Reserved
// second PLL (PLL2) and Pulsegenerator Programming
// 0xfc to 0xfe - Reserved
