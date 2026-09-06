//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_vf.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_vf_tlv_msg_id {
    FM10K_VF_MSG_ID_TEST = 0,	/* msg ID reserved for testing */
    FM10K_VF_MSG_ID_MSIX,
    FM10K_VF_MSG_ID_MAC_VLAN,
    FM10K_VF_MSG_ID_LPORT_STATE,
    FM10K_VF_MSG_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_tlv_mac_vlan_attr_id {
    FM10K_MAC_VLAN_MSG_VLAN,
    FM10K_MAC_VLAN_MSG_SET,
    FM10K_MAC_VLAN_MSG_MAC,
    FM10K_MAC_VLAN_MSG_DEFAULT_MAC,
    FM10K_MAC_VLAN_MSG_MULTICAST,
    FM10K_MAC_VLAN_MSG_ID_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_tlv_lport_state_attr_id {
    FM10K_LPORT_STATE_MSG_DISABLE,
    FM10K_LPORT_STATE_MSG_XCAST_MODE,
    FM10K_LPORT_STATE_MSG_READY,
    FM10K_LPORT_STATE_MSG_MAX
}

extern "C" {
    pub fn fm10k_msg_mac_vlan_vf(: *mut fm10k_hw, : *mut u32, : *mut fm10k_mbx_info) -> i32;
}

