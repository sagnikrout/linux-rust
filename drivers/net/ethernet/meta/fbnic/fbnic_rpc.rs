//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_rpc.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

// The TCAM state definitions follow an expected ordering.
// They start out disabled, then move through the following states:
// Disabled  0	-> Add	      2
// Add	      2	-> Valid      1
//
// Valid     1	-> Add/Update 2
// Add	      2	-> Valid      1
//
// Valid     1	-> Delete     3
// Delete    3	-> Disabled   0
//
// 32 MAC Destination Address TCAM Entries
// 4 registers DA[1:0], DA[3:2], DA[5:4], Validate
//
pub const FBNIC_RPC_TCAM_MACDA_WORD_LEN: c_int = 3;
pub const FBNIC_RPC_TCAM_MACDA_NUM_ENTRIES: c_int = 32;
// 8 IPSRC and IPDST TCAM Entries each
// 8 registers, Validate each
//
pub const FBNIC_RPC_TCAM_IP_ADDR_WORD_LEN: c_int = 8;
pub const FBNIC_RPC_TCAM_IP_ADDR_NUM_ENTRIES: c_int = 8;
pub const FBNIC_RPC_TCAM_ACT_WORD_LEN: c_int = 11;
pub const FBNIC_RPC_TCAM_ACT_NUM_ENTRIES: c_int = 64;
pub const FBNIC_TCE_TCAM_WORD_LEN: c_int = 3;
pub const FBNIC_TCE_TCAM_NUM_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_mac_addr {
    pub addr8: [c_uchar; ETH_ALEN],
    pub addr16: [__be16; FBNIC_RPC_TCAM_MACDA_WORD_LEN],
    pub value: } mask,,
    pub state: c_uchar,
    pub FBNIC_RPC_TCAM_ACT_NUM_ENTRIES): DECLARE_BITMAP(act_tcam,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_ip_addr {
    pub value: in6_addr mask,,
    pub version: c_uchar,
    pub state: c_uchar,
    pub FBNIC_RPC_TCAM_ACT_NUM_ENTRIES): DECLARE_BITMAP(act_tcam,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_act_tcam {
    pub tcam: [u16; FBNIC_RPC_TCAM_ACT_WORD_LEN],
    pub value: } mask,,
    pub state: c_uchar,
    pub rss_en_mask: u16,
    pub dest: u32,
}

// Reserve the first 2 entries for the use by the BMC so that we can
// avoid allowing rules to get in the way of BMC unicast traffic.
//
pub const FBNIC_RPC_ACT_TBL_BMC_OFFSET: c_int = 0;
pub const FBNIC_RPC_ACT_TBL_BMC_ALL_MULTI_OFFSET: c_int = 1;
// This should leave us with 48 total entries in the TCAM that can be used
// for NFC after also deducting the 14 needed for RSS table programming.
//
pub const FBNIC_RPC_ACT_TBL_NFC_OFFSET: c_int = 2;
// We reserve the last 14 entries for RSS rules on the host. The BMC
// unicast rule will need to be populated above these and is expected to
// use MACDA TCAM entry 23 to store the BMC MAC address.
//

// Flags used to identify the owner for this MAC filter. Note that any
// flags set for Broadcast thru Promisc indicate that the rule belongs
// to the RSS filters for the host.
//

// TCAM 0 - 3 reserved for BMC MAC addresses
pub const FBNIC_RPC_TCAM_MACDA_BMC_ADDR_IDX: c_int = 0;
// TCAM 4 reserved for broadcast MAC address
pub const FBNIC_RPC_TCAM_MACDA_BROADCAST_IDX: c_int = 4;
// TCAMs 5 - 30 will be used for multicast and unicast addresses. The
// boundary between the two can be variable it is currently set to 24
// on which the unicast addresses start. The general idea is that we will
// always go top-down with unicast, and bottom-up with multicast so that
// there should be free-space in the middle between the two.
//
// The entry at MADCA_DEFAULT_BOUNDARY is a special case as it can be used
// for the ALL MULTI address if the list is full, or the BMC has requested
// it.
//
pub const FBNIC_RPC_TCAM_MACDA_MULTICAST_IDX: c_int = 5;
pub const FBNIC_RPC_TCAM_MACDA_DEFAULT_BOUNDARY: c_int = 24;
pub const FBNIC_RPC_TCAM_MACDA_HOST_ADDR_IDX: c_int = 30;
// Reserved for use to record Multicast promisc, or Promiscuous
pub const FBNIC_RPC_TCAM_MACDA_PROMISC_IDX: c_int = 31;

extern "C" {
    pub fn fbnic_bmc_rpc_init(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_bmc_rpc_all_multi_config(fbd: *mut fbnic_dev, enable_host: bool);
}
extern "C" {
    pub fn fbnic_bmc_rpc_check(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_reset_indir_tbl(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_rss_key_fill(buffer: *mut u32);
}
extern "C" {
    pub fn fbnic_rss_init_en_mask(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_rss_disable_hw(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_rss_reinit_hw(fbd: *mut fbnic_dev, fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_rss_reinit(fbd: *mut fbnic_dev, fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_flow_hash_2_rss_en_mask(fbn: *mut fbnic_net, flow_type: c_int) -> u16;
}
extern "C" {
    pub fn __fbnic_xc_unsync(mac_addr: *mut fbnic_mac_addr, tcam_idx: c_uint) -> c_int;
}
extern "C" {
    pub fn fbnic_sift_macda(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_write_macda(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn __fbnic_ip_unsync(ip_addr: *mut fbnic_ip_addr, tcam_idx: c_uint) -> c_int;
}
extern "C" {
    pub fn fbnic_write_ip_addr(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn __fbnic_xc_unsync(_arg: mac_addr, _arg: FBNIC_MAC_ADDR_T_UNICAST) -> return;
}
extern "C" {
    pub fn __fbnic_xc_unsync(_arg: mac_addr, _arg: FBNIC_MAC_ADDR_T_MULTICAST) -> return;
}
extern "C" {
    pub fn fbnic_clear_rules(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_write_rules(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_write_tce_tcam(fbd: *mut fbnic_dev);
}
