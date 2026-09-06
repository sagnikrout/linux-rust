//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu11_driver_if_navi10.h
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
// Be aware of that the version should be updated in
// smu_v11_0.h, maybe rename is also needed.
// #define SMU11_DRIVER_IF_VERSION 0x33
pub const PPTABLE_NV10_SMU_VERSION: c_int = 8;
pub const NUM_GFXCLK_DPM_LEVELS: c_int = 16;
pub const NUM_SMNCLK_DPM_LEVELS: c_int = 2;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const NUM_MP0CLK_DPM_LEVELS: c_int = 2;
pub const NUM_DCLK_DPM_LEVELS: c_int = 8;
pub const NUM_VCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DCEFCLK_DPM_LEVELS: c_int = 8;
pub const NUM_PHYCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DISPCLK_DPM_LEVELS: c_int = 8;
pub const NUM_PIXCLK_DPM_LEVELS: c_int = 8;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_MP1CLK_DPM_LEVELS: c_int = 2;
pub const NUM_LINK_LEVELS: c_int = 2;

// Gemini Modes

// Feature Control Defines
// DPM
pub const FEATURE_DPM_PREFETCHER_BIT: c_int = 0;
pub const FEATURE_DPM_GFXCLK_BIT: c_int = 1;
pub const FEATURE_DPM_GFX_PACE_BIT: c_int = 2;
pub const FEATURE_DPM_UCLK_BIT: c_int = 3;
pub const FEATURE_DPM_SOCCLK_BIT: c_int = 4;
pub const FEATURE_DPM_MP0CLK_BIT: c_int = 5;
pub const FEATURE_DPM_LINK_BIT: c_int = 6;
pub const FEATURE_DPM_DCEFCLK_BIT: c_int = 7;
pub const FEATURE_MEM_VDDCI_SCALING_BIT: c_int = 8;
pub const FEATURE_MEM_MVDD_SCALING_BIT: c_int = 9;
// Idle
pub const FEATURE_DS_GFXCLK_BIT: c_int = 10;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 11;
pub const FEATURE_DS_LCLK_BIT: c_int = 12;
pub const FEATURE_DS_DCEFCLK_BIT: c_int = 13;
pub const FEATURE_DS_UCLK_BIT: c_int = 14;
pub const FEATURE_GFX_ULV_BIT: c_int = 15;
pub const FEATURE_FW_DSTATE_BIT: c_int = 16;
pub const FEATURE_GFXOFF_BIT: c_int = 17;
pub const FEATURE_BACO_BIT: c_int = 18;
pub const FEATURE_VCN_PG_BIT: c_int = 19;
pub const FEATURE_JPEG_PG_BIT: c_int = 20;
pub const FEATURE_USB_PG_BIT: c_int = 21;
pub const FEATURE_RSMU_SMN_CG_BIT: c_int = 22;
// Throttler/Response
pub const FEATURE_PPT_BIT: c_int = 23;
pub const FEATURE_TDC_BIT: c_int = 24;
pub const FEATURE_GFX_EDC_BIT: c_int = 25;
pub const FEATURE_APCC_PLUS_BIT: c_int = 26;
pub const FEATURE_GTHR_BIT: c_int = 27;
pub const FEATURE_ACDC_BIT: c_int = 28;
pub const FEATURE_VR0HOT_BIT: c_int = 29;
pub const FEATURE_VR1HOT_BIT: c_int = 30;
pub const FEATURE_FW_CTF_BIT: c_int = 31;
pub const FEATURE_FAN_CONTROL_BIT: c_int = 32;
pub const FEATURE_THERMAL_BIT: c_int = 33;
pub const FEATURE_GFX_DCS_BIT: c_int = 34;
// VF
pub const FEATURE_RM_BIT: c_int = 35;
pub const FEATURE_LED_DISPLAY_BIT: c_int = 36;
// Other
pub const FEATURE_GFX_SS_BIT: c_int = 37;
pub const FEATURE_OUT_OF_BAND_MONITOR_BIT: c_int = 38;
pub const FEATURE_TEMP_DEPENDENT_VMIN_BIT: c_int = 39;
pub const FEATURE_MMHUB_PG_BIT: c_int = 40;
pub const FEATURE_ATHUB_PG_BIT: c_int = 41;
pub const FEATURE_APCC_DFLL_BIT: c_int = 42;
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
// Debug Overrides Bitmask
pub const DPM_OVERRIDE_DISABLE_SOCCLK_PID: c_uint = 0x00000001;
pub const DPM_OVERRIDE_DISABLE_UCLK_PID: c_uint = 0x00000002;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_VCN_SOCCLK: c_uint = 0x00000004;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_VCLK_SOCCLK: c_uint = 0x00000008;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_DCLK_SOCCLK: c_uint = 0x00000010;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_GFXCLK_SOCCLK: c_uint = 0x00000020;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_GFXCLK_UCLK: c_uint = 0x00000040;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_DCE_SOCCLK: c_uint = 0x00000080;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_MP0_SOCCLK: c_uint = 0x00000100;
pub const DPM_OVERRIDE_DISABLE_DFLL_PLL_SHUTDOWN: c_uint = 0x00000200;
pub const DPM_OVERRIDE_DISABLE_MEMORY_TEMPERATURE_READ: c_uint = 0x00000400;
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
pub const THROTTLER_TEMP_VR_MEM0_BIT: c_int = 5;
pub const THROTTLER_TEMP_VR_MEM1_BIT: c_int = 6;
pub const THROTTLER_TEMP_VR_SOC_BIT: c_int = 7;
pub const THROTTLER_TEMP_LIQUID0_BIT: c_int = 8;
pub const THROTTLER_TEMP_LIQUID1_BIT: c_int = 9;
pub const THROTTLER_TEMP_PLX_BIT: c_int = 10;
pub const THROTTLER_TEMP_SKIN_BIT: c_int = 11;
pub const THROTTLER_TDC_GFX_BIT: c_int = 12;
pub const THROTTLER_TDC_SOC_BIT: c_int = 13;
pub const THROTTLER_PPT0_BIT: c_int = 14;
pub const THROTTLER_PPT1_BIT: c_int = 15;
pub const THROTTLER_PPT2_BIT: c_int = 16;
pub const THROTTLER_PPT3_BIT: c_int = 17;
pub const THROTTLER_FIT_BIT: c_int = 18;
pub const THROTTLER_PPM_BIT: c_int = 19;
pub const THROTTLER_APCC_BIT: c_int = 20;
// FW DState Features Control Bits
pub const FW_DSTATE_SOC_ULV_BIT: c_int = 0;
pub const FW_DSTATE_G6_HSR_BIT: c_int = 1;
pub const FW_DSTATE_G6_PHY_VDDCI_OFF_BIT: c_int = 2;
pub const FW_DSTATE_MP0_DS_BIT: c_int = 3;
pub const FW_DSTATE_SMN_DS_BIT: c_int = 4;
pub const FW_DSTATE_MP1_DS_BIT: c_int = 5;
pub const FW_DSTATE_MP1_WHISPER_MODE_BIT: c_int = 6;
pub const FW_DSTATE_LIV_MIN_BIT: c_int = 7;
pub const FW_DSTATE_SOC_PLL_PWRDN_BIT: c_int = 8;

