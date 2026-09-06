//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci-esdhc.h
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
// Freescale eSDHC controller driver generics for OF and pltfm.
//
// Copyright (c) 2007 Freescale Semiconductor, Inc.
// Copyright (c) 2009 MontaVista Software, Inc.
// Copyright (c) 2010 Pengutronix e.K.
// Copyright 2020 NXP
// Author: Wolfram Sang <kernel@pengutronix.de>
//
// Ops and quirks for the Freescale eSDHC controller.
//

// pltfm-specific
pub const ESDHC_HOST_CONTROL_LE: c_uint = 0x20;
//
// eSDHC register definition
//
// Present State Register
pub const ESDHC_PRSSTAT: c_uint = 0x24;
pub const ESDHC_CLOCK_GATE_OFF: c_uint = 0x00000080;
pub const ESDHC_CLOCK_STABLE: c_uint = 0x00000008;
// Protocol Control Register
pub const ESDHC_PROCTL: c_uint = 0x28;
pub const ESDHC_VOLT_SEL: c_uint = 0x00000400;

pub const ESDHC_HOST_CONTROL_RES: c_uint = 0x01;
// System Control Register
pub const ESDHC_SYSTEM_CONTROL: c_uint = 0x2c;
pub const ESDHC_CLOCK_MASK: c_uint = 0x0000fff0;
pub const ESDHC_PREDIV_SHIFT: c_int = 8;
pub const ESDHC_DIVIDER_SHIFT: c_int = 4;
pub const ESDHC_CLOCK_SDCLKEN: c_uint = 0x00000008;
pub const ESDHC_CLOCK_PEREN: c_uint = 0x00000004;
pub const ESDHC_CLOCK_HCKEN: c_uint = 0x00000002;
pub const ESDHC_CLOCK_IPGEN: c_uint = 0x00000001;
// System Control 2 Register
pub const ESDHC_SYSTEM_CONTROL_2: c_uint = 0x3c;
pub const ESDHC_SMPCLKSEL: c_uint = 0x00800000;
pub const ESDHC_EXTN: c_uint = 0x00400000;
// Host Controller Capabilities Register 2
pub const ESDHC_CAPABILITIES_1: c_uint = 0x114;
// Tuning Block Control Register
pub const ESDHC_TBCTL: c_uint = 0x120;
pub const ESDHC_HS400_WNDW_ADJUST: c_uint = 0x00000040;
pub const ESDHC_HS400_MODE: c_uint = 0x00000010;
pub const ESDHC_TB_EN: c_uint = 0x00000004;
pub const ESDHC_TB_MODE_MASK: c_uint = 0x00000003;
pub const ESDHC_TB_MODE_SW: c_uint = 0x00000003;
pub const ESDHC_TB_MODE_3: c_uint = 0x00000002;
pub const ESDHC_TBSTAT: c_uint = 0x124;
pub const ESDHC_TBPTR: c_uint = 0x128;
pub const ESDHC_WNDW_STRT_PTR_SHIFT: c_int = 8;

pub const ESDHC_WNDW_END_PTR_MASK: c_uint = 0x7f;
// SD Clock Control Register
pub const ESDHC_SDCLKCTL: c_uint = 0x144;
pub const ESDHC_LPBK_CLK_SEL: c_uint = 0x80000000;
pub const ESDHC_CMD_CLK_CTL: c_uint = 0x00008000;
// SD Timing Control Register
pub const ESDHC_SDTIMNGCTL: c_uint = 0x148;
pub const ESDHC_FLW_CTL_BG: c_uint = 0x00008000;
// DLL Config 0 Register
pub const ESDHC_DLLCFG0: c_uint = 0x160;
pub const ESDHC_DLL_ENABLE: c_uint = 0x80000000;
pub const ESDHC_DLL_RESET: c_uint = 0x40000000;
pub const ESDHC_DLL_FREQ_SEL: c_uint = 0x08000000;
// DLL Config 1 Register
pub const ESDHC_DLLCFG1: c_uint = 0x164;
pub const ESDHC_DLL_PD_PULSE_STRETCH_SEL: c_uint = 0x80000000;
// DLL Status 0 Register
pub const ESDHC_DLLSTAT0: c_uint = 0x170;
pub const ESDHC_DLL_STS_SLV_LOCK: c_uint = 0x08000000;
// Control Register for DMA transfer
pub const ESDHC_DMA_SYSCTL: c_uint = 0x40c;
pub const ESDHC_PERIPHERAL_CLK_SEL: c_uint = 0x00080000;
pub const ESDHC_FLUSH_ASYNC_FIFO: c_uint = 0x00040000;
pub const ESDHC_DMA_SNOOP: c_uint = 0x00000040;
