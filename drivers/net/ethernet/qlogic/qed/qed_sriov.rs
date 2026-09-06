//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_sriov.h
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

pub const QED_ETH_VF_NUM_MAC_FILTERS: c_int = 1;
pub const QED_ETH_VF_NUM_VLAN_FILTERS: c_int = 2;

pub const QED_MAX_VF_CHAINS_PER_PF: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_iov_vport_update_flag {
    QED_IOV_VP_UPDATE_ACTIVATE,
    QED_IOV_VP_UPDATE_VLAN_STRIP,
    QED_IOV_VP_UPDATE_TX_SWITCH,
    QED_IOV_VP_UPDATE_MCAST,
    QED_IOV_VP_UPDATE_ACCEPT_PARAM,
    QED_IOV_VP_UPDATE_RSS,
    QED_IOV_VP_UPDATE_ACCEPT_ANY_VLAN,
    QED_IOV_VP_UPDATE_SGE_TPA,
    QED_IOV_VP_UPDATE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_public_vf_info {
// These copies will later be reflected in the bulletin board,
// but this copy should be newer.
//
    pub forced_mac: [u8; ETH_ALEN],
    pub forced_vlan: u16,
    pub mac: [u8; ETH_ALEN],
// IFLA_VF_LINK_STATE_<X>
    pub link_state: c_int,
// Currently configured Tx rate in MB/sec. 0 if unconfigured
    pub tx_rate: c_int,
// Trusted VFs can configure promiscuous mode.
// Also store shadow promisc configuration if needed.
//
    pub is_trusted_configured: bool,
    pub is_trusted_request: bool,
    pub rx_accept_mode: u8,
    pub tx_accept_mode: u8,
    pub accept_any_vlan: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iov_vf_init_params {
    pub rel_vf_id: u16,
// Number of requested Queues; Currently, don't support different
// number of Rx/Tx queues.
//
    pub num_queues: u16,
// Allow the client to choose which qzones to use for Rx/Tx,
// and which queue_base to use for Tx queues on a per-queue basis.
// Notice values should be relative to the PF resources.
//
    pub req_rx_queue: [u16; QED_MAX_VF_CHAINS_PER_PF],
    pub req_tx_queue: [u16; QED_MAX_VF_CHAINS_PER_PF],
}

// This struct is part of qed_dev and contains data relevant to all hwfns;
// Initialized only if SR-IOV cpabability is exposed in PCIe config space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_hw_sriov_info {
    pub /: *mut *mut int pos; / capability position,
    pub /: *mut *mut int nres; / number of resources,
    pub /: *mut *mut u32 cap; / SR-IOV Capabilities,
    pub /: *mut *mut u16 ctrl; / SR-IOV Control,
    pub /: *mut *mut u16 total_vfs; / total VFs associated with the PF,
    pub /: *mut *mut u16 num_vfs; / number of vfs that have been started,
    pub /: *mut *mut u16 initial_vfs; / initial VFs associated with the PF,
    pub /: *mut *mut u16 nr_virtfn; / number of VFs available,
    pub /: *mut *mut u16 offset; / first VF Routing ID offset,
    pub /: *mut *mut u16 stride; / following VF stride,
    pub /: *mut *mut u16 vf_device_id; / VF device id,
    pub /: *mut *mut u32 pgsz; / page size for BAR alignment,
    pub /: *mut *mut u8 link; / Function Dependency Link,
    pub first_vf_in_pf: u32,
}

// This mailbox is maintained per VF in its PF contains all information
// required for sending / receiving a message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iov_vf_mbx {
    pub req_virt: *mut vfpf_tlvs,
    pub req_phys: dma_addr_t,
    pub reply_virt: *mut pfvf_tlvs,
    pub reply_phys: dma_addr_t,
// Address in VF where a pending message is located
    pub pending_req: dma_addr_t,
// Message from VF awaits handling
    pub b_pending_msg: bool,
    pub offset: *mut u8,
// saved VF request header
    pub first_tlv: vfpf_first_tlv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_vf_queue_cid {
    pub b_is_tx: bool,
    pub p_cid: *mut qed_queue_cid,
}

// Describes a qzone associated with the VF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_vf_queue {
    pub fw_rx_qid: u16,
    pub fw_tx_qid: u16,
    pub cids: [qed_vf_queue_cid; MAX_QUEUES_PER_QZONE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf_state {
    VF_FREE = 0,		/* VF ready to be acquired holds no resc */
    VF_ACQUIRED,		/* VF, acquired, but not initialized */
    VF_ENABLED,		/* VF, Enabled */
    VF_RESET,		/* VF, FLR'd, pending cleanup */
    VF_STOPPED		/* VF, Stopped */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_vf_vlan_shadow {
    pub used: bool,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_vf_shadow_config {
// Shadow copy of all guest vlans
    pub 1]: qed_vf_vlan_shadow vlans[QED_ETH_VF_NUM_VLAN_FILTERS +,
// Shadow copy of all configured MACs; Empty if forcing MACs
    pub macs: [u8; QED_ETH_VF_NUM_MAC_FILTERS][ETH_ALEN],
    pub inner_vlan_removal: u8,
}

// PFs maintain an array of this structure, per VF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_vf_info {
    pub vf_mbx: qed_iov_vf_mbx,
    pub state: vf_state,
    pub b_init: bool,
    pub b_malicious: bool,
    pub to_disable: u8,
    pub bulletin: qed_bulletin,
    pub vf_bulletin: dma_addr_t,
// PF saves a copy of the last VF acquire message
    pub acquire: vfpf_acquire_tlv,
    pub concrete_fid: u32,
    pub opaque_fid: u16,
    pub mtu: u16,
    pub vport_id: u8,
    pub relative_vf_id: u8,
    pub abs_vf_id: u8,

    pub vport_instance: u8,
    pub num_rxqs: u8,
    pub num_txqs: u8,
    pub rx_coal: u16,
    pub tx_coal: u16,
    pub num_sbs: u8,
    pub num_mac_filters: u8,
    pub num_vlan_filters: u8,
    pub vf_queues: [qed_vf_queue; QED_MAX_VF_CHAINS_PER_PF],
    pub igu_sbs: [u16; QED_MAX_VF_CHAINS_PER_PF],
    pub num_active_rxqs: u8,
    pub p_vf_info: qed_public_vf_info,
    pub spoof_chk: bool,
    pub req_spoofchk_val: bool,
// Stores the configuration requested by VF
    pub shadow_config: qed_vf_shadow_config,
// A bitfield using bulletin's valid-map bits, used to indicate
// which of the bulletin board features have been configured.
//
    pub configured_features: u64,

}

// This structure is part of qed_hwfn and used only for PFs that have sriov
// capability enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_pf_iov {
    pub vfs_array: [qed_vf_info; MAX_NUM_VFS],
    pub pending_flr: [u64; QED_VF_ARRAY_LENGTH],
// Allocate message address continuosuly and split to each VF
    pub mbx_msg_virt_addr: *mut c_void,
    pub mbx_msg_phys_addr: dma_addr_t,
    pub mbx_msg_size: u32,
    pub mbx_reply_virt_addr: *mut c_void,
    pub mbx_reply_phys_addr: dma_addr_t,
    pub mbx_reply_size: u32,
    pub p_bulletins: *mut c_void,
    pub bulletins_phys: dma_addr_t,
    pub bulletins_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_iov_wq_flag {
    QED_IOV_WQ_MSG_FLAG,
    QED_IOV_WQ_SET_UNICAST_FILTER_FLAG,
    QED_IOV_WQ_BULLETIN_UPDATE_FLAG,
    QED_IOV_WQ_STOP_WQ_FLAG,
    QED_IOV_WQ_FLR_FLAG,
    QED_IOV_WQ_TRUST_FLAG,
    QED_IOV_WQ_VF_FORCE_LINK_QUERY_FLAG,
}

//
// qed_iov_is_valid_vfid(): Check if given VF ID @vfid is valid
// w.r.t. @b_enabled_only value
// if b_enabled_only = true - only enabled
// VF id is valid.
// else any VF id less than max_vfs is valid.
//
// @p_hwfn: HW device data.
// @rel_vf_id: Relative VF ID.
// @b_enabled_only: consider only enabled VF.
// @b_non_malicious: true iff we want to validate vf isn't malicious.
//
// Return: bool - true for valid VF ID
//
// qed_iov_get_next_active_vf(): Given a VF index, return index of
// next [including that] active VF.
//
// @p_hwfn: HW device data.
// @rel_vf_id: VF ID.
//
// Return: MAX_NUM_VFS in case no further active VFs, otherwise index.
//
extern "C" {
    pub fn qed_iov_get_next_active_vf(p_hwfn: *mut qed_hwfn, rel_vf_id: u16) -> u16;
}
//
// qed_iov_hw_info(): Read sriov related information and allocated resources
// reads from configuration space, shmem, etc.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_iov_hw_info(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_add_tlv(): place a given tlv on the tlv buffer at next offset
//
// @p_hwfn: HW device data.
// @offset: offset.
// @type: Type
// @length: Length.
//
// Return: pointer to the newly placed tlv
//
// qed_dp_tlv_list(): list the types and lengths of the tlvs on the buffer
//
// @p_hwfn: HW device data.
// @tlvs_list: Tlvs_list.
//
// Return: Void.
//
extern "C" {
    pub fn qed_dp_tlv_list(p_hwfn: *mut qed_hwfn, tlvs_list: *mut c_void);
}
//
// qed_sriov_vfpf_malicious(): Handle malicious VF/PF.
//
// @p_hwfn: HW device data.
// @p_data: Pointer to data.
//
// Return: Void.
//
// qed_sriov_eqe_event(): Callback for SRIOV events.
//
// @p_hwfn: HW device data.
// @opcode: Opcode.
// @echo: Echo.
// @data: data
// @fw_return_code: FW return code.
//
// Return: Int.
//
// qed_iov_alloc(): allocate sriov related resources
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_iov_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_iov_setup(): setup sriov related resources
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_iov_setup(p_hwfn: *mut qed_hwfn);
}
//
// qed_iov_free(): free sriov related resources
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_iov_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_iov_free_hw_info(): free sriov related memory that was
// allocated during hw_prepare
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_iov_free_hw_info(cdev: *mut qed_dev);
}
//
// qed_iov_mark_vf_flr(): Mark structs of vfs that have been FLR-ed.
//
// @p_hwfn: HW device data.
// @disabled_vfs: bitmask of all VFs on path that were FLRed
//
// Return: true iff one of the PF's vfs got FLRed. false otherwise.
//
extern "C" {
    pub fn qed_iov_mark_vf_flr(p_hwfn: *mut qed_hwfn, disabled_vfs: *mut u32) -> bool;
}
//
// qed_iov_search_list_tlvs(): Search extended TLVs in request/reply buffer.
//
// @p_hwfn: HW device data.
// @p_tlvs_list: Pointer to tlvs list
// @req_type: Type of TLV
//
// Return: pointer to tlv type if found, otherwise returns NULL.
//
extern "C" {
    pub fn qed_iov_wq_stop(cdev: *mut qed_dev, schedule_first: bool);
}
extern "C" {
    pub fn qed_iov_wq_start(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_schedule_iov(hwfn: *mut qed_hwfn, flag: qed_iov_wq_flag);
}
extern "C" {
    pub fn qed_vf_start_iov_wq(cdev: *mut qed_dev);
}
extern "C" {
    pub fn qed_sriov_disable(cdev: *mut qed_dev, pci_enabled: bool) -> c_int;
}
extern "C" {
    pub fn qed_inform_vf_link_state(hwfn: *mut qed_hwfn);
}

