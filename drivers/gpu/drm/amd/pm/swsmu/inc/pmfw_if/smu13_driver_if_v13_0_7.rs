//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu13_driver_if_v13_0_7.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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
// PMFW TEAM: Always increment the interface version on any change to this file
pub const SMU13_0_7_DRIVER_IF_VERSION: c_uint = 0x35;
// Increment this version if SkuTable_t or BoardTable_t change
pub const PPTABLE_VERSION: c_uint = 0x27;
pub const NUM_GFXCLK_DPM_LEVELS: c_int = 16;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const NUM_MP0CLK_DPM_LEVELS: c_int = 2;
pub const NUM_DCLK_DPM_LEVELS: c_int = 8;
pub const NUM_VCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DISPCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DPPCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DPREFCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DCFCLK_DPM_LEVELS: c_int = 8;
pub const NUM_DTBCLK_DPM_LEVELS: c_int = 8;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_LINK_LEVELS: c_int = 3;
pub const NUM_FCLK_DPM_LEVELS: c_int = 8;
pub const NUM_OD_FAN_MAX_POINTS: c_int = 6;
// Feature Control Defines
pub const FEATURE_FW_DATA_READ_BIT: c_int = 0;
pub const FEATURE_DPM_GFXCLK_BIT: c_int = 1;
pub const FEATURE_DPM_GFX_POWER_OPTIMIZER_BIT: c_int = 2;
pub const FEATURE_DPM_UCLK_BIT: c_int = 3;
pub const FEATURE_DPM_FCLK_BIT: c_int = 4;
pub const FEATURE_DPM_SOCCLK_BIT: c_int = 5;
pub const FEATURE_DPM_MP0CLK_BIT: c_int = 6;
pub const FEATURE_DPM_LINK_BIT: c_int = 7;
pub const FEATURE_DPM_DCN_BIT: c_int = 8;
pub const FEATURE_VMEMP_SCALING_BIT: c_int = 9;
pub const FEATURE_VDDIO_MEM_SCALING_BIT: c_int = 10;
pub const FEATURE_DS_GFXCLK_BIT: c_int = 11;
pub const FEATURE_DS_SOCCLK_BIT: c_int = 12;
pub const FEATURE_DS_FCLK_BIT: c_int = 13;
pub const FEATURE_DS_LCLK_BIT: c_int = 14;
pub const FEATURE_DS_DCFCLK_BIT: c_int = 15;
pub const FEATURE_DS_UCLK_BIT: c_int = 16;
pub const FEATURE_GFX_ULV_BIT: c_int = 17;
pub const FEATURE_FW_DSTATE_BIT: c_int = 18;
pub const FEATURE_GFXOFF_BIT: c_int = 19;
pub const FEATURE_BACO_BIT: c_int = 20;
pub const FEATURE_MM_DPM_BIT: c_int = 21;
pub const FEATURE_SOC_MPCLK_DS_BIT: c_int = 22;
pub const FEATURE_BACO_MPCLK_DS_BIT: c_int = 23;
pub const FEATURE_THROTTLERS_BIT: c_int = 24;
pub const FEATURE_SMARTSHIFT_BIT: c_int = 25;
pub const FEATURE_GTHR_BIT: c_int = 26;
pub const FEATURE_ACDC_BIT: c_int = 27;
pub const FEATURE_VR0HOT_BIT: c_int = 28;
pub const FEATURE_FW_CTF_BIT: c_int = 29;
pub const FEATURE_FAN_CONTROL_BIT: c_int = 30;
pub const FEATURE_GFX_DCS_BIT: c_int = 31;
pub const FEATURE_GFX_READ_MARGIN_BIT: c_int = 32;
pub const FEATURE_LED_DISPLAY_BIT: c_int = 33;
pub const FEATURE_GFXCLK_SPREAD_SPECTRUM_BIT: c_int = 34;
pub const FEATURE_OUT_OF_BAND_MONITOR_BIT: c_int = 35;
pub const FEATURE_OPTIMIZED_VMIN_BIT: c_int = 36;
pub const FEATURE_GFX_IMU_BIT: c_int = 37;
pub const FEATURE_BOOT_TIME_CAL_BIT: c_int = 38;
pub const FEATURE_GFX_PCC_DFLL_BIT: c_int = 39;
pub const FEATURE_SOC_CG_BIT: c_int = 40;
pub const FEATURE_DF_CSTATE_BIT: c_int = 41;
pub const FEATURE_GFX_EDC_BIT: c_int = 42;
pub const FEATURE_BOOT_POWER_OPT_BIT: c_int = 43;
pub const FEATURE_CLOCK_POWER_DOWN_BYPASS_BIT: c_int = 44;
pub const FEATURE_DS_VCN_BIT: c_int = 45;
pub const FEATURE_BACO_CG_BIT: c_int = 46;
pub const FEATURE_MEM_TEMP_READ_BIT: c_int = 47;
pub const FEATURE_ATHUB_MMHUB_PG_BIT: c_int = 48;
pub const FEATURE_SOC_PCC_BIT: c_int = 49;
pub const FEATURE_EDC_PWRBRK_BIT: c_int = 50;
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
pub const ALLOWED_FEATURE_CTRL_DEFAULT: c_uint = 0xFFFFFFFFFFFFFFFFULL;

