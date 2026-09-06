//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/altr,rst-mgr-s10.h
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
// Copyright (C) 2016 Intel Corporation. All rights reserved
// Copyright (C) 2016 Altera Corporation. All rights reserved
//
// derived from Steffen Trumtrar's "altr,rst-mgr-a10.h"
//
// MPUMODRST
pub const CPU0_RESET: c_int = 0;
pub const CPU1_RESET: c_int = 1;
pub const CPU2_RESET: c_int = 2;
pub const CPU3_RESET: c_int = 3;
// PER0MODRST
pub const EMAC0_RESET: c_int = 32;
pub const EMAC1_RESET: c_int = 33;
pub const EMAC2_RESET: c_int = 34;
pub const USB0_RESET: c_int = 35;
pub const USB1_RESET: c_int = 36;
pub const NAND_RESET: c_int = 37;
pub const COMBOPHY_RESET: c_int = 38;
pub const SDMMC_RESET: c_int = 39;
pub const EMAC0_OCP_RESET: c_int = 40;
pub const EMAC1_OCP_RESET: c_int = 41;
pub const EMAC2_OCP_RESET: c_int = 42;
pub const USB0_OCP_RESET: c_int = 43;
pub const USB1_OCP_RESET: c_int = 44;
pub const NAND_OCP_RESET: c_int = 45;
// 46 is empty
pub const SDMMC_OCP_RESET: c_int = 47;
pub const DMA_RESET: c_int = 48;
pub const SPIM0_RESET: c_int = 49;
pub const SPIM1_RESET: c_int = 50;
pub const SPIS0_RESET: c_int = 51;
pub const SPIS1_RESET: c_int = 52;
pub const DMA_OCP_RESET: c_int = 53;
pub const EMAC_PTP_RESET: c_int = 54;
// 55 is empty
pub const DMAIF0_RESET: c_int = 56;
pub const DMAIF1_RESET: c_int = 57;
pub const DMAIF2_RESET: c_int = 58;
pub const DMAIF3_RESET: c_int = 59;
pub const DMAIF4_RESET: c_int = 60;
pub const DMAIF5_RESET: c_int = 61;
pub const DMAIF6_RESET: c_int = 62;
pub const DMAIF7_RESET: c_int = 63;
// PER1MODRST
pub const WATCHDOG0_RESET: c_int = 64;
pub const WATCHDOG1_RESET: c_int = 65;
pub const WATCHDOG2_RESET: c_int = 66;
pub const WATCHDOG3_RESET: c_int = 67;
pub const L4SYSTIMER0_RESET: c_int = 68;
pub const L4SYSTIMER1_RESET: c_int = 69;
pub const SPTIMER0_RESET: c_int = 70;
pub const SPTIMER1_RESET: c_int = 71;
pub const I2C0_RESET: c_int = 72;
pub const I2C1_RESET: c_int = 73;
pub const I2C2_RESET: c_int = 74;
pub const I2C3_RESET: c_int = 75;
pub const I2C4_RESET: c_int = 76;
pub const I3C0_RESET: c_int = 77;
pub const I3C1_RESET: c_int = 78;
// 79 is empty
pub const UART0_RESET: c_int = 80;
pub const UART1_RESET: c_int = 81;
// 82-87 is empty
pub const GPIO0_RESET: c_int = 88;
pub const GPIO1_RESET: c_int = 89;
pub const WATCHDOG4_RESET: c_int = 90;
// BRGMODRST
pub const SOC2FPGA_RESET: c_int = 96;
pub const LWHPS2FPGA_RESET: c_int = 97;
pub const FPGA2SOC_RESET: c_int = 98;
pub const F2SSDRAM0_RESET: c_int = 99;
pub const F2SSDRAM1_RESET: c_int = 100;
pub const F2SSDRAM2_RESET: c_int = 101;
pub const DDRSCH_RESET: c_int = 102;
// COLDMODRST
pub const CPUPO0_RESET: c_int = 160;
pub const CPUPO1_RESET: c_int = 161;
pub const CPUPO2_RESET: c_int = 162;
pub const CPUPO3_RESET: c_int = 163;
// 164-167 is empty
pub const L2_RESET: c_int = 168;
// DBGMODRST
pub const DBG_RESET: c_int = 224;
pub const CSDAP_RESET: c_int = 225;
// TAPMODRST
pub const TAP_RESET: c_int = 256;
