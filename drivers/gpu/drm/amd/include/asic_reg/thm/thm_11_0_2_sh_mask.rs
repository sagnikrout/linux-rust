//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/thm/thm_11_0_2_sh_mask.h
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
// Copyright (C) 2018  Advanced Micro Devices, Inc.
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

// Macro flag: #define _thm_11_0_2_SH_MASK_HEADER
// CG_MULT_THERMAL_STATUS
pub const CG_MULT_THERMAL_STATUS__ASIC_MAX_TEMP__SHIFT: c_uint = 0x0;
pub const CG_MULT_THERMAL_STATUS__CTF_TEMP__SHIFT: c_uint = 0x9;
pub const CG_MULT_THERMAL_STATUS__ASIC_MAX_TEMP_MASK: c_uint = 0x000001FFL;
pub const CG_MULT_THERMAL_STATUS__CTF_TEMP_MASK: c_uint = 0x0003FE00L;
pub const CG_FDO_CTRL2__TMIN__SHIFT: c_uint = 0x0;
pub const CG_FDO_CTRL2__TMIN_MASK: c_uint = 0x000000FFL;
pub const CG_FDO_CTRL2__FDO_PWM_MODE__SHIFT: c_uint = 0xb;
pub const CG_FDO_CTRL2__FDO_PWM_MODE_MASK: c_uint = 0x00003800L;
pub const CG_FDO_CTRL1__FMAX_DUTY100__SHIFT: c_uint = 0x0;
pub const CG_FDO_CTRL1__FMAX_DUTY100_MASK: c_uint = 0x000000FFL;
pub const CG_FDO_CTRL0__FDO_STATIC_DUTY__SHIFT: c_uint = 0x0;
pub const CG_FDO_CTRL0__FDO_STATIC_DUTY_MASK: c_uint = 0x000000FFL;
pub const CG_TACH_CTRL__TARGET_PERIOD__SHIFT: c_uint = 0x3;
pub const CG_TACH_CTRL__TARGET_PERIOD_MASK: c_uint = 0xFFFFFFF8L;
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
pub const CG_THERMAL_STATUS__FDO_PWM_DUTY__SHIFT: c_uint = 0x9;
pub const CG_THERMAL_STATUS__FDO_PWM_DUTY_MASK: c_uint = 0x0001FE00L;
