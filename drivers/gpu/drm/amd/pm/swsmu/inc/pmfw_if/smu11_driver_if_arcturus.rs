//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu11_driver_if_arcturus.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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
// *** IMPORTANT
// SMU TEAM: Always increment the interface version if
// any structure is changed in this file
// #define SMU11_DRIVER_IF_VERSION 0x09
pub const PPTABLE_ARCTURUS_SMU_VERSION: c_int = 4;
pub const NUM_GFXCLK_DPM_LEVELS: c_int = 16;
pub const NUM_VCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DCLK_DPM_LEVELS: c_int = 8;
pub const NUM_MP0CLK_DPM_LEVELS: c_int = 2;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_FCLK_DPM_LEVELS: c_int = 8;
pub const NUM_XGMI_LEVELS: c_int = 2;
pub const NUM_XGMI_PSTATE_LEVELS: c_int = 4;

// Feature Control Defines
// DPM
pub const FEATURE_DPM_PREFETCHER_BIT: c_int = 0;
pub const FEATURE_DPM_GFXCLK_BIT: c_int = 1;
pub const FEATURE_DPM_UCLK_BIT: c_int = 2;
pub const FEATURE_DPM_SOCCLK_BIT: c_int = 3;
pub const FEATURE_DPM_FCLK_BIT: c_int = 4;
pub const FEATURE_DPM_MP0CLK_BIT: c_int = 5;
pub const FEATURE_DPM_XGMI_BIT: c_int = 6;
// Idle
pub const FEATURE_DS_GFXCLK_BIT: c_int = 7;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 8;
pub const FEATURE_DS_LCLK_BIT: c_int = 9;
pub const FEATURE_DS_FCLK_BIT: c_int = 10;
pub const FEATURE_DS_UCLK_BIT: c_int = 11;
pub const FEATURE_GFX_ULV_BIT: c_int = 12;
pub const FEATURE_DPM_VCN_BIT: c_int = 13;
pub const FEATURE_RSMU_SMN_CG_BIT: c_int = 14;
pub const FEATURE_WAFL_CG_BIT: c_int = 15;
// Throttler/Response
pub const FEATURE_PPT_BIT: c_int = 16;
pub const FEATURE_TDC_BIT: c_int = 17;
pub const FEATURE_APCC_PLUS_BIT: c_int = 18;
pub const FEATURE_VR0HOT_BIT: c_int = 19;
pub const FEATURE_VR1HOT_BIT: c_int = 20;
pub const FEATURE_FW_CTF_BIT: c_int = 21;
pub const FEATURE_FAN_CONTROL_BIT: c_int = 22;
pub const FEATURE_THERMAL_BIT: c_int = 23;
// Other
pub const FEATURE_OUT_OF_BAND_MONITOR_BIT: c_int = 24;
pub const FEATURE_TEMP_DEPENDENT_VMIN_BIT: c_int = 25;
pub const FEATURE_PER_PART_VMIN_BIT: c_int = 26;
pub const FEATURE_SPARE_27_BIT: c_int = 27;
pub const FEATURE_SPARE_28_BIT: c_int = 28;
pub const FEATURE_SPARE_29_BIT: c_int = 29;
pub const FEATURE_SPARE_30_BIT: c_int = 30;
pub const FEATURE_SPARE_31_BIT: c_int = 31;
pub const FEATURE_SPARE_32_BIT: c_int = 32;
pub const FEATURE_SPARE_33_BIT: c_int = 33;
pub const FEATURE_SPARE_34_BIT: c_int = 34;
pub const FEATURE_SPARE_35_BIT: c_int = 35;
pub const FEATURE_SPARE_36_BIT: c_int = 36;
pub const FEATURE_SPARE_37_BIT: c_int = 37;
pub const FEATURE_SPARE_38_BIT: c_int = 38;
pub const FEATURE_SPARE_39_BIT: c_int = 39;
pub const FEATURE_SPARE_40_BIT: c_int = 40;
pub const FEATURE_SPARE_41_BIT: c_int = 41;
pub const FEATURE_SPARE_42_BIT: c_int = 42;
pub const FEATURE_SPARE_43_BIT: c_int = 43;
pub const FEATURE_SPARE_44_BIT: c_int = 44;
pub const FEATURE_SPARE_45_BIT: c_int = 45;
pub const FEATURE_SPARE_46_BIT: c_int = 46;
pub const FEATURE_SPARE_47_BIT: c_int = 47;
pub const FEATURE_SPARE_48_BIT: c_int = 48;
pub const FEATURE_SPARE_49_BIT: c_int = 49;
pub const FEATURE_SPARE_50_BIT: c_int = 50;
pub const FEATURE_SPARE_51_BIT: c_int = 51;
pub const FEATURE_SPARE_52_BIT: c_int = 52;
pub const FEATURE_SPARE_53_BIT: c_int = 53;
pub const FEATURE_SPARE_54_BIT: c_int = 54;
pub const FEATURE_SPARE_55_BIT: c_int = 55;
pub const FEATURE_SPARE_56_BIT: c_int = 56;
pub const FEATURE_SPARE_57_BIT: c_int = 57;
pub const FEATURE_SPARE_58_BIT: c_int = 58;
pub const FEATURE_SPARE_59_BIT: c_int = 59;
pub const FEATURE_SPARE_60_BIT: c_int = 60;
pub const FEATURE_SPARE_61_BIT: c_int = 61;
pub const FEATURE_SPARE_62_BIT: c_int = 62;
pub const FEATURE_SPARE_63_BIT: c_int = 63;
pub const NUM_FEATURES: c_int = 64;

