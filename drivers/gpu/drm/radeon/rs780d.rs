//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rs780d.h
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
// Copyright 2011 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

// RS780/RS880 PM
pub const FVTHROT_CNTRL_REG: c_uint = 0x3000;

pub const MINIMUM_CIP_SHIFT: c_int = 1;
pub const MINIMUM_CIP_MASK: c_uint = 0x1fffffe;

pub const REFRESH_RATE_DIVISOR_SHIFT: c_int = 25;

pub const FVTHROT_TARGET_REG: c_uint = 0x3004;

pub const TARGET_IDLE_COUNT_MASK: c_uint = 0xffffff;
pub const TARGET_IDLE_COUNT_SHIFT: c_int = 0;
pub const FVTHROT_CB1: c_uint = 0x3008;
pub const FVTHROT_CB2: c_uint = 0x300c;
pub const FVTHROT_CB3: c_uint = 0x3010;
pub const FVTHROT_CB4: c_uint = 0x3014;
pub const FVTHROT_UTC0: c_uint = 0x3018;
pub const FVTHROT_UTC1: c_uint = 0x301c;
pub const FVTHROT_UTC2: c_uint = 0x3020;
pub const FVTHROT_UTC3: c_uint = 0x3024;
pub const FVTHROT_UTC4: c_uint = 0x3028;
pub const FVTHROT_DTC0: c_uint = 0x302c;
pub const FVTHROT_DTC1: c_uint = 0x3030;
pub const FVTHROT_DTC2: c_uint = 0x3034;
pub const FVTHROT_DTC3: c_uint = 0x3038;
pub const FVTHROT_DTC4: c_uint = 0x303c;
pub const FVTHROT_FBDIV_REG0: c_uint = 0x3040;

pub const MIN_FEEDBACK_DIV_MASK: c_uint = 0xfff;
pub const MIN_FEEDBACK_DIV_SHIFT: c_int = 0;

pub const MAX_FEEDBACK_DIV_SHIFT: c_int = 12;
pub const FVTHROT_FBDIV_REG1: c_uint = 0x3044;

pub const MAX_FEEDBACK_STEP_MASK: c_uint = 0xfff;
pub const MAX_FEEDBACK_STEP_SHIFT: c_int = 0;

pub const STARTING_FEEDBACK_DIV_SHIFT: c_int = 12;

pub const FVTHROT_FBDIV_REG2: c_uint = 0x3048;

pub const FORCED_FEEDBACK_DIV_MASK: c_uint = 0xfff;
pub const FORCED_FEEDBACK_DIV_SHIFT: c_int = 0;

pub const FB_DIV_TIMER_VAL_SHIFT: c_int = 12;
pub const FVTHROT_FB_US_REG0: c_uint = 0x304c;
pub const FVTHROT_FB_US_REG1: c_uint = 0x3050;
pub const FVTHROT_FB_DS_REG0: c_uint = 0x3054;
pub const FVTHROT_FB_DS_REG1: c_uint = 0x3058;
pub const FVTHROT_PWM_CTRL_REG0: c_uint = 0x305c;

pub const STARTING_PWM_HIGHTIME_MASK: c_uint = 0xfff;
pub const STARTING_PWM_HIGHTIME_SHIFT: c_int = 0;

pub const NUMBER_OF_CYCLES_IN_PERIOD_SHIFT: c_int = 12;

pub const FVTHROT_PWM_CTRL_REG1: c_uint = 0x3060;

pub const MIN_PWM_HIGHTIME_MASK: c_uint = 0xfff;
pub const MIN_PWM_HIGHTIME_SHIFT: c_int = 0;

pub const MAX_PWM_HIGHTIME_SHIFT: c_int = 12;
pub const FVTHROT_PWM_US_REG0: c_uint = 0x3064;
pub const FVTHROT_PWM_US_REG1: c_uint = 0x3068;
pub const FVTHROT_PWM_DS_REG0: c_uint = 0x306c;
pub const FVTHROT_PWM_DS_REG1: c_uint = 0x3070;
pub const FVTHROT_STATUS_REG0: c_uint = 0x3074;
pub const CURRENT_FEEDBACK_DIV_MASK: c_uint = 0xfff;
pub const CURRENT_FEEDBACK_DIV_SHIFT: c_int = 0;
pub const FVTHROT_STATUS_REG1: c_uint = 0x3078;
pub const FVTHROT_STATUS_REG2: c_uint = 0x307c;
pub const CG_INTGFX_MISC: c_uint = 0x3080;

pub const FVTHROT_PWM_FEEDBACK_DIV_REG1: c_uint = 0x308c;

pub const RANGE0_PWM_FEEDBACK_DIV_MASK: c_uint = 0xfff;
pub const RANGE0_PWM_FEEDBACK_DIV_SHIFT: c_int = 0;

pub const FVTHROT_PWM_FEEDBACK_DIV_REG2: c_uint = 0x3090;

pub const RANGE1_PWM_FEEDBACK_DIV_MASK: c_uint = 0xfff;
pub const RANGE1_PWM_FEEDBACK_DIV_SHIFT: c_int = 0;

pub const RANGE2_PWM_FEEDBACK_DIV_SHIFT: c_int = 12;
pub const FVTHROT_PWM_FEEDBACK_DIV_REG3: c_uint = 0x3094;

pub const RANGE0_PWM_MASK: c_uint = 0xfff;
pub const RANGE0_PWM_SHIFT: c_int = 0;

pub const RANGE1_PWM_SHIFT: c_int = 12;
pub const FVTHROT_PWM_FEEDBACK_DIV_REG4: c_uint = 0x3098;

pub const RANGE2_PWM_MASK: c_uint = 0xfff;
pub const RANGE2_PWM_SHIFT: c_int = 0;

pub const RANGE3_PWM_SHIFT: c_int = 12;
pub const FVTHROT_SLOW_CLK_FEEDBACK_DIV_REG1: c_uint = 0x30ac;

pub const RANGE0_SLOW_CLK_FEEDBACK_DIV_MASK: c_uint = 0xfff;
pub const RANGE0_SLOW_CLK_FEEDBACK_DIV_SHIFT: c_int = 0;

pub const GFX_MACRO_BYPASS_CNTL: c_uint = 0x30c0;

