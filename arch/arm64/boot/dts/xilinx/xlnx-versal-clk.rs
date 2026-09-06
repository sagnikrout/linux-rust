//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/xilinx/xlnx-versal-clk.h
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
// Copyright (C) 2019 - 2022, Xilinx, Inc.
// Copyright (C) 2022 - 2026, Advanced Micro Devices, Inc.
//
pub const PMC_PLL: c_int = 1;
pub const APU_PLL: c_int = 2;
pub const RPU_PLL: c_int = 3;
pub const CPM_PLL: c_int = 4;
pub const NOC_PLL: c_int = 5;
pub const PLL_MAX: c_int = 6;
pub const PMC_PRESRC: c_int = 7;
pub const PMC_POSTCLK: c_int = 8;
pub const PMC_PLL_OUT: c_int = 9;
pub const PPLL: c_int = 10;
pub const NOC_PRESRC: c_int = 11;
pub const NOC_POSTCLK: c_int = 12;
pub const NOC_PLL_OUT: c_int = 13;
pub const NPLL: c_int = 14;
pub const APU_PRESRC: c_int = 15;
pub const APU_POSTCLK: c_int = 16;
pub const APU_PLL_OUT: c_int = 17;
pub const APLL: c_int = 18;
pub const RPU_PRESRC: c_int = 19;
pub const RPU_POSTCLK: c_int = 20;
pub const RPU_PLL_OUT: c_int = 21;
pub const RPLL: c_int = 22;
pub const CPM_PRESRC: c_int = 23;
pub const CPM_POSTCLK: c_int = 24;
pub const CPM_PLL_OUT: c_int = 25;
pub const CPLL: c_int = 26;
pub const PPLL_TO_XPD: c_int = 27;
pub const NPLL_TO_XPD: c_int = 28;
pub const APLL_TO_XPD: c_int = 29;
pub const RPLL_TO_XPD: c_int = 30;
pub const EFUSE_REF: c_int = 31;
pub const SYSMON_REF: c_int = 32;
pub const IRO_SUSPEND_REF: c_int = 33;
pub const USB_SUSPEND: c_int = 34;
pub const SWITCH_TIMEOUT: c_int = 35;
pub const RCLK_PMC: c_int = 36;
pub const RCLK_LPD: c_int = 37;
pub const WDT: c_int = 38;
pub const TTC0: c_int = 39;
pub const TTC1: c_int = 40;
pub const TTC2: c_int = 41;
pub const TTC3: c_int = 42;
pub const GEM_TSU: c_int = 43;
pub const GEM_TSU_LB: c_int = 44;
pub const MUXED_IRO_DIV2: c_int = 45;
pub const MUXED_IRO_DIV4: c_int = 46;
pub const PSM_REF: c_int = 47;
pub const GEM0_RX: c_int = 48;
pub const GEM0_TX: c_int = 49;
pub const GEM1_RX: c_int = 50;
pub const GEM1_TX: c_int = 51;
pub const CPM_CORE_REF: c_int = 52;
pub const CPM_LSBUS_REF: c_int = 53;
pub const CPM_DBG_REF: c_int = 54;
pub const CPM_AUX0_REF: c_int = 55;
pub const CPM_AUX1_REF: c_int = 56;
pub const QSPI_REF: c_int = 57;
pub const OSPI_REF: c_int = 58;
pub const SDIO0_REF: c_int = 59;
pub const SDIO1_REF: c_int = 60;
pub const PMC_LSBUS_REF: c_int = 61;
pub const I2C_REF: c_int = 62;
pub const TEST_PATTERN_REF: c_int = 63;
pub const DFT_OSC_REF: c_int = 64;
pub const PMC_PL0_REF: c_int = 65;
pub const PMC_PL1_REF: c_int = 66;
pub const PMC_PL2_REF: c_int = 67;
pub const PMC_PL3_REF: c_int = 68;
pub const CFU_REF: c_int = 69;
pub const SPARE_REF: c_int = 70;
pub const NPI_REF: c_int = 71;
pub const HSM0_REF: c_int = 72;
pub const HSM1_REF: c_int = 73;
pub const SD_DLL_REF: c_int = 74;
pub const FPD_TOP_SWITCH: c_int = 75;
pub const FPD_LSBUS: c_int = 76;
pub const ACPU: c_int = 77;
pub const DBG_TRACE: c_int = 78;
pub const DBG_FPD: c_int = 79;
pub const LPD_TOP_SWITCH: c_int = 80;
pub const ADMA: c_int = 81;
pub const LPD_LSBUS: c_int = 82;
pub const CPU_R5: c_int = 83;
pub const CPU_R5_CORE: c_int = 84;
pub const CPU_R5_OCM: c_int = 85;
pub const CPU_R5_OCM2: c_int = 86;
pub const IOU_SWITCH: c_int = 87;
pub const GEM0_REF: c_int = 88;
pub const GEM1_REF: c_int = 89;
pub const GEM_TSU_REF: c_int = 90;
pub const USB0_BUS_REF: c_int = 91;
pub const UART0_REF: c_int = 92;
pub const UART1_REF: c_int = 93;
pub const SPI0_REF: c_int = 94;
pub const SPI1_REF: c_int = 95;
pub const CAN0_REF: c_int = 96;
pub const CAN1_REF: c_int = 97;
pub const I2C0_REF: c_int = 98;
pub const I2C1_REF: c_int = 99;
pub const DBG_LPD: c_int = 100;
pub const TIMESTAMP_REF: c_int = 101;
pub const DBG_TSTMP: c_int = 102;
pub const CPM_TOPSW_REF: c_int = 103;
pub const USB3_DUAL_REF: c_int = 104;
pub const OUTCLK_MAX: c_int = 105;
pub const REF_CLK: c_int = 106;
pub const PL_ALT_REF_CLK: c_int = 107;
pub const MUXED_IRO: c_int = 108;
pub const PL_EXT: c_int = 109;
pub const PL_LB: c_int = 110;
pub const MIO_50_OR_51: c_int = 111;
pub const MIO_24_OR_25: c_int = 112;
