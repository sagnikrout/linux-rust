//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smumgr.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_TABLE {
    SMU_UVD_TABLE = 0,
    SMU_VCE_TABLE,
    SMU_BIF_TABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_TYPE {
    SMU_SoftRegisters = 0,
    SMU_Discrete_DpmTable,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_MEMBER {
    HandshakeDisables = 0,
    VoltageChangeTimeout,
    AverageGraphicsActivity,
    AverageMemoryActivity,
    PreVBlankGap,
    VBlankTimeout,
    UcodeLoadStatus,
    UvdBootLevel,
    VceBootLevel,
    LowSclkInterruptThreshold,
    DRAM_LOG_ADDR_H,
    DRAM_LOG_ADDR_L,
    DRAM_LOG_PHY_ADDR_H,
    DRAM_LOG_PHY_ADDR_L,
    DRAM_LOG_BUFF_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU_MAC_DEFINITION {
    SMU_MAX_LEVELS_GRAPHICS = 0,
    SMU_MAX_LEVELS_MEMORY,
    SMU_MAX_LEVELS_LINK,
    SMU_MAX_ENTRIES_SMIO,
    SMU_MAX_LEVELS_VDDC,
    SMU_MAX_LEVELS_VDDGFX,
    SMU_MAX_LEVELS_VDDCI,
    SMU_MAX_LEVELS_MVDD,
    SMU_UVD_MCLK_HANDSHAKE_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU9_TABLE_ID {
    PPTABLE = 0,
    WMTABLE,
    AVFSTABLE,
    TOOLSTABLE,
    AVFSFUSETABLE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMU10_TABLE_ID {
    SMU10_WMTABLE = 0,
    SMU10_CLOCKTABLE,
}

extern "C" {
    pub fn smum_download_powerplay_table(hwmgr: *mut pp_hwmgr, table: *mut c_void) -> c_int;
}
extern "C" {
    pub fn smum_upload_powerplay_table(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_send_msg_to_smc(hwmgr: *mut pp_hwmgr, msg: u16, resp: *mut u32) -> c_int;
}
extern "C" {
    pub fn smum_update_sclk_threshold(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_update_smc_table(hwmgr: *mut pp_hwmgr, type: u32) -> c_int;
}
extern "C" {
    pub fn smum_process_firmware_header(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_thermal_avfs_enable(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_thermal_setup_fan_table(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_init_smc_table(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_populate_all_graphic_levels(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_populate_all_memory_levels(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_initialize_mc_reg_table(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smum_get_mac_definition(hwmgr: *mut pp_hwmgr, value: u32) -> u32;
}
extern "C" {
    pub fn smum_is_dpm_running(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn smum_is_hw_avfs_present(hwmgr: *mut pp_hwmgr) -> bool;
}
extern "C" {
    pub fn smum_update_dpm_settings(hwmgr: *mut pp_hwmgr, profile_setting: *mut c_void) -> c_int;
}
extern "C" {
    pub fn smum_smc_table_manager(hwmgr: *mut pp_hwmgr, table: *mut u8, table_id: u16, rw: bool) -> c_int;
}
extern "C" {
    pub fn smum_stop_smc(hwmgr: *mut pp_hwmgr) -> c_int;
}
