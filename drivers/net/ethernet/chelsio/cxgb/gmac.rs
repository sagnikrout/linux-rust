//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/gmac.h
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
// File: gmac.h
// $Revision: 1.6 $
// $Date: 2005/06/21 18:29:47 $
// Description:
// Generic MAC functionality.
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
pub struct cmac_statistics {
// Transmit
    pub TxOctetsOK: u64,
    pub TxOctetsBad: u64,
    pub TxUnicastFramesOK: u64,
    pub TxMulticastFramesOK: u64,
    pub TxBroadcastFramesOK: u64,
    pub TxPauseFrames: u64,
    pub TxFramesWithDeferredXmissions: u64,
    pub TxLateCollisions: u64,
    pub TxTotalCollisions: u64,
    pub TxFramesAbortedDueToXSCollisions: u64,
    pub TxUnderrun: u64,
    pub TxLengthErrors: u64,
    pub TxInternalMACXmitError: u64,
    pub TxFramesWithExcessiveDeferral: u64,
    pub TxFCSErrors: u64,
    pub TxJumboFramesOK: u64,
    pub TxJumboOctetsOK: u64,
// Receive
    pub RxOctetsOK: u64,
    pub RxOctetsBad: u64,
    pub RxUnicastFramesOK: u64,
    pub RxMulticastFramesOK: u64,
    pub RxBroadcastFramesOK: u64,
    pub RxPauseFrames: u64,
    pub RxFCSErrors: u64,
    pub RxAlignErrors: u64,
    pub RxSymbolErrors: u64,
    pub RxDataErrors: u64,
    pub RxSequenceErrors: u64,
    pub RxRuntErrors: u64,
    pub RxJabberErrors: u64,
    pub RxInternalMACRcvError: u64,
    pub RxInRangeLengthErrors: u64,
    pub RxOutOfRangeLengthField: u64,
    pub RxFrameTooLongErrors: u64,
    pub RxJumboFramesOK: u64,
    pub RxJumboOctetsOK: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmac_ops {
    pub ): *mut *mut void (destroy)(struct cmac,
    pub ): *mut *mut int (reset)(struct cmac,
    pub ): *mut *mut int (interrupt_enable)(struct cmac,
    pub ): *mut *mut int (interrupt_disable)(struct cmac,
    pub ): *mut *mut int (interrupt_clear)(struct cmac,
    pub ): *mut *mut int (interrupt_handler)(struct cmac,
    pub int): *mut *mut *mut int (enable)(struct cmac ,,
    pub int): *mut *mut *mut int (disable)(struct cmac ,,
    pub ): *mut *mut int (loopback_enable)(struct cmac,
    pub ): *mut *mut int (loopback_disable)(struct cmac,
    pub mtu): *mut *mut *mut int (set_mtu)(struct cmac , int,
    pub rm): *mut *mut *mut int (set_rx_mode)(struct cmac , struct t1_rx_mode,
    pub fc): *mut *mut *mut int (set_speed_duplex_fc)(struct cmac , int speed, int duplex, int,
    pub fc): *mut c_int,
    pub int): *const *const *const *const cmac_statistics (statistics_update)(cmac ,,
    pub mac_addr[6]): *mut *mut *mut int (macaddress_get)(struct cmac , u8,
    pub mac_addr[6]): *const *const *const int (macaddress_set)(struct cmac , u8,
}

pub type cmac_instance = _cmac_instance;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmac {
    pub stats: cmac_statistics,
    pub adapter: *mut adapter_t,
    pub ops: *const cmac_ops,
    pub instance: *mut cmac_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmac {
    pub stats_update_period: c_uint,
    pub index): *mut *mut *mut *mut cmac (create)(adapter_t adapter, int,
    pub ): *mut *mut int (reset)(adapter_t,
}
