//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,sm6350-camcc.h
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
// Copyright (c) 2022, The Linux Foundation. All rights reserved.
// Copyright (c) 2022, Linaro Limited
//
// CAMCC clocks
pub const CAMCC_PLL2_OUT_EARLY: c_int = 0;
pub const CAMCC_PLL0: c_int = 1;
pub const CAMCC_PLL0_OUT_EVEN: c_int = 2;
pub const CAMCC_PLL1: c_int = 3;
pub const CAMCC_PLL1_OUT_EVEN: c_int = 4;
pub const CAMCC_PLL2: c_int = 5;
pub const CAMCC_PLL2_OUT_MAIN: c_int = 6;
pub const CAMCC_PLL3: c_int = 7;
pub const CAMCC_BPS_AHB_CLK: c_int = 8;
pub const CAMCC_BPS_AREG_CLK: c_int = 9;
pub const CAMCC_BPS_AXI_CLK: c_int = 10;
pub const CAMCC_BPS_CLK: c_int = 11;
pub const CAMCC_BPS_CLK_SRC: c_int = 12;
pub const CAMCC_CAMNOC_ATB_CLK: c_int = 13;
pub const CAMCC_CAMNOC_AXI_CLK: c_int = 14;
pub const CAMCC_CCI_0_CLK: c_int = 15;
pub const CAMCC_CCI_0_CLK_SRC: c_int = 16;
pub const CAMCC_CCI_1_CLK: c_int = 17;
pub const CAMCC_CCI_1_CLK_SRC: c_int = 18;
pub const CAMCC_CORE_AHB_CLK: c_int = 19;
pub const CAMCC_CPAS_AHB_CLK: c_int = 20;
pub const CAMCC_CPHY_RX_CLK_SRC: c_int = 21;
pub const CAMCC_CSI0PHYTIMER_CLK: c_int = 22;
pub const CAMCC_CSI0PHYTIMER_CLK_SRC: c_int = 23;
pub const CAMCC_CSI1PHYTIMER_CLK: c_int = 24;
pub const CAMCC_CSI1PHYTIMER_CLK_SRC: c_int = 25;
pub const CAMCC_CSI2PHYTIMER_CLK: c_int = 26;
pub const CAMCC_CSI2PHYTIMER_CLK_SRC: c_int = 27;
pub const CAMCC_CSI3PHYTIMER_CLK: c_int = 28;
pub const CAMCC_CSI3PHYTIMER_CLK_SRC: c_int = 29;
pub const CAMCC_CSIPHY0_CLK: c_int = 30;
pub const CAMCC_CSIPHY1_CLK: c_int = 31;
pub const CAMCC_CSIPHY2_CLK: c_int = 32;
pub const CAMCC_CSIPHY3_CLK: c_int = 33;
pub const CAMCC_FAST_AHB_CLK_SRC: c_int = 34;
pub const CAMCC_ICP_APB_CLK: c_int = 35;
pub const CAMCC_ICP_ATB_CLK: c_int = 36;
pub const CAMCC_ICP_CLK: c_int = 37;
pub const CAMCC_ICP_CLK_SRC: c_int = 38;
pub const CAMCC_ICP_CTI_CLK: c_int = 39;
pub const CAMCC_ICP_TS_CLK: c_int = 40;
pub const CAMCC_IFE_0_AXI_CLK: c_int = 41;
pub const CAMCC_IFE_0_CLK: c_int = 42;
pub const CAMCC_IFE_0_CLK_SRC: c_int = 43;
pub const CAMCC_IFE_0_CPHY_RX_CLK: c_int = 44;
pub const CAMCC_IFE_0_CSID_CLK: c_int = 45;
pub const CAMCC_IFE_0_CSID_CLK_SRC: c_int = 46;
pub const CAMCC_IFE_0_DSP_CLK: c_int = 47;
pub const CAMCC_IFE_1_AXI_CLK: c_int = 48;
pub const CAMCC_IFE_1_CLK: c_int = 49;
pub const CAMCC_IFE_1_CLK_SRC: c_int = 50;
pub const CAMCC_IFE_1_CPHY_RX_CLK: c_int = 51;
pub const CAMCC_IFE_1_CSID_CLK: c_int = 52;
pub const CAMCC_IFE_1_CSID_CLK_SRC: c_int = 53;
pub const CAMCC_IFE_1_DSP_CLK: c_int = 54;
pub const CAMCC_IFE_2_AXI_CLK: c_int = 55;
pub const CAMCC_IFE_2_CLK: c_int = 56;
pub const CAMCC_IFE_2_CLK_SRC: c_int = 57;
pub const CAMCC_IFE_2_CPHY_RX_CLK: c_int = 58;
pub const CAMCC_IFE_2_CSID_CLK: c_int = 59;
pub const CAMCC_IFE_2_CSID_CLK_SRC: c_int = 60;
pub const CAMCC_IFE_2_DSP_CLK: c_int = 61;
pub const CAMCC_IFE_LITE_CLK: c_int = 62;
pub const CAMCC_IFE_LITE_CLK_SRC: c_int = 63;
pub const CAMCC_IFE_LITE_CPHY_RX_CLK: c_int = 64;
pub const CAMCC_IFE_LITE_CSID_CLK: c_int = 65;
pub const CAMCC_IFE_LITE_CSID_CLK_SRC: c_int = 66;
pub const CAMCC_IPE_0_AHB_CLK: c_int = 67;
pub const CAMCC_IPE_0_AREG_CLK: c_int = 68;
pub const CAMCC_IPE_0_AXI_CLK: c_int = 69;
pub const CAMCC_IPE_0_CLK: c_int = 70;
pub const CAMCC_IPE_0_CLK_SRC: c_int = 71;
pub const CAMCC_JPEG_CLK: c_int = 72;
pub const CAMCC_JPEG_CLK_SRC: c_int = 73;
pub const CAMCC_LRME_CLK: c_int = 74;
pub const CAMCC_LRME_CLK_SRC: c_int = 75;
pub const CAMCC_MCLK0_CLK: c_int = 76;
pub const CAMCC_MCLK0_CLK_SRC: c_int = 77;
pub const CAMCC_MCLK1_CLK: c_int = 78;
pub const CAMCC_MCLK1_CLK_SRC: c_int = 79;
pub const CAMCC_MCLK2_CLK: c_int = 80;
pub const CAMCC_MCLK2_CLK_SRC: c_int = 81;
pub const CAMCC_MCLK3_CLK: c_int = 82;
pub const CAMCC_MCLK3_CLK_SRC: c_int = 83;
pub const CAMCC_MCLK4_CLK: c_int = 84;
pub const CAMCC_MCLK4_CLK_SRC: c_int = 85;
pub const CAMCC_SLOW_AHB_CLK_SRC: c_int = 86;
pub const CAMCC_SOC_AHB_CLK: c_int = 87;
pub const CAMCC_SYS_TMR_CLK: c_int = 88;
// GDSCs
pub const BPS_GDSC: c_int = 0;
pub const IPE_0_GDSC: c_int = 1;
pub const IFE_0_GDSC: c_int = 2;
pub const IFE_1_GDSC: c_int = 3;
pub const IFE_2_GDSC: c_int = 4;
pub const TITAN_TOP_GDSC: c_int = 5;
