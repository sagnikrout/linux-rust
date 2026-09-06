//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,sdx75-gcc.h
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
// Copyright (c) 2022-2023, Qualcomm Innovation Center, Inc. All rights reserved.
//
// GCC clocks
pub const GPLL0: c_int = 0;
pub const GPLL0_OUT_EVEN: c_int = 1;
pub const GPLL4: c_int = 2;
pub const GPLL5: c_int = 3;
pub const GPLL6: c_int = 4;
pub const GPLL8: c_int = 5;
pub const GCC_AHB_PCIE_LINK_CLK: c_int = 6;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 7;
pub const GCC_EEE_EMAC0_CLK: c_int = 8;
pub const GCC_EEE_EMAC0_CLK_SRC: c_int = 9;
pub const GCC_EEE_EMAC1_CLK: c_int = 10;
pub const GCC_EEE_EMAC1_CLK_SRC: c_int = 11;
pub const GCC_EMAC0_AXI_CLK: c_int = 12;
pub const GCC_EMAC0_CC_SGMIIPHY_RX_CLK: c_int = 13;
pub const GCC_EMAC0_CC_SGMIIPHY_RX_CLK_SRC: c_int = 14;
pub const GCC_EMAC0_CC_SGMIIPHY_TX_CLK: c_int = 15;
pub const GCC_EMAC0_CC_SGMIIPHY_TX_CLK_SRC: c_int = 16;
pub const GCC_EMAC0_PHY_AUX_CLK: c_int = 17;
pub const GCC_EMAC0_PHY_AUX_CLK_SRC: c_int = 18;
pub const GCC_EMAC0_PTP_CLK: c_int = 19;
pub const GCC_EMAC0_PTP_CLK_SRC: c_int = 20;
pub const GCC_EMAC0_RGMII_CLK: c_int = 21;
pub const GCC_EMAC0_RGMII_CLK_SRC: c_int = 22;
pub const GCC_EMAC0_RPCS_RX_CLK: c_int = 23;
pub const GCC_EMAC0_RPCS_TX_CLK: c_int = 24;
pub const GCC_EMAC0_SGMIIPHY_MAC_RCLK_SRC: c_int = 25;
pub const GCC_EMAC0_SGMIIPHY_MAC_TCLK_SRC: c_int = 26;
pub const GCC_EMAC0_SLV_AHB_CLK: c_int = 27;
pub const GCC_EMAC0_XGXS_RX_CLK: c_int = 28;
pub const GCC_EMAC0_XGXS_TX_CLK: c_int = 29;
pub const GCC_EMAC1_AXI_CLK: c_int = 30;
pub const GCC_EMAC1_CC_SGMIIPHY_RX_CLK: c_int = 31;
pub const GCC_EMAC1_CC_SGMIIPHY_RX_CLK_SRC: c_int = 32;
pub const GCC_EMAC1_CC_SGMIIPHY_TX_CLK: c_int = 33;
pub const GCC_EMAC1_CC_SGMIIPHY_TX_CLK_SRC: c_int = 34;
pub const GCC_EMAC1_PHY_AUX_CLK: c_int = 35;
pub const GCC_EMAC1_PHY_AUX_CLK_SRC: c_int = 36;
pub const GCC_EMAC1_PTP_CLK: c_int = 37;
pub const GCC_EMAC1_PTP_CLK_SRC: c_int = 38;
pub const GCC_EMAC1_RGMII_CLK: c_int = 39;
pub const GCC_EMAC1_RGMII_CLK_SRC: c_int = 40;
pub const GCC_EMAC1_RPCS_RX_CLK: c_int = 41;
pub const GCC_EMAC1_RPCS_TX_CLK: c_int = 42;
pub const GCC_EMAC1_SGMIIPHY_MAC_RCLK_SRC: c_int = 43;
pub const GCC_EMAC1_SGMIIPHY_MAC_TCLK_SRC: c_int = 44;
pub const GCC_EMAC1_SLV_AHB_CLK: c_int = 45;
pub const GCC_EMAC1_XGXS_RX_CLK: c_int = 46;
pub const GCC_EMAC1_XGXS_TX_CLK: c_int = 47;
pub const GCC_EMAC_0_CLKREF_EN: c_int = 48;
pub const GCC_EMAC_1_CLKREF_EN: c_int = 49;
pub const GCC_GP1_CLK: c_int = 50;
pub const GCC_GP1_CLK_SRC: c_int = 51;
pub const GCC_GP2_CLK: c_int = 52;
pub const GCC_GP2_CLK_SRC: c_int = 53;
pub const GCC_GP3_CLK: c_int = 54;
pub const GCC_GP3_CLK_SRC: c_int = 55;
pub const GCC_PCIE_0_CLKREF_EN: c_int = 56;
pub const GCC_PCIE_1_AUX_CLK: c_int = 57;
pub const GCC_PCIE_1_AUX_PHY_CLK_SRC: c_int = 58;
pub const GCC_PCIE_1_CFG_AHB_CLK: c_int = 59;
pub const GCC_PCIE_1_CLKREF_EN: c_int = 60;
pub const GCC_PCIE_1_MSTR_AXI_CLK: c_int = 61;
pub const GCC_PCIE_1_PHY_RCHNG_CLK: c_int = 62;
pub const GCC_PCIE_1_PHY_RCHNG_CLK_SRC: c_int = 63;
pub const GCC_PCIE_1_PIPE_CLK: c_int = 64;
pub const GCC_PCIE_1_PIPE_CLK_SRC: c_int = 65;
pub const GCC_PCIE_1_PIPE_DIV2_CLK: c_int = 66;
pub const GCC_PCIE_1_PIPE_DIV2_CLK_SRC: c_int = 67;
pub const GCC_PCIE_1_SLV_AXI_CLK: c_int = 68;
pub const GCC_PCIE_1_SLV_Q2A_AXI_CLK: c_int = 69;
pub const GCC_PCIE_2_AUX_CLK: c_int = 70;
pub const GCC_PCIE_2_AUX_PHY_CLK_SRC: c_int = 71;
pub const GCC_PCIE_2_CFG_AHB_CLK: c_int = 72;
pub const GCC_PCIE_2_CLKREF_EN: c_int = 73;
pub const GCC_PCIE_2_MSTR_AXI_CLK: c_int = 74;
pub const GCC_PCIE_2_PHY_RCHNG_CLK: c_int = 75;
pub const GCC_PCIE_2_PHY_RCHNG_CLK_SRC: c_int = 76;
pub const GCC_PCIE_2_PIPE_CLK: c_int = 77;
pub const GCC_PCIE_2_PIPE_CLK_SRC: c_int = 78;
pub const GCC_PCIE_2_PIPE_DIV2_CLK: c_int = 79;
pub const GCC_PCIE_2_PIPE_DIV2_CLK_SRC: c_int = 80;
pub const GCC_PCIE_2_SLV_AXI_CLK: c_int = 81;
pub const GCC_PCIE_2_SLV_Q2A_AXI_CLK: c_int = 82;
pub const GCC_PCIE_AUX_CLK: c_int = 83;
pub const GCC_PCIE_AUX_CLK_SRC: c_int = 84;
pub const GCC_PCIE_AUX_PHY_CLK_SRC: c_int = 85;
pub const GCC_PCIE_CFG_AHB_CLK: c_int = 86;
pub const GCC_PCIE_MSTR_AXI_CLK: c_int = 87;
pub const GCC_PCIE_PIPE_CLK: c_int = 88;
pub const GCC_PCIE_PIPE_CLK_SRC: c_int = 89;
pub const GCC_PCIE_RCHNG_PHY_CLK: c_int = 90;
pub const GCC_PCIE_RCHNG_PHY_CLK_SRC: c_int = 91;
pub const GCC_PCIE_SLEEP_CLK: c_int = 92;
pub const GCC_PCIE_SLV_AXI_CLK: c_int = 93;
pub const GCC_PCIE_SLV_Q2A_AXI_CLK: c_int = 94;
pub const GCC_PDM2_CLK: c_int = 95;
pub const GCC_PDM2_CLK_SRC: c_int = 96;
pub const GCC_PDM_AHB_CLK: c_int = 97;
pub const GCC_PDM_XO4_CLK: c_int = 98;
pub const GCC_QUPV3_WRAP0_CORE_2X_CLK: c_int = 99;
pub const GCC_QUPV3_WRAP0_CORE_CLK: c_int = 100;
pub const GCC_QUPV3_WRAP0_S0_CLK: c_int = 101;
pub const GCC_QUPV3_WRAP0_S0_CLK_SRC: c_int = 102;
pub const GCC_QUPV3_WRAP0_S1_CLK: c_int = 103;
pub const GCC_QUPV3_WRAP0_S1_CLK_SRC: c_int = 104;
pub const GCC_QUPV3_WRAP0_S2_CLK: c_int = 105;
pub const GCC_QUPV3_WRAP0_S2_CLK_SRC: c_int = 106;
pub const GCC_QUPV3_WRAP0_S3_CLK: c_int = 107;
pub const GCC_QUPV3_WRAP0_S3_CLK_SRC: c_int = 108;
pub const GCC_QUPV3_WRAP0_S4_CLK: c_int = 109;
pub const GCC_QUPV3_WRAP0_S4_CLK_SRC: c_int = 110;
pub const GCC_QUPV3_WRAP0_S5_CLK: c_int = 111;
pub const GCC_QUPV3_WRAP0_S5_CLK_SRC: c_int = 112;
pub const GCC_QUPV3_WRAP0_S6_CLK: c_int = 113;
pub const GCC_QUPV3_WRAP0_S6_CLK_SRC: c_int = 114;
pub const GCC_QUPV3_WRAP0_S7_CLK: c_int = 115;
pub const GCC_QUPV3_WRAP0_S7_CLK_SRC: c_int = 116;
pub const GCC_QUPV3_WRAP0_S8_CLK: c_int = 117;
pub const GCC_QUPV3_WRAP0_S8_CLK_SRC: c_int = 118;
pub const GCC_QUPV3_WRAP_0_M_AHB_CLK: c_int = 119;
pub const GCC_QUPV3_WRAP_0_S_AHB_CLK: c_int = 120;
pub const GCC_SDCC1_AHB_CLK: c_int = 121;
pub const GCC_SDCC1_APPS_CLK: c_int = 122;
pub const GCC_SDCC1_APPS_CLK_SRC: c_int = 123;
pub const GCC_SDCC2_AHB_CLK: c_int = 124;
pub const GCC_SDCC2_APPS_CLK: c_int = 125;
pub const GCC_SDCC2_APPS_CLK_SRC: c_int = 126;
pub const GCC_USB2_CLKREF_EN: c_int = 127;
pub const GCC_USB30_MASTER_CLK: c_int = 128;
pub const GCC_USB30_MASTER_CLK_SRC: c_int = 129;
pub const GCC_USB30_MOCK_UTMI_CLK: c_int = 130;
pub const GCC_USB30_MOCK_UTMI_CLK_SRC: c_int = 131;
pub const GCC_USB30_MOCK_UTMI_POSTDIV_CLK_SRC: c_int = 132;
pub const GCC_USB30_MSTR_AXI_CLK: c_int = 133;
pub const GCC_USB30_SLEEP_CLK: c_int = 134;
pub const GCC_USB30_SLV_AHB_CLK: c_int = 135;
pub const GCC_USB3_PHY_AUX_CLK: c_int = 136;
pub const GCC_USB3_PHY_AUX_CLK_SRC: c_int = 137;
pub const GCC_USB3_PHY_PIPE_CLK: c_int = 138;
pub const GCC_USB3_PHY_PIPE_CLK_SRC: c_int = 139;
pub const GCC_USB3_PRIM_CLKREF_EN: c_int = 140;
pub const GCC_USB_PHY_CFG_AHB2PHY_CLK: c_int = 141;
pub const GCC_XO_PCIE_LINK_CLK: c_int = 142;
// GCC power domains
pub const GCC_EMAC0_GDSC: c_int = 0;
pub const GCC_EMAC1_GDSC: c_int = 1;
pub const GCC_PCIE_1_GDSC: c_int = 2;
pub const GCC_PCIE_1_PHY_GDSC: c_int = 3;
pub const GCC_PCIE_2_GDSC: c_int = 4;
pub const GCC_PCIE_2_PHY_GDSC: c_int = 5;
pub const GCC_PCIE_GDSC: c_int = 6;
pub const GCC_PCIE_PHY_GDSC: c_int = 7;
pub const GCC_USB30_GDSC: c_int = 8;
pub const GCC_USB3_PHY_GDSC: c_int = 9;
// GCC resets
pub const GCC_EMAC0_BCR: c_int = 0;
pub const GCC_EMAC1_BCR: c_int = 1;
pub const GCC_EMMC_BCR: c_int = 2;
pub const GCC_PCIE_1_BCR: c_int = 3;
pub const GCC_PCIE_1_LINK_DOWN_BCR: c_int = 4;
pub const GCC_PCIE_1_NOCSR_COM_PHY_BCR: c_int = 5;
pub const GCC_PCIE_1_PHY_BCR: c_int = 6;
pub const GCC_PCIE_2_BCR: c_int = 7;
pub const GCC_PCIE_2_LINK_DOWN_BCR: c_int = 8;
pub const GCC_PCIE_2_NOCSR_COM_PHY_BCR: c_int = 9;
pub const GCC_PCIE_2_PHY_BCR: c_int = 10;
pub const GCC_PCIE_BCR: c_int = 11;
pub const GCC_PCIE_LINK_DOWN_BCR: c_int = 12;
pub const GCC_PCIE_NOCSR_COM_PHY_BCR: c_int = 13;
pub const GCC_PCIE_PHY_BCR: c_int = 14;
pub const GCC_PCIE_PHY_CFG_AHB_BCR: c_int = 15;
pub const GCC_PCIE_PHY_COM_BCR: c_int = 16;
pub const GCC_PCIE_PHY_NOCSR_COM_PHY_BCR: c_int = 17;
pub const GCC_QUSB2PHY_BCR: c_int = 18;
pub const GCC_TCSR_PCIE_BCR: c_int = 19;
pub const GCC_USB30_BCR: c_int = 20;
pub const GCC_USB3_PHY_BCR: c_int = 21;
pub const GCC_USB3PHY_PHY_BCR: c_int = 22;
pub const GCC_USB_PHY_CFG_AHB2PHY_BCR: c_int = 23;
pub const GCC_EMAC0_RGMII_CLK_ARES: c_int = 24;
