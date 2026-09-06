//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_vf.h
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
//

pub const T_ETH_INDIRECTION_TABLE_SIZE: c_int = 128;
pub const T_ETH_RSS_KEY_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_resc_request {
    pub num_rxqs: u8,
    pub num_txqs: u8,
    pub num_sbs: u8,
    pub num_mac_filters: u8,
    pub num_vlan_filters: u8,
    pub num_mc_filters: u8,
    pub num_cids: u8,
    pub padding: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_sb_info {
    pub hw_sb_id: u16,
    pub sb_qid: u8,
    pub padding: [u8; 5],
}

pub const TLV_BUFFER_SIZE: c_int = 1024;
// vf pf channel tlvs
// general tlv header (used for both vf->pf request and pf->vf response)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_tlv {
    pub type: u16,
    pub length: u16,
}

// header of first vf->pf tlv carries the offset used to calculate response
// buffer address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_first_tlv {
    pub tl: channel_tlv,
    pub padding: u32,
    pub reply_address: u64,
}

// header of pf->vf tlvs, carries the status of handling the request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_tlv {
    pub tl: channel_tlv,
    pub status: u8,
    pub padding: [u8; 3],
}

// response tlv used for most tlvs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_def_resp_tlv {
    pub hdr: pfvf_tlv,
}

// used to terminate and pad a tlv list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_list_end_tlv {
    pub tl: channel_tlv,
    pub padding: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_acquire_tlv {
    pub first_tlv: vfpf_first_tlv,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_vfdev_info {

// A requirement for supporting multi-Tx queues on a single queue-zone,
// VF would pass qids as additional information whenever passing queue
// references.
//

// The VF is using the physical bar. While this is mostly internal
// to the VF, might affect the number of CIDs supported assuming
// QUEUE_QIDS is set.
//

    pub capabilities: u64,
    pub fw_major: u8,
    pub fw_minor: u8,
    pub fw_revision: u8,
    pub fw_engineering: u8,
    pub driver_version: u32,
    pub /: *mut *mut u16 opaque_fid; / ME register value,
    pub /: *mut *mut *mut u8 os_type; / VFPF_ACQUIRE_OS_ value,
    pub eth_fp_hsi_major: u8,
    pub eth_fp_hsi_minor: u8,
    pub padding: [u8; 3],
    pub vfdev_info: },
    pub resc_request: vf_pf_resc_request,
    pub bulletin_addr: u64,
    pub bulletin_size: u32,
    pub padding: u32,
}

