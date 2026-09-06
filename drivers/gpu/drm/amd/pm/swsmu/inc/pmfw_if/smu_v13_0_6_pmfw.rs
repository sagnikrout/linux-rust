//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v13_0_6_pmfw.h
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
pub const NUM_VCLK_DPM_LEVELS: c_int = 4;
pub const NUM_DCLK_DPM_LEVELS: c_int = 4;
pub const NUM_SOCCLK_DPM_LEVELS: c_int = 4;
pub const NUM_LCLK_DPM_LEVELS: c_int = 4;
pub const NUM_UCLK_DPM_LEVELS: c_int = 4;
pub const NUM_FCLK_DPM_LEVELS: c_int = 4;
pub const NUM_XGMI_DPM_LEVELS: c_int = 2;
pub const NUM_CXL_BITRATES: c_int = 4;
pub const NUM_PCIE_BITRATES: c_int = 4;
pub const NUM_XGMI_BITRATES: c_int = 4;
pub const NUM_XGMI_WIDTHS: c_int = 3;
pub const NUM_SOC_P2S_TABLES: c_int = 3;
pub const NUM_TDP_GROUPS: c_int = 4;
// 0*/   FEATURE_DATA_CALCULATION            = 0,
// 1*/   FEATURE_DPM_CCLK                    = 1,
// 2*/   FEATURE_DPM_FCLK                    = 2,
// 3*/   FEATURE_DPM_GFXCLK                  = 3,
// 4*/   FEATURE_DPM_LCLK                    = 4,
// 5*/   FEATURE_DPM_SOCCLK                  = 5,
// 6*/   FEATURE_DPM_UCLK                    = 6,
// 7*/   FEATURE_DPM_VCN                     = 7,
// 8*/   FEATURE_DPM_XGMI                    = 8,
// 9*/   FEATURE_DS_FCLK                     = 9,
// 10*/  FEATURE_DS_GFXCLK                   = 10,
// 11*/  FEATURE_DS_LCLK                     = 11,
// 12*/  FEATURE_DS_MP0CLK                   = 12,
// 13*/  FEATURE_DS_MP1CLK                   = 13,
// 14*/  FEATURE_DS_MPIOCLK                  = 14,
// 15*/  FEATURE_DS_SOCCLK                   = 15,
// 16*/  FEATURE_DS_VCN                      = 16,
// 17*/  FEATURE_APCC_DFLL                   = 17,
// 18*/  FEATURE_APCC_PLUS                   = 18,
// 19*/  FEATURE_DF_CSTATE                   = 19,
// 20*/  FEATURE_CC6                         = 20,
// 21*/  FEATURE_PC6                         = 21,
// 22*/  FEATURE_CPPC                        = 22,
// 23*/  FEATURE_PPT                         = 23,
// 24*/  FEATURE_TDC                         = 24,
// 25*/  FEATURE_THERMAL                     = 25,
// 26*/  FEATURE_SOC_PCC                     = 26,
// 27*/  FEATURE_CCD_PCC                     = 27,
// 28*/  FEATURE_CCD_EDC                     = 28,
// 29*/  FEATURE_PROCHOT                     = 29,
// 30*/  FEATURE_DVO_CCLK                    = 30,
// 31*/  FEATURE_FDD_AID_HBM                 = 31,
// 32*/  FEATURE_FDD_AID_SOC                 = 32,
// 33*/  FEATURE_FDD_XCD_EDC                 = 33,
// 34*/  FEATURE_FDD_XCD_XVMIN               = 34,
// 35*/  FEATURE_FW_CTF                      = 35,
// 36*/  FEATURE_GFXOFF                      = 36,
// 37*/  FEATURE_SMU_CG                      = 37,
// 38*/  FEATURE_PSI7                        = 38,
// 39*/  FEATURE_CSTATE_BOOST                = 39,
// 40*/  FEATURE_XGMI_PER_LINK_PWR_DOWN      = 40,
// 41*/  FEATURE_CXL_QOS                     = 41,
// 42*/  FEATURE_SOC_DC_RTC                  = 42,
// 43*/  FEATURE_GFX_DC_RTC                  = 43,
// 44*/  FEATURE_DVM_MIN_PSM                 = 44,
// 45*/  FEATURE_PRC                         = 45,
// 46*/  NUM_FEATURES                        = 46
// enum for MPIO PCIe gen speed msgs
pub const SMU_METRICS_TABLE_VERSION: c_uint = 0x11;
// Unified metrics table for smu_v13_0_6
// TEMPERATURE
// POWER
// ENERGY
// FREQUENCY
// FREQUENCY RANGE
// XGMI
// ACTIVITY
// THROTTLERS
// New Items at end to maintain driver compatibility
// PSNs
// XGMI Data tranfser size
// PCIE BW Data and error count
// VCN/JPEG ACTIVITY
// PCIE LINK Speed and width
// PER XCD ACTIVITY
// PCIE BW Data and error count
// Total App Clock Counter
// Metrics table for smu_v13_0_6 APUS
// TEMPERATURE
// POWER
// ENERGY
// FREQUENCY
// FREQUENCY RANGE
// XGMI
// ACTIVITY
// THROTTLERS
// New Items at end to maintain driver compatibility
// PSNs
// XGMI Data tranfser size
// VCN/JPEG ACTIVITY
// Metrics table for smu_v13_0_12
// TEMPERATURE
// POWER
// ENERGY
// FREQUENCY
// FREQUENCY RANGE
// XGMI
// ACTIVITY
// THROTTLERS
// New Items at end to maintain driver compatibility
// PSNs
// XGMI Data tranfser size
// PCIE BW Data and error count
// VCN/JPEG ACTIVITY
// PCIE LINK Speed and width
// PER XCD ACTIVITY
// PCIE BW Data and error count
// Total App Clock Counter
pub const SMU_VF_METRICS_TABLE_VERSION: c_uint = 0x5;

// Telemetry
// General info

