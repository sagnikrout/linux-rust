//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci-xenon.h
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
// Copyright (C) 2016 Marvell, All Rights Reserved.
//
// Author:	Hu Ziji <huziji@marvell.com>
// Date:	2016-8-24
//
// Register Offset of Xenon SDHC self-defined register
pub const XENON_SYS_CFG_INFO: c_uint = 0x0104;
pub const XENON_SLOT_TYPE_SDIO_SHIFT: c_int = 24;
pub const XENON_NR_SUPPORTED_SLOT_MASK: c_uint = 0x7;
pub const XENON_SYS_OP_CTRL: c_uint = 0x0108;

pub const XENON_SDCLK_IDLEOFF_ENABLE_SHIFT: c_int = 8;
pub const XENON_SLOT_ENABLE_SHIFT: c_int = 0;
pub const XENON_SYS_EXT_OP_CTRL: c_uint = 0x010C;

pub const XENON_SLOT_OP_STATUS_CTRL: c_uint = 0x0128;
pub const XENON_TUN_CONSECUTIVE_TIMES_SHIFT: c_int = 16;
pub const XENON_TUN_CONSECUTIVE_TIMES_MASK: c_uint = 0x7;
pub const XENON_TUN_CONSECUTIVE_TIMES: c_uint = 0x4;
pub const XENON_TUNING_STEP_SHIFT: c_int = 12;
pub const XENON_TUNING_STEP_MASK: c_uint = 0xF;

pub const XENON_SLOT_EMMC_CTRL: c_uint = 0x0130;

pub const XENON_SLOT_RETUNING_REQ_CTRL: c_uint = 0x0144;
// retuning compatible
pub const XENON_RETUNING_COMPATIBLE: c_uint = 0x1;
pub const XENON_SLOT_EXT_PRESENT_STATE: c_uint = 0x014C;
pub const XENON_DLL_LOCK_STATE: c_uint = 0x1;
pub const XENON_SLOT_DLL_CUR_DLY_VAL: c_uint = 0x0150;
// Tuning Parameter
pub const XENON_TMR_RETUN_NO_PRESENT: c_uint = 0xF;
pub const XENON_DEF_TUNING_COUNT: c_uint = 0x9;
pub const XENON_DEFAULT_SDCLK_FREQ: c_int = 400000;
pub const XENON_LOWEST_SDCLK_FREQ: c_int = 100000;
// Xenon specific Mode Select value
pub const XENON_CTRL_HS200: c_uint = 0x5;
pub const XENON_CTRL_HS400: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xenon_variant {
    XENON_A3700,
    XENON_AP806,
    XENON_AP807,
    XENON_CP110,
    XENON_AC5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenon_priv {
    pub tuning_count: c_uchar,
// idx of SDHC
    pub sdhc_id: u8,
//
// eMMC/SD/SDIO require different register settings.
// Xenon driver has to recognize card type
// before mmc_host->card is not available.
// This field records the card type during init.
// It is updated in xenon_init_card().
//
// It is only valid during initialization after it is updated.
// Do not access this variable in normal transfers after
// initialization completes.
//
    pub init_card_type: c_uint,
//
// The bus_width, timing, and clock fields in below
// record the current ios setting of Xenon SDHC.
// Driver will adjust PHY setting if any change to
// ios affects PHY timing.
//
    pub bus_width: c_uchar,
    pub timing: c_uchar,
    pub clock: c_uint,
    pub axi_clk: *mut clk,
    pub phy_type: c_int,
//
// Contains board-specific PHY parameters
// passed from device tree.
//
    pub phy_params: *mut c_void,
    pub emmc_phy_regs: *mut xenon_emmc_phy_regs,
    pub restore_needed: bool,
    pub hw_version: xenon_variant,
}

extern "C" {
    pub fn xenon_phy_adj(host: *mut sdhci_host, ios: *mut mmc_ios) -> c_int;
}
