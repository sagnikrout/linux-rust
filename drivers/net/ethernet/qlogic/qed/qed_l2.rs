//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_l2.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rss_params {
    pub update_rss_config: u8,
    pub rss_enable: u8,
    pub rss_eng_id: u8,
    pub update_rss_capabilities: u8,
    pub update_rss_ind_table: u8,
    pub update_rss_key: u8,
    pub rss_caps: u8,
    pub rss_table_size_log: u8,
// Indirection table consist of rx queue handles
    pub rss_ind_table: [*mut c_void; QED_RSS_IND_TABLE_SIZE],
    pub rss_key: [u32; QED_RSS_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sge_tpa_params {
    pub max_buffers_per_cqe: u8,
    pub update_tpa_en_flg: u8,
    pub tpa_ipv4_en_flg: u8,
    pub tpa_ipv6_en_flg: u8,
    pub tpa_ipv4_tunn_en_flg: u8,
    pub tpa_ipv6_tunn_en_flg: u8,
    pub update_tpa_param_flg: u8,
    pub tpa_pkt_split_flg: u8,
    pub tpa_hdr_data_split_flg: u8,
    pub tpa_gro_consistent_flg: u8,
    pub tpa_max_aggs_num: u8,
    pub tpa_max_size: u16,
    pub tpa_min_size_to_start: u16,
    pub tpa_min_size_to_cont: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_filter_opcode {
    QED_FILTER_ADD,
    QED_FILTER_REMOVE,
    QED_FILTER_MOVE,
    QED_FILTER_REPLACE,	/* Delete all MACs and add new one instead */
    QED_FILTER_FLUSH,	/* Removes all filters */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_filter_ucast_type {
    QED_FILTER_MAC,
    QED_FILTER_VLAN,
    QED_FILTER_MAC_VLAN,
    QED_FILTER_INNER_MAC,
    QED_FILTER_INNER_VLAN,
    QED_FILTER_INNER_PAIR,
    QED_FILTER_INNER_MAC_VNI_PAIR,
    QED_FILTER_MAC_VNI_PAIR,
    QED_FILTER_VNI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_filter_ucast {
    pub opcode: qed_filter_opcode,
    pub type: qed_filter_ucast_type,
    pub is_rx_filter: u8,
    pub is_tx_filter: u8,
    pub vport_to_add_to: u8,
    pub vport_to_remove_from: u8,
    pub mac: [c_uchar; ETH_ALEN],
    pub assert_on_error: u8,
    pub vlan: u16,
    pub vni: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_filter_mcast {
// MOVE is not supported for multicast
    pub opcode: qed_filter_opcode,
    pub vport_to_add_to: u8,
    pub vport_to_remove_from: u8,
    pub num_mc_addrs: u8,
pub const QED_MAX_MC_ADDRS: c_int = 64;
    pub mac: [c_uchar; QED_MAX_MC_ADDRS][ETH_ALEN],
}

//
// qed_eth_rx_queue_stop(): This ramrod closes an Rx queue.
//
// @p_hwfn: HW device data.
// @p_rxq: Handler of queue to close
// @eq_completion_only: If True completion will be on
// EQe, if False completion will be
// on EQe if p_hwfn opaque
// different from the RXQ opaque
// otherwise on CQe.
// @cqe_completion: If True completion will be receive on CQe.
//
// Return: Int.
//
// qed_eth_tx_queue_stop(): Closes a Tx queue.
//
// @p_hwfn: HW device data.
// @p_txq: handle to Tx queue needed to be closed.
//
// Return: Int.
//
extern "C" {
    pub fn qed_eth_tx_queue_stop(p_hwfn: *mut qed_hwfn, p_txq: *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_tpa_mode {
    QED_TPA_MODE_NONE,
    QED_TPA_MODE_UNUSED,
    QED_TPA_MODE_GRO,
    QED_TPA_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sp_vport_start_params {
    pub tpa_mode: qed_tpa_mode,
    pub remove_inner_vlan: bool,
    pub tx_switching: bool,
    pub handle_ptp_pkts: bool,
    pub only_untagged: bool,
    pub drop_ttl0: bool,
    pub max_buffers_per_cqe: u8,
    pub concrete_fid: u32,
    pub opaque_fid: u16,
    pub vport_id: u8,
    pub mtu: u16,
    pub check_mac: bool,
    pub check_ethtype: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_filter_accept_flags {
    pub update_rx_mode_config: u8,
    pub update_tx_mode_config: u8,
    pub rx_accept_filter: u8,
    pub tx_accept_filter: u8,
pub const QED_ACCEPT_NONE: c_uint = 0x01;
pub const QED_ACCEPT_UCAST_MATCHED: c_uint = 0x02;
pub const QED_ACCEPT_UCAST_UNMATCHED: c_uint = 0x04;
pub const QED_ACCEPT_MCAST_MATCHED: c_uint = 0x08;
pub const QED_ACCEPT_MCAST_UNMATCHED: c_uint = 0x10;
pub const QED_ACCEPT_BCAST: c_uint = 0x20;
pub const QED_ACCEPT_ANY_VNI: c_uint = 0x40;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_arfs_config_params {
    pub tcp: bool,
    pub udp: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub mode: qed_filter_config_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sp_vport_update_params {
    pub opaque_fid: u16,
    pub vport_id: u8,
    pub update_vport_active_rx_flg: u8,
    pub vport_active_rx_flg: u8,
    pub update_vport_active_tx_flg: u8,
    pub vport_active_tx_flg: u8,
    pub update_inner_vlan_removal_flg: u8,
    pub inner_vlan_removal_flg: u8,
    pub silent_vlan_removal_flg: u8,
    pub update_default_vlan_enable_flg: u8,
    pub default_vlan_enable_flg: u8,
    pub update_default_vlan_flg: u8,
    pub default_vlan: u16,
    pub update_tx_switching_flg: u8,
    pub tx_switching_flg: u8,
    pub update_approx_mcast_flg: u8,
    pub update_anti_spoofing_en_flg: u8,
    pub anti_spoofing_en: u8,
    pub update_accept_any_vlan_flg: u8,
    pub accept_any_vlan: u8,
    pub bins: [u32; 8],
    pub rss_params: *mut qed_rss_params,
    pub accept_flags: qed_filter_accept_flags,
    pub sge_tpa_params: *mut qed_sge_tpa_params,
    pub update_ctl_frame_check: u8,
    pub mac_chk_en: u8,
    pub ethtype_chk_en: u8,
}

//
// qed_sp_vport_stop: This ramrod closes a VPort after all its
// RX and TX queues are terminated.
// An Assert is generated if any queues are left open.
//
// @p_hwfn: HW device data.
// @opaque_fid: Opaque FID
// @vport_id: VPort ID.
//
// Return: Int.
//
extern "C" {
    pub fn qed_sp_vport_stop(p_hwfn: *mut qed_hwfn, opaque_fid: u16, vport_id: u8) -> c_int;
}
//
// qed_sp_eth_rx_queues_update(): This ramrod updates an RX queue.
// It is used for setting the active state
// of the queue and updating the TPA and
// SGE parameters.
// @p_hwfn: HW device data.
// @pp_rxq_handlers: An array of queue handlers to be updated.
// @num_rxqs: number of queues to update.
// @complete_cqe_flg: Post completion to the CQE Ring if set.
// @complete_event_flg: Post completion to the Event Ring if set.
// @comp_mode: Comp mode.
// @p_comp_data: Pointer Comp data.
//
// Return: Int.
//
// Note At the moment - only used by non-linux VFs.
//
// qed_get_vport_stats(): Fills provided statistics
// struct with statistics.
//
// @cdev: Qed dev pointer.
// @stats: Points to struct that will be filled with statistics.
//
// Return: Void.
//
extern "C" {
    pub fn qed_get_vport_stats(cdev: *mut qed_dev, stats: *mut qed_eth_stats);
}
//
// qed_get_vport_stats_context(): Fills provided statistics
// struct with statistics.
//
// @cdev: Qed dev pointer.
// @stats: Points to struct that will be filled with statistics.
// @is_atomic: Hint from the caller - if the func can sleep or not.
//
// Context: The function should not sleep in case is_atomic == true.
// Return: Void.
//
extern "C" {
    pub fn qed_reset_vport_stats(cdev: *mut qed_dev);
}
//
// qed_arfs_mode_configure(): Enable or disable rfs mode.
// It must accept at least one of tcp or udp true
// and at least one of ipv4 or ipv6 true to enable
// rfs mode.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_cfg_params: arfs mode configuration parameters.
//
// Return. Void.
//
// qed_configure_rfs_ntuple_filter(): This ramrod should be used to add
// or remove arfs hw filter
//
// @p_hwfn: HW device data.
// @p_cb: Used for QED_SPQ_MODE_CB,where client would initialize
// it with cookie and callback function address, if not
// using this mode then client must pass NULL.
// @p_params: Pointer to params.
//
// Return: Void.
//

// Almost identical to the qed_queue_start_common_params,
// but here we maintain the SB index in IGU CAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_queue_cid_params {
    pub vport_id: u8,
    pub queue_id: u16,
    pub stats_id: u8,
}

// Additional parameters required for initialization of the queue_cid
// and are relevant only for a PF initializing one for its VFs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_queue_cid_vf_params {
// Should match the VF's relative index
    pub vfid: u8,
// 0-based queue index. Should reflect the relative qzone the
// VF thinks is associated with it [in its range].
//
    pub vf_qid: u8,
// Indicates a VF is legacy, making it differ in several things:
// - Producers would be placed in a different place.
// - Makes assumptions regarding the CIDs.
//
    pub vf_legacy: u8,
    pub qid_usage_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_queue_cid {
// For stats-id, the `rel' is actually absolute as well
    pub rel: qed_queue_cid_params,
    pub abs: qed_queue_cid_params,
// These have no 'relative' meaning
    pub sb_igu_id: u16,
    pub sb_idx: u8,
    pub cid: u32,
    pub opaque_fid: u16,
    pub b_is_rx: bool,
// VFs queues are mapped differently, so we need to know the
// relative queue associated with them [0-based].
// Notice this is relevant on the *PF* queue-cid of its VF's queues,
// and not on the VF itself.
//
    pub vfid: u8,
    pub vf_qid: u8,
// We need an additional index to differentiate between queues opened
// for same queue-zone, as VFs would have to communicate the info
// to the PF [otherwise PF has no way to differentiate].
//
    pub qid_usage_idx: u8,
    pub vf_legacy: u8,

    pub p_owner: *mut qed_hwfn,
}

extern "C" {
    pub fn qed_l2_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_l2_setup(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_l2_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_eth_rxq_start_ramrod(): Starts an Rx queue, when queue_cid is
// already prepared
//
// @p_hwfn: HW device data.
// @p_cid: Pointer CID.
// @bd_max_bytes: Max bytes.
// @bd_chain_phys_addr: Chain physcial address.
// @cqe_pbl_addr: PBL address.
// @cqe_pbl_size: PBL size.
//
// Return: Int.
//
// qed_eth_txq_start_ramrod(): Starts a Tx queue, where queue_cid is
// already prepared
//
// @p_hwfn: HW device data.
// @p_cid: Pointer CID.
// @pbl_addr: PBL address.
// @pbl_size: PBL size.
// @pq_id: Parameters for choosing the PQ for this Tx queue.
//
// Return: Int.
//
extern "C" {
    pub fn qed_mcast_bin_from_mac(mac: *mut u8) -> u8;
}
