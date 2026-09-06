//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,gcc-qcm2290.h
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
// Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
//
// GCC clocks
pub const GPLL0: c_int = 0;
pub const GPLL0_OUT_AUX2: c_int = 1;
pub const GPLL1: c_int = 2;
pub const GPLL10: c_int = 3;
pub const GPLL11: c_int = 4;
pub const GPLL3: c_int = 5;
pub const GPLL3_OUT_MAIN: c_int = 6;
pub const GPLL4: c_int = 7;
pub const GPLL5: c_int = 8;
pub const GPLL6: c_int = 9;
pub const GPLL6_OUT_MAIN: c_int = 10;
pub const GPLL7: c_int = 11;
pub const GPLL8: c_int = 12;
pub const GPLL8_OUT_MAIN: c_int = 13;
pub const GPLL9: c_int = 14;
pub const GPLL9_OUT_MAIN: c_int = 15;
pub const GCC_AHB2PHY_CSI_CLK: c_int = 16;
pub const GCC_AHB2PHY_USB_CLK: c_int = 17;
pub const GCC_APC_VS_CLK: c_int = 18;
pub const GCC_BIMC_GPU_AXI_CLK: c_int = 19;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 20;
pub const GCC_CAM_THROTTLE_NRT_CLK: c_int = 21;
pub const GCC_CAM_THROTTLE_RT_CLK: c_int = 22;
pub const GCC_CAMERA_AHB_CLK: c_int = 23;
pub const GCC_CAMERA_XO_CLK: c_int = 24;
pub const GCC_CAMSS_AXI_CLK: c_int = 25;
pub const GCC_CAMSS_AXI_CLK_SRC: c_int = 26;
pub const GCC_CAMSS_CAMNOC_ATB_CLK: c_int = 27;
pub const GCC_CAMSS_CAMNOC_NTS_XO_CLK: c_int = 28;
pub const GCC_CAMSS_CCI_0_CLK: c_int = 29;
pub const GCC_CAMSS_CCI_CLK_SRC: c_int = 30;
pub const GCC_CAMSS_CPHY_0_CLK: c_int = 31;
pub const GCC_CAMSS_CPHY_1_CLK: c_int = 32;
pub const GCC_CAMSS_CSI0PHYTIMER_CLK: c_int = 33;
pub const GCC_CAMSS_CSI0PHYTIMER_CLK_SRC: c_int = 34;
pub const GCC_CAMSS_CSI1PHYTIMER_CLK: c_int = 35;
pub const GCC_CAMSS_CSI1PHYTIMER_CLK_SRC: c_int = 36;
pub const GCC_CAMSS_MCLK0_CLK: c_int = 37;
pub const GCC_CAMSS_MCLK0_CLK_SRC: c_int = 38;
pub const GCC_CAMSS_MCLK1_CLK: c_int = 39;
pub const GCC_CAMSS_MCLK1_CLK_SRC: c_int = 40;
pub const GCC_CAMSS_MCLK2_CLK: c_int = 41;
pub const GCC_CAMSS_MCLK2_CLK_SRC: c_int = 42;
pub const GCC_CAMSS_MCLK3_CLK: c_int = 43;
pub const GCC_CAMSS_MCLK3_CLK_SRC: c_int = 44;
pub const GCC_CAMSS_NRT_AXI_CLK: c_int = 45;
pub const GCC_CAMSS_OPE_AHB_CLK: c_int = 46;
pub const GCC_CAMSS_OPE_AHB_CLK_SRC: c_int = 47;
pub const GCC_CAMSS_OPE_CLK: c_int = 48;
pub const GCC_CAMSS_OPE_CLK_SRC: c_int = 49;
pub const GCC_CAMSS_RT_AXI_CLK: c_int = 50;
pub const GCC_CAMSS_TFE_0_CLK: c_int = 51;
pub const GCC_CAMSS_TFE_0_CLK_SRC: c_int = 52;
pub const GCC_CAMSS_TFE_0_CPHY_RX_CLK: c_int = 53;
pub const GCC_CAMSS_TFE_0_CSID_CLK: c_int = 54;
pub const GCC_CAMSS_TFE_0_CSID_CLK_SRC: c_int = 55;
pub const GCC_CAMSS_TFE_1_CLK: c_int = 56;
pub const GCC_CAMSS_TFE_1_CLK_SRC: c_int = 57;
pub const GCC_CAMSS_TFE_1_CPHY_RX_CLK: c_int = 58;
pub const GCC_CAMSS_TFE_1_CSID_CLK: c_int = 59;
pub const GCC_CAMSS_TFE_1_CSID_CLK_SRC: c_int = 60;
pub const GCC_CAMSS_TFE_CPHY_RX_CLK_SRC: c_int = 61;
pub const GCC_CAMSS_TOP_AHB_CLK: c_int = 62;
pub const GCC_CAMSS_TOP_AHB_CLK_SRC: c_int = 63;
pub const GCC_CFG_NOC_USB3_PRIM_AXI_CLK: c_int = 64;
pub const GCC_CPUSS_AHB_CLK: c_int = 65;
pub const GCC_CPUSS_AHB_CLK_SRC: c_int = 66;
pub const GCC_CPUSS_AHB_POSTDIV_CLK_SRC: c_int = 67;
pub const GCC_CPUSS_GNOC_CLK: c_int = 68;
pub const GCC_CPUSS_THROTTLE_CORE_CLK: c_int = 69;
pub const GCC_CPUSS_THROTTLE_XO_CLK: c_int = 70;
pub const GCC_DISP_AHB_CLK: c_int = 71;
pub const GCC_DISP_GPLL0_CLK_SRC: c_int = 72;
pub const GCC_DISP_GPLL0_DIV_CLK_SRC: c_int = 73;
pub const GCC_DISP_HF_AXI_CLK: c_int = 74;
pub const GCC_DISP_THROTTLE_CORE_CLK: c_int = 75;
pub const GCC_DISP_XO_CLK: c_int = 76;
pub const GCC_GP1_CLK: c_int = 77;
pub const GCC_GP1_CLK_SRC: c_int = 78;
pub const GCC_GP2_CLK: c_int = 79;
pub const GCC_GP2_CLK_SRC: c_int = 80;
pub const GCC_GP3_CLK: c_int = 81;
pub const GCC_GP3_CLK_SRC: c_int = 82;
pub const GCC_GPU_CFG_AHB_CLK: c_int = 83;
pub const GCC_GPU_GPLL0_CLK_SRC: c_int = 84;
pub const GCC_GPU_GPLL0_DIV_CLK_SRC: c_int = 85;
pub const GCC_GPU_IREF_CLK: c_int = 86;
pub const GCC_GPU_MEMNOC_GFX_CLK: c_int = 87;
pub const GCC_GPU_SNOC_DVM_GFX_CLK: c_int = 88;
pub const GCC_GPU_THROTTLE_CORE_CLK: c_int = 89;
pub const GCC_GPU_THROTTLE_XO_CLK: c_int = 90;
pub const GCC_PDM2_CLK: c_int = 91;
pub const GCC_PDM2_CLK_SRC: c_int = 92;
pub const GCC_PDM_AHB_CLK: c_int = 93;
pub const GCC_PDM_XO4_CLK: c_int = 94;
pub const GCC_PWM0_XO512_CLK: c_int = 95;
pub const GCC_QMIP_CAMERA_NRT_AHB_CLK: c_int = 96;
pub const GCC_QMIP_CAMERA_RT_AHB_CLK: c_int = 97;
pub const GCC_QMIP_CPUSS_CFG_AHB_CLK: c_int = 98;
pub const GCC_QMIP_DISP_AHB_CLK: c_int = 99;
pub const GCC_QMIP_GPU_CFG_AHB_CLK: c_int = 100;
pub const GCC_QMIP_VIDEO_VCODEC_AHB_CLK: c_int = 101;
pub const GCC_QUPV3_WRAP0_CORE_2X_CLK: c_int = 102;
pub const GCC_QUPV3_WRAP0_CORE_CLK: c_int = 103;
pub const GCC_QUPV3_WRAP0_S0_CLK: c_int = 104;
pub const GCC_QUPV3_WRAP0_S0_CLK_SRC: c_int = 105;
pub const GCC_QUPV3_WRAP0_S1_CLK: c_int = 106;
pub const GCC_QUPV3_WRAP0_S1_CLK_SRC: c_int = 107;
pub const GCC_QUPV3_WRAP0_S2_CLK: c_int = 108;
pub const GCC_QUPV3_WRAP0_S2_CLK_SRC: c_int = 109;
pub const GCC_QUPV3_WRAP0_S3_CLK: c_int = 110;
pub const GCC_QUPV3_WRAP0_S3_CLK_SRC: c_int = 111;
pub const GCC_QUPV3_WRAP0_S4_CLK: c_int = 112;
pub const GCC_QUPV3_WRAP0_S4_CLK_SRC: c_int = 113;
pub const GCC_QUPV3_WRAP0_S5_CLK: c_int = 114;
pub const GCC_QUPV3_WRAP0_S5_CLK_SRC: c_int = 115;
pub const GCC_QUPV3_WRAP_0_M_AHB_CLK: c_int = 116;
pub const GCC_QUPV3_WRAP_0_S_AHB_CLK: c_int = 117;
pub const GCC_SDCC1_AHB_CLK: c_int = 118;
pub const GCC_SDCC1_APPS_CLK: c_int = 119;
pub const GCC_SDCC1_APPS_CLK_SRC: c_int = 120;
pub const GCC_SDCC1_ICE_CORE_CLK: c_int = 121;
pub const GCC_SDCC1_ICE_CORE_CLK_SRC: c_int = 122;
pub const GCC_SDCC2_AHB_CLK: c_int = 123;
pub const GCC_SDCC2_APPS_CLK: c_int = 124;
pub const GCC_SDCC2_APPS_CLK_SRC: c_int = 125;
pub const GCC_SYS_NOC_CPUSS_AHB_CLK: c_int = 126;
pub const GCC_SYS_NOC_USB3_PRIM_AXI_CLK: c_int = 127;
pub const GCC_USB30_PRIM_MASTER_CLK: c_int = 128;
pub const GCC_USB30_PRIM_MASTER_CLK_SRC: c_int = 129;
pub const GCC_USB30_PRIM_MOCK_UTMI_CLK: c_int = 130;
pub const GCC_USB30_PRIM_MOCK_UTMI_CLK_SRC: c_int = 131;
pub const GCC_USB30_PRIM_MOCK_UTMI_POSTDIV: c_int = 132;
pub const GCC_USB30_PRIM_SLEEP_CLK: c_int = 133;
pub const GCC_USB3_PRIM_CLKREF_CLK: c_int = 134;
pub const GCC_USB3_PRIM_PHY_AUX_CLK_SRC: c_int = 135;
pub const GCC_USB3_PRIM_PHY_COM_AUX_CLK: c_int = 136;
pub const GCC_USB3_PRIM_PHY_PIPE_CLK: c_int = 137;
pub const GCC_VCODEC0_AXI_CLK: c_int = 138;
pub const GCC_VENUS_AHB_CLK: c_int = 139;
pub const GCC_VENUS_CTL_AXI_CLK: c_int = 140;
pub const GCC_VIDEO_AHB_CLK: c_int = 141;
pub const GCC_VIDEO_AXI0_CLK: c_int = 142;
pub const GCC_VIDEO_THROTTLE_CORE_CLK: c_int = 143;
pub const GCC_VIDEO_VCODEC0_SYS_CLK: c_int = 144;
pub const GCC_VIDEO_VENUS_CLK_SRC: c_int = 145;
pub const GCC_VIDEO_VENUS_CTL_CLK: c_int = 146;
pub const GCC_VIDEO_XO_CLK: c_int = 147;
// GCC resets
pub const GCC_CAMSS_OPE_BCR: c_int = 0;
pub const GCC_CAMSS_TFE_BCR: c_int = 1;
pub const GCC_CAMSS_TOP_BCR: c_int = 2;
pub const GCC_GPU_BCR: c_int = 3;
pub const GCC_MMSS_BCR: c_int = 4;
pub const GCC_PDM_BCR: c_int = 5;
pub const GCC_QUPV3_WRAPPER_0_BCR: c_int = 6;
pub const GCC_SDCC1_BCR: c_int = 7;
pub const GCC_SDCC2_BCR: c_int = 8;
pub const GCC_USB30_PRIM_BCR: c_int = 9;
pub const GCC_USB_PHY_CFG_AHB2PHY_BCR: c_int = 10;
pub const GCC_VCODEC0_BCR: c_int = 11;
pub const GCC_VENUS_BCR: c_int = 12;
pub const GCC_VIDEO_INTERFACE_BCR: c_int = 13;
pub const GCC_QUSB2PHY_PRIM_BCR: c_int = 14;
pub const GCC_USB3_PHY_PRIM_SP0_BCR: c_int = 15;
pub const GCC_USB3PHY_PHY_PRIM_SP0_BCR: c_int = 16;
// Indexes for GDSCs
pub const GCC_CAMSS_TOP_GDSC: c_int = 0;
pub const GCC_USB30_PRIM_GDSC: c_int = 1;
pub const GCC_VCODEC0_GDSC: c_int = 2;
pub const GCC_VENUS_GDSC: c_int = 3;
pub const HLOS1_VOTE_TURING_MMU_TBU1_GDSC: c_int = 4;
pub const HLOS1_VOTE_TURING_MMU_TBU0_GDSC: c_int = 5;
pub const HLOS1_VOTE_MM_SNOC_MMU_TBU_RT_GDSC: c_int = 6;
pub const HLOS1_VOTE_MM_SNOC_MMU_TBU_NRT_GDSC: c_int = 7;
