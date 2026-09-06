//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/kv_dpm.h
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
// Copyright 2013 Advanced Micro Devices, Inc.
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
pub const SMU__NUM_SCLK_DPM_STATE: c_int = 8;
pub const SMU__NUM_MCLK_DPM_LEVELS: c_int = 4;
pub const SMU__NUM_LCLK_DPM_LEVELS: c_int = 8;

pub const KV_NUM_NBPSTATES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kv_pt_config_reg_type {
    KV_CONFIGREG_MMR = 0,
    KV_CONFIGREG_SMC_IND,
    KV_CONFIGREG_DIDT_IND,
    KV_CONFIGREG_CACHE,
    KV_CONFIGREG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_pt_config_reg {
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub value: u32,
    pub type: kv_pt_config_reg_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_lcac_config_values {
    pub block_id: u32,
    pub signal_id: u32,
    pub t: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_lcac_config_reg {
    pub cntl: u32,
    pub block_mask: u32,
    pub block_shift: u32,
    pub signal_mask: u32,
    pub signal_shift: u32,
    pub t_mask: u32,
    pub t_shift: u32,
    pub enable_mask: u32,
    pub enable_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_pl {
    pub sclk: u32,
    pub vddc_index: u8,
    pub ds_divider_index: u8,
    pub ss_divider_index: u8,
    pub allow_gnb_slow: u8,
    pub force_nbp_state: u8,
    pub display_wm: u8,
    pub vce_wm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_ps {
    pub levels: [kv_pl; SUMO_MAX_HARDWARE_POWERLEVELS],
    pub num_levels: u32,
    pub need_dfs_bypass: bool,
    pub dpm0_pg_nb_ps_lo: u8,
    pub dpm0_pg_nb_ps_hi: u8,
    pub dpmx_nb_ps_lo: u8,
    pub dpmx_nb_ps_hi: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_sys_info {
    pub bootup_uma_clk: u32,
    pub bootup_sclk: u32,
    pub dentist_vco_freq: u32,
    pub nb_dpm_enable: u32,
    pub nbp_memory_clock: [u32; KV_NUM_NBPSTATES],
    pub nbp_n_clock: [u32; KV_NUM_NBPSTATES],
    pub bootup_nb_voltage_index: u16,
    pub htc_tmp_lmt: u8,
    pub htc_hyst_lmt: u8,
    pub sclk_voltage_mapping_table: sumo_sclk_voltage_mapping_table,
    pub vid_mapping_table: sumo_vid_mapping_table,
    pub uma_channel_number: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kv_power_info {
    pub at: [u32; SUMO_MAX_HARDWARE_POWERLEVELS],
    pub voltage_drop_t: u32,
    pub sys_info: kv_sys_info,
    pub boot_pl: kv_pl,
    pub enable_nb_ps_policy: bool,
    pub disable_nb_ps3_in_battery: bool,
    pub video_start: bool,
    pub battery_state: bool,
    pub lowest_valid: u32,
    pub highest_valid: u32,
    pub high_voltage_t: u16,
    pub cac_enabled: bool,
    pub bapm_enable: bool,
// smc offsets
    pub sram_end: u32,
    pub dpm_table_start: u32,
    pub soft_regs_start: u32,
// dpm SMU tables
    pub graphics_dpm_level_count: u8,
    pub uvd_level_count: u8,
    pub vce_level_count: u8,
    pub acp_level_count: u8,
    pub samu_level_count: u8,
    pub fps_high_t: u16,
    pub graphics_level: [SMU7_Fusion_GraphicsLevel; SMU__NUM_SCLK_DPM_STATE],
    pub acpi_level: SMU7_Fusion_ACPILevel,
    pub uvd_level: [SMU7_Fusion_UvdLevel; SMU7_MAX_LEVELS_UVD],
    pub vce_level: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_VCE],
    pub acp_level: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_ACP],
    pub samu_level: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_SAMU],
    pub uvd_boot_level: u8,
    pub vce_boot_level: u8,
    pub acp_boot_level: u8,
    pub samu_boot_level: u8,
    pub uvd_interval: u8,
    pub vce_interval: u8,
    pub acp_interval: u8,
    pub samu_interval: u8,
    pub graphics_boot_level: u8,
    pub graphics_interval: u8,
    pub graphics_therm_throttle_enable: u8,
    pub graphics_voltage_change_enable: u8,
    pub graphics_clk_slow_enable: u8,
    pub graphics_clk_slow_divider: u8,
    pub fps_low_t: u8,
    pub low_sclk_interrupt_t: u32,
    pub uvd_power_gated: bool,
    pub vce_power_gated: bool,
    pub acp_power_gated: bool,
    pub samu_power_gated: bool,
    pub nb_dpm_enabled: bool,
// flags
    pub enable_didt: bool,
    pub enable_dpm: bool,
    pub enable_auto_thermal_throttling: bool,
    pub enable_nb_dpm: bool,
// caps
    pub caps_cac: bool,
    pub caps_power_containment: bool,
    pub caps_sq_ramping: bool,
    pub caps_db_ramping: bool,
    pub caps_td_ramping: bool,
    pub caps_tcp_ramping: bool,
    pub caps_sclk_throttle_low_notification: bool,
    pub caps_fps: bool,
    pub caps_uvd_dpm: bool,
    pub caps_uvd_pg: bool,
    pub caps_vce_pg: bool,
    pub caps_samu_pg: bool,
    pub caps_acp_pg: bool,
    pub caps_stable_p_state: bool,
    pub caps_enable_dfs_bypass: bool,
    pub caps_sclk_ds: bool,
    pub current_rps: radeon_ps,
    pub current_ps: kv_ps,
    pub requested_rps: radeon_ps,
    pub requested_ps: kv_ps,
}

// kv_smc.c
extern "C" {
    pub fn kv_notify_message_to_smu(rdev: *mut radeon_device, id: u32) -> c_int;
}
extern "C" {
    pub fn kv_dpm_get_enable_mask(rdev: *mut radeon_device, enable_mask: *mut u32) -> c_int;
}
extern "C" {
    pub fn kv_smc_dpm_enable(rdev: *mut radeon_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn kv_smc_bapm_enable(rdev: *mut radeon_device, enable: bool) -> c_int;
}
