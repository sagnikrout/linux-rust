//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas675x.h
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
// ALSA SoC Texas Instruments TAS675x Quad-Channel Audio Amplifier
//
// Copyright (C) 2026 Texas Instruments Incorporated - https://www.ti.com
// Author: Sen Wang <sen@ti.com>
//
// Book 0, Page 0 — Register Addresses
//
pub const TAS675X_PAGE_SIZE: c_int = 256;

// Page Control & Basic Config
pub const TAS675X_PAGE_CTRL_REG: c_uint = 0x00;
pub const TAS675X_RESET_REG: c_uint = 0x01;
pub const TAS675X_OUTPUT_CTRL_REG: c_uint = 0x02;
pub const TAS675X_STATE_CTRL_CH1_CH2_REG: c_uint = 0x03;
pub const TAS675X_STATE_CTRL_CH3_CH4_REG: c_uint = 0x04;
pub const TAS675X_ISENSE_CTRL_REG: c_uint = 0x05;
pub const TAS675X_DC_DETECT_CTRL_REG: c_uint = 0x06;
// Serial Audio Port
pub const TAS675X_SCLK_INV_CTRL_REG: c_uint = 0x20;
pub const TAS675X_AUDIO_IF_CTRL_REG: c_uint = 0x21;
pub const TAS675X_SDIN_CTRL_REG: c_uint = 0x23;
pub const TAS675X_SDOUT_CTRL_REG: c_uint = 0x25;
pub const TAS675X_SDIN_OFFSET_MSB_REG: c_uint = 0x27;
pub const TAS675X_SDIN_AUDIO_OFFSET_REG: c_uint = 0x28;
pub const TAS675X_SDIN_LL_OFFSET_REG: c_uint = 0x29;
pub const TAS675X_SDIN_CH_SWAP_REG: c_uint = 0x2A;
pub const TAS675X_SDOUT_OFFSET_MSB_REG: c_uint = 0x2C;
pub const TAS675X_VPREDICT_OFFSET_REG: c_uint = 0x2D;
pub const TAS675X_ISENSE_OFFSET_REG: c_uint = 0x2E;
pub const TAS675X_SDOUT_EN_REG: c_uint = 0x31;
pub const TAS675X_LL_EN_REG: c_uint = 0x32;
// DSP & Core Audio Control
pub const TAS675X_RTLDG_EN_REG: c_uint = 0x37;
pub const TAS675X_DC_BLOCK_BYP_REG: c_uint = 0x39;
pub const TAS675X_DSP_CTRL_REG: c_uint = 0x3A;
pub const TAS675X_PAGE_AUTO_INC_REG: c_uint = 0x3B;
// Volume & Mute
pub const TAS675X_DIG_VOL_CH1_REG: c_uint = 0x40;
pub const TAS675X_DIG_VOL_CH2_REG: c_uint = 0x41;
pub const TAS675X_DIG_VOL_CH3_REG: c_uint = 0x42;
pub const TAS675X_DIG_VOL_CH4_REG: c_uint = 0x43;
pub const TAS675X_DIG_VOL_RAMP_CTRL_REG: c_uint = 0x44;
pub const TAS675X_DIG_VOL_COMBINE_CTRL_REG: c_uint = 0x46;
pub const TAS675X_AUTO_MUTE_EN_REG: c_uint = 0x47;
pub const TAS675X_AUTO_MUTE_TIMING_CH1_CH2_REG: c_uint = 0x48;
pub const TAS675X_AUTO_MUTE_TIMING_CH3_CH4_REG: c_uint = 0x49;
// Analog Gain & Power Stage
pub const TAS675X_ANALOG_GAIN_CH1_CH2_REG: c_uint = 0x4A;
pub const TAS675X_ANALOG_GAIN_CH3_CH4_REG: c_uint = 0x4B;
pub const TAS675X_ANALOG_GAIN_RAMP_CTRL_REG: c_uint = 0x4E;
pub const TAS675X_PULSE_INJECTION_EN_REG: c_uint = 0x52;
pub const TAS675X_CBC_CTRL_REG: c_uint = 0x54;
pub const TAS675X_CURRENT_LIMIT_CTRL_REG: c_uint = 0x55;
pub const TAS675X_DAC_CLK_REG: c_uint = 0x5A;
pub const TAS675X_ISENSE_CAL_REG: c_uint = 0x5B;
// Spread Spectrum & PWM Phase
pub const TAS675X_PWM_PHASE_CTRL_REG: c_uint = 0x60;
pub const TAS675X_SS_CTRL_REG: c_uint = 0x61;
pub const TAS675X_SS_RANGE_CTRL_REG: c_uint = 0x62;
pub const TAS675X_SS_DWELL_CTRL_REG: c_uint = 0x66;
pub const TAS675X_RAMP_PHASE_CTRL_GPO_REG: c_uint = 0x68;
pub const TAS675X_PWM_PHASE_M_CTRL_CH1_REG: c_uint = 0x69;
pub const TAS675X_PWM_PHASE_M_CTRL_CH2_REG: c_uint = 0x6A;
pub const TAS675X_PWM_PHASE_M_CTRL_CH3_REG: c_uint = 0x6B;
pub const TAS675X_PWM_PHASE_M_CTRL_CH4_REG: c_uint = 0x6C;
// Status & Reporting
pub const TAS675X_AUTO_MUTE_STATUS_REG: c_uint = 0x71;
pub const TAS675X_STATE_REPORT_CH1_CH2_REG: c_uint = 0x72;
pub const TAS675X_STATE_REPORT_CH3_CH4_REG: c_uint = 0x73;
pub const TAS675X_PVDD_SENSE_REG: c_uint = 0x74;
pub const TAS675X_TEMP_GLOBAL_REG: c_uint = 0x75;
pub const TAS675X_FS_MON_REG: c_uint = 0x76;
pub const TAS675X_SCLK_MON_REG: c_uint = 0x77;
pub const TAS675X_REPORT_ROUTING_1_REG: c_uint = 0x7C;
// Memory Paging & Book Control
pub const TAS675X_SETUP_REG1: c_uint = 0x7D;
pub const TAS675X_SETUP_REG2: c_uint = 0x7E;
pub const TAS675X_BOOK_CTRL_REG: c_uint = 0x7F;
// Fault Status
pub const TAS675X_POWER_FAULT_STATUS_1_REG: c_uint = 0x7D;
pub const TAS675X_POWER_FAULT_STATUS_2_REG: c_uint = 0x80;
pub const TAS675X_OT_FAULT_REG: c_uint = 0x81;
pub const TAS675X_OTW_STATUS_REG: c_uint = 0x82;
pub const TAS675X_CLIP_WARN_STATUS_REG: c_uint = 0x83;
pub const TAS675X_CBC_WARNING_STATUS_REG: c_uint = 0x85;
// Latched Fault Registers
pub const TAS675X_POWER_FAULT_LATCHED_REG: c_uint = 0x86;
pub const TAS675X_OTSD_LATCHED_REG: c_uint = 0x87;
pub const TAS675X_OTW_LATCHED_REG: c_uint = 0x88;
pub const TAS675X_CLIP_WARN_LATCHED_REG: c_uint = 0x89;
pub const TAS675X_CLK_FAULT_LATCHED_REG: c_uint = 0x8A;
pub const TAS675X_RTLDG_OL_SL_FAULT_LATCHED_REG: c_uint = 0x8B;
pub const TAS675X_CBC_FAULT_WARN_LATCHED_REG: c_uint = 0x8D;
pub const TAS675X_OC_DC_FAULT_LATCHED_REG: c_uint = 0x8E;
pub const TAS675X_OTSD_RECOVERY_EN_REG: c_uint = 0x8F;
// Protection & Routing Controls
pub const TAS675X_REPORT_ROUTING_2_REG: c_uint = 0x90;
pub const TAS675X_REPORT_ROUTING_3_REG: c_uint = 0x91;
pub const TAS675X_REPORT_ROUTING_4_REG: c_uint = 0x92;
pub const TAS675X_CLIP_DETECT_CTRL_REG: c_uint = 0x93;
pub const TAS675X_REPORT_ROUTING_5_REG: c_uint = 0x94;
// GPIO Pin Configuration
pub const TAS675X_GPIO1_OUTPUT_SEL_REG: c_uint = 0x95;
pub const TAS675X_GPIO2_OUTPUT_SEL_REG: c_uint = 0x96;
pub const TAS675X_GPIO_INPUT_SLEEP_HIZ_REG: c_uint = 0x9B;
pub const TAS675X_GPIO_INPUT_PLAY_SLEEP_REG: c_uint = 0x9C;
pub const TAS675X_GPIO_INPUT_MUTE_REG: c_uint = 0x9D;
pub const TAS675X_GPIO_INPUT_SYNC_REG: c_uint = 0x9E;
pub const TAS675X_GPIO_INPUT_SDIN2_REG: c_uint = 0x9F;
pub const TAS675X_GPIO_CTRL_REG: c_uint = 0xA0;
pub const TAS675X_GPIO_INVERT_REG: c_uint = 0xA1;
// Load Diagnostics Config
pub const TAS675X_DC_LDG_CTRL_REG: c_uint = 0xB0;
pub const TAS675X_DC_LDG_LO_CTRL_REG: c_uint = 0xB1;
pub const TAS675X_DC_LDG_TIME_CTRL_REG: c_uint = 0xB2;
pub const TAS675X_DC_LDG_SL_CH1_CH2_CTRL_REG: c_uint = 0xB3;
pub const TAS675X_DC_LDG_SL_CH3_CH4_CTRL_REG: c_uint = 0xB4;
pub const TAS675X_AC_LDG_CTRL_REG: c_uint = 0xB5;
pub const TAS675X_TWEETER_DETECT_CTRL_REG: c_uint = 0xB6;
pub const TAS675X_TWEETER_DETECT_THRESH_REG: c_uint = 0xB7;
pub const TAS675X_AC_LDG_FREQ_CTRL_REG: c_uint = 0xB8;
pub const TAS675X_TEMP_CH1_CH2_REG: c_uint = 0xBB;
pub const TAS675X_TEMP_CH3_CH4_REG: c_uint = 0xBC;
pub const TAS675X_WARN_OT_MAX_FLAG_REG: c_uint = 0xBD;
// DC Load Diagnostic Reports
pub const TAS675X_DC_LDG_REPORT_CH1_CH2_REG: c_uint = 0xC0;
pub const TAS675X_DC_LDG_REPORT_CH3_CH4_REG: c_uint = 0xC1;
pub const TAS675X_DC_LDG_RESULT_REG: c_uint = 0xC2;
pub const TAS675X_AC_LDG_REPORT_CH1_R_REG: c_uint = 0xC3;
pub const TAS675X_AC_LDG_REPORT_CH1_I_REG: c_uint = 0xC4;
pub const TAS675X_AC_LDG_REPORT_CH2_R_REG: c_uint = 0xC5;
pub const TAS675X_AC_LDG_REPORT_CH2_I_REG: c_uint = 0xC6;
pub const TAS675X_AC_LDG_REPORT_CH3_R_REG: c_uint = 0xC7;
pub const TAS675X_AC_LDG_REPORT_CH3_I_REG: c_uint = 0xC8;
pub const TAS675X_AC_LDG_REPORT_CH4_R_REG: c_uint = 0xC9;
pub const TAS675X_AC_LDG_REPORT_CH4_I_REG: c_uint = 0xCA;
pub const TAS675X_TWEETER_REPORT_REG: c_uint = 0xCB;
// RTLDG Impedance
pub const TAS675X_CH1_RTLDG_IMP_MSB_REG: c_uint = 0xD1;
pub const TAS675X_CH1_RTLDG_IMP_LSB_REG: c_uint = 0xD2;
pub const TAS675X_CH2_RTLDG_IMP_MSB_REG: c_uint = 0xD3;
pub const TAS675X_CH2_RTLDG_IMP_LSB_REG: c_uint = 0xD4;
pub const TAS675X_CH3_RTLDG_IMP_MSB_REG: c_uint = 0xD5;
pub const TAS675X_CH3_RTLDG_IMP_LSB_REG: c_uint = 0xD6;
pub const TAS675X_CH4_RTLDG_IMP_MSB_REG: c_uint = 0xD7;
pub const TAS675X_CH4_RTLDG_IMP_LSB_REG: c_uint = 0xD8;
// DC Load Diagnostic Resistance
pub const TAS675X_DC_LDG_DCR_MSB_REG: c_uint = 0xD9;
pub const TAS675X_CH1_DC_LDG_DCR_LSB_REG: c_uint = 0xDA;
pub const TAS675X_CH2_DC_LDG_DCR_LSB_REG: c_uint = 0xDB;
pub const TAS675X_CH3_DC_LDG_DCR_LSB_REG: c_uint = 0xDC;
pub const TAS675X_CH4_DC_LDG_DCR_LSB_REG: c_uint = 0xDD;
// Over-Temperature Warning
pub const TAS675X_OTW_CTRL_CH1_CH2_REG: c_uint = 0xE2;
pub const TAS675X_OTW_CTRL_CH3_CH4_REG: c_uint = 0xE3;
// RESET_REG (all bits auto-clear)

