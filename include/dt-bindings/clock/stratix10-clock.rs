//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stratix10-clock.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2017, Intel Corporation
//
// fixed rate clocks
pub const STRATIX10_OSC1: c_int = 0;
pub const STRATIX10_CB_INTOSC_HS_DIV2_CLK: c_int = 1;
pub const STRATIX10_CB_INTOSC_LS_CLK: c_int = 2;
pub const STRATIX10_F2S_FREE_CLK: c_int = 3;
// fixed factor clocks
pub const STRATIX10_L4_SYS_FREE_CLK: c_int = 4;
pub const STRATIX10_MPU_PERIPH_CLK: c_int = 5;
pub const STRATIX10_MPU_L2RAM_CLK: c_int = 6;
pub const STRATIX10_SDMMC_CIU_CLK: c_int = 7;
// PLL clocks
pub const STRATIX10_MAIN_PLL_CLK: c_int = 8;
pub const STRATIX10_PERIPH_PLL_CLK: c_int = 9;
pub const STRATIX10_BOOT_CLK: c_int = 10;
// Periph clocks
pub const STRATIX10_MAIN_MPU_BASE_CLK: c_int = 11;
pub const STRATIX10_MAIN_NOC_BASE_CLK: c_int = 12;
pub const STRATIX10_MAIN_EMACA_CLK: c_int = 13;
pub const STRATIX10_MAIN_EMACB_CLK: c_int = 14;
pub const STRATIX10_MAIN_EMAC_PTP_CLK: c_int = 15;
pub const STRATIX10_MAIN_GPIO_DB_CLK: c_int = 16;
pub const STRATIX10_MAIN_SDMMC_CLK: c_int = 17;
pub const STRATIX10_MAIN_S2F_USR0_CLK: c_int = 18;
pub const STRATIX10_MAIN_S2F_USR1_CLK: c_int = 19;
pub const STRATIX10_MAIN_PSI_REF_CLK: c_int = 20;
pub const STRATIX10_PERI_MPU_BASE_CLK: c_int = 21;
pub const STRATIX10_PERI_NOC_BASE_CLK: c_int = 22;
pub const STRATIX10_PERI_EMACA_CLK: c_int = 23;
pub const STRATIX10_PERI_EMACB_CLK: c_int = 24;
pub const STRATIX10_PERI_EMAC_PTP_CLK: c_int = 25;
pub const STRATIX10_PERI_GPIO_DB_CLK: c_int = 26;
pub const STRATIX10_PERI_SDMMC_CLK: c_int = 27;
pub const STRATIX10_PERI_S2F_USR0_CLK: c_int = 28;
pub const STRATIX10_PERI_S2F_USR1_CLK: c_int = 29;
pub const STRATIX10_PERI_PSI_REF_CLK: c_int = 30;
pub const STRATIX10_MPU_FREE_CLK: c_int = 31;
pub const STRATIX10_NOC_FREE_CLK: c_int = 32;
pub const STRATIX10_S2F_USR0_CLK: c_int = 33;
pub const STRATIX10_NOC_CLK: c_int = 34;
pub const STRATIX10_EMAC_A_FREE_CLK: c_int = 35;
pub const STRATIX10_EMAC_B_FREE_CLK: c_int = 36;
pub const STRATIX10_EMAC_PTP_FREE_CLK: c_int = 37;
pub const STRATIX10_GPIO_DB_FREE_CLK: c_int = 38;
pub const STRATIX10_SDMMC_FREE_CLK: c_int = 39;
pub const STRATIX10_S2F_USER1_FREE_CLK: c_int = 40;
pub const STRATIX10_PSI_REF_FREE_CLK: c_int = 41;
// Gate clocks
pub const STRATIX10_MPU_CLK: c_int = 42;
pub const STRATIX10_L4_MAIN_CLK: c_int = 43;
pub const STRATIX10_L4_MP_CLK: c_int = 44;
pub const STRATIX10_L4_SP_CLK: c_int = 45;
pub const STRATIX10_CS_AT_CLK: c_int = 46;
pub const STRATIX10_CS_TRACE_CLK: c_int = 47;
pub const STRATIX10_CS_PDBG_CLK: c_int = 48;
pub const STRATIX10_CS_TIMER_CLK: c_int = 49;
pub const STRATIX10_S2F_USER0_CLK: c_int = 50;
pub const STRATIX10_S2F_USER1_CLK: c_int = 51;
pub const STRATIX10_EMAC0_CLK: c_int = 52;
pub const STRATIX10_EMAC1_CLK: c_int = 53;
pub const STRATIX10_EMAC2_CLK: c_int = 54;
pub const STRATIX10_EMAC_PTP_CLK: c_int = 55;
pub const STRATIX10_GPIO_DB_CLK: c_int = 56;
pub const STRATIX10_SDMMC_CLK: c_int = 57;
pub const STRATIX10_PSI_REF_CLK: c_int = 58;
pub const STRATIX10_USB_CLK: c_int = 59;
pub const STRATIX10_SPI_M_CLK: c_int = 60;
pub const STRATIX10_NAND_CLK: c_int = 61;
pub const STRATIX10_NAND_X_CLK: c_int = 62;
pub const STRATIX10_NAND_ECC_CLK: c_int = 63;
pub const STRATIX10_NUM_CLKS: c_int = 64;
