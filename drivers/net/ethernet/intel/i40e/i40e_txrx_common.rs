//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_txrx_common.h
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
// Copyright(c) 2018 Intel Corporation.

// Macro flag: #define I40E_TXRX_COMMON_

extern "C" {
    pub fn i40e_xmit_xdp_tx_ring(xdp: *mut xdp_buff, xdp_ring: *mut i40e_ring) -> c_int;
}
extern "C" {
    pub fn i40e_xdp_ring_update_tail(xdp_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_finalize_xdp_rx(rx_ring: *mut i40e_ring, xdp_res: c_uint);
}
extern "C" {
    pub fn i40e_release_rx_desc(rx_ring: *mut i40e_ring, val: u32);
}
pub const I40E_XDP_PASS: c_int = 0;

//
// build_ctob - Builds the Tx descriptor (cmd, offset and type) qword
//
// i40e_update_tx_stats - Update the egress statistics for the Tx ring
// @tx_ring: Tx ring to update
// @total_packets: total packets sent
// @total_bytes: total bytes sent
//
pub const WB_STRIDE: c_int = 4;
//
// i40e_arm_wb - (Possibly) arms Tx write-back
// @tx_ring: Tx ring to update
// @vsi: the VSI
// @budget: the NAPI budget left
//
// check to see if there are < 4 descriptors
// waiting to be written back, then kick the hardware to force
// them to be written back in case we stay in NAPI.
// In this mode on X722 we do not enable Interrupt.
//
// i40e_rx_is_programming_status - check for programming status descriptor
// @qword1: qword1 representing status_error_len in CPU ordering
//
// The value of in the descriptor length field indicate if this
// is a programming status descriptor for flow director or FCoE
// by the value of I40E_RX_PROG_STATUS_DESC_LENGTH, otherwise
// it is a packet descriptor.
//
// The Rx filter programming status and SPH bit occupy the same
// spot in the descriptor. Since we don't support packet split we
// can just reuse the bit as an indication that this is a
// programming status descriptor.
//
extern "C" {
    pub fn i40e_xsk_clean_rx_ring(rx_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_xsk_clean_tx_ring(tx_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_xsk_any_rx_ring_enabled(vsi: *mut i40e_vsi) -> bool;
}