// For use with feature control messages
// Debug Overrides Bitmask
pub const DEBUG_OVERRIDE_DISABLE_VOLT_LINK_VCN_FCLK: c_uint = 0x00000001;
pub const DEBUG_OVERRIDE_DISABLE_VOLT_LINK_DCN_FCLK: c_uint = 0x00000002;
pub const DEBUG_OVERRIDE_DISABLE_VOLT_LINK_MP0_FCLK: c_uint = 0x00000004;
pub const DEBUG_OVERRIDE_DISABLE_VOLT_LINK_VCN_DCFCLK: c_uint = 0x00000008;
pub const DEBUG_OVERRIDE_DISABLE_FAST_FCLK_TIMER: c_uint = 0x00000010;
pub const DEBUG_OVERRIDE_DISABLE_VCN_PG: c_uint = 0x00000020;
pub const DEBUG_OVERRIDE_DISABLE_FMAX_VMAX: c_uint = 0x00000040;
pub const DEBUG_OVERRIDE_DISABLE_IMU_FW_CHECKS: c_uint = 0x00000080;
pub const DEBUG_OVERRIDE_DISABLE_D0i2_REENTRY_HSR_TIMER_CHECK: c_uint = 0x00000100;
pub const DEBUG_OVERRIDE_DISABLE_DFLL: c_uint = 0x00000200;
pub const DEBUG_OVERRIDE_ENABLE_RLC_VF_BRINGUP_MODE: c_uint = 0x00000400;
pub const DEBUG_OVERRIDE_DFLL_MASTER_MODE: c_uint = 0x00000800;
pub const DEBUG_OVERRIDE_ENABLE_PROFILING_MODE: c_uint = 0x00001000;
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
pub const THROTTLER_TEMP_EDGE_BIT: c_int = 0;
pub const THROTTLER_TEMP_HOTSPOT_BIT: c_int = 1;
pub const THROTTLER_TEMP_HOTSPOT_G_BIT: c_int = 2;
pub const THROTTLER_TEMP_HOTSPOT_M_BIT: c_int = 3;
pub const THROTTLER_TEMP_MEM_BIT: c_int = 4;
pub const THROTTLER_TEMP_VR_GFX_BIT: c_int = 5;
pub const THROTTLER_TEMP_VR_MEM0_BIT: c_int = 6;
pub const THROTTLER_TEMP_VR_MEM1_BIT: c_int = 7;
pub const THROTTLER_TEMP_VR_SOC_BIT: c_int = 8;
pub const THROTTLER_TEMP_VR_U_BIT: c_int = 9;
pub const THROTTLER_TEMP_LIQUID0_BIT: c_int = 10;
pub const THROTTLER_TEMP_LIQUID1_BIT: c_int = 11;
pub const THROTTLER_TEMP_PLX_BIT: c_int = 12;
pub const THROTTLER_TDC_GFX_BIT: c_int = 13;
pub const THROTTLER_TDC_SOC_BIT: c_int = 14;
pub const THROTTLER_TDC_U_BIT: c_int = 15;
pub const THROTTLER_PPT0_BIT: c_int = 16;
pub const THROTTLER_PPT1_BIT: c_int = 17;
pub const THROTTLER_PPT2_BIT: c_int = 18;
pub const THROTTLER_PPT3_BIT: c_int = 19;
pub const THROTTLER_FIT_BIT: c_int = 20;
pub const THROTTLER_GFX_APCC_PLUS_BIT: c_int = 21;
pub const THROTTLER_COUNT: c_int = 22;
// FW DState Features Control Bits
pub const FW_DSTATE_SOC_ULV_BIT: c_int = 0;
pub const FW_DSTATE_G6_HSR_BIT: c_int = 1;
pub const FW_DSTATE_G6_PHY_VMEMP_OFF_BIT: c_int = 2;
pub const FW_DSTATE_SMN_DS_BIT: c_int = 3;
pub const FW_DSTATE_MP1_WHISPER_MODE_BIT: c_int = 4;
pub const FW_DSTATE_SOC_LIV_MIN_BIT: c_int = 5;
pub const FW_DSTATE_SOC_PLL_PWRDN_BIT: c_int = 6;
pub const FW_DSTATE_MEM_PLL_PWRDN_BIT: c_int = 7;
pub const FW_DSTATE_MALL_ALLOC_BIT: c_int = 8;
pub const FW_DSTATE_MEM_PSI_BIT: c_int = 9;
pub const FW_DSTATE_HSR_NON_STROBE_BIT: c_int = 10;
pub const FW_DSTATE_MP0_ENTER_WFI_BIT: c_int = 11;
pub const FW_DSTATE_U_ULV_BIT: c_int = 12;
pub const FW_DSTATE_MALL_FLUSH_BIT: c_int = 13;
pub const FW_DSTATE_SOC_PSI_BIT: c_int = 14;
pub const FW_DSTATE_U_PSI_BIT: c_int = 15;
pub const FW_DSTATE_UCP_DS_BIT: c_int = 16;
pub const FW_DSTATE_CSRCLK_DS_BIT: c_int = 17;
pub const FW_DSTATE_MMHUB_INTERLOCK_BIT: c_int = 18;
pub const FW_DSTATE_D0i3_2_QUIET_FW_BIT: c_int = 19;
pub const FW_DSTATE_CLDO_PRG_BIT: c_int = 20;
pub const FW_DSTATE_DF_PLL_PWRDN_BIT: c_int = 21;
pub const FW_DSTATE_U_LOW_PWR_MODE_EN_BIT: c_int = 22;
pub const FW_DSTATE_GFX_PSI6_BIT: c_int = 23;
pub const FW_DSTATE_GFX_VR_PWR_STAGE_BIT: c_int = 24;
// LED Display Mask & Control Bits
pub const LED_DISPLAY_GFX_DPM_BIT: c_int = 0;
pub const LED_DISPLAY_PCIE_BIT: c_int = 1;
pub const LED_DISPLAY_ERROR_BIT: c_int = 2;
pub const MEM_TEMP_READ_OUT_OF_BAND_BIT: c_int = 0;
pub const MEM_TEMP_READ_IN_BAND_REFRESH_BIT: c_int = 1;
pub const MEM_TEMP_READ_IN_BAND_DUMMY_PSTATE_BIT: c_int = 2;
// I2C Interface
pub const NUM_I2C_CONTROLLERS: c_int = 8;
pub const I2C_CONTROLLER_ENABLED: c_int = 1;
pub const I2C_CONTROLLER_DISABLED: c_int = 0;
pub const MAX_SW_I2C_COMMANDS: c_int = 24;
pub const CMDCONFIG_STOP_BIT: c_int = 0;
pub const CMDCONFIG_RESTART_BIT: c_int = 1;