// FIXME need updating
// Debug Overrides Bitmask
pub const DPM_OVERRIDE_DISABLE_UCLK_PID: c_uint = 0x00000001;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_VCN_FCLK: c_uint = 0x00000002;
// I2C Config Bit Defines
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
// VR Mapping Bit Defines
pub const VR_MAPPING_VR_SELECT_MASK: c_uint = 0x01;
pub const VR_MAPPING_VR_SELECT_SHIFT: c_uint = 0x00;
pub const VR_MAPPING_PLANE_SELECT_MASK: c_uint = 0x02;
pub const VR_MAPPING_PLANE_SELECT_SHIFT: c_uint = 0x01;
// PSI Bit Defines
pub const PSI_SEL_VR0_PLANE0_PSI0: c_uint = 0x01;
pub const PSI_SEL_VR0_PLANE0_PSI1: c_uint = 0x02;
pub const PSI_SEL_VR0_PLANE1_PSI0: c_uint = 0x04;
pub const PSI_SEL_VR0_PLANE1_PSI1: c_uint = 0x08;
pub const PSI_SEL_VR1_PLANE0_PSI0: c_uint = 0x10;
pub const PSI_SEL_VR1_PLANE0_PSI1: c_uint = 0x20;
pub const PSI_SEL_VR1_PLANE1_PSI0: c_uint = 0x40;
pub const PSI_SEL_VR1_PLANE1_PSI1: c_uint = 0x80;
// Throttler Control/Status Bits
pub const THROTTLER_PADDING_BIT: c_int = 0;
pub const THROTTLER_TEMP_EDGE_BIT: c_int = 1;
pub const THROTTLER_TEMP_HOTSPOT_BIT: c_int = 2;
pub const THROTTLER_TEMP_MEM_BIT: c_int = 3;
pub const THROTTLER_TEMP_VR_GFX_BIT: c_int = 4;
pub const THROTTLER_TEMP_VR_MEM_BIT: c_int = 5;
pub const THROTTLER_TEMP_VR_SOC_BIT: c_int = 6;
pub const THROTTLER_TDC_GFX_BIT: c_int = 7;
pub const THROTTLER_TDC_SOC_BIT: c_int = 8;
pub const THROTTLER_PPT0_BIT: c_int = 9;
pub const THROTTLER_PPT1_BIT: c_int = 10;
pub const THROTTLER_PPT2_BIT: c_int = 11;
pub const THROTTLER_PPT3_BIT: c_int = 12;
pub const THROTTLER_PPM_BIT: c_int = 13;
pub const THROTTLER_FIT_BIT: c_int = 14;
pub const THROTTLER_APCC_BIT: c_int = 15;
pub const THROTTLER_VRHOT0_BIT: c_int = 16;
pub const THROTTLER_VRHOT1_BIT: c_int = 17;
// Table transfer status
pub const TABLE_TRANSFER_OK: c_uint = 0x0;
pub const TABLE_TRANSFER_FAILED: c_uint = 0xFF;
pub const TABLE_TRANSFER_PENDING: c_uint = 0xAB;
// Workload bits
pub const WORKLOAD_PPLIB_DEFAULT_BIT: c_int = 0;
pub const WORKLOAD_PPLIB_POWER_SAVING_BIT: c_int = 1;
pub const WORKLOAD_PPLIB_VIDEO_BIT: c_int = 2;
pub const WORKLOAD_PPLIB_COMPUTE_BIT: c_int = 3;
pub const WORKLOAD_PPLIB_CUSTOM_BIT: c_int = 4;
pub const WORKLOAD_PPLIB_COUNT: c_int = 5;
// XGMI performance states
pub const XGMI_STATE_D0: c_int = 1;
pub const XGMI_STATE_D3: c_int = 0;
pub const NUM_I2C_CONTROLLERS: c_int = 8;
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
pub const MAX_SW_I2C_COMMANDS: c_int = 8;
pub const CMDCONFIG_STOP_BIT: c_int = 0;
pub const CMDCONFIG_RESTART_BIT: c_int = 1;

