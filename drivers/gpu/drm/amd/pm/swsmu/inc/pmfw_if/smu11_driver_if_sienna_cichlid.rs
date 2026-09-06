//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu11_driver_if_sienna_cichlid.h
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
// *** IMPORTANT
// SMU TEAM: Always increment the interface version if
// any structure is changed in this file
pub const SMU11_DRIVER_IF_VERSION: c_uint = 0x40;
pub const PPTABLE_Sienna_Cichlid_SMU_VERSION: c_int = 7;
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
pub const NUM_DTBCLK_DPM_LEVELS: c_int = 8;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_MP1CLK_DPM_LEVELS: c_int = 2;
pub const NUM_LINK_LEVELS: c_int = 2;
pub const NUM_FCLK_DPM_LEVELS: c_int = 8;
pub const NUM_XGMI_LEVELS: c_int = 2;
pub const NUM_XGMI_PSTATE_LEVELS: c_int = 4;
pub const NUM_OD_FAN_MAX_POINTS: c_int = 6;

// Gemini Modes

// Feature Control Defines
// DPM
pub const FEATURE_DPM_PREFETCHER_BIT: c_int = 0;
pub const FEATURE_DPM_GFXCLK_BIT: c_int = 1;
pub const FEATURE_DPM_GFX_GPO_BIT: c_int = 2;
pub const FEATURE_DPM_UCLK_BIT: c_int = 3;
pub const FEATURE_DPM_FCLK_BIT: c_int = 4;
pub const FEATURE_DPM_SOCCLK_BIT: c_int = 5;
pub const FEATURE_DPM_MP0CLK_BIT: c_int = 6;
pub const FEATURE_DPM_LINK_BIT: c_int = 7;
pub const FEATURE_DPM_DCEFCLK_BIT: c_int = 8;
pub const FEATURE_DPM_XGMI_BIT: c_int = 9;
pub const FEATURE_MEM_VDDCI_SCALING_BIT: c_int = 10;
pub const FEATURE_MEM_MVDD_SCALING_BIT: c_int = 11;
// Idle
pub const FEATURE_DS_GFXCLK_BIT: c_int = 12;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 13;
pub const FEATURE_DS_FCLK_BIT: c_int = 14;
pub const FEATURE_DS_LCLK_BIT: c_int = 15;
pub const FEATURE_DS_DCEFCLK_BIT: c_int = 16;
pub const FEATURE_DS_UCLK_BIT: c_int = 17;
pub const FEATURE_GFX_ULV_BIT: c_int = 18;
pub const FEATURE_FW_DSTATE_BIT: c_int = 19;
pub const FEATURE_GFXOFF_BIT: c_int = 20;
pub const FEATURE_BACO_BIT: c_int = 21;
pub const FEATURE_MM_DPM_PG_BIT: c_int = 22;
pub const FEATURE_SPARE_23_BIT: c_int = 23;
// Throttler/Response
pub const FEATURE_PPT_BIT: c_int = 24;
pub const FEATURE_TDC_BIT: c_int = 25;
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
pub const FEATURE_DF_SUPERV_BIT: c_int = 43;
pub const FEATURE_RSMU_SMN_CG_BIT: c_int = 44;
pub const FEATURE_DF_CSTATE_BIT: c_int = 45;
pub const FEATURE_2_STEP_PSTATE_BIT: c_int = 46;
pub const FEATURE_SMNCLK_DPM_BIT: c_int = 47;
pub const FEATURE_PERLINK_GMIDOWN_BIT: c_int = 48;
pub const FEATURE_GFX_EDC_BIT: c_int = 49;
pub const FEATURE_GFX_PER_PART_VMIN_BIT: c_int = 50;
pub const FEATURE_SMART_SHIFT_BIT: c_int = 51;
pub const FEATURE_APT_BIT: c_int = 52;
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
// For use with feature control messages
// Debug Overrides Bitmask
pub const DPM_OVERRIDE_DISABLE_FCLK_PID: c_uint = 0x00000001;
pub const DPM_OVERRIDE_DISABLE_UCLK_PID: c_uint = 0x00000002;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_VCN_FCLK: c_uint = 0x00000004;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_VCLK_FCLK: c_uint = 0x00000008;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_DCLK_FCLK: c_uint = 0x00000010;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_GFXCLK_SOCCLK: c_uint = 0x00000020;
pub const DPM_OVERRIDE_ENABLE_FREQ_LINK_GFXCLK_UCLK: c_uint = 0x00000040;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_DCE_FCLK: c_uint = 0x00000080;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_MP0_SOCCLK: c_uint = 0x00000100;
pub const DPM_OVERRIDE_DISABLE_DFLL_PLL_SHUTDOWN: c_uint = 0x00000200;
pub const DPM_OVERRIDE_DISABLE_MEMORY_TEMPERATURE_READ: c_uint = 0x00000400;
pub const DPM_OVERRIDE_DISABLE_VOLT_LINK_VCN_DCEFCLK: c_uint = 0x00000800;
pub const DPM_OVERRIDE_DISABLE_FAST_FCLK_TIMER: c_uint = 0x00001000;
pub const DPM_OVERRIDE_DISABLE_VCN_PG: c_uint = 0x00002000;
pub const DPM_OVERRIDE_DISABLE_FMAX_VMAX: c_uint = 0x00004000;
pub const DPM_OVERRIDE_ENABLE_eGPU_USB_WA: c_uint = 0x00008000;
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
pub const THROTTLER_TDC_GFX_BIT: c_int = 11;
pub const THROTTLER_TDC_SOC_BIT: c_int = 12;
pub const THROTTLER_PPT0_BIT: c_int = 13;
pub const THROTTLER_PPT1_BIT: c_int = 14;
pub const THROTTLER_PPT2_BIT: c_int = 15;
pub const THROTTLER_PPT3_BIT: c_int = 16;
pub const THROTTLER_FIT_BIT: c_int = 17;
pub const THROTTLER_PPM_BIT: c_int = 18;
pub const THROTTLER_APCC_BIT: c_int = 19;
pub const THROTTLER_COUNT: c_int = 20;
// FW DState Features Control Bits
pub const FW_DSTATE_SOC_ULV_BIT: c_int = 0;
pub const FW_DSTATE_G6_HSR_BIT: c_int = 1;
pub const FW_DSTATE_G6_PHY_VDDCI_OFF_BIT: c_int = 2;
pub const FW_DSTATE_MP0_DS_BIT: c_int = 3;
pub const FW_DSTATE_SMN_DS_BIT: c_int = 4;
pub const FW_DSTATE_MP1_DS_BIT: c_int = 5;
pub const FW_DSTATE_MP1_WHISPER_MODE_BIT: c_int = 6;
pub const FW_DSTATE_SOC_LIV_MIN_BIT: c_int = 7;
pub const FW_DSTATE_SOC_PLL_PWRDN_BIT: c_int = 8;
pub const FW_DSTATE_MEM_PLL_PWRDN_BIT: c_int = 9;
pub const FW_DSTATE_OPTIMIZE_MALL_REFRESH_BIT: c_int = 10;
pub const FW_DSTATE_MEM_PSI_BIT: c_int = 11;
pub const FW_DSTATE_HSR_NON_STROBE_BIT: c_int = 12;
pub const FW_DSTATE_MP0_ENTER_WFI_BIT: c_int = 13;