// D3HOT sequences
// This is aligned with RSMU PGFSM Register Mapping
// Only Clks that have DPM descriptors are listed here
pub const PP_NUM_RTAVFS_PWL_ZONES: c_int = 5;
// VBIOS or PPLIB configures telemetry slope and offset. Only slope expected to be set for SVI3
// Slope Q1.7, Offset Q1.2

pub const PP_OD_FEATURE_GFX_VF_CURVE_BIT: c_int = 0;
pub const PP_OD_FEATURE_PPT_BIT: c_int = 2;
pub const PP_OD_FEATURE_FAN_CURVE_BIT: c_int = 3;
pub const PP_OD_FEATURE_GFXCLK_BIT: c_int = 7;
pub const PP_OD_FEATURE_UCLK_BIT: c_int = 8;
pub const PP_OD_FEATURE_ZERO_FAN_BIT: c_int = 9;
pub const PP_OD_FEATURE_TEMPERATURE_BIT: c_int = 10;
pub const PP_OD_FEATURE_COUNT: c_int = 13;
// Voltage control
// Frequency changes
// PPT
// Fan control
// PPT
pub const INVALID_BOARD_GPIO: c_uint = 0xFF;
// PLL 0
// PLL 1
// PLL 2
// PLL 3
// PLL 4
// PLL 5
// PLL 6
// encoding will change depending on SVI2/SVI3
// SECTION: Version
// SECTION: Feature Control
// SECTION: Miscellaneous Configuration
// SECTION: Infrastructure Limits
// if set to 1, SocketPowerLimitAc and SocketPowerLimitDc will be interpreted as legacy programs(i.e absolute power). If 0, all except index 0 will be scalars
// relative index 0
// Per year normalized Vmax state failure rates (sum of the two domains divided by life time in years)
// Expected GFX Duty Cycle at Vmax.
// Expected SOC Duty Cycle at Vmax.
// This offset will be deducted from the controller output to before it goes through the SOC Vset limiter block.
// SECTION: Throttler settings
// SECTION: FW DSTATE Settings
// SECTION: Voltage Control Parameters
// Voltage Limits
// Vmin Optimizations
// This is a fixed/minimum VMIN aging degradation offset which is applied at T0. This reflects the minimum amount of aging already accounted for.
// Linear offset or GB term to account for mis-correlation between PSM and Vmin shift trends across parts.
// Scalar coefficient of the PSM aging degradation function
// Exponential coefficient of the PSM aging degradation function
// Scalar coefficient of the VMIN aging degradation function. Specified as worst case between hot and cold.
// Exponential coefficient of the VMIN aging degradation function. Specified as worst case between hot and cold.
// SECTION: DPM Configuration 1
// SECTION: DPM Configuration 2
// GFX Idle Power Settings
// GFX GPO
// GFX DCS
// UCLK section
// FCLK Section
// Link DPM Settings
// SECTION: Fan Control
// The following are AFC override parameters. Leave at 0 to use FW defaults.
// SECTION: VDD_GFX AVFS
// SECTION: VDD_SOC AVFS
// SECTION: Boot clock and voltage values
// SECTION: Driver Reported Clocks
// SECTION: Message Limits
// SECTION: OverDrive Limits
// SECTION: Advanced Options
// Section: Total Board Power idle vs active coefficients
// SECTION: Sku Reserved
// Padding for MMHUB - do not modify this
// SECTION: Version
// SECTION: I2C Control
// SECTION: SVI2 Board Parameters
// SECTION SVI3 Board Parameters
// SECTION: Voltage Regulator Settings
// SECTION: GPIO Settings
// LED Display Settings
// SECTION: Clock Spread Spectrum
// UCLK Spread Spectrum
// FCLK Spread Spectrum
// Section: Memory Config
// SECTION: UMC feature flags
// SECTION: Board Reserved
// SECTION: Structure Padding
// Padding for MMHUB - do not modify this

