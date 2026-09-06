//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,gcc-mdm9607.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// Copyright (c) 2021, Konrad Dybcio <konrad.dybcio@somainline.org>
//
pub const GPLL0: c_int = 0;
pub const GPLL0_EARLY: c_int = 1;
pub const GPLL1: c_int = 2;
pub const GPLL1_VOTE: c_int = 3;
pub const GPLL2: c_int = 4;
pub const GPLL2_EARLY: c_int = 5;
pub const PCNOC_BFDCD_CLK_SRC: c_int = 6;
pub const GCC_SMMU_CFG_CLK: c_int = 8;
pub const APSS_AHB_CLK_SRC: c_int = 9;
pub const GCC_QDSS_DAP_CLK: c_int = 10;
pub const BLSP1_QUP1_I2C_APPS_CLK_SRC: c_int = 11;
pub const BLSP1_QUP1_SPI_APPS_CLK_SRC: c_int = 12;
pub const BLSP1_QUP2_I2C_APPS_CLK_SRC: c_int = 13;
pub const BLSP1_QUP2_SPI_APPS_CLK_SRC: c_int = 14;
pub const BLSP1_QUP3_I2C_APPS_CLK_SRC: c_int = 15;
pub const BLSP1_QUP3_SPI_APPS_CLK_SRC: c_int = 16;
pub const BLSP1_QUP4_I2C_APPS_CLK_SRC: c_int = 17;
pub const BLSP1_QUP4_SPI_APPS_CLK_SRC: c_int = 18;
pub const BLSP1_QUP5_I2C_APPS_CLK_SRC: c_int = 19;
pub const BLSP1_QUP5_SPI_APPS_CLK_SRC: c_int = 20;
pub const BLSP1_QUP6_I2C_APPS_CLK_SRC: c_int = 21;
pub const BLSP1_QUP6_SPI_APPS_CLK_SRC: c_int = 22;
pub const BLSP1_UART1_APPS_CLK_SRC: c_int = 23;
pub const BLSP1_UART2_APPS_CLK_SRC: c_int = 24;
pub const CRYPTO_CLK_SRC: c_int = 25;
pub const GP1_CLK_SRC: c_int = 26;
pub const GP2_CLK_SRC: c_int = 27;
pub const GP3_CLK_SRC: c_int = 28;
pub const PDM2_CLK_SRC: c_int = 29;
pub const SDCC1_APPS_CLK_SRC: c_int = 30;
pub const SDCC2_APPS_CLK_SRC: c_int = 31;
pub const APSS_TCU_CLK_SRC: c_int = 32;
pub const USB_HS_SYSTEM_CLK_SRC: c_int = 33;
pub const GCC_BLSP1_AHB_CLK: c_int = 34;
pub const GCC_BLSP1_SLEEP_CLK: c_int = 35;
pub const GCC_BLSP1_QUP1_I2C_APPS_CLK: c_int = 36;
pub const GCC_BLSP1_QUP1_SPI_APPS_CLK: c_int = 37;
pub const GCC_BLSP1_QUP2_I2C_APPS_CLK: c_int = 38;
pub const GCC_BLSP1_QUP2_SPI_APPS_CLK: c_int = 39;
pub const GCC_BLSP1_QUP3_I2C_APPS_CLK: c_int = 40;
pub const GCC_BLSP1_QUP3_SPI_APPS_CLK: c_int = 41;
pub const GCC_BLSP1_QUP4_I2C_APPS_CLK: c_int = 42;
pub const GCC_BLSP1_QUP4_SPI_APPS_CLK: c_int = 43;
pub const GCC_BLSP1_QUP5_I2C_APPS_CLK: c_int = 44;
pub const GCC_BLSP1_QUP5_SPI_APPS_CLK: c_int = 45;
pub const GCC_BLSP1_QUP6_I2C_APPS_CLK: c_int = 46;
pub const GCC_BLSP1_QUP6_SPI_APPS_CLK: c_int = 47;
pub const GCC_BLSP1_UART1_APPS_CLK: c_int = 48;
pub const GCC_BLSP1_UART2_APPS_CLK: c_int = 49;
pub const GCC_BOOT_ROM_AHB_CLK: c_int = 50;
pub const GCC_CRYPTO_AHB_CLK: c_int = 51;
pub const GCC_CRYPTO_AXI_CLK: c_int = 52;
pub const GCC_CRYPTO_CLK: c_int = 53;
pub const GCC_GP1_CLK: c_int = 54;
pub const GCC_GP2_CLK: c_int = 55;
pub const GCC_GP3_CLK: c_int = 56;
pub const GCC_MSS_CFG_AHB_CLK: c_int = 57;
pub const GCC_PDM2_CLK: c_int = 58;
pub const GCC_PDM_AHB_CLK: c_int = 59;
pub const GCC_PRNG_AHB_CLK: c_int = 60;
pub const GCC_SDCC1_AHB_CLK: c_int = 61;
pub const GCC_SDCC1_APPS_CLK: c_int = 62;
pub const GCC_SDCC2_AHB_CLK: c_int = 63;
pub const GCC_SDCC2_APPS_CLK: c_int = 64;
pub const GCC_USB2A_PHY_SLEEP_CLK: c_int = 65;
pub const GCC_USB_HS_AHB_CLK: c_int = 66;
pub const GCC_USB_HS_SYSTEM_CLK: c_int = 67;
pub const GCC_APSS_TCU_CLK: c_int = 68;
pub const GCC_MSS_Q6_BIMC_AXI_CLK: c_int = 69;
pub const BIMC_PLL: c_int = 70;
pub const BIMC_PLL_VOTE: c_int = 71;
pub const BIMC_DDR_CLK_SRC: c_int = 72;
pub const BLSP1_UART3_APPS_CLK_SRC: c_int = 73;
pub const BLSP1_UART4_APPS_CLK_SRC: c_int = 74;
pub const BLSP1_UART5_APPS_CLK_SRC: c_int = 75;
pub const BLSP1_UART6_APPS_CLK_SRC: c_int = 76;
pub const GCC_BLSP1_UART3_APPS_CLK: c_int = 77;
pub const GCC_BLSP1_UART4_APPS_CLK: c_int = 78;
pub const GCC_BLSP1_UART5_APPS_CLK: c_int = 79;
pub const GCC_BLSP1_UART6_APPS_CLK: c_int = 80;
pub const GCC_APSS_AHB_CLK: c_int = 81;
pub const GCC_APSS_AXI_CLK: c_int = 82;
pub const GCC_USB_HS_PHY_CFG_AHB_CLK: c_int = 83;
pub const GCC_USB_HSIC_CLK_SRC: c_int = 84;
pub const GCC_USB_HSIC_IO_CAL_CLK_SRC: c_int = 85;
pub const GCC_USB_HSIC_SYSTEM_CLK_SRC: c_int = 86;
// Resets
pub const USB2_HS_PHY_ONLY_BCR: c_int = 0;
pub const QUSB2_PHY_BCR: c_int = 1;
pub const GCC_MSS_RESTART: c_int = 2;
pub const USB_HS_HSIC_BCR: c_int = 3;
pub const USB_HS_BCR: c_int = 4;