// STATE_CTRL and STATE_REPORT — Channel state values
pub const TAS675X_STATE_DEEPSLEEP: c_uint = 0x00;
pub const TAS675X_STATE_LOAD_DIAG: c_uint = 0x01;
pub const TAS675X_STATE_SLEEP: c_uint = 0x02;
pub const TAS675X_STATE_HIZ: c_uint = 0x03;
pub const TAS675X_STATE_PLAY: c_uint = 0x04;
// Additional STATE_REPORT values
pub const TAS675X_STATE_FAULT: c_uint = 0x05;
pub const TAS675X_STATE_AUTOREC: c_uint = 0x06;
// Combined values for both channel pairs in one register

// STATE_CTRL_CH1_CH2 / STATE_CTRL_CH3_CH4 — mute bits

// SCLK_INV_CTRL_REG

// AUDIO_IF_CTRL_REG

pub const TAS675X_FS_PULSE_SHORT: c_uint = 0x01;
// SDIN_CTRL_REG

// SDOUT_CTRL_REG

pub const TAS675X_SDOUT_SELECT_TDM_SDOUT1: c_uint = 0x00;
pub const TAS675X_SDOUT_SELECT_NON_TDM: c_uint = 0x10;

// SDOUT_EN_REG

// Word length values (shared by SDIN_CTRL and SDOUT_CTRL)
pub const TAS675X_WL_16BIT: c_uint = 0x00;
pub const TAS675X_WL_20BIT: c_uint = 0x01;
pub const TAS675X_WL_24BIT: c_uint = 0x02;
pub const TAS675X_WL_32BIT: c_uint = 0x03;
// SDIN_OFFSET_MSB_REG