// Time constant parameters for clock averages in ms
// Padding - ignore
// metrics for D3hot entry/exit and driver ARM msgs
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
pub const WORKLOAD_PPLIB_WINDOW_3D_BIT: c_int = 7;
pub const WORKLOAD_PPLIB_COUNT: c_int = 8;
// These defines are used with the following messages:
// SMC_MSG_TransferTableDram2Smu
// SMC_MSG_TransferTableSmu2Dram
// Table transfer status
pub const TABLE_TRANSFER_OK: c_uint = 0x0;
pub const TABLE_TRANSFER_FAILED: c_uint = 0xFF;
pub const TABLE_TRANSFER_PENDING: c_uint = 0xAB;
// Table types
pub const TABLE_PPTABLE: c_int = 0;
pub const TABLE_COMBO_PPTABLE: c_int = 1;
pub const TABLE_WATERMARKS: c_int = 2;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 3;
pub const TABLE_PMSTATUSLOG: c_int = 4;
pub const TABLE_SMU_METRICS: c_int = 5;
pub const TABLE_DRIVER_SMU_CONFIG: c_int = 6;
pub const TABLE_ACTIVITY_MONITOR_COEFF: c_int = 7;
pub const TABLE_OVERDRIVE: c_int = 8;
pub const TABLE_I2C_COMMANDS: c_int = 9;
pub const TABLE_DRIVER_INFO: c_int = 10;
pub const TABLE_ECCINFO: c_int = 11;
pub const TABLE_WIFIBAND: c_int = 12;
pub const TABLE_COUNT: c_int = 13;
// IH Interupt ID
pub const IH_INTERRUPT_ID_TO_DRIVER: c_uint = 0xFE;
pub const IH_INTERRUPT_CONTEXT_ID_BACO: c_uint = 0x2;
pub const IH_INTERRUPT_CONTEXT_ID_AC: c_uint = 0x3;
pub const IH_INTERRUPT_CONTEXT_ID_DC: c_uint = 0x4;
pub const IH_INTERRUPT_CONTEXT_ID_AUDIO_D0: c_uint = 0x5;
pub const IH_INTERRUPT_CONTEXT_ID_AUDIO_D3: c_uint = 0x6;
pub const IH_INTERRUPT_CONTEXT_ID_THERMAL_THROTTLING: c_uint = 0x7;
pub const IH_INTERRUPT_CONTEXT_ID_FAN_ABNORMAL: c_uint = 0x8;
pub const IH_INTERRUPT_CONTEXT_ID_FAN_RECOVERY: c_uint = 0x9;
