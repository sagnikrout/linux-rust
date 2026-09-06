//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/cphy.h
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
// File: cphy.h
// $Revision: 1.7 $
// $Date: 2005/06/21 18:29:47 $
// Description:
// part of the Chelsio 10Gb Ethernet Driver.
//
// http://www.chelsio.com
//
// Copyright (c) 2003 - 2005 Chelsio Communications, Inc.
// All rights reserved.
//
// Maintainers: maintainers@chelsio.com
//
// Authors: Dimitrios Michailidis   <dm@chelsio.com>
// Tina Yang               <tainay@chelsio.com>
// Felix Marti             <felix@chelsio.com>
// Scott Bardone           <sbardone@chelsio.com>
// Kurt Ottaway            <kottaway@chelsio.com>
// Frank DiMambro          <frank@chelsio.com>
//
// History:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_ops {
    pub bi): *const *const *const void (init)(adapter_t adapter, struct board_info,
    pub reg_addr): u16,
    pub val): u16 reg_addr, u16,
    pub mode_support: unsigned,
}

// PHY interrupt types
// PHY operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cphy_ops {
    pub ): *mut *mut void (destroy)(struct cphy,
    pub wait): *mut *mut *mut int (reset)(struct cphy , int,
    pub ): *mut *mut int (interrupt_enable)(struct cphy,
    pub ): *mut *mut int (interrupt_disable)(struct cphy,
    pub ): *mut *mut int (interrupt_clear)(struct cphy,
    pub ): *mut *mut int (interrupt_handler)(struct cphy,
    pub ): *mut *mut int (autoneg_enable)(struct cphy,
    pub ): *mut *mut int (autoneg_disable)(struct cphy,
    pub ): *mut *mut int (autoneg_restart)(struct cphy,
    pub advertise_map): *mut *mut *mut int (advertise)(struct cphy phy, unsigned int,
    pub on): *mut *mut *mut int (set_loopback)(struct cphy , int,
    pub duplex): *mut *mut *mut int (set_speed_duplex)(struct cphy phy, int speed, int,
    pub fc): *mut *mut int duplex, int,
    pub mmds: u32,
}

// A PHY instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cphy {
    pub /: *mut *mut int state; / Link status state machine,
    pub /: *mut *mut *mut adapter_t adapter; / associated adapter,
    pub phy_update: delayed_work,
    pub bmsr: u16,
    pub count: c_int,
    pub act_count: c_int,
    pub act_on: c_int,
    pub elmer_gpo: u32,
    pub /: *const *const *const cphy_ops ops; / PHY operations,
    pub mdio: mdio_if_info,
    pub instance: *mut cphy_instance,
}

// Convenience MDIO read/write wrappers
// valp = (rc >= 0) ? rc : -1;
extern "C" {
    pub fn cphy_mdio_read(_arg: cphy, _arg: MDIO_DEVAD_NONE, _arg: reg, _arg: valp) -> return;
}
extern "C" {
    pub fn cphy_mdio_write(_arg: cphy, _arg: MDIO_DEVAD_NONE, _arg: reg, _arg: val) -> return;
}
// Convenience initializer
// Operations of the PHY-instance factory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gphy {
// Construct a PHY instance with the given PHY address
    pub mdio_ops): *const mdio_ops,
//
// Reset the PHY chip.  This resets the whole PHY chip, not individual
// ports.
//
    pub adapter): *mut *mut int (reset)(adapter_t,
}
