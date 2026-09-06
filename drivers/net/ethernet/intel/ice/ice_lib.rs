//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_lib.h
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

// Flags used for VSI configuration and rebuild

pub const ICE_VSI_FLAG_NO_INIT: c_int = 0;
pub const ICE_L2TSEL_QRX_CONTEXT_REG_IDX: c_int = 3;
pub const ICE_L2TSEL_BIT_OFFSET: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_l2tsel {
    ICE_L2TSEL_EXTRACT_FIRST_TAG_L2TAG2_2ND,
    ICE_L2TSEL_EXTRACT_FIRST_TAG_L2TAG1,
}

extern "C" {
    pub fn ice_pf_state_is_nominal(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_update_eth_stats(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_cfg_msix(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_start_all_rx_rings(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_stop_all_rx_rings(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_stop_xdp_tx_rings(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg_sw_lldp(vsi: *mut ice_vsi, tx: bool, create: bool);
}
extern "C" {
    pub fn ice_cfg_sw_rx_lldp(pf: *mut ice_pf, enable: bool);
}
extern "C" {
    pub fn ice_set_link(vsi: *mut ice_vsi, ena: bool) -> c_int;
}
extern "C" {
    pub fn ice_vsi_delete(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_cfg_tc(vsi: *mut ice_vsi, ena_tc: u8) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg_rss_lut_key(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg_netdev_tc(vsi: *mut ice_vsi, ena_tc: u8);
}
extern "C" {
    pub fn ice_vsi_set_napi_queues(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_napi_add(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_clear_napi_queues(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_release(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_close(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_ena_vsi(vsi: *mut ice_vsi, locked: bool) -> c_int;
}
extern "C" {
    pub fn ice_vsi_decfg(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_dis_vsi(vsi: *mut ice_vsi, locked: bool);
}
extern "C" {
    pub fn ice_vsi_rebuild(vsi: *mut ice_vsi, vsi_flags: u32) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_free(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_is_reset_in_progress(state: *mut c_ulong) -> bool;
}
extern "C" {
    pub fn ice_wait_for_reset(pf: *mut ice_pf, timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn ice_vsi_free_irq(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_free_rx_rings(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_free_tx_rings(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_manage_rss_lut(vsi: *mut ice_vsi, ena: bool);
}
extern "C" {
    pub fn ice_vsi_cfg_crc_strip(vsi: *mut ice_vsi, disable: bool);
}
extern "C" {
    pub fn ice_update_tx_ring_stats(ring: *mut ice_tx_ring, pkts: u64, bytes: u64);
}
extern "C" {
    pub fn ice_update_rx_ring_stats(ring: *mut ice_rx_ring, pkts: u64, bytes: u64);
}
extern "C" {
    pub fn ice_write_intrl(q_vector: *mut ice_q_vector, intrl: u8);
}
extern "C" {
    pub fn ice_write_itr(rc: *mut ice_ring_container, itr: u16);
}
extern "C" {
    pub fn ice_set_q_vector_intrl(q_vector: *mut ice_q_vector);
}
extern "C" {
    pub fn ice_is_safe_mode(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_is_rdma_ena(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_is_recovery_mode(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_is_dflt_vsi_in_use(pi: *mut ice_port_info) -> bool;
}
extern "C" {
    pub fn ice_is_vsi_dflt_vsi(vsi: *mut ice_vsi) -> bool;
}
extern "C" {
    pub fn ice_set_dflt_vsi(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_clear_dflt_vsi(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_set_min_bw_limit(vsi: *mut ice_vsi, min_tx_rate: u64) -> c_int;
}
extern "C" {
    pub fn ice_set_max_bw_limit(vsi: *mut ice_vsi, max_tx_rate: u64) -> c_int;
}
extern "C" {
    pub fn ice_get_link_speed_kbps(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_get_link_speed_mbps(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_ctx_set_antispoof(ctx: *mut ice_vsi_ctx);
}
extern "C" {
    pub fn ice_vsi_ctx_clear_antispoof(ctx: *mut ice_vsi_ctx);
}
extern "C" {
    pub fn ice_vsi_update_local_lb(vsi: *mut ice_vsi, set: bool) -> c_int;
}
extern "C" {
    pub fn ice_vsi_add_vlan_zero(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_del_vlan_zero(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_has_non_zero_vlans(vsi: *mut ice_vsi) -> bool;
}
extern "C" {
    pub fn ice_vsi_num_non_zero_vlans(vsi: *mut ice_vsi) -> u16;
}
extern "C" {
    pub fn ice_is_feature_supported(pf: *mut ice_pf, f: ice_feature) -> bool;
}
extern "C" {
    pub fn ice_set_feature_support(pf: *mut ice_pf, f: ice_feature);
}
extern "C" {
    pub fn ice_clear_feature_support(pf: *mut ice_pf, f: ice_feature);
}
extern "C" {
    pub fn ice_init_feature_support(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_vsi_is_rx_queue_active(vsi: *mut ice_vsi) -> bool;
}
extern "C" {
    pub fn ice_vsi_update_l2tsel(vsi: *mut ice_vsi, l2tsel: ice_l2tsel);
}