// I2C Interface
pub const NUM_I2C_CONTROLLERS: c_int = 8;
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
pub const MAX_SW_I2C_COMMANDS: c_int = 8;
pub const CMDCONFIG_STOP_BIT: c_int = 0;
pub const CMDCONFIG_RESTART_BIT: c_int = 1;

// D3HOT sequences
// THis is aligned with RSMU PGFSM Register Mapping
// This is aligned with RSMU PGFSM Register Mapping
// Only Clks that have DPM descriptors are listed here
// Out of band monitor status defines
// see SPEC //gpu/doc/soc_arch/spec/feature/SMBUS/SMBUS.xlsx
pub const POWER_MANAGER_CONTROLLER_NOT_RUNNING: c_int = 0;
pub const POWER_MANAGER_CONTROLLER_RUNNING: c_int = 1;
pub const POWER_MANAGER_CONTROLLER_BIT: c_int = 0;
pub const MAXIMUM_DPM_STATE_GFX_ENGINE_RESTRICTED_BIT: c_int = 8;
pub const GPU_DIE_TEMPERATURE_THROTTLING_BIT: c_int = 9;
pub const HBM_DIE_TEMPERATURE_THROTTLING_BIT: c_int = 10;
pub const TGP_THROTTLING_BIT: c_int = 11;
pub const PCC_THROTTLING_BIT: c_int = 12;
pub const HBM_TEMPERATURE_EXCEEDING_TEMPERATURE_LIMIT_BIT: c_int = 13;
pub const HBM_TEMPERATURE_EXCEEDING_MAX_MEMORY_TEMPERATURE_BIT: c_int = 14;

