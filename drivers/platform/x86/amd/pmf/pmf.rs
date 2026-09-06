//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/amd/pmf/pmf.h
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


// SPDX-License-Identifier: GPL-2.0
//
// AMD Platform Management Framework Driver
//
// Copyright (c) 2022, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
//

pub const POLICY_BUF_MAX_SZ: c_uint = 0x4b000;
pub const POLICY_SIGN_COOKIE: c_uint = 0x31535024;
pub const POLICY_COOKIE_OFFSET: c_uint = 0x10;
// List of supported CPU ids
pub const AMD_CPU_ID_RMB: c_uint = 0x14b5;
pub const AMD_CPU_ID_PS: c_uint = 0x14e8;
pub const PCI_DEVICE_ID_AMD_1AH_M20H_ROOT: c_uint = 0x1507;
pub const PCI_DEVICE_ID_AMD_1AH_M60H_ROOT: c_uint = 0x1122;
pub const PCI_DEVICE_ID_AMD_1AH_M80H_ROOT: c_uint = 0x115b;
// Aliases required by PCI_DEVICE_DATA() macro naming convention

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cookie_header {
    pub sign: u32,
    pub length: u32,
    pub __packed: },
// APMF Functions
pub const APMF_FUNC_VERIFY_INTERFACE: c_int = 0;
pub const APMF_FUNC_GET_SYS_PARAMS: c_int = 1;
pub const APMF_FUNC_SBIOS_REQUESTS: c_int = 2;
pub const APMF_FUNC_SBIOS_HEARTBEAT: c_int = 4;
pub const APMF_FUNC_AUTO_MODE: c_int = 5;
pub const APMF_FUNC_SET_FAN_IDX: c_int = 7;
pub const APMF_FUNC_OS_POWER_SLIDER_UPDATE: c_int = 8;
pub const APMF_FUNC_STATIC_SLIDER_GRANULAR: c_int = 9;
pub const APMF_FUNC_DYN_SLIDER_AC: c_int = 11;
pub const APMF_FUNC_DYN_SLIDER_DC: c_int = 12;
pub const APMF_FUNC_NOTIFY_SMART_PC_UPDATES: c_int = 14;
pub const APMF_FUNC_SBIOS_HEARTBEAT_V2: c_int = 16;
// Message Definitions
pub const SET_SPL: c_uint = 0x03 /* SPL: Sustained Power Limit */;
pub const SET_SPPT: c_uint = 0x05 /* SPPT: Slow Package Power Tracking */;
pub const SET_FPPT: c_uint = 0x07 /* FPPT: Fast Package Power Tracking */;
pub const GET_SPL: c_uint = 0x0B;
pub const GET_SPPT: c_uint = 0x0D;
pub const GET_FPPT: c_uint = 0x0F;
pub const SET_DRAM_ADDR_HIGH: c_uint = 0x14;
pub const SET_DRAM_ADDR_LOW: c_uint = 0x15;
pub const SET_TRANSFER_TABLE: c_uint = 0x16;
pub const SET_STT_MIN_LIMIT: c_uint = 0x18 /* STT: Skin Temperature Tracking */;
pub const SET_STT_LIMIT_APU: c_uint = 0x19;
pub const SET_STT_LIMIT_HS2: c_uint = 0x1A;
pub const SET_SPPT_APU_ONLY: c_uint = 0x1D;
pub const GET_SPPT_APU_ONLY: c_uint = 0x1E;
pub const GET_STT_MIN_LIMIT: c_uint = 0x1F;
pub const GET_STT_LIMIT_APU: c_uint = 0x20;
pub const GET_STT_LIMIT_HS2: c_uint = 0x21;
pub const SET_P3T: c_uint = 0x23 /* P3T: Peak Package Power Limit */;
pub const SET_PMF_PPT: c_uint = 0x25;
pub const SET_PMF_PPT_APU_ONLY: c_uint = 0x26;
// Message IDs for 1AH_M80H platform
pub const GET_1AH_M80H_METRICS_TABLE_LOG_SAMPLE: c_uint = 0x0E;
pub const GET_1AH_M80H_METRICS_TABLE_DRAM_ADDR: c_uint = 0x0F;
// OS slider update notification
pub const DC_BEST_PERF: c_int = 0;
pub const DC_BETTER_PERF: c_int = 1;
pub const DC_BATTERY_SAVER: c_int = 3;
pub const AC_BEST_PERF: c_int = 4;
pub const AC_BETTER_PERF: c_int = 5;
pub const AC_BETTER_BATTERY: c_int = 6;
// Fan Index for Auto Mode
pub const FAN_INDEX_AUTO: c_uint = 0xFFFFFFFF;
pub const ARG_NONE: c_int = 0;
pub const AVG_SAMPLE_SIZE: c_int = 3;
// Policy Actions
pub const PMF_POLICY_SPL: c_int = 2;
pub const PMF_POLICY_SPPT: c_int = 3;
pub const PMF_POLICY_FPPT: c_int = 4;
pub const PMF_POLICY_SPPT_APU_ONLY: c_int = 5;
pub const PMF_POLICY_STT_MIN: c_int = 6;
pub const PMF_POLICY_STT_SKINTEMP_APU: c_int = 7;
pub const PMF_POLICY_STT_SKINTEMP_HS2: c_int = 8;
pub const PMF_POLICY_SYSTEM_STATE: c_int = 9;
pub const PMF_POLICY_BIOS_OUTPUT_1: c_int = 10;
pub const PMF_POLICY_BIOS_OUTPUT_2: c_int = 11;
pub const PMF_POLICY_P3T: c_int = 38;
pub const PMF_POLICY_PMF_PPT: c_int = 54;
pub const PMF_POLICY_PMF_PPT_APU_ONLY: c_int = 55;
pub const PMF_POLICY_BIOS_OUTPUT_3: c_int = 57;
pub const PMF_POLICY_BIOS_OUTPUT_4: c_int = 58;
pub const PMF_POLICY_BIOS_OUTPUT_5: c_int = 59;
pub const PMF_POLICY_BIOS_OUTPUT_6: c_int = 60;
pub const PMF_POLICY_BIOS_OUTPUT_7: c_int = 61;
pub const PMF_POLICY_BIOS_OUTPUT_8: c_int = 62;
pub const PMF_POLICY_BIOS_OUTPUT_9: c_int = 63;
pub const PMF_POLICY_BIOS_OUTPUT_10: c_int = 64;
// TA macros
pub const PMF_TA_IF_VERSION_MAJOR: c_int = 1;
pub const TA_PMF_ACTION_MAX: c_int = 32;
pub const TA_PMF_UNDO_MAX: c_int = 8;
pub const TA_OUTPUT_RESERVED_MEM: c_int = 922;
pub const MAX_OPERATION_PARAMS: c_int = 4;
pub const TA_ERROR_CRYPTO_INVALID_PARAM: c_uint = 0x20002;
pub const TA_ERROR_CRYPTO_BIN_TOO_LARGE: c_uint = 0x2000d;
pub const PMF_IF_V1: c_int = 1;
pub const PMF_IF_V2: c_int = 2;
pub const APTS_MAX_STATES: c_int = 16;

