//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_link.h
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
// Copyright (c) 2026 Broadcom

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_link_state {
    BNGE_LINK_STATE_UNKNOWN,
    BNGE_LINK_STATE_DOWN,
    BNGE_LINK_STATE_UP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_link_info {
    pub phy_type: u8,
    pub media_type: u8,
    pub phy_addr: u8,
    pub phy_link_status: u8,
    pub phy_enabled: bool,
    pub link_state: u8,
    pub active_lanes: u8,
    pub duplex: u8,
    pub pause: u8,
    pub lp_pause: u8,
    pub auto_pause_setting: u8,
    pub force_pause_setting: u8,
    pub duplex_setting: u8,
    pub auto_mode: u8,
    pub link_speed: u16,
    pub support_speeds2: u16,
    pub auto_link_speeds2: u16,
    pub support_auto_speeds2: u16,
    pub lp_auto_link_speeds: u16,
    pub force_link_speed2: u16,
    pub module_status: u8,
    pub active_fec_sig_mode: u8,
    pub fec_cfg: u16,
// A copy of phy_qcfg output used to report link
// info to VF
//
    pub phy_qcfg_resp: hwrm_port_phy_qcfg_output,
    pub phy_retry: bool,
    pub phy_retry_expires: c_ulong,
}

pub const BNGE_AUTONEG_SPEED: c_int = 1;
pub const BNGE_AUTONEG_FLOW_CTRL: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ethtool_link_info {
// copy of requested setting from ethtool cmd
    pub autoneg: u8,
    pub req_signal_mode: u8,
    pub req_duplex: u8,
    pub req_flow_ctrl: u8,
    pub req_link_speed: u16,
    pub /: *mut *mut u16 advertising; / user adv setting,
    pub force_link_chng: bool,
}

extern "C" {
    pub fn bnge_update_phy_setting(bn: *mut bnge_net) -> c_int;
}
extern "C" {
    pub fn bnge_get_port_module_status(bn: *mut bnge_net);
}
extern "C" {
    pub fn bnge_report_link(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_support_speed_dropped(bn: *mut bnge_net) -> bool;
}
extern "C" {
    pub fn bnge_init_ethtool_link_settings(bn: *mut bnge_net);
}
extern "C" {
    pub fn bnge_probe_phy(bn: *mut bnge_net, fw_dflt: bool) -> c_int;
}
extern "C" {
    pub fn bnge_get_link(dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn bnge_link_async_event_process(bn: *mut bnge_net, event_id: u16);
}