// This structure to be DMA to SMBUS Config register space

// SECTION: Feature Enablement
// SECTION: Infrastructure Limits
// SECTION: Throttler settings
// SECTION: FW DSTATE Settings
// SECTION: ULV Settings
// SECTION: Voltage Control Parameters
// SECTION: DPM Config 1
// SECTION: DPM Config 2
// GFXCLK DPM
// UCLK section
// Link DPM Settings
// GFXCLK Thermal DPM (formerly 'Boost' Settings)
// SECTION: Fan Control
// uint8_t      padding8_Fan[2];
// The following are AFC override parameters. Leave at 0 to use FW defaults.
// SECTION: AVFS
// Overrides
// SECTION: Advanced Options
// Total Power configuration, use defines from PwrConfig_e
// APCC Settings
// Temperature Dependent Vmin
// BTC Setting
// SECTION: Board Reserved
// SECTION: BOARD PARAMETERS
// I2C Control
// SVI2 Board Parameters
// Telemetry Settings
// GPIO Settings
// LED Display Settings
// GFXCLK PLL Spread Spectrum
// GFXCLK DFLL Spread Spectrum
// UCLK Spread Spectrum
// SOCCLK Spread Spectrum
// Total board power
// Mvdd Svi2 Div Ratio Setting
// Padding for MMHUB - do not modify this

// Time constant parameters for clock averages in ms
// Padding - ignore
pub const NUM_WM_RANGES: c_int = 4;
// Watermarks
// Workload bits
pub const WORKLOAD_PPLIB_DEFAULT_BIT: c_int = 0;
pub const WORKLOAD_PPLIB_FULL_SCREEN_3D_BIT: c_int = 1;
pub const WORKLOAD_PPLIB_POWER_SAVING_BIT: c_int = 2;
pub const WORKLOAD_PPLIB_VIDEO_BIT: c_int = 3;
pub const WORKLOAD_PPLIB_VR_BIT: c_int = 4;
pub const WORKLOAD_PPLIB_COMPUTE_BIT: c_int = 5;
pub const WORKLOAD_PPLIB_CUSTOM_BIT: c_int = 6;
pub const WORKLOAD_PPLIB_COUNT: c_int = 7;
// These defines are used with the following messages:
// SMC_MSG_TransferTableDram2Smu
// SMC_MSG_TransferTableSmu2Dram
// Table transfer status
pub const TABLE_TRANSFER_OK: c_uint = 0x0;
pub const TABLE_TRANSFER_FAILED: c_uint = 0xFF;
// Table types
pub const TABLE_PPTABLE: c_int = 0;
pub const TABLE_WATERMARKS: c_int = 1;
pub const TABLE_AVFS: c_int = 2;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 3;
pub const TABLE_AVFS_FUSE_OVERRIDE: c_int = 4;
pub const TABLE_PMSTATUSLOG: c_int = 5;
pub const TABLE_SMU_METRICS: c_int = 6;
pub const TABLE_DRIVER_SMU_CONFIG: c_int = 7;
pub const TABLE_ACTIVITY_MONITOR_COEFF: c_int = 8;
pub const TABLE_OVERDRIVE: c_int = 9;
pub const TABLE_I2C_COMMANDS: c_int = 10;
pub const TABLE_PACE: c_int = 11;
pub const TABLE_COUNT: c_int = 12;
// RLC Pace Table total number of levels
pub const RLC_PACE_TABLE_NUM_LEVELS: c_int = 16;
// These defines are used with the SMC_MSG_SetUclkFastSwitch message.
pub const UCLK_SWITCH_SLOW: c_int = 0;
pub const UCLK_SWITCH_FAST: c_int = 1;
