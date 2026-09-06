//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/mxl111sf-reg.h
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
// mxl111sf-reg.h - driver for the MaxLinear MXL111SF
//
// Copyright (C) 2010-2014 Michael Krufky <mkrufky@linuxtv.org>
//
pub const CHIP_ID_REG: c_uint = 0xFC;
pub const TOP_CHIP_REV_ID_REG: c_uint = 0xFA;
pub const V6_SNR_RB_LSB_REG: c_uint = 0x27;
pub const V6_SNR_RB_MSB_REG: c_uint = 0x28;
pub const V6_N_ACCUMULATE_REG: c_uint = 0x11;
pub const V6_RS_AVG_ERRORS_LSB_REG: c_uint = 0x2C;
pub const V6_RS_AVG_ERRORS_MSB_REG: c_uint = 0x2D;
pub const V6_IRQ_STATUS_REG: c_uint = 0x24;
pub const IRQ_MASK_FEC_LOCK: c_uint = 0x10;
pub const V6_SYNC_LOCK_REG: c_uint = 0x28;
pub const SYNC_LOCK_MASK: c_uint = 0x10;
pub const V6_RS_LOCK_DET_REG: c_uint = 0x28;
pub const RS_LOCK_DET_MASK: c_uint = 0x08;
pub const V6_INITACQ_NODETECT_REG: c_uint = 0x20;
pub const V6_FORCE_NFFT_CPSIZE_REG: c_uint = 0x20;
pub const V6_CODE_RATE_TPS_REG: c_uint = 0x29;
pub const V6_CODE_RATE_TPS_MASK: c_uint = 0x07;
pub const V6_CP_LOCK_DET_REG: c_uint = 0x28;
pub const V6_CP_LOCK_DET_MASK: c_uint = 0x04;
pub const V6_TPS_HIERACHY_REG: c_uint = 0x29;
pub const V6_TPS_HIERARCHY_INFO_MASK: c_uint = 0x40;
pub const V6_MODORDER_TPS_REG: c_uint = 0x2A;
pub const V6_PARAM_CONSTELLATION_MASK: c_uint = 0x30;
pub const V6_MODE_TPS_REG: c_uint = 0x2A;
pub const V6_PARAM_FFT_MODE_MASK: c_uint = 0x0C;
pub const V6_CP_TPS_REG: c_uint = 0x29;
pub const V6_PARAM_GI_MASK: c_uint = 0x30;
pub const V6_TPS_LOCK_REG: c_uint = 0x2A;
pub const V6_PARAM_TPS_LOCK_MASK: c_uint = 0x40;
pub const V6_FEC_PER_COUNT_REG: c_uint = 0x2E;
pub const V6_FEC_PER_SCALE_REG: c_uint = 0x2B;
pub const V6_FEC_PER_SCALE_MASK: c_uint = 0x03;
pub const V6_FEC_PER_CLR_REG: c_uint = 0x20;
pub const V6_FEC_PER_CLR_MASK: c_uint = 0x01;
pub const V6_PIN_MUX_MODE_REG: c_uint = 0x1B;
pub const V6_ENABLE_PIN_MUX: c_uint = 0x1E;
pub const V6_I2S_NUM_SAMPLES_REG: c_uint = 0x16;
pub const V6_MPEG_IN_CLK_INV_REG: c_uint = 0x17;
pub const V6_MPEG_IN_CTRL_REG: c_uint = 0x18;
pub const V6_INVERTED_CLK_PHASE: c_uint = 0x20;
pub const V6_MPEG_IN_DATA_PARALLEL: c_uint = 0x01;
pub const V6_MPEG_IN_DATA_SERIAL: c_uint = 0x02;
pub const V6_INVERTED_MPEG_SYNC: c_uint = 0x04;
pub const V6_INVERTED_MPEG_VALID: c_uint = 0x08;
pub const TSIF_INPUT_PARALLEL: c_int = 0;
pub const TSIF_INPUT_SERIAL: c_int = 1;
pub const TSIF_NORMAL: c_int = 0;
pub const V6_MPEG_INOUT_BIT_ORDER_CTRL_REG: c_uint = 0x19;
pub const V6_MPEG_SER_MSB_FIRST: c_uint = 0x80;
pub const MPEG_SER_MSB_FIRST_ENABLED: c_uint = 0x01;
pub const V6_656_I2S_BUFF_STATUS_REG: c_uint = 0x2F;
pub const V6_656_OVERFLOW_MASK_BIT: c_uint = 0x08;
pub const V6_I2S_OVERFLOW_MASK_BIT: c_uint = 0x01;
pub const V6_I2S_STREAM_START_BIT_REG: c_uint = 0x14;
pub const V6_I2S_STREAM_END_BIT_REG: c_uint = 0x15;
pub const I2S_RIGHT_JUSTIFIED: c_int = 0;
pub const I2S_LEFT_JUSTIFIED: c_int = 1;
pub const I2S_DATA_FORMAT: c_int = 2;
pub const V6_TUNER_LOOP_THRU_CONTROL_REG: c_uint = 0x09;
pub const V6_ENABLE_LOOP_THRU: c_uint = 0x01;
pub const TOTAL_NUM_IF_OUTPUT_FREQ: c_int = 16;
pub const TUNER_NORMAL_IF_SPECTRUM: c_uint = 0x0;
pub const TUNER_INVERT_IF_SPECTRUM: c_uint = 0x10;
pub const V6_TUNER_IF_SEL_REG: c_uint = 0x06;
pub const V6_TUNER_IF_FCW_REG: c_uint = 0x3C;
pub const V6_TUNER_IF_FCW_BYP_REG: c_uint = 0x3D;
pub const V6_RF_LOCK_STATUS_REG: c_uint = 0x23;
pub const NUM_DIG_TV_CHANNEL: c_int = 1000;
pub const V6_DIG_CLK_FREQ_SEL_REG: c_uint = 0x07;
pub const V6_REF_SYNTH_INT_REG: c_uint = 0x5C;
pub const V6_REF_SYNTH_REMAIN_REG: c_uint = 0x58;
pub const V6_DIG_RFREFSELECT_REG: c_uint = 0x32;
pub const V6_XTAL_CLK_OUT_GAIN_REG: c_uint = 0x31;
pub const V6_TUNER_LOOP_THRU_CTRL_REG: c_uint = 0x09;
pub const V6_DIG_XTAL_ENABLE_REG: c_uint = 0x06;
pub const V6_DIG_XTAL_BIAS_REG: c_uint = 0x66;
pub const V6_XTAL_CAP_REG: c_uint = 0x08;
pub const V6_GPO_CTRL_REG: c_uint = 0x18;
pub const MXL_GPO_0: c_uint = 0x00;
pub const MXL_GPO_1: c_uint = 0x01;
pub const V6_GPO_0_MASK: c_uint = 0x10;
pub const V6_GPO_1_MASK: c_uint = 0x20;
pub const V6_111SF_GPO_CTRL_REG: c_uint = 0x19;
pub const MXL_111SF_GPO_1: c_uint = 0x00;
pub const MXL_111SF_GPO_2: c_uint = 0x01;
pub const MXL_111SF_GPO_3: c_uint = 0x02;
pub const MXL_111SF_GPO_4: c_uint = 0x03;
pub const MXL_111SF_GPO_5: c_uint = 0x04;
pub const MXL_111SF_GPO_6: c_uint = 0x05;
pub const MXL_111SF_GPO_7: c_uint = 0x06;
pub const MXL_111SF_GPO_0_MASK: c_uint = 0x01;
pub const MXL_111SF_GPO_1_MASK: c_uint = 0x02;
pub const MXL_111SF_GPO_2_MASK: c_uint = 0x04;
pub const MXL_111SF_GPO_3_MASK: c_uint = 0x08;
pub const MXL_111SF_GPO_4_MASK: c_uint = 0x10;
pub const MXL_111SF_GPO_5_MASK: c_uint = 0x20;
pub const MXL_111SF_GPO_6_MASK: c_uint = 0x40;
pub const V6_ATSC_CONFIG_REG: c_uint = 0x0A;
pub const MXL_MODE_REG: c_uint = 0x03;
pub const START_TUNE_REG: c_uint = 0x1C;
pub const V6_IDAC_HYSTERESIS_REG: c_uint = 0x0B;
pub const V6_IDAC_SETTINGS_REG: c_uint = 0x0C;
pub const IDAC_MANUAL_CONTROL: c_int = 1;
pub const IDAC_CURRENT_SINKING_ENABLE: c_int = 1;
pub const IDAC_MANUAL_CONTROL_BIT_MASK: c_uint = 0x80;
pub const IDAC_CURRENT_SINKING_BIT_MASK: c_uint = 0x40;
pub const V8_SPI_MODE_REG: c_uint = 0xE9;
pub const V6_DIG_RF_PWR_LSB_REG: c_uint = 0x46;
pub const V6_DIG_RF_PWR_MSB_REG: c_uint = 0x47;
