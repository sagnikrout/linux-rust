//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,nord-negcc.h
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
// NE_GCC clocks
pub const NE_GCC_AGGRE_NOC_UFS_PHY_AXI_CLK: c_int = 0;
pub const NE_GCC_AGGRE_NOC_USB2_AXI_CLK: c_int = 1;
pub const NE_GCC_AGGRE_NOC_USB3_PRIM_AXI_CLK: c_int = 2;
pub const NE_GCC_AGGRE_NOC_USB3_SEC_AXI_CLK: c_int = 3;
pub const NE_GCC_AHB2PHY_CLK: c_int = 4;
pub const NE_GCC_CNOC_USB2_AXI_CLK: c_int = 5;
pub const NE_GCC_CNOC_USB3_PRIM_AXI_CLK: c_int = 6;
pub const NE_GCC_CNOC_USB3_SEC_AXI_CLK: c_int = 7;
pub const NE_GCC_FRQ_MEASURE_REF_CLK: c_int = 8;
pub const NE_GCC_GP1_CLK: c_int = 9;
pub const NE_GCC_GP1_CLK_SRC: c_int = 10;
pub const NE_GCC_GP2_CLK: c_int = 11;
pub const NE_GCC_GP2_CLK_SRC: c_int = 12;
pub const NE_GCC_GPLL0: c_int = 13;
pub const NE_GCC_GPLL0_OUT_EVEN: c_int = 14;
pub const NE_GCC_GPLL2: c_int = 15;
pub const NE_GCC_GPU_2_CFG_CLK: c_int = 16;
pub const NE_GCC_GPU_2_GPLL0_CLK_SRC: c_int = 17;
pub const NE_GCC_GPU_2_GPLL0_DIV_CLK_SRC: c_int = 18;
pub const NE_GCC_GPU_2_HSCNOC_GFX_CLK: c_int = 19;
pub const NE_GCC_GPU_2_SMMU_VOTE_CLK: c_int = 20;
pub const NE_GCC_QUPV3_WRAP2_CORE_2X_CLK: c_int = 21;
pub const NE_GCC_QUPV3_WRAP2_CORE_CLK: c_int = 22;
pub const NE_GCC_QUPV3_WRAP2_M_AHB_CLK: c_int = 23;
pub const NE_GCC_QUPV3_WRAP2_S0_CLK: c_int = 24;
pub const NE_GCC_QUPV3_WRAP2_S0_CLK_SRC: c_int = 25;
pub const NE_GCC_QUPV3_WRAP2_S1_CLK: c_int = 26;
pub const NE_GCC_QUPV3_WRAP2_S1_CLK_SRC: c_int = 27;
pub const NE_GCC_QUPV3_WRAP2_S2_CLK: c_int = 28;
pub const NE_GCC_QUPV3_WRAP2_S2_CLK_SRC: c_int = 29;
pub const NE_GCC_QUPV3_WRAP2_S3_CLK: c_int = 30;
pub const NE_GCC_QUPV3_WRAP2_S3_CLK_SRC: c_int = 31;
pub const NE_GCC_QUPV3_WRAP2_S4_CLK: c_int = 32;
pub const NE_GCC_QUPV3_WRAP2_S4_CLK_SRC: c_int = 33;
pub const NE_GCC_QUPV3_WRAP2_S5_CLK: c_int = 34;
pub const NE_GCC_QUPV3_WRAP2_S5_CLK_SRC: c_int = 35;
pub const NE_GCC_QUPV3_WRAP2_S6_CLK: c_int = 36;
pub const NE_GCC_QUPV3_WRAP2_S6_CLK_SRC: c_int = 37;
pub const NE_GCC_QUPV3_WRAP2_S_AHB_CLK: c_int = 38;
pub const NE_GCC_SDCC4_APPS_CLK: c_int = 39;
pub const NE_GCC_SDCC4_APPS_CLK_SRC: c_int = 40;
pub const NE_GCC_SDCC4_AXI_CLK: c_int = 41;
pub const NE_GCC_UFS_PHY_AHB_CLK: c_int = 42;
pub const NE_GCC_UFS_PHY_AXI_CLK: c_int = 43;
pub const NE_GCC_UFS_PHY_AXI_CLK_SRC: c_int = 44;
pub const NE_GCC_UFS_PHY_ICE_CORE_CLK: c_int = 45;
pub const NE_GCC_UFS_PHY_ICE_CORE_CLK_SRC: c_int = 46;
pub const NE_GCC_UFS_PHY_PHY_AUX_CLK: c_int = 47;
pub const NE_GCC_UFS_PHY_PHY_AUX_CLK_SRC: c_int = 48;
pub const NE_GCC_UFS_PHY_RX_SYMBOL_0_CLK: c_int = 49;
pub const NE_GCC_UFS_PHY_RX_SYMBOL_0_CLK_SRC: c_int = 50;
pub const NE_GCC_UFS_PHY_RX_SYMBOL_1_CLK: c_int = 51;
pub const NE_GCC_UFS_PHY_RX_SYMBOL_1_CLK_SRC: c_int = 52;
pub const NE_GCC_UFS_PHY_TX_SYMBOL_0_CLK: c_int = 53;
pub const NE_GCC_UFS_PHY_TX_SYMBOL_0_CLK_SRC: c_int = 54;
pub const NE_GCC_UFS_PHY_UNIPRO_CORE_CLK: c_int = 55;
pub const NE_GCC_UFS_PHY_UNIPRO_CORE_CLK_SRC: c_int = 56;
pub const NE_GCC_USB20_MASTER_CLK: c_int = 57;
pub const NE_GCC_USB20_MASTER_CLK_SRC: c_int = 58;
pub const NE_GCC_USB20_MOCK_UTMI_CLK: c_int = 59;
pub const NE_GCC_USB20_MOCK_UTMI_CLK_SRC: c_int = 60;
pub const NE_GCC_USB20_MOCK_UTMI_POSTDIV_CLK_SRC: c_int = 61;
pub const NE_GCC_USB20_SLEEP_CLK: c_int = 62;
pub const NE_GCC_USB31_PRIM_ATB_CLK: c_int = 63;
pub const NE_GCC_USB31_PRIM_EUD_AHB_CLK: c_int = 64;
pub const NE_GCC_USB31_PRIM_MASTER_CLK: c_int = 65;
pub const NE_GCC_USB31_PRIM_MASTER_CLK_SRC: c_int = 66;
pub const NE_GCC_USB31_PRIM_MOCK_UTMI_CLK: c_int = 67;
pub const NE_GCC_USB31_PRIM_MOCK_UTMI_CLK_SRC: c_int = 68;
pub const NE_GCC_USB31_PRIM_MOCK_UTMI_POSTDIV_CLK_SRC: c_int = 69;
pub const NE_GCC_USB31_PRIM_SLEEP_CLK: c_int = 70;
pub const NE_GCC_USB31_SEC_ATB_CLK: c_int = 71;
pub const NE_GCC_USB31_SEC_EUD_AHB_CLK: c_int = 72;
pub const NE_GCC_USB31_SEC_MASTER_CLK: c_int = 73;
pub const NE_GCC_USB31_SEC_MASTER_CLK_SRC: c_int = 74;
pub const NE_GCC_USB31_SEC_MOCK_UTMI_CLK: c_int = 75;
pub const NE_GCC_USB31_SEC_MOCK_UTMI_CLK_SRC: c_int = 76;
pub const NE_GCC_USB31_SEC_MOCK_UTMI_POSTDIV_CLK_SRC: c_int = 77;
pub const NE_GCC_USB31_SEC_SLEEP_CLK: c_int = 78;
pub const NE_GCC_USB3_PRIM_PHY_AUX_CLK: c_int = 79;
pub const NE_GCC_USB3_PRIM_PHY_AUX_CLK_SRC: c_int = 80;
pub const NE_GCC_USB3_PRIM_PHY_COM_AUX_CLK: c_int = 81;
pub const NE_GCC_USB3_PRIM_PHY_PIPE_CLK: c_int = 82;
pub const NE_GCC_USB3_PRIM_PHY_PIPE_CLK_SRC: c_int = 83;
pub const NE_GCC_USB3_SEC_PHY_AUX_CLK: c_int = 84;
pub const NE_GCC_USB3_SEC_PHY_AUX_CLK_SRC: c_int = 85;
pub const NE_GCC_USB3_SEC_PHY_COM_AUX_CLK: c_int = 86;
pub const NE_GCC_USB3_SEC_PHY_PIPE_CLK: c_int = 87;
pub const NE_GCC_USB3_SEC_PHY_PIPE_CLK_SRC: c_int = 88;
// NE_GCC power domains
pub const NE_GCC_UFS_MEM_PHY_GDSC: c_int = 0;
pub const NE_GCC_UFS_PHY_GDSC: c_int = 1;
pub const NE_GCC_USB20_PRIM_GDSC: c_int = 2;
pub const NE_GCC_USB31_PRIM_GDSC: c_int = 3;
pub const NE_GCC_USB31_SEC_GDSC: c_int = 4;
pub const NE_GCC_USB3_PHY_GDSC: c_int = 5;
pub const NE_GCC_USB3_SEC_PHY_GDSC: c_int = 6;
// NE_GCC resets
pub const NE_GCC_GPU_2_BCR: c_int = 0;
pub const NE_GCC_QUPV3_WRAPPER_2_BCR: c_int = 1;
pub const NE_GCC_SDCC4_BCR: c_int = 2;
pub const NE_GCC_UFS_PHY_BCR: c_int = 3;
pub const NE_GCC_USB20_PRIM_BCR: c_int = 4;
pub const NE_GCC_USB31_PRIM_BCR: c_int = 5;
pub const NE_GCC_USB31_SEC_BCR: c_int = 6;
pub const NE_GCC_USB3_DP_PHY_PRIM_BCR: c_int = 7;
pub const NE_GCC_USB3_DP_PHY_SEC_BCR: c_int = 8;
pub const NE_GCC_USB3_PHY_PRIM_BCR: c_int = 9;
pub const NE_GCC_USB3_PHY_SEC_BCR: c_int = 10;
pub const NE_GCC_USB3PHY_PHY_PRIM_BCR: c_int = 11;
pub const NE_GCC_USB3PHY_PHY_SEC_BCR: c_int = 12;
pub const NE_GCC_QUSB2PHY_PRIM_BCR: c_int = 13;
