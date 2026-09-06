//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rz-du/rzg2l_mipi_dsi_regs.h
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
// RZ/G2L MIPI DSI Interface Registers Definitions
//
// Copyright (C) 2022 Renesas Electronics Corporation
//

// DPHY Registers
pub const DSIDPHYCTRL0: c_uint = 0x00;

pub const DSIDPHYTIM0: c_uint = 0x04;

pub const DSIDPHYTIM1: c_uint = 0x08;

pub const DSIDPHYTIM2: c_uint = 0x0c;

pub const DSIDPHYTIM3: c_uint = 0x10;

// RZ/V2H DPHY Registers
pub const PLLENR: c_uint = 0x000;

pub const PHYRSTR: c_uint = 0x004;

pub const PLLCLKSET0R: c_uint = 0x010;

pub const PLLCLKSET1R: c_uint = 0x014;

pub const PHYTCLKSETR: c_uint = 0x020;

pub const PHYTHSSETR: c_uint = 0x024;

pub const PHYTLPXSETR: c_uint = 0x028;

pub const PHYCR: c_uint = 0x030;

// --------------------------------------------------------
// Link Status Register
pub const LINKSR: c_uint = 0x10;

// Tx Set Register
pub const TXSETR: c_uint = 0x100;

// HS Clock Set Register
pub const HSCLKSETR: c_uint = 0x104;

// Reset Control Register
pub const RSTCR: c_uint = 0x110;

// Reset Status Register
pub const RSTSR: c_uint = 0x114;

// DSI Set Register
pub const DSISETR: c_uint = 0x120;

// Rx Result Save Slot 0 Register
pub const RXRSS0R: c_uint = 0x240;

// Clock Lane Stop Time Set Register
pub const CLSTPTSETR: c_uint = 0x314;

// LP Transition Time Set Register
pub const LPTRNSTSETR: c_uint = 0x318;

// Physical Lane Status Register
pub const PLSR: c_uint = 0x320;

// Video-Input Channel 1 Set 0 Register
pub const VICH1SET0R: c_uint = 0x400;

// Video-Input Channel 1 Set 1 Register
pub const VICH1SET1R: c_uint = 0x404;

// Video-Input Channel 1 Status Register
pub const VICH1SR: c_uint = 0x410;

// Video-Input Channel 1 Pixel Packet Set Register
pub const VICH1PPSETR: c_uint = 0x420;

// Video-Input Channel 1 Vertical Size Set Register
pub const VICH1VSSETR: c_uint = 0x428;

// Video-Input Channel 1 Vertical Porch Set Register
pub const VICH1VPSETR: c_uint = 0x42c;

// Video-Input Channel 1 Horizontal Size Set Register
pub const VICH1HSSETR: c_uint = 0x430;

// Video-Input Channel 1 Horizontal Porch Set Register
pub const VICH1HPSETR: c_uint = 0x434;

// Sequence Channel 0 Set 0 Register
pub const SQCH0SET0R: c_uint = 0x5c0;

// Sequence Channel 0 Status Register
pub const SQCH0SR: c_uint = 0x5d0;

// Sequence Channel 0 Status Clear Register
pub const SQCH0SCR: c_uint = 0x5d4;

// Sequence Channel 0 Descriptor 0-A Register
pub const SQCH0DSC0AR: c_uint = 0x780;

pub const SQCH0DSC0AR_BTA_NONE: c_int = 0;
pub const SQCH0DSC0AR_BTA_NON_READ: c_int = 1;
pub const SQCH0DSC0AR_BTA_READ: c_int = 2;
pub const SQCH0DSC0AR_BTA_ONLY: c_int = 3;
pub const SQCH0DSC0AR_SPD_HIGH: c_int = 0;

pub const SQCH0DSC0AR_FMT_SHORT: c_int = 0;

// Sequence Channel 0 Descriptor 0-B Register
pub const SQCH0DSC0BR: c_uint = 0x784;

// Sequence Channel 0 Descriptor 0-C Register
pub const SQCH0DSC0CR: c_uint = 0x788;

// Sequence Channel 0 Descriptor 0-D Register
pub const SQCH0DSC0DR: c_uint = 0x78c;
