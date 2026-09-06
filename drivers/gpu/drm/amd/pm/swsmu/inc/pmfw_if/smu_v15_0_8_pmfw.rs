//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/inc/pmfw_if/smu_v15_0_8_pmfw.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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
pub const NUM_PCIE_BITRATES: c_int = 4;
pub const NUM_XGMI_BITRATES: c_int = 4;
pub const NUM_XGMI_WIDTHS: c_int = 3;
pub const NUM_GFX_P2S_TABLES: c_int = 8;
pub const NUM_PSM_DIDT_THRESHOLDS: c_int = 3;
pub const NUM_XCD_XVMIN_VMIN_THRESHOLDS: c_int = 3;
pub const PRODUCT_MODEL_NUMBER_LEN: c_int = 20;
pub const PRODUCT_NAME_LEN: c_int = 64;
pub const PRODUCT_SERIAL_LEN: c_int = 20;
pub const PRODUCT_MANUFACTURER_NAME_LEN: c_int = 32;
pub const PRODUCT_FRU_ID_LEN: c_int = 32;
// Feature ID list
pub const FEATURE_ID_DATA_CALCULATION: c_int = 1;
pub const FEATURE_ID_DPM_FCLK: c_int = 2;
pub const FEATURE_ID_DPM_GFXCLK: c_int = 3;
pub const FEATURE_ID_DPM_SPARE_4: c_int = 4;
pub const FEATURE_ID_DPM_SPARE_5: c_int = 5;
pub const FEATURE_ID_DPM_UCLK: c_int = 6;
pub const FEATURE_ID_DPM_SPARE_7: c_int = 7;
pub const FEATURE_ID_DPM_XGMI: c_int = 8;
pub const FEATURE_ID_DS_FCLK: c_int = 9;
pub const FEATURE_ID_DS_GFXCLK: c_int = 10;
pub const FEATURE_ID_DS_LCLK: c_int = 11;
pub const FEATURE_ID_DS_MP0CLK: c_int = 12;
pub const FEATURE_ID_DS_MP1CLK: c_int = 13;
pub const FEATURE_ID_DS_MPIOCLK: c_int = 14;
pub const FEATURE_ID_DS_SOCCLK: c_int = 15;
pub const FEATURE_ID_DS_VCN: c_int = 16;
pub const FEATURE_ID_PPT: c_int = 17;
pub const FEATURE_ID_TDC: c_int = 18;
pub const FEATURE_ID_THERMAL: c_int = 19;
pub const FEATURE_ID_SOC_PCC: c_int = 20;
pub const FEATURE_ID_PROCHOT: c_int = 21;
pub const FEATURE_ID_XVMIN0_VMIN_AID: c_int = 22;
pub const FEATURE_ID_XVMIN1_DD_AID: c_int = 23;
pub const FEATURE_ID_XVMIN0_VMIN_XCD: c_int = 24;
pub const FEATURE_ID_XVMIN1_DD_XCD: c_int = 25;
pub const FEATURE_ID_FW_CTF: c_int = 26;
pub const FEATURE_ID_MGCG: c_int = 27;
pub const FEATURE_ID_PSI7: c_int = 28;
pub const FEATURE_ID_XGMI_PER_LINK_PWR_DOWN: c_int = 29;
pub const FEATURE_ID_SOC_DC_RTC: c_int = 30;
pub const FEATURE_ID_GFX_DC_RTC: c_int = 31;
pub const FEATURE_ID_DVM_MIN_PSM: c_int = 32;
pub const FEATURE_ID_PRC: c_int = 33;
pub const FEATURE_ID_PSM_DIDT: c_int = 34;
pub const FEATURE_ID_PIT: c_int = 35;
pub const FEATURE_ID_DVO: c_int = 36;
pub const FEATURE_ID_XVMIN_CLKSTOP_DS: c_int = 37;
pub const FEATURE_ID_HBM_THROTTLE_CTRL: c_int = 38;
pub const FEATURE_ID_DPM_GL2CLK: c_int = 39;
pub const FEATURE_ID_GC_CAC_EDC: c_int = 40;
pub const FEATURE_ID_DS_DMABECLK: c_int = 41;
pub const FEATURE_ID_DS_MPIFOECLK: c_int = 42;
pub const FEATURE_ID_DS_MPRASCLK: c_int = 43;
pub const FEATURE_ID_DS_MPNHTCLK: c_int = 44;
pub const FEATURE_ID_DS_FIOCLK: c_int = 45;
pub const FEATURE_ID_DS_DXIOCLK: c_int = 46;
pub const FEATURE_ID_PCC: c_int = 47;
pub const FEATURE_ID_OCP: c_int = 48;
pub const FEATURE_ID_TRO: c_int = 49;
pub const FEATURE_ID_GL2_CAC_EDC: c_int = 50;
pub const FEATURE_ID_SPARE_51: c_int = 51;
pub const FEATURE_ID_GL2_CGCG: c_int = 52;
pub const FEATURE_ID_XCAC: c_int = 53;
pub const FEATURE_ID_DS_GL2CLK: c_int = 54;
pub const FEATURE_ID_FCS_VIN_PCC: c_int = 55;
pub const FEATURE_ID_FCS_VDDX_OCP_WARN: c_int = 56;
pub const FEATURE_ID_FCS_PWRBRK: c_int = 57;
pub const FEATURE_ID_DF_CSTATE: c_int = 58;
pub const FEATURE_ID_ARO: c_int = 59;
pub const FEATURE_ID_PS_PsPowerLimit: c_int = 60;
pub const FEATURE_ID_PS_PsPowerFloor: c_int = 61;
pub const FEATURE_ID_OCPWARNRC: c_int = 62;
pub const FEATURE_ID_XGMI_FOLDING: c_int = 63;
pub const FEATURE_ID_SMU_CG: c_int = 64;
pub const NUM_FEATURES: c_int = 65;
// MGCG Feature ID List
pub const WAFL_CG: c_int = 0;
pub const SMU_FUSE_CG_DEEPSLEEP: c_int = 1;
pub const SMUIO_CG: c_int = 2;
pub const RSMU_MGCG: c_int = 3;
pub const SMU_CLK_MGCG: c_int = 4;
pub const MP5_CG: c_int = 5;
pub const UMC_CG: c_int = 6;
pub const WAFL0_CLK: c_int = 7;
pub const WAFL1_CLK: c_int = 8;
pub const VCN_MGCG: c_int = 9;
pub const GL2_MGCG: c_int = 10;
pub const MGCG_NUM_FEATURES: c_int = 11;
// enum for MPIO PCIe gen speed msgs
pub const SMU_METRICS_TABLE_VERSION: c_uint = 0xF;
// TEMPERATURE
// POWER
// ENERGY
// FREQUENCY
// XGMI:
// ACTIVITY:
// THROTTLERS
// PCIE BW Data and error count
// VCN/JPEG ACTIVITY
// PCIE LINK Speed and width
// PER XCD ACTIVITY
// NVML-Parity: Total App Clock Counter
pub const SMU_SYSTEM_METRICS_TABLE_VERSION: c_uint = 0x1;

// NPM: NODE POWER MANAGEMENT

pub const SMU_VF_METRICS_TABLE_VERSION: c_uint = 0x5;
// FRU product information
pub const SMU_STATIC_METRICS_TABLE_VERSION: c_uint = 0x1;

// FRU PRODUCT INFO
// POWER
// FREQUENCY RANGE
// CTF limits
// Thermal Throttling limits
// PSNs
// XGMI
// Telemetry
// General info

