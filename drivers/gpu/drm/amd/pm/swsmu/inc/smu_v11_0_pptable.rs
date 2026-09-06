//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/smu_v11_0_pptable.h
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

pub const SMU_11_0_TABLE_FORMAT_REVISION: c_int = 12;
// POWERPLAYTABLE::ulPlatformCaps
pub const SMU_11_0_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x1;
pub const SMU_11_0_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x2;
pub const SMU_11_0_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x4;
pub const SMU_11_0_PP_PLATFORM_CAP_BACO: c_uint = 0x8;
pub const SMU_11_0_PP_PLATFORM_CAP_MACO: c_uint = 0x10;
pub const SMU_11_0_PP_PLATFORM_CAP_SHADOWPSTATE: c_uint = 0x20;
// SMU_11_0_PP_THERMALCONTROLLER - Thermal Controller Type
pub const SMU_11_0_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const SMU_11_0_PP_OVERDRIVE_VERSION: c_uint = 0x0800;
pub const SMU_11_0_PP_POWERSAVINGCLOCK_VERSION: c_uint = 0x0100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_ODFEATURE_CAP {
    SMU_11_0_ODCAP_GFXCLK_LIMITS = 0,
    SMU_11_0_ODCAP_GFXCLK_CURVE,
    SMU_11_0_ODCAP_UCLK_MAX,
    SMU_11_0_ODCAP_POWER_LIMIT,
    SMU_11_0_ODCAP_FAN_ACOUSTIC_LIMIT,
    SMU_11_0_ODCAP_FAN_SPEED_MIN,
    SMU_11_0_ODCAP_TEMPERATURE_FAN,
    SMU_11_0_ODCAP_TEMPERATURE_SYSTEM,
    SMU_11_0_ODCAP_MEMORY_TIMING_TUNE,
    SMU_11_0_ODCAP_FAN_ZERO_RPM_CONTROL,
    SMU_11_0_ODCAP_AUTO_UV_ENGINE,
    SMU_11_0_ODCAP_AUTO_OC_ENGINE,
    SMU_11_0_ODCAP_AUTO_OC_MEMORY,
    SMU_11_0_ODCAP_FAN_CURVE,
    SMU_11_0_ODCAP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_ODFEATURE_ID {
    SMU_11_0_ODFEATURE_GFXCLK_LIMITS        = 1 << SMU_11_0_ODCAP_GFXCLK_LIMITS,            //GFXCLK Limit feature
    SMU_11_0_ODFEATURE_GFXCLK_CURVE         = 1 << SMU_11_0_ODCAP_GFXCLK_CURVE,             //GFXCLK Curve feature
    SMU_11_0_ODFEATURE_UCLK_MAX             = 1 << SMU_11_0_ODCAP_UCLK_MAX,                 //UCLK Limit feature
    SMU_11_0_ODFEATURE_POWER_LIMIT          = 1 << SMU_11_0_ODCAP_POWER_LIMIT,              //Power Limit feature
    SMU_11_0_ODFEATURE_FAN_ACOUSTIC_LIMIT   = 1 << SMU_11_0_ODCAP_FAN_ACOUSTIC_LIMIT,       //Fan Acoustic RPM feature
    SMU_11_0_ODFEATURE_FAN_SPEED_MIN        = 1 << SMU_11_0_ODCAP_FAN_SPEED_MIN,            //Minimum Fan Speed feature
    SMU_11_0_ODFEATURE_TEMPERATURE_FAN      = 1 << SMU_11_0_ODCAP_TEMPERATURE_FAN,          //Fan Target Temperature Limit feature
    SMU_11_0_ODFEATURE_TEMPERATURE_SYSTEM   = 1 << SMU_11_0_ODCAP_TEMPERATURE_SYSTEM,       //Operating Temperature Limit feature
    SMU_11_0_ODFEATURE_MEMORY_TIMING_TUNE   = 1 << SMU_11_0_ODCAP_MEMORY_TIMING_TUNE,       //AC Timing Tuning feature
    SMU_11_0_ODFEATURE_FAN_ZERO_RPM_CONTROL = 1 << SMU_11_0_ODCAP_FAN_ZERO_RPM_CONTROL,     //Zero RPM feature
    SMU_11_0_ODFEATURE_AUTO_UV_ENGINE       = 1 << SMU_11_0_ODCAP_AUTO_UV_ENGINE,           //Auto Under Volt GFXCLK feature
    SMU_11_0_ODFEATURE_AUTO_OC_ENGINE       = 1 << SMU_11_0_ODCAP_AUTO_OC_ENGINE,           //Auto Over Clock GFXCLK feature
    SMU_11_0_ODFEATURE_AUTO_OC_MEMORY       = 1 << SMU_11_0_ODCAP_AUTO_OC_MEMORY,           //Auto Over Clock MCLK feature
    SMU_11_0_ODFEATURE_FAN_CURVE            = 1 << SMU_11_0_ODCAP_FAN_CURVE,                //Fan Curve feature
    SMU_11_0_ODFEATURE_COUNT                = 14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_ODSETTING_ID {
    SMU_11_0_ODSETTING_GFXCLKFMAX = 0,
    SMU_11_0_ODSETTING_GFXCLKFMIN,
    SMU_11_0_ODSETTING_VDDGFXCURVEFREQ_P1,
    SMU_11_0_ODSETTING_VDDGFXCURVEVOLTAGE_P1,
    SMU_11_0_ODSETTING_VDDGFXCURVEFREQ_P2,
    SMU_11_0_ODSETTING_VDDGFXCURVEVOLTAGE_P2,
    SMU_11_0_ODSETTING_VDDGFXCURVEFREQ_P3,
    SMU_11_0_ODSETTING_VDDGFXCURVEVOLTAGE_P3,
    SMU_11_0_ODSETTING_UCLKFMAX,
    SMU_11_0_ODSETTING_POWERPERCENTAGE,
    SMU_11_0_ODSETTING_FANRPMMIN,
    SMU_11_0_ODSETTING_FANRPMACOUSTICLIMIT,
    SMU_11_0_ODSETTING_FANTARGETTEMPERATURE,
    SMU_11_0_ODSETTING_OPERATINGTEMPMAX,
    SMU_11_0_ODSETTING_ACTIMING,
    SMU_11_0_ODSETTING_FAN_ZERO_RPM_CONTROL,
    SMU_11_0_ODSETTING_AUTOUVENGINE,
    SMU_11_0_ODSETTING_AUTOOCENGINE,
    SMU_11_0_ODSETTING_AUTOOCMEMORY,
    SMU_11_0_ODSETTING_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_11_0_overdrive_table {
    pub SMU_11_0_PP_OVERDRIVE_VERSION: uint8_t revision; //Revision =,
    pub use: uint8_t reserve[3]; //Zero filled field reserved for future,
    pub features: uint32_t feature_count; //Total number of supported,
    pub settings: uint32_t setting_count; //Total number of supported,
    pub flags: uint8_t cap[SMU_11_0_MAX_ODFEATURE]; //OD feature support,
    pub settings: uint32_t max[SMU_11_0_MAX_ODSETTING]; //default maximum,
    pub settings: uint32_t min[SMU_11_0_MAX_ODSETTING]; //default minimum,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_11_0_PPCLOCK_ID {
    SMU_11_0_PPCLOCK_GFXCLK = 0,
    SMU_11_0_PPCLOCK_VCLK,
    SMU_11_0_PPCLOCK_DCLK,
    SMU_11_0_PPCLOCK_ECLK,
    SMU_11_0_PPCLOCK_SOCCLK,
    SMU_11_0_PPCLOCK_UCLK,
    SMU_11_0_PPCLOCK_DCEFCLK,
    SMU_11_0_PPCLOCK_DISPCLK,
    SMU_11_0_PPCLOCK_PIXCLK,
    SMU_11_0_PPCLOCK_PHYCLK,
    SMU_11_0_PPCLOCK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_11_0_power_saving_clock_table {
    pub SMU_11_0_PP_POWERSAVINGCLOCK_VERSION: uint8_t revision; //Revision =,
    pub use: uint8_t reserve[3]; //Zero filled field reserved for future,
    pub SMU_11_0_PPCLOCK_COUNT: uint32_t count; //power_saving_clock_count =,
    pub MHz: uint32_t max[SMU_11_0_MAX_PPCLOCK]; //PowerSavingClock Mode Clock Maximum array In,
    pub MHz: uint32_t min[SMU_11_0_MAX_PPCLOCK]; //PowerSavingClock Mode Clock Minimum array In,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_11_0_powerplay_table {
    pub header: atom_common_table_header,
    pub table_revision: u8,
    pub size: uint16_t table_size; //Driver portion table size. The offset to smc_pptable including header,
    pub golden_pp_id: u32,
    pub golden_revision: u32,
    pub format_id: u16,
    pub //POWERPLAYABLE::ulPlatformCaps: uint32_t platform_caps;,
    pub SMU_11_0_PP_THERMALCONTROLLER: uint8_t thermal_controller_type; //one of,
    pub small_power_limit1: u16,
    pub small_power_limit2: u16,
    pub boost_power_limit: u16,
    pub Tuning.: uint16_t od_turbo_power_limit; //Power limit setting for Turbo mode in Performance UI,
    pub Tuning.: uint16_t od_power_save_power_limit; //Power limit setting for PowerSave/Optimal mode in Performance UI,
    pub software_shutdown_temp: u16,
    pub use: uint16_t reserve[6]; //Zero filled field reserved for future,
    pub power_saving_clock: smu_11_0_power_saving_clock_table,
    pub overdrive_table: smu_11_0_overdrive_table,

    pub smu11_driver_if.h: PPTable_t smc_pptable; //PPTable_t in,

}

