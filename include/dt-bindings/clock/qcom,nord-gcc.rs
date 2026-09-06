//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,nord-gcc.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// GCC clocks
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 0;
pub const GCC_GP1_CLK: c_int = 1;
pub const GCC_GP1_CLK_SRC: c_int = 2;
pub const GCC_GP2_CLK: c_int = 3;
pub const GCC_GP2_CLK_SRC: c_int = 4;
pub const GCC_GPLL0: c_int = 5;
pub const GCC_GPLL0_OUT_EVEN: c_int = 6;
pub const GCC_MMU_0_TCU_VOTE_CLK: c_int = 7;
pub const GCC_PCIE_A_AUX_CLK: c_int = 8;
pub const GCC_PCIE_A_AUX_CLK_SRC: c_int = 9;
pub const GCC_PCIE_A_CFG_AHB_CLK: c_int = 10;
pub const GCC_PCIE_A_DTI_QTC_CLK: c_int = 11;
pub const GCC_PCIE_A_MSTR_AXI_CLK: c_int = 12;
pub const GCC_PCIE_A_PHY_AUX_CLK: c_int = 13;
pub const GCC_PCIE_A_PHY_AUX_CLK_SRC: c_int = 14;
pub const GCC_PCIE_A_PHY_RCHNG_CLK: c_int = 15;
pub const GCC_PCIE_A_PHY_RCHNG_CLK_SRC: c_int = 16;
pub const GCC_PCIE_A_PIPE_CLK: c_int = 17;
pub const GCC_PCIE_A_PIPE_CLK_SRC: c_int = 18;
pub const GCC_PCIE_A_SLV_AXI_CLK: c_int = 19;
pub const GCC_PCIE_A_SLV_Q2A_AXI_CLK: c_int = 20;
pub const GCC_PCIE_B_AUX_CLK: c_int = 21;
pub const GCC_PCIE_B_AUX_CLK_SRC: c_int = 22;
pub const GCC_PCIE_B_CFG_AHB_CLK: c_int = 23;
pub const GCC_PCIE_B_DTI_QTC_CLK: c_int = 24;
pub const GCC_PCIE_B_MSTR_AXI_CLK: c_int = 25;
pub const GCC_PCIE_B_PHY_AUX_CLK: c_int = 26;
pub const GCC_PCIE_B_PHY_AUX_CLK_SRC: c_int = 27;
pub const GCC_PCIE_B_PHY_RCHNG_CLK: c_int = 28;
pub const GCC_PCIE_B_PHY_RCHNG_CLK_SRC: c_int = 29;
pub const GCC_PCIE_B_PIPE_CLK: c_int = 30;
pub const GCC_PCIE_B_PIPE_CLK_SRC: c_int = 31;
pub const GCC_PCIE_B_SLV_AXI_CLK: c_int = 32;
pub const GCC_PCIE_B_SLV_Q2A_AXI_CLK: c_int = 33;
pub const GCC_PCIE_C_AUX_CLK: c_int = 34;
pub const GCC_PCIE_C_AUX_CLK_SRC: c_int = 35;
pub const GCC_PCIE_C_CFG_AHB_CLK: c_int = 36;
pub const GCC_PCIE_C_DTI_QTC_CLK: c_int = 37;
pub const GCC_PCIE_C_MSTR_AXI_CLK: c_int = 38;
pub const GCC_PCIE_C_PHY_AUX_CLK: c_int = 39;
pub const GCC_PCIE_C_PHY_AUX_CLK_SRC: c_int = 40;
pub const GCC_PCIE_C_PHY_RCHNG_CLK: c_int = 41;
pub const GCC_PCIE_C_PHY_RCHNG_CLK_SRC: c_int = 42;
pub const GCC_PCIE_C_PIPE_CLK: c_int = 43;
pub const GCC_PCIE_C_PIPE_CLK_SRC: c_int = 44;
pub const GCC_PCIE_C_SLV_AXI_CLK: c_int = 45;
pub const GCC_PCIE_C_SLV_Q2A_AXI_CLK: c_int = 46;
pub const GCC_PCIE_D_AUX_CLK: c_int = 47;
pub const GCC_PCIE_D_AUX_CLK_SRC: c_int = 48;
pub const GCC_PCIE_D_CFG_AHB_CLK: c_int = 49;
pub const GCC_PCIE_D_DTI_QTC_CLK: c_int = 50;
pub const GCC_PCIE_D_MSTR_AXI_CLK: c_int = 51;
pub const GCC_PCIE_D_PHY_AUX_CLK: c_int = 52;
pub const GCC_PCIE_D_PHY_AUX_CLK_SRC: c_int = 53;
pub const GCC_PCIE_D_PHY_RCHNG_CLK: c_int = 54;
pub const GCC_PCIE_D_PHY_RCHNG_CLK_SRC: c_int = 55;
pub const GCC_PCIE_D_PIPE_CLK: c_int = 56;
pub const GCC_PCIE_D_PIPE_CLK_SRC: c_int = 57;
pub const GCC_PCIE_D_SLV_AXI_CLK: c_int = 58;
pub const GCC_PCIE_D_SLV_Q2A_AXI_CLK: c_int = 59;
pub const GCC_PCIE_LINK_AHB_CLK: c_int = 60;
pub const GCC_PCIE_LINK_XO_CLK: c_int = 61;
pub const GCC_PCIE_NOC_ASYNC_BRIDGE_CLK: c_int = 62;
pub const GCC_PCIE_NOC_CNOC_SF_QX_CLK: c_int = 63;
pub const GCC_PCIE_NOC_M_CFG_CLK: c_int = 64;
pub const GCC_PCIE_NOC_M_PDB_CLK: c_int = 65;
pub const GCC_PCIE_NOC_MSTR_AXI_CLK: c_int = 66;
pub const GCC_PCIE_NOC_PWRCTL_CLK: c_int = 67;
pub const GCC_PCIE_NOC_QOSGEN_EXTREF_CLK: c_int = 68;
pub const GCC_PCIE_NOC_REFGEN_CLK: c_int = 69;
pub const GCC_PCIE_NOC_REFGEN_CLK_SRC: c_int = 70;
pub const GCC_PCIE_NOC_S_CFG_CLK: c_int = 71;
pub const GCC_PCIE_NOC_S_PDB_CLK: c_int = 72;
pub const GCC_PCIE_NOC_SAFETY_CLK: c_int = 73;
pub const GCC_PCIE_NOC_SAFETY_CLK_SRC: c_int = 74;
pub const GCC_PCIE_NOC_SLAVE_AXI_CLK: c_int = 75;
pub const GCC_PCIE_NOC_TSCTR_CLK: c_int = 76;
pub const GCC_PCIE_NOC_XO_CLK: c_int = 77;
pub const GCC_PDM2_CLK: c_int = 78;
pub const GCC_PDM2_CLK_SRC: c_int = 79;
pub const GCC_PDM_AHB_CLK: c_int = 80;
pub const GCC_PDM_XO4_CLK: c_int = 81;
pub const GCC_QUPV3_WRAP3_CORE_2X_CLK: c_int = 82;
pub const GCC_QUPV3_WRAP3_CORE_CLK: c_int = 83;
pub const GCC_QUPV3_WRAP3_M_CLK: c_int = 84;
pub const GCC_QUPV3_WRAP3_QSPI_REF_CLK: c_int = 85;
pub const GCC_QUPV3_WRAP3_QSPI_REF_CLK_SRC: c_int = 86;
pub const GCC_QUPV3_WRAP3_S0_CLK: c_int = 87;
pub const GCC_QUPV3_WRAP3_S0_CLK_SRC: c_int = 88;
pub const GCC_QUPV3_WRAP3_S_AHB_CLK: c_int = 89;
pub const GCC_SMMU_PCIE_QTC_VOTE_CLK: c_int = 90;
// GCC power domains
pub const GCC_PCIE_A_GDSC: c_int = 0;
pub const GCC_PCIE_A_PHY_GDSC: c_int = 1;
pub const GCC_PCIE_B_GDSC: c_int = 2;
pub const GCC_PCIE_B_PHY_GDSC: c_int = 3;
pub const GCC_PCIE_C_GDSC: c_int = 4;
pub const GCC_PCIE_C_PHY_GDSC: c_int = 5;
pub const GCC_PCIE_D_GDSC: c_int = 6;
pub const GCC_PCIE_D_PHY_GDSC: c_int = 7;
pub const GCC_PCIE_NOC_GDSC: c_int = 8;
// GCC resets
pub const GCC_PCIE_A_BCR: c_int = 0;
pub const GCC_PCIE_A_LINK_DOWN_BCR: c_int = 1;
pub const GCC_PCIE_A_NOCSR_COM_PHY_BCR: c_int = 2;
pub const GCC_PCIE_A_PHY_BCR: c_int = 3;
pub const GCC_PCIE_A_PHY_CFG_AHB_BCR: c_int = 4;
pub const GCC_PCIE_A_PHY_COM_BCR: c_int = 5;
pub const GCC_PCIE_A_PHY_NOCSR_COM_PHY_BCR: c_int = 6;
pub const GCC_PCIE_B_BCR: c_int = 7;
pub const GCC_PCIE_B_LINK_DOWN_BCR: c_int = 8;
pub const GCC_PCIE_B_NOCSR_COM_PHY_BCR: c_int = 9;
pub const GCC_PCIE_B_PHY_BCR: c_int = 10;
pub const GCC_PCIE_B_PHY_CFG_AHB_BCR: c_int = 11;
pub const GCC_PCIE_B_PHY_COM_BCR: c_int = 12;
pub const GCC_PCIE_B_PHY_NOCSR_COM_PHY_BCR: c_int = 13;
pub const GCC_PCIE_C_BCR: c_int = 14;
pub const GCC_PCIE_C_LINK_DOWN_BCR: c_int = 15;
pub const GCC_PCIE_C_NOCSR_COM_PHY_BCR: c_int = 16;
pub const GCC_PCIE_C_PHY_BCR: c_int = 17;
pub const GCC_PCIE_C_PHY_CFG_AHB_BCR: c_int = 18;
pub const GCC_PCIE_C_PHY_COM_BCR: c_int = 19;
pub const GCC_PCIE_C_PHY_NOCSR_COM_PHY_BCR: c_int = 20;
pub const GCC_PCIE_D_BCR: c_int = 21;
pub const GCC_PCIE_D_LINK_DOWN_BCR: c_int = 22;
pub const GCC_PCIE_D_NOCSR_COM_PHY_BCR: c_int = 23;
pub const GCC_PCIE_D_PHY_BCR: c_int = 24;
pub const GCC_PCIE_D_PHY_CFG_AHB_BCR: c_int = 25;
pub const GCC_PCIE_D_PHY_COM_BCR: c_int = 26;
pub const GCC_PCIE_D_PHY_NOCSR_COM_PHY_BCR: c_int = 27;
pub const GCC_PCIE_NOC_BCR: c_int = 28;
pub const GCC_PDM_BCR: c_int = 29;
pub const GCC_QUPV3_WRAPPER_3_BCR: c_int = 30;
pub const GCC_TCSR_PCIE_BCR: c_int = 31;
