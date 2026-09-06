//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/intel,agilex5-clkmgr.h
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
// Copyright (C) 2023, Intel Corporation
//
// fixed rate clocks
pub const AGILEX5_OSC1: c_int = 0;
pub const AGILEX5_CB_INTOSC_HS_DIV2_CLK: c_int = 1;
pub const AGILEX5_CB_INTOSC_LS_CLK: c_int = 2;
pub const AGILEX5_F2S_FREE_CLK: c_int = 3;
// PLL clocks
pub const AGILEX5_MAIN_PLL_CLK: c_int = 4;
pub const AGILEX5_MAIN_PLL_C0_CLK: c_int = 5;
pub const AGILEX5_MAIN_PLL_C1_CLK: c_int = 6;
pub const AGILEX5_MAIN_PLL_C2_CLK: c_int = 7;
pub const AGILEX5_MAIN_PLL_C3_CLK: c_int = 8;
pub const AGILEX5_PERIPH_PLL_CLK: c_int = 9;
pub const AGILEX5_PERIPH_PLL_C0_CLK: c_int = 10;
pub const AGILEX5_PERIPH_PLL_C1_CLK: c_int = 11;
pub const AGILEX5_PERIPH_PLL_C2_CLK: c_int = 12;
pub const AGILEX5_PERIPH_PLL_C3_CLK: c_int = 13;
pub const AGILEX5_CORE0_FREE_CLK: c_int = 14;
pub const AGILEX5_CORE1_FREE_CLK: c_int = 15;
pub const AGILEX5_CORE2_FREE_CLK: c_int = 16;
pub const AGILEX5_CORE3_FREE_CLK: c_int = 17;
pub const AGILEX5_DSU_FREE_CLK: c_int = 18;
pub const AGILEX5_BOOT_CLK: c_int = 19;
// fixed factor clocks
pub const AGILEX5_L3_MAIN_FREE_CLK: c_int = 20;
pub const AGILEX5_NOC_FREE_CLK: c_int = 21;
pub const AGILEX5_S2F_USR0_CLK: c_int = 22;
pub const AGILEX5_NOC_CLK: c_int = 23;
pub const AGILEX5_EMAC_A_FREE_CLK: c_int = 24;
pub const AGILEX5_EMAC_B_FREE_CLK: c_int = 25;
pub const AGILEX5_EMAC_PTP_FREE_CLK: c_int = 26;
pub const AGILEX5_GPIO_DB_FREE_CLK: c_int = 27;
pub const AGILEX5_S2F_USER0_FREE_CLK: c_int = 28;
pub const AGILEX5_S2F_USER1_FREE_CLK: c_int = 29;
pub const AGILEX5_PSI_REF_FREE_CLK: c_int = 30;
pub const AGILEX5_USB31_FREE_CLK: c_int = 31;
// Gate clocks
pub const AGILEX5_CORE0_CLK: c_int = 32;
pub const AGILEX5_CORE1_CLK: c_int = 33;
pub const AGILEX5_CORE2_CLK: c_int = 34;
pub const AGILEX5_CORE3_CLK: c_int = 35;
pub const AGILEX5_MPU_CLK: c_int = 36;
pub const AGILEX5_MPU_PERIPH_CLK: c_int = 37;
pub const AGILEX5_MPU_CCU_CLK: c_int = 38;
pub const AGILEX5_L4_MAIN_CLK: c_int = 39;
pub const AGILEX5_L4_MP_CLK: c_int = 40;
pub const AGILEX5_L4_SYS_FREE_CLK: c_int = 41;
pub const AGILEX5_L4_SP_CLK: c_int = 42;
pub const AGILEX5_CS_AT_CLK: c_int = 43;
pub const AGILEX5_CS_TRACE_CLK: c_int = 44;
pub const AGILEX5_CS_PDBG_CLK: c_int = 45;
pub const AGILEX5_EMAC1_CLK: c_int = 47;
pub const AGILEX5_EMAC2_CLK: c_int = 48;
pub const AGILEX5_EMAC_PTP_CLK: c_int = 49;
pub const AGILEX5_GPIO_DB_CLK: c_int = 50;
pub const AGILEX5_S2F_USER0_CLK: c_int = 51;
pub const AGILEX5_S2F_USER1_CLK: c_int = 52;
pub const AGILEX5_PSI_REF_CLK: c_int = 53;
pub const AGILEX5_USB31_SUSPEND_CLK: c_int = 54;
pub const AGILEX5_EMAC0_CLK: c_int = 46;
pub const AGILEX5_USB31_BUS_CLK_EARLY: c_int = 55;
pub const AGILEX5_USB2OTG_HCLK: c_int = 56;
pub const AGILEX5_SPIM_0_CLK: c_int = 57;
pub const AGILEX5_SPIM_1_CLK: c_int = 58;
pub const AGILEX5_SPIS_0_CLK: c_int = 59;
pub const AGILEX5_SPIS_1_CLK: c_int = 60;
pub const AGILEX5_DMA_CORE_CLK: c_int = 61;
pub const AGILEX5_DMA_HS_CLK: c_int = 62;
pub const AGILEX5_I3C_0_CORE_CLK: c_int = 63;
pub const AGILEX5_I3C_1_CORE_CLK: c_int = 64;
pub const AGILEX5_I2C_0_PCLK: c_int = 65;
pub const AGILEX5_I2C_1_PCLK: c_int = 66;
pub const AGILEX5_I2C_EMAC0_PCLK: c_int = 67;
pub const AGILEX5_I2C_EMAC1_PCLK: c_int = 68;
pub const AGILEX5_I2C_EMAC2_PCLK: c_int = 69;
pub const AGILEX5_UART_0_PCLK: c_int = 70;
pub const AGILEX5_UART_1_PCLK: c_int = 71;
pub const AGILEX5_SPTIMER_0_PCLK: c_int = 72;
pub const AGILEX5_SPTIMER_1_PCLK: c_int = 73;
pub const AGILEX5_DFI_CLK: c_int = 74;
pub const AGILEX5_NAND_NF_CLK: c_int = 75;
pub const AGILEX5_NAND_BCH_CLK: c_int = 76;
pub const AGILEX5_SDMMC_SDPHY_REG_CLK: c_int = 77;
pub const AGILEX5_SDMCLK: c_int = 78;
pub const AGILEX5_SOFTPHY_REG_PCLK: c_int = 79;
pub const AGILEX5_SOFTPHY_PHY_CLK: c_int = 80;
pub const AGILEX5_SOFTPHY_CTRL_CLK: c_int = 81;
pub const AGILEX5_NUM_CLKS: c_int = 82;