// SDOUT_OFFSET_MSB_REG

// RTLDG_EN_REG

// DC_LDG_CTRL_REG

// DC_LDG_TIME_CTRL_REG

// AC_LDG_CTRL_REG

// DC_LDG_RESULT_REG

// Load Diagnostics Timing Constants
pub const TAS675X_POLL_INTERVAL_US: c_int = 10000;
pub const TAS675X_STATE_TRANSITION_TIMEOUT_US: c_int = 50000;
pub const TAS675X_DC_LDG_TIMEOUT_US: c_int = 300000;
pub const TAS675X_AC_LDG_TIMEOUT_US: c_int = 400000;
// GPIO_CTRL_REG

pub const TAS675X_GPIO_CTRL_RSTVAL: c_uint = 0x22;
// GPIO output select values
pub const TAS675X_GPIO_SEL_LOW: c_uint = 0x00;
pub const TAS675X_GPIO_SEL_AUTO_MUTE_ALL: c_uint = 0x02;
pub const TAS675X_GPIO_SEL_AUTO_MUTE_CH4: c_uint = 0x03;
pub const TAS675X_GPIO_SEL_AUTO_MUTE_CH3: c_uint = 0x04;
pub const TAS675X_GPIO_SEL_AUTO_MUTE_CH2: c_uint = 0x05;
pub const TAS675X_GPIO_SEL_AUTO_MUTE_CH1: c_uint = 0x06;
pub const TAS675X_GPIO_SEL_SDOUT2: c_uint = 0x08;
pub const TAS675X_GPIO_SEL_SDOUT1: c_uint = 0x09;
pub const TAS675X_GPIO_SEL_WARN: c_uint = 0x0A;
pub const TAS675X_GPIO_SEL_FAULT: c_uint = 0x0B;
pub const TAS675X_GPIO_SEL_CLOCK_SYNC: c_uint = 0x0E;
pub const TAS675X_GPIO_SEL_INVALID_CLK: c_uint = 0x0F;
pub const TAS675X_GPIO_SEL_HIGH: c_uint = 0x13;
// GPIO input function encoding (flag bit | function ID)
pub const TAS675X_GPIO_FUNC_INPUT: c_uint = 0x100;
// Input Function IDs
pub const TAS675X_GPIO_IN_ID_MUTE: c_int = 0;
pub const TAS675X_GPIO_IN_ID_PHASE_SYNC: c_int = 1;
pub const TAS675X_GPIO_IN_ID_SDIN2: c_int = 2;
pub const TAS675X_GPIO_IN_ID_DEEP_SLEEP: c_int = 3;
pub const TAS675X_GPIO_IN_ID_HIZ: c_int = 4;
pub const TAS675X_GPIO_IN_ID_PLAY: c_int = 5;
pub const TAS675X_GPIO_IN_ID_SLEEP: c_int = 6;
pub const TAS675X_GPIO_IN_NUM: c_int = 7;

// GPIO input 3-bit mux field masks

// Book addresses for tas675x_select_book()
pub const TAS675X_BOOK_DEFAULT: c_uint = 0x00;
pub const TAS675X_BOOK_DSP: c_uint = 0x8C;
// DSP memory addresses (DSP Book)
pub const TAS675X_DSP_PAGE_RTLDG: c_uint = 0x22;
pub const TAS675X_DSP_RTLDG_OL_THRESH_REG: c_uint = 0x98;
pub const TAS675X_DSP_RTLDG_SL_THRESH_REG: c_uint = 0x9C;
pub const TAS675X_DSP_PARAM_ID_OL_THRESH: c_int = 0;
pub const TAS675X_DSP_PARAM_ID_SL_THRESH: c_int = 1;
// Setup Mode Entry/Exit
pub const TAS675X_SETUP_ENTER_VAL1: c_uint = 0x11;
pub const TAS675X_SETUP_ENTER_VAL2: c_uint = 0xFF;
pub const TAS675X_SETUP_EXIT_VAL: c_uint = 0x00;
