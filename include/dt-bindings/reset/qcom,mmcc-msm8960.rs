//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,mmcc-msm8960.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
//
pub const VPE_AXI_RESET: c_int = 0;
pub const IJPEG_AXI_RESET: c_int = 1;
pub const MPD_AXI_RESET: c_int = 2;
pub const VFE_AXI_RESET: c_int = 3;
pub const SP_AXI_RESET: c_int = 4;
pub const VCODEC_AXI_RESET: c_int = 5;
pub const ROT_AXI_RESET: c_int = 6;
pub const VCODEC_AXI_A_RESET: c_int = 7;
pub const VCODEC_AXI_B_RESET: c_int = 8;
pub const FAB_S3_AXI_RESET: c_int = 9;
pub const FAB_S2_AXI_RESET: c_int = 10;
pub const FAB_S1_AXI_RESET: c_int = 11;
pub const FAB_S0_AXI_RESET: c_int = 12;
pub const SMMU_GFX3D_ABH_RESET: c_int = 13;
pub const SMMU_VPE_AHB_RESET: c_int = 14;
pub const SMMU_VFE_AHB_RESET: c_int = 15;
pub const SMMU_ROT_AHB_RESET: c_int = 16;
pub const SMMU_VCODEC_B_AHB_RESET: c_int = 17;
pub const SMMU_VCODEC_A_AHB_RESET: c_int = 18;
pub const SMMU_MDP1_AHB_RESET: c_int = 19;
pub const SMMU_MDP0_AHB_RESET: c_int = 20;
pub const SMMU_JPEGD_AHB_RESET: c_int = 21;
pub const SMMU_IJPEG_AHB_RESET: c_int = 22;
pub const SMMU_GFX2D0_AHB_RESET: c_int = 23;
pub const SMMU_GFX2D1_AHB_RESET: c_int = 24;
pub const APU_AHB_RESET: c_int = 25;
pub const CSI_AHB_RESET: c_int = 26;
pub const TV_ENC_AHB_RESET: c_int = 27;
pub const VPE_AHB_RESET: c_int = 28;
pub const FABRIC_AHB_RESET: c_int = 29;
pub const GFX2D0_AHB_RESET: c_int = 30;
pub const GFX2D1_AHB_RESET: c_int = 31;
pub const GFX3D_AHB_RESET: c_int = 32;
pub const HDMI_AHB_RESET: c_int = 33;
pub const MSSS_IMEM_AHB_RESET: c_int = 34;
pub const IJPEG_AHB_RESET: c_int = 35;
pub const DSI_M_AHB_RESET: c_int = 36;
pub const DSI_S_AHB_RESET: c_int = 37;
pub const JPEGD_AHB_RESET: c_int = 38;
pub const MDP_AHB_RESET: c_int = 39;
pub const ROT_AHB_RESET: c_int = 40;
pub const VCODEC_AHB_RESET: c_int = 41;
pub const VFE_AHB_RESET: c_int = 42;
pub const DSI2_M_AHB_RESET: c_int = 43;
pub const DSI2_S_AHB_RESET: c_int = 44;
pub const CSIPHY2_RESET: c_int = 45;
pub const CSI_PIX1_RESET: c_int = 46;
pub const CSIPHY0_RESET: c_int = 47;
pub const CSIPHY1_RESET: c_int = 48;
pub const DSI2_RESET: c_int = 49;
pub const VFE_CSI_RESET: c_int = 50;
pub const MDP_RESET: c_int = 51;
pub const AMP_RESET: c_int = 52;
pub const JPEGD_RESET: c_int = 53;
pub const CSI1_RESET: c_int = 54;
pub const VPE_RESET: c_int = 55;
pub const MMSS_FABRIC_RESET: c_int = 56;
pub const VFE_RESET: c_int = 57;
pub const GFX2D0_RESET: c_int = 58;
pub const GFX2D1_RESET: c_int = 59;
pub const GFX3D_RESET: c_int = 60;
pub const HDMI_RESET: c_int = 61;
pub const MMSS_IMEM_RESET: c_int = 62;
pub const IJPEG_RESET: c_int = 63;
pub const CSI0_RESET: c_int = 64;
pub const DSI_RESET: c_int = 65;
pub const VCODEC_RESET: c_int = 66;
pub const MDP_TV_RESET: c_int = 67;
pub const MDP_VSYNC_RESET: c_int = 68;
pub const ROT_RESET: c_int = 69;
pub const TV_HDMI_RESET: c_int = 70;
pub const TV_ENC_RESET: c_int = 71;
pub const CSI2_RESET: c_int = 72;
pub const CSI_RDI1_RESET: c_int = 73;
pub const CSI_RDI2_RESET: c_int = 74;
pub const GFX3D_AXI_RESET: c_int = 75;
pub const VCAP_AXI_RESET: c_int = 76;
pub const SMMU_VCAP_AHB_RESET: c_int = 77;
pub const VCAP_AHB_RESET: c_int = 78;
pub const CSI_RDI_RESET: c_int = 79;
pub const CSI_PIX_RESET: c_int = 80;
pub const VCAP_NPL_RESET: c_int = 81;
pub const VCAP_RESET: c_int = 82;
