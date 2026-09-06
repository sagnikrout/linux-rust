//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,mmcc-msm8960.h
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
pub const MMSS_AHB_SRC: c_int = 0;
pub const FAB_AHB_CLK: c_int = 1;
pub const APU_AHB_CLK: c_int = 2;
pub const TV_ENC_AHB_CLK: c_int = 3;
pub const AMP_AHB_CLK: c_int = 4;
pub const DSI2_S_AHB_CLK: c_int = 5;
pub const JPEGD_AHB_CLK: c_int = 6;
pub const GFX2D0_AHB_CLK: c_int = 7;
pub const DSI_S_AHB_CLK: c_int = 8;
pub const DSI2_M_AHB_CLK: c_int = 9;
pub const VPE_AHB_CLK: c_int = 10;
pub const SMMU_AHB_CLK: c_int = 11;
pub const HDMI_M_AHB_CLK: c_int = 12;
pub const VFE_AHB_CLK: c_int = 13;
pub const ROT_AHB_CLK: c_int = 14;
pub const VCODEC_AHB_CLK: c_int = 15;
pub const MDP_AHB_CLK: c_int = 16;
pub const DSI_M_AHB_CLK: c_int = 17;
pub const CSI_AHB_CLK: c_int = 18;
pub const MMSS_IMEM_AHB_CLK: c_int = 19;
pub const IJPEG_AHB_CLK: c_int = 20;
pub const HDMI_S_AHB_CLK: c_int = 21;
pub const GFX3D_AHB_CLK: c_int = 22;
pub const GFX2D1_AHB_CLK: c_int = 23;
pub const MMSS_FPB_CLK: c_int = 24;
pub const MMSS_AXI_SRC: c_int = 25;
pub const MMSS_FAB_CORE: c_int = 26;
pub const FAB_MSP_AXI_CLK: c_int = 27;
pub const JPEGD_AXI_CLK: c_int = 28;
pub const GMEM_AXI_CLK: c_int = 29;
pub const MDP_AXI_CLK: c_int = 30;
pub const MMSS_IMEM_AXI_CLK: c_int = 31;
pub const IJPEG_AXI_CLK: c_int = 32;
pub const GFX3D_AXI_CLK: c_int = 33;
pub const VCODEC_AXI_CLK: c_int = 34;
pub const VFE_AXI_CLK: c_int = 35;
pub const VPE_AXI_CLK: c_int = 36;
pub const ROT_AXI_CLK: c_int = 37;
pub const VCODEC_AXI_A_CLK: c_int = 38;
pub const VCODEC_AXI_B_CLK: c_int = 39;
pub const MM_AXI_S3_FCLK: c_int = 40;
pub const MM_AXI_S2_FCLK: c_int = 41;
pub const MM_AXI_S1_FCLK: c_int = 42;
pub const MM_AXI_S0_FCLK: c_int = 43;
pub const MM_AXI_S2_CLK: c_int = 44;
pub const MM_AXI_S1_CLK: c_int = 45;
pub const MM_AXI_S0_CLK: c_int = 46;
pub const CSI0_SRC: c_int = 47;
pub const CSI0_CLK: c_int = 48;
pub const CSI0_PHY_CLK: c_int = 49;
pub const CSI1_SRC: c_int = 50;
pub const CSI1_CLK: c_int = 51;
pub const CSI1_PHY_CLK: c_int = 52;
pub const CSI2_SRC: c_int = 53;
pub const CSI2_CLK: c_int = 54;
pub const CSI2_PHY_CLK: c_int = 55;
pub const DSI_SRC: c_int = 56;
pub const DSI_CLK: c_int = 57;
pub const CSI_PIX_CLK: c_int = 58;
pub const CSI_RDI_CLK: c_int = 59;
pub const MDP_VSYNC_CLK: c_int = 60;
pub const HDMI_DIV_CLK: c_int = 61;
pub const HDMI_APP_CLK: c_int = 62;
pub const CSI_PIX1_CLK: c_int = 63;
pub const CSI_RDI2_CLK: c_int = 64;
pub const CSI_RDI1_CLK: c_int = 65;
pub const GFX2D0_SRC: c_int = 66;
pub const GFX2D0_CLK: c_int = 67;
pub const GFX2D1_SRC: c_int = 68;
pub const GFX2D1_CLK: c_int = 69;
pub const GFX3D_SRC: c_int = 70;
pub const GFX3D_CLK: c_int = 71;
pub const IJPEG_SRC: c_int = 72;
pub const IJPEG_CLK: c_int = 73;
pub const JPEGD_SRC: c_int = 74;
pub const JPEGD_CLK: c_int = 75;
pub const MDP_SRC: c_int = 76;
pub const MDP_CLK: c_int = 77;
pub const MDP_LUT_CLK: c_int = 78;
pub const DSI2_PIXEL_SRC: c_int = 79;
pub const DSI2_PIXEL_CLK: c_int = 80;
pub const DSI2_SRC: c_int = 81;
pub const DSI2_CLK: c_int = 82;
pub const DSI1_BYTE_SRC: c_int = 83;
pub const DSI1_BYTE_CLK: c_int = 84;
pub const DSI2_BYTE_SRC: c_int = 85;
pub const DSI2_BYTE_CLK: c_int = 86;
pub const DSI1_ESC_SRC: c_int = 87;
pub const DSI1_ESC_CLK: c_int = 88;
pub const DSI2_ESC_SRC: c_int = 89;
pub const DSI2_ESC_CLK: c_int = 90;
pub const ROT_SRC: c_int = 91;
pub const ROT_CLK: c_int = 92;
pub const TV_ENC_CLK: c_int = 93;
pub const TV_DAC_CLK: c_int = 94;
pub const HDMI_TV_CLK: c_int = 95;
pub const MDP_TV_CLK: c_int = 96;
pub const TV_SRC: c_int = 97;
pub const VCODEC_SRC: c_int = 98;
pub const VCODEC_CLK: c_int = 99;
pub const VFE_SRC: c_int = 100;
pub const VFE_CLK: c_int = 101;
pub const VFE_CSI_CLK: c_int = 102;
pub const VPE_SRC: c_int = 103;
pub const VPE_CLK: c_int = 104;
pub const DSI_PIXEL_SRC: c_int = 105;
pub const DSI_PIXEL_CLK: c_int = 106;
pub const CAMCLK0_SRC: c_int = 107;
pub const CAMCLK0_CLK: c_int = 108;
pub const CAMCLK1_SRC: c_int = 109;
pub const CAMCLK1_CLK: c_int = 110;
pub const CAMCLK2_SRC: c_int = 111;
pub const CAMCLK2_CLK: c_int = 112;
pub const CSIPHYTIMER_SRC: c_int = 113;
pub const CSIPHY2_TIMER_CLK: c_int = 114;
pub const CSIPHY1_TIMER_CLK: c_int = 115;
pub const CSIPHY0_TIMER_CLK: c_int = 116;
pub const PLL1: c_int = 117;
pub const PLL2: c_int = 118;
pub const RGB_TV_CLK: c_int = 119;
pub const NPL_TV_CLK: c_int = 120;
pub const VCAP_AHB_CLK: c_int = 121;
pub const VCAP_AXI_CLK: c_int = 122;
pub const VCAP_SRC: c_int = 123;
pub const VCAP_CLK: c_int = 124;
pub const VCAP_NPL_CLK: c_int = 125;
pub const PLL15: c_int = 126;
pub const DSI2_PIXEL_LVDS_SRC: c_int = 127;
pub const LVDS_CLK: c_int = 128;
