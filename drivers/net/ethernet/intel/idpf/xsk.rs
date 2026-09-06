//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/xsk.h
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
// Copyright (C) 2025 Intel Corporation

extern "C" {
    pub fn idpf_xsk_clear_queue(q: *mut c_void, type: virtchnl2_queue_type);
}
extern "C" {
    pub fn idpf_xsk_init_wakeup(qv: *mut idpf_q_vector);
}
extern "C" {
    pub fn idpf_xskfq_init(bufq: *mut idpf_buf_queue) -> c_int;
}
extern "C" {
    pub fn idpf_xskfq_rel(bufq: *mut idpf_buf_queue);
}
extern "C" {
    pub fn idpf_xsksq_clean(xdpq: *mut idpf_tx_queue);
}
extern "C" {
    pub fn idpf_xskrq_poll(rxq: *mut idpf_rx_queue, budget: u32) -> c_int;
}
extern "C" {
    pub fn idpf_xsk_xmit(xsksq: *mut idpf_tx_queue) -> bool;
}
extern "C" {
    pub fn idpf_xsk_pool_setup(vport: *mut idpf_vport, xdp: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn idpf_xsk_wakeup(dev: *mut net_device, qid: u32, flags: u32) -> c_int;
}
