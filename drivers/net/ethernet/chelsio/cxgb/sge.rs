//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/sge.h
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
// File: sge.h
// $Revision: 1.11 $
// $Date: 2005/06/21 22:10:55 $
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
pub struct sge_intr_counts {
    pub /: *mut *mut unsigned int rx_drops; / # of packets dropped due to no mem,
    pub /: *mut *mut unsigned int pure_rsps; / # of non-payload responses,
    pub /: *mut *mut unsigned int unhandled_irqs; / # of unhandled interrupts,
    pub /: *mut *mut unsigned int respQ_empty; / # times respQ empty,
    pub /: *mut *mut unsigned int respQ_overflow; / # respQ overflow (fatal),
    pub /: *mut *mut unsigned int freelistQ_empty; / # times freelist empty,
    pub /: *mut *mut unsigned int pkt_too_big; / packet too large (fatal),
    pub pkt_mismatch: c_uint,
    pub /: *mut *mut unsigned int cmdQ_full[3]; / not HW IRQ, host cmdQ[] full,
    pub /: *mut *mut unsigned int cmdQ_restarted[3];/ # of times cmdQ X was restarted,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_port_stats {
    pub /: *mut *mut u64 rx_cso_good; / # of successful RX csum offloads,
    pub /: *mut *mut u64 tx_cso; / # of TX checksum offloads,
    pub /: *mut *mut u64 tx_tso; / # of TSO requests,
    pub /: *mut *mut u64 vlan_xtract; / # of VLAN tag extractions,
    pub /: *mut *mut u64 vlan_insert; / # of VLAN tag insertions,
    pub /: *mut *mut u64 tx_need_hdrroom; / # of TX skbs in need of more header room,
}

extern "C" {
    pub fn t1_sge_configure(: *mut sge, : *mut sge_params) -> c_int;
}
extern "C" {
    pub fn t1_sge_set_coalesce_params(: *mut sge, : *mut sge_params) -> c_int;
}
extern "C" {
    pub fn t1_sge_destroy(: *mut sge);
}
extern "C" {
    pub fn t1_interrupt_thread(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn t1_interrupt(irq: c_int, cookie: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn t1_poll(: *mut napi_struct, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn t1_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn t1_vlan_mode(adapter: *mut adapter, features: netdev_features_t);
}
extern "C" {
    pub fn t1_sge_start(: *mut sge);
}
extern "C" {
    pub fn t1_sge_stop(: *mut sge);
}
extern "C" {
    pub fn t1_sge_intr_error_handler(sge: *mut sge) -> bool;
}
extern "C" {
    pub fn t1_sge_intr_enable(: *mut sge);
}
extern "C" {
    pub fn t1_sge_intr_disable(: *mut sge);
}
extern "C" {
    pub fn t1_sge_intr_clear(: *mut sge);
}
extern "C" {
    pub fn t1_sge_get_port_stats(sge: *const sge, port: c_int, : *mut sge_port_stats);
}
