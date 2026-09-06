//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_txrx_lib.h
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
// Copyright (c) 2019, Intel Corporation.

//
// ice_test_staterr - tests bits in Rx descriptor status and error fields
// @status_err_n: Rx descriptor status_error0 or status_error1 bits
// @stat_err_bits: value to mask
//
// This function does some fast chicanery in order to return the
// value of the mask which is really only used for boolean tests.
// The status_error_len doesn't need to be shifted because it begins
// at offset zero.
//
// ice_is_non_eop - process handling of non-EOP buffers
// @rx_ring: Rx ring being processed
// @rx_desc: Rx descriptor for current buffer
//
// If the buffer is an EOP buffer, this function exits returning false,
// otherwise return true indicating that this is in fact a non-EOP buffer.
//
// if we are the last buffer then there is nothing else to do

//
// ice_build_tstamp_desc - build Tx time stamp descriptor
// @tx_desc: Tx LAN descriptor index
// @tstamp: time stamp
//
// Return: Tx time stamp descriptor
//
// ice_get_vlan_tci - get VLAN TCI from Rx flex descriptor
// @rx_desc: Rx 32b flex descriptor with RXDID=2
//
// The OS and current PF implementation only support stripping a single VLAN tag
// at a time, so there should only ever be 0 or 1 tags in the l2tag* fields. If
// one is found return the tag, else return 0 to mean no VLAN tag was found.
//
extern "C" {
    pub fn le16_to_cpu(_arg: rx_desc->wb.l2tag1) -> return;
}
extern "C" {
    pub fn le16_to_cpu(_arg: rx_desc->wb.l2tag2_2nd) -> return;
}
//
// ice_xdp_ring_update_tail - Updates the XDP Tx ring tail register
// @xdp_ring: XDP Tx ring
//
// This function updates the XDP Tx ring tail register.
//
// Force memory writes to complete before letting h/w
// know there are new descriptors to fetch.
//
// ice_set_rs_bit - set RS bit on last produced descriptor (one behind current NTU)
// @xdp_ring: XDP ring to produce the HW Tx descriptors on
//
// returns index of descriptor that had RS bit produced on
//
extern "C" {
    pub fn ice_finalize_xdp_rx(xdp_ring: *mut ice_tx_ring, xdp_res: c_uint, first_idx: u32);
}
extern "C" {
    pub fn ice_release_rx_desc(rx_ring: *mut ice_rx_ring, val: u16);
}
