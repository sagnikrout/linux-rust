//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,qdu1000-gcc.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (c) 2021-2023, Qualcomm Innovation Center, Inc. All rights reserved.
//
// GCC clocks
pub const GCC_GPLL0: c_int = 0;
pub const GCC_GPLL0_OUT_EVEN: c_int = 1;
pub const GCC_GPLL1: c_int = 2;
pub const GCC_GPLL2: c_int = 3;
pub const GCC_GPLL2_OUT_EVEN: c_int = 4;
pub const GCC_GPLL3: c_int = 5;
pub const GCC_GPLL4: c_int = 6;
pub const GCC_GPLL5: c_int = 7;
pub const GCC_GPLL5_OUT_EVEN: c_int = 8;
pub const GCC_GPLL6: c_int = 9;
pub const GCC_GPLL7: c_int = 10;
pub const GCC_GPLL8: c_int = 11;
pub const GCC_AGGRE_NOC_ECPRI_DMA_CLK: c_int = 12;
pub const GCC_AGGRE_NOC_ECPRI_DMA_CLK_SRC: c_int = 13;
pub const GCC_AGGRE_NOC_ECPRI_GSI_CLK_SRC: c_int = 14;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 15;
pub const GCC_CFG_NOC_ECPRI_CC_AHB_CLK: c_int = 16;
pub const GCC_CFG_NOC_USB3_PRIM_AXI_CLK: c_int = 17;
pub const GCC_DDRSS_ECPRI_DMA_CLK: c_int = 18;
pub const GCC_ECPRI_AHB_CLK: c_int = 19;
pub const GCC_ECPRI_CC_GPLL0_CLK_SRC: c_int = 20;
pub const GCC_ECPRI_CC_GPLL1_EVEN_CLK_SRC: c_int = 21;
pub const GCC_ECPRI_CC_GPLL2_EVEN_CLK_SRC: c_int = 22;
pub const GCC_ECPRI_CC_GPLL3_CLK_SRC: c_int = 23;
pub const GCC_ECPRI_CC_GPLL4_CLK_SRC: c_int = 24;
pub const GCC_ECPRI_CC_GPLL5_EVEN_CLK_SRC: c_int = 25;
pub const GCC_ECPRI_XO_CLK: c_int = 26;
pub const GCC_ETH_DBG_SNOC_AXI_CLK: c_int = 27;
pub const GCC_GEMNOC_PCIE_QX_CLK: c_int = 28;
pub const GCC_GP1_CLK: c_int = 29;
pub const GCC_GP1_CLK_SRC: c_int = 30;
pub const GCC_GP2_CLK: c_int = 31;
pub const GCC_GP2_CLK_SRC: c_int = 32;
pub const GCC_GP3_CLK: c_int = 33;
pub const GCC_GP3_CLK_SRC: c_int = 34;
pub const GCC_PCIE_0_AUX_CLK: c_int = 35;
pub const GCC_PCIE_0_AUX_CLK_SRC: c_int = 36;
pub const GCC_PCIE_0_CFG_AHB_CLK: c_int = 37;
pub const GCC_PCIE_0_CLKREF_EN: c_int = 38;
pub const GCC_PCIE_0_MSTR_AXI_CLK: c_int = 39;
pub const GCC_PCIE_0_PHY_AUX_CLK: c_int = 40;
pub const GCC_PCIE_0_PHY_RCHNG_CLK: c_int = 41;
pub const GCC_PCIE_0_PHY_RCHNG_CLK_SRC: c_int = 42;
pub const GCC_PCIE_0_PIPE_CLK: c_int = 43;
pub const GCC_PCIE_0_SLV_AXI_CLK: c_int = 44;
pub const GCC_PCIE_0_SLV_Q2A_AXI_CLK: c_int = 45;
pub const GCC_PDM2_CLK: c_int = 46;
pub const GCC_PDM2_CLK_SRC: c_int = 47;
pub const GCC_PDM_AHB_CLK: c_int = 48;
pub const GCC_PDM_XO4_CLK: c_int = 49;
pub const GCC_QMIP_ANOC_PCIE_CLK: c_int = 50;
pub const GCC_QMIP_ECPRI_DMA0_CLK: c_int = 51;
pub const GCC_QMIP_ECPRI_DMA1_CLK: c_int = 52;
pub const GCC_QMIP_ECPRI_GSI_CLK: c_int = 53;
pub const GCC_QUPV3_WRAP0_CORE_2X_CLK: c_int = 54;
pub const GCC_QUPV3_WRAP0_CORE_CLK: c_int = 55;
pub const GCC_QUPV3_WRAP0_S0_CLK: c_int = 56;
pub const GCC_QUPV3_WRAP0_S0_CLK_SRC: c_int = 57;
pub const GCC_QUPV3_WRAP0_S1_CLK: c_int = 58;
pub const GCC_QUPV3_WRAP0_S1_CLK_SRC: c_int = 59;
pub const GCC_QUPV3_WRAP0_S2_CLK: c_int = 60;
pub const GCC_QUPV3_WRAP0_S2_CLK_SRC: c_int = 61;
pub const GCC_QUPV3_WRAP0_S3_CLK: c_int = 62;
pub const GCC_QUPV3_WRAP0_S3_CLK_SRC: c_int = 63;
pub const GCC_QUPV3_WRAP0_S4_CLK: c_int = 64;
pub const GCC_QUPV3_WRAP0_S4_CLK_SRC: c_int = 65;
pub const GCC_QUPV3_WRAP0_S5_CLK: c_int = 66;
pub const GCC_QUPV3_WRAP0_S5_CLK_SRC: c_int = 67;
pub const GCC_QUPV3_WRAP0_S6_CLK: c_int = 68;
pub const GCC_QUPV3_WRAP0_S6_CLK_SRC: c_int = 69;
pub const GCC_QUPV3_WRAP0_S7_CLK: c_int = 70;
pub const GCC_QUPV3_WRAP0_S7_CLK_SRC: c_int = 71;
pub const GCC_QUPV3_WRAP1_CORE_2X_CLK: c_int = 72;
pub const GCC_QUPV3_WRAP1_CORE_CLK: c_int = 73;
pub const GCC_QUPV3_WRAP1_S0_CLK: c_int = 74;
pub const GCC_QUPV3_WRAP1_S0_CLK_SRC: c_int = 75;
pub const GCC_QUPV3_WRAP1_S1_CLK: c_int = 76;
pub const GCC_QUPV3_WRAP1_S1_CLK_SRC: c_int = 77;
pub const GCC_QUPV3_WRAP1_S2_CLK: c_int = 78;
pub const GCC_QUPV3_WRAP1_S2_CLK_SRC: c_int = 79;
pub const GCC_QUPV3_WRAP1_S3_CLK: c_int = 80;
pub const GCC_QUPV3_WRAP1_S3_CLK_SRC: c_int = 81;
pub const GCC_QUPV3_WRAP1_S4_CLK: c_int = 82;
pub const GCC_QUPV3_WRAP1_S4_CLK_SRC: c_int = 83;
pub const GCC_QUPV3_WRAP1_S5_CLK: c_int = 84;
pub const GCC_QUPV3_WRAP1_S5_CLK_SRC: c_int = 85;
pub const GCC_QUPV3_WRAP1_S6_CLK: c_int = 86;
pub const GCC_QUPV3_WRAP1_S6_CLK_SRC: c_int = 87;
pub const GCC_QUPV3_WRAP1_S7_CLK: c_int = 88;
pub const GCC_QUPV3_WRAP1_S7_CLK_SRC: c_int = 89;
pub const GCC_QUPV3_WRAP_0_M_AHB_CLK: c_int = 90;
pub const GCC_QUPV3_WRAP_0_S_AHB_CLK: c_int = 91;
pub const GCC_QUPV3_WRAP_1_M_AHB_CLK: c_int = 92;
pub const GCC_QUPV3_WRAP_1_S_AHB_CLK: c_int = 93;
pub const GCC_SDCC5_AHB_CLK: c_int = 94;
pub const GCC_SDCC5_APPS_CLK: c_int = 95;
pub const GCC_SDCC5_APPS_CLK_SRC: c_int = 96;
pub const GCC_SDCC5_ICE_CORE_CLK: c_int = 97;
pub const GCC_SDCC5_ICE_CORE_CLK_SRC: c_int = 98;
pub const GCC_SNOC_CNOC_GEMNOC_PCIE_QX_CLK: c_int = 99;
pub const GCC_SNOC_CNOC_GEMNOC_PCIE_SOUTH_QX_CLK: c_int = 100;
pub const GCC_SNOC_CNOC_PCIE_QX_CLK: c_int = 101;
pub const GCC_SNOC_PCIE_SF_CENTER_QX_CLK: c_int = 102;
pub const GCC_SNOC_PCIE_SF_SOUTH_QX_CLK: c_int = 103;
pub const GCC_TSC_CFG_AHB_CLK: c_int = 104;
pub const GCC_TSC_CLK_SRC: c_int = 105;
pub const GCC_TSC_CNTR_CLK: c_int = 106;
pub const GCC_TSC_ETU_CLK: c_int = 107;
pub const GCC_USB2_CLKREF_EN: c_int = 108;
pub const GCC_USB30_PRIM_MASTER_CLK: c_int = 109;
pub const GCC_USB30_PRIM_MASTER_CLK_SRC: c_int = 110;
pub const GCC_USB30_PRIM_MOCK_UTMI_CLK: c_int = 111;
pub const GCC_USB30_PRIM_MOCK_UTMI_CLK_SRC: c_int = 112;
pub const GCC_USB30_PRIM_MOCK_UTMI_POSTDIV_CLK_SRC: c_int = 113;
pub const GCC_USB30_PRIM_SLEEP_CLK: c_int = 114;
pub const GCC_USB3_PRIM_PHY_AUX_CLK: c_int = 115;
pub const GCC_USB3_PRIM_PHY_AUX_CLK_SRC: c_int = 116;
pub const GCC_USB3_PRIM_PHY_COM_AUX_CLK: c_int = 117;
pub const GCC_USB3_PRIM_PHY_PIPE_CLK: c_int = 118;
pub const GCC_SM_BUS_AHB_CLK: c_int = 119;
pub const GCC_SM_BUS_XO_CLK: c_int = 120;
pub const GCC_SM_BUS_XO_CLK_SRC: c_int = 121;
pub const GCC_USB3_PRIM_PHY_PIPE_CLK_SRC: c_int = 122;
pub const GCC_ETH_100G_C2C_HM_APB_CLK: c_int = 123;
pub const GCC_ETH_100G_FH_HM_APB_0_CLK: c_int = 124;
pub const GCC_ETH_100G_FH_HM_APB_1_CLK: c_int = 125;
pub const GCC_ETH_100G_FH_HM_APB_2_CLK: c_int = 126;
pub const GCC_ETH_DBG_C2C_HM_APB_CLK: c_int = 127;
pub const GCC_AGGRE_NOC_ECPRI_GSI_CLK: c_int = 128;
pub const GCC_PCIE_0_PIPE_CLK_SRC: c_int = 129;
pub const GCC_PCIE_0_PHY_AUX_CLK_SRC: c_int = 130;
pub const GCC_GPLL1_OUT_EVEN: c_int = 131;
pub const GCC_DDRSS_ECPRI_GSI_CLK: c_int = 132;
// GCC resets
pub const GCC_ECPRI_CC_BCR: c_int = 0;
pub const GCC_ECPRI_SS_BCR: c_int = 1;
pub const GCC_ETH_WRAPPER_BCR: c_int = 2;
pub const GCC_PCIE_0_BCR: c_int = 3;
pub const GCC_PCIE_0_LINK_DOWN_BCR: c_int = 4;
pub const GCC_PCIE_0_NOCSR_COM_PHY_BCR: c_int = 5;
pub const GCC_PCIE_0_PHY_BCR: c_int = 6;
pub const GCC_PCIE_0_PHY_NOCSR_COM_PHY_BCR: c_int = 7;
pub const GCC_PCIE_PHY_CFG_AHB_BCR: c_int = 8;
pub const GCC_PCIE_PHY_COM_BCR: c_int = 9;
pub const GCC_PDM_BCR: c_int = 10;
pub const GCC_QUPV3_WRAPPER_0_BCR: c_int = 11;
pub const GCC_QUPV3_WRAPPER_1_BCR: c_int = 12;
pub const GCC_QUSB2PHY_PRIM_BCR: c_int = 13;
pub const GCC_QUSB2PHY_SEC_BCR: c_int = 14;
pub const GCC_SDCC5_BCR: c_int = 15;
pub const GCC_TCSR_PCIE_BCR: c_int = 16;
pub const GCC_TSC_BCR: c_int = 17;
pub const GCC_USB30_PRIM_BCR: c_int = 18;
pub const GCC_USB3_DP_PHY_PRIM_BCR: c_int = 19;
pub const GCC_USB3_DP_PHY_SEC_BCR: c_int = 20;
pub const GCC_USB3_PHY_PRIM_BCR: c_int = 21;
pub const GCC_USB3_PHY_SEC_BCR: c_int = 22;
pub const GCC_USB3PHY_PHY_PRIM_BCR: c_int = 23;
pub const GCC_USB3PHY_PHY_SEC_BCR: c_int = 24;
pub const GCC_USB_PHY_CFG_AHB2PHY_BCR: c_int = 25;
// GCC power domains
pub const PCIE_0_GDSC: c_int = 0;
pub const PCIE_0_PHY_GDSC: c_int = 1;
pub const USB30_PRIM_GDSC: c_int = 2;