// D3HOT sequences
// THis is aligned with RSMU PGFSM Register Mapping
// This is aligned with RSMU PGFSM Register Mapping

// SECTION: Feature Enablement
// SECTION: Infrastructure Limits
// SECTION: Throttler settings
// SECTION: ULV Settings
// SECTION: Voltage Control Parameters
// SECTION: DPM Config 1
// SECTION: DPM Config 2
// GFXCLK DPM
// GFXCLK Thermal DPM (formerly 'Boost' Settings)
// SECTION: Fan Control
// The following are AFC override parameters. Leave at 0 to use FW defaults.
// SECTION: AVFS
// Overrides
// SECTION: XGMI
// Temperature Dependent Vmin
// SECTION: Advanced Options
// Total Power configuration, use defines from PwrConfig_e
// APCC Settings
// OOB Settings
// Per-Part Vmin
// SECTION: Reserved
// SECTION: BOARD PARAMETERS
// SVI2 Board Parameters
// Telemetry Settings
// GPIO Settings
// GFXCLK PLL Spread Spectrum
// UCLK Spread Spectrum
// FCLK Spread Spectrum
// GFXCLK Fll Spread Spectrum
// I2C Controller Structure
// Memory section
// Total board power
// SECTION: XGMI Training
// GPIO pins for I2C communications with 2nd controller for Input Telemetry Sequence
// Platform input telemetry voltage coefficient
// Padding for MMHUB - do not modify this

// Time constant parameters for clock averages in ms
// Padding - ignore
// These defines are used with the following messages:
// SMC_MSG_TransferTableDram2Smu
// SMC_MSG_TransferTableSmu2Dram
pub const TABLE_PPTABLE: c_int = 0;
pub const TABLE_AVFS: c_int = 1;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 2;
pub const TABLE_AVFS_FUSE_OVERRIDE: c_int = 3;
pub const TABLE_PMSTATUSLOG: c_int = 4;
pub const TABLE_SMU_METRICS: c_int = 5;
pub const TABLE_DRIVER_SMU_CONFIG: c_int = 6;
pub const TABLE_OVERDRIVE: c_int = 7;
pub const TABLE_WAFL_XGMI_TOPOLOGY: c_int = 8;
pub const TABLE_I2C_COMMANDS: c_int = 9;
pub const TABLE_ACTIVITY_MONITOR_COEFF: c_int = 10;
pub const TABLE_COUNT: c_int = 11;
// These defines are used with the SMC_MSG_SetUclkFastSwitch message.
pub const REMOVE_FMAX_MARGIN_BIT: c_uint = 0x0;
pub const REMOVE_DCTOL_MARGIN_BIT: c_uint = 0x1;
pub const REMOVE_PLATFORM_MARGIN_BIT: c_uint = 0x2;