// GFX GPO Feature Contains PACE and DEM sub features
pub const GFX_GPO_PACE_BIT: c_int = 0;
pub const GFX_GPO_DEM_BIT: c_int = 1;

pub const GPO_UPDATE_REQ_UCLKDPM_MASK: c_uint = 0x1;
pub const GPO_UPDATE_REQ_FCLKDPM_MASK: c_uint = 0x2;
pub const GPO_UPDATE_REQ_MALLHIT_MASK: c_uint = 0x4;
// LED Display Mask & Control Bits
pub const LED_DISPLAY_GFX_DPM_BIT: c_int = 0;
pub const LED_DISPLAY_PCIE_BIT: c_int = 1;
pub const LED_DISPLAY_ERROR_BIT: c_int = 2;
// RLC Pace Table total number of levels
pub const RLC_PACE_TABLE_NUM_LEVELS: c_int = 16;
pub const SIENNA_CICHLID_UMC_CHANNEL_NUM: c_int = 16;
// I2C Interface
pub const NUM_I2C_CONTROLLERS: c_int = 16;
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
pub const MAX_SW_I2C_COMMANDS: c_int = 24;
pub const CMDCONFIG_STOP_BIT: c_int = 0;
pub const CMDCONFIG_RESTART_BIT: c_int = 1;

// D3HOT sequences
// THis is aligned with RSMU PGFSM Register Mapping
// This is aligned with RSMU PGFSM Register Mapping
// Piecewise linear droop model, Sienna_Cichlid currently used only for GFX DFLL
pub const NUM_PIECE_WISE_LINEAR_DROOP_MODEL_VF_POINTS: c_int = 5;
// Only Clks that have DPM descriptors are listed here
// Used for 2-step UCLK DPM change workaround

