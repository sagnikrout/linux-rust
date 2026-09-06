//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/xilinx/xlnx-zynqmp-clk.h
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
// Xilinx Zynq MPSoC Firmware layer
//
// Copyright (C) 2014-2018 Xilinx, Inc.
//
pub const IOPLL: c_int = 0;
pub const RPLL: c_int = 1;
pub const APLL: c_int = 2;
pub const DPLL: c_int = 3;
pub const VPLL: c_int = 4;
pub const IOPLL_TO_FPD: c_int = 5;
pub const RPLL_TO_FPD: c_int = 6;
pub const APLL_TO_LPD: c_int = 7;
pub const DPLL_TO_LPD: c_int = 8;
pub const VPLL_TO_LPD: c_int = 9;
pub const ACPU: c_int = 10;
pub const ACPU_HALF: c_int = 11;
pub const DBF_FPD: c_int = 12;
pub const DBF_LPD: c_int = 13;
pub const DBG_TRACE: c_int = 14;
pub const DBG_TSTMP: c_int = 15;
pub const DP_VIDEO_REF: c_int = 16;
pub const DP_AUDIO_REF: c_int = 17;
pub const DP_STC_REF: c_int = 18;
pub const GDMA_REF: c_int = 19;
pub const DPDMA_REF: c_int = 20;
pub const DDR_REF: c_int = 21;
pub const SATA_REF: c_int = 22;
pub const PCIE_REF: c_int = 23;
pub const GPU_REF: c_int = 24;
pub const GPU_PP0_REF: c_int = 25;
pub const GPU_PP1_REF: c_int = 26;
pub const TOPSW_MAIN: c_int = 27;
pub const TOPSW_LSBUS: c_int = 28;
pub const GTGREF0_REF: c_int = 29;
pub const LPD_SWITCH: c_int = 30;
pub const LPD_LSBUS: c_int = 31;
pub const USB0_BUS_REF: c_int = 32;
pub const USB1_BUS_REF: c_int = 33;
pub const USB3_DUAL_REF: c_int = 34;
pub const USB0: c_int = 35;
pub const USB1: c_int = 36;
pub const CPU_R5: c_int = 37;
pub const CPU_R5_CORE: c_int = 38;
pub const CSU_SPB: c_int = 39;
pub const CSU_PLL: c_int = 40;
pub const PCAP: c_int = 41;
pub const IOU_SWITCH: c_int = 42;
pub const GEM_TSU_REF: c_int = 43;
pub const GEM_TSU: c_int = 44;
pub const GEM0_TX: c_int = 45;
pub const GEM1_TX: c_int = 46;
pub const GEM2_TX: c_int = 47;
pub const GEM3_TX: c_int = 48;
pub const GEM0_RX: c_int = 49;
pub const GEM1_RX: c_int = 50;
pub const GEM2_RX: c_int = 51;
pub const GEM3_RX: c_int = 52;
pub const QSPI_REF: c_int = 53;
pub const SDIO0_REF: c_int = 54;
pub const SDIO1_REF: c_int = 55;
pub const UART0_REF: c_int = 56;
pub const UART1_REF: c_int = 57;
pub const SPI0_REF: c_int = 58;
pub const SPI1_REF: c_int = 59;
pub const NAND_REF: c_int = 60;
pub const I2C0_REF: c_int = 61;
pub const I2C1_REF: c_int = 62;
pub const CAN0_REF: c_int = 63;
pub const CAN1_REF: c_int = 64;
pub const CAN0: c_int = 65;
pub const CAN1: c_int = 66;
pub const DLL_REF: c_int = 67;
pub const ADMA_REF: c_int = 68;
pub const TIMESTAMP_REF: c_int = 69;
pub const AMS_REF: c_int = 70;
pub const PL0_REF: c_int = 71;
pub const PL1_REF: c_int = 72;
pub const PL2_REF: c_int = 73;
pub const PL3_REF: c_int = 74;
pub const WDT: c_int = 75;
pub const IOPLL_INT: c_int = 76;
pub const IOPLL_PRE_SRC: c_int = 77;
pub const IOPLL_HALF: c_int = 78;
pub const IOPLL_INT_MUX: c_int = 79;
pub const IOPLL_POST_SRC: c_int = 80;
pub const RPLL_INT: c_int = 81;
pub const RPLL_PRE_SRC: c_int = 82;
pub const RPLL_HALF: c_int = 83;
pub const RPLL_INT_MUX: c_int = 84;
pub const RPLL_POST_SRC: c_int = 85;
pub const APLL_INT: c_int = 86;
pub const APLL_PRE_SRC: c_int = 87;
pub const APLL_HALF: c_int = 88;
pub const APLL_INT_MUX: c_int = 89;
pub const APLL_POST_SRC: c_int = 90;
pub const DPLL_INT: c_int = 91;
pub const DPLL_PRE_SRC: c_int = 92;
pub const DPLL_HALF: c_int = 93;
pub const DPLL_INT_MUX: c_int = 94;
pub const DPLL_POST_SRC: c_int = 95;
pub const VPLL_INT: c_int = 96;
pub const VPLL_PRE_SRC: c_int = 97;
pub const VPLL_HALF: c_int = 98;
pub const VPLL_INT_MUX: c_int = 99;
pub const VPLL_POST_SRC: c_int = 100;
pub const CAN0_MIO: c_int = 101;
pub const CAN1_MIO: c_int = 102;
pub const ACPU_FULL: c_int = 103;
pub const GEM0_REF: c_int = 104;
pub const GEM1_REF: c_int = 105;
pub const GEM2_REF: c_int = 106;
pub const GEM3_REF: c_int = 107;
pub const GEM0_REF_UNG: c_int = 108;
pub const GEM1_REF_UNG: c_int = 109;
pub const GEM2_REF_UNG: c_int = 110;
pub const GEM3_REF_UNG: c_int = 111;
pub const LPD_WDT: c_int = 112;
