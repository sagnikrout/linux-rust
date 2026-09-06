//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/cix,sky1-s5-system-control.h
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
// Author: Jerry Zhu <jerry.zhu@cixtech.com>
// reset for csu_pm
pub const SKY1_CSU_PM_RESET_N: c_int = 0;
pub const SKY1_SENSORFUSION_RESET_N: c_int = 1;
pub const SKY1_SENSORFUSION_NOC_RESET_N: c_int = 2;
// reset group0 for s0 domain modules
pub const SKY1_DDRC_RESET_N: c_int = 3;
pub const SKY1_GIC_RESET_N: c_int = 4;
pub const SKY1_CI700_RESET_N: c_int = 5;
pub const SKY1_SYS_NI700_RESET_N: c_int = 6;
pub const SKY1_MM_NI700_RESET_N: c_int = 7;
pub const SKY1_PCIE_NI700_RESET_N: c_int = 8;
pub const SKY1_GPU_RESET_N: c_int = 9;
pub const SKY1_NPUTOP_RESET_N: c_int = 10;
pub const SKY1_NPUCORE0_RESET_N: c_int = 11;
pub const SKY1_NPUCORE1_RESET_N: c_int = 12;
pub const SKY1_NPUCORE2_RESET_N: c_int = 13;
pub const SKY1_VPU_RESET_N: c_int = 14;
pub const SKY1_ISP_SRESET_N: c_int = 15;
pub const SKY1_ISP_ARESET_N: c_int = 16;
pub const SKY1_ISP_HRESET_N: c_int = 17;
pub const SKY1_ISP_GDCRESET_N: c_int = 18;
pub const SKY1_DPU_RESET0_N: c_int = 19;
pub const SKY1_DPU_RESET1_N: c_int = 20;
pub const SKY1_DPU_RESET2_N: c_int = 21;
pub const SKY1_DPU_RESET3_N: c_int = 22;
pub const SKY1_DPU_RESET4_N: c_int = 23;
pub const SKY1_DP_RESET0_N: c_int = 24;
pub const SKY1_DP_RESET1_N: c_int = 25;
pub const SKY1_DP_RESET2_N: c_int = 26;
pub const SKY1_DP_RESET3_N: c_int = 27;
pub const SKY1_DP_RESET4_N: c_int = 28;
pub const SKY1_DP_PHY_RST_N: c_int = 29;
// reset group1 for s0 domain modules
pub const SKY1_AUDIO_HIFI5_RESET_N: c_int = 30;
pub const SKY1_AUDIO_HIFI5_NOC_RESET_N: c_int = 31;
pub const SKY1_CSIDPHY_PRST0_N: c_int = 32;
pub const SKY1_CSIDPHY_CMNRST0_N: c_int = 33;
pub const SKY1_CSI0_RST_N: c_int = 34;
pub const SKY1_CSIDPHY_PRST1_N: c_int = 35;
pub const SKY1_CSIDPHY_CMNRST1_N: c_int = 36;
pub const SKY1_CSI1_RST_N: c_int = 37;
pub const SKY1_CSI2_RST_N: c_int = 38;
pub const SKY1_CSI3_RST_N: c_int = 39;
pub const SKY1_CSIBRDGE0_RST_N: c_int = 40;
pub const SKY1_CSIBRDGE1_RST_N: c_int = 41;
pub const SKY1_CSIBRDGE2_RST_N: c_int = 42;
pub const SKY1_CSIBRDGE3_RST_N: c_int = 43;
pub const SKY1_GMAC0_RST_N: c_int = 44;
pub const SKY1_GMAC1_RST_N: c_int = 45;
pub const SKY1_PCIE0_RESET_N: c_int = 46;
pub const SKY1_PCIE1_RESET_N: c_int = 47;
pub const SKY1_PCIE2_RESET_N: c_int = 48;
pub const SKY1_PCIE3_RESET_N: c_int = 49;
pub const SKY1_PCIE4_RESET_N: c_int = 50;
// reset group1 for usb phys
pub const SKY1_USB_DP_PHY0_PRST_N: c_int = 51;
pub const SKY1_USB_DP_PHY1_PRST_N: c_int = 52;
pub const SKY1_USB_DP_PHY2_PRST_N: c_int = 53;
pub const SKY1_USB_DP_PHY3_PRST_N: c_int = 54;
pub const SKY1_USB_DP_PHY0_RST_N: c_int = 55;
pub const SKY1_USB_DP_PHY1_RST_N: c_int = 56;
pub const SKY1_USB_DP_PHY2_RST_N: c_int = 57;
pub const SKY1_USB_DP_PHY3_RST_N: c_int = 58;
pub const SKY1_USBPHY_SS_PST_N: c_int = 59;
pub const SKY1_USBPHY_SS_RST_N: c_int = 60;
pub const SKY1_USBPHY_HS0_PRST_N: c_int = 61;
pub const SKY1_USBPHY_HS1_PRST_N: c_int = 62;
pub const SKY1_USBPHY_HS2_PRST_N: c_int = 63;
pub const SKY1_USBPHY_HS3_PRST_N: c_int = 64;
pub const SKY1_USBPHY_HS4_PRST_N: c_int = 65;
pub const SKY1_USBPHY_HS5_PRST_N: c_int = 66;
pub const SKY1_USBPHY_HS6_PRST_N: c_int = 67;
pub const SKY1_USBPHY_HS7_PRST_N: c_int = 68;
pub const SKY1_USBPHY_HS8_PRST_N: c_int = 69;
pub const SKY1_USBPHY_HS9_PRST_N: c_int = 70;
// reset group1 for usb controllers
pub const SKY1_USBC_SS0_PRST_N: c_int = 71;
pub const SKY1_USBC_SS1_PRST_N: c_int = 72;
pub const SKY1_USBC_SS2_PRST_N: c_int = 73;
pub const SKY1_USBC_SS3_PRST_N: c_int = 74;
pub const SKY1_USBC_SS4_PRST_N: c_int = 75;
pub const SKY1_USBC_SS5_PRST_N: c_int = 76;
pub const SKY1_USBC_SS0_RST_N: c_int = 77;
pub const SKY1_USBC_SS1_RST_N: c_int = 78;
pub const SKY1_USBC_SS2_RST_N: c_int = 79;
pub const SKY1_USBC_SS3_RST_N: c_int = 80;
pub const SKY1_USBC_SS4_RST_N: c_int = 81;
pub const SKY1_USBC_SS5_RST_N: c_int = 82;
pub const SKY1_USBC_HS0_PRST_N: c_int = 83;
pub const SKY1_USBC_HS1_PRST_N: c_int = 84;
pub const SKY1_USBC_HS2_PRST_N: c_int = 85;
pub const SKY1_USBC_HS3_PRST_N: c_int = 86;
pub const SKY1_USBC_HS0_RST_N: c_int = 87;
pub const SKY1_USBC_HS1_RST_N: c_int = 88;
pub const SKY1_USBC_HS2_RST_N: c_int = 89;
pub const SKY1_USBC_HS3_RST_N: c_int = 90;
// reset group0 for rcsu
pub const SKY1_AUDIO_RCSU_RESET_N: c_int = 91;
pub const SKY1_CI700_RCSU_RESET_N: c_int = 92;
pub const SKY1_CSI_RCSU0_RESET_N: c_int = 93;
pub const SKY1_CSI_RCSU1_RESET_N: c_int = 94;
pub const SKY1_CSU_PM_RCSU_RESET_N: c_int = 95;
pub const SKY1_DDR_BROADCAST_RCSU_RESET_N: c_int = 96;
pub const SKY1_DDR_CTRL_RCSU_0_RESET_N: c_int = 97;
pub const SKY1_DDR_CTRL_RCSU_1_RESET_N: c_int = 98;
pub const SKY1_DDR_CTRL_RCSU_2_RESET_N: c_int = 99;
pub const SKY1_DDR_CTRL_RCSU_3_RESET_N: c_int = 100;
pub const SKY1_DDR_TZC400_RCSU_0_RESET_N: c_int = 101;
pub const SKY1_DDR_TZC400_RCSU_1_RESET_N: c_int = 102;
pub const SKY1_DDR_TZC400_RCSU_2_RESET_N: c_int = 103;
pub const SKY1_DDR_TZC400_RCSU_3_RESET_N: c_int = 104;
pub const SKY1_DP0_RCSU_RESET_N: c_int = 105;
pub const SKY1_DP1_RCSU_RESET_N: c_int = 106;
pub const SKY1_DP2_RCSU_RESET_N: c_int = 107;
pub const SKY1_DP3_RCSU_RESET_N: c_int = 108;
pub const SKY1_DP4_RCSU_RESET_N: c_int = 109;
pub const SKY1_DPU0_RCSU_RESET_N: c_int = 110;
pub const SKY1_DPU1_RCSU_RESET_N: c_int = 111;
pub const SKY1_DPU2_RCSU_RESET_N: c_int = 112;
pub const SKY1_DPU3_RCSU_RESET_N: c_int = 113;
pub const SKY1_DPU4_RCSU_RESET_N: c_int = 114;
pub const SKY1_DSU_RCSU_RESET_N: c_int = 115;
pub const SKY1_FCH_RCSU_RESET_N: c_int = 116;
pub const SKY1_GICD_RCSU_RESET_N: c_int = 117;
pub const SKY1_GMAC_RCSU_RESET_N: c_int = 118;
pub const SKY1_GPU_RCSU_RESET_N: c_int = 119;
pub const SKY1_ISP_RCSU0_RESET_N: c_int = 120;
pub const SKY1_ISP_RCSU1_RESET_N: c_int = 121;
pub const SKY1_NI700_MMHUB_RCSU_RESET_N: c_int = 122;
// reset group1 for rcsu
pub const SKY1_NPU_RCSU_RESET_N: c_int = 123;
pub const SKY1_NI700_PCIE_RCSU_RESET_N: c_int = 124;
pub const SKY1_PCIE_X421_RCSU_RESET_N: c_int = 125;
pub const SKY1_PCIE_X8_RCSU_RESET_N: c_int = 126;
pub const SKY1_SF_RCSU_RESET_N: c_int = 127;
pub const SKY1_RCSU_SMMU_MMHUB_RESET_N: c_int = 128;
pub const SKY1_RCSU_SMMU_PCIEHUB_RESET_N: c_int = 129;
pub const SKY1_RCSU_SYSHUB_RESET_N: c_int = 130;
pub const SKY1_NI700_SMN_RCSU_RESET_N: c_int = 131;
pub const SKY1_NI700_SYSHUB_RCSU_RESET_N: c_int = 132;
pub const SKY1_RCSU_USB2_HOST0_RESET_N: c_int = 133;
pub const SKY1_RCSU_USB2_HOST1_RESET_N: c_int = 134;
pub const SKY1_RCSU_USB2_HOST2_RESET_N: c_int = 135;
pub const SKY1_RCSU_USB2_HOST3_RESET_N: c_int = 136;
pub const SKY1_RCSU_USB3_TYPEA_DRD_RESET_N: c_int = 137;
pub const SKY1_RCSU_USB3_TYPEC_DRD_RESET_N: c_int = 138;
pub const SKY1_RCSU_USB3_TYPEC_HOST0_RESET_N: c_int = 139;
pub const SKY1_RCSU_USB3_TYPEC_HOST1_RESET_N: c_int = 140;
pub const SKY1_RCSU_USB3_TYPEC_HOST2_RESET_N: c_int = 141;
pub const SKY1_VPU_RCSU_RESET_N: c_int = 142;
