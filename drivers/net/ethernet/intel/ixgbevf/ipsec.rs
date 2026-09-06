//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbevf/ipsec.h
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
// Copyright(c) 2018 Oracle and/or its affiliates. All rights reserved.
pub const IXGBE_IPSEC_MAX_SA_COUNT: c_int = 1024;
pub const IXGBE_IPSEC_BASE_RX_INDEX: c_int = 0;

pub const IXGBE_IPSEC_AUTH_BITS: c_int = 128;
pub const IXGBE_RXMOD_VALID: c_uint = 0x00000001;
pub const IXGBE_RXMOD_PROTO_ESP: c_uint = 0x00000004;
pub const IXGBE_RXMOD_DECRYPT: c_uint = 0x00000008;
pub const IXGBE_RXMOD_IPV6: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_sa {
    pub hlist: hlist_node,
    pub xs: *mut xfrm_state,
    pub ipaddr: [__be32; 4],
    pub key: [u32; 4],
    pub salt: u32,
    pub mode: u32,
    pub pfsa: u32,
    pub used: bool,
    pub decrypt: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_ip_sa {
    pub ipaddr: [__be32; 4],
    pub ref_cnt: u32,
    pub used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_sa {
    pub xs: *mut xfrm_state,
    pub key: [u32; 4],
    pub salt: u32,
    pub pfsa: u32,
    pub encrypt: bool,
    pub used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_ipsec_tx_data {
    pub flags: u32,
    pub trailer_len: u16,
    pub pfsa: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_ipsec {
    pub num_rx_sa: u16,
    pub num_tx_sa: u16,
    pub rx_tbl: *mut rx_sa,
    pub tx_tbl: *mut tx_sa,
    pub 10): DECLARE_HASHTABLE(rx_sa_list,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_mbx_msg {
    pub spi: __be32,
    pub dir: u8,
    pub proto: u8,
    pub family: u16,
    pub addr: [__be32; 4],
    pub key: [u32; 5],
}