// receive side scaling tlv
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_rss_tlv {
    pub tl: channel_tlv,
    pub update_rss_flags: u8,

    pub rss_enable: u8,
    pub rss_caps: u8,
    pub /: *mut *mut u8 rss_table_size_log; / The table size is 2 ^ rss_table_size_log,
    pub rss_ind_table: [u16; T_ETH_INDIRECTION_TABLE_SIZE],
    pub rss_key: [u32; T_ETH_RSS_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_storm_stats {
    pub address: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_stats_info {
    pub mstats: pfvf_storm_stats,
    pub pstats: pfvf_storm_stats,
    pub tstats: pfvf_storm_stats,
    pub ustats: pfvf_storm_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_acquire_resp_tlv {
    pub hdr: pfvf_tlv,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_vf_pfdev_info {
    pub chip_num: u32,
    pub mfw_ver: u32,
    pub fw_major: u16,
    pub fw_minor: u16,
    pub fw_rev: u16,
    pub fw_eng: u16,
    pub capabilities: u64,

// There are old PF versions where the PF might mistakenly override the sanity
// mechanism [version-based] and allow a VF that can't be supported to pass
// the acquisition phase.
// To overcome this, PFs now indicate that they're past that point and the new
// VFs would fail probe on the older PFs that fail to do so.
//

// PF expects queues to be received with additional qids

    pub db_size: u16,
    pub indices_per_sb: u8,
    pub os_type: u8,
// These should match the PF's qed_dev values
    pub chip_rev: u16,
    pub dev_type: u8,
// Doorbell bar size configured in HW: log(size) or 0
    pub bar_size: u8,
    pub stats_info: pfvf_stats_info,
    pub port_mac: [u8; ETH_ALEN],
// It's possible PF had to configure an older fastpath HSI
// [in case VF is newer than PF]. This is communicated back
// to the VF. It can also be used in case of error due to
// non-matching versions to shed light in VF about failure.
//
    pub major_fp_hsi: u8,
    pub minor_fp_hsi: u8,
    pub pfdev_info: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_vf_resc {
pub const PFVF_MAX_QUEUES_PER_VF: c_int = 16;
pub const PFVF_MAX_SBS_PER_VF: c_int = 16;
    pub hw_sbs: [hw_sb_info; PFVF_MAX_SBS_PER_VF],
    pub hw_qid: [u8; PFVF_MAX_QUEUES_PER_VF],
    pub cid: [u8; PFVF_MAX_QUEUES_PER_VF],
    pub num_rxqs: u8,
    pub num_txqs: u8,
    pub num_sbs: u8,
    pub num_mac_filters: u8,
    pub num_vlan_filters: u8,
    pub num_mc_filters: u8,
    pub num_cids: u8,
    pub padding: u8,
    pub resc: },
    pub bulletin_size: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_start_queue_resp_tlv {
    pub hdr: pfvf_tlv,
    pub /: *mut *mut u32 offset; / offset to consumer/producer of queue,
    pub padding: [u8; 4],
}

// Extended queue information - additional index for reference inside qzone.
// If communicated between VF/PF, each TLV relating to queues should be
// extended by one such [or have a future base TLV that already contains info].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_qid_tlv {
    pub tl: channel_tlv,
    pub qid: u8,
    pub padding: [u8; 3],
}

// Setup Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_start_rxq_tlv {
    pub first_tlv: vfpf_first_tlv,
// physical addresses
    pub rxq_addr: u64,
    pub deprecated_sge_addr: u64,
    pub cqe_pbl_addr: u64,
    pub cqe_pbl_size: u16,
    pub hw_sb: u16,
    pub rx_qid: u16,
    pub /: *mut *mut u16 hc_rate; / desired interrupts per sec.,
    pub bd_max_bytes: u16,
    pub stat_id: u16,
    pub sb_index: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_start_txq_tlv {
    pub first_tlv: vfpf_first_tlv,
// physical addresses
    pub pbl_addr: u64,
    pub pbl_size: u16,
    pub stat_id: u16,
    pub tx_qid: u16,
    pub hw_sb: u16,
    pub /: *mut *mut u32 flags; / VFPF_QUEUE_FLG_X flags,
    pub /: *mut *mut u16 hc_rate; / desired interrupts per sec.,
    pub sb_index: u8,
    pub padding: [u8; 3],
}

// Stop RX Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_stop_rxqs_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub rx_qid: u16,
// this field is deprecated and should *always* be set to '1'
    pub num_rxqs: u8,
    pub cqe_completion: u8,
    pub padding: [u8; 4],
}

// Stop TX Queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_stop_txqs_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub tx_qid: u16,
// this field is deprecated and should *always* be set to '1'
    pub num_txqs: u8,
    pub padding: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_update_rxq_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub deprecated_sge_addr: [u64; PFVF_MAX_QUEUES_PER_VF],
    pub rx_qid: u16,
    pub num_rxqs: u8,
    pub flags: u8,
    pub padding: [u8; 4],
}

// Set Queue Filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_q_mac_vlan_filter {
    pub flags: u32,
pub const VFPF_Q_FILTER_DEST_MAC_VALID: c_uint = 0x01;
pub const VFPF_Q_FILTER_VLAN_TAG_VALID: c_uint = 0x02;
pub const VFPF_Q_FILTER_SET_MAC: c_uint = 0x100	/* set/clear */;
    pub mac: [u8; ETH_ALEN],
    pub vlan_tag: u16,
    pub padding: [u8; 4],
}

// Start a vport
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_start_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub sb_addr: [u64; PFVF_MAX_SBS_PER_VF],
    pub tpa_mode: u32,
    pub dep1: u16,
    pub mtu: u16,
    pub vport_id: u8,
    pub inner_vlan_removal: u8,
    pub only_untagged: u8,
    pub max_buffers_per_cqe: u8,
    pub padding: [u8; 4],
}

// Extended tlvs - need to add rss, mcast, accept mode tlvs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_activate_tlv {
    pub tl: channel_tlv,
    pub update_rx: u8,
    pub update_tx: u8,
    pub active_rx: u8,
    pub active_tx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_tx_switch_tlv {
    pub tl: channel_tlv,
    pub tx_switching: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_vlan_strip_tlv {
    pub tl: channel_tlv,
    pub remove_vlan: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_mcast_bin_tlv {
    pub tl: channel_tlv,
    pub padding: [u8; 4],
// There are only 256 approx bins, and in HSI they're divided into
// 32-bit values. As old VFs used to set-bit to the values on its side,
// the upper half of the array is never expected to contain any data.
//
    pub bins: [u64; 4],
    pub obsolete_bins: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_accept_param_tlv {
    pub tl: channel_tlv,
    pub update_rx_mode: u8,
    pub update_tx_mode: u8,
    pub rx_accept_filter: u8,
    pub tx_accept_filter: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_accept_any_vlan_tlv {
    pub tl: channel_tlv,
    pub update_accept_any_vlan_flg: u8,
    pub accept_any_vlan: u8,
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_sge_tpa_tlv {
    pub tl: channel_tlv,
    pub sge_tpa_flags: u16,

    pub update_sge_tpa_flags: u8,

    pub max_buffers_per_cqe: u8,
    pub deprecated_sge_buff_size: u16,
    pub tpa_max_size: u16,
    pub tpa_min_size_to_start: u16,
    pub tpa_min_size_to_cont: u16,
    pub tpa_max_aggs_num: u8,
    pub padding: [u8; 7],
}

// Primary tlv as a header for various extended tlvs for
// various functionalities in vport update ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_vport_update_tlv {
    pub first_tlv: vfpf_first_tlv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_ucast_filter_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub opcode: u8,
    pub type: u8,
    pub mac: [u8; ETH_ALEN],
    pub vlan: u16,
    pub padding: [u16; 3],
}

// tunnel update param tlv
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_update_tunn_param_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub tun_mode_update_mask: u8,
    pub tunn_mode: u8,
    pub update_tun_cls: u8,
    pub vxlan_clss: u8,
    pub l2gre_clss: u8,
    pub ipgre_clss: u8,
    pub l2geneve_clss: u8,
    pub ipgeneve_clss: u8,
    pub update_geneve_port: u8,
    pub update_vxlan_port: u8,
    pub geneve_port: u16,
    pub vxlan_port: u16,
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_update_tunn_param_tlv {
    pub hdr: pfvf_tlv,
    pub tunn_feature_mask: u16,
    pub vxlan_mode: u8,
    pub l2geneve_mode: u8,
    pub ipgeneve_mode: u8,
    pub l2gre_mode: u8,
    pub ipgre_mode: u8,
    pub vxlan_clss: u8,
    pub l2gre_clss: u8,
    pub ipgre_clss: u8,
    pub l2geneve_clss: u8,
    pub ipgeneve_clss: u8,
    pub vxlan_udp_port: u16,
    pub geneve_udp_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_buffer_size {
    pub tlv_buffer: [u8; TLV_BUFFER_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_update_coalesce {
    pub first_tlv: vfpf_first_tlv,
    pub rx_coal: u16,
    pub tx_coal: u16,
    pub qid: u16,
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_read_coal_req_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub qid: u16,
    pub is_rx: u8,
    pub padding: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_read_coal_resp_tlv {
    pub hdr: pfvf_tlv,
    pub coal: u16,
    pub padding: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_bulletin_update_mac_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub mac: [u8; ETH_ALEN],
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vfpf_tlvs {
    pub first_tlv: vfpf_first_tlv,
    pub acquire: vfpf_acquire_tlv,
    pub start_rxq: vfpf_start_rxq_tlv,
    pub start_txq: vfpf_start_txq_tlv,
    pub stop_rxqs: vfpf_stop_rxqs_tlv,
    pub stop_txqs: vfpf_stop_txqs_tlv,
    pub update_rxq: vfpf_update_rxq_tlv,
    pub start_vport: vfpf_vport_start_tlv,
    pub vport_update: vfpf_vport_update_tlv,
    pub ucast_filter: vfpf_ucast_filter_tlv,
    pub tunn_param_update: vfpf_update_tunn_param_tlv,
    pub update_coalesce: vfpf_update_coalesce,
    pub read_coal_req: vfpf_read_coal_req_tlv,
    pub bulletin_update_mac: vfpf_bulletin_update_mac_tlv,
    pub tlv_buf_size: tlv_buffer_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pfvf_tlvs {
    pub default_resp: pfvf_def_resp_tlv,
    pub acquire_resp: pfvf_acquire_resp_tlv,
    pub tlv_buf_size: tlv_buffer_size,
    pub queue_start: pfvf_start_queue_resp_tlv,
    pub tunn_param_resp: pfvf_update_tunn_param_tlv,
    pub read_coal_resp: pfvf_read_coal_resp_tlv,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_bulletin_bit {
// Alert the VF that a forced MAC was set by the PF
    MAC_ADDR_FORCED = 0,
// Alert the VF that a forced VLAN was set by the PF
    VLAN_ADDR_FORCED = 2,

// Indicate that `default_only_untagged' contains actual data
    VFPF_BULLETIN_UNTAGGED_DEFAULT = 3,
    VFPF_BULLETIN_UNTAGGED_DEFAULT_FORCED = 4,

// Alert the VF that suggested mac was sent by the PF.
// MAC_ADDR will be disabled in case MAC_ADDR_FORCED is set.
//
    VFPF_BULLETIN_MAC_ADDR = 5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_bulletin_content {
// crc of structure to ensure is not in mid-update
    pub crc: u32,
    pub version: u32,
// bitmap indicating which fields hold valid values
    pub valid_bitmap: u64,
// used for MAC_ADDR or MAC_ADDR_FORCED
    pub mac: [u8; ETH_ALEN],
// If valid, 1 => only untagged Rx if no vlan is configured
    pub default_only_untagged: u8,
    pub padding: u8,
// The following is a 'copy' of qed_mcp_link_state,
// qed_mcp_link_params and qed_mcp_link_capabilities. Since it's
// possible the structs will increase further along the road we cannot
// have it here; Instead we need to have all of its fields.
//
    pub req_autoneg: u8,
    pub req_autoneg_pause: u8,
    pub req_forced_rx: u8,
    pub req_forced_tx: u8,
    pub padding2: [u8; 4],
    pub req_adv_speed: u32,
    pub req_forced_speed: u32,
    pub req_loopback: u32,
    pub padding3: u32,
    pub link_up: u8,
    pub full_duplex: u8,
    pub autoneg: u8,
    pub autoneg_complete: u8,
    pub parallel_detection: u8,
    pub pfc_enabled: u8,
    pub partner_tx_flow_ctrl_en: u8,
    pub partner_rx_flow_ctrl_en: u8,
    pub partner_adv_pause: u8,
    pub sfp_tx_fault: u8,
    pub vxlan_udp_port: u16,
    pub geneve_udp_port: u16,
    pub padding4: [u8; 2],
    pub speed: u32,
    pub partner_adv_speed: u32,
    pub capability_speed: u32,
// Forced vlan
    pub pvid: u16,
    pub padding5: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_bulletin {
    pub phys: dma_addr_t,
    pub p_virt: *mut qed_bulletin_content,
    pub size: u32,
}

// Required for iterating over vport-update tlvs.
// Will break in case non-sequential vport-update tlvs.
//
// Default number of CIDs [total of both Rx and Tx] to be requested
// by default, and maximum possible number.
//

// This data is held in the qed_hwfn structure for VFs only.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_vf_iov {
    pub vf2pf_request: *mut vfpf_tlvs,
    pub vf2pf_request_phys: dma_addr_t,
    pub pf2vf_reply: *mut pfvf_tlvs,
    pub pf2vf_reply_phys: dma_addr_t,
// Should be taken whenever the mailbox buffers are accessed
    pub mutex: mutex,
    pub offset: *mut u8,
// Bulletin Board
    pub bulletin: qed_bulletin,
    pub bulletin_shadow: qed_bulletin_content,
// we set aside a copy of the acquire response
    pub acquire_resp: pfvf_acquire_resp_tlv,
// In case PF originates prior to the fp-hsi version comparison,
// this has to be propagated as it affects the fastpath.
//
    pub b_pre_fp_hsi: bool,
// Current day VFs are passing the SBs physical address on vport
// start, and as they lack an IGU mapping they need to store the
// addresses of previously registered SBs.
// Even if we were to change configuration flow, due to backward
// compatibility [with older PFs] we'd still need to store these.
//
    pub sbs_info: [*mut qed_sb_info; PFVF_MAX_SBS_PER_VF],
// Determines whether VF utilizes doorbells via limited register
// bar or via the doorbell bar.
//
    pub b_doorbell_bar: bool,
}

//
// qed_vf_pf_set_coalesce(): VF - Set Rx/Tx coalesce per VF's relative queue.
// Coalesce value '0' will omit the
// configuration.
//
// @p_hwfn: HW device data.
// @rx_coal: coalesce value in micro second for rx queue.
// @tx_coal: coalesce value in micro second for tx queue.
// @p_cid: queue cid.
//
// Return: Int.
//
// qed_vf_pf_get_coalesce(): VF - Get coalesce per VF's relative queue.
//
// @p_hwfn: HW device data.
// @p_coal: coalesce value in micro second for VF queues.
// @p_cid: queue cid.
//
// Return: Int.
//

//
// qed_vf_read_bulletin(): Read the VF bulletin and act on it if needed.
//
// @p_hwfn: HW device data.
// @p_change: qed fills 1 iff bulletin board has changed, 0 otherwise.
//
// Return: enum _qed_status.
//
extern "C" {
    pub fn qed_vf_read_bulletin(p_hwfn: *mut qed_hwfn, p_change: *mut u8) -> c_int;
}
//
// qed_vf_get_link_params(): Get link parameters for VF from qed
//
// @p_hwfn: HW device data.
// @params: the link params structure to be filled for the VF.
//
// Return: Void.
//
// qed_vf_get_link_state(): Get link state for VF from qed.
//
// @p_hwfn: HW device data.
// @link: the link state structure to be filled for the VF
//
// Return: Void.
//
// qed_vf_get_link_caps(): Get link capabilities for VF from qed.
//
// @p_hwfn: HW device data.
// @p_link_caps: the link capabilities structure to be filled for the VF
//
// Return: Void.
//
// qed_vf_get_num_rxqs(): Get number of Rx queues allocated for VF by qed
//
// @p_hwfn: HW device data.
// @num_rxqs: allocated RX queues
//
// Return: Void.
//
extern "C" {
    pub fn qed_vf_get_num_rxqs(p_hwfn: *mut qed_hwfn, num_rxqs: *mut u8);
}
//
// qed_vf_get_num_txqs(): Get number of Rx queues allocated for VF by qed
//
// @p_hwfn: HW device data.
// @num_txqs: allocated RX queues
//
// Return: Void.
//
extern "C" {
    pub fn qed_vf_get_num_txqs(p_hwfn: *mut qed_hwfn, num_txqs: *mut u8);
}
//
// qed_vf_get_num_cids(): Get number of available connections
// [both Rx and Tx] for VF
//
// @p_hwfn: HW device data.
// @num_cids: allocated number of connections
//
// Return: Void.
//
extern "C" {
    pub fn qed_vf_get_num_cids(p_hwfn: *mut qed_hwfn, num_cids: *mut u8);
}
//
// qed_vf_get_port_mac(): Get port mac address for VF.
//
// @p_hwfn: HW device data.
// @port_mac: destination location for port mac
//
// Return: Void.
//
extern "C" {
    pub fn qed_vf_get_port_mac(p_hwfn: *mut qed_hwfn, port_mac: *mut u8);
}
//
// qed_vf_get_num_vlan_filters(): Get number of VLAN filters allocated
// for VF by qed.
//
// @p_hwfn: HW device data.
// @num_vlan_filters: allocated VLAN filters
//
// Return: Void.
//
// qed_vf_get_num_mac_filters(): Get number of MAC filters allocated
// for VF by qed
//
// @p_hwfn: HW device data.
// @num_mac_filters: allocated MAC filters
//
// Return: Void.
//
extern "C" {
    pub fn qed_vf_get_num_mac_filters(p_hwfn: *mut qed_hwfn, num_mac_filters: *mut u8);
}
//
// qed_vf_check_mac(): Check if VF can set a MAC address
//
// @p_hwfn: HW device data.
// @mac: Mac.
//
// Return: bool.
//
extern "C" {
    pub fn qed_vf_check_mac(p_hwfn: *mut qed_hwfn, mac: *mut u8) -> bool;
}
//
// qed_vf_get_fw_version(): Set firmware version information
// in dev_info from VFs acquire response tlv
//
// @p_hwfn: HW device data.
// @fw_major: FW major.
// @fw_minor: FW minor.
// @fw_rev: FW rev.
// @fw_eng: FW eng.
//
// Return: Void.
//
// qed_vf_hw_prepare(): hw preparation for VF  sends ACQUIRE message
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_vf_hw_prepare(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_vf_pf_rxq_start(): start the RX Queue by sending a message to the PF
//
// @p_hwfn: HW device data.
// @p_cid: Only relative fields are relevant
// @bd_max_bytes: maximum number of bytes per bd
// @bd_chain_phys_addr: physical address of bd chain
// @cqe_pbl_addr: physical address of pbl
// @cqe_pbl_size: pbl size
// @pp_prod: pointer to the producer to be used in fastpath
//
// Return: Int.
//
// qed_vf_pf_txq_start(): VF - start the TX queue by sending a message to the
// PF.
//
// @p_hwfn: HW device data.
// @p_cid: CID.
// @pbl_addr: PBL address.
// @pbl_size: PBL Size.
// @pp_doorbell: pointer to address to which to write the doorbell too.
//
// Return: Int.
//
// qed_vf_pf_rxq_stop(): VF - stop the RX queue by sending a message to the PF.
//
// @p_hwfn: HW device data.
// @p_cid: CID.
// @cqe_completion: CQE Completion.
//
// Return: Int.
//
// qed_vf_pf_txq_stop(): VF - stop the TX queue by sending a message to the PF.
//
// @p_hwfn: HW device data.
// @p_cid: CID.
//
// Return: Int.
//
extern "C" {
    pub fn qed_vf_pf_txq_stop(p_hwfn: *mut qed_hwfn, p_cid: *mut qed_queue_cid) -> c_int;
}
//
// qed_vf_pf_vport_update(): VF - send a vport update command.
//
// @p_hwfn: HW device data.
// @p_params: Params
//
// Return: Int.
//
// qed_vf_pf_reset(): VF - send a close message to PF.
//
// @p_hwfn: HW device data.
//
// Return: enum _qed_status
//
extern "C" {
    pub fn qed_vf_pf_reset(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_vf_pf_release(): VF - free vf`s memories.
//
// @p_hwfn: HW device data.
//
// Return: enum _qed_status
//
extern "C" {
    pub fn qed_vf_pf_release(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_vf_get_igu_sb_id(): Get the IGU SB ID for a given
// sb_id. For VFs igu sbs don't have to be contiguous
//
// @p_hwfn: HW device data.
// @sb_id: SB ID.
//
// Return: INLINE u16
//
extern "C" {
    pub fn qed_vf_get_igu_sb_id(p_hwfn: *mut qed_hwfn, sb_id: u16) -> u16;
}
//
// qed_vf_set_sb_info(): Stores [or removes] a configured sb_info.
//
// @p_hwfn: HW device data.
// @sb_id: zero-based SB index [for fastpath]
// @p_sb:  may be NULL [during removal].
//
// Return: Void.
//
// qed_vf_pf_vport_start(): perform vport start for VF.
//
// @p_hwfn: HW device data.
// @vport_id: Vport ID.
// @mtu: MTU.
// @inner_vlan_removal: Innter VLAN removal.
// @tpa_mode: TPA mode
// @max_buffers_per_cqe: Max buffer pre CQE.
// @only_untagged: default behavior regarding vlan acceptance
//
// Return: enum _qed_status
//
// qed_vf_pf_vport_stop(): stop the VF's vport
//
// @p_hwfn: HW device data.
//
// Return: enum _qed_status
//
extern "C" {
    pub fn qed_vf_pf_vport_stop(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_vf_pf_int_cleanup(): clean the SB of the VF
//
// @p_hwfn: HW device data.
//
// Return: enum _qed_status
//
extern "C" {
    pub fn qed_vf_pf_int_cleanup(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// __qed_vf_get_link_params(): return the link params in a given bulletin board
//
// @p_hwfn: HW device data.
// @p_params: pointer to a struct to fill with link params
// @p_bulletin: Bulletin.
//
// Return: Void.
//
// __qed_vf_get_link_state(): return the link state in a given bulletin board
//
// @p_hwfn: HW device data.
// @p_link: pointer to a struct to fill with link state
// @p_bulletin: Bulletin.
//
// Return: Void.
//
// __qed_vf_get_link_caps(): return the link capabilities in a given
// bulletin board
//
// @p_hwfn: HW device data.
// @p_link_caps: pointer to a struct to fill with link capabilities
// @p_bulletin: Bulletin.
//
// Return: Void.
//
extern "C" {
    pub fn qed_iov_vf_task(work: *mut work_struct);
}
extern "C" {
    pub fn qed_vf_set_vf_start_tunn_update_param(p_tun: *mut qed_tunnel_info);
}
extern "C" {
    pub fn qed_vf_hw_bar_size(p_hwfn: *mut qed_hwfn, bar_id: BAR_ID) -> u32;
}
//
// qed_vf_pf_bulletin_update_mac(): Ask PF to update the MAC address in
// it's bulletin board
//
// @p_hwfn: HW device data.
// @p_mac: mac address to be updated in bulletin board
//
// Return: Int.
//
extern "C" {
    pub fn qed_vf_pf_bulletin_update_mac(p_hwfn: *mut qed_hwfn, p_mac: *const u8) -> c_int;
}

// p_params,
// p_bulletin)

