//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_mipi_dsi_regs.h
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
// R-Car MIPI DSI Interface Registers Definitions
//
// Copyright (C) 2020 Renesas Electronics Corporation
//
pub const LINKSR: c_uint = 0x010;

pub const TXSETR: c_uint = 0x100;

//
// DSI Command Transfer Registers
//
pub const TXCMSETR: c_uint = 0x110;

pub const TXCMCR: c_uint = 0x120;

pub const TXCMSR: c_uint = 0x130;

pub const TXCMSCR: c_uint = 0x134;

pub const TXCMIER: c_uint = 0x138;

pub const TXCMADDRSET0R: c_uint = 0x140;
pub const TXCMPHDR: c_uint = 0x150;

pub const TXCMPPD0R: c_uint = 0x160;
pub const TXCMPPD1R: c_uint = 0x164;
pub const TXCMPPD2R: c_uint = 0x168;
pub const TXCMPPD3R: c_uint = 0x16c;
pub const RXSETR: c_uint = 0x200;

pub const RXPSETR: c_uint = 0x210;

pub const RXPSR: c_uint = 0x220;

pub const RXPSCR: c_uint = 0x224;

pub const RXPIER: c_uint = 0x228;

pub const RXPADDRSET0R: c_uint = 0x230;
pub const RXPSIZESETR: c_uint = 0x238;

pub const RXPHDR: c_uint = 0x240;

pub const RXPPD0R: c_uint = 0x250;
pub const RXPPD1R: c_uint = 0x254;
pub const RXPPD2R: c_uint = 0x258;
pub const RXPPD3R: c_uint = 0x25c;
pub const AKEPR: c_uint = 0x300;

pub const RXRESPTOSETR: c_uint = 0x400;
pub const TACR: c_uint = 0x500;
pub const TASR: c_uint = 0x510;
pub const TASCR: c_uint = 0x514;
pub const TAIER: c_uint = 0x518;
pub const TOSR: c_uint = 0x610;

pub const TOSCR: c_uint = 0x614;

//
// Video Mode Register
//
pub const TXVMSETR: c_uint = 0x180;

pub const TXVMCR: c_uint = 0x190;

pub const TXVMSR: c_uint = 0x1a0;

pub const TXVMSCR: c_uint = 0x1a4;

pub const TXVMPSPHSETR: c_uint = 0x1c0;

pub const TXVMVPRMSET0R: c_uint = 0x1d0;

pub const TXVMVPRMSET1R: c_uint = 0x1d4;

pub const TXVMVPRMSET2R: c_uint = 0x1d8;

pub const TXVMVPRMSET3R: c_uint = 0x1dc;

pub const TXVMVPRMSET4R: c_uint = 0x1e0;

//
// PHY-Protocol Interface (PPI) Registers
//
pub const PPISETR: c_uint = 0x700;

pub const PPICLCR: c_uint = 0x710;

pub const PPICLSR: c_uint = 0x720;

pub const PPICLSCR: c_uint = 0x724;

pub const PPIDL0SR: c_uint = 0x740;

pub const PPIDLSR: c_uint = 0x760;

//
// Clocks registers
//
pub const LPCLKSET: c_uint = 0x1000;

pub const CFGCLKSET: c_uint = 0x1004;

pub const DOTCLKDIV: c_uint = 0x1008;

pub const VCLKSET: c_uint = 0x100c;

pub const VCLKEN: c_uint = 0x1010;

pub const PHYSETUP: c_uint = 0x1014;

pub const CLOCKSET1: c_uint = 0x101c;

pub const CLOCKSET2: c_uint = 0x1020;

pub const CLOCKSET3: c_uint = 0x1024;

pub const PHTW: c_uint = 0x1034;

pub const PHTR: c_uint = 0x1038;

pub const PHTC: c_uint = 0x103c;

