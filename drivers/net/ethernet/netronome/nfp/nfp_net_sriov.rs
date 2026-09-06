//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_net_sriov.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2019 Netronome Systems, Inc.
// SRIOV VF configuration.
// The configuration memory begins with a mailbox region for communication with
// the firmware followed by individual VF entries.
//
pub const NFP_NET_VF_CFG_SZ: c_int = 16;
pub const NFP_NET_VF_CFG_MB_SZ: c_int = 16;
// VF config mailbox
pub const NFP_NET_VF_CFG_MB: c_uint = 0x0;
pub const NFP_NET_VF_CFG_MB_CAP: c_uint = 0x0;

pub const NFP_NET_VF_CFG_MB_RET: c_uint = 0x2;
pub const NFP_NET_VF_CFG_MB_UPD: c_uint = 0x4;

pub const NFP_NET_VF_CFG_MB_VF_NUM: c_uint = 0x7;
// VF config entry
// MAC_LO is set that the MAC address can be read in a single 6 byte read
// by the NFP
//
pub const NFP_NET_VF_CFG_MAC: c_uint = 0x0;
pub const NFP_NET_VF_CFG_MAC_HI: c_uint = 0x0;
pub const NFP_NET_VF_CFG_MAC_LO: c_uint = 0x6;
pub const NFP_NET_VF_CFG_CTRL: c_uint = 0x4;
pub const NFP_NET_VF_CFG_CTRL_TRUST: c_uint = 0x8;
pub const NFP_NET_VF_CFG_CTRL_SPOOF: c_uint = 0x4;
pub const NFP_NET_VF_CFG_CTRL_LINK_STATE: c_uint = 0x3;
pub const NFP_NET_VF_CFG_LS_MODE_AUTO: c_int = 0;
pub const NFP_NET_VF_CFG_LS_MODE_ENABLE: c_int = 1;
pub const NFP_NET_VF_CFG_LS_MODE_DISABLE: c_int = 2;
pub const NFP_NET_VF_CFG_VLAN: c_uint = 0x8;
pub const NFP_NET_VF_CFG_VLAN_PROT: c_uint = 0xffff0000;
pub const NFP_NET_VF_CFG_VLAN_QOS: c_uint = 0xe000;
pub const NFP_NET_VF_CFG_VLAN_VID: c_uint = 0x0fff;
pub const NFP_NET_VF_CFG_RATE: c_uint = 0xc;
pub const NFP_NET_VF_CFG_MIN_RATE: c_uint = 0x0000ffff;
pub const NFP_NET_VF_CFG_MAX_RATE: c_uint = 0xffff0000;
pub const NFP_NET_VF_RATE_MAX: c_uint = 0xffff;
extern "C" {
    pub fn nfp_app_set_vf_mac(netdev: *mut net_device, vf: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn nfp_app_set_vf_spoofchk(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn nfp_app_set_vf_trust(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
