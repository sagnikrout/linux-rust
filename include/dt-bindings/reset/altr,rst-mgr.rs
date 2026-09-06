//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/altr,rst-mgr.h
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
// Copyright (c) 2014, Steffen Trumtrar <s.trumtrar@pengutronix.de>
//
// MPUMODRST
pub const CPU0_RESET: c_int = 0;
pub const CPU1_RESET: c_int = 1;
pub const WDS_RESET: c_int = 2;
pub const SCUPER_RESET: c_int = 3;
pub const L2_RESET: c_int = 4;
// PERMODRST
pub const EMAC0_RESET: c_int = 32;
pub const EMAC1_RESET: c_int = 33;
pub const USB0_RESET: c_int = 34;
pub const USB1_RESET: c_int = 35;
pub const NAND_RESET: c_int = 36;
pub const QSPI_RESET: c_int = 37;
pub const L4WD0_RESET: c_int = 38;
pub const L4WD1_RESET: c_int = 39;
pub const OSC1TIMER0_RESET: c_int = 40;
pub const OSC1TIMER1_RESET: c_int = 41;
pub const SPTIMER0_RESET: c_int = 42;
pub const SPTIMER1_RESET: c_int = 43;
pub const I2C0_RESET: c_int = 44;
pub const I2C1_RESET: c_int = 45;
pub const I2C2_RESET: c_int = 46;
pub const I2C3_RESET: c_int = 47;
pub const UART0_RESET: c_int = 48;
pub const UART1_RESET: c_int = 49;
pub const SPIM0_RESET: c_int = 50;
pub const SPIM1_RESET: c_int = 51;
pub const SPIS0_RESET: c_int = 52;
pub const SPIS1_RESET: c_int = 53;
pub const SDMMC_RESET: c_int = 54;
pub const CAN0_RESET: c_int = 55;
pub const CAN1_RESET: c_int = 56;
pub const GPIO0_RESET: c_int = 57;
pub const GPIO1_RESET: c_int = 58;
pub const GPIO2_RESET: c_int = 59;
pub const DMA_RESET: c_int = 60;
pub const SDR_RESET: c_int = 61;
// PER2MODRST
pub const DMAIF0_RESET: c_int = 64;
pub const DMAIF1_RESET: c_int = 65;
pub const DMAIF2_RESET: c_int = 66;
pub const DMAIF3_RESET: c_int = 67;
pub const DMAIF4_RESET: c_int = 68;
pub const DMAIF5_RESET: c_int = 69;
pub const DMAIF6_RESET: c_int = 70;
pub const DMAIF7_RESET: c_int = 71;
// BRGMODRST
pub const HPS2FPGA_RESET: c_int = 96;
pub const LWHPS2FPGA_RESET: c_int = 97;
pub const FPGA2HPS_RESET: c_int = 98;
// MISCMODRST
pub const ROM_RESET: c_int = 128;
pub const OCRAM_RESET: c_int = 129;
pub const SYSMGR_RESET: c_int = 130;
pub const SYSMGRCOLD_RESET: c_int = 131;
pub const FPGAMGR_RESET: c_int = 132;
pub const ACPIDMAP_RESET: c_int = 133;
pub const S2F_RESET: c_int = 134;
pub const S2FCOLD_RESET: c_int = 135;
pub const NRSTPIN_RESET: c_int = 136;
pub const TIMESTAMPCOLD_RESET: c_int = 137;
pub const CLKMGRCOLD_RESET: c_int = 138;
pub const SCANMGR_RESET: c_int = 139;
pub const FRZCTRLCOLD_RESET: c_int = 140;
pub const SYSDBG_RESET: c_int = 141;
pub const DBG_RESET: c_int = 142;
pub const TAPCOLD_RESET: c_int = 143;
pub const SDRCOLD_RESET: c_int = 144;
