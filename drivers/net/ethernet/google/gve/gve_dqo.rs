//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve_dqo.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2021 Google, Inc.
//

pub const GVE_ITR_INTERVAL_DQO_SHIFT: c_int = 5;

pub const GVE_TX_IRQ_RATELIMIT_US_DQO: c_int = 50;
pub const GVE_RX_IRQ_RATELIMIT_US_DQO: c_int = 20;

// Timeout in seconds to wait for a reinjection completion after receiving
// its corresponding miss completion.
//
pub const GVE_REINJECT_COMPL_TIMEOUT: c_int = 1;
// Timeout in seconds to deallocate the completion tag for a packet that was
// prematurely freed for not receiving a valid completion. This should be large
// enough to rule out the possibility of receiving the corresponding valid
// completion after this interval.
//
pub const GVE_DEALLOCATE_COMPL_TIMEOUT: c_int = 60;
extern "C" {
    pub fn gve_tx_dqo(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn gve_xdp_rx_timestamp(_ctx: *const xdp_md, timestamp: *mut u64) -> c_int;
}
extern "C" {
    pub fn gve_tx_poll_dqo(block: *mut gve_notify_block, do_clean: bool) -> bool;
}
extern "C" {
    pub fn gve_xdp_poll_dqo(block: *mut gve_notify_block) -> bool;
}
extern "C" {
    pub fn gve_xsk_tx_poll_dqo(block: *mut gve_notify_block, budget: c_int) -> bool;
}
extern "C" {
    pub fn gve_rx_poll_dqo(block: *mut gve_notify_block, budget: c_int) -> c_int;
}
extern "C" {
    pub fn gve_tx_start_ring_dqo(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_tx_stop_ring_dqo(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_rx_start_ring_dqo(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_rx_stop_ring_dqo(priv: *mut gve_priv, idx: c_int);
}
extern "C" {
    pub fn gve_rx_post_buffers_dqo(rx: *mut gve_rx_ring);
}
extern "C" {
    pub fn gve_rx_write_doorbell_dqo(priv: *const gve_priv, queue_idx: c_int);
}
extern "C" {
    pub fn gve_xdp_tx_flush_dqo(priv: *mut gve_priv, xdp_qid: u32);
}
// Builds register value to write to DQO IRQ doorbell to enable with specified
// ITR interval.
//
// Interval has 2us granularity.
// Sets interrupt throttling interval and enables interrupt
// by writing to IRQ doorbell.
//
extern "C" {
    pub fn gve_napi_poll_dqo(napi: *mut napi_struct, budget: c_int) -> c_int;
}
