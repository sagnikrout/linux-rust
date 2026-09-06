//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,gcc-sdx65.h
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
// Copyright (c) 2021, Qualcomm Innovation Center, Inc. All rights reserved.
//
// GCC clocks
pub const GPLL0: c_int = 0;
pub const GPLL0_OUT_EVEN: c_int = 1;
pub const GCC_AHB_PCIE_LINK_CLK: c_int = 2;
pub const GCC_BLSP1_AHB_CLK: c_int = 3;
pub const GCC_BLSP1_QUP1_I2C_APPS_CLK: c_int = 4;
pub const GCC_BLSP1_QUP1_I2C_APPS_CLK_SRC: c_int = 5;
pub const GCC_BLSP1_QUP1_SPI_APPS_CLK: c_int = 6;
pub const GCC_BLSP1_QUP1_SPI_APPS_CLK_SRC: c_int = 7;
pub const GCC_BLSP1_QUP2_I2C_APPS_CLK: c_int = 8;
pub const GCC_BLSP1_QUP2_I2C_APPS_CLK_SRC: c_int = 9;
pub const GCC_BLSP1_QUP2_SPI_APPS_CLK: c_int = 10;
pub const GCC_BLSP1_QUP2_SPI_APPS_CLK_SRC: c_int = 11;
pub const GCC_BLSP1_QUP3_I2C_APPS_CLK: c_int = 12;
pub const GCC_BLSP1_QUP3_I2C_APPS_CLK_SRC: c_int = 13;
pub const GCC_BLSP1_QUP3_SPI_APPS_CLK: c_int = 14;
pub const GCC_BLSP1_QUP3_SPI_APPS_CLK_SRC: c_int = 15;
pub const GCC_BLSP1_QUP4_I2C_APPS_CLK: c_int = 16;
pub const GCC_BLSP1_QUP4_I2C_APPS_CLK_SRC: c_int = 17;
pub const GCC_BLSP1_QUP4_SPI_APPS_CLK: c_int = 18;
pub const GCC_BLSP1_QUP4_SPI_APPS_CLK_SRC: c_int = 19;
pub const GCC_BLSP1_SLEEP_CLK: c_int = 20;
pub const GCC_BLSP1_UART1_APPS_CLK: c_int = 21;
pub const GCC_BLSP1_UART1_APPS_CLK_SRC: c_int = 22;
pub const GCC_BLSP1_UART2_APPS_CLK: c_int = 23;
pub const GCC_BLSP1_UART2_APPS_CLK_SRC: c_int = 24;
pub const GCC_BLSP1_UART3_APPS_CLK: c_int = 25;
pub const GCC_BLSP1_UART3_APPS_CLK_SRC: c_int = 26;
pub const GCC_BLSP1_UART4_APPS_CLK: c_int = 27;
pub const GCC_BLSP1_UART4_APPS_CLK_SRC: c_int = 28;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 29;
pub const GCC_CPUSS_AHB_CLK: c_int = 30;
pub const GCC_CPUSS_AHB_CLK_SRC: c_int = 31;
pub const GCC_CPUSS_AHB_POSTDIV_CLK_SRC: c_int = 32;
pub const GCC_CPUSS_GNOC_CLK: c_int = 33;
pub const GCC_GP1_CLK: c_int = 34;
pub const GCC_GP1_CLK_SRC: c_int = 35;
pub const GCC_GP2_CLK: c_int = 36;
pub const GCC_GP2_CLK_SRC: c_int = 37;
pub const GCC_GP3_CLK: c_int = 38;
pub const GCC_GP3_CLK_SRC: c_int = 39;
pub const GCC_PCIE_0_CLKREF_EN: c_int = 40;
pub const GCC_PCIE_AUX_CLK: c_int = 41;
pub const GCC_PCIE_AUX_CLK_SRC: c_int = 42;
pub const GCC_PCIE_AUX_PHY_CLK_SRC: c_int = 43;
pub const GCC_PCIE_CFG_AHB_CLK: c_int = 44;
pub const GCC_PCIE_MSTR_AXI_CLK: c_int = 45;
pub const GCC_PCIE_PIPE_CLK: c_int = 46;
pub const GCC_PCIE_PIPE_CLK_SRC: c_int = 47;
pub const GCC_PCIE_RCHNG_PHY_CLK: c_int = 48;
pub const GCC_PCIE_RCHNG_PHY_CLK_SRC: c_int = 49;
pub const GCC_PCIE_SLEEP_CLK: c_int = 50;
pub const GCC_PCIE_SLV_AXI_CLK: c_int = 51;
pub const GCC_PCIE_SLV_Q2A_AXI_CLK: c_int = 52;
pub const GCC_PDM2_CLK: c_int = 53;
pub const GCC_PDM2_CLK_SRC: c_int = 54;
pub const GCC_PDM_AHB_CLK: c_int = 55;
pub const GCC_PDM_XO4_CLK: c_int = 56;
pub const GCC_RX1_USB2_CLKREF_EN: c_int = 57;
pub const GCC_SDCC1_AHB_CLK: c_int = 58;
pub const GCC_SDCC1_APPS_CLK: c_int = 59;
pub const GCC_SDCC1_APPS_CLK_SRC: c_int = 60;
pub const GCC_SPMI_FETCHER_AHB_CLK: c_int = 61;
pub const GCC_SPMI_FETCHER_CLK: c_int = 62;
pub const GCC_SPMI_FETCHER_CLK_SRC: c_int = 63;
pub const GCC_SYS_NOC_CPUSS_AHB_CLK: c_int = 64;
pub const GCC_USB30_MASTER_CLK: c_int = 65;
pub const GCC_USB30_MASTER_CLK_SRC: c_int = 66;
pub const GCC_USB30_MOCK_UTMI_CLK: c_int = 67;
pub const GCC_USB30_MOCK_UTMI_CLK_SRC: c_int = 68;
pub const GCC_USB30_MOCK_UTMI_POSTDIV_CLK_SRC: c_int = 69;
pub const GCC_USB30_MSTR_AXI_CLK: c_int = 70;
pub const GCC_USB30_SLEEP_CLK: c_int = 71;
pub const GCC_USB30_SLV_AHB_CLK: c_int = 72;
pub const GCC_USB3_PHY_AUX_CLK: c_int = 73;
pub const GCC_USB3_PHY_AUX_CLK_SRC: c_int = 74;
pub const GCC_USB3_PHY_PIPE_CLK: c_int = 75;
pub const GCC_USB3_PHY_PIPE_CLK_SRC: c_int = 76;
pub const GCC_USB3_PRIM_CLKREF_EN: c_int = 77;
pub const GCC_USB_PHY_CFG_AHB2PHY_CLK: c_int = 78;
pub const GCC_XO_DIV4_CLK: c_int = 79;
pub const GCC_XO_PCIE_LINK_CLK: c_int = 80;
// GCC resets
pub const GCC_BLSP1_QUP1_BCR: c_int = 0;
pub const GCC_BLSP1_QUP2_BCR: c_int = 1;
pub const GCC_BLSP1_QUP3_BCR: c_int = 2;
pub const GCC_BLSP1_QUP4_BCR: c_int = 3;
pub const GCC_BLSP1_UART1_BCR: c_int = 4;
pub const GCC_BLSP1_UART2_BCR: c_int = 5;
pub const GCC_BLSP1_UART3_BCR: c_int = 6;
pub const GCC_BLSP1_UART4_BCR: c_int = 7;
pub const GCC_PCIE_BCR: c_int = 8;
pub const GCC_PCIE_LINK_DOWN_BCR: c_int = 9;
pub const GCC_PCIE_NOCSR_COM_PHY_BCR: c_int = 10;
pub const GCC_PCIE_PHY_BCR: c_int = 11;
pub const GCC_PCIE_PHY_CFG_AHB_BCR: c_int = 12;
pub const GCC_PCIE_PHY_COM_BCR: c_int = 13;
pub const GCC_PCIE_PHY_NOCSR_COM_PHY_BCR: c_int = 14;
pub const GCC_PDM_BCR: c_int = 15;
pub const GCC_QUSB2PHY_BCR: c_int = 16;
pub const GCC_SDCC1_BCR: c_int = 17;
pub const GCC_SPMI_FETCHER_BCR: c_int = 18;
pub const GCC_TCSR_PCIE_BCR: c_int = 19;
pub const GCC_USB30_BCR: c_int = 20;
pub const GCC_USB3_PHY_BCR: c_int = 21;
pub const GCC_USB3PHY_PHY_BCR: c_int = 22;
pub const GCC_USB_PHY_CFG_AHB2PHY_BCR: c_int = 23;
// GCC power domains
pub const USB30_GDSC: c_int = 0;
pub const PCIE_GDSC: c_int = 1;
