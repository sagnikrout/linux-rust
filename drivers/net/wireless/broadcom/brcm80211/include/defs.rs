//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/defs.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

pub const SI_BUS: c_int = 0;
pub const PCI_BUS: c_int = 1;
pub const PCMCIA_BUS: c_int = 2;
pub const SDIO_BUS: c_int = 3;
pub const JTAG_BUS: c_int = 4;
pub const USB_BUS: c_int = 5;
pub const SPI_BUS: c_int = 6;
pub const OFF: c_int = 0;

//
// Priority definitions according 802.1D
//
pub const PRIO_8021D_NONE: c_int = 2;
pub const PRIO_8021D_BK: c_int = 1;
pub const PRIO_8021D_BE: c_int = 0;
pub const PRIO_8021D_EE: c_int = 3;
pub const PRIO_8021D_CL: c_int = 4;
pub const PRIO_8021D_VI: c_int = 5;
pub const PRIO_8021D_VO: c_int = 6;
pub const PRIO_8021D_NC: c_int = 7;
pub const MAXPRIO: c_int = 7;

pub const BRCM_SET_CHANNEL: c_int = 30;
pub const BRCM_SET_SRL: c_int = 32;
pub const BRCM_SET_LRL: c_int = 34;
pub const BRCM_SET_BCNPRD: c_int = 76;

pub const BRCM_GET_PHYLIST: c_int = 180;
// Bit masks for radio disabled status - returned by WL_GET_RADIO

// some countries don't support any channel

// Override bit for SET_TXPWR.  if set, ignore other level limits

// band types

// Debug levels
pub const BRCM_DL_INFO: c_uint = 0x00000001;
pub const BRCM_DL_MAC80211: c_uint = 0x00000002;
pub const BRCM_DL_RX: c_uint = 0x00000004;
pub const BRCM_DL_TX: c_uint = 0x00000008;
pub const BRCM_DL_INT: c_uint = 0x00000010;
pub const BRCM_DL_DMA: c_uint = 0x00000020;
pub const BRCM_DL_HT: c_uint = 0x00000040;
// Values for PM
pub const PM_OFF: c_int = 0;
pub const PM_MAX: c_int = 1;
pub const PM_FAST: c_int = 2;
//
// Sonics Configuration Space Registers.
//
// core sbconfig regs are top 256bytes of regs
pub const SBCONFIGOFF: c_uint = 0xf00;
// cpp contortions to concatenate w/arg prescan

