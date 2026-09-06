//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/xlnx-zynqmp-resets.h
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
// Copyright (C) 2018 Xilinx, Inc.
//
pub const ZYNQMP_RESET_PCIE_CFG: c_int = 0;
pub const ZYNQMP_RESET_PCIE_BRIDGE: c_int = 1;
pub const ZYNQMP_RESET_PCIE_CTRL: c_int = 2;
pub const ZYNQMP_RESET_DP: c_int = 3;
pub const ZYNQMP_RESET_SWDT_CRF: c_int = 4;
pub const ZYNQMP_RESET_AFI_FM5: c_int = 5;
pub const ZYNQMP_RESET_AFI_FM4: c_int = 6;
pub const ZYNQMP_RESET_AFI_FM3: c_int = 7;
pub const ZYNQMP_RESET_AFI_FM2: c_int = 8;
pub const ZYNQMP_RESET_AFI_FM1: c_int = 9;
pub const ZYNQMP_RESET_AFI_FM0: c_int = 10;
pub const ZYNQMP_RESET_GDMA: c_int = 11;
pub const ZYNQMP_RESET_GPU_PP1: c_int = 12;
pub const ZYNQMP_RESET_GPU_PP0: c_int = 13;
pub const ZYNQMP_RESET_GPU: c_int = 14;
pub const ZYNQMP_RESET_GT: c_int = 15;
pub const ZYNQMP_RESET_SATA: c_int = 16;
pub const ZYNQMP_RESET_ACPU3_PWRON: c_int = 17;
pub const ZYNQMP_RESET_ACPU2_PWRON: c_int = 18;
pub const ZYNQMP_RESET_ACPU1_PWRON: c_int = 19;
pub const ZYNQMP_RESET_ACPU0_PWRON: c_int = 20;
pub const ZYNQMP_RESET_APU_L2: c_int = 21;
pub const ZYNQMP_RESET_ACPU3: c_int = 22;
pub const ZYNQMP_RESET_ACPU2: c_int = 23;
pub const ZYNQMP_RESET_ACPU1: c_int = 24;
pub const ZYNQMP_RESET_ACPU0: c_int = 25;
pub const ZYNQMP_RESET_DDR: c_int = 26;
pub const ZYNQMP_RESET_APM_FPD: c_int = 27;
pub const ZYNQMP_RESET_SOFT: c_int = 28;
pub const ZYNQMP_RESET_GEM0: c_int = 29;
pub const ZYNQMP_RESET_GEM1: c_int = 30;
pub const ZYNQMP_RESET_GEM2: c_int = 31;
pub const ZYNQMP_RESET_GEM3: c_int = 32;
pub const ZYNQMP_RESET_QSPI: c_int = 33;
pub const ZYNQMP_RESET_UART0: c_int = 34;
pub const ZYNQMP_RESET_UART1: c_int = 35;
pub const ZYNQMP_RESET_SPI0: c_int = 36;
pub const ZYNQMP_RESET_SPI1: c_int = 37;
pub const ZYNQMP_RESET_SDIO0: c_int = 38;
pub const ZYNQMP_RESET_SDIO1: c_int = 39;
pub const ZYNQMP_RESET_CAN0: c_int = 40;
pub const ZYNQMP_RESET_CAN1: c_int = 41;
pub const ZYNQMP_RESET_I2C0: c_int = 42;
pub const ZYNQMP_RESET_I2C1: c_int = 43;
pub const ZYNQMP_RESET_TTC0: c_int = 44;
pub const ZYNQMP_RESET_TTC1: c_int = 45;
pub const ZYNQMP_RESET_TTC2: c_int = 46;
pub const ZYNQMP_RESET_TTC3: c_int = 47;
pub const ZYNQMP_RESET_SWDT_CRL: c_int = 48;
pub const ZYNQMP_RESET_NAND: c_int = 49;
pub const ZYNQMP_RESET_ADMA: c_int = 50;
pub const ZYNQMP_RESET_GPIO: c_int = 51;
pub const ZYNQMP_RESET_IOU_CC: c_int = 52;
pub const ZYNQMP_RESET_TIMESTAMP: c_int = 53;
pub const ZYNQMP_RESET_RPU_R50: c_int = 54;
pub const ZYNQMP_RESET_RPU_R51: c_int = 55;
pub const ZYNQMP_RESET_RPU_AMBA: c_int = 56;
pub const ZYNQMP_RESET_OCM: c_int = 57;
pub const ZYNQMP_RESET_RPU_PGE: c_int = 58;
pub const ZYNQMP_RESET_USB0_CORERESET: c_int = 59;
pub const ZYNQMP_RESET_USB1_CORERESET: c_int = 60;
pub const ZYNQMP_RESET_USB0_HIBERRESET: c_int = 61;
pub const ZYNQMP_RESET_USB1_HIBERRESET: c_int = 62;
pub const ZYNQMP_RESET_USB0_APB: c_int = 63;
pub const ZYNQMP_RESET_USB1_APB: c_int = 64;
pub const ZYNQMP_RESET_IPI: c_int = 65;
pub const ZYNQMP_RESET_APM_LPD: c_int = 66;
pub const ZYNQMP_RESET_RTC: c_int = 67;
pub const ZYNQMP_RESET_SYSMON: c_int = 68;
pub const ZYNQMP_RESET_AFI_FM6: c_int = 69;
pub const ZYNQMP_RESET_LPD_SWDT: c_int = 70;
pub const ZYNQMP_RESET_FPD: c_int = 71;
pub const ZYNQMP_RESET_RPU_DBG1: c_int = 72;
pub const ZYNQMP_RESET_RPU_DBG0: c_int = 73;
pub const ZYNQMP_RESET_DBG_LPD: c_int = 74;
pub const ZYNQMP_RESET_DBG_FPD: c_int = 75;
pub const ZYNQMP_RESET_APLL: c_int = 76;
pub const ZYNQMP_RESET_DPLL: c_int = 77;
pub const ZYNQMP_RESET_VPLL: c_int = 78;
pub const ZYNQMP_RESET_IOPLL: c_int = 79;
pub const ZYNQMP_RESET_RPLL: c_int = 80;
pub const ZYNQMP_RESET_GPO3_PL_0: c_int = 81;
pub const ZYNQMP_RESET_GPO3_PL_1: c_int = 82;
pub const ZYNQMP_RESET_GPO3_PL_2: c_int = 83;
pub const ZYNQMP_RESET_GPO3_PL_3: c_int = 84;
pub const ZYNQMP_RESET_GPO3_PL_4: c_int = 85;
pub const ZYNQMP_RESET_GPO3_PL_5: c_int = 86;
pub const ZYNQMP_RESET_GPO3_PL_6: c_int = 87;
pub const ZYNQMP_RESET_GPO3_PL_7: c_int = 88;
pub const ZYNQMP_RESET_GPO3_PL_8: c_int = 89;
pub const ZYNQMP_RESET_GPO3_PL_9: c_int = 90;
pub const ZYNQMP_RESET_GPO3_PL_10: c_int = 91;
pub const ZYNQMP_RESET_GPO3_PL_11: c_int = 92;
pub const ZYNQMP_RESET_GPO3_PL_12: c_int = 93;
pub const ZYNQMP_RESET_GPO3_PL_13: c_int = 94;
pub const ZYNQMP_RESET_GPO3_PL_14: c_int = 95;
pub const ZYNQMP_RESET_GPO3_PL_15: c_int = 96;
pub const ZYNQMP_RESET_GPO3_PL_16: c_int = 97;
pub const ZYNQMP_RESET_GPO3_PL_17: c_int = 98;
pub const ZYNQMP_RESET_GPO3_PL_18: c_int = 99;
pub const ZYNQMP_RESET_GPO3_PL_19: c_int = 100;
pub const ZYNQMP_RESET_GPO3_PL_20: c_int = 101;
pub const ZYNQMP_RESET_GPO3_PL_21: c_int = 102;
pub const ZYNQMP_RESET_GPO3_PL_22: c_int = 103;
pub const ZYNQMP_RESET_GPO3_PL_23: c_int = 104;
pub const ZYNQMP_RESET_GPO3_PL_24: c_int = 105;
pub const ZYNQMP_RESET_GPO3_PL_25: c_int = 106;
pub const ZYNQMP_RESET_GPO3_PL_26: c_int = 107;
pub const ZYNQMP_RESET_GPO3_PL_27: c_int = 108;
pub const ZYNQMP_RESET_GPO3_PL_28: c_int = 109;
pub const ZYNQMP_RESET_GPO3_PL_29: c_int = 110;
pub const ZYNQMP_RESET_GPO3_PL_30: c_int = 111;
pub const ZYNQMP_RESET_GPO3_PL_31: c_int = 112;
pub const ZYNQMP_RESET_RPU_LS: c_int = 113;
pub const ZYNQMP_RESET_PS_ONLY: c_int = 114;
pub const ZYNQMP_RESET_PL: c_int = 115;
pub const ZYNQMP_RESET_PS_PL0: c_int = 116;
pub const ZYNQMP_RESET_PS_PL1: c_int = 117;
pub const ZYNQMP_RESET_PS_PL2: c_int = 118;
pub const ZYNQMP_RESET_PS_PL3: c_int = 119;
