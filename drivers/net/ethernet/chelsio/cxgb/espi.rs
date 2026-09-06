//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/espi.h
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
// File: espi.h
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
pub struct espi_intr_counts {
    pub DIP4_err: c_uint,
    pub rx_drops: c_uint,
    pub tx_drops: c_uint,
    pub rx_ovflw: c_uint,
    pub parity_err: c_uint,
    pub DIP2_parity_err: c_uint,
}

extern "C" {
    pub fn t1_espi_destroy(espi: *mut peespi);
}
extern "C" {
    pub fn t1_espi_init(espi: *mut peespi, mac_type: c_int, nports: c_int) -> c_int;
}
extern "C" {
    pub fn t1_espi_intr_enable(: *mut peespi);
}
extern "C" {
    pub fn t1_espi_intr_clear(: *mut peespi);
}
extern "C" {
    pub fn t1_espi_intr_disable(: *mut peespi);
}
extern "C" {
    pub fn t1_espi_intr_handler(: *mut peespi) -> c_int;
}
extern "C" {
    pub fn t1_espi_get_mon(adapter: *mut adapter_t, addr: u32, wait: u8) -> u32;
}
extern "C" {
    pub fn t1_espi_get_mon_t204(: *mut adapter_t, : *mut u32, _arg: u8) -> c_int;
}
