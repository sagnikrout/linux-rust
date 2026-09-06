//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/vega20_pptable.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

pub const ATOM_VEGA20_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const ATOM_VEGA20_PP_THERMALCONTROLLER_VEGA20: c_int = 26;
pub const ATOM_VEGA20_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x1;
pub const ATOM_VEGA20_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x2;
pub const ATOM_VEGA20_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x4;
pub const ATOM_VEGA20_PP_PLATFORM_CAP_BACO: c_uint = 0x8;
pub const ATOM_VEGA20_PP_PLATFORM_CAP_BAMACO: c_uint = 0x10;
pub const ATOM_VEGA20_PP_PLATFORM_CAP_ENABLESHADOWPSTATE: c_uint = 0x20;
pub const ATOM_VEGA20_TABLE_REVISION_VEGA20: c_int = 11;
pub const ATOM_VEGA20_ODFEATURE_MAX_COUNT: c_int = 32;
pub const ATOM_VEGA20_ODSETTING_MAX_COUNT: c_int = 32;
pub const ATOM_VEGA20_PPCLOCK_MAX_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATOM_VEGA20_ODFEATURE_ID {
    ATOM_VEGA20_ODFEATURE_GFXCLK_LIMITS = 0,
    ATOM_VEGA20_ODFEATURE_GFXCLK_CURVE,
    ATOM_VEGA20_ODFEATURE_UCLK_MAX,
    ATOM_VEGA20_ODFEATURE_POWER_LIMIT,
    ATOM_VEGA20_ODFEATURE_FAN_ACOUSTIC_LIMIT,    //FanMaximumRpm
    ATOM_VEGA20_ODFEATURE_FAN_SPEED_MIN,         //FanMinimumPwm
    ATOM_VEGA20_ODFEATURE_TEMPERATURE_FAN,       //FanTargetTemperature
    ATOM_VEGA20_ODFEATURE_TEMPERATURE_SYSTEM,    //MaxOpTemp
    ATOM_VEGA20_ODFEATURE_MEMORY_TIMING_TUNE,
    ATOM_VEGA20_ODFEATURE_FAN_ZERO_RPM_CONTROL,
    ATOM_VEGA20_ODFEATURE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATOM_VEGA20_ODSETTING_ID {
    ATOM_VEGA20_ODSETTING_GFXCLKFMAX = 0,
    ATOM_VEGA20_ODSETTING_GFXCLKFMIN,
    ATOM_VEGA20_ODSETTING_VDDGFXCURVEFREQ_P1,
    ATOM_VEGA20_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P1,
    ATOM_VEGA20_ODSETTING_VDDGFXCURVEFREQ_P2,
    ATOM_VEGA20_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P2,
    ATOM_VEGA20_ODSETTING_VDDGFXCURVEFREQ_P3,
    ATOM_VEGA20_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P3,
    ATOM_VEGA20_ODSETTING_UCLKFMAX,
    ATOM_VEGA20_ODSETTING_POWERPERCENTAGE,
    ATOM_VEGA20_ODSETTING_FANRPMMIN,
    ATOM_VEGA20_ODSETTING_FANRPMACOUSTICLIMIT,
    ATOM_VEGA20_ODSETTING_FANTARGETTEMPERATURE,
    ATOM_VEGA20_ODSETTING_OPERATINGTEMPMAX,
    ATOM_VEGA20_ODSETTING_COUNT,
}

pub type ATOM_VEGA20_ODSETTING_ID = ATOM_VEGA20_ODSETTING_ID;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATOM_VEGA20_PPCLOCK_ID {
    ATOM_VEGA20_PPCLOCK_GFXCLK = 0,
    ATOM_VEGA20_PPCLOCK_VCLK,
    ATOM_VEGA20_PPCLOCK_DCLK,
    ATOM_VEGA20_PPCLOCK_ECLK,
    ATOM_VEGA20_PPCLOCK_SOCCLK,
    ATOM_VEGA20_PPCLOCK_UCLK,
    ATOM_VEGA20_PPCLOCK_FCLK,
    ATOM_VEGA20_PPCLOCK_DCEFCLK,
    ATOM_VEGA20_PPCLOCK_DISPCLK,
    ATOM_VEGA20_PPCLOCK_PIXCLK,
    ATOM_VEGA20_PPCLOCK_PHYCLK,
    ATOM_VEGA20_PPCLOCK_COUNT,
}

pub type ATOM_VEGA20_PPCLOCK_ID = ATOM_VEGA20_PPCLOCK_ID;

