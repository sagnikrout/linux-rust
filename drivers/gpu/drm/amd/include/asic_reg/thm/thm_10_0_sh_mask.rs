//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/thm/thm_10_0_sh_mask.h
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
// Copyright (C) 2017  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _thm_10_0_SH_MASK_HEADER
// addressBlock: thm_thm_SmuThmDec
// THM_TCON_CUR_TMP
pub const THM_TCON_CUR_TMP__PER_STEP_TIME_UP__SHIFT: c_uint = 0x0;
pub const THM_TCON_CUR_TMP__TMP_MAX_DIFF_UP__SHIFT: c_uint = 0x5;
pub const THM_TCON_CUR_TMP__TMP_SLEW_DN_EN__SHIFT: c_uint = 0x7;
pub const THM_TCON_CUR_TMP__PER_STEP_TIME_DN__SHIFT: c_uint = 0x8;
pub const THM_TCON_CUR_TMP__CUR_TEMP_TJ_SEL__SHIFT: c_uint = 0x10;
pub const THM_TCON_CUR_TMP__CUR_TEMP_TJ_SLEW_SEL__SHIFT: c_uint = 0x12;
pub const THM_TCON_CUR_TMP__CUR_TEMP_RANGE_SEL__SHIFT: c_uint = 0x13;
pub const THM_TCON_CUR_TMP__MCM_EN__SHIFT: c_uint = 0x14;
pub const THM_TCON_CUR_TMP__CUR_TEMP__SHIFT: c_uint = 0x15;
pub const THM_TCON_CUR_TMP__PER_STEP_TIME_UP_MASK: c_uint = 0x0000001FL;
pub const THM_TCON_CUR_TMP__TMP_MAX_DIFF_UP_MASK: c_uint = 0x00000060L;
pub const THM_TCON_CUR_TMP__TMP_SLEW_DN_EN_MASK: c_uint = 0x00000080L;
pub const THM_TCON_CUR_TMP__PER_STEP_TIME_DN_MASK: c_uint = 0x00001F00L;
pub const THM_TCON_CUR_TMP__CUR_TEMP_TJ_SEL_MASK: c_uint = 0x00030000L;
pub const THM_TCON_CUR_TMP__CUR_TEMP_TJ_SLEW_SEL_MASK: c_uint = 0x00040000L;
pub const THM_TCON_CUR_TMP__CUR_TEMP_RANGE_SEL_MASK: c_uint = 0x00080000L;
pub const THM_TCON_CUR_TMP__MCM_EN_MASK: c_uint = 0x00100000L;
pub const THM_TCON_CUR_TMP__CUR_TEMP_MASK: c_uint = 0xFFE00000L;
// THM_TCON_HTC
pub const THM_TCON_HTC__HTC_EN__SHIFT: c_uint = 0x0;
pub const THM_TCON_HTC__EXTERNAL_PROCHOT__SHIFT: c_uint = 0x2;
pub const THM_TCON_HTC__INTERNAL_PROCHOT__SHIFT: c_uint = 0x3;
pub const THM_TCON_HTC__HTC_ACTIVE__SHIFT: c_uint = 0x4;
pub const THM_TCON_HTC__HTC_ACTIVE_LOG__SHIFT: c_uint = 0x5;
pub const THM_TCON_HTC__HTC_DIAG__SHIFT: c_uint = 0x8;
pub const THM_TCON_HTC__DIS_PROCHOT_PIN__SHIFT: c_uint = 0x9;
pub const THM_TCON_HTC__HTC_TO_IH_EN__SHIFT: c_uint = 0xa;
pub const THM_TCON_HTC__PROCHOT_TO_IH_EN__SHIFT: c_uint = 0xb;
pub const THM_TCON_HTC__PROCHOT_EVENT_SRC__SHIFT: c_uint = 0xc;
pub const THM_TCON_HTC__HTC_TMP_LMT__SHIFT: c_uint = 0x10;
pub const THM_TCON_HTC__HTC_HYST_LMT__SHIFT: c_uint = 0x17;
pub const THM_TCON_HTC__HTC_SLEW_SEL__SHIFT: c_uint = 0x1b;
pub const THM_TCON_HTC__HTC_EN_MASK: c_uint = 0x00000001L;
pub const THM_TCON_HTC__EXTERNAL_PROCHOT_MASK: c_uint = 0x00000004L;
pub const THM_TCON_HTC__INTERNAL_PROCHOT_MASK: c_uint = 0x00000008L;
pub const THM_TCON_HTC__HTC_ACTIVE_MASK: c_uint = 0x00000010L;
pub const THM_TCON_HTC__HTC_ACTIVE_LOG_MASK: c_uint = 0x00000020L;
pub const THM_TCON_HTC__HTC_DIAG_MASK: c_uint = 0x00000100L;
pub const THM_TCON_HTC__DIS_PROCHOT_PIN_MASK: c_uint = 0x00000200L;
pub const THM_TCON_HTC__HTC_TO_IH_EN_MASK: c_uint = 0x00000400L;
pub const THM_TCON_HTC__PROCHOT_TO_IH_EN_MASK: c_uint = 0x00000800L;
pub const THM_TCON_HTC__PROCHOT_EVENT_SRC_MASK: c_uint = 0x00007000L;
pub const THM_TCON_HTC__HTC_TMP_LMT_MASK: c_uint = 0x007F0000L;
pub const THM_TCON_HTC__HTC_HYST_LMT_MASK: c_uint = 0x07800000L;
pub const THM_TCON_HTC__HTC_SLEW_SEL_MASK: c_uint = 0x18000000L;
// THM_TCON_THERM_TRIP
pub const THM_TCON_THERM_TRIP__CTF_PAD_POLARITY__SHIFT: c_uint = 0x0;
pub const THM_TCON_THERM_TRIP__THERM_TP__SHIFT: c_uint = 0x1;
pub const THM_TCON_THERM_TRIP__CTF_THRESHOLD_EXCEEDED__SHIFT: c_uint = 0x2;
pub const THM_TCON_THERM_TRIP__THERM_TP_SENSE__SHIFT: c_uint = 0x3;
pub const THM_TCON_THERM_TRIP__RSVD2__SHIFT: c_uint = 0x4;
pub const THM_TCON_THERM_TRIP__THERM_TP_EN__SHIFT: c_uint = 0x5;
pub const THM_TCON_THERM_TRIP__THERM_TP_LMT__SHIFT: c_uint = 0x6;
pub const THM_TCON_THERM_TRIP__RSVD3__SHIFT: c_uint = 0xe;
pub const THM_TCON_THERM_TRIP__SW_THERM_TP__SHIFT: c_uint = 0x1f;
pub const THM_TCON_THERM_TRIP__CTF_PAD_POLARITY_MASK: c_uint = 0x00000001L;
pub const THM_TCON_THERM_TRIP__THERM_TP_MASK: c_uint = 0x00000002L;
pub const THM_TCON_THERM_TRIP__CTF_THRESHOLD_EXCEEDED_MASK: c_uint = 0x00000004L;
pub const THM_TCON_THERM_TRIP__THERM_TP_SENSE_MASK: c_uint = 0x00000008L;
pub const THM_TCON_THERM_TRIP__RSVD2_MASK: c_uint = 0x00000010L;
pub const THM_TCON_THERM_TRIP__THERM_TP_EN_MASK: c_uint = 0x00000020L;
pub const THM_TCON_THERM_TRIP__THERM_TP_LMT_MASK: c_uint = 0x00003FC0L;
pub const THM_TCON_THERM_TRIP__RSVD3_MASK: c_uint = 0x7FFFC000L;
pub const THM_TCON_THERM_TRIP__SW_THERM_TP_MASK: c_uint = 0x80000000L;
// THM_CTF_DELAY
pub const THM_CTF_DELAY__CTF_DELAY_CNT__SHIFT: c_uint = 0x0;
pub const THM_CTF_DELAY__CTF_DELAY_CNT_MASK: c_uint = 0x000FFFFFL;
// THM_GPIO_PROCHOT_CTRL
pub const THM_GPIO_PROCHOT_CTRL__TXIMPSEL__SHIFT: c_uint = 0x0;
pub const THM_GPIO_PROCHOT_CTRL__PD__SHIFT: c_uint = 0x1;
pub const THM_GPIO_PROCHOT_CTRL__PU__SHIFT: c_uint = 0x2;
pub const THM_GPIO_PROCHOT_CTRL__SCHMEN__SHIFT: c_uint = 0x3;
pub const THM_GPIO_PROCHOT_CTRL__S0__SHIFT: c_uint = 0x4;
pub const THM_GPIO_PROCHOT_CTRL__S1__SHIFT: c_uint = 0x5;
pub const THM_GPIO_PROCHOT_CTRL__RXEN__SHIFT: c_uint = 0x6;
pub const THM_GPIO_PROCHOT_CTRL__RXSEL0__SHIFT: c_uint = 0x7;
pub const THM_GPIO_PROCHOT_CTRL__RXSEL1__SHIFT: c_uint = 0x8;
pub const THM_GPIO_PROCHOT_CTRL__OE_OVERRIDE__SHIFT: c_uint = 0x10;
pub const THM_GPIO_PROCHOT_CTRL__OE__SHIFT: c_uint = 0x11;
pub const THM_GPIO_PROCHOT_CTRL__A_OVERRIDE__SHIFT: c_uint = 0x12;
pub const THM_GPIO_PROCHOT_CTRL__A__SHIFT: c_uint = 0x13;
pub const THM_GPIO_PROCHOT_CTRL__Y__SHIFT: c_uint = 0x1f;
pub const THM_GPIO_PROCHOT_CTRL__TXIMPSEL_MASK: c_uint = 0x00000001L;
pub const THM_GPIO_PROCHOT_CTRL__PD_MASK: c_uint = 0x00000002L;
pub const THM_GPIO_PROCHOT_CTRL__PU_MASK: c_uint = 0x00000004L;
pub const THM_GPIO_PROCHOT_CTRL__SCHMEN_MASK: c_uint = 0x00000008L;
pub const THM_GPIO_PROCHOT_CTRL__S0_MASK: c_uint = 0x00000010L;
pub const THM_GPIO_PROCHOT_CTRL__S1_MASK: c_uint = 0x00000020L;
pub const THM_GPIO_PROCHOT_CTRL__RXEN_MASK: c_uint = 0x00000040L;
pub const THM_GPIO_PROCHOT_CTRL__RXSEL0_MASK: c_uint = 0x00000080L;
pub const THM_GPIO_PROCHOT_CTRL__RXSEL1_MASK: c_uint = 0x00000100L;
pub const THM_GPIO_PROCHOT_CTRL__OE_OVERRIDE_MASK: c_uint = 0x00010000L;
pub const THM_GPIO_PROCHOT_CTRL__OE_MASK: c_uint = 0x00020000L;
pub const THM_GPIO_PROCHOT_CTRL__A_OVERRIDE_MASK: c_uint = 0x00040000L;
pub const THM_GPIO_PROCHOT_CTRL__A_MASK: c_uint = 0x00080000L;
pub const THM_GPIO_PROCHOT_CTRL__Y_MASK: c_uint = 0x80000000L;
// THM_THERMAL_INT_ENA
pub const THM_THERMAL_INT_ENA__THERM_INTH_SET__SHIFT: c_uint = 0x0;
pub const THM_THERMAL_INT_ENA__THERM_INTL_SET__SHIFT: c_uint = 0x1;
pub const THM_THERMAL_INT_ENA__THERM_TRIGGER_SET__SHIFT: c_uint = 0x2;
pub const THM_THERMAL_INT_ENA__THERM_INTH_CLR__SHIFT: c_uint = 0x3;
pub const THM_THERMAL_INT_ENA__THERM_INTL_CLR__SHIFT: c_uint = 0x4;
pub const THM_THERMAL_INT_ENA__THERM_TRIGGER_CLR__SHIFT: c_uint = 0x5;
pub const THM_THERMAL_INT_ENA__THERM_INTH_SET_MASK: c_uint = 0x00000001L;
pub const THM_THERMAL_INT_ENA__THERM_INTL_SET_MASK: c_uint = 0x00000002L;
pub const THM_THERMAL_INT_ENA__THERM_TRIGGER_SET_MASK: c_uint = 0x00000004L;
pub const THM_THERMAL_INT_ENA__THERM_INTH_CLR_MASK: c_uint = 0x00000008L;
pub const THM_THERMAL_INT_ENA__THERM_INTL_CLR_MASK: c_uint = 0x00000010L;
pub const THM_THERMAL_INT_ENA__THERM_TRIGGER_CLR_MASK: c_uint = 0x00000020L;
// THM_THERMAL_INT_CTRL
pub const THM_THERMAL_INT_CTRL__DIG_THERM_INTH__SHIFT: c_uint = 0x0;
pub const THM_THERMAL_INT_CTRL__DIG_THERM_INTL__SHIFT: c_uint = 0x8;
pub const THM_THERMAL_INT_CTRL__TEMP_THRESHOLD__SHIFT: c_uint = 0x10;
pub const THM_THERMAL_INT_CTRL__THERM_INTH_MASK__SHIFT: c_uint = 0x18;
pub const THM_THERMAL_INT_CTRL__THERM_INTL_MASK__SHIFT: c_uint = 0x19;
pub const THM_THERMAL_INT_CTRL__THERM_TRIGGER_MASK__SHIFT: c_uint = 0x1a;
pub const THM_THERMAL_INT_CTRL__THERM_PROCHOT_MASK__SHIFT: c_uint = 0x1b;
pub const THM_THERMAL_INT_CTRL__THERM_IH_HW_ENA__SHIFT: c_uint = 0x1c;
pub const THM_THERMAL_INT_CTRL__MAX_IH_CREDIT__SHIFT: c_uint = 0x1d;
pub const THM_THERMAL_INT_CTRL__DIG_THERM_INTH_MASK: c_uint = 0x000000FFL;
pub const THM_THERMAL_INT_CTRL__DIG_THERM_INTL_MASK: c_uint = 0x0000FF00L;
pub const THM_THERMAL_INT_CTRL__TEMP_THRESHOLD_MASK: c_uint = 0x00FF0000L;
pub const THM_THERMAL_INT_CTRL__THERM_INTH_MASK_MASK: c_uint = 0x01000000L;
pub const THM_THERMAL_INT_CTRL__THERM_INTL_MASK_MASK: c_uint = 0x02000000L;
pub const THM_THERMAL_INT_CTRL__THERM_TRIGGER_MASK_MASK: c_uint = 0x04000000L;
pub const THM_THERMAL_INT_CTRL__THERM_PROCHOT_MASK_MASK: c_uint = 0x08000000L;
pub const THM_THERMAL_INT_CTRL__THERM_IH_HW_ENA_MASK: c_uint = 0x10000000L;
pub const THM_THERMAL_INT_CTRL__MAX_IH_CREDIT_MASK: c_uint = 0xE0000000L;
// THM_THERMAL_INT_STATUS
pub const THM_THERMAL_INT_STATUS__THERM_INTH_DETECT__SHIFT: c_uint = 0x0;
pub const THM_THERMAL_INT_STATUS__THERM_INTL_DETECT__SHIFT: c_uint = 0x1;
pub const THM_THERMAL_INT_STATUS__THERM_TRIGGER_DETECT__SHIFT: c_uint = 0x2;
pub const THM_THERMAL_INT_STATUS__THERM_PROCHOT_DETECT__SHIFT: c_uint = 0x3;
pub const THM_THERMAL_INT_STATUS__THERM_INTH_DETECT_MASK: c_uint = 0x00000001L;
pub const THM_THERMAL_INT_STATUS__THERM_INTL_DETECT_MASK: c_uint = 0x00000002L;
pub const THM_THERMAL_INT_STATUS__THERM_TRIGGER_DETECT_MASK: c_uint = 0x00000004L;
pub const THM_THERMAL_INT_STATUS__THERM_PROCHOT_DETECT_MASK: c_uint = 0x00000008L;
// THM_TMON0_RDIL0_DATA
pub const THM_TMON0_RDIL0_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL0_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL0_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL0_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL0_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL0_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL1_DATA
pub const THM_TMON0_RDIL1_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL1_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL1_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL1_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL1_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL1_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL2_DATA
pub const THM_TMON0_RDIL2_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL2_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL2_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL2_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL2_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL2_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL3_DATA
pub const THM_TMON0_RDIL3_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL3_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL3_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL3_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL3_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL3_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL4_DATA
pub const THM_TMON0_RDIL4_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL4_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL4_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL4_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL4_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL4_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL5_DATA
pub const THM_TMON0_RDIL5_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL5_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL5_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL5_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL5_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL5_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL6_DATA
pub const THM_TMON0_RDIL6_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL6_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL6_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL6_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL6_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL6_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL7_DATA
pub const THM_TMON0_RDIL7_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL7_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL7_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL7_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL7_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL7_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL8_DATA
pub const THM_TMON0_RDIL8_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL8_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL8_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL8_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL8_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL8_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL9_DATA
pub const THM_TMON0_RDIL9_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL9_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL9_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL9_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL9_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL9_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL10_DATA
pub const THM_TMON0_RDIL10_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL10_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL10_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL10_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL10_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL10_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL11_DATA
pub const THM_TMON0_RDIL11_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL11_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL11_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL11_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL11_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL11_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL12_DATA
pub const THM_TMON0_RDIL12_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL12_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL12_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL12_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL12_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL12_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL13_DATA
pub const THM_TMON0_RDIL13_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL13_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL13_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL13_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL13_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL13_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL14_DATA
pub const THM_TMON0_RDIL14_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL14_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL14_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL14_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL14_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL14_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIL15_DATA
pub const THM_TMON0_RDIL15_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIL15_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIL15_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIL15_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIL15_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIL15_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR0_DATA
pub const THM_TMON0_RDIR0_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR0_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR0_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR0_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR0_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR0_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR1_DATA
pub const THM_TMON0_RDIR1_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR1_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR1_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR1_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR1_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR1_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR2_DATA
pub const THM_TMON0_RDIR2_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR2_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR2_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR2_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR2_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR2_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR3_DATA
pub const THM_TMON0_RDIR3_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR3_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR3_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR3_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR3_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR3_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR4_DATA
pub const THM_TMON0_RDIR4_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR4_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR4_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR4_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR4_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR4_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR5_DATA
pub const THM_TMON0_RDIR5_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR5_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR5_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR5_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR5_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR5_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR6_DATA
pub const THM_TMON0_RDIR6_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR6_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR6_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR6_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR6_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR6_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR7_DATA
pub const THM_TMON0_RDIR7_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR7_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR7_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR7_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR7_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR7_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR8_DATA
pub const THM_TMON0_RDIR8_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR8_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR8_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR8_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR8_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR8_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR9_DATA
pub const THM_TMON0_RDIR9_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR9_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR9_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR9_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR9_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR9_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR10_DATA
pub const THM_TMON0_RDIR10_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR10_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR10_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR10_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR10_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR10_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR11_DATA
pub const THM_TMON0_RDIR11_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR11_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR11_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR11_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR11_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR11_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR12_DATA
pub const THM_TMON0_RDIR12_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR12_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR12_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR12_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR12_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR12_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR13_DATA
pub const THM_TMON0_RDIR13_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR13_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR13_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR13_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR13_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR13_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR14_DATA
pub const THM_TMON0_RDIR14_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR14_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR14_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR14_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR14_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR14_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_RDIR15_DATA
pub const THM_TMON0_RDIR15_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_RDIR15_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_RDIR15_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_RDIR15_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_RDIR15_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_RDIR15_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_INT_DATA
pub const THM_TMON0_INT_DATA__Z__SHIFT: c_uint = 0x0;
pub const THM_TMON0_INT_DATA__VALID__SHIFT: c_uint = 0xb;
pub const THM_TMON0_INT_DATA__TEMP__SHIFT: c_uint = 0xc;
pub const THM_TMON0_INT_DATA__Z_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_INT_DATA__VALID_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_INT_DATA__TEMP_MASK: c_uint = 0x00FFF000L;
// THM_TMON0_CTRL
pub const THM_TMON0_CTRL__POWER_DOWN__SHIFT: c_uint = 0x0;
pub const THM_TMON0_CTRL__BGADJ__SHIFT: c_uint = 0x1;
pub const THM_TMON0_CTRL__BGADJ_MODE__SHIFT: c_uint = 0x9;
pub const THM_TMON0_CTRL__TMON_PAUSE__SHIFT: c_uint = 0xa;
pub const THM_TMON0_CTRL__INT_MEAS_EN__SHIFT: c_uint = 0xb;
pub const THM_TMON0_CTRL__DEBUG_MODE__SHIFT: c_uint = 0xc;
pub const THM_TMON0_CTRL__EN_CFG_SERDES__SHIFT: c_uint = 0xd;
pub const THM_TMON0_CTRL__POWER_DOWN_MASK: c_uint = 0x00000001L;
pub const THM_TMON0_CTRL__BGADJ_MASK: c_uint = 0x000001FEL;
pub const THM_TMON0_CTRL__BGADJ_MODE_MASK: c_uint = 0x00000200L;
pub const THM_TMON0_CTRL__TMON_PAUSE_MASK: c_uint = 0x00000400L;
pub const THM_TMON0_CTRL__INT_MEAS_EN_MASK: c_uint = 0x00000800L;
pub const THM_TMON0_CTRL__DEBUG_MODE_MASK: c_uint = 0x00001000L;
pub const THM_TMON0_CTRL__EN_CFG_SERDES_MASK: c_uint = 0x00002000L;
// THM_TMON0_CTRL2
pub const THM_TMON0_CTRL2__RDIL_PRESENT__SHIFT: c_uint = 0x0;
pub const THM_TMON0_CTRL2__RDIR_PRESENT__SHIFT: c_uint = 0x10;
pub const THM_TMON0_CTRL2__RDIL_PRESENT_MASK: c_uint = 0x0000FFFFL;
pub const THM_TMON0_CTRL2__RDIR_PRESENT_MASK: c_uint = 0xFFFF0000L;
// THM_TMON0_DEBUG
pub const THM_TMON0_DEBUG__DEBUG_RDI__SHIFT: c_uint = 0x0;
pub const THM_TMON0_DEBUG__DEBUG_Z__SHIFT: c_uint = 0x5;
pub const THM_TMON0_DEBUG__DEBUG_RDI_MASK: c_uint = 0x0000001FL;
pub const THM_TMON0_DEBUG__DEBUG_Z_MASK: c_uint = 0x0000FFE0L;
// THM_DIE1_TEMP
pub const THM_DIE1_TEMP__TEMP__SHIFT: c_uint = 0x0;
pub const THM_DIE1_TEMP__VALID__SHIFT: c_uint = 0xb;
pub const THM_DIE1_TEMP__TEMP_MASK: c_uint = 0x000007FFL;
pub const THM_DIE1_TEMP__VALID_MASK: c_uint = 0x00000800L;
// THM_DIE2_TEMP
pub const THM_DIE2_TEMP__TEMP__SHIFT: c_uint = 0x0;
pub const THM_DIE2_TEMP__VALID__SHIFT: c_uint = 0xb;
pub const THM_DIE2_TEMP__TEMP_MASK: c_uint = 0x000007FFL;
pub const THM_DIE2_TEMP__VALID_MASK: c_uint = 0x00000800L;
// THM_DIE3_TEMP
pub const THM_DIE3_TEMP__TEMP__SHIFT: c_uint = 0x0;
pub const THM_DIE3_TEMP__VALID__SHIFT: c_uint = 0xb;
pub const THM_DIE3_TEMP__TEMP_MASK: c_uint = 0x000007FFL;
pub const THM_DIE3_TEMP__VALID_MASK: c_uint = 0x00000800L;
// THM_SW_TEMP
pub const THM_SW_TEMP__SW_TEMP__SHIFT: c_uint = 0x0;
pub const THM_SW_TEMP__SW_TEMP_MASK: c_uint = 0x000001FFL;
// CG_MULT_THERMAL_CTRL
pub const CG_MULT_THERMAL_CTRL__TS_FILTER__SHIFT: c_uint = 0x0;
pub const CG_MULT_THERMAL_CTRL__UNUSED__SHIFT: c_uint = 0x4;
pub const CG_MULT_THERMAL_CTRL__THERMAL_RANGE_RST__SHIFT: c_uint = 0x9;
pub const CG_MULT_THERMAL_CTRL__TEMP_SEL__SHIFT: c_uint = 0x14;
pub const CG_MULT_THERMAL_CTRL__TS_FILTER_MASK: c_uint = 0x0000000FL;
pub const CG_MULT_THERMAL_CTRL__UNUSED_MASK: c_uint = 0x000001F0L;
pub const CG_MULT_THERMAL_CTRL__THERMAL_RANGE_RST_MASK: c_uint = 0x00000200L;
pub const CG_MULT_THERMAL_CTRL__TEMP_SEL_MASK: c_uint = 0x0FF00000L;
// CG_MULT_THERMAL_STATUS
pub const CG_MULT_THERMAL_STATUS__ASIC_MAX_TEMP__SHIFT: c_uint = 0x0;
pub const CG_MULT_THERMAL_STATUS__CTF_TEMP__SHIFT: c_uint = 0x9;
pub const CG_MULT_THERMAL_STATUS__ASIC_MAX_TEMP_MASK: c_uint = 0x000001FFL;
pub const CG_MULT_THERMAL_STATUS__CTF_TEMP_MASK: c_uint = 0x0003FE00L;
// CG_THERMAL_RANGE
pub const CG_THERMAL_RANGE__ASIC_T_MAX__SHIFT: c_uint = 0x0;
pub const CG_THERMAL_RANGE__ASIC_T_MIN__SHIFT: c_uint = 0x10;
pub const CG_THERMAL_RANGE__ASIC_T_MAX_MASK: c_uint = 0x000001FFL;
pub const CG_THERMAL_RANGE__ASIC_T_MIN_MASK: c_uint = 0x01FF0000L;
// THM_TMON_CONFIG
pub const THM_TMON_CONFIG__NUM_ACQ__SHIFT: c_uint = 0x0;
pub const THM_TMON_CONFIG__FORCE_MAX_ACQ__SHIFT: c_uint = 0x3;
pub const THM_TMON_CONFIG__RDI_INTERLEAVE__SHIFT: c_uint = 0x4;
pub const THM_TMON_CONFIG__CONFIG_SOURCE__SHIFT: c_uint = 0x5;
pub const THM_TMON_CONFIG__RE_CALIB_EN__SHIFT: c_uint = 0x6;
pub const THM_TMON_CONFIG__Z__SHIFT: c_uint = 0x15;
pub const THM_TMON_CONFIG__NUM_ACQ_MASK: c_uint = 0x00000007L;
pub const THM_TMON_CONFIG__FORCE_MAX_ACQ_MASK: c_uint = 0x00000008L;
pub const THM_TMON_CONFIG__RDI_INTERLEAVE_MASK: c_uint = 0x00000010L;
pub const THM_TMON_CONFIG__CONFIG_SOURCE_MASK: c_uint = 0x00000020L;
pub const THM_TMON_CONFIG__RE_CALIB_EN_MASK: c_uint = 0x00000040L;
pub const THM_TMON_CONFIG__Z_MASK: c_uint = 0xFFE00000L;
// THM_TMON_CONFIG2
pub const THM_TMON_CONFIG2__A__SHIFT: c_uint = 0x0;
pub const THM_TMON_CONFIG2__B__SHIFT: c_uint = 0xc;
pub const THM_TMON_CONFIG2__C__SHIFT: c_uint = 0x12;
pub const THM_TMON_CONFIG2__K__SHIFT: c_uint = 0x1d;
pub const THM_TMON_CONFIG2__A_MASK: c_uint = 0x00000FFFL;
pub const THM_TMON_CONFIG2__B_MASK: c_uint = 0x0003F000L;
pub const THM_TMON_CONFIG2__C_MASK: c_uint = 0x1FFC0000L;
pub const THM_TMON_CONFIG2__K_MASK: c_uint = 0x20000000L;
// THM_TMON0_COEFF
pub const THM_TMON0_COEFF__C_OFFSET__SHIFT: c_uint = 0x0;
pub const THM_TMON0_COEFF__D__SHIFT: c_uint = 0xb;
pub const THM_TMON0_COEFF__C_OFFSET_MASK: c_uint = 0x000007FFL;
pub const THM_TMON0_COEFF__D_MASK: c_uint = 0x0003F800L;
// THM_TCON_LOCAL0
pub const THM_TCON_LOCAL0__TMON0_PwrDn_Dis__SHIFT: c_uint = 0x1;
pub const THM_TCON_LOCAL0__TMON1_PwrDn_Dis__SHIFT: c_uint = 0x2;
pub const THM_TCON_LOCAL0__TMON0_PwrDn_Dis_MASK: c_uint = 0x00000002L;
pub const THM_TCON_LOCAL0__TMON1_PwrDn_Dis_MASK: c_uint = 0x00000004L;
// THM_TCON_LOCAL1
pub const THM_TCON_LOCAL1__Turn_Off_TMON0__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL1__Turn_Off_TMON1__SHIFT: c_uint = 0x1;
pub const THM_TCON_LOCAL1__PowerDownTmon0__SHIFT: c_uint = 0x4;
pub const THM_TCON_LOCAL1__PowerDownTmon1__SHIFT: c_uint = 0x5;
pub const THM_TCON_LOCAL1__Turn_Off_TMON0_MASK: c_uint = 0x00000001L;
pub const THM_TCON_LOCAL1__Turn_Off_TMON1_MASK: c_uint = 0x00000002L;
pub const THM_TCON_LOCAL1__PowerDownTmon0_MASK: c_uint = 0x00000010L;
pub const THM_TCON_LOCAL1__PowerDownTmon1_MASK: c_uint = 0x00000020L;
// THM_TCON_LOCAL2
pub const THM_TCON_LOCAL2__TMON_init_delay__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL2__TMON_pwrup_stagger_time__SHIFT: c_uint = 0x2;
pub const THM_TCON_LOCAL2__short_stagger_count__SHIFT: c_uint = 0x5;
pub const THM_TCON_LOCAL2__sbtsi_use_corrected__SHIFT: c_uint = 0x6;
pub const THM_TCON_LOCAL2__temp_read_skip_scale__SHIFT: c_uint = 0xa;
pub const THM_TCON_LOCAL2__skip_scale_correction__SHIFT: c_uint = 0xb;
pub const THM_TCON_LOCAL2__TMON_init_delay_MASK: c_uint = 0x00000003L;
pub const THM_TCON_LOCAL2__TMON_pwrup_stagger_time_MASK: c_uint = 0x0000000CL;
pub const THM_TCON_LOCAL2__short_stagger_count_MASK: c_uint = 0x00000020L;
pub const THM_TCON_LOCAL2__sbtsi_use_corrected_MASK: c_uint = 0x00000040L;
pub const THM_TCON_LOCAL2__temp_read_skip_scale_MASK: c_uint = 0x00000400L;
pub const THM_TCON_LOCAL2__skip_scale_correction_MASK: c_uint = 0x00000800L;
// THM_TCON_LOCAL3
pub const THM_TCON_LOCAL3__Global_TMAX__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL3__Global_TMAX_MASK: c_uint = 0x000007FFL;
// THM_TCON_LOCAL4
pub const THM_TCON_LOCAL4__Global_TMAX_ID__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL4__Global_TMAX_ID_MASK: c_uint = 0x000000FFL;
// THM_TCON_LOCAL5
pub const THM_TCON_LOCAL5__Global_TMIN__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL5__Global_TMIN_MASK: c_uint = 0x000007FFL;
// THM_TCON_LOCAL6
pub const THM_TCON_LOCAL6__Global_TMIN_ID__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL6__Global_TMIN_ID_MASK: c_uint = 0x000000FFL;
// THM_TCON_LOCAL7
pub const THM_TCON_LOCAL7__THERMID__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL7__THERMID_MASK: c_uint = 0x000000FFL;
// THM_TCON_LOCAL8
pub const THM_TCON_LOCAL8__THERMMAX__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL8__THERMMAX_MASK: c_uint = 0x000007FFL;
// THM_TCON_LOCAL9
pub const THM_TCON_LOCAL9__Tj_Max_TMON0__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL9__Tj_Max_TMON0_MASK: c_uint = 0x000007FFL;
// THM_TCON_LOCAL10
pub const THM_TCON_LOCAL10__TMON0_Tj_Max_RS_ID__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL10__TMON0_Tj_Max_RS_ID_MASK: c_uint = 0x000000FFL;
// THM_TCON_LOCAL11
pub const THM_TCON_LOCAL11__Tj_Max_TMON1__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL11__Tj_Max_TMON1_MASK: c_uint = 0x000007FFL;
// THM_TCON_LOCAL12
pub const THM_TCON_LOCAL12__TMON1_Tj_Max_RS_ID__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL12__TMON1_Tj_Max_RS_ID_MASK: c_uint = 0x000000FFL;
// THM_TCON_LOCAL13
pub const THM_TCON_LOCAL13__boot_done__SHIFT: c_uint = 0x0;
pub const THM_TCON_LOCAL13__boot_done_MASK: c_uint = 0x00000001L;
// THM_PWRMGT
pub const THM_PWRMGT__SBTSI_SBRMI_CLK_GATE_EN__SHIFT: c_uint = 0x0;
pub const THM_PWRMGT__SBAXI_CLK_GATE_EN__SHIFT: c_uint = 0x1;
pub const THM_PWRMGT__SB_CLK_GATE_MAX_CNT__SHIFT: c_uint = 0x8;
pub const THM_PWRMGT__SBTSI_SBRMI_CLK_GATE_EN_MASK: c_uint = 0x00000001L;
pub const THM_PWRMGT__SBAXI_CLK_GATE_EN_MASK: c_uint = 0x00000002L;
pub const THM_PWRMGT__SB_CLK_GATE_MAX_CNT_MASK: c_uint = 0x00FFFF00L;
// SMUSBI_SBIREGADDR
pub const SMUSBI_SBIREGADDR__SBI_REGADDR__SHIFT: c_uint = 0x0;
pub const SMUSBI_SBIREGADDR__SBI_REGADDR_MASK: c_uint = 0x000007FFL;
// SMUSBI_SBIREGDATA
pub const SMUSBI_SBIREGDATA__SBI_REGDATA__SHIFT: c_uint = 0x0;
pub const SMUSBI_SBIREGDATA__SBI_REGDATA_MASK: c_uint = 0xFFFFFFFFL;
// SMUSBI_ERRATA_STAT_REG
pub const SMUSBI_ERRATA_STAT_REG__ERRATA_STAT_REG__SHIFT: c_uint = 0x0;
pub const SMUSBI_ERRATA_STAT_REG__ERRATA_STAT_REG_MASK: c_uint = 0xFFFFFFFFL;
// SMUSBI_SBICTRL
pub const SMUSBI_SBICTRL__CK_SPRSBIWRDONE__SHIFT: c_uint = 0x0;
pub const SMUSBI_SBICTRL__NB_SBISELECT__SHIFT: c_uint = 0x1;
pub const SMUSBI_SBICTRL__NB_SBIADDR__SHIFT: c_uint = 0x2;
pub const SMUSBI_SBICTRL__NB_SBIADDR_OVERRIDE__SHIFT: c_uint = 0x5;
pub const SMUSBI_SBICTRL__CK_SPRSBIWRDONE_MASK: c_uint = 0x00000001L;
pub const SMUSBI_SBICTRL__NB_SBISELECT_MASK: c_uint = 0x00000002L;
pub const SMUSBI_SBICTRL__NB_SBIADDR_MASK: c_uint = 0x0000001CL;
pub const SMUSBI_SBICTRL__NB_SBIADDR_OVERRIDE_MASK: c_uint = 0x00000020L;
// SMUSBI_CKNBIRESET
pub const SMUSBI_CKNBIRESET__CKNBIRESET__SHIFT: c_uint = 0x0;
pub const SMUSBI_CKNBIRESET__CKNBIRESET_MASK: c_uint = 0x00000001L;
// SMUSBI_TIMING
pub const SMUSBI_TIMING__SETUP_TIME__SHIFT: c_uint = 0x0;
pub const SMUSBI_TIMING__SETUP_TIME_OVERRIDE__SHIFT: c_uint = 0x8;
pub const SMUSBI_TIMING__HOLD_TIME__SHIFT: c_uint = 0x10;
pub const SMUSBI_TIMING__HOLD_TIME_OVERRIDE__SHIFT: c_uint = 0x18;
pub const SMUSBI_TIMING__SETUP_TIME_MASK: c_uint = 0x0000003FL;
pub const SMUSBI_TIMING__SETUP_TIME_OVERRIDE_MASK: c_uint = 0x00000100L;
pub const SMUSBI_TIMING__HOLD_TIME_MASK: c_uint = 0x00FF0000L;
pub const SMUSBI_TIMING__HOLD_TIME_OVERRIDE_MASK: c_uint = 0x01000000L;
// SMUSBI_HS_TIMING
pub const SMUSBI_HS_TIMING__HS_SETUP_TIME__SHIFT: c_uint = 0x0;
pub const SMUSBI_HS_TIMING__HS_SETUP_TIME_OVERRIDE__SHIFT: c_uint = 0x8;
pub const SMUSBI_HS_TIMING__HS_HOLD_TIME__SHIFT: c_uint = 0x10;
pub const SMUSBI_HS_TIMING__HS_HOLD_TIME_OVERRIDE__SHIFT: c_uint = 0x18;
pub const SMUSBI_HS_TIMING__HS_SETUP_TIME_MASK: c_uint = 0x0000003FL;
pub const SMUSBI_HS_TIMING__HS_SETUP_TIME_OVERRIDE_MASK: c_uint = 0x00000100L;
pub const SMUSBI_HS_TIMING__HS_HOLD_TIME_MASK: c_uint = 0x00FF0000L;
pub const SMUSBI_HS_TIMING__HS_HOLD_TIME_OVERRIDE_MASK: c_uint = 0x01000000L;
// SBTSI_REMOTE_TEMP
pub const SBTSI_REMOTE_TEMP__RemoteTcenSensor__SHIFT: c_uint = 0x0;
pub const SBTSI_REMOTE_TEMP__RemoteTcenSensorId__SHIFT: c_uint = 0xb;
pub const SBTSI_REMOTE_TEMP__RemoteTcenSensorValid__SHIFT: c_uint = 0x13;
pub const SBTSI_REMOTE_TEMP__RemoteTcenSensor_MASK: c_uint = 0x000007FFL;
pub const SBTSI_REMOTE_TEMP__RemoteTcenSensorId_MASK: c_uint = 0x0007F800L;
pub const SBTSI_REMOTE_TEMP__RemoteTcenSensorValid_MASK: c_uint = 0x00080000L;
// SBRMI_CONTROL
pub const SBRMI_CONTROL__READ_CMD_INT_DIS__SHIFT: c_uint = 0x0;
pub const SBRMI_CONTROL__DPD__SHIFT: c_uint = 0x1;
pub const SBRMI_CONTROL__DbrdySts__SHIFT: c_uint = 0x2;
pub const SBRMI_CONTROL__READ_CMD_INT_DIS_MASK: c_uint = 0x00000001L;
pub const SBRMI_CONTROL__DPD_MASK: c_uint = 0x00000002L;
pub const SBRMI_CONTROL__DbrdySts_MASK: c_uint = 0x00000004L;
// SBRMI_COMMAND
pub const SBRMI_COMMAND__Command__SHIFT: c_uint = 0x0;
pub const SBRMI_COMMAND__WrDataLen__SHIFT: c_uint = 0x8;
pub const SBRMI_COMMAND__RdDataLen__SHIFT: c_uint = 0x10;
pub const SBRMI_COMMAND__CommandSent__SHIFT: c_uint = 0x18;
pub const SBRMI_COMMAND__CommandNotSupported__SHIFT: c_uint = 0x19;
pub const SBRMI_COMMAND__CommandAborted__SHIFT: c_uint = 0x1a;
pub const SBRMI_COMMAND__Status__SHIFT: c_uint = 0x1c;
pub const SBRMI_COMMAND__Command_MASK: c_uint = 0x000000FFL;
pub const SBRMI_COMMAND__WrDataLen_MASK: c_uint = 0x0000FF00L;
pub const SBRMI_COMMAND__RdDataLen_MASK: c_uint = 0x00FF0000L;
pub const SBRMI_COMMAND__CommandSent_MASK: c_uint = 0x01000000L;
pub const SBRMI_COMMAND__CommandNotSupported_MASK: c_uint = 0x02000000L;
pub const SBRMI_COMMAND__CommandAborted_MASK: c_uint = 0x04000000L;
pub const SBRMI_COMMAND__Status_MASK: c_uint = 0xF0000000L;
// SBRMI_WRITE_DATA0
pub const SBRMI_WRITE_DATA0__WrByte0__SHIFT: c_uint = 0x0;
pub const SBRMI_WRITE_DATA0__WrByte1__SHIFT: c_uint = 0x8;
pub const SBRMI_WRITE_DATA0__WrByte2__SHIFT: c_uint = 0x10;
pub const SBRMI_WRITE_DATA0__WrByte3__SHIFT: c_uint = 0x18;
pub const SBRMI_WRITE_DATA0__WrByte0_MASK: c_uint = 0x000000FFL;
pub const SBRMI_WRITE_DATA0__WrByte1_MASK: c_uint = 0x0000FF00L;
pub const SBRMI_WRITE_DATA0__WrByte2_MASK: c_uint = 0x00FF0000L;
pub const SBRMI_WRITE_DATA0__WrByte3_MASK: c_uint = 0xFF000000L;
// SBRMI_WRITE_DATA1
pub const SBRMI_WRITE_DATA1__WrByte4__SHIFT: c_uint = 0x0;
pub const SBRMI_WRITE_DATA1__WrByte5__SHIFT: c_uint = 0x8;
pub const SBRMI_WRITE_DATA1__WrByte6__SHIFT: c_uint = 0x10;
pub const SBRMI_WRITE_DATA1__WrByte7__SHIFT: c_uint = 0x18;
pub const SBRMI_WRITE_DATA1__WrByte4_MASK: c_uint = 0x000000FFL;
pub const SBRMI_WRITE_DATA1__WrByte5_MASK: c_uint = 0x0000FF00L;
pub const SBRMI_WRITE_DATA1__WrByte6_MASK: c_uint = 0x00FF0000L;
pub const SBRMI_WRITE_DATA1__WrByte7_MASK: c_uint = 0xFF000000L;
// SBRMI_WRITE_DATA2
pub const SBRMI_WRITE_DATA2__WrByte8__SHIFT: c_uint = 0x0;
pub const SBRMI_WRITE_DATA2__WrByte9__SHIFT: c_uint = 0x8;
pub const SBRMI_WRITE_DATA2__WrByte10__SHIFT: c_uint = 0x10;
pub const SBRMI_WRITE_DATA2__WrByte11__SHIFT: c_uint = 0x18;
pub const SBRMI_WRITE_DATA2__WrByte8_MASK: c_uint = 0x000000FFL;
pub const SBRMI_WRITE_DATA2__WrByte9_MASK: c_uint = 0x0000FF00L;
pub const SBRMI_WRITE_DATA2__WrByte10_MASK: c_uint = 0x00FF0000L;
pub const SBRMI_WRITE_DATA2__WrByte11_MASK: c_uint = 0xFF000000L;
// SBRMI_READ_DATA0
pub const SBRMI_READ_DATA0__RdByte0__SHIFT: c_uint = 0x0;
pub const SBRMI_READ_DATA0__RdByte1__SHIFT: c_uint = 0x8;
pub const SBRMI_READ_DATA0__RdByte2__SHIFT: c_uint = 0x10;
pub const SBRMI_READ_DATA0__RdByte3__SHIFT: c_uint = 0x18;
pub const SBRMI_READ_DATA0__RdByte0_MASK: c_uint = 0x000000FFL;
pub const SBRMI_READ_DATA0__RdByte1_MASK: c_uint = 0x0000FF00L;
pub const SBRMI_READ_DATA0__RdByte2_MASK: c_uint = 0x00FF0000L;
pub const SBRMI_READ_DATA0__RdByte3_MASK: c_uint = 0xFF000000L;
// SBRMI_READ_DATA1
pub const SBRMI_READ_DATA1__RdByte4__SHIFT: c_uint = 0x0;
pub const SBRMI_READ_DATA1__RdByte5__SHIFT: c_uint = 0x8;
pub const SBRMI_READ_DATA1__RdByte6__SHIFT: c_uint = 0x10;
pub const SBRMI_READ_DATA1__RdByte7__SHIFT: c_uint = 0x18;
pub const SBRMI_READ_DATA1__RdByte4_MASK: c_uint = 0x000000FFL;
pub const SBRMI_READ_DATA1__RdByte5_MASK: c_uint = 0x0000FF00L;
pub const SBRMI_READ_DATA1__RdByte6_MASK: c_uint = 0x00FF0000L;
pub const SBRMI_READ_DATA1__RdByte7_MASK: c_uint = 0xFF000000L;
// SBRMI_CORE_EN_NUMBER
pub const SBRMI_CORE_EN_NUMBER__EnabledCoreNum__SHIFT: c_uint = 0x0;
pub const SBRMI_CORE_EN_NUMBER__EnabledCoreNum_MASK: c_uint = 0x0000007FL;
// SBRMI_CORE_EN_STATUS0
pub const SBRMI_CORE_EN_STATUS0__CoreEnStat0__SHIFT: c_uint = 0x0;
pub const SBRMI_CORE_EN_STATUS0__CoreEnStat0_MASK: c_uint = 0xFFFFFFFFL;
// SBRMI_CORE_EN_STATUS1
pub const SBRMI_CORE_EN_STATUS1__CoreEnStat1__SHIFT: c_uint = 0x0;
pub const SBRMI_CORE_EN_STATUS1__CoreEnStat1_MASK: c_uint = 0xFFFFFFFFL;
// SBRMI_APIC_STATUS0
pub const SBRMI_APIC_STATUS0__APICStat0__SHIFT: c_uint = 0x0;
pub const SBRMI_APIC_STATUS0__APICStat0_MASK: c_uint = 0xFFFFFFFFL;
// SBRMI_APIC_STATUS1
pub const SBRMI_APIC_STATUS1__APICStat1__SHIFT: c_uint = 0x0;
pub const SBRMI_APIC_STATUS1__APICStat1_MASK: c_uint = 0xFFFFFFFFL;
// SBRMI_MCE_STATUS0
pub const SBRMI_MCE_STATUS0__MceStat0__SHIFT: c_uint = 0x0;
pub const SBRMI_MCE_STATUS0__MceStat0_MASK: c_uint = 0xFFFFFFFFL;
// SBRMI_MCE_STATUS1
pub const SBRMI_MCE_STATUS1__MceStat1__SHIFT: c_uint = 0x0;
pub const SBRMI_MCE_STATUS1__MceStat1_MASK: c_uint = 0xFFFFFFFFL;
// SMBUS_CNTL0
pub const SMBUS_CNTL0__SMB_DEFAULT_SLV_ADDR_OVERRIDE__SHIFT: c_uint = 0x0;
pub const SMBUS_CNTL0__SMB_DEFAULT_SLV_ADDR__SHIFT: c_uint = 0x1;
pub const SMBUS_CNTL0__SMB_CPL_DUMMY_BYTE__SHIFT: c_uint = 0x8;
pub const SMBUS_CNTL0__SMB_NOTIFY_ARP_MAX_TIMES__SHIFT: c_uint = 0x10;
pub const SMBUS_CNTL0__THM_READY__SHIFT: c_uint = 0x14;
pub const SMBUS_CNTL0__SMB_DEFAULT_SLV_ADDR_OVERRIDE_MASK: c_uint = 0x00000001L;
pub const SMBUS_CNTL0__SMB_DEFAULT_SLV_ADDR_MASK: c_uint = 0x000000FEL;
pub const SMBUS_CNTL0__SMB_CPL_DUMMY_BYTE_MASK: c_uint = 0x0000FF00L;
pub const SMBUS_CNTL0__SMB_NOTIFY_ARP_MAX_TIMES_MASK: c_uint = 0x00070000L;
pub const SMBUS_CNTL0__THM_READY_MASK: c_uint = 0x00100000L;
// SMBUS_CNTL1
pub const SMBUS_CNTL1__SMB_TIMEOUT_EN__SHIFT: c_uint = 0x0;
pub const SMBUS_CNTL1__SMB_BLK_WR_CMD_EN__SHIFT: c_uint = 0x1;
pub const SMBUS_CNTL1__SMB_BLK_RD_CMD_EN__SHIFT: c_uint = 0x9;
pub const SMBUS_CNTL1__SMB_TIMEOUT_EN_MASK: c_uint = 0x00000001L;
pub const SMBUS_CNTL1__SMB_BLK_WR_CMD_EN_MASK: c_uint = 0x000001FEL;
pub const SMBUS_CNTL1__SMB_BLK_RD_CMD_EN_MASK: c_uint = 0x0001FE00L;
// SMBUS_BLKWR_CMD_CTRL0
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD0__SHIFT: c_uint = 0x0;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD1__SHIFT: c_uint = 0x8;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD2__SHIFT: c_uint = 0x10;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD3__SHIFT: c_uint = 0x18;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD0_MASK: c_uint = 0x000000FFL;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD1_MASK: c_uint = 0x0000FF00L;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD2_MASK: c_uint = 0x00FF0000L;
pub const SMBUS_BLKWR_CMD_CTRL0__SMB_BLK_WR_CMD3_MASK: c_uint = 0xFF000000L;
// SMBUS_BLKWR_CMD_CTRL1
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD4__SHIFT: c_uint = 0x0;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD5__SHIFT: c_uint = 0x8;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD6__SHIFT: c_uint = 0x10;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD7__SHIFT: c_uint = 0x18;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD4_MASK: c_uint = 0x000000FFL;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD5_MASK: c_uint = 0x0000FF00L;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD6_MASK: c_uint = 0x00FF0000L;
pub const SMBUS_BLKWR_CMD_CTRL1__SMB_BLK_WR_CMD7_MASK: c_uint = 0xFF000000L;
// SMBUS_BLKRD_CMD_CTRL0
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD0__SHIFT: c_uint = 0x0;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD1__SHIFT: c_uint = 0x8;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD2__SHIFT: c_uint = 0x10;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD3__SHIFT: c_uint = 0x18;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD0_MASK: c_uint = 0x000000FFL;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD1_MASK: c_uint = 0x0000FF00L;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD2_MASK: c_uint = 0x00FF0000L;
pub const SMBUS_BLKRD_CMD_CTRL0__SMB_BLK_RD_CMD3_MASK: c_uint = 0xFF000000L;
// SMBUS_BLKRD_CMD_CTRL1
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD4__SHIFT: c_uint = 0x0;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD5__SHIFT: c_uint = 0x8;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD6__SHIFT: c_uint = 0x10;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD7__SHIFT: c_uint = 0x18;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD4_MASK: c_uint = 0x000000FFL;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD5_MASK: c_uint = 0x0000FF00L;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD6_MASK: c_uint = 0x00FF0000L;
pub const SMBUS_BLKRD_CMD_CTRL1__SMB_BLK_RD_CMD7_MASK: c_uint = 0xFF000000L;
// SMBUS_TIMING_CNTL0
pub const SMBUS_TIMING_CNTL0__SMB_TIMEOUT_MARGIN__SHIFT: c_uint = 0x0;
pub const SMBUS_TIMING_CNTL0__SMB_FILTER_LEVEL_CONVERT_MARGIN__SHIFT: c_uint = 0x16;
pub const SMBUS_TIMING_CNTL0__SMB_TIMEOUT_MARGIN_MASK: c_uint = 0x003FFFFFL;
pub const SMBUS_TIMING_CNTL0__SMB_FILTER_LEVEL_CONVERT_MARGIN_MASK: c_uint = 0x3FC00000L;
// SMBUS_TIMING_CNTL1
pub const SMBUS_TIMING_CNTL1__SMB_DAT_SETUP_TIME_MARGIN__SHIFT: c_uint = 0x0;
pub const SMBUS_TIMING_CNTL1__SMB_DAT_HOLD_TIME_MARGIN__SHIFT: c_uint = 0x5;
pub const SMBUS_TIMING_CNTL1__SMB_START_AND_STOP_TIMING_MARGIN__SHIFT: c_uint = 0xb;
pub const SMBUS_TIMING_CNTL1__SMB_BUS_FREE_MARGIN__SHIFT: c_uint = 0x14;
pub const SMBUS_TIMING_CNTL1__SMB_DAT_SETUP_TIME_MARGIN_MASK: c_uint = 0x0000001FL;
pub const SMBUS_TIMING_CNTL1__SMB_DAT_HOLD_TIME_MARGIN_MASK: c_uint = 0x000007E0L;
pub const SMBUS_TIMING_CNTL1__SMB_START_AND_STOP_TIMING_MARGIN_MASK: c_uint = 0x000FF800L;
pub const SMBUS_TIMING_CNTL1__SMB_BUS_FREE_MARGIN_MASK: c_uint = 0x3FF00000L;
// SMBUS_TIMING_CNTL2
pub const SMBUS_TIMING_CNTL2__SMB_SMBCLK_HIGHMAX_MARGIN__SHIFT: c_uint = 0x0;
pub const SMBUS_TIMING_CNTL2__SMBCLK_LEVEL_CTRL_MARGIN__SHIFT: c_uint = 0xd;
pub const SMBUS_TIMING_CNTL2__SMB_SMBCLK_HIGHMAX_MARGIN_MASK: c_uint = 0x00001FFFL;
pub const SMBUS_TIMING_CNTL2__SMBCLK_LEVEL_CTRL_MARGIN_MASK: c_uint = 0x07FFE000L;
// SMBUS_TRIGGER_CNTL
pub const SMBUS_TRIGGER_CNTL__SMB_SOFT_RESET_TRIGGER__SHIFT: c_uint = 0x0;
pub const SMBUS_TRIGGER_CNTL__SMB_NOTIFY_ARP_TRIGGER__SHIFT: c_uint = 0x8;
pub const SMBUS_TRIGGER_CNTL__SMB_SOFT_RESET_TRIGGER_MASK: c_uint = 0x00000001L;
pub const SMBUS_TRIGGER_CNTL__SMB_NOTIFY_ARP_TRIGGER_MASK: c_uint = 0x00000100L;
// SMBUS_UDID_CNTL0
pub const SMBUS_UDID_CNTL0__SMB_PRBS_INI_SEED__SHIFT: c_uint = 0x0;
pub const SMBUS_UDID_CNTL0__SMB_SRST_REGEN_UDID_EN__SHIFT: c_uint = 0x1f;
pub const SMBUS_UDID_CNTL0__SMB_PRBS_INI_SEED_MASK: c_uint = 0x7FFFFFFFL;
pub const SMBUS_UDID_CNTL0__SMB_SRST_REGEN_UDID_EN_MASK: c_uint = 0x80000000L;
// SMBUS_UDID_CNTL1
pub const SMBUS_UDID_CNTL1__SMB_UDID_31_0__SHIFT: c_uint = 0x0;
pub const SMBUS_UDID_CNTL1__SMB_UDID_31_0_MASK: c_uint = 0xFFFFFFFFL;
// SMBUS_UDID_CNTL2
pub const SMBUS_UDID_CNTL2__PEC_SUPPORTED__SHIFT: c_uint = 0x0;
pub const SMBUS_UDID_CNTL2__UDID_VERSION__SHIFT: c_uint = 0x1;
pub const SMBUS_UDID_CNTL2__SMBUS_VERSION__SHIFT: c_uint = 0x4;
pub const SMBUS_UDID_CNTL2__OEM__SHIFT: c_uint = 0x8;
pub const SMBUS_UDID_CNTL2__ASF__SHIFT: c_uint = 0x9;
pub const SMBUS_UDID_CNTL2__IPMI__SHIFT: c_uint = 0xa;
pub const SMBUS_UDID_CNTL2__PEC_SUPPORTED_MASK: c_uint = 0x00000001L;
pub const SMBUS_UDID_CNTL2__UDID_VERSION_MASK: c_uint = 0x0000000EL;
pub const SMBUS_UDID_CNTL2__SMBUS_VERSION_MASK: c_uint = 0x000000F0L;
pub const SMBUS_UDID_CNTL2__OEM_MASK: c_uint = 0x00000100L;
pub const SMBUS_UDID_CNTL2__ASF_MASK: c_uint = 0x00000200L;
pub const SMBUS_UDID_CNTL2__IPMI_MASK: c_uint = 0x00000400L;
// SMUSBI_SMBUS
pub const SMUSBI_SMBUS__Spare0__SHIFT: c_uint = 0x0;
pub const SMUSBI_SMBUS__Spare1__SHIFT: c_uint = 0x1;
pub const SMUSBI_SMBUS__ResBiasEn__SHIFT: c_uint = 0x2;
pub const SMUSBI_SMBUS__CompSel__SHIFT: c_uint = 0x3;
pub const SMUSBI_SMBUS__NG__SHIFT: c_uint = 0x4;
pub const SMUSBI_SMBUS__I2cRxSel__SHIFT: c_uint = 0x8;
pub const SMUSBI_SMBUS__PdEn0__SHIFT: c_uint = 0xa;
pub const SMUSBI_SMBUS__PdEn1__SHIFT: c_uint = 0xb;
pub const SMUSBI_SMBUS__FallSlewSel__SHIFT: c_uint = 0xc;
pub const SMUSBI_SMBUS__Slewn__SHIFT: c_uint = 0xe;
pub const SMUSBI_SMBUS__SpikeRcEn__SHIFT: c_uint = 0xf;
pub const SMUSBI_SMBUS__SpikeRcSel__SHIFT: c_uint = 0x10;
pub const SMUSBI_SMBUS__CSel0p9__SHIFT: c_uint = 0x11;
pub const SMUSBI_SMBUS__CSel1p1__SHIFT: c_uint = 0x12;
pub const SMUSBI_SMBUS__RSel0p9__SHIFT: c_uint = 0x13;
pub const SMUSBI_SMBUS__RSel1p1__SHIFT: c_uint = 0x14;
pub const SMUSBI_SMBUS__BiasCrtEn__SHIFT: c_uint = 0x15;
pub const SMUSBI_SMBUS__DI2C0__SHIFT: c_uint = 0x16;
pub const SMUSBI_SMBUS__DI2C1__SHIFT: c_uint = 0x17;
pub const SMUSBI_SMBUS__DI2C0_OVERRIDE__SHIFT: c_uint = 0x18;
pub const SMUSBI_SMBUS__DI2C1_OVERRIDE__SHIFT: c_uint = 0x19;
pub const SMUSBI_SMBUS__Y0__SHIFT: c_uint = 0x1e;
pub const SMUSBI_SMBUS__Y1__SHIFT: c_uint = 0x1f;
pub const SMUSBI_SMBUS__Spare0_MASK: c_uint = 0x00000001L;
pub const SMUSBI_SMBUS__Spare1_MASK: c_uint = 0x00000002L;
pub const SMUSBI_SMBUS__ResBiasEn_MASK: c_uint = 0x00000004L;
pub const SMUSBI_SMBUS__CompSel_MASK: c_uint = 0x00000008L;
pub const SMUSBI_SMBUS__NG_MASK: c_uint = 0x000000F0L;
pub const SMUSBI_SMBUS__I2cRxSel_MASK: c_uint = 0x00000300L;
pub const SMUSBI_SMBUS__PdEn0_MASK: c_uint = 0x00000400L;
pub const SMUSBI_SMBUS__PdEn1_MASK: c_uint = 0x00000800L;
pub const SMUSBI_SMBUS__FallSlewSel_MASK: c_uint = 0x00003000L;
pub const SMUSBI_SMBUS__Slewn_MASK: c_uint = 0x00004000L;
pub const SMUSBI_SMBUS__SpikeRcEn_MASK: c_uint = 0x00008000L;
pub const SMUSBI_SMBUS__SpikeRcSel_MASK: c_uint = 0x00010000L;
pub const SMUSBI_SMBUS__CSel0p9_MASK: c_uint = 0x00020000L;
pub const SMUSBI_SMBUS__CSel1p1_MASK: c_uint = 0x00040000L;
pub const SMUSBI_SMBUS__RSel0p9_MASK: c_uint = 0x00080000L;
pub const SMUSBI_SMBUS__RSel1p1_MASK: c_uint = 0x00100000L;
pub const SMUSBI_SMBUS__BiasCrtEn_MASK: c_uint = 0x00200000L;
pub const SMUSBI_SMBUS__DI2C0_MASK: c_uint = 0x00400000L;
pub const SMUSBI_SMBUS__DI2C1_MASK: c_uint = 0x00800000L;
pub const SMUSBI_SMBUS__DI2C0_OVERRIDE_MASK: c_uint = 0x01000000L;
pub const SMUSBI_SMBUS__DI2C1_OVERRIDE_MASK: c_uint = 0x02000000L;
pub const SMUSBI_SMBUS__Y0_MASK: c_uint = 0x40000000L;
pub const SMUSBI_SMBUS__Y1_MASK: c_uint = 0x80000000L;
// SMUSBI_ALERT
pub const SMUSBI_ALERT__TXIMPSEL__SHIFT: c_uint = 0x0;
pub const SMUSBI_ALERT__PD__SHIFT: c_uint = 0x1;
pub const SMUSBI_ALERT__PU__SHIFT: c_uint = 0x2;
pub const SMUSBI_ALERT__SCHMEN__SHIFT: c_uint = 0x3;
pub const SMUSBI_ALERT__S0__SHIFT: c_uint = 0x4;
pub const SMUSBI_ALERT__S1__SHIFT: c_uint = 0x5;
pub const SMUSBI_ALERT__RXEN__SHIFT: c_uint = 0x6;
pub const SMUSBI_ALERT__RXSEL0__SHIFT: c_uint = 0x7;
pub const SMUSBI_ALERT__RXSEL1__SHIFT: c_uint = 0x8;
pub const SMUSBI_ALERT__OE_OVERRIDE__SHIFT: c_uint = 0x10;
pub const SMUSBI_ALERT__OE__SHIFT: c_uint = 0x11;
pub const SMUSBI_ALERT__A_OVERRIDE__SHIFT: c_uint = 0x12;
pub const SMUSBI_ALERT__A__SHIFT: c_uint = 0x13;
pub const SMUSBI_ALERT__Y__SHIFT: c_uint = 0x1f;
pub const SMUSBI_ALERT__TXIMPSEL_MASK: c_uint = 0x00000001L;
pub const SMUSBI_ALERT__PD_MASK: c_uint = 0x00000002L;
pub const SMUSBI_ALERT__PU_MASK: c_uint = 0x00000004L;
pub const SMUSBI_ALERT__SCHMEN_MASK: c_uint = 0x00000008L;
pub const SMUSBI_ALERT__S0_MASK: c_uint = 0x00000010L;
pub const SMUSBI_ALERT__S1_MASK: c_uint = 0x00000020L;
pub const SMUSBI_ALERT__RXEN_MASK: c_uint = 0x00000040L;
pub const SMUSBI_ALERT__RXSEL0_MASK: c_uint = 0x00000080L;
pub const SMUSBI_ALERT__RXSEL1_MASK: c_uint = 0x00000100L;
pub const SMUSBI_ALERT__OE_OVERRIDE_MASK: c_uint = 0x00010000L;
pub const SMUSBI_ALERT__OE_MASK: c_uint = 0x00020000L;
pub const SMUSBI_ALERT__A_OVERRIDE_MASK: c_uint = 0x00040000L;
pub const SMUSBI_ALERT__A_MASK: c_uint = 0x00080000L;
pub const SMUSBI_ALERT__Y_MASK: c_uint = 0x80000000L;
// THM_TMON0_REMOTE_START
pub const THM_TMON0_REMOTE_START__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON0_REMOTE_START__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON0_REMOTE_END
pub const THM_TMON0_REMOTE_END__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON0_REMOTE_END__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON1_REMOTE_START
pub const THM_TMON1_REMOTE_START__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON1_REMOTE_START__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON1_REMOTE_END
pub const THM_TMON1_REMOTE_END__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON1_REMOTE_END__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON2_REMOTE_START
pub const THM_TMON2_REMOTE_START__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON2_REMOTE_START__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON2_REMOTE_END
pub const THM_TMON2_REMOTE_END__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON2_REMOTE_END__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON3_REMOTE_START
pub const THM_TMON3_REMOTE_START__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON3_REMOTE_START__DATA_MASK: c_uint = 0xFFFFFFFFL;
// THM_TMON3_REMOTE_END
pub const THM_TMON3_REMOTE_END__DATA__SHIFT: c_uint = 0x0;
pub const THM_TMON3_REMOTE_END__DATA_MASK: c_uint = 0xFFFFFFFFL;
