//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/mv88e1xxx.h
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
// $Date: 2005/03/07 23:59:05 $ $RCSfile: mv88e1xxx.h,v $ $Revision: 1.13 $

// Gigabit MII registers

// 1000Base-T control register fields
pub const GBCR_ADV_1000HALF: c_uint = 0x100;
pub const GBCR_ADV_1000FULL: c_uint = 0x200;
pub const GBCR_PREFER_MASTER: c_uint = 0x400;
pub const GBCR_MANUAL_AS_MASTER: c_uint = 0x800;
pub const GBCR_MANUAL_CONFIG_ENABLE: c_uint = 0x1000;
// 1000Base-T status register fields
pub const GBSR_LP_1000HALF: c_uint = 0x400;
pub const GBSR_LP_1000FULL: c_uint = 0x800;
pub const GBSR_REMOTE_OK: c_uint = 0x1000;
pub const GBSR_LOCAL_OK: c_uint = 0x2000;
pub const GBSR_LOCAL_MASTER: c_uint = 0x4000;
pub const GBSR_MASTER_FAULT: c_uint = 0x8000;
// Marvell PHY interrupt status bits.
pub const MV88E1XXX_INTR_JABBER: c_uint = 0x0001;
pub const MV88E1XXX_INTR_POLARITY_CHNG: c_uint = 0x0002;
pub const MV88E1XXX_INTR_ENG_DETECT_CHNG: c_uint = 0x0010;
pub const MV88E1XXX_INTR_DOWNSHIFT: c_uint = 0x0020;
pub const MV88E1XXX_INTR_MDI_XOVER_CHNG: c_uint = 0x0040;
pub const MV88E1XXX_INTR_FIFO_OVER_UNDER: c_uint = 0x0080;
pub const MV88E1XXX_INTR_FALSE_CARRIER: c_uint = 0x0100;
pub const MV88E1XXX_INTR_SYMBOL_ERROR: c_uint = 0x0200;
pub const MV88E1XXX_INTR_LINK_CHNG: c_uint = 0x0400;
pub const MV88E1XXX_INTR_AUTONEG_DONE: c_uint = 0x0800;
pub const MV88E1XXX_INTR_PAGE_RECV: c_uint = 0x1000;
pub const MV88E1XXX_INTR_DUPLEX_CHNG: c_uint = 0x2000;
pub const MV88E1XXX_INTR_SPEED_CHNG: c_uint = 0x4000;
pub const MV88E1XXX_INTR_AUTONEG_ERR: c_uint = 0x8000;
// Marvell PHY specific registers.
pub const MV88E1XXX_SPECIFIC_CNTRL_REGISTER: c_int = 16;
pub const MV88E1XXX_SPECIFIC_STATUS_REGISTER: c_int = 17;
pub const MV88E1XXX_INTERRUPT_ENABLE_REGISTER: c_int = 18;
pub const MV88E1XXX_INTERRUPT_STATUS_REGISTER: c_int = 19;
pub const MV88E1XXX_EXT_PHY_SPECIFIC_CNTRL_REGISTER: c_int = 20;
pub const MV88E1XXX_RECV_ERR_CNTR_REGISTER: c_int = 21;
pub const MV88E1XXX_RES_REGISTER: c_int = 22;
pub const MV88E1XXX_GLOBAL_STATUS_REGISTER: c_int = 23;
pub const MV88E1XXX_LED_CONTROL_REGISTER: c_int = 24;
pub const MV88E1XXX_MANUAL_LED_OVERRIDE_REGISTER: c_int = 25;
pub const MV88E1XXX_EXT_PHY_SPECIFIC_CNTRL_2_REGISTER: c_int = 26;
pub const MV88E1XXX_EXT_PHY_SPECIFIC_STATUS_REGISTER: c_int = 27;
pub const MV88E1XXX_VIRTUAL_CABLE_TESTER_REGISTER: c_int = 28;
pub const MV88E1XXX_EXTENDED_ADDR_REGISTER: c_int = 29;
pub const MV88E1XXX_EXTENDED_REGISTER: c_int = 30;
// PHY specific control register fields
pub const S_PSCR_MDI_XOVER_MODE: c_int = 5;
pub const M_PSCR_MDI_XOVER_MODE: c_uint = 0x3;

// Extended PHY specific control register fields
pub const S_DOWNSHIFT_ENABLE: c_int = 8;

pub const S_DOWNSHIFT_CNT: c_int = 9;
pub const M_DOWNSHIFT_CNT: c_uint = 0x7;

// PHY specific status register fields
pub const S_PSSR_JABBER: c_int = 0;

pub const S_PSSR_POLARITY: c_int = 1;

pub const S_PSSR_RX_PAUSE: c_int = 2;

pub const S_PSSR_TX_PAUSE: c_int = 3;

pub const S_PSSR_ENERGY_DETECT: c_int = 4;

pub const S_PSSR_DOWNSHIFT_STATUS: c_int = 5;

pub const S_PSSR_MDI: c_int = 6;

pub const S_PSSR_CABLE_LEN: c_int = 7;
pub const M_PSSR_CABLE_LEN: c_uint = 0x7;

pub const S_PSSR_LINK: c_int = 10;

pub const S_PSSR_STATUS_RESOLVED: c_int = 11;

pub const S_PSSR_PAGE_RECEIVED: c_int = 12;

pub const S_PSSR_DUPLEX: c_int = 13;

pub const S_PSSR_SPEED: c_int = 14;
pub const M_PSSR_SPEED: c_uint = 0x3;

