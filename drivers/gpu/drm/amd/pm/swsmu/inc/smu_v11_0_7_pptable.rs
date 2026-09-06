//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/smu_v11_0_7_pptable.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

pub const SMU_11_0_7_TABLE_FORMAT_REVISION: c_int = 15;
// POWERPLAYTABLE::ulPlatformCaps
pub const SMU_11_0_7_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x1            // This cap indicates whether CCC need to show Powerplay page.;
pub const SMU_11_0_7_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x2            // This cap indicates whether power source notificaiton is done by SBIOS instead of OS.;
pub const SMU_11_0_7_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x4            // This cap indicates whether DC mode notificaiton is done by GPIO pin directly.;
pub const SMU_11_0_7_PP_PLATFORM_CAP_BACO: c_uint = 0x8            // This cap indicates whether board supports the BACO circuitry.;
pub const SMU_11_0_7_PP_PLATFORM_CAP_MACO: c_uint = 0x10           // This cap indicates whether board supports the MACO circuitry.;
pub const SMU_11_0_7_PP_PLATFORM_CAP_SHADOWPSTATE: c_uint = 0x20           // This cap indicates whether board supports the Shadow Pstate.;
// SMU_11_0_7_PP_THERMALCONTROLLER - Thermal Controller Type
pub const SMU_11_0_7_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const SMU_11_0_7_PP_THERMALCONTROLLER_SIENNA_CICHLID: c_int = 28;
pub const SMU_11_0_7_PP_OVERDRIVE_VERSION: c_uint = 0x81           // OverDrive 8 Table Version 0.2;
pub const SMU_11_0_7_PP_POWERSAVINGCLOCK_VERSION: c_uint = 0x01           // Power Saving Clock Table Version 1.00;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_7_ODFEATURE_CAP {
    SMU_11_0_7_ODCAP_GFXCLK_LIMITS = 0,
    SMU_11_0_7_ODCAP_GFXCLK_CURVE,
    SMU_11_0_7_ODCAP_UCLK_LIMITS,
    SMU_11_0_7_ODCAP_POWER_LIMIT,
    SMU_11_0_7_ODCAP_FAN_ACOUSTIC_LIMIT,
    SMU_11_0_7_ODCAP_FAN_SPEED_MIN,
    SMU_11_0_7_ODCAP_TEMPERATURE_FAN,
    SMU_11_0_7_ODCAP_TEMPERATURE_SYSTEM,
    SMU_11_0_7_ODCAP_MEMORY_TIMING_TUNE,
    SMU_11_0_7_ODCAP_FAN_ZERO_RPM_CONTROL,
    SMU_11_0_7_ODCAP_AUTO_UV_ENGINE,
    SMU_11_0_7_ODCAP_AUTO_OC_ENGINE,
    SMU_11_0_7_ODCAP_AUTO_OC_MEMORY,
    SMU_11_0_7_ODCAP_FAN_CURVE,
    SMU_11_0_ODCAP_AUTO_FAN_ACOUSTIC_LIMIT,
    SMU_11_0_7_ODCAP_POWER_MODE,
    SMU_11_0_7_ODCAP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_7_ODFEATURE_ID {
    SMU_11_0_7_ODFEATURE_GFXCLK_LIMITS         = 1 << SMU_11_0_7_ODCAP_GFXCLK_LIMITS,            //GFXCLK Limit feature
    SMU_11_0_7_ODFEATURE_GFXCLK_CURVE          = 1 << SMU_11_0_7_ODCAP_GFXCLK_CURVE,             //GFXCLK Curve feature
    SMU_11_0_7_ODFEATURE_UCLK_LIMITS           = 1 << SMU_11_0_7_ODCAP_UCLK_LIMITS,              //UCLK Limit feature
    SMU_11_0_7_ODFEATURE_POWER_LIMIT           = 1 << SMU_11_0_7_ODCAP_POWER_LIMIT,              //Power Limit feature
    SMU_11_0_7_ODFEATURE_FAN_ACOUSTIC_LIMIT    = 1 << SMU_11_0_7_ODCAP_FAN_ACOUSTIC_LIMIT,       //Fan Acoustic RPM feature
    SMU_11_0_7_ODFEATURE_FAN_SPEED_MIN         = 1 << SMU_11_0_7_ODCAP_FAN_SPEED_MIN,            //Minimum Fan Speed feature
    SMU_11_0_7_ODFEATURE_TEMPERATURE_FAN       = 1 << SMU_11_0_7_ODCAP_TEMPERATURE_FAN,          //Fan Target Temperature Limit feature
    SMU_11_0_7_ODFEATURE_TEMPERATURE_SYSTEM    = 1 << SMU_11_0_7_ODCAP_TEMPERATURE_SYSTEM,       //Operating Temperature Limit feature
    SMU_11_0_7_ODFEATURE_MEMORY_TIMING_TUNE    = 1 << SMU_11_0_7_ODCAP_MEMORY_TIMING_TUNE,       //AC Timing Tuning feature
    SMU_11_0_7_ODFEATURE_FAN_ZERO_RPM_CONTROL  = 1 << SMU_11_0_7_ODCAP_FAN_ZERO_RPM_CONTROL,     //Zero RPM feature
    SMU_11_0_7_ODFEATURE_AUTO_UV_ENGINE        = 1 << SMU_11_0_7_ODCAP_AUTO_UV_ENGINE,           //Auto Under Volt GFXCLK feature
    SMU_11_0_7_ODFEATURE_AUTO_OC_ENGINE        = 1 << SMU_11_0_7_ODCAP_AUTO_OC_ENGINE,           //Auto Over Clock GFXCLK feature
    SMU_11_0_7_ODFEATURE_AUTO_OC_MEMORY        = 1 << SMU_11_0_7_ODCAP_AUTO_OC_MEMORY,           //Auto Over Clock MCLK feature
    SMU_11_0_7_ODFEATURE_FAN_CURVE             = 1 << SMU_11_0_7_ODCAP_FAN_CURVE,                //Fan Curve feature
    SMU_11_0_ODFEATURE_AUTO_FAN_ACOUSTIC_LIMIT = 1 << SMU_11_0_ODCAP_AUTO_FAN_ACOUSTIC_LIMIT,  //Auto Fan Acoustic RPM feature
    SMU_11_0_7_ODFEATURE_POWER_MODE            = 1 << SMU_11_0_7_ODCAP_POWER_MODE,               //Optimized GPU Power Mode feature
    SMU_11_0_7_ODFEATURE_COUNT                 = 16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_7_ODSETTING_ID {
    SMU_11_0_7_ODSETTING_GFXCLKFMAX = 0,
    SMU_11_0_7_ODSETTING_GFXCLKFMIN,
    SMU_11_0_7_ODSETTING_CUSTOM_GFX_VF_CURVE_A,
    SMU_11_0_7_ODSETTING_CUSTOM_GFX_VF_CURVE_B,
    SMU_11_0_7_ODSETTING_CUSTOM_GFX_VF_CURVE_C,
    SMU_11_0_7_ODSETTING_CUSTOM_CURVE_VFT_FMIN,
    SMU_11_0_7_ODSETTING_UCLKFMIN,
    SMU_11_0_7_ODSETTING_UCLKFMAX,
    SMU_11_0_7_ODSETTING_POWERPERCENTAGE,
    SMU_11_0_7_ODSETTING_FANRPMMIN,
    SMU_11_0_7_ODSETTING_FANRPMACOUSTICLIMIT,
    SMU_11_0_7_ODSETTING_FANTARGETTEMPERATURE,
    SMU_11_0_7_ODSETTING_OPERATINGTEMPMAX,
    SMU_11_0_7_ODSETTING_ACTIMING,
    SMU_11_0_7_ODSETTING_FAN_ZERO_RPM_CONTROL,
    SMU_11_0_7_ODSETTING_AUTOUVENGINE,
    SMU_11_0_7_ODSETTING_AUTOOCENGINE,
    SMU_11_0_7_ODSETTING_AUTOOCMEMORY,
    SMU_11_0_7_ODSETTING_FAN_CURVE_TEMPERATURE_1,
    SMU_11_0_7_ODSETTING_FAN_CURVE_SPEED_1,
    SMU_11_0_7_ODSETTING_FAN_CURVE_TEMPERATURE_2,
    SMU_11_0_7_ODSETTING_FAN_CURVE_SPEED_2,
    SMU_11_0_7_ODSETTING_FAN_CURVE_TEMPERATURE_3,
    SMU_11_0_7_ODSETTING_FAN_CURVE_SPEED_3,
    SMU_11_0_7_ODSETTING_FAN_CURVE_TEMPERATURE_4,
    SMU_11_0_7_ODSETTING_FAN_CURVE_SPEED_4,
    SMU_11_0_7_ODSETTING_FAN_CURVE_TEMPERATURE_5,
    SMU_11_0_7_ODSETTING_FAN_CURVE_SPEED_5,
    SMU_11_0_7_ODSETTING_AUTO_FAN_ACOUSTIC_LIMIT,
    SMU_11_0_7_ODSETTING_POWER_MODE,
    SMU_11_0_7_ODSETTING_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_7_PWRMODE_SETTING {
    SMU_11_0_7_PMSETTING_POWER_LIMIT_QUIET = 0,
    SMU_11_0_7_PMSETTING_POWER_LIMIT_BALANCE,
    SMU_11_0_7_PMSETTING_POWER_LIMIT_TURBO,
    SMU_11_0_7_PMSETTING_POWER_LIMIT_RAGE,
    SMU_11_0_7_PMSETTING_ACOUSTIC_TEMP_QUIET,
    SMU_11_0_7_PMSETTING_ACOUSTIC_TEMP_BALANCE,
    SMU_11_0_7_PMSETTING_ACOUSTIC_TEMP_TURBO,
    SMU_11_0_7_PMSETTING_ACOUSTIC_TEMP_RAGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_7_PPCLOCK_ID {
    SMU_11_0_7_PPCLOCK_GFXCLK = 0,
    SMU_11_0_7_PPCLOCK_SOCCLK,
    SMU_11_0_7_PPCLOCK_UCLK,
    SMU_11_0_7_PPCLOCK_FCLK,
    SMU_11_0_7_PPCLOCK_DCLK_0,
    SMU_11_0_7_PPCLOCK_VCLK_0,
    SMU_11_0_7_PPCLOCK_DCLK_1,
    SMU_11_0_7_PPCLOCK_VCLK_1,
    SMU_11_0_7_PPCLOCK_DCEFCLK,
    SMU_11_0_7_PPCLOCK_DISPCLK,
    SMU_11_0_7_PPCLOCK_PIXCLK,
    SMU_11_0_7_PPCLOCK_PHYCLK,
    SMU_11_0_7_PPCLOCK_DTBCLK,
    SMU_11_0_7_PPCLOCK_COUNT,
}

