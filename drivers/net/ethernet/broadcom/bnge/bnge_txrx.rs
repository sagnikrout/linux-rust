//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_txrx.h
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
// Copyright (c) 2025 Broadcom

// For TX and RX ring doorbells with no ordering guarantee
pub const TX_OPAQUE_IDX_MASK: c_uint = 0x0000ffff;
pub const TX_OPAQUE_BDS_MASK: c_uint = 0x00ff0000;
pub const TX_OPAQUE_BDS_SHIFT: c_int = 16;
pub const TX_OPAQUE_RING_MASK: c_uint = 0xff000000;
pub const TX_OPAQUE_RING_SHIFT: c_int = 24;

pub const TX_MAX_BD_CNT: c_int = 32;

// Minimum TX BDs for a TX packet with MAX_SKB_FRAGS + 1.  We need one extra
// BD because the first TX BD is always a long BD.
//

extern "C" {
    pub fn bnge_msix(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn bnge_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn bnge_reuse_rx_data(rxr: *mut bnge_rx_ring_info, cons: u16, data: *mut c_void);
}
extern "C" {
    pub fn bnge_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
