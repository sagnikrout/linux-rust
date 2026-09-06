//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_vfpf.h
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


// bnx2x_vfpf.h: Qlogic Everest network driver.
//
// Copyright (c) 2011-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// Unless you and Qlogic execute a separate written software license
// agreement governing use of this software, this software is licensed to you
// under the terms of the GNU General Public License version 2 (the “GPL”),
// available at http://www.gnu.org/licenses/gpl-2.0.html, with the following
// added to such license:
//
// As a special exception, the copyright holders of this software give you
// permission to link this software with independent modules, and to copy and
// distribute the resulting executable under terms of your choice, provided that
// you also meet, for each linked independent module, the terms and conditions
// of the license of that module.  An independent module is a module which is
// not derived from this software.  The special exception does not apply to any
// modifications of the software.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Ariel Elior <ariel.elior@qlogic.com>
//

// Common definitions for all HVs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_resc_request {
    pub num_rxqs: u8,
    pub num_txqs: u8,
    pub num_sbs: u8,
    pub num_mac_filters: u8,
    pub num_vlan_filters: u8,
    pub /: *mut *mut u8 num_mc_filters; / No limit so superfluous,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_sb_info {
    pub /: *mut *mut u8 hw_sb_id; / aka absolute igu id, used to ack the sb,
    pub /: *mut *mut u8 sb_qid; / used to update DHC for sb,
}

// HW VF-PF channel definitions
// A.K.A VF-PF mailbox
//
pub const TLV_BUFFER_SIZE: c_int = 1024;
pub const PF_VF_BULLETIN_SIZE: c_int = 512;
pub const VFPF_QUEUE_FLG_TPA: c_uint = 0x0001;
pub const VFPF_QUEUE_FLG_TPA_IPV6: c_uint = 0x0002;
pub const VFPF_QUEUE_FLG_TPA_GRO: c_uint = 0x0004;
pub const VFPF_QUEUE_FLG_CACHE_ALIGN: c_uint = 0x0008;
pub const VFPF_QUEUE_FLG_STATS: c_uint = 0x0010;
pub const VFPF_QUEUE_FLG_OV: c_uint = 0x0020;
pub const VFPF_QUEUE_FLG_VLAN: c_uint = 0x0040;
pub const VFPF_QUEUE_FLG_COS: c_uint = 0x0080;
pub const VFPF_QUEUE_FLG_HC: c_uint = 0x0100;
pub const VFPF_QUEUE_FLG_DHC: c_uint = 0x0200;
pub const VFPF_QUEUE_FLG_LEADING_RSS: c_uint = 0x0400;

pub const VFPF_RX_MASK_ACCEPT_NONE: c_uint = 0x00000000;
pub const VFPF_RX_MASK_ACCEPT_MATCHED_UNICAST: c_uint = 0x00000001;
pub const VFPF_RX_MASK_ACCEPT_MATCHED_MULTICAST: c_uint = 0x00000002;
pub const VFPF_RX_MASK_ACCEPT_ALL_UNICAST: c_uint = 0x00000004;
pub const VFPF_RX_MASK_ACCEPT_ALL_MULTICAST: c_uint = 0x00000008;
pub const VFPF_RX_MASK_ACCEPT_BROADCAST: c_uint = 0x00000010;
pub const VFPF_RX_MASK_ACCEPT_ANY_VLAN: c_uint = 0x00000020;

pub const BULLETIN_CRC_SEED: c_int = 0;
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
    pub resp_msg_offset: u32,
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
pub struct pfvf_general_resp_tlv {
    pub hdr: pfvf_tlv,
}

// used to terminate and pad a tlv list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_list_end_tlv {
    pub tl: channel_tlv,
    pub padding: [u8; 4],
}

