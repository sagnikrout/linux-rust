//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_pf.h
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

extern "C" {
    pub fn fm10k_glort_valid_pf(hw: *mut fm10k_hw, glort: u16) -> bool;
}
extern "C" {
    pub fn fm10k_queues_per_pool(hw: *mut fm10k_hw) -> u16;
}
extern "C" {
    pub fn fm10k_vf_queue_index(hw: *mut fm10k_hw, vf_idx: u16) -> u16;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_pf_tlv_msg_id_v1 {
    FM10K_PF_MSG_ID_TEST			= 0x000, /* msg ID reserved */
    FM10K_PF_MSG_ID_XCAST_MODES		= 0x001,
    FM10K_PF_MSG_ID_UPDATE_MAC_FWD_RULE	= 0x002,
    FM10K_PF_MSG_ID_LPORT_MAP		= 0x100,
    FM10K_PF_MSG_ID_LPORT_CREATE		= 0x200,
    FM10K_PF_MSG_ID_LPORT_DELETE		= 0x201,
    FM10K_PF_MSG_ID_CONFIG			= 0x300,
    FM10K_PF_MSG_ID_UPDATE_PVID		= 0x400,
    FM10K_PF_MSG_ID_CREATE_FLOW_TABLE	= 0x501,
    FM10K_PF_MSG_ID_DELETE_FLOW_TABLE	= 0x502,
    FM10K_PF_MSG_ID_UPDATE_FLOW		= 0x503,
    FM10K_PF_MSG_ID_DELETE_FLOW		= 0x504,
    FM10K_PF_MSG_ID_SET_FLOW_STATE		= 0x505,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_pf_tlv_attr_id_v1 {
    FM10K_PF_ATTR_ID_ERR			= 0x00,
    FM10K_PF_ATTR_ID_LPORT_MAP		= 0x01,
    FM10K_PF_ATTR_ID_XCAST_MODE		= 0x02,
    FM10K_PF_ATTR_ID_MAC_UPDATE		= 0x03,
    FM10K_PF_ATTR_ID_VLAN_UPDATE		= 0x04,
    FM10K_PF_ATTR_ID_CONFIG			= 0x05,
    FM10K_PF_ATTR_ID_CREATE_FLOW_TABLE	= 0x06,
    FM10K_PF_ATTR_ID_DELETE_FLOW_TABLE	= 0x07,
    FM10K_PF_ATTR_ID_UPDATE_FLOW		= 0x08,
    FM10K_PF_ATTR_ID_FLOW_STATE		= 0x09,
    FM10K_PF_ATTR_ID_FLOW_HANDLE		= 0x0A,
    FM10K_PF_ATTR_ID_DELETE_FLOW		= 0x0B,
    FM10K_PF_ATTR_ID_PORT			= 0x0C,
    FM10K_PF_ATTR_ID_UPDATE_PVID		= 0x0D,
}

pub const FM10K_MSG_LPORT_MAP_GLORT_SHIFT: c_int = 0;
pub const FM10K_MSG_LPORT_MAP_GLORT_SIZE: c_int = 16;
pub const FM10K_MSG_LPORT_MAP_MASK_SHIFT: c_int = 16;
pub const FM10K_MSG_LPORT_MAP_MASK_SIZE: c_int = 16;
pub const FM10K_MSG_UPDATE_PVID_GLORT_SHIFT: c_int = 0;
pub const FM10K_MSG_UPDATE_PVID_GLORT_SIZE: c_int = 16;
pub const FM10K_MSG_UPDATE_PVID_PVID_SHIFT: c_int = 16;
pub const FM10K_MSG_UPDATE_PVID_PVID_SIZE: c_int = 16;
pub const FM10K_MSG_ERR_PEP_NOT_SCHEDULED: c_int = 280;
// The following data structures are overlayed directly onto TLV mailbox
// messages, and must not break 4 byte alignment. Ensure the structures line
// up correctly as per their TLV definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mac_update {
    pub mac_lower: __le32,
    pub mac_upper: __le16,
    pub vlan: __le16,
    pub glort: __le16,
    pub flags: u8,
    pub action: u8,
    pub __packed: } __aligned(4),
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_global_table_data {
    pub used: __le32,
    pub avail: __le32,
    pub __packed: } __aligned(4),
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_swapi_error {
    pub status: __le32,
    pub mac: fm10k_global_table_data,
    pub nexthop: fm10k_global_table_data,
    pub ffu: fm10k_global_table_data,
    pub __packed: } __aligned(4),
    pub ): *mut *mut *mut *mut s32 fm10k_msg_lport_map_pf(struct fm10k_hw , u32 , struct fm10k_mbx_info,
    pub fm10k_lport_map_msg_attr: [extern struct fm10k_tlv_attr; ],    pub fm10k_update_pvid_msg_attr: [extern struct fm10k_tlv_attr; ],
    pub ): *mut *mut *mut *mut s32 fm10k_msg_err_pf(struct fm10k_hw , u32 , struct fm10k_mbx_info,
    pub fm10k_err_msg_attr: [extern struct fm10k_tlv_attr; ],
    pub vid): *mut *mut s32 fm10k_iov_select_vid(struct fm10k_vf_info vf_info, u16,
    pub ): *mut *mut *mut *mut s32 fm10k_iov_msg_msix_pf(struct fm10k_hw , u32 , struct fm10k_mbx_info,
    pub ): *mut fm10k_mbx_info,
    pub fm10k_pf_info: extern struct fm10k_info,