pub const BIOS_INPUTS_MAX: c_int = 10;

// amd_pmf_send_cmd() set/get

pub const METRICS_TABLE_ID: c_int = 7;
pub const BIOS_OUTPUT_MAX: c_int = 10;
    pub metrics_table_loop_ms: extern int,
pub const AMD_PMF_METRIC_CORE_TYPE_MAX: c_int = 4;
pub const AMD_PMF_METRIC_CCX_MAX: c_int = 4;
pub const AMD_PMF_NUM_CLK_DPM_LEVELS: c_int = 8;
pub const AMD_PMF_NUM_MAX_CORES: c_int = 12;
    pub data): *mut *mut typedef void (apmf_event_handler_t)(acpi_handle handle, u32 event, void,
}

// APTS PMF BIOS Interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_apts_output {
    pub table_version: u16,
    pub fan_table_idx: u32,
    pub pmf_ppt: u32,
    pub ppt_pmf_apu_only: u32,
    pub stt_min_limit: u32,
    pub stt_skin_temp_limit_apu: u8,
    pub stt_skin_temp_limit_hs2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_apts_granular_output {
    pub size: u16,
    pub val: amd_pmf_apts_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_apts_granular {
    pub size: u16,
    pub val: [amd_pmf_apts_output; APTS_MAX_STATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbios_hb_event_v2 {
    pub size: u16,
    pub load: u8,
    pub unload: u8,
    pub suspend: u8,
    pub resume: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sbios_hb_v2 {
    ON_LOAD,
    ON_UNLOAD,
    ON_SUSPEND,
    ON_RESUME,
}

// AMD PMF BIOS interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_verify_interface {
    pub size: u16,
    pub version: u16,
    pub notification_mask: u32,
    pub supported_functions: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_system_params {
    pub size: u16,
    pub valid_mask: u32,
    pub flags: u32,
    pub command_code: u8,
    pub heartbeat_int: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_sbios_req {
    pub size: u16,
    pub pending_req: u32,
    pub rsd: u8,
    pub cql_event: u8,
    pub amt_event: u8,
    pub fppt: u32,
    pub sppt: u32,
    pub fppt_apu_only: u32,
    pub spl: u32,
    pub stt_min_limit: u32,
    pub skin_temp_apu: u8,
    pub skin_temp_hs2: u8,
    pub __packed: },
// As per APMF spec 1.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_sbios_req_v1 {
    pub size: u16,
    pub pending_req: u32,
    pub rsvd: u8,
    pub cql_event: u8,
    pub amt_event: u8,
    pub fppt: u32,
    pub sppt: u32,
    pub sppt_apu_only: u32,
    pub spl: u32,
    pub stt_min_limit: u32,
    pub skin_temp_apu: u8,
    pub skin_temp_hs2: u8,
    pub enable_cnqf: u8,
    pub custom_policy: [u32; BIOS_INPUTS_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_sbios_req_v2 {
    pub size: u16,
    pub pending_req: u32,
    pub rsd: u8,
    pub ppt_pmf: u32,
    pub ppt_pmf_apu_only: u32,
    pub stt_min_limit: u32,
    pub skin_temp_apu: u8,
    pub skin_temp_hs2: u8,
    pub custom_policy: [u32; BIOS_INPUTS_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_fan_idx {
    pub size: u16,
    pub fan_ctl_mode: u8,
    pub fan_ctl_idx: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_metrics_iod {
    pub counter_acc: u32,
// Set Voltage
    pub vddcr_set_voltage: u64,
    pub vddcr_soc_set_voltage: u64,
    pub vddcr_npu_set_voltage: u64,
    pub vddcr_lp_set_voltage: u64,
    pub vddcr_gfx_set_voltage: u64,
    pub vdd_misc_set_voltage: u64,
// Telemetry Voltages
    pub vddcr_telemetry_voltage: u64,
    pub vddcr_soc_telemetry_voltage: u64,
    pub vddcr_npu_telemetry_voltage: u64,
    pub vddcr_lp_telemetry_voltage: u64,
    pub vddcr_gfx_telemetry_voltage: u64,
    pub vdd_misc_telemetry_voltage: u64,
// Telemetry Powers
    pub vddcr_telemetry_power: u64,
    pub vddcr_soc_telemetry_power: u64,
    pub vddcr_npu_telemetry_power: u64,
    pub vddcr_lp_telemetry_power: u64,
    pub vddcr_gfx_telemetry_power: u64,
    pub vdd_misc_telemetry_power: u64,
// Throttlers - Fast PPT
    pub fppt_fused_limit: u32,
    pub fppt_max_irm_limit: u32,
    pub fppt_max_pbo_limit: u32,
    pub fppt_limit: u32,
    pub fppt_value_acc: u64,
    pub fppt_residency_acc: u32,
// Throttlers - Slow PPT
    pub sppt_fused_limit: u32,
    pub sppt_max_irm_limit: u32,
    pub sppt_max_pbo_limit: u32,
    pub sppt_limit: u32,
    pub sppt_value_acc: u64,
    pub sppt_residency_acc: u32,
// Throttlers - STAPM
    pub spl_fused_limit: u32,
    pub spl_max_irm_limit: u32,
    pub spl_max_pbo_limit: u32,
    pub spl_limit: u32,
    pub spl_value_acc: u64,
    pub spl_residency_acc: u32,
// Throttlers - TDC VDDCR
    pub tdc_vddcr_fused_limit: u32,
    pub tdc_vddcr_max_irm_limit: u32,
    pub tdc_vddcr_max_pbo_limit: u32,
    pub tdc_vddcr_limit: u32,
    pub tdc_vddcr_value_acc: u64,
    pub tdc_vddcr_residency_acc: u32,
// Throttlers - TDC VDDCR_SOC
    pub tdc_vddcr_soc_fused_limit: u32,
    pub tdc_vddcr_soc_max_irm_limit: u32,
    pub tdc_vddcr_soc_max_pbo_limit: u32,
    pub tdc_vddcr_soc_limit: u32,
    pub tdc_vddcr_soc_value_acc: u64,
    pub tdc_vddcr_soc_residency_acc: u32,
// Throttlers - TDC VDDCR_NPU
    pub tdc_vddcr_npu_fused_limit: u32,
    pub tdc_vddcr_npu_max_irm_limit: u32,
    pub tdc_vddcr_npu_max_pbo_limit: u32,
    pub tdc_vddcr_npu_limit: u32,
    pub tdc_vddcr_npu_value_acc: u64,
    pub tdc_vddcr_npu_residency_acc: u32,
// Throttlers - TDC VDDCR_LP
    pub tdc_vddcr_lp_fused_limit: u32,
    pub tdc_vddcr_lp_max_irm_limit: u32,
    pub tdc_vddcr_lp_max_pbo_limit: u32,
    pub tdc_vddcr_lp_limit: u32,
    pub tdc_vddcr_lp_value_acc: u64,
    pub tdc_vddcr_lp_residency_acc: u32,
// Throttlers - TDC VDDCR_GFX
    pub tdc_vddcr_gfx_fused_limit: u32,
    pub tdc_vddcr_gfx_max_irm_limit: u32,
    pub tdc_vddcr_gfx_max_pbo_limit: u32,
    pub tdc_vddcr_gfx_limit: u32,
    pub tdc_vddcr_gfx_value_acc: u64,
    pub tdc_vddcr_gfx_residency_acc: u32,
// Throttlers - EDC VDDCR
    pub edc_vddcr_fused_limit: u32,
    pub edc_vddcr_max_irm_limit: u32,
    pub edc_vddcr_max_pbo_limit: u32,
    pub edc_vddcr_limit: u32,
// Throttlers - Thermal
    pub thm_fused_limit: u32,
    pub thm_limit: u32,
    pub thm_value_acc: u64,
    pub thm_residency_acc: u32,
    pub prochot_residency_acc: u32,
    pub gfx_temp_acc: u64,
    pub soc_temp_acc: u64,
    pub p3t_fused_limit: u32,
    pub p3t_value_acc: u64,
// Power
    pub system_power_acc: u64,
    pub apu_power_acc: u64,
    pub dgpu_power_acc: u64,
    pub npu_power_acc: u64,
// Frequencies
    pub fclk_freq_eff_acc: u64,
    pub memclk_freq_eff_acc: u64,
    pub lclk_freq_eff_acc: u64,
    pub gfxclk_freq_eff_acc: u64,
    pub socclk_freq_eff_acc: u64,
    pub vclk_freq_eff_acc: u64,
    pub vpeclk_freq_eff_acc: u64,
    pub aieclk_freq_eff_acc: u64,
    pub npuhclk_freq_eff_acc: u64,
// Bandwidth
    pub dram_read_bandwidth: u64,
    pub dram_write_bandwidth: u64,
// Activity Monitors
    pub gfx_busy_acc: u64,
    pub vcn_busy_acc: u64,
    pub npu_busy_acc: [u64; 3],
// STT Limits
    pub stt_min_limit: u32,
    pub stt_apu_hotspot_temp_acc: u64,
    pub stt_hs2_hotspot_temp_acc: u64,
    pub stt_apu_temp_limit: u32,
    pub stt_apu_skin_temp_acc: u64,
// Residencies
    pub cpuoff_residency_ccx0: u64,
    pub cpuoff_residency_ccx1: u64,
    pub cpuoff_residency_ccx2: u64,
    pub cpuoff_residency_ccx3: u64,
// DF-pstates
    pub fclk_freq_table: [u32; AMD_PMF_NUM_CLK_DPM_LEVELS],
    pub uclk_freq_table: [u32; AMD_PMF_NUM_CLK_DPM_LEVELS],
    pub ddr_rate_table: [u32; AMD_PMF_NUM_CLK_DPM_LEVELS],
    pub dfpstate_source: [u8; AMD_PMF_NUM_CLK_DPM_LEVELS],
// System
    pub gfx_disabled: u8,
    pub spare2: [u8; 3],
    pub gfxclk_fmax: u32,
    pub cclk_core_fuse_enable: [u8; AMD_PMF_METRIC_CORE_TYPE_MAX][AMD_PMF_NUM_MAX_CORES],
    pub cclk_core_enabled: [u8; AMD_PMF_METRIC_CORE_TYPE_MAX][AMD_PMF_NUM_MAX_CORES],
    pub cclk_fmax: [u32; AMD_PMF_METRIC_CORE_TYPE_MAX][AMD_PMF_NUM_MAX_CORES],
// Overclock Capable
    pub cpu_precise_and_direct_oc_capable: u8,
    pub gfx_precise_and_direct_oc_capable: u8,
    pub pbo_basic_oc_capable: u8,
    pub pbo_advanced_oc_capable: u8,
    pub pbo_nitro_oc_capable: u8,
    pub memory_and_fabric_oc_capable: u8,
    pub misc_oc_capable: u8,
    pub extreme_cold_oc_capable: u8,
    pub down_config_control_capable: u8,
    pub spare0: [u8; 3],
// Overclock Status
    pub fit_limit_scalar: u32,
    pub ln2_enabled: u8,
    pub cpu_precise_and_direct_oc_enabled: u8,
    pub gfx_precise_and_direct_oc_enabled: u8,
    pub spare1: [u8; 2],
// Voltage Guardband in PSM count
    pub psm_guardband: [i8; 5][5][3],
    pub core_power_limit_offset: i32,
    pub max_freq_offset: [u32; 5],
    pub npu_temp_acc: u64,
    pub dfpstate_residency_acc: [u64; AMD_PMF_NUM_CLK_DPM_LEVELS],
    pub cclk_fboost: u32,
// PMF
    pub pmf_fast_apu_ppt_limit: u32,
    pub pmf_fast_apu_ppt_value_acc: u64,
    pub pmf_fast_apu_ppt_residency_acc: u32,
    pub pmf_slow_apu_ppt_limit: u32,
    pub pmf_slow_apu_ppt_value_acc: u64,
    pub pmf_slow_apu_ppt_residency_acc: u32,
    pub pmf_fast_spm_limit: u32,
    pub pmf_fast_spm_value_acc: u64,
    pub pmf_fast_spm_residency_acc: u32,
    pub pmf_slow_spm_limit: u32,
    pub pmf_slow_spm_value_acc: u64,
    pub pmf_slow_spm_residency_acc: u32,
    pub spare3: [u32; 5],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_metrics_ccx {
    pub core_c0: [u64; AMD_PMF_NUM_MAX_CORES],
    pub core_cc6: [u64; AMD_PMF_NUM_MAX_CORES],
    pub core_freq: [u64; AMD_PMF_NUM_MAX_CORES],
    pub core_freqeff: [u64; AMD_PMF_NUM_MAX_CORES],
    pub core_temp: [u64; AMD_PMF_NUM_MAX_CORES],
    pub core_power: [u64; AMD_PMF_NUM_MAX_CORES],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_metrics_v3 {
    pub iod: amd_pmf_metrics_iod,
    pub ccx: [amd_pmf_metrics_ccx; AMD_PMF_METRIC_CCX_MAX],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_pmf_metrics_v2 {
    pub /: *mut *mut u16 core_frequency[16]; / MHz,
    pub /: *mut *mut u16 core_power[16]; / mW,
    pub /: *mut *mut u16 core_temp[16]; / centi-C,
    pub /: *mut *mut u16 gfx_temp; / centi-C,
    pub /: *mut *mut u16 soc_temp; / centi-C,
    pub /: *mut *mut u16 stapm_opn_limit; / mW,
    pub /: *mut *mut u16 stapm_cur_limit; / mW,
    pub /: *mut *mut u16 infra_cpu_maxfreq; / MHz,
    pub /: *mut *mut u16 infra_gfx_maxfreq; / MHz,
    pub /: *mut *mut u16 skin_temp; / centi-C,
    pub /: *mut *mut u16 gfxclk_freq; / MHz,
    pub /: *mut *mut u16 fclk_freq; / MHz,
    pub /: *mut *mut u16 gfx_activity; / GFX busy % [0-100],
    pub /: *mut *mut u16 socclk_freq; / MHz,
    pub /: *mut *mut u16 vclk_freq; / MHz,
    pub /: *mut *mut u16 vcn_activity; / VCN busy % [0-100],
    pub /: *mut *mut u16 vpeclk_freq; / MHz,
    pub /: *mut *mut u16 npuclk_freq; / MHz,
    pub /: *mut *mut u16 npu_busy[8]; / NPU busy % [0-100],
    pub /: *mut *mut u16 dram_reads; / MB/sec,
    pub /: *mut *mut u16 dram_writes; / MB/sec,
    pub /: *mut *mut u16 core_c0residency[16]; / C0 residency % [0-100],
    pub /: *mut *mut u16 npu_power; / mW,
    pub /: *mut *mut u32 apu_power; / mW,
    pub /: *mut *mut u32 gfx_power; / mW,
    pub /: *mut *mut u32 dgpu_power; / mW,
    pub /: *mut *mut u32 socket_power; / mW,
    pub /: *mut *mut u32 all_core_power; / mW,
    pub /: *mut *mut u32 filter_alpha_value; / time constant [us],
    pub metrics_counter: u32,
    pub /: *mut *mut u16 memclk_freq; / MHz,
    pub /: *mut *mut u16 mpnpuclk_freq; / MHz,
    pub /: *mut *mut u16 npu_reads; / MB/sec,
    pub /: *mut *mut u16 npu_writes; / MB/sec,
    pub throttle_residency_prochot: u32,
    pub throttle_residency_spl: u32,
    pub throttle_residency_fppt: u32,
    pub throttle_residency_sppt: u32,
    pub throttle_residency_thm_core: u32,
    pub throttle_residency_thm_gfx: u32,
    pub throttle_residency_thm_soc: u32,
    pub psys: u16,
    pub spare1: u16,
    pub spare: [u32; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_pmf_metrics {
    pub /: *mut *mut u16 gfxclk_freq; / in MHz,
    pub /: *mut *mut u16 socclk_freq; / in MHz,
    pub /: *mut *mut u16 vclk_freq; / in MHz,
    pub /: *mut *mut u16 dclk_freq; / in MHz,
    pub /: *mut *mut u16 memclk_freq; / in MHz,
    pub spare: u16,
    pub /: *mut *mut u16 gfx_activity; / in Centi,
    pub /: *mut *mut u16 uvd_activity; / in Centi,
    pub /: *mut *mut u16 voltage[2]; / in mV,
    pub /: *mut *mut u16 currents[2]; / in mA,
    pub /: *mut *mut u16 power[2];/ in mW,
    pub /: *mut *mut u16 core_freq[8]; / in MHz,
    pub /: *mut *mut u16 core_power[8]; / in mW,
    pub /: *mut *mut u16 core_temp[8]; / in centi-Celsius,
    pub /: *mut *mut u16 l3_freq; / in MHz,
    pub /: *mut *mut u16 l3_temp; / in centi-Celsius,
    pub /: *mut *mut u16 gfx_temp; / in centi-Celsius,
    pub /: *mut *mut u16 soc_temp; / in centi-Celsius,
    pub throttler_status: u16,
    pub /: *mut *mut u16 current_socketpower; / in mW,
    pub /: *mut *mut u16 stapm_orig_limit; / in W,
    pub /: *mut *mut u16 stapm_cur_limit; / in W,
    pub /: *mut *mut u32 apu_power; / in mW,
    pub /: *mut *mut u32 dgpu_power; / in mW,
    pub /: *mut *mut u16 vdd_tdc_val; / in mA,
    pub /: *mut *mut u16 soc_tdc_val; / in mA,
    pub /: *mut *mut u16 vdd_edc_val; / in mA,
    pub /: *mut *mut u16 soc_edcv_al; / in mA,
    pub /: *mut *mut u16 infra_cpu_maxfreq; / in MHz,
    pub /: *mut *mut u16 infra_gfx_maxfreq; / in MHz,
    pub /: *mut *mut u16 skin_temp; / in centi-Celsius,
    pub device_state: u16,
    pub /: *mut *mut u16 curtemp; / in centi-Celsius,
    pub filter_alpha_value: u16,
    pub avg_gfx_clkfrequency: u16,
    pub avg_fclk_frequency: u16,
    pub avg_gfx_activity: u16,
    pub avg_socclk_frequency: u16,
    pub avg_vclk_frequency: u16,
    pub avg_vcn_activity: u16,
    pub avg_dram_reads: u16,
    pub avg_dram_writes: u16,
    pub avg_socket_power: u16,
    pub avg_core_power: [u16; 2],
    pub avg_core_c0residency: [u16; 16],
    pub spare1: u16,
    pub metrics_counter: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_stt_skin_temp {
    STT_TEMP_APU,
    STT_TEMP_HS2,
    STT_TEMP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_slider_op {
    SLIDER_OP_GET,
    SLIDER_OP_SET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_source {
    POWER_SOURCE_AC,
    POWER_SOURCE_DC,
    POWER_SOURCE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_modes {
    POWER_MODE_PERFORMANCE,
    POWER_MODE_BALANCED_POWER,
    POWER_MODE_POWER_SAVER,
    POWER_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_modes_v2 {
    POWER_MODE_BEST_PERFORMANCE,
    POWER_MODE_BALANCED,
    POWER_MODE_BEST_POWER_EFFICIENCY,
    POWER_MODE_ENERGY_SAVE,
    POWER_MODE_V2_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_bios_inputs_prev {
    pub custom_bios_inputs: [u32; BIOS_INPUTS_MAX],
}

//
// struct pmf_bios_input_entry - Snapshot of custom BIOS input event
// @val: Array of custom BIOS input values
// @preq: Pending request value associated with this event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_bios_input_entry {
    pub val: [u32; BIOS_INPUTS_MAX],
    pub preq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_cbi_ring_buffer {
    pub data: [pmf_bios_input_entry; CUSTOM_BIOS_INPUT_RING_ENTRIES],
    pub head: c_int,
    pub tail: c_int,
}

// SoC-specific SMU mailbox register offsets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_smu_regs {
    pub msg_reg: u32,
    pub resp_reg: u32,
    pub arg_reg: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_arg_data {
    pub lo: u32,
    pub hi: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_dev {
    pub regbase: *mut void __iomem,
    pub smu_virt_addr: *mut void __iomem,
    pub buf: *mut c_void,
    pub base_addr: u32,
    pub cpu_id: u32,
    pub dev: *mut device,
    pub /: *mut *mut mutex lock; / protects the PMF interface,
    pub supported_func: u32,
    pub current_profile: platform_profile_option,
    pub /: *mut *mut *mut device ppdev; / platform profile class device,
    pub dbgfs_dir: *mut dentry,
    pub /: *mut *mut int hb_interval; / SBIOS heartbeat interval,
    pub heart_beat: delayed_work,
    pub m_table: smu_pmf_metrics,
    pub m_table_v2: smu_pmf_metrics_v2,
    pub work_buffer: delayed_work,
    pub start_time: ktime_t,
    pub socket_power_history: [c_int; AVG_SAMPLE_SIZE],
    pub socket_power_history_idx: c_int,
    pub amt_enabled: bool,
    pub /: *mut *mut mutex update_mutex; / protects race between ACPI handler and metrics thread,
    pub cnqf_enabled: bool,
    pub cnqf_supported: bool,
    pub pwr_src_notifier: notifier_block,
// Smart PC solution builder
    pub esbin: *mut dentry,
    pub policy_buf: *mut c_uchar,
    pub policy_sz: resource_size_t,
    pub tee_ctx: *mut tee_context,
    pub fw_shm_pool: *mut tee_shm,
    pub session_id: u32,
    pub shbuf: *mut c_void,
    pub pb_work: delayed_work,
    pub prev_data: *mut pmf_action_table,
    pub policy_addr: resource_size_t,
    pub policy_base: *mut void __iomem,
    pub smart_pc_enabled: bool,
    pub pmf_if_version: u16,
    pub pmf_idev: *mut input_dev,
    pub mtable_size: usize,
    pub res: *mut resource,
    pub /: *mut *mut apmf_sbios_req_v2 req; / To get custom bios pending request,
    pub cb_mutex: mutex,
    pub notifications: u32,
    pub req1: apmf_sbios_req_v1,
    pub /: *mut *mut pmf_bios_inputs_prev cb_prev; / To preserve custom BIOS inputs,
    pub /: *mut *mut bool cb_flag; / To handle first custom BIOS input,
    pub cbi_buf: pmf_cbi_ring_buffer,
    pub /: *mut *mut mutex cbi_mutex; / Protects ring buffer access,
    pub metrics_mutex: mutex,
    pub bios_output: [u32; BIOS_OUTPUT_MAX],
    pub smu_regs: *const amd_pmf_smu_regs,
    pub /: *mut *mut *mut void __iomem metrics_table_virt; / Mapped DRAM virtual address for metrics table,
    pub /: *mut *mut phys_addr_t metrics_table_phys; / DRAM physical address for metrics table,
    pub /: *mut *mut amd_pmf_metrics_v3 mtable_v3; / IOD and CCX,
    pub dram_addr: amd_pmf_arg_data,
    pub /: *mut *mut amd_pmf_metrics_v3 prev_metrics; / Previous metrics for delta calculation,
    pub npu_metrics_have_prev: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_sps_prop_granular_v2 {
    pub power_states: [u8; POWER_SOURCE_MAX][POWER_MODE_V2_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_sps_prop_granular {
    pub fppt: u32,
    pub sppt: u32,
    pub sppt_apu_only: u32,
    pub spl: u32,
    pub stt_min: u32,
    pub stt_skin_temp: [u8; STT_TEMP_COUNT],
    pub fan_id: u32,
    pub __packed: },
// Static Slider
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_static_slider_granular_output {
    pub size: u16,
    pub POWER_MODE_MAX]: *mut *mut apmf_sps_prop_granular prop[POWER_SOURCE_MAX,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_static_slider_granular {
    pub size: u16,
    pub prop: [apmf_sps_prop_granular; POWER_SOURCE_MAX][POWER_MODE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_static_slider_granular_output_v2 {
    pub size: u16,
    pub sps_idx: apmf_sps_prop_granular_v2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_static_slider_granular_v2 {
    pub size: u16,
    pub sps_idx: apmf_sps_prop_granular_v2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct os_power_slider {
    pub size: u16,
    pub slider_event: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_notify_smart_pc_update {
    pub size: u16,
    pub pending_req: u32,
    pub custom_bios: [u32; BIOS_INPUTS_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fan_table_control {
    pub manual: bool,
    pub fan_id: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_table_control {
    pub spl: u32,
    pub sppt: u32,
    pub fppt: u32,
    pub sppt_apu_only: u32,
    pub stt_min: u32,
    pub stt_skin_temp: [u32; STT_TEMP_COUNT],
    pub reserved: [u32; 16],
}

// Auto Mode Layer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum auto_mode_transition_priority {
    AUTO_TRANSITION_TO_PERFORMANCE, /* Any other mode to Performance Mode */
    AUTO_TRANSITION_FROM_QUIET_TO_BALANCE, /* Quiet Mode to Balance Mode */
    AUTO_TRANSITION_TO_QUIET, /* Any other mode to Quiet Mode */
    AUTO_TRANSITION_FROM_PERFORMANCE_TO_BALANCE, /* Performance Mode to Balance Mode */
    AUTO_TRANSITION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum auto_mode_mode {
    AUTO_QUIET,
    AUTO_BALANCE,
    AUTO_PERFORMANCE_ON_LAP,
    AUTO_PERFORMANCE,
    AUTO_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_mode_trans_params {
    pub /: *mut *mut u32 time_constant; / minimum time required to switch to next mode,
    pub /: *mut *mut u32 power_delta; / delta power to shift mode,
    pub power_threshold: u32,
    pub /: *mut *mut u32 timer; / elapsed time. if timer > TimeThreshold, it will move to next mode,
    pub applied: u32,
    pub target_mode: auto_mode_mode,
    pub shifting_up: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_mode_mode_settings {
    pub power_control: power_table_control,
    pub fan_control: fan_table_control,
    pub power_floor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_mode_mode_config {
    pub transition: [auto_mode_trans_params; AUTO_TRANSITION_MAX],
    pub mode_set: [auto_mode_mode_settings; AUTO_MODE_MAX],
    pub current_mode: auto_mode_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_auto_mode {
    pub size: u16,
// time constant
    pub balanced_to_perf: u32,
    pub perf_to_balanced: u32,
    pub quiet_to_balanced: u32,
    pub balanced_to_quiet: u32,
// power floor
    pub pfloor_perf: u32,
    pub pfloor_balanced: u32,
    pub pfloor_quiet: u32,
// Power delta for mode change
    pub pd_balanced_to_perf: u32,
    pub pd_perf_to_balanced: u32,
    pub pd_quiet_to_balanced: u32,
    pub pd_balanced_to_quiet: u32,
// skin temperature limits
    pub /: *mut *mut u8 stt_apu_perf_on_lap; / CQL ON,
    pub /: *mut *mut u8 stt_hs2_perf_on_lap; / CQL ON,
    pub stt_apu_perf: u8,
    pub stt_hs2_perf: u8,
    pub stt_apu_balanced: u8,
    pub stt_hs2_balanced: u8,
    pub stt_apu_quiet: u8,
    pub stt_hs2_quiet: u8,
    pub /: *mut *mut u32 stt_min_limit_perf_on_lap; / CQL ON,
    pub stt_min_limit_perf: u32,
    pub stt_min_limit_balanced: u32,
    pub stt_min_limit_quiet: u32,
// SPL based
    pub /: *mut *mut u32 fppt_perf_on_lap; / CQL ON,
    pub /: *mut *mut u32 sppt_perf_on_lap; / CQL ON,
    pub /: *mut *mut u32 spl_perf_on_lap; / CQL ON,
    pub /: *mut *mut u32 sppt_apu_only_perf_on_lap; / CQL ON,
    pub fppt_perf: u32,
    pub sppt_perf: u32,
    pub spl_perf: u32,
    pub sppt_apu_only_perf: u32,
    pub fppt_balanced: u32,
    pub sppt_balanced: u32,
    pub spl_balanced: u32,
    pub sppt_apu_only_balanced: u32,
    pub fppt_quiet: u32,
    pub sppt_quiet: u32,
    pub spl_quiet: u32,
    pub sppt_apu_only_quiet: u32,
// Fan ID
    pub fan_id_perf: u32,
    pub fan_id_balanced: u32,
    pub fan_id_quiet: u32,
    pub __packed: },
// CnQF Layer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cnqf_trans_priority {
    CNQF_TRANSITION_TO_TURBO, /* Any other mode to Turbo Mode */
    CNQF_TRANSITION_FROM_BALANCE_TO_PERFORMANCE, /* quiet/balance to Performance Mode */
    CNQF_TRANSITION_FROM_QUIET_TO_BALANCE, /* Quiet Mode to Balance Mode */
    CNQF_TRANSITION_TO_QUIET, /* Any other mode to Quiet Mode */
    CNQF_TRANSITION_FROM_PERFORMANCE_TO_BALANCE, /* Performance/Turbo to Balance Mode */
    CNQF_TRANSITION_FROM_TURBO_TO_PERFORMANCE, /* Turbo mode to Performance Mode */
    CNQF_TRANSITION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cnqf_mode {
    CNQF_MODE_QUIET,
    CNQF_MODE_BALANCE,
    CNQF_MODE_PERFORMANCE,
    CNQF_MODE_TURBO,
    CNQF_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum apmf_cnqf_pos {
    APMF_CNQF_TURBO,
    APMF_CNQF_PERFORMANCE,
    APMF_CNQF_BALANCE,
    APMF_CNQF_QUIET,
    APMF_CNQF_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnqf_mode_settings {
    pub power_control: power_table_control,
    pub fan_control: fan_table_control,
    pub power_floor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnqf_tran_params {
    pub /: *mut *mut u32 time_constant; / minimum time required to switch to next mode,
    pub power_threshold: u32,
    pub /: *mut *mut u32 timer; / elapsed time. if timer > timethreshold, it will move to next mode,
    pub total_power: u32,
    pub count: u32,
    pub priority: bool,
    pub shifting_up: bool,
    pub target_mode: cnqf_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnqf_config {
    pub trans_param: [cnqf_tran_params; POWER_SOURCE_MAX][CNQF_TRANSITION_MAX],
    pub mode_set: [cnqf_mode_settings; POWER_SOURCE_MAX][CNQF_MODE_MAX],
    pub defaults: power_table_control,
    pub current_mode: cnqf_mode,
    pub power_src: u32,
    pub avg_power: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_cnqf_power_set {
    pub pfloor: u32,
    pub fppt: u32,
    pub sppt: u32,
    pub sppt_apu_only: u32,
    pub spl: u32,
    pub stt_min_limit: u32,
    pub stt_skintemp: [u8; STT_TEMP_COUNT],
    pub fan_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apmf_dyn_slider_output {
    pub size: u16,
    pub flags: u16,
    pub t_perf_to_turbo: u32,
    pub t_balanced_to_perf: u32,
    pub t_quiet_to_balanced: u32,
    pub t_balanced_to_quiet: u32,
    pub t_perf_to_balanced: u32,
    pub t_turbo_to_perf: u32,
    pub ps: [apmf_cnqf_power_set; APMF_CNQF_MAX],
    pub __packed: },
// Smart PC - TA internals
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum system_state {
    SYSTEM_STATE_S0i3,
    SYSTEM_STATE_S4,
    SYSTEM_STATE_SCREEN_LOCK,
    SYSTEM_STATE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_pb_bitmap {
    pub name: *const c_char,
    pub bit_mask: u32,
}

// Command ids for TA communication
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_pmf_command {
    TA_PMF_COMMAND_POLICY_BUILDER_INITIALIZE,
    TA_PMF_COMMAND_POLICY_BUILDER_ENACT_POLICIES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_pmf_error_type {
    TA_PMF_TYPE_SUCCESS,
    TA_PMF_ERROR_TYPE_GENERIC,
    TA_PMF_ERROR_TYPE_CRYPTO,
    TA_PMF_ERROR_TYPE_CRYPTO_VALIDATE,
    TA_PMF_ERROR_TYPE_CRYPTO_VERIFY_OEM,
    TA_PMF_ERROR_TYPE_POLICY_BUILDER,
    TA_PMF_ERROR_TYPE_PB_CONVERT,
    TA_PMF_ERROR_TYPE_PB_SETUP,
    TA_PMF_ERROR_TYPE_PB_ENACT,
    TA_PMF_ERROR_TYPE_ASD_GET_DEVICE_INFO,
    TA_PMF_ERROR_TYPE_ASD_GET_DEVICE_PCIE_INFO,
    TA_PMF_ERROR_TYPE_SYS_DRV_FW_VALIDATION,
    TA_PMF_ERROR_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmf_action_table {
    pub system_state: system_state,
    pub /: *mut *mut u32 spl; / in mW,
    pub /: *mut *mut u32 sppt; / in mW,
    pub /: *mut *mut u32 sppt_apuonly; / in mW,
    pub /: *mut *mut u32 fppt; / in mW,
    pub /: *mut *mut u32 stt_minlimit; / in mW,
    pub /: *mut *mut u32 stt_skintemp_apu; / in C,
    pub /: *mut *mut u32 stt_skintemp_hs2; / in C,
    pub /: *mut *mut u32 p3t_limit; / in mW,
    pub /: *mut *mut u32 pmf_ppt; / in mW,
    pub /: *mut *mut u32 pmf_ppt_apu_only; / in mW,
}

// Input conditions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_condition_info {
    pub power_source: u32,
    pub bat_percentage: u32,
    pub power_slider: u32,
    pub lid_state: u32,
    pub user_present: bool,
    pub bios_input_1: [u32; 2],
    pub monitor_count: u32,
    pub rsvd2: [u32; 2],
    pub bat_design: u32,
    pub full_charge_capacity: u32,
    pub drain_rate: c_int,
    pub user_engaged: bool,
    pub device_state: u32,
    pub socket_power: u32,
    pub skin_temperature: u32,
    pub rsvd3: [u32; 2],
    pub platform_type: u32,
    pub rsvd3_1: [u32; 2],
    pub ambient_light: u32,
    pub length: u32,
    pub avg_c0residency: u32,
    pub max_c0residency: u32,
    pub s0i3_entry: u32,
    pub gfx_busy: u32,
    pub rsvd4: [u32; 7],
    pub camera_state: bool,
    pub workload_type: u32,
    pub display_type: u32,
    pub display_state: u32,
    pub rsvd5_1: [u32; 17],
    pub bios_input_2: [u32; 8],
    pub rsvd5: [u32; 125],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_load_policy_table {
    pub table_size: u32,
    pub table: [u8; POLICY_BUF_MAX_SZ],
}

// TA initialization params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_init_table {
    pub /: *mut *mut u32 frequency; / SMU sampling frequency,
    pub validate: bool,
    pub sku_check: bool,
    pub metadata_macrocheck: bool,
    pub policies_table: ta_pmf_load_policy_table,
}

// Everything the TA needs to Enact Policies
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_enact_table {
    pub ev_info: ta_pmf_condition_info,
    pub name: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_action {
    pub action_index: u32,
    pub value: u32,
    pub spl_arg: u32,
}

// Output actions from TA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_enact_result {
    pub actions_count: u32,
    pub actions_list: [ta_pmf_action; TA_PMF_ACTION_MAX],
    pub undo_count: u32,
    pub undo_list: [ta_pmf_action; TA_PMF_UNDO_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ta_pmf_input {
    pub enact_table: ta_pmf_enact_table,
    pub init_table: ta_pmf_init_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ta_pmf_output {
    pub policy_apply_table: ta_pmf_enact_result,
    pub rsvd: [u32; TA_OUTPUT_RESERVED_MEM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_pmf_shared_memory {
    pub command_id: c_int,
    pub resp_id: c_int,
    pub pmf_result: u32,
    pub if_version: u32,
    pub pmf_output: ta_pmf_output,
    pub pmf_input: ta_pmf_input,
}

// Core Layer
extern "C" {
    pub fn apmf_acpi_init(pmf_dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn apmf_acpi_deinit(pmf_dev: *mut amd_pmf_dev);
}
extern "C" {
    pub fn is_apmf_func_supported(pdev: *mut amd_pmf_dev, index: c_ulong) -> c_int;
}
extern "C" {
    pub fn amd_pmf_send_cmd(dev: *mut amd_pmf_dev, message: u8, get: bool, arg: u32, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn amd_pmf_init_metrics_table(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_get_power_source() -> c_int;
}
extern "C" {
    pub fn apmf_install_handler(pmf_dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn apmf_os_power_slider_update(dev: *mut amd_pmf_dev, flag: u8) -> c_int;
}
extern "C" {
    pub fn amd_pmf_set_dram_addr(dev: *mut amd_pmf_dev, alloc_buffer: bool) -> c_int;
}
extern "C" {
    pub fn amd_pmf_notify_sbios_heartbeat_event_v2(dev: *mut amd_pmf_dev, flag: u8) -> c_int;
}
extern "C" {
    pub fn fixp_q88_fromint(val: u32) -> u32;
}
extern "C" {
    pub fn is_apmf_bios_input_notifications_supported(pdev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_set_device(p_device: *mut device);
}
// Metrics layer
extern "C" {
    pub fn amd_pmf_get_tbl_dram_addr(dev: *mut amd_pmf_dev) -> c_int;
}
// SPS Layer
extern "C" {
    pub fn amd_pmf_get_pprof_modes(pmf: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_init_sps(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn is_pprof_balanced(pmf: *mut amd_pmf_dev) -> bool;
}
extern "C" {
    pub fn amd_pmf_power_slider_update_event(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn apmf_update_fan_idx(pdev: *mut amd_pmf_dev, manual: bool, idx: u32) -> c_int;
}
extern "C" {
    pub fn amd_pmf_set_sps_power_limits(pmf: *mut amd_pmf_dev) -> c_int;
}
// Auto Mode Layer
extern "C" {
    pub fn apmf_get_auto_mode_def(pdev: *mut amd_pmf_dev, data: *mut apmf_auto_mode) -> c_int;
}
extern "C" {
    pub fn amd_pmf_init_auto_mode(dev: *mut amd_pmf_dev);
}
extern "C" {
    pub fn amd_pmf_deinit_auto_mode(dev: *mut amd_pmf_dev);
}
extern "C" {
    pub fn amd_pmf_trans_automode(dev: *mut amd_pmf_dev, socket_power: c_int, time_elapsed_ms: ktime_t);
}
extern "C" {
    pub fn apmf_get_sbios_requests(pdev: *mut amd_pmf_dev, req: *mut apmf_sbios_req) -> c_int;
}
extern "C" {
    pub fn apmf_get_sbios_requests_v1(pdev: *mut amd_pmf_dev, req: *mut apmf_sbios_req_v1) -> c_int;
}
extern "C" {
    pub fn apmf_get_sbios_requests_v2(pdev: *mut amd_pmf_dev, req: *mut apmf_sbios_req_v2) -> c_int;
}
extern "C" {
    pub fn amd_pmf_update_2_cql(dev: *mut amd_pmf_dev, is_cql_event: bool);
}
extern "C" {
    pub fn amd_pmf_reset_amt(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_handle_amt(dev: *mut amd_pmf_dev);
}
// CnQF Layer
extern "C" {
    pub fn apmf_get_dyn_slider_def_ac(pdev: *mut amd_pmf_dev, data: *mut apmf_dyn_slider_output) -> c_int;
}
extern "C" {
    pub fn apmf_get_dyn_slider_def_dc(pdev: *mut amd_pmf_dev, data: *mut apmf_dyn_slider_output) -> c_int;
}
extern "C" {
    pub fn amd_pmf_init_cnqf(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_deinit_cnqf(dev: *mut amd_pmf_dev);
}
extern "C" {
    pub fn amd_pmf_trans_cnqf(dev: *mut amd_pmf_dev, socket_power: c_int, time_lapsed_ms: ktime_t) -> c_int;
}
// Smart PC builder Layer
extern "C" {
    pub fn amd_pmf_init_smart_pc(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_deinit_smart_pc(dev: *mut amd_pmf_dev);
}
extern "C" {
    pub fn apmf_check_smart_pc(pmf_dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_smartpc_apply_bios_output(dev: *mut amd_pmf_dev, val: u32, preq: u32, idx: u32) -> c_int;
}
// Smart PC - TA interfaces
extern "C" {
    pub fn amd_pmf_populate_ta_inputs(dev: *mut amd_pmf_dev, in: *mut ta_pmf_enact_table);
}
extern "C" {
    pub fn amd_pmf_dump_ta_inputs(dev: *mut amd_pmf_dev, in: *mut ta_pmf_enact_table);
}
extern "C" {
    pub fn amd_pmf_invoke_cmd_enact(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_get_ta_custom_bios_inputs(in: *mut ta_pmf_enact_table, index: c_int) -> u32;
}
extern "C" {
    pub fn amd_pmf_tee_init(dev: *mut amd_pmf_dev, uuid: *const uuid_t) -> c_int;
}
extern "C" {
    pub fn amd_pmf_tee_deinit(dev: *mut amd_pmf_dev);
}
extern "C" {
    pub fn amd_pmf_start_policy_engine(dev: *mut amd_pmf_dev) -> c_int;
}
// Util Layer

extern "C" {
    pub fn amd_pmf_cdev_register(dev: *mut amd_pmf_dev) -> c_int;
}
extern "C" {
    pub fn amd_pmf_cdev_unregister();
}

