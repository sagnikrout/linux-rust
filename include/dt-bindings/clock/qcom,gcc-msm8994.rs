//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,gcc-msm8994.h
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
// Copyright (c) 2016, The Linux Foundation. All rights reserved.
//
pub const GPLL0_EARLY: c_int = 0;
pub const GPLL0: c_int = 1;
pub const GPLL4_EARLY: c_int = 2;
pub const GPLL4: c_int = 3;
pub const UFS_AXI_CLK_SRC: c_int = 4;
pub const USB30_MASTER_CLK_SRC: c_int = 5;
pub const BLSP1_QUP1_I2C_APPS_CLK_SRC: c_int = 6;
pub const BLSP1_QUP1_SPI_APPS_CLK_SRC: c_int = 7;
pub const BLSP1_QUP2_I2C_APPS_CLK_SRC: c_int = 8;
pub const BLSP1_QUP2_SPI_APPS_CLK_SRC: c_int = 9;
pub const BLSP1_QUP3_I2C_APPS_CLK_SRC: c_int = 10;
pub const BLSP1_QUP3_SPI_APPS_CLK_SRC: c_int = 11;
pub const BLSP1_QUP4_I2C_APPS_CLK_SRC: c_int = 12;
pub const BLSP1_QUP4_SPI_APPS_CLK_SRC: c_int = 13;
pub const BLSP1_QUP5_I2C_APPS_CLK_SRC: c_int = 14;
pub const BLSP1_QUP5_SPI_APPS_CLK_SRC: c_int = 15;
pub const BLSP1_QUP6_I2C_APPS_CLK_SRC: c_int = 16;
pub const BLSP1_QUP6_SPI_APPS_CLK_SRC: c_int = 17;
pub const BLSP1_UART1_APPS_CLK_SRC: c_int = 18;
pub const BLSP1_UART2_APPS_CLK_SRC: c_int = 19;
pub const BLSP1_UART3_APPS_CLK_SRC: c_int = 20;
pub const BLSP1_UART4_APPS_CLK_SRC: c_int = 21;
pub const BLSP1_UART5_APPS_CLK_SRC: c_int = 22;
pub const BLSP1_UART6_APPS_CLK_SRC: c_int = 23;
pub const BLSP2_QUP1_I2C_APPS_CLK_SRC: c_int = 24;
pub const BLSP2_QUP1_SPI_APPS_CLK_SRC: c_int = 25;
pub const BLSP2_QUP2_I2C_APPS_CLK_SRC: c_int = 26;
pub const BLSP2_QUP2_SPI_APPS_CLK_SRC: c_int = 27;
pub const BLSP2_QUP3_I2C_APPS_CLK_SRC: c_int = 28;
pub const BLSP2_QUP3_SPI_APPS_CLK_SRC: c_int = 29;
pub const BLSP2_QUP4_I2C_APPS_CLK_SRC: c_int = 30;
pub const BLSP2_QUP4_SPI_APPS_CLK_SRC: c_int = 31;
pub const BLSP2_QUP5_I2C_APPS_CLK_SRC: c_int = 32;
pub const BLSP2_QUP5_SPI_APPS_CLK_SRC: c_int = 33;
pub const BLSP2_QUP6_I2C_APPS_CLK_SRC: c_int = 34;
pub const BLSP2_QUP6_SPI_APPS_CLK_SRC: c_int = 35;
pub const BLSP2_UART1_APPS_CLK_SRC: c_int = 36;
pub const BLSP2_UART2_APPS_CLK_SRC: c_int = 37;
pub const BLSP2_UART3_APPS_CLK_SRC: c_int = 38;
pub const BLSP2_UART4_APPS_CLK_SRC: c_int = 39;
pub const BLSP2_UART5_APPS_CLK_SRC: c_int = 40;
pub const BLSP2_UART6_APPS_CLK_SRC: c_int = 41;
pub const GP1_CLK_SRC: c_int = 42;
pub const GP2_CLK_SRC: c_int = 43;
pub const GP3_CLK_SRC: c_int = 44;
pub const PCIE_0_AUX_CLK_SRC: c_int = 45;
pub const PCIE_0_PIPE_CLK_SRC: c_int = 46;
pub const PCIE_1_AUX_CLK_SRC: c_int = 47;
pub const PCIE_1_PIPE_CLK_SRC: c_int = 48;
pub const PDM2_CLK_SRC: c_int = 49;
pub const SDCC1_APPS_CLK_SRC: c_int = 50;
pub const SDCC2_APPS_CLK_SRC: c_int = 51;
pub const SDCC3_APPS_CLK_SRC: c_int = 52;
pub const SDCC4_APPS_CLK_SRC: c_int = 53;
pub const TSIF_REF_CLK_SRC: c_int = 54;
pub const USB30_MOCK_UTMI_CLK_SRC: c_int = 55;
pub const USB3_PHY_AUX_CLK_SRC: c_int = 56;
pub const USB_HS_SYSTEM_CLK_SRC: c_int = 57;
pub const GCC_BLSP1_AHB_CLK: c_int = 58;
pub const GCC_BLSP1_QUP1_I2C_APPS_CLK: c_int = 59;
pub const GCC_BLSP1_QUP1_SPI_APPS_CLK: c_int = 60;
pub const GCC_BLSP1_QUP2_I2C_APPS_CLK: c_int = 61;
pub const GCC_BLSP1_QUP2_SPI_APPS_CLK: c_int = 62;
pub const GCC_BLSP1_QUP3_I2C_APPS_CLK: c_int = 63;
pub const GCC_BLSP1_QUP3_SPI_APPS_CLK: c_int = 64;
pub const GCC_BLSP1_QUP4_I2C_APPS_CLK: c_int = 65;
pub const GCC_BLSP1_QUP4_SPI_APPS_CLK: c_int = 66;
pub const GCC_BLSP1_QUP5_I2C_APPS_CLK: c_int = 67;
pub const GCC_BLSP1_QUP5_SPI_APPS_CLK: c_int = 68;
pub const GCC_BLSP1_QUP6_I2C_APPS_CLK: c_int = 69;
pub const GCC_BLSP1_QUP6_SPI_APPS_CLK: c_int = 70;
pub const GCC_BLSP1_UART1_APPS_CLK: c_int = 71;
pub const GCC_BLSP1_UART2_APPS_CLK: c_int = 72;
pub const GCC_BLSP1_UART3_APPS_CLK: c_int = 73;
pub const GCC_BLSP1_UART4_APPS_CLK: c_int = 74;
pub const GCC_BLSP1_UART5_APPS_CLK: c_int = 75;
pub const GCC_BLSP1_UART6_APPS_CLK: c_int = 76;
pub const GCC_BLSP2_AHB_CLK: c_int = 77;
pub const GCC_BLSP2_QUP1_I2C_APPS_CLK: c_int = 78;
pub const GCC_BLSP2_QUP1_SPI_APPS_CLK: c_int = 79;
pub const GCC_BLSP2_QUP2_I2C_APPS_CLK: c_int = 80;
pub const GCC_BLSP2_QUP2_SPI_APPS_CLK: c_int = 81;
pub const GCC_BLSP2_QUP3_I2C_APPS_CLK: c_int = 82;
pub const GCC_BLSP2_QUP3_SPI_APPS_CLK: c_int = 83;
pub const GCC_BLSP2_QUP4_I2C_APPS_CLK: c_int = 84;
pub const GCC_BLSP2_QUP4_SPI_APPS_CLK: c_int = 85;
pub const GCC_BLSP2_QUP5_I2C_APPS_CLK: c_int = 86;
pub const GCC_BLSP2_QUP5_SPI_APPS_CLK: c_int = 87;
pub const GCC_BLSP2_QUP6_I2C_APPS_CLK: c_int = 88;
pub const GCC_BLSP2_QUP6_SPI_APPS_CLK: c_int = 89;
pub const GCC_BLSP2_UART1_APPS_CLK: c_int = 90;
pub const GCC_BLSP2_UART2_APPS_CLK: c_int = 91;
pub const GCC_BLSP2_UART3_APPS_CLK: c_int = 92;
pub const GCC_BLSP2_UART4_APPS_CLK: c_int = 93;
pub const GCC_BLSP2_UART5_APPS_CLK: c_int = 94;
pub const GCC_BLSP2_UART6_APPS_CLK: c_int = 95;
pub const GCC_GP1_CLK: c_int = 96;
pub const GCC_GP2_CLK: c_int = 97;
pub const GCC_GP3_CLK: c_int = 98;
pub const GCC_PCIE_0_AUX_CLK: c_int = 99;
pub const GCC_PCIE_0_PIPE_CLK: c_int = 100;
pub const GCC_PCIE_1_AUX_CLK: c_int = 101;
pub const GCC_PCIE_1_PIPE_CLK: c_int = 102;
pub const GCC_PDM2_CLK: c_int = 103;
pub const GCC_SDCC1_APPS_CLK: c_int = 104;
pub const GCC_SDCC2_APPS_CLK: c_int = 105;
pub const GCC_SDCC3_APPS_CLK: c_int = 106;
pub const GCC_SDCC4_APPS_CLK: c_int = 107;
pub const GCC_SYS_NOC_UFS_AXI_CLK: c_int = 108;
pub const GCC_SYS_NOC_USB3_AXI_CLK: c_int = 109;
pub const GCC_TSIF_REF_CLK: c_int = 110;
pub const GCC_UFS_AXI_CLK: c_int = 111;
pub const GCC_UFS_RX_CFG_CLK: c_int = 112;
pub const GCC_UFS_TX_CFG_CLK: c_int = 113;
pub const GCC_USB30_MASTER_CLK: c_int = 114;
pub const GCC_USB30_MOCK_UTMI_CLK: c_int = 115;
pub const GCC_USB3_PHY_AUX_CLK: c_int = 116;
pub const GCC_USB_HS_SYSTEM_CLK: c_int = 117;
pub const GCC_SDCC1_AHB_CLK: c_int = 118;
pub const GCC_LPASS_Q6_AXI_CLK: c_int = 119;
pub const GCC_MSS_Q6_BIMC_AXI_CLK: c_int = 120;
pub const GCC_PCIE_0_CFG_AHB_CLK: c_int = 121;
pub const GCC_PCIE_0_MSTR_AXI_CLK: c_int = 122;
pub const GCC_PCIE_0_SLV_AXI_CLK: c_int = 123;
pub const GCC_PCIE_1_CFG_AHB_CLK: c_int = 124;
pub const GCC_PCIE_1_MSTR_AXI_CLK: c_int = 125;
pub const GCC_PCIE_1_SLV_AXI_CLK: c_int = 126;
pub const GCC_PDM_AHB_CLK: c_int = 127;
pub const GCC_SDCC2_AHB_CLK: c_int = 128;
pub const GCC_SDCC3_AHB_CLK: c_int = 129;
pub const GCC_SDCC4_AHB_CLK: c_int = 130;
pub const GCC_TSIF_AHB_CLK: c_int = 131;
pub const GCC_UFS_AHB_CLK: c_int = 132;
pub const GCC_UFS_RX_SYMBOL_0_CLK: c_int = 133;
pub const GCC_UFS_RX_SYMBOL_1_CLK: c_int = 134;
pub const GCC_UFS_TX_SYMBOL_0_CLK: c_int = 135;
pub const GCC_UFS_TX_SYMBOL_1_CLK: c_int = 136;
pub const GCC_USB2_HS_PHY_SLEEP_CLK: c_int = 137;
pub const GCC_USB30_SLEEP_CLK: c_int = 138;
pub const GCC_USB_HS_AHB_CLK: c_int = 139;
pub const GCC_USB_PHY_CFG_AHB2PHY_CLK: c_int = 140;
pub const CONFIG_NOC_CLK_SRC: c_int = 141;
pub const PERIPH_NOC_CLK_SRC: c_int = 142;
pub const SYSTEM_NOC_CLK_SRC: c_int = 143;
pub const GPLL0_OUT_MMSSCC: c_int = 144;
pub const GPLL0_OUT_MSSCC: c_int = 145;
pub const PCIE_0_PHY_LDO: c_int = 146;
pub const PCIE_1_PHY_LDO: c_int = 147;
pub const UFS_PHY_LDO: c_int = 148;
pub const USB_SS_PHY_LDO: c_int = 149;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 150;
pub const GCC_PRNG_AHB_CLK: c_int = 151;
pub const GCC_USB3_PHY_PIPE_CLK: c_int = 152;
// GDSCs
pub const PCIE_GDSC: c_int = 0;
pub const PCIE_0_GDSC: c_int = 1;
pub const PCIE_1_GDSC: c_int = 2;
pub const USB30_GDSC: c_int = 3;
pub const UFS_GDSC: c_int = 4;
// Resets
pub const USB3_PHY_RESET: c_int = 0;
pub const USB3PHY_PHY_RESET: c_int = 1;
pub const PCIE_PHY_0_RESET: c_int = 2;
pub const PCIE_PHY_1_RESET: c_int = 3;
pub const QUSB2_PHY_RESET: c_int = 4;
pub const MSS_RESET: c_int = 5;
