//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbevf/vf.h
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
// Copyright(c) 1999 - 2024 Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mac_operations {
    pub ): *mut *mut s32 (init_hw)(struct ixgbe_hw,
    pub ): *mut *mut s32 (reset_hw)(struct ixgbe_hw,
    pub ): *mut *mut s32 (start_hw)(struct ixgbe_hw,
    pub ): *mut *mut s32 (clear_hw_cntrs)(struct ixgbe_hw,
    pub ): *mut *mut ixgbe_media_type (get_media_type)(struct ixgbe_hw,
    pub ): *mut *mut *mut s32 (get_mac_addr)(struct ixgbe_hw , u8,
    pub ): *mut *mut s32 (stop_adapter)(struct ixgbe_hw,
    pub ): *mut *mut s32 (get_bus_info)(struct ixgbe_hw,
    pub api): *mut *mut *mut s32 (negotiate_api_version)(struct ixgbe_hw hw, int,
    pub pf_features): *mut *mut *mut int (negotiate_features)(struct ixgbe_hw hw, u32,
// Link
    pub bool): *mut *mut *mut s32 (setup_link)(struct ixgbe_hw , ixgbe_link_speed, bool,,
    pub bool): *mut *mut *mut *mut *mut s32 (check_link)(struct ixgbe_hw , ixgbe_link_speed , bool ,,
    pub ): *mut bool,
// RAR, Multicast, VLAN
    pub u32): *mut *mut *mut *mut s32 (set_rar)(struct ixgbe_hw , u32, u8 ,,
    pub ): *mut *mut *mut s32 (set_uc_addr)(struct ixgbe_hw , u32, u8,
    pub ): *mut *mut s32 (init_rx_addrs)(struct ixgbe_hw,
    pub ): *mut *mut *mut s32 (update_mc_addr_list)(struct ixgbe_hw , struct net_device,
    pub int): *mut *mut *mut s32 (update_xcast_mode)(struct ixgbe_hw ,,
    pub link_state): *mut *mut *mut s32 (get_link_state)(struct ixgbe_hw hw, bool,
    pub ): *mut *mut s32 (enable_mc)(struct ixgbe_hw,
    pub ): *mut *mut s32 (disable_mc)(struct ixgbe_hw,
    pub ): *mut *mut s32 (clear_vfta)(struct ixgbe_hw,
    pub bool): *mut *mut *mut s32 (set_vfta)(struct ixgbe_hw , u32, u32,,
    pub u16): *mut *mut *mut s32 (set_rlpml)(struct ixgbe_hw ,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_mac_type {
    ixgbe_mac_unknown = 0,
    ixgbe_mac_82599_vf,
    ixgbe_mac_X540_vf,
    ixgbe_mac_X550_vf,
    ixgbe_mac_X550EM_x_vf,
    ixgbe_mac_x550em_a_vf,
    ixgbe_mac_e610,
    ixgbe_mac_e610_vf,
    ixgbe_num_macs
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mac_info {
    pub ops: ixgbe_mac_operations,
    pub addr: [u8; 6],
    pub perm_addr: [u8; 6],
    pub type: ixgbe_mac_type,
    pub mc_filter_type: i32,
    pub get_link_status: bool,
    pub max_tx_queues: u32,
    pub max_rx_queues: u32,
    pub max_msix_vectors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mbx_operations {
    pub hw): *mut *mut s32 (init_params)(struct ixgbe_hw,
    pub hw): *mut *mut void (release)(struct ixgbe_hw,
    pub u16): *mut *mut *mut *mut s32 (read)(struct ixgbe_hw , u32 ,,
    pub u16): *mut *mut *mut *mut s32 (write)(struct ixgbe_hw , u32 ,,
    pub ): *mut *mut s32 (check_for_msg)(struct ixgbe_hw,
    pub ): *mut *mut s32 (check_for_ack)(struct ixgbe_hw,
    pub ): *mut *mut s32 (check_for_rst)(struct ixgbe_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mbx_stats {
    pub msgs_tx: u32,
    pub msgs_rx: u32,
    pub acks: u32,
    pub reqs: u32,
    pub rsts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mbx_info {
    pub ops: ixgbe_mbx_operations,
    pub stats: ixgbe_mbx_stats,
    pub timeout: u32,
    pub udelay: u32,
    pub vf_mailbox: u32,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_hw {
    pub back: *mut c_void,
    pub hw_addr: *mut u8 __iomem,
    pub mac: ixgbe_mac_info,
    pub mbx: ixgbe_mbx_info,
    pub device_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub vendor_id: u16,
    pub revision_id: u8,
    pub adapter_stopped: bool,
    pub api_version: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_hw_stats {
    pub base_vfgprc: u64,
    pub base_vfgptc: u64,
    pub base_vfgorc: u64,
    pub base_vfgotc: u64,
    pub base_vfmprc: u64,
    pub last_vfgprc: u64,
    pub last_vfgptc: u64,
    pub last_vfgorc: u64,
    pub last_vfgotc: u64,
    pub last_vfmprc: u64,
    pub vfgprc: u64,
    pub vfgptc: u64,
    pub vfgorc: u64,
    pub vfgotc: u64,
    pub vfmprc: u64,
    pub saved_reset_vfgprc: u64,
    pub saved_reset_vfgptc: u64,
    pub saved_reset_vfgorc: u64,
    pub saved_reset_vfgotc: u64,
    pub saved_reset_vfmprc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbevf_info {
    pub mac: ixgbe_mac_type,
    pub mac_ops: *const ixgbe_mac_operations,
}

pub const IXGBE_FAILED_READ_REG: c_uint = 0xffffffffU;

extern "C" {
    pub fn ixgbevf_read_reg(hw: *mut ixgbe_hw, reg: u32) -> u32;
}

extern "C" {
    pub fn ixgbevf_read_reg(_arg: hw, 2): reg + (offset <<) -> return;
}

extern "C" {
    pub fn ixgbevf_get_reta_locked(hw: *mut ixgbe_hw, reta: *mut u32, num_rx_queues: c_int) -> c_int;
}
extern "C" {
    pub fn ixgbevf_get_rss_key_locked(hw: *mut ixgbe_hw, rss_key: *mut u8) -> c_int;
}