// Acquire
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_acquire_tlv {
    pub first_tlv: vfpf_first_tlv,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_vfdev_info {
// the following fields are for debug purposes
    pub /: *mut *mut u8 vf_id; / ME register value,
    pub /: *mut *mut u8 vf_os; / e.g. Linux, W2K8,

    pub fp_hsi_ver: u8,
    pub caps: u8,

    pub vfdev_info: },
    pub resc_request: vf_pf_resc_request,
    pub bulletin_addr: aligned_u64,
}

// simple operation request on queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_q_op_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub vf_qid: u8,
    pub padding: [u8; 3],
}

// receive side scaling tlv
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_rss_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub rss_flags: u32,

    pub rss_result_mask: u8,
    pub ind_table_size: u8,
    pub rss_key_size: u8,
    pub padding: u8,
    pub ind_table: [u8; T_ETH_INDIRECTION_TABLE_SIZE],
    pub /: *mut *mut u32 rss_key[T_ETH_RSS_KEY]; / hash values,
}

// acquire response tlv - carries the allocated resources
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_acquire_resp_tlv {
    pub hdr: pfvf_tlv,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_vf_pfdev_info {
    pub chip_num: u32,
    pub pf_cap: u32,
pub const PFVF_CAP_RSS: c_uint = 0x00000001;
pub const PFVF_CAP_DHC: c_uint = 0x00000002;
pub const PFVF_CAP_TPA: c_uint = 0x00000004;
pub const PFVF_CAP_TPA_UPDATE: c_uint = 0x00000008;
pub const PFVF_CAP_VLAN_FILTER: c_uint = 0x00000010;
    pub fw_ver: [c_char; 32],
    pub db_size: u16,
    pub indices_per_sb: u8,
    pub padding: u8,
    pub pfdev_info: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_vf_resc {
// in case of status NO_RESOURCE in message hdr, pf will fill
// this struct with suggested amount of resources for next
// acquire request
//
pub const PFVF_MAX_QUEUES_PER_VF: c_int = 16;
pub const PFVF_MAX_SBS_PER_VF: c_int = 16;
    pub hw_sbs: [hw_sb_info; PFVF_MAX_SBS_PER_VF],
    pub hw_qid: [u8; PFVF_MAX_QUEUES_PER_VF],
    pub num_rxqs: u8,
    pub num_txqs: u8,
    pub num_sbs: u8,
    pub num_mac_filters: u8,
    pub num_vlan_filters: u8,
    pub num_mc_filters: u8,
    pub permanent_mac_addr: [u8; ETH_ALEN],
    pub current_mac_addr: [u8; ETH_ALEN],
    pub padding: [u8; 2],
    pub resc: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_port_phys_id_resp_tlv {
    pub tl: channel_tlv,
    pub id: [u8; ETH_ALEN],
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_fp_hsi_resp_tlv {
    pub tl: channel_tlv,
    pub is_supported: u8,
    pub padding: [u8; 3],
}

// stats will be coalesced on
// the leading RSS queue
//
// Init VF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_init_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub /: *mut *mut aligned_u64 sb_addr[PFVF_MAX_SBS_PER_VF]; / vf_sb based,
    pub spq_addr: aligned_u64,
    pub stats_addr: aligned_u64,
    pub stats_stride: u16,
    pub flags: u32,
    pub padding: [u32; 2],
}

// Setup Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_setup_q_tlv {
    pub first_tlv: vfpf_first_tlv,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_rxq_params {
// physical addresses
    pub rcq_addr: aligned_u64,
    pub rcq_np_addr: aligned_u64,
    pub rxq_addr: aligned_u64,
    pub sge_addr: aligned_u64,
// sb + hc info
    pub /: *mut *mut u8 vf_sb; / index in hw_sbs[],
    pub /: *mut *mut u8 sb_index; / Index in the SB,
    pub /: *mut *mut u16 hc_rate; / desired interrupts per sec.,
// valid iff VFPF_QUEUE_FLG_HC
// rx buffer info
    pub mtu: u16,
    pub buf_sz: u16,
    pub /: *mut *mut u16 flags; / VFPF_QUEUE_FLG_X flags,
    pub /: *mut *mut u16 stat_id; / valid iff VFPF_QUEUE_FLG_STATS,
// valid iff VFPF_QUEUE_FLG_TPA
    pub sge_buf_sz: u16,
    pub tpa_agg_sz: u16,
    pub max_sge_pkt: u8,
    pub VMs: *mut *mut u8 drop_flags; / VFPF_QUEUE_DROP_X, for Linux,
// all the flags are turned off
//
    pub /: *mut *mut u8 cache_line_log; / VFPF_QUEUE_FLG_CACHE_ALIGN,
    pub padding: u8,
    pub rxq: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_txq_params {
// physical addresses
    pub txq_addr: aligned_u64,
// sb + hc info
    pub /: *mut *mut u8 vf_sb; / index in hw_sbs[],
    pub /: *mut *mut u8 sb_index; / Index in the SB,
    pub /: *mut *mut u16 hc_rate; / desired interrupts per sec.,
// valid iff VFPF_QUEUE_FLG_HC
    pub /: *mut *mut u32 flags; / VFPF_QUEUE_FLG_X flags,
    pub /: *mut *mut u16 stat_id; / valid iff VFPF_QUEUE_FLG_STATS,
    pub /: *mut *mut u8 traffic_type; / see in setup_context(),
    pub padding: u8,
    pub txq: },
    pub /: *mut *mut u8 vf_qid; / index in hw_qid[],
    pub param_valid: u8,
pub const VFPF_RXQ_VALID: c_uint = 0x01;
pub const VFPF_TXQ_VALID: c_uint = 0x02;
    pub padding: [u8; 2],
}

// Set Queue Filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_q_mac_vlan_filter {
    pub flags: u32,
pub const VFPF_Q_FILTER_DEST_MAC_VALID: c_uint = 0x01;
pub const VFPF_Q_FILTER_VLAN_TAG_VALID: c_uint = 0x02;
pub const VFPF_Q_FILTER_SET: c_uint = 0x100	/* set/clear */;
    pub mac: [u8; ETH_ALEN],
    pub vlan_tag: u16,
}

// configure queue filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_set_q_filters_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub flags: u32,
pub const VFPF_SET_Q_FILTERS_MAC_VLAN_CHANGED: c_uint = 0x01;
pub const VFPF_SET_Q_FILTERS_MULTICAST_CHANGED: c_uint = 0x02;
pub const VFPF_SET_Q_FILTERS_RX_MASK_CHANGED: c_uint = 0x04;
    pub /: *mut *mut u8 vf_qid; / index in hw_qid[],
    pub n_mac_vlan_filters: u8,
    pub n_multicast: u8,
    pub padding: u8,
pub const PFVF_MAX_MAC_FILTERS: c_int = 16;
pub const PFVF_MAX_VLAN_FILTERS: c_int = 16;
    pub filters: [vfpf_q_mac_vlan_filter; PFVF_MAX_FILTERS],
pub const PFVF_MAX_MULTICAST_PER_VF: c_int = 32;
    pub multicast: [u8; PFVF_MAX_MULTICAST_PER_VF][ETH_ALEN],
    pub /: *mut *mut u32 rx_mask; / see mask constants at the top of the file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_tpa_tlv {
    pub first_tlv: vfpf_first_tlv,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_tpa_client_info {
    pub sge_addr: [aligned_u64; PFVF_MAX_QUEUES_PER_VF],
    pub update_ipv4: u8,
    pub update_ipv6: u8,
    pub max_tpa_queues: u8,
    pub max_sges_for_packet: u8,
    pub complete_on_both_clients: u8,
    pub dont_verify_thr: u8,
    pub tpa_mode: u8,
    pub sge_buff_size: u16,
    pub max_agg_size: u16,
    pub sge_pause_thr_low: u16,
    pub sge_pause_thr_high: u16,
    pub tpa_client_info: },
}

// close VF (disable VF)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_close_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub /: *mut *mut u16 vf_id; / for debug,
    pub padding: [u8; 2],
}

// release the VF's acquired resources
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfpf_release_tlv {
    pub first_tlv: vfpf_first_tlv,
    pub vf_id: u16,
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_buffer_size {
    pub tlv_buffer: [u8; TLV_BUFFER_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vfpf_tlvs {
    pub first_tlv: vfpf_first_tlv,
    pub acquire: vfpf_acquire_tlv,
    pub init: vfpf_init_tlv,
    pub close: vfpf_close_tlv,
    pub q_op: vfpf_q_op_tlv,
    pub setup_q: vfpf_setup_q_tlv,
    pub set_q_filters: vfpf_set_q_filters_tlv,
    pub release: vfpf_release_tlv,
    pub update_rss: vfpf_rss_tlv,
    pub update_tpa: vfpf_tpa_tlv,
    pub list_end: channel_list_end_tlv,
    pub tlv_buf_size: tlv_buffer_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pfvf_tlvs {
    pub general_resp: pfvf_general_resp_tlv,
    pub acquire_resp: pfvf_acquire_resp_tlv,
    pub list_end: channel_list_end_tlv,
    pub tlv_buf_size: tlv_buffer_size,
}

// This is a structure which is allocated in the VF, which the PF may update
// when it deems it necessary to do so. The bulletin board is sampled
// periodically by the VF. A copy per VF is maintained in the PF (to prevent
// loss of data upon multiple updates (or the need for read modify write)).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_vf_bulletin_size {
    pub size: [u8; PF_VF_BULLETIN_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_vf_bulletin_content {
    pub in: *mut *mut u32 crc; / crc of structure to ensure is not,
// mid-update
//
    pub version: u16,
    pub length: u16,
    pub fields: *mut *mut aligned_u64 valid_bitmap; / bitmap indicating which,
// hold valid values
//

// is available for it
//

// the vfpf channel
//

// to attempt to send messages on the
// channel after this bit is set
//

// update is available for it
//
    pub mac: [u8; ETH_ALEN],
    pub mac_padding: [u8; 2],
    pub vlan: u16,
    pub vlan_padding: [u8; 6],
    pub /: *mut *mut u16 link_speed; / Effective line speed,
    pub link_speed_padding: [u8; 6],
    pub /: *mut *mut u32 link_flags; / VFPF_LINK_REPORT_XXX flags,
    pub link_flags_padding: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pf_vf_bulletin {
    pub content: pf_vf_bulletin_content,
    pub size: pf_vf_bulletin_size,
}

pub const MAX_TLVS_IN_LIST: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum channel_tlvs {
    CHANNEL_TLV_NONE,
    CHANNEL_TLV_ACQUIRE,
    CHANNEL_TLV_INIT,
    CHANNEL_TLV_SETUP_Q,
    CHANNEL_TLV_SET_Q_FILTERS,
    CHANNEL_TLV_ACTIVATE_Q,
    CHANNEL_TLV_DEACTIVATE_Q,
    CHANNEL_TLV_TEARDOWN_Q,
    CHANNEL_TLV_CLOSE,
    CHANNEL_TLV_RELEASE,
    CHANNEL_TLV_UPDATE_RSS_DEPRECATED,
    CHANNEL_TLV_PF_RELEASE_VF,
    CHANNEL_TLV_LIST_END,
    CHANNEL_TLV_FLR,
    CHANNEL_TLV_PF_SET_MAC,
    CHANNEL_TLV_PF_SET_VLAN,
    CHANNEL_TLV_UPDATE_RSS,
    CHANNEL_TLV_PHYS_PORT_ID,
    CHANNEL_TLV_UPDATE_TPA,
    CHANNEL_TLV_FP_HSI_SUPPORT,
    CHANNEL_TLV_MAX
}

