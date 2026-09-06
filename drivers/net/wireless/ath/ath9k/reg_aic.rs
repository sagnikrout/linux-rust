//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/reg_aic.h
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


//
// Copyright (c) 2015 Qualcomm Atheros Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// AIC fields
pub const AR_PHY_AIC_MON_ENABLE: c_uint = 0x80000000;
pub const AR_PHY_AIC_MON_ENABLE_S: c_int = 31;
pub const AR_PHY_AIC_CAL_MAX_HOP_COUNT: c_uint = 0x7F000000;
pub const AR_PHY_AIC_CAL_MAX_HOP_COUNT_S: c_int = 24;
pub const AR_PHY_AIC_CAL_MIN_VALID_COUNT: c_uint = 0x00FE0000;
pub const AR_PHY_AIC_CAL_MIN_VALID_COUNT_S: c_int = 17;
pub const AR_PHY_AIC_F_WLAN: c_uint = 0x0001FC00;
pub const AR_PHY_AIC_F_WLAN_S: c_int = 10;
pub const AR_PHY_AIC_CAL_CH_VALID_RESET: c_uint = 0x00000200;
pub const AR_PHY_AIC_CAL_CH_VALID_RESET_S: c_int = 9;
pub const AR_PHY_AIC_CAL_ENABLE: c_uint = 0x00000100;
pub const AR_PHY_AIC_CAL_ENABLE_S: c_int = 8;
pub const AR_PHY_AIC_BTTX_PWR_THR: c_uint = 0x000000FE;
pub const AR_PHY_AIC_BTTX_PWR_THR_S: c_int = 1;
pub const AR_PHY_AIC_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_AIC_ENABLE_S: c_int = 0;
pub const AR_PHY_AIC_CAL_BT_REF_DELAY: c_uint = 0x00F00000;
pub const AR_PHY_AIC_CAL_BT_REF_DELAY_S: c_int = 20;
pub const AR_PHY_AIC_BT_IDLE_CFG: c_uint = 0x00080000;
pub const AR_PHY_AIC_BT_IDLE_CFG_S: c_int = 19;
pub const AR_PHY_AIC_STDBY_COND: c_uint = 0x00060000;
pub const AR_PHY_AIC_STDBY_COND_S: c_int = 17;
pub const AR_PHY_AIC_STDBY_ROT_ATT_DB: c_uint = 0x0001F800;
pub const AR_PHY_AIC_STDBY_ROT_ATT_DB_S: c_int = 11;
pub const AR_PHY_AIC_STDBY_COM_ATT_DB: c_uint = 0x00000700;
pub const AR_PHY_AIC_STDBY_COM_ATT_DB_S: c_int = 8;
pub const AR_PHY_AIC_RSSI_MAX: c_uint = 0x000000F0;
pub const AR_PHY_AIC_RSSI_MAX_S: c_int = 4;
pub const AR_PHY_AIC_RSSI_MIN: c_uint = 0x0000000F;
pub const AR_PHY_AIC_RSSI_MIN_S: c_int = 0;
pub const AR_PHY_AIC_RADIO_DELAY: c_uint = 0x7F000000;
pub const AR_PHY_AIC_RADIO_DELAY_S: c_int = 24;
pub const AR_PHY_AIC_CAL_STEP_SIZE_CORR: c_uint = 0x00F00000;
pub const AR_PHY_AIC_CAL_STEP_SIZE_CORR_S: c_int = 20;
pub const AR_PHY_AIC_CAL_ROT_IDX_CORR: c_uint = 0x000F8000;
pub const AR_PHY_AIC_CAL_ROT_IDX_CORR_S: c_int = 15;
pub const AR_PHY_AIC_CAL_CONV_CHECK_FACTOR: c_uint = 0x00006000;
pub const AR_PHY_AIC_CAL_CONV_CHECK_FACTOR_S: c_int = 13;
pub const AR_PHY_AIC_ROT_IDX_COUNT_MAX: c_uint = 0x00001C00;
pub const AR_PHY_AIC_ROT_IDX_COUNT_MAX_S: c_int = 10;
pub const AR_PHY_AIC_CAL_SYNTH_TOGGLE: c_uint = 0x00000200;
pub const AR_PHY_AIC_CAL_SYNTH_TOGGLE_S: c_int = 9;
pub const AR_PHY_AIC_CAL_SYNTH_AFTER_BTRX: c_uint = 0x00000100;
pub const AR_PHY_AIC_CAL_SYNTH_AFTER_BTRX_S: c_int = 8;
pub const AR_PHY_AIC_CAL_SYNTH_SETTLING: c_uint = 0x000000FF;
pub const AR_PHY_AIC_CAL_SYNTH_SETTLING_S: c_int = 0;
pub const AR_PHY_AIC_MON_MAX_HOP_COUNT: c_uint = 0x07F00000;
pub const AR_PHY_AIC_MON_MAX_HOP_COUNT_S: c_int = 20;
pub const AR_PHY_AIC_MON_MIN_STALE_COUNT: c_uint = 0x000FE000;
pub const AR_PHY_AIC_MON_MIN_STALE_COUNT_S: c_int = 13;
pub const AR_PHY_AIC_MON_PWR_EST_LONG: c_uint = 0x00001000;
pub const AR_PHY_AIC_MON_PWR_EST_LONG_S: c_int = 12;
pub const AR_PHY_AIC_MON_PD_TALLY_SCALING: c_uint = 0x00000C00;
pub const AR_PHY_AIC_MON_PD_TALLY_SCALING_S: c_int = 10;
pub const AR_PHY_AIC_MON_PERF_THR: c_uint = 0x000003E0;
pub const AR_PHY_AIC_MON_PERF_THR_S: c_int = 5;
pub const AR_PHY_AIC_CAL_TARGET_MAG_SETTING: c_uint = 0x00000018;
pub const AR_PHY_AIC_CAL_TARGET_MAG_SETTING_S: c_int = 3;
pub const AR_PHY_AIC_CAL_PERF_CHECK_FACTOR: c_uint = 0x00000006;
pub const AR_PHY_AIC_CAL_PERF_CHECK_FACTOR_S: c_int = 1;
pub const AR_PHY_AIC_CAL_PWR_EST_LONG: c_uint = 0x00000001;
pub const AR_PHY_AIC_CAL_PWR_EST_LONG_S: c_int = 0;
pub const AR_PHY_AIC_MON_DONE: c_uint = 0x80000000;
pub const AR_PHY_AIC_MON_DONE_S: c_int = 31;
pub const AR_PHY_AIC_MON_ACTIVE: c_uint = 0x40000000;
pub const AR_PHY_AIC_MON_ACTIVE_S: c_int = 30;
pub const AR_PHY_AIC_MEAS_COUNT: c_uint = 0x3F000000;
pub const AR_PHY_AIC_MEAS_COUNT_S: c_int = 24;
pub const AR_PHY_AIC_CAL_ANT_ISO_EST: c_uint = 0x00FC0000;
pub const AR_PHY_AIC_CAL_ANT_ISO_EST_S: c_int = 18;
pub const AR_PHY_AIC_CAL_HOP_COUNT: c_uint = 0x0003F800;
pub const AR_PHY_AIC_CAL_HOP_COUNT_S: c_int = 11;
pub const AR_PHY_AIC_CAL_VALID_COUNT: c_uint = 0x000007F0;
pub const AR_PHY_AIC_CAL_VALID_COUNT_S: c_int = 4;
pub const AR_PHY_AIC_CAL_BT_TOO_WEAK_ERR: c_uint = 0x00000008;
pub const AR_PHY_AIC_CAL_BT_TOO_WEAK_ERR_S: c_int = 3;
pub const AR_PHY_AIC_CAL_BT_TOO_STRONG_ERR: c_uint = 0x00000004;
pub const AR_PHY_AIC_CAL_BT_TOO_STRONG_ERR_S: c_int = 2;
pub const AR_PHY_AIC_CAL_DONE: c_uint = 0x00000002;
pub const AR_PHY_AIC_CAL_DONE_S: c_int = 1;
pub const AR_PHY_AIC_CAL_ACTIVE: c_uint = 0x00000001;
pub const AR_PHY_AIC_CAL_ACTIVE_S: c_int = 0;
pub const AR_PHY_AIC_MEAS_MAG_MIN: c_uint = 0xFFC00000;
pub const AR_PHY_AIC_MEAS_MAG_MIN_S: c_int = 22;
pub const AR_PHY_AIC_MON_STALE_COUNT: c_uint = 0x003F8000;
pub const AR_PHY_AIC_MON_STALE_COUNT_S: c_int = 15;
pub const AR_PHY_AIC_MON_HOP_COUNT: c_uint = 0x00007F00;
pub const AR_PHY_AIC_MON_HOP_COUNT_S: c_int = 8;
pub const AR_PHY_AIC_CAL_AIC_SM: c_uint = 0x000000F8;
pub const AR_PHY_AIC_CAL_AIC_SM_S: c_int = 3;
pub const AR_PHY_AIC_SM: c_uint = 0x00000007;
pub const AR_PHY_AIC_SM_S: c_int = 0;
pub const AR_PHY_AIC_SRAM_VALID: c_uint = 0x00000001;
pub const AR_PHY_AIC_SRAM_VALID_S: c_int = 0;
pub const AR_PHY_AIC_SRAM_ROT_QUAD_ATT_DB: c_uint = 0x0000007E;
pub const AR_PHY_AIC_SRAM_ROT_QUAD_ATT_DB_S: c_int = 1;
pub const AR_PHY_AIC_SRAM_VGA_QUAD_SIGN: c_uint = 0x00000080;
pub const AR_PHY_AIC_SRAM_VGA_QUAD_SIGN_S: c_int = 7;
pub const AR_PHY_AIC_SRAM_ROT_DIR_ATT_DB: c_uint = 0x00003F00;
pub const AR_PHY_AIC_SRAM_ROT_DIR_ATT_DB_S: c_int = 8;
pub const AR_PHY_AIC_SRAM_VGA_DIR_SIGN: c_uint = 0x00004000;
pub const AR_PHY_AIC_SRAM_VGA_DIR_SIGN_S: c_int = 14;
pub const AR_PHY_AIC_SRAM_COM_ATT_6DB: c_uint = 0x00038000;
pub const AR_PHY_AIC_SRAM_COM_ATT_6DB_S: c_int = 15;
pub const AR_PHY_AIC_CAL_ROT_ATT_DB_EST_ISO: c_uint = 0x0000E000;
pub const AR_PHY_AIC_CAL_ROT_ATT_DB_EST_ISO_S: c_int = 13;
pub const AR_PHY_AIC_CAL_COM_ATT_DB_EST_ISO: c_uint = 0x00001E00;
pub const AR_PHY_AIC_CAL_COM_ATT_DB_EST_ISO_S: c_int = 9;
pub const AR_PHY_AIC_CAL_ISO_EST_INIT_SETTING: c_uint = 0x000001F8;
pub const AR_PHY_AIC_CAL_ISO_EST_INIT_SETTING_S: c_int = 3;
pub const AR_PHY_AIC_CAL_COM_ATT_DB_BACKOFF: c_uint = 0x00000006;
pub const AR_PHY_AIC_CAL_COM_ATT_DB_BACKOFF_S: c_int = 1;
pub const AR_PHY_AIC_CAL_COM_ATT_DB_FIXED: c_uint = 0x00000001;
pub const AR_PHY_AIC_CAL_COM_ATT_DB_FIXED_S: c_int = 0;
