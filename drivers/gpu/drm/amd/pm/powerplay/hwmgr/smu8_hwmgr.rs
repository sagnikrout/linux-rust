//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/smu8_hwmgr.h
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

pub const SMU8_NUM_NBPSTATES: c_int = 4;
pub const SMU8_NUM_NBPMEMORYCLOCK: c_int = 2;
pub const MAX_DISPLAY_CLOCK_LEVEL: c_int = 8;
pub const SMU8_MAX_HARDWARE_POWERLEVELS: c_int = 8;
pub const SMU8_VOTINGRIGHTSCLIENTS_DFLT0: c_uint = 0x3FFFC102;
pub const SMU8_MIN_DEEP_SLEEP_SCLK: c_int = 800;
// Carrizo device IDs
pub const DEVICE_ID_CZ_9870: c_uint = 0x9870;
pub const DEVICE_ID_CZ_9874: c_uint = 0x9874;
pub const DEVICE_ID_CZ_9875: c_uint = 0x9875;
pub const DEVICE_ID_CZ_9876: c_uint = 0x9876;
pub const DEVICE_ID_CZ_9877: c_uint = 0x9877;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_dpm_entry {
    pub soft_min_clk: u32,
    pub hard_min_clk: u32,
    pub soft_max_clk: u32,
    pub hard_max_clk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_sys_info {
    pub bootup_uma_clock: u32,
    pub bootup_engine_clock: u32,
    pub dentist_vco_freq: u32,
    pub nb_dpm_enable: u32,
    pub nbp_memory_clock: [u32; SMU8_NUM_NBPMEMORYCLOCK],
    pub nbp_n_clock: [u32; SMU8_NUM_NBPSTATES],
    pub nbp_voltage_index: [u16; SMU8_NUM_NBPSTATES],
    pub display_clock: [u32; MAX_DISPLAY_CLOCK_LEVEL],
    pub bootup_nb_voltage_index: u16,
    pub htc_tmp_lmt: u8,
    pub htc_hyst_lmt: u8,
    pub system_config: u32,
    pub uma_channel_number: u32,
}

pub const MAX_DISPLAYPHY_IDS: c_uint = 0x8;
pub const DISPLAYPHY_LANEMASK: c_uint = 0xF;

pub const DISPLAYPHY_PHYID_SHIFT: c_int = 24;
pub const DISPLAYPHY_LANESELECT_SHIFT: c_int = 16;
pub const DISPLAYPHY_RX_SELECT: c_uint = 0x1;
pub const DISPLAYPHY_TX_SELECT: c_uint = 0x2;
pub const DISPLAYPHY_CORE_SELECT: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_display_phy_info_entry {
    pub phy_present: u8,
    pub active_lane_mapping: u8,
    pub display_config_type: u8,
    pub active_number_of_lanes: u8,
}

pub const SMU8_MAX_DISPLAYPHY_IDS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_display_phy_info {
    pub display_phy_access_initialized: bool,
    pub entries: [smu8_display_phy_info_entry; SMU8_MAX_DISPLAYPHY_IDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_power_level {
    pub engineClock: u32,
    pub vddcIndex: u8,
    pub dsDividerIndex: u8,
    pub ssDividerIndex: u8,
    pub allowGnbSlow: u8,
    pub forceNBPstate: u8,
    pub display_wm: u8,
    pub vce_wm: u8,
    pub numSIMDToPowerDown: u8,
    pub hysteresis_up: u8,
    pub rsv: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_uvd_clocks {
    pub vclk: u32,
    pub dclk: u32,
    pub vclk_low_divider: u32,
    pub vclk_high_divider: u32,
    pub dclk_low_divider: u32,
    pub dclk_high_divider: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smu8_pstate_previous_action {
    DO_NOTHING = 1,
    FORCE_HIGH,
    CANCEL_FORCE_HIGH
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_disable_nb_ps_flags {
    pub 1: uint32_t entry :,
    pub 1: uint32_t display :,
    pub 1: uint32_t driver:,
    pub 1: uint32_t vce :,
    pub 1: uint32_t uvd :,
    pub 1: uint32_t acp :,
    pub 26: uint32_t reserved:,
    pub bits: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_power_state {
    pub magic: c_uint,
    pub level: u32,
    pub uvd_clocks: smu8_uvd_clocks,
    pub evclk: u32,
    pub ecclk: u32,
    pub samclk: u32,
    pub acpclk: u32,
    pub need_dfs_bypass: bool,
    pub nbps_flags: u32,
    pub bapm_flags: u32,
    pub dpm_0_pg_nb_ps_low: u8,
    pub dpm_0_pg_nb_ps_high: u8,
    pub dpm_x_nb_ps_low: u8,
    pub dpm_x_nb_ps_high: u8,
    pub action: smu8_pstate_previous_action,
    pub levels: [smu8_power_level; SMU8_MAX_HARDWARE_POWERLEVELS],
    pub disable_nb_ps_flag: pp_disable_nb_ps_flags,
}

pub const DPMFlags_SCLK_Enabled: c_uint = 0x00000001;
pub const DPMFlags_UVD_Enabled: c_uint = 0x00000002;
pub const DPMFlags_VCE_Enabled: c_uint = 0x00000004;
pub const DPMFlags_ACP_Enabled: c_uint = 0x00000008;
pub const DPMFlags_ForceHighestValid: c_uint = 0x40000000;
pub const DPMFlags_Debug: c_uint = 0x80000000;
pub const SMU_EnabledFeatureScoreboard_AcpDpmOn: c_uint = 0x00000001 /* bit 0 */;
pub const SMU_EnabledFeatureScoreboard_UvdDpmOn: c_uint = 0x00800000 /* bit 23 */;
pub const SMU_EnabledFeatureScoreboard_VceDpmOn: c_uint = 0x01000000 /* bit 24 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc6_settings {
    pub cc6_setting_changed: bool,
    pub /: *mut *mut bool nb_pstate_switch_disable;/ controls NB PState switch,
    pub /: *mut *mut bool cpu_cc6_disable; / controls CPU CState switch ( on or off),
    pub cpu_pstate_disable: bool,
    pub cpu_pstate_separation_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu8_hwmgr {
    pub dpm_interval: u32,
    pub voltage_drop_threshold: u32,
    pub voting_rights_clients: u32,
    pub disable_driver_thermal_policy: u32,
    pub static_screen_threshold: u32,
    pub gfx_power_gating_threshold: u32,
    pub activity_hysteresis: u32,
    pub bootup_sclk_divider: u32,
    pub gfx_ramp_step: u32,
    pub /: *mut *mut uint32_t gfx_ramp_delay; / in micro-seconds,
    pub thermal_auto_throttling_treshold: u32,
    pub sys_info: smu8_sys_info,
    pub boot_power_level: smu8_power_level,
    pub smu8_current_ps: *mut smu8_power_state,
    pub smu8_requested_ps: *mut smu8_power_state,
    pub mgcg_cgtt_local0: u32,
    pub mgcg_cgtt_local1: u32,
    pub /: *mut *mut uint32_t tdr_clock; / in 10khz unit,
    pub ddi_power_gating_disabled: u32,
    pub disable_gfx_power_gating_in_uvd: u32,
    pub disable_nb_ps3_in_battery: u32,
    pub lock_nb_ps_in_uvd_play_back: u32,
    pub display_phy_info: smu8_display_phy_info,
    pub /: *mut *mut uint32_t vce_slow_sclk_threshold; / default 200mhz,
    pub /: *mut *mut uint32_t dce_slow_sclk_threshold; / default 300mhz,
    pub /: *mut *mut uint32_t min_sclk_did; / minimum sclk divider,
    pub disp_clk_bypass: bool,
    pub disp_clk_bypass_pending: bool,
    pub bapm_enabled: u32,
    pub clock_slow_down_freq: u32,
    pub skip_clock_slow_down: u32,
    pub enable_nb_ps_policy: u32,
    pub voltage_drop_in_dce_power_gating: u32,
    pub uvd_dpm_interval: u32,
    pub override_dynamic_mgpg: u32,
    pub lclk_deep_enabled: u32,
    pub uvd_performance: u32,
    pub video_start: bool,
    pub battery_state: bool,
    pub lowest_valid: u32,
    pub highest_valid: u32,
    pub high_voltage_threshold: u32,
    pub is_nb_dpm_enabled: u32,
    pub cc6_settings: cc6_settings,
    pub is_voltage_island_enabled: u32,
    pub pgacpinit: bool,
    pub disp_config: u8,
// PowerTune
    pub power_containment_features: u32,
    pub cac_enabled: bool,
    pub disable_uvd_power_tune_feature: bool,
    pub enable_ba_pm_feature: bool,
    pub enable_tdc_limit_feature: bool,
    pub sram_end: u32,
    pub dpm_table_start: u32,
    pub soft_regs_start: u32,
    pub uvd_level_count: u8,
    pub vce_level_count: u8,
    pub acp_level_count: u8,
    pub samu_level_count: u8,
    pub fps_high_threshold: u32,
    pub fps_low_threshold: u32,
    pub dpm_flags: u32,
    pub sclk_dpm: smu8_dpm_entry,
    pub uvd_dpm: smu8_dpm_entry,
    pub vce_dpm: smu8_dpm_entry,
    pub acp_dpm: smu8_dpm_entry,
    pub uvd_boot_level: u8,
    pub vce_boot_level: u8,
    pub acp_boot_level: u8,
    pub samu_boot_level: u8,
    pub uvd_interval: u8,
    pub vce_interval: u8,
    pub acp_interval: u8,
    pub samu_interval: u8,
    pub graphics_interval: u8,
    pub graphics_therm_throttle_enable: u8,
    pub graphics_voltage_change_enable: u8,
    pub graphics_clk_slow_enable: u8,
    pub graphics_clk_slow_divider: u8,
    pub display_cac: u32,
    pub low_sclk_interrupt_threshold: u32,
    pub dram_log_addr_h: u32,
    pub dram_log_addr_l: u32,
    pub dram_log_phy_addr_h: u32,
    pub dram_log_phy_addr_l: u32,
    pub dram_log_buff_size: u32,
    pub uvd_power_gated: bool,
    pub vce_power_gated: bool,
    pub samu_power_gated: bool,
    pub acp_power_gated: bool,
    pub acp_power_up_no_dsp: bool,
    pub active_process_mask: u32,
    pub max_sclk_level: u32,
    pub num_of_clk_entries: u32,
}
