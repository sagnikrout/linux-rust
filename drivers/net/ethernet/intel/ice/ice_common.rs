//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_common.h
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
// Copyright (c) 2018, Intel Corporation.

pub const ICE_SQ_SEND_DELAY_TIME_MS: c_int = 10;
pub const ICE_SQ_SEND_MAX_EXECUTE: c_int = 3;
pub const FEC_REG_SHIFT: c_int = 2;
pub const FEC_RECV_ID_SHIFT: c_int = 4;

pub const ICE_CGU_R9: c_uint = 0x24;

pub const ICE_CGU_R16: c_uint = 0x40;

pub const ICE_CGU_R19: c_uint = 0x4C;

pub const ICE_CGU_R22: c_uint = 0x58;

pub const ICE_CGU_R23: c_uint = 0x5C;
pub const ICE_CGU_R24: c_uint = 0x60;

pub const ICE_CGU_BW_TDC: c_uint = 0x31C;

pub const ICE_CGU_RO_LOCK: c_uint = 0x3F0;

pub const ICE_CGU_CNTR_BIST: c_uint = 0x344;

pub const ICE_CGU_RO_BWM_LF: c_uint = 0x370;

extern "C" {
    pub fn ice_init_hw(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_deinit_hw(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_check_reset(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_reset(hw: *mut ice_hw, req: ice_reset_req) -> c_int;
}
extern "C" {
    pub fn ice_create_all_ctrlq(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_init_all_ctrlq(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_shutdown_all_ctrlq(hw: *mut ice_hw, unloading: bool);
}
extern "C" {
    pub fn ice_destroy_all_ctrlq(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_update_link_info(pi: *mut ice_port_info) -> c_int;
}
extern "C" {
    pub fn ice_release_res(hw: *mut ice_hw, res: ice_aq_res_ids);
}
extern "C" {
    pub fn ice_is_sbq_supported(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_clear_pxe_mode(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_get_caps(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_set_safe_mode_caps(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_check_sq_alive(hw: *mut ice_hw, cq: *mut ice_ctl_q_info) -> bool;
}
extern "C" {
    pub fn ice_aq_q_shutdown(hw: *mut ice_hw, unloading: bool) -> c_int;
}
extern "C" {
    pub fn ice_fill_dflt_direct_cmd_desc(desc: *mut libie_aq_desc, opcode: u16);
}
extern "C" {
    pub fn ice_pack_txq_ctx(ctx: *const ice_tlan_ctx, buf: *mut ice_txq_ctx_buf_t);
}
extern "C" {
    pub fn ice_aq_get_fw_ver(hw: *mut ice_hw, cd: *mut ice_sq_cd) -> c_int;
}
extern "C" {
    pub fn ice_is_phy_rclk_in_netlist(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_is_clock_mux_in_netlist(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_is_cgu_in_netlist(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_is_gps_in_netlist(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_is_generic_mac(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_clear_pf_cfg(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_fw_supports_link_override(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_is_phy_caps_an_enabled(caps: *mut ice_aqc_get_phy_caps_data) -> bool;
}
extern "C" {
    pub fn ice_is_fw_health_report_supported(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_aq_set_health_status_cfg(hw: *mut ice_hw, event_source: u8) -> c_int;
}
extern "C" {
    pub fn ice_caps_to_fc_mode(caps: u8) -> ice_fc_mode;
}
extern "C" {
    pub fn ice_caps_to_fec_mode(caps: u8, fec_options: u8) -> ice_fec_mode;
}
extern "C" {
    pub fn ice_get_phy_lane_number(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_get_link_speed(index: u16) -> u32;
}
extern "C" {
    pub fn ice_replay_vsi(hw: *mut ice_hw, vsi_handle: u16) -> c_int;
}
extern "C" {
    pub fn ice_replay_post(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_sbq_rw_reg(hw: *mut ice_hw, in: *mut ice_sbq_msg_input, flag: u16) -> c_int;
}
extern "C" {
    pub fn ice_is_100m_speed_supported(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_get_link_speed_based_on_phy_type(phy_type_low: u64, phy_type_high: u64) -> u16;
}
extern "C" {
    pub fn ice_fw_supports_lldp_fltr_ctrl(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_lldp_fltr_add_remove(hw: *mut ice_hw, vsi: *mut ice_vsi, add: bool) -> c_int;
}
extern "C" {
    pub fn ice_lldp_execute_pending_mib(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_get_pca9575_handle(hw: *mut ice_hw, pca9575_handle: *mut u16) -> c_int;
}
extern "C" {
    pub fn ice_read_pca9575_reg(hw: *mut ice_hw, offset: u8, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_fw_supports_report_dflt_cfg(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_read_cgu_reg(hw: *mut ice_hw, addr: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ice_write_cgu_reg(hw: *mut ice_hw, addr: u32, val: u32) -> c_int;
}
