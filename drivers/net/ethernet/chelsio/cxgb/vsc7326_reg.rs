//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/vsc7326_reg.h
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
// $Date: 2006/04/28 19:20:17 $ $RCSfile: vsc7326_reg.h,v $ $Revision: 1.5 $
// Register definitions for Vitesse VSC7321 (Meigs II) MAC
//
// Straight off the data sheet, VMDS-10038 Rev 2.0 and
// PD0011-01-14-Meigs-II 2002-12-12
//
// Just 'cause it's in here doesn't mean it's used.

// System and CPU comm's registers

// Aggregator registers

// BIST registers
// #define REG_RAM_BIST_CMD	CRA(0x7,0x2,0x00)*/	/* RAM BIST Command Register
// #define REG_RAM_BIST_RESULT	CRA(0x7,0x2,0x01)*/	/* RAM BIST Read Status/Result

pub const BIST_PORT_SELECT: c_uint = 0x00			/* BIST port select */;
pub const BIST_COMMAND: c_uint = 0x01			/* BIST enable/disable */;
pub const BIST_STATUS: c_uint = 0x02			/* BIST operation status */;
pub const BIST_ERR_CNT_LSB: c_uint = 0x03			/* BIST error count lo 8b */;
pub const BIST_ERR_CNT_MSB: c_uint = 0x04			/* BIST error count hi 8b */;
pub const BIST_ERR_SEL_LSB: c_uint = 0x05			/* BIST error select lo 8b */;
pub const BIST_ERR_SEL_MSB: c_uint = 0x06			/* BIST error select hi 8b */;
pub const BIST_ERROR_STATE: c_uint = 0x07			/* BIST engine internal state */;
pub const BIST_ERR_ADR0: c_uint = 0x08			/* BIST error address lo 8b */;
pub const BIST_ERR_ADR1: c_uint = 0x09			/* BIST error address lomid 8b */;
pub const BIST_ERR_ADR2: c_uint = 0x0a			/* BIST error address himid 8b */;
pub const BIST_ERR_ADR3: c_uint = 0x0b			/* BIST error address hi 8b */;
// FIFO registers
// ie = 0 for ingress, 1 for egress
// fn = FIFO number, 0-9
//

// Traffic shaper buckets
// ie = 0 for ingress, 1 for egress
// bn = bucket number 0-10 (yes, 11 buckets)
//
// OK, this one's kinda ugly.  Some hardware designers are perverse.

// REG_ING_CONTROL equals REG_CONTROL with ie = 0, likewise REG_EGR_CONTROL is ie = 1

// SPI4 interface

// 10GbE MAC Block Registers
// Note that those registers that are exactly the same for 10GbE as for
// tri-speed are only defined with the version that needs a port number.
// Pass 0xa in those cases.
//
// Also note that despite the presence of a MAC address register, this part
// does no ingress MAC address filtering.  That register is used only for
// pause frame detection and generation.
//
// 10GbE specific, and different from tri-speed

// pn = port number 0-9 for tri-speed, 10 for 10GbE
// Both tri-speed and 10GbE

// tri-speed only
// pn = port number, 0-9
//

// Statistics
// CRA(0x4,pn,reg)
// reg below
// pn = port number, 0-a, a = 10GbE
// Hole. See REG_RX_XGMII_PROT_ERR below.
// Duplicate. See REG_STAT_STICKY10G below.

// MII-Management Block registers
// These are for MII-M interface 0, which is the bidirectional LVTTL one.  If
// we hooked up to the one with separate directions, the middle 0x0 needs to
// change to 0x1.  And the current errata states that MII-M 1 doesn't work.
//

// Whew.