// MAJOR SECTION: SKU PARAMETERS
// SECTION: Feature Enablement
// SECTION: Infrastructure Limits
// SECTION: Power Configuration
// SECTION: APCC Settings
// SECTION: SMNCLK DPM
// SECTION: Throttler settings
// SECTION: FW DSTATE Settings
// SECTION: ULV Settings
// SECTION: Voltage Control Parameters
// SECTION: Temperature Dependent Vmin
// SECTION: DPM Config 1
// Used for MALL performance boost
// SECTION: DPM Config 2
// GFXCLK DPM
// GFX GPO
// UCLK section
// Used for 2-Step UCLK change workaround
// Link DPM Settings
// SECTION: Fan Control
// The following are AFC override parameters. Leave at 0 to use FW defaults.
// SECTION: AVFS
// Overrides
// SECTION: XGMI
// SECTION: Advanced Options
// SECTION: Sku Reserved
// VC BTC parameters are only applicable to VDD_GFX domain
// GPIO Board feature
// MAJOR SECTION: BOARD PARAMETERS
// SECTION: Gaming Clocks
// SECTION: I2C Control
// SECTION: SVI2 Board Parameters
// SECTION: Telemetry Settings
// SECTION: GPIO Settings
// LED Display Settings
// SECTION: Clock Spread Spectrum
// GFXCLK PLL Spread Spectrum
// GFXCLK DFLL Spread Spectrum
// UCLK Spread Spectrum
// FCLK Spread Spectrum
// Section: Memory Config
// Section: Total Board Power
// SECTION: XGMI Training
// SECTION: UMC feature flags
// UCLK Spread Spectrum
// SECTION: Board Reserved
// SECTION: Structure Padding
// Padding for MMHUB - do not modify this

// MAJOR SECTION: SKU PARAMETERS
// SECTION: Feature Enablement
// SECTION: Infrastructure Limits
// SECTION: Power Configuration
// SECTION: APCC Settings
// SECTION: SMNCLK DPM
// SECTION: Throttler settings
// SECTION: FW DSTATE Settings
// SECTION: ULV Settings
// SECTION: Voltage Control Parameters
// SECTION: Temperature Dependent Vmin
// SECTION: DPM Config 1
// Used for MALL performance boost
// SECTION: DPM Config 2
// GFXCLK DPM
// GFX GPO
// UCLK section
// Used for 2-Step UCLK change workaround
// Link DPM Settings
// SECTION: Fan Control
// The following are AFC override parameters. Leave at 0 to use FW defaults.
// SECTION: AVFS
// Overrides
// SECTION: XGMI
// SECTION: Advanced Options
// SECTION: Sku Reserved
// VC BTC parameters are only applicable to VDD_GFX domain
// GPIO Board feature
// MAJOR SECTION: BOARD PARAMETERS
// SECTION: Gaming Clocks
// SECTION: I2C Control
// SECTION: SVI2 Board Parameters
// SECTION: Telemetry Settings
// SECTION: GPIO Settings
// LED Display Settings
// SECTION: Clock Spread Spectrum
// GFXCLK PLL Spread Spectrum
// GFXCLK DFLL Spread Spectrum
// UCLK Spread Spectrum
// FCLK Spread Spectrum
// Section: Memory Config
// Section: Total Board Power
// SECTION: XGMI Training
// SECTION: UMC feature flags
// UCLK Spread Spectrum
// SECTION: Board Reserved
// SECTION: Structure Padding
// Padding for MMHUB - do not modify this
// Time constant parameters for clock averages in ms
// Padding - ignore
// BACO metrics, PMFW-1721
// metrics for D3hot entry/exit and driver ARM msgs
// PMFW-4362
// BACO metrics, PMFW-1721
// metrics for D3hot entry/exit and driver ARM msgs
// PMFW-4362
// BACO metrics, PMFW-1721
// metrics for D3hot entry/exit and driver ARM msgs
// PMFW-4362
// BACO metrics, PMFW-1721
// metrics for D3hot entry/exit and driver ARM msgs
// PMFW-4362
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
pub const WORKLOAD_PPLIB_W3D_BIT: c_int = 7;
pub const WORKLOAD_PPLIB_COUNT: c_int = 8;
// These defines are used with the following messages:
// SMC_MSG_TransferTableDram2Smu
// SMC_MSG_TransferTableSmu2Dram
// Table transfer status
pub const TABLE_TRANSFER_OK: c_uint = 0x0;
pub const TABLE_TRANSFER_FAILED: c_uint = 0xFF;
// Table types
pub const TABLE_PPTABLE: c_int = 0;
pub const TABLE_WATERMARKS: c_int = 1;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 2;
pub const TABLE_AVFS_FUSE_OVERRIDE: c_int = 3;
pub const TABLE_PMSTATUSLOG: c_int = 4;
pub const TABLE_SMU_METRICS: c_int = 5;
pub const TABLE_DRIVER_SMU_CONFIG: c_int = 6;
pub const TABLE_ACTIVITY_MONITOR_COEFF: c_int = 7;
pub const TABLE_OVERDRIVE: c_int = 8;
pub const TABLE_I2C_COMMANDS: c_int = 9;
pub const TABLE_PACE: c_int = 10;
pub const TABLE_ECCINFO: c_int = 11;
pub const TABLE_COUNT: c_int = 12;
// These defines are used with the SMC_MSG_SetUclkFastSwitch message.
pub const UCLK_SWITCH_SLOW: c_int = 0;
pub const UCLK_SWITCH_FAST: c_int = 1;
pub const UCLK_SWITCH_DUMMY: c_int = 2;
