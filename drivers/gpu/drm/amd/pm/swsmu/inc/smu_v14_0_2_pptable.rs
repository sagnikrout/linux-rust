//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/smu_v14_0_2_pptable.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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

pub const SMU_14_0_2_TABLE_FORMAT_REVISION: c_int = 23;
pub const SMU_14_0_2_CUSTOM_TABLE_FORMAT_REVISION: c_int = 1;
// POWERPLAYTABLE::ulPlatformCaps
pub const SMU_14_0_2_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x1     // This cap indicates whether CCC need to show Powerplay page.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x2     // This cap indicates whether power source notificaiton is done by SBIOS instead of OS.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x4     // This cap indicates whether DC mode notificaiton is done by GPIO pin directly.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_BACO: c_uint = 0x8     // This cap indicates whether board supports the BACO circuitry.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_MACO: c_uint = 0x10    // This cap indicates whether board supports the MACO circuitry.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_SHADOWPSTATE: c_uint = 0x20    // This cap indicates whether board supports the Shadow Pstate.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_LEDSUPPORTED: c_uint = 0x40    // This cap indicates whether board supports the LED.;
pub const SMU_14_0_2_PP_PLATFORM_CAP_MOBILEOVERDRIVE: c_uint = 0x80    // This cap indicates whether board supports the Mobile Overdrive.;
// SMU_14_0_2_PP_THERMALCONTROLLER - Thermal Controller Type
pub const SMU_14_0_2_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const SMU_14_0_2_PP_OVERDRIVE_VERSION: c_uint = 0x1     // TODO: FIX OverDrive Version TBD;
pub const SMU_14_0_2_PP_CUSTOM_OVERDRIVE_VERSION: c_uint = 0x1;
pub const SMU_14_0_2_PP_POWERSAVINGCLOCK_VERSION: c_uint = 0x01    // Power Saving Clock Table Version 1.00;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu_14_0_3_pptable_source {
    PPTABLE_SOURCE_IFWI             = 0,
    PPTABLE_SOURCE_DRIVER_HARDCODED = 1,
    PPTABLE_SOURCE_PPGEN_REGISTRY   = 2,
    PPTABLE_SOURCE_MAX              = PPTABLE_SOURCE_PPGEN_REGISTRY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_14_0_2_CUSTOM_OD_SW_FEATURE_CAP {
    SMU_14_0_2_CUSTOM_ODCAP_POWER_MODE = 0,
    SMU_14_0_2_CUSTOM_ODCAP_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_14_0_2_CUSTOM_OD_FEATURE_SETTING_ID {
    SMU_14_0_2_CUSTOM_ODSETTING_POWER_MODE = 0,
    SMU_14_0_2_CUSTOM_ODSETTING_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_14_0_2_custom_overdrive_table {
    pub revision: u8,
    pub reserve: [u8; 3],
    pub cap: [u8; SMU_14_0_2_CUSTOM_ODCAP_COUNT],
    pub max: [i32; SMU_14_0_2_CUSTOM_ODSETTING_COUNT],
    pub min: [i32; SMU_14_0_2_CUSTOM_ODSETTING_COUNT],
    pub pm_setting: [i16; SMU_14_0_2_PMSETTING_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_14_0_3_custom_powerplay_table {
    pub custom_table_revision: u8,
    pub custom_table_size: u16,
    pub custom_sku_table_offset: u16,
    pub custom_platform_caps: u32,
    pub software_shutdown_temp: u16,
    pub custom_overdrive_table: smu_14_0_2_custom_overdrive_table,
    pub reserve: [u32; 8],
    pub custom_sku_table_pmfw: CustomSkuTable_t,
}

