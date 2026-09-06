//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,sm8150-camcc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
//
// CAM_CC clocks
pub const CAM_CC_PLL0: c_int = 0;
pub const CAM_CC_PLL0_OUT_EVEN: c_int = 1;
pub const CAM_CC_PLL0_OUT_ODD: c_int = 2;
pub const CAM_CC_PLL1: c_int = 3;
pub const CAM_CC_PLL1_OUT_EVEN: c_int = 4;
pub const CAM_CC_PLL2: c_int = 5;
pub const CAM_CC_PLL2_OUT_MAIN: c_int = 6;
pub const CAM_CC_PLL3: c_int = 7;
pub const CAM_CC_PLL3_OUT_EVEN: c_int = 8;
pub const CAM_CC_PLL4: c_int = 9;
pub const CAM_CC_PLL4_OUT_EVEN: c_int = 10;
pub const CAM_CC_BPS_AHB_CLK: c_int = 11;
pub const CAM_CC_BPS_AREG_CLK: c_int = 12;
pub const CAM_CC_BPS_AXI_CLK: c_int = 13;
pub const CAM_CC_BPS_CLK: c_int = 14;
pub const CAM_CC_BPS_CLK_SRC: c_int = 15;
pub const CAM_CC_CAMNOC_AXI_CLK: c_int = 16;
pub const CAM_CC_CAMNOC_AXI_CLK_SRC: c_int = 17;
pub const CAM_CC_CAMNOC_DCD_XO_CLK: c_int = 18;
pub const CAM_CC_CCI_0_CLK: c_int = 19;
pub const CAM_CC_CCI_0_CLK_SRC: c_int = 20;
pub const CAM_CC_CCI_1_CLK: c_int = 21;
pub const CAM_CC_CCI_1_CLK_SRC: c_int = 22;
pub const CAM_CC_CORE_AHB_CLK: c_int = 23;
pub const CAM_CC_CPAS_AHB_CLK: c_int = 24;
pub const CAM_CC_CPHY_RX_CLK_SRC: c_int = 25;
pub const CAM_CC_CSI0PHYTIMER_CLK: c_int = 26;
pub const CAM_CC_CSI0PHYTIMER_CLK_SRC: c_int = 27;
pub const CAM_CC_CSI1PHYTIMER_CLK: c_int = 28;
pub const CAM_CC_CSI1PHYTIMER_CLK_SRC: c_int = 29;
pub const CAM_CC_CSI2PHYTIMER_CLK: c_int = 30;
pub const CAM_CC_CSI2PHYTIMER_CLK_SRC: c_int = 31;
pub const CAM_CC_CSI3PHYTIMER_CLK: c_int = 32;
pub const CAM_CC_CSI3PHYTIMER_CLK_SRC: c_int = 33;
pub const CAM_CC_CSIPHY0_CLK: c_int = 34;
pub const CAM_CC_CSIPHY1_CLK: c_int = 35;
pub const CAM_CC_CSIPHY2_CLK: c_int = 36;
pub const CAM_CC_CSIPHY3_CLK: c_int = 37;
pub const CAM_CC_FAST_AHB_CLK_SRC: c_int = 38;
pub const CAM_CC_FD_CORE_CLK: c_int = 39;
pub const CAM_CC_FD_CORE_CLK_SRC: c_int = 40;
pub const CAM_CC_FD_CORE_UAR_CLK: c_int = 41;
pub const CAM_CC_GDSC_CLK: c_int = 42;
pub const CAM_CC_ICP_AHB_CLK: c_int = 43;
pub const CAM_CC_ICP_CLK: c_int = 44;
pub const CAM_CC_ICP_CLK_SRC: c_int = 45;
pub const CAM_CC_IFE_0_AXI_CLK: c_int = 46;
pub const CAM_CC_IFE_0_CLK: c_int = 47;
pub const CAM_CC_IFE_0_CLK_SRC: c_int = 48;
pub const CAM_CC_IFE_0_CPHY_RX_CLK: c_int = 49;
pub const CAM_CC_IFE_0_CSID_CLK: c_int = 50;
pub const CAM_CC_IFE_0_CSID_CLK_SRC: c_int = 51;
pub const CAM_CC_IFE_0_DSP_CLK: c_int = 52;
pub const CAM_CC_IFE_1_AXI_CLK: c_int = 53;
pub const CAM_CC_IFE_1_CLK: c_int = 54;
pub const CAM_CC_IFE_1_CLK_SRC: c_int = 55;
pub const CAM_CC_IFE_1_CPHY_RX_CLK: c_int = 56;
pub const CAM_CC_IFE_1_CSID_CLK: c_int = 57;
pub const CAM_CC_IFE_1_CSID_CLK_SRC: c_int = 58;
pub const CAM_CC_IFE_1_DSP_CLK: c_int = 59;
pub const CAM_CC_IFE_LITE_0_CLK: c_int = 60;
pub const CAM_CC_IFE_LITE_0_CLK_SRC: c_int = 61;
pub const CAM_CC_IFE_LITE_0_CPHY_RX_CLK: c_int = 62;
pub const CAM_CC_IFE_LITE_0_CSID_CLK: c_int = 63;
pub const CAM_CC_IFE_LITE_0_CSID_CLK_SRC: c_int = 64;
pub const CAM_CC_IFE_LITE_1_CLK: c_int = 65;
pub const CAM_CC_IFE_LITE_1_CLK_SRC: c_int = 66;
pub const CAM_CC_IFE_LITE_1_CPHY_RX_CLK: c_int = 67;
pub const CAM_CC_IFE_LITE_1_CSID_CLK: c_int = 68;
pub const CAM_CC_IFE_LITE_1_CSID_CLK_SRC: c_int = 69;
pub const CAM_CC_IPE_0_AHB_CLK: c_int = 70;
pub const CAM_CC_IPE_0_AREG_CLK: c_int = 71;
pub const CAM_CC_IPE_0_AXI_CLK: c_int = 72;
pub const CAM_CC_IPE_0_CLK: c_int = 73;
pub const CAM_CC_IPE_0_CLK_SRC: c_int = 74;
pub const CAM_CC_IPE_1_AHB_CLK: c_int = 75;
pub const CAM_CC_IPE_1_AREG_CLK: c_int = 76;
pub const CAM_CC_IPE_1_AXI_CLK: c_int = 77;
pub const CAM_CC_IPE_1_CLK: c_int = 78;
pub const CAM_CC_JPEG_CLK: c_int = 79;
pub const CAM_CC_JPEG_CLK_SRC: c_int = 80;
pub const CAM_CC_LRME_CLK: c_int = 81;
pub const CAM_CC_LRME_CLK_SRC: c_int = 82;
pub const CAM_CC_MCLK0_CLK: c_int = 83;
pub const CAM_CC_MCLK0_CLK_SRC: c_int = 84;
pub const CAM_CC_MCLK1_CLK: c_int = 85;
pub const CAM_CC_MCLK1_CLK_SRC: c_int = 86;
pub const CAM_CC_MCLK2_CLK: c_int = 87;
pub const CAM_CC_MCLK2_CLK_SRC: c_int = 88;
pub const CAM_CC_MCLK3_CLK: c_int = 89;
pub const CAM_CC_MCLK3_CLK_SRC: c_int = 90;
pub const CAM_CC_SLOW_AHB_CLK_SRC: c_int = 91;
// CAM_CC power domains
pub const TITAN_TOP_GDSC: c_int = 0;
pub const BPS_GDSC: c_int = 1;
pub const IFE_0_GDSC: c_int = 2;
pub const IFE_1_GDSC: c_int = 3;
pub const IPE_0_GDSC: c_int = 4;
pub const IPE_1_GDSC: c_int = 5;
// CAM_CC resets
pub const CAM_CC_BPS_BCR: c_int = 0;
pub const CAM_CC_CAMNOC_BCR: c_int = 1;
pub const CAM_CC_CCI_BCR: c_int = 2;
pub const CAM_CC_CPAS_BCR: c_int = 3;
pub const CAM_CC_CSI0PHY_BCR: c_int = 4;
pub const CAM_CC_CSI1PHY_BCR: c_int = 5;
pub const CAM_CC_CSI2PHY_BCR: c_int = 6;
pub const CAM_CC_CSI3PHY_BCR: c_int = 7;
pub const CAM_CC_FD_BCR: c_int = 8;
pub const CAM_CC_ICP_BCR: c_int = 9;
pub const CAM_CC_IFE_0_BCR: c_int = 10;
pub const CAM_CC_IFE_1_BCR: c_int = 11;
pub const CAM_CC_IFE_LITE_0_BCR: c_int = 12;
pub const CAM_CC_IFE_LITE_1_BCR: c_int = 13;
pub const CAM_CC_IPE_0_BCR: c_int = 14;
pub const CAM_CC_IPE_1_BCR: c_int = 15;
pub const CAM_CC_JPEG_BCR: c_int = 16;
pub const CAM_CC_LRME_BCR: c_int = 17;
pub const CAM_CC_MCLK0_BCR: c_int = 18;
pub const CAM_CC_MCLK1_BCR: c_int = 19;
pub const CAM_CC_MCLK2_BCR: c_int = 20;
pub const CAM_CC_MCLK3_BCR: c_int = 21;
