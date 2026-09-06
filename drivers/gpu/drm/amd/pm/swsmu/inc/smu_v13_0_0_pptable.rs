//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/smu_v13_0_0_pptable.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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

pub const SMU_13_0_0_TABLE_FORMAT_REVISION: c_int = 15;
// POWERPLAYTABLE::ulPlatformCaps
pub const SMU_13_0_0_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x1        // This cap indicates whether CCC need to show Powerplay page.;
pub const SMU_13_0_0_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x2 // This cap indicates whether power source notificaiton is done by SBIOS instead of OS.;
pub const SMU_13_0_0_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x4       // This cap indicates whether DC mode notificaiton is done by GPIO pin directly.;
pub const SMU_13_0_0_PP_PLATFORM_CAP_BACO: c_uint = 0x8             // This cap indicates whether board supports the BACO circuitry.;
pub const SMU_13_0_0_PP_PLATFORM_CAP_MACO: c_uint = 0x10            // This cap indicates whether board supports the MACO circuitry.;
pub const SMU_13_0_0_PP_PLATFORM_CAP_SHADOWPSTATE: c_uint = 0x20    // This cap indicates whether board supports the Shadow Pstate.;
// SMU_13_0_0_PP_THERMALCONTROLLER - Thermal Controller Type
pub const SMU_13_0_0_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const SMU_13_0_0_PP_THERMALCONTROLLER_NAVI21: c_int = 28;
pub const SMU_13_0_0_PP_OVERDRIVE_VERSION: c_uint = 0x83        // OverDrive 8 Table Version 0.2;
pub const SMU_13_0_0_PP_POWERSAVINGCLOCK_VERSION: c_uint = 0x01 // Power Saving Clock Table Version 1.00;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_13_0_0_ODFEATURE_CAP {
    SMU_13_0_0_ODCAP_GFXCLK_LIMITS = 0,
    SMU_13_0_0_ODCAP_UCLK_LIMITS,
    SMU_13_0_0_ODCAP_POWER_LIMIT,
    SMU_13_0_0_ODCAP_FAN_ACOUSTIC_LIMIT,
    SMU_13_0_0_ODCAP_FAN_SPEED_MIN,
    SMU_13_0_0_ODCAP_TEMPERATURE_FAN,
    SMU_13_0_0_ODCAP_TEMPERATURE_SYSTEM,
    SMU_13_0_0_ODCAP_MEMORY_TIMING_TUNE,
    SMU_13_0_0_ODCAP_FAN_ZERO_RPM_CONTROL,
    SMU_13_0_0_ODCAP_AUTO_UV_ENGINE,
    SMU_13_0_0_ODCAP_AUTO_OC_ENGINE,
    SMU_13_0_0_ODCAP_AUTO_OC_MEMORY,
    SMU_13_0_0_ODCAP_FAN_CURVE,
    SMU_13_0_0_ODCAP_AUTO_FAN_ACOUSTIC_LIMIT,
    SMU_13_0_0_ODCAP_POWER_MODE,
    SMU_13_0_0_ODCAP_PER_ZONE_GFX_VOLTAGE_OFFSET,
    SMU_13_0_0_ODCAP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_13_0_0_ODFEATURE_ID {
    SMU_13_0_0_ODFEATURE_GFXCLK_LIMITS           = 1 << SMU_13_0_0_ODCAP_GFXCLK_LIMITS,           //GFXCLK Limit feature
    SMU_13_0_0_ODFEATURE_UCLK_LIMITS             = 1 << SMU_13_0_0_ODCAP_UCLK_LIMITS,             //UCLK Limit feature
    SMU_13_0_0_ODFEATURE_POWER_LIMIT             = 1 << SMU_13_0_0_ODCAP_POWER_LIMIT,             //Power Limit feature
    SMU_13_0_0_ODFEATURE_FAN_ACOUSTIC_LIMIT      = 1 << SMU_13_0_0_ODCAP_FAN_ACOUSTIC_LIMIT,      //Fan Acoustic RPM feature
    SMU_13_0_0_ODFEATURE_FAN_SPEED_MIN           = 1 << SMU_13_0_0_ODCAP_FAN_SPEED_MIN,           //Minimum Fan Speed feature
    SMU_13_0_0_ODFEATURE_TEMPERATURE_FAN         = 1 << SMU_13_0_0_ODCAP_TEMPERATURE_FAN,         //Fan Target Temperature Limit feature
    SMU_13_0_0_ODFEATURE_TEMPERATURE_SYSTEM      = 1 << SMU_13_0_0_ODCAP_TEMPERATURE_SYSTEM,      //Operating Temperature Limit feature
    SMU_13_0_0_ODFEATURE_MEMORY_TIMING_TUNE      = 1 << SMU_13_0_0_ODCAP_MEMORY_TIMING_TUNE,      //AC Timing Tuning feature
    SMU_13_0_0_ODFEATURE_FAN_ZERO_RPM_CONTROL    = 1 << SMU_13_0_0_ODCAP_FAN_ZERO_RPM_CONTROL,    //Zero RPM feature
    SMU_13_0_0_ODFEATURE_AUTO_UV_ENGINE          = 1 << SMU_13_0_0_ODCAP_AUTO_UV_ENGINE,          //Auto Under Volt GFXCLK feature
    SMU_13_0_0_ODFEATURE_AUTO_OC_ENGINE          = 1 << SMU_13_0_0_ODCAP_AUTO_OC_ENGINE,          //Auto Over Clock GFXCLK feature
    SMU_13_0_0_ODFEATURE_AUTO_OC_MEMORY          = 1 << SMU_13_0_0_ODCAP_AUTO_OC_MEMORY,          //Auto Over Clock MCLK feature
    SMU_13_0_0_ODFEATURE_FAN_CURVE               = 1 << SMU_13_0_0_ODCAP_FAN_CURVE,               //Fan Curve feature
    SMU_13_0_0_ODFEATURE_AUTO_FAN_ACOUSTIC_LIMIT = 1 << SMU_13_0_0_ODCAP_AUTO_FAN_ACOUSTIC_LIMIT, //Auto Fan Acoustic RPM feature
    SMU_13_0_0_ODFEATURE_POWER_MODE              = 1 << SMU_13_0_0_ODCAP_POWER_MODE,              //Optimized GPU Power Mode feature
    SMU_13_0_0_ODFEATURE_PER_ZONE_GFX_VOLTAGE_OFFSET  = 1 << SMU_13_0_0_ODCAP_PER_ZONE_GFX_VOLTAGE_OFFSET,  //Perzone voltage offset feature
    SMU_13_0_0_ODFEATURE_COUNT                   = 16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_13_0_0_ODSETTING_ID {
    SMU_13_0_0_ODSETTING_GFXCLKFMAX = 0,
    SMU_13_0_0_ODSETTING_GFXCLKFMIN,
    SMU_13_0_0_ODSETTING_UCLKFMIN,
    SMU_13_0_0_ODSETTING_UCLKFMAX,
    SMU_13_0_0_ODSETTING_POWERPERCENTAGE,
    SMU_13_0_0_ODSETTING_FANRPMMIN,
    SMU_13_0_0_ODSETTING_FANRPMACOUSTICLIMIT,
    SMU_13_0_0_ODSETTING_FANTARGETTEMPERATURE,
    SMU_13_0_0_ODSETTING_OPERATINGTEMPMAX,
    SMU_13_0_0_ODSETTING_ACTIMING,
    SMU_13_0_0_ODSETTING_FAN_ZERO_RPM_CONTROL,
    SMU_13_0_0_ODSETTING_AUTOUVENGINE,
    SMU_13_0_0_ODSETTING_AUTOOCENGINE,
    SMU_13_0_0_ODSETTING_AUTOOCMEMORY,
    SMU_13_0_0_ODSETTING_FAN_CURVE_TEMPERATURE_1,
    SMU_13_0_0_ODSETTING_FAN_CURVE_SPEED_1,
    SMU_13_0_0_ODSETTING_FAN_CURVE_TEMPERATURE_2,
    SMU_13_0_0_ODSETTING_FAN_CURVE_SPEED_2,
    SMU_13_0_0_ODSETTING_FAN_CURVE_TEMPERATURE_3,
    SMU_13_0_0_ODSETTING_FAN_CURVE_SPEED_3,
    SMU_13_0_0_ODSETTING_FAN_CURVE_TEMPERATURE_4,
    SMU_13_0_0_ODSETTING_FAN_CURVE_SPEED_4,
    SMU_13_0_0_ODSETTING_FAN_CURVE_TEMPERATURE_5,
    SMU_13_0_0_ODSETTING_FAN_CURVE_SPEED_5,
    SMU_13_0_0_ODSETTING_AUTO_FAN_ACOUSTIC_LIMIT,
    SMU_13_0_0_ODSETTING_POWER_MODE,
    SMU_13_0_0_ODSETTING_PER_ZONE_GFX_VOLTAGE_OFFSET_POINT_1,
    SMU_13_0_0_ODSETTING_PER_ZONE_GFX_VOLTAGE_OFFSET_POINT_2,
    SMU_13_0_0_ODSETTING_PER_ZONE_GFX_VOLTAGE_OFFSET_POINT_3,
    SMU_13_0_0_ODSETTING_PER_ZONE_GFX_VOLTAGE_OFFSET_POINT_4,
    SMU_13_0_0_ODSETTING_PER_ZONE_GFX_VOLTAGE_OFFSET_POINT_5,
    SMU_13_0_0_ODSETTING_PER_ZONE_GFX_VOLTAGE_OFFSET_POINT_6,
    SMU_13_0_0_ODSETTING_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_13_0_0_PWRMODE_SETTING {
    SMU_13_0_0_PMSETTING_POWER_LIMIT_QUIET = 0,
    SMU_13_0_0_PMSETTING_POWER_LIMIT_BALANCE,
    SMU_13_0_0_PMSETTING_POWER_LIMIT_TURBO,
    SMU_13_0_0_PMSETTING_POWER_LIMIT_RAGE,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TEMP_QUIET,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TEMP_BALANCE,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TEMP_TURBO,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TEMP_RAGE,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TARGET_RPM_QUIET,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TARGET_RPM_BALANCE,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TARGET_RPM_TURBO,
    SMU_13_0_0_PMSETTING_ACOUSTIC_TARGET_RPM_RAGE,
    SMU_13_0_0_PMSETTING_ACOUSTIC_LIMIT_RPM_QUIET,
    SMU_13_0_0_PMSETTING_ACOUSTIC_LIMIT_RPM_BALANCE,
    SMU_13_0_0_PMSETTING_ACOUSTIC_LIMIT_RPM_TURBO,
    SMU_13_0_0_PMSETTING_ACOUSTIC_LIMIT_RPM_RAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_13_0_0_overdrive_table {
    pub SMU_13_0_0_PP_OVERDRIVE_VERSION: uint8_t revision; //Revision =,
    pub use: uint8_t reserve[3]; //Zero filled field reserved for future,
    pub features: uint32_t feature_count; //Total number of supported,
    pub settings: uint32_t setting_count; //Total number of supported,
    pub flags: uint8_t cap[SMU_13_0_0_MAX_ODFEATURE]; //OD feature support,
    pub settings: uint32_t max[SMU_13_0_0_MAX_ODSETTING]; //default maximum,
    pub settings: uint32_t min[SMU_13_0_0_MAX_ODSETTING]; //default minimum,
    pub settings: int16_t pm_setting[SMU_13_0_0_MAX_PMSETTING]; //Optimized power mode feature,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_13_0_0_PPCLOCK_ID {
    SMU_13_0_0_PPCLOCK_GFXCLK = 0,
    SMU_13_0_0_PPCLOCK_SOCCLK,
    SMU_13_0_0_PPCLOCK_UCLK,
    SMU_13_0_0_PPCLOCK_FCLK,
    SMU_13_0_0_PPCLOCK_DCLK_0,
    SMU_13_0_0_PPCLOCK_VCLK_0,
    SMU_13_0_0_PPCLOCK_DCLK_1,
    SMU_13_0_0_PPCLOCK_VCLK_1,
    SMU_13_0_0_PPCLOCK_DCEFCLK,
    SMU_13_0_0_PPCLOCK_DISPCLK,
    SMU_13_0_0_PPCLOCK_PIXCLK,
    SMU_13_0_0_PPCLOCK_PHYCLK,
    SMU_13_0_0_PPCLOCK_DTBCLK,
    SMU_13_0_0_PPCLOCK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_13_0_0_powerplay_table {
    pub 0: atom_common_table_header header; //For SMU13, header.format_revision = 15, header.content_revision =,
    pub 2: uint8_t table_revision; //For SMU13, table_revision =,
    pub padding: u8,
    pub size: uint16_t table_size; //Driver portion table size. The offset to smc_pptable including header,
    pub Base: uint32_t golden_pp_id; //PPGen use only: PP Table ID on the Golden Data,
    pub Base: uint32_t golden_revision; //PPGen use only: PP Table Revision on the Golden Data,
    pub 0x80: uint16_t format_id; //PPGen use only: PPTable for different ASICs. For SMU13 this should be,
    pub //POWERPLAYABLE::ulPlatformCaps: uint32_t platform_caps;,
    pub SMU_13_0_0_PP_THERMALCONTROLLER: uint8_t thermal_controller_type; //one of,
    pub small_power_limit1: u16,
    pub small_power_limit2: u16,
    pub limit.: uint16_t boost_power_limit; //For Gemini Board, when the slave adapter is in BACO mode, the master adapter will use this boost power limit instead of the default power limit to boost the power,
    pub software_shutdown_temp: u16,
    pub reserve: [u32; 45],
    pub overdrive_table: smu_13_0_0_overdrive_table,
    pub padding1: u8,
    pub driver_if.h: PPTable_t smc_pptable; //PPTable_t in,
}

