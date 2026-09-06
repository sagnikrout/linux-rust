//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_adminq_cmd.h
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

// This header file defines the Admin Queue commands, error codes and
// descriptor format. It is shared between Firmware and Software.
//
pub const ICE_MAX_VSI: c_int = 768;
pub const ICE_AQC_TOPO_MAX_LEVEL_NUM: c_uint = 0x9;
pub const ICE_AQ_SET_MAC_FRAME_SIZE_MAX: c_int = 9728;
pub const ICE_RXQ_CTX_SIZE_DWORDS: c_int = 8;

// The Tx queue context is 40 bytes, and includes some internal state. The
// Admin Queue buffers don't include the internal state, so only include the
// first 22 bytes of the context.
//
pub const ICE_TXQ_CTX_SZ: c_int = 22;
pub const ICE_TXQ_CTX_FULL_SIZE_DWORDS: c_int = 10;

pub const ICE_TXTIME_CTX_SZ: c_int = 25;
// Queue Shutdown (direct 0x0003)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_q_shutdown {
    pub driver_unloading: u8,
    pub reserved: [u8; 15],
}

// Manage MAC address, read command - indirect (0x0107)
// This struct is also used for the response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_manage_mac_read {
    pub /: *mut *mut __le16 flags; / Zeroed by device driver,

pub const ICE_AQC_MAN_MAC_READ_S: c_int = 4;
    pub rsvd: [u8; 2],
    pub /: *mut *mut u8 num_addr; / Used in response,
    pub rsvd1: [u8; 3],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Response buffer format for manage MAC read command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_manage_mac_read_resp {
    pub lport_num: u8,
    pub addr_type: u8,
pub const ICE_AQC_MAN_MAC_ADDR_TYPE_LAN: c_int = 0;
pub const ICE_AQC_MAN_MAC_ADDR_TYPE_WOL: c_int = 1;
    pub mac_addr: [u8; ETH_ALEN],
}

// Manage MAC address, write command - direct (0x0108)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_manage_mac_write {
    pub rsvd: u8,
    pub flags: u8,

pub const ICE_AQC_MAN_MAC_WR_S: c_int = 6;

pub const ICE_AQC_MAN_MAC_UPDATE_LAA: c_int = 0;

// byte stream in network order
    pub mac_addr: [u8; ETH_ALEN],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Clear PXE Command and response (direct 0x0110)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_clear_pxe {
    pub rx_cnt: u8,
pub const ICE_AQC_CLEAR_PXE_RX_CNT: c_uint = 0x2;
    pub reserved: [u8; 15],
}

// Get switch configuration (0x0200)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_sw_cfg {
// Reserved for command and copy of request flags for response
    pub flags: __le16,
// First desc in case of command and next_elem in case of response
// In case of response, if it is not zero, means all the configuration
// was not returned and new command shall be sent with this value in
// the 'first desc' field
//
    pub element: __le16,
// Reserved for command, only used for response
    pub num_elems: __le16,
    pub rsvd: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Each entry in the response buffer is of the following type:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_sw_cfg_resp_elem {
// VSI/Port Number
    pub vsi_port_num: __le16,
pub const ICE_AQC_GET_SW_CONF_RESP_VSI_PORT_NUM_S: c_int = 0;

pub const ICE_AQC_GET_SW_CONF_RESP_TYPE_S: c_int = 14;

pub const ICE_AQC_GET_SW_CONF_RESP_PHYS_PORT: c_int = 0;
pub const ICE_AQC_GET_SW_CONF_RESP_VIRT_PORT: c_int = 1;
pub const ICE_AQC_GET_SW_CONF_RESP_VSI: c_int = 2;
// SWID VSI/Port belongs to
    pub swid: __le16,
// Bit 14..0 : PF/VF number VSI belongs to
// Bit 15 : VF indication bit
//
    pub pf_vf_num: __le16,
pub const ICE_AQC_GET_SW_CONF_RESP_FUNC_NUM_S: c_int = 0;

}

// Loopback port parameter mode values.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_local_fwd_mode {
    ICE_LOCAL_FWD_MODE_ENABLED = 0,
    ICE_LOCAL_FWD_MODE_DISABLED = 1,
    ICE_LOCAL_FWD_MODE_PRIORITIZED = 2,
}

// Set Port parameters, (direct, 0x0203)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_port_params {
    pub cmd_flags: __le16,

    pub bad_frame_vsi: __le16,
    pub swid: __le16,

pub const ICE_AQC_PORT_SWID_M: c_uint = 0xFF;
    pub local_fwd_mode: u8,
    pub reserved: [u8; 9],
}

// These resource type defines are used for all switch resource
// commands where a resource type is required, such as:
// Get Resource Allocation command (indirect 0x0204)
// Allocate Resources command (indirect 0x0208)
// Free Resources command (indirect 0x0209)
// Get Allocated Resource Descriptors Command (indirect 0x020A)
// Share Resource command (indirect 0x020B)
//
pub const ICE_AQC_RES_TYPE_VSI_LIST_REP: c_uint = 0x03;
pub const ICE_AQC_RES_TYPE_VSI_LIST_PRUNE: c_uint = 0x04;
pub const ICE_AQC_RES_TYPE_RECIPE: c_uint = 0x05;
pub const ICE_AQC_RES_TYPE_SWID: c_uint = 0x07;
pub const ICE_AQC_RES_TYPE_FDIR_COUNTER_BLOCK: c_uint = 0x21;
pub const ICE_AQC_RES_TYPE_FDIR_GUARANTEED_ENTRIES: c_uint = 0x22;
pub const ICE_AQC_RES_TYPE_FDIR_SHARED_ENTRIES: c_uint = 0x23;
pub const ICE_AQC_RES_TYPE_FD_PROF_BLDR_PROFID: c_uint = 0x58;
pub const ICE_AQC_RES_TYPE_FD_PROF_BLDR_TCAM: c_uint = 0x59;
pub const ICE_AQC_RES_TYPE_HASH_PROF_BLDR_PROFID: c_uint = 0x60;
pub const ICE_AQC_RES_TYPE_HASH_PROF_BLDR_TCAM: c_uint = 0x61;

pub const ICE_AQC_RES_TYPE_FLAG_DEDICATED: c_uint = 0x00;
pub const ICE_AQC_RES_TYPE_S: c_int = 0;

// Allocate Resources command (indirect 0x0208)
// Free Resources command (indirect 0x0209)
// Share Resource command (indirect 0x020B)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_alloc_free_res_cmd {
    pub /: *mut *mut __le16 num_entries; / Number of Resource entries,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Resource descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_res_elem {
    pub sw_resp: __le16,
    pub flu_resp: __le16,
    pub e: },
}

// Buffer for Allocate/Free Resources commands
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_alloc_free_res_elem {
    pub /: *mut *mut __le16 res_type; / Types defined above cmd 0x0204,
pub const ICE_AQC_RES_TYPE_SHARED_S: c_int = 7;

pub const ICE_AQC_RES_TYPE_VSI_PRUNE_LIST_S: c_int = 8;

    pub num_elems: __le16,
    pub elem: [ice_aqc_res_elem; ],
}

// Request buffer for Set VLAN Mode AQ command (indirect 0x020C)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_vlan_mode {
    pub reserved: u8,
    pub l2tag_prio_tagging: u8,
pub const ICE_AQ_VLAN_PRIO_TAG_S: c_int = 0;

pub const ICE_AQ_VLAN_PRIO_TAG_NOT_SUPPORTED: c_uint = 0x0;
pub const ICE_AQ_VLAN_PRIO_TAG_STAG: c_uint = 0x1;
pub const ICE_AQ_VLAN_PRIO_TAG_OUTER_CTAG: c_uint = 0x2;
pub const ICE_AQ_VLAN_PRIO_TAG_OUTER_VLAN: c_uint = 0x3;
pub const ICE_AQ_VLAN_PRIO_TAG_INNER_CTAG: c_uint = 0x4;
pub const ICE_AQ_VLAN_PRIO_TAG_MAX: c_uint = 0x4;
pub const ICE_AQ_VLAN_PRIO_TAG_ERROR: c_uint = 0x7;
    pub l2tag_reserved: [u8; 64],
    pub rdma_packet: u8,
pub const ICE_AQ_VLAN_RDMA_TAG_S: c_int = 0;

pub const ICE_AQ_SVM_VLAN_RDMA_PKT_FLAG_SETTING: c_uint = 0x10;
pub const ICE_AQ_DVM_VLAN_RDMA_PKT_FLAG_SETTING: c_uint = 0x1A;
    pub rdma_reserved: [u8; 2],
    pub mng_vlan_prot_id: u8,
pub const ICE_AQ_VLAN_MNG_PROTOCOL_ID_OUTER: c_uint = 0x10;
pub const ICE_AQ_VLAN_MNG_PROTOCOL_ID_INNER: c_uint = 0x11;
    pub prot_id_reserved: [u8; 30],
}

// Response buffer for Get VLAN Mode AQ command (indirect 0x020D)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_vlan_mode {
    pub vlan_mode: u8,

    pub l2tag_prio_tagging: u8,
    pub reserved: [u8; 98],
}

// Add VSI (indirect 0x0210)
// Update VSI (indirect 0x0211)
// Get VSI (indirect 0x0212)
// Free VSI (indirect 0x0213)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_get_update_free_vsi {
    pub vsi_num: __le16,
pub const ICE_AQ_VSI_NUM_S: c_int = 0;

    pub cmd_flags: __le16,
pub const ICE_AQ_VSI_KEEP_ALLOC: c_uint = 0x1;
    pub vf_id: u8,
    pub reserved: u8,
    pub vsi_flags: __le16,
pub const ICE_AQ_VSI_TYPE_S: c_int = 0;

pub const ICE_AQ_VSI_TYPE_VF: c_uint = 0x0;
pub const ICE_AQ_VSI_TYPE_VMDQ2: c_uint = 0x1;
pub const ICE_AQ_VSI_TYPE_PF: c_uint = 0x2;
pub const ICE_AQ_VSI_TYPE_EMP_MNG: c_uint = 0x3;
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Response descriptor for:
// Add VSI (indirect 0x0210)
// Update VSI (indirect 0x0211)
// Free VSI (indirect 0x0213)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_update_free_vsi_resp {
    pub vsi_num: __le16,
    pub ext_status: __le16,
    pub vsi_used: __le16,
    pub vsi_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_vsi_props {
    pub valid_sections: __le16,

// switch section
    pub sw_id: u8,
    pub sw_flags: u8,

    pub sw_flags2: u8,
pub const ICE_AQ_VSI_SW_FLAG_RX_PRUNE_EN_S: c_int = 0;

    pub veb_stat_id: u8,
pub const ICE_AQ_VSI_SW_VEB_STAT_ID_S: c_int = 0;

// security section
    pub sec_flags: u8,

pub const ICE_AQ_VSI_SEC_TX_PRUNE_ENA_S: c_int = 4;

    pub sec_reserved: u8,
// VLAN section
    pub /: *mut *mut __le16 port_based_inner_vlan; / VLANS include priority bits,
    pub inner_vlan_reserved: [u8; 2],
    pub inner_vlan_flags: u8,
pub const ICE_AQ_VSI_INNER_VLAN_TX_MODE_S: c_int = 0;

pub const ICE_AQ_VSI_INNER_VLAN_TX_MODE_ACCEPTUNTAGGED: c_uint = 0x1;
pub const ICE_AQ_VSI_INNER_VLAN_TX_MODE_ACCEPTTAGGED: c_uint = 0x2;
pub const ICE_AQ_VSI_INNER_VLAN_TX_MODE_ALL: c_uint = 0x3;

pub const ICE_AQ_VSI_INNER_VLAN_EMODE_S: c_int = 3;

pub const ICE_AQ_VSI_INNER_VLAN_EMODE_STR_BOTH: c_uint = 0x0U;
pub const ICE_AQ_VSI_INNER_VLAN_EMODE_STR_UP: c_uint = 0x1U;
pub const ICE_AQ_VSI_INNER_VLAN_EMODE_STR: c_uint = 0x2U;
pub const ICE_AQ_VSI_INNER_VLAN_EMODE_NOTHING: c_uint = 0x3U;
    pub inner_vlan_reserved2: [u8; 3],
// ingress egress up sections
    pub /: *mut *mut __le32 ingress_table; / bitmap, 3 bits per up,
pub const ICE_AQ_VSI_UP_TABLE_UP0_S: c_int = 0;

pub const ICE_AQ_VSI_UP_TABLE_UP1_S: c_int = 3;

pub const ICE_AQ_VSI_UP_TABLE_UP2_S: c_int = 6;

pub const ICE_AQ_VSI_UP_TABLE_UP3_S: c_int = 9;

pub const ICE_AQ_VSI_UP_TABLE_UP4_S: c_int = 12;

pub const ICE_AQ_VSI_UP_TABLE_UP5_S: c_int = 15;

pub const ICE_AQ_VSI_UP_TABLE_UP6_S: c_int = 18;

pub const ICE_AQ_VSI_UP_TABLE_UP7_S: c_int = 21;

    pub /: *mut *mut __le32 egress_table; / same defines as for ingress table,
// outer tags section
    pub port_based_outer_vlan: __le16,
    pub outer_vlan_flags: u8,
pub const ICE_AQ_VSI_OUTER_VLAN_EMODE_S: c_int = 0;

pub const ICE_AQ_VSI_OUTER_VLAN_EMODE_SHOW_BOTH: c_uint = 0x0;
pub const ICE_AQ_VSI_OUTER_VLAN_EMODE_SHOW_UP: c_uint = 0x1;
pub const ICE_AQ_VSI_OUTER_VLAN_EMODE_SHOW: c_uint = 0x2;
pub const ICE_AQ_VSI_OUTER_VLAN_EMODE_NOTHING: c_uint = 0x3;
pub const ICE_AQ_VSI_OUTER_TAG_TYPE_S: c_int = 2;

pub const ICE_AQ_VSI_OUTER_TAG_NONE: c_uint = 0x0;
pub const ICE_AQ_VSI_OUTER_TAG_STAG: c_uint = 0x1;
pub const ICE_AQ_VSI_OUTER_TAG_VLAN_8100: c_uint = 0x2;
pub const ICE_AQ_VSI_OUTER_TAG_VLAN_9100: c_uint = 0x3;

pub const ICE_AQ_VSI_OUTER_VLAN_TX_MODE_S: c_int = 5;

pub const ICE_AQ_VSI_OUTER_VLAN_TX_MODE_ACCEPTUNTAGGED: c_uint = 0x1;
pub const ICE_AQ_VSI_OUTER_VLAN_TX_MODE_ACCEPTTAGGED: c_uint = 0x2;
pub const ICE_AQ_VSI_OUTER_VLAN_TX_MODE_ALL: c_uint = 0x3;

    pub outer_vlan_reserved: u8,
// queue mapping section
    pub mapping_flags: __le16,
pub const ICE_AQ_VSI_Q_MAP_CONTIG: c_uint = 0x0;
    pub q_mapping: [__le16; 16],
pub const ICE_AQ_VSI_Q_S: c_int = 0;
    pub tc_mapping: [__le16; 8],
pub const ICE_AQ_VSI_TC_Q_OFFSET_S: c_int = 0;

pub const ICE_AQ_VSI_TC_Q_NUM_S: c_int = 11;

// queueing option section
    pub q_opt_rss: u8,
pub const ICE_AQ_VSI_Q_OPT_RSS_LUT_S: c_int = 0;

pub const ICE_AQ_VSI_Q_OPT_RSS_LUT_VSI: c_uint = 0x0;
pub const ICE_AQ_VSI_Q_OPT_RSS_LUT_PF: c_uint = 0x2;
pub const ICE_AQ_VSI_Q_OPT_RSS_LUT_GBL: c_uint = 0x3;
pub const ICE_AQ_VSI_Q_OPT_RSS_GBL_LUT_S: c_int = 2;

pub const ICE_AQ_VSI_Q_OPT_RSS_HASH_S: c_int = 6;

pub const ICE_AQ_VSI_Q_OPT_RSS_HASH_TPLZ: c_uint = 0x0U;
pub const ICE_AQ_VSI_Q_OPT_RSS_HASH_SYM_TPLZ: c_uint = 0x1U;
pub const ICE_AQ_VSI_Q_OPT_RSS_HASH_XOR: c_uint = 0x2U;
pub const ICE_AQ_VSI_Q_OPT_RSS_HASH_JHASH: c_uint = 0x3U;
    pub q_opt_tc: u8,
pub const ICE_AQ_VSI_Q_OPT_TC_OVR_S: c_int = 0;

    pub q_opt_flags: u8,
    pub q_opt_reserved: [u8; 3],
// outer up section
    pub /: *mut *mut __le32 outer_up_table; / same structure and defines as ingress tbl,
// section 10
    pub sect_10_reserved: __le16,
// flow director section
    pub fd_options: __le16,

    pub max_fd_fltr_dedicated: __le16,
    pub max_fd_fltr_shared: __le16,
    pub fd_def_q: __le16,
pub const ICE_AQ_VSI_FD_DEF_Q_S: c_int = 0;

pub const ICE_AQ_VSI_FD_DEF_GRP_S: c_int = 12;

    pub fd_report_opt: __le16,
pub const ICE_AQ_VSI_FD_REPORT_Q_S: c_int = 0;

pub const ICE_AQ_VSI_FD_DEF_PRIORITY_S: c_int = 12;

// PASID section
    pub pasid_id: __le32,
pub const ICE_AQ_VSI_PASID_ID_S: c_int = 0;
    pub reserved: [u8; 24],
}

pub const ICE_MAX_NUM_RECIPES: c_int = 64;
// Add/Get Recipe (indirect 0x0290/0x0292)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_get_recipe {
    pub /: *mut *mut __le16 num_sub_recipes; / Input in Add cmd, Output in Get cmd,
    pub /: *mut *mut __le16 return_index; / Input, used for Get cmd only,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_recipe_content {
    pub rid: u8,
pub const ICE_AQ_RECIPE_ID_S: c_int = 0;

pub const ICE_AQ_SW_ID_LKUP_IDX: c_int = 0;
    pub lkup_indx: [u8; 5],
pub const ICE_AQ_RECIPE_LKUP_DATA_S: c_int = 0;

pub const ICE_AQ_SW_ID_LKUP_MASK: c_uint = 0x00FF;
    pub mask: [__le16; 5],
    pub result_indx: u8,
pub const ICE_AQ_RECIPE_RESULT_DATA_S: c_int = 0;
    pub rsvd0: [u8; 3],
    pub act_ctrl_join_priority: u8,
    pub act_ctrl_fwd_priority: u8,
pub const ICE_AQ_RECIPE_FWD_PRIORITY_S: c_int = 0;

    pub act_ctrl: u8,

pub const ICE_AQ_RECIPE_ACT_PRUNE_INDX_S: c_int = 4;

    pub rsvd1: u8,
    pub dflt_act: __le32,
pub const ICE_AQ_RECIPE_DFLT_ACT_S: c_int = 0;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_recipe_data_elem {
    pub recipe_indx: u8,
    pub resp_bits: u8,
    pub rsvd0: [u8; 2],
    pub recipe_bitmap: [u8; 8],
    pub rsvd1: [u8; 4],
    pub content: ice_aqc_recipe_content,
    pub rsvd2: [u8; 20],
}

// Set/Get Recipes to Profile Association (direct 0x0291/0x0293)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_recipe_to_profile {
    pub profile_id: __le16,
    pub rsvd: [u8; 6],
    pub recipe_assoc: __le64,
}

// Add/Update/Remove/Get switch rules (indirect 0x02A0, 0x02A1, 0x02A2, 0x02A3)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_sw_rules {
// ops: add switch rules, referring the number of rules.
// ops: update switch rules, referring the number of filters
// ops: remove switch rules, referring the entry index.
// ops: get switch rules, referring to the number of filters.
//
    pub num_rules_fltr_entry_index: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Add switch rule response:
// Content of return buffer is same as the input buffer. The status field and
// LUT index are updated as part of the response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_sw_rules_elem_hdr {
    pub /: *mut *mut __le16 type; / Switch rule type, one of T_...,
pub const ICE_AQC_SW_RULES_T_LKUP_RX: c_uint = 0x0;
pub const ICE_AQC_SW_RULES_T_LKUP_TX: c_uint = 0x1;
pub const ICE_AQC_SW_RULES_T_LG_ACT: c_uint = 0x2;
pub const ICE_AQC_SW_RULES_T_VSI_LIST_SET: c_uint = 0x3;
pub const ICE_AQC_SW_RULES_T_VSI_LIST_CLEAR: c_uint = 0x4;
pub const ICE_AQC_SW_RULES_T_PRUNE_LIST_SET: c_uint = 0x5;
pub const ICE_AQC_SW_RULES_T_PRUNE_LIST_CLEAR: c_uint = 0x6;
    pub status: __le16,
    pub __aligned(sizeof(__le16)): } __packed,
// Add/Update/Get/Remove lookup Rx/Tx command/response entry
// This structures describes the lookup rules and associated actions. "index"
// is returned as part of a response to a successful Add command, and can be
// used to identify the rule for Update/Get/Remove commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_rule_lkup_rx_tx {
    pub hdr: ice_aqc_sw_rules_elem_hdr,
    pub recipe_id: __le16,
pub const ICE_SW_RECIPE_LOGICAL_PORT_FWD: c_int = 10;
// Source port for LOOKUP_RX and source VSI in case of LOOKUP_TX
    pub src: __le16,
    pub act: __le32,
// Bit 0:1 - Action type
pub const ICE_SINGLE_ACT_TYPE_S: c_uint = 0x00;

// Bit 2 - Loop back enable
// Bit 3 - LAN enable
//

// Action type = 0 - Forward to VSI or VSI list
pub const ICE_SINGLE_ACT_VSI_FORWARDING: c_uint = 0x0;
pub const ICE_SINGLE_ACT_VSI_ID_S: c_int = 4;

pub const ICE_SINGLE_ACT_VSI_LIST_ID_S: c_int = 4;

// This bit needs to be set if action is forward to VSI list

// Action type = 1 - Forward to Queue of Queue group
pub const ICE_SINGLE_ACT_TO_Q: c_uint = 0x1;
pub const ICE_SINGLE_ACT_Q_INDEX_S: c_int = 4;

pub const ICE_SINGLE_ACT_Q_REGION_S: c_int = 15;

// Action type = 2 - Prune
pub const ICE_SINGLE_ACT_PRUNE: c_uint = 0x2;

// Bit 18 should be set to 0 for this action
// Action type = 2 - Pointer
pub const ICE_SINGLE_ACT_PTR: c_uint = 0x2;
pub const ICE_SINGLE_ACT_PTR_VAL_S: c_int = 4;

// Bit 18 should be set to 1

// Action type = 3 - Other actions. Last two bits
// are other action identifier
//
pub const ICE_SINGLE_ACT_OTHER_ACTS: c_uint = 0x3;
pub const ICE_SINGLE_OTHER_ACT_IDENTIFIER_S: c_int = 17;

// Bit 17:18 - Defines other actions
// Other action = 0 - Mirror VSI
pub const ICE_SINGLE_OTHER_ACT_MIRROR: c_int = 0;
pub const ICE_SINGLE_ACT_MIRROR_VSI_ID_S: c_int = 4;

// Other action = 3 - Set Stat count
pub const ICE_SINGLE_OTHER_ACT_STAT_COUNT: c_int = 3;
pub const ICE_SINGLE_ACT_STAT_COUNT_INDEX_S: c_int = 4;

    pub /: *mut *mut __le16 index; / The index of the rule in the lookup table,
// Length and values of the header to be matched per recipe or
// lookup-type
//
    pub hdr_len: __le16,
    pub hdr_data: [u8; ],
    pub __aligned(sizeof(__le16)): } __packed,
// Add/Update/Remove large action command/response entry
// "index" is returned as part of a response to a successful Add command, and
// can be used to identify the action for Update/Get/Remove commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_rule_lg_act {
    pub hdr: ice_aqc_sw_rules_elem_hdr,
    pub /: *mut *mut __le16 index; / Index in large action table,
    pub size: __le16,
// Max number of large actions
pub const ICE_MAX_LG_ACT: c_int = 4;
// Bit 0:1 - Action type
pub const ICE_LG_ACT_TYPE_S: c_int = 0;

// Action type = 0 - Forward to VSI or VSI list
pub const ICE_LG_ACT_VSI_FORWARDING: c_int = 0;
pub const ICE_LG_ACT_VSI_ID_S: c_int = 3;

pub const ICE_LG_ACT_VSI_LIST_ID_S: c_int = 3;

// This bit needs to be set if action is forward to VSI list

// Action type = 1 - Forward to Queue of Queue group
pub const ICE_LG_ACT_TO_Q: c_uint = 0x1;
pub const ICE_LG_ACT_Q_INDEX_S: c_int = 3;

pub const ICE_LG_ACT_Q_REGION_S: c_int = 14;

// Action type = 2 - Prune
pub const ICE_LG_ACT_PRUNE: c_uint = 0x2;

// Action type = 3 - Mirror VSI
pub const ICE_LG_OTHER_ACT_MIRROR: c_uint = 0x3;
pub const ICE_LG_ACT_MIRROR_VSI_ID_S: c_int = 3;

// Action type = 5 - Generic Value
pub const ICE_LG_ACT_GENERIC: c_uint = 0x5;
pub const ICE_LG_ACT_GENERIC_VALUE_S: c_int = 3;

pub const ICE_LG_ACT_GENERIC_OFFSET_S: c_int = 19;

pub const ICE_LG_ACT_GENERIC_PRIORITY_S: c_int = 22;

pub const ICE_LG_ACT_GENERIC_OFF_RX_DESC_PROF_IDX: c_int = 7;
// Action = 7 - Set Stat count
pub const ICE_LG_ACT_STAT_COUNT: c_uint = 0x7;
pub const ICE_LG_ACT_STAT_COUNT_S: c_int = 3;

    pub /: *mut *mut __le32 act[]; / array of size for actions,
    pub __aligned(sizeof(__le16)): } __packed,
// Add/Update/Remove VSI list command/response entry
// "index" is returned as part of a response to a successful Add command, and
// can be used to identify the VSI list for Update/Get/Remove commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_rule_vsi_list {
    pub hdr: ice_aqc_sw_rules_elem_hdr,
    pub /: *mut *mut __le16 index; / Index of VSI/Prune list,
    pub number_vsi: __le16,
    pub /: *mut *mut __le16 vsi[]; / Array of number_vsi VSI numbers,
    pub __aligned(sizeof(__le16)): } __packed,
// Query PFC Mode (direct 0x0302)
// Set PFC Mode (direct 0x0303)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_query_pfc_mode {
    pub pfc_mode: u8,
// For Query Command response, reserved in all other cases
pub const ICE_AQC_PFC_VLAN_BASED_PFC: c_int = 1;
pub const ICE_AQC_PFC_DSCP_BASED_PFC: c_int = 2;
    pub rsvd: [u8; 15],
}

// Get Default Topology (indirect 0x0400)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_topo {
    pub port_num: u8,
    pub num_branches: u8,
    pub reserved1: __le16,
    pub reserved2: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get/Set Tx Topology (indirect 0x0418/0x0417)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_set_tx_topo {
    pub set_flags: u8,

    pub get_flags: u8,
pub const ICE_AQC_TX_TOPO_GET_RAM: c_int = 2;
    pub reserved1: __le16,
    pub reserved2: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Update TSE (indirect 0x0403)
// Get TSE (indirect 0x0404)
// Add TSE (indirect 0x0401)
// Delete TSE (indirect 0x040F)
// Move TSE (indirect 0x0408)
// Suspend Nodes (indirect 0x0409)
// Resume Nodes (indirect 0x040A)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_sched_elem_cmd {
    pub /: *mut *mut __le16 num_elem_req; / Used by commands,
    pub /: *mut *mut __le16 num_elem_resp; / Used by responses,
    pub reserved: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_txsched_move_grp_info_hdr {
    pub src_parent_teid: __le32,
    pub dest_parent_teid: __le32,
    pub num_elems: __le16,
    pub mode: u8,
pub const ICE_AQC_MOVE_ELEM_MODE_SAME_PF: c_uint = 0x0;
pub const ICE_AQC_MOVE_ELEM_MODE_GIVE_OWN: c_uint = 0x1;
pub const ICE_AQC_MOVE_ELEM_MODE_KEEP_OWN: c_uint = 0x2;
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_move_elem {
    pub hdr: ice_aqc_txsched_move_grp_info_hdr,
    pub teid: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_elem_info_bw {
    pub bw_profile_idx: __le16,
    pub bw_alloc: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_txsched_elem {
    pub /: *mut *mut u8 elem_type; / Special field, reserved for some aq calls,
pub const ICE_AQC_ELEM_TYPE_UNDEFINED: c_uint = 0x0;
pub const ICE_AQC_ELEM_TYPE_ROOT_PORT: c_uint = 0x1;
pub const ICE_AQC_ELEM_TYPE_TC: c_uint = 0x2;
pub const ICE_AQC_ELEM_TYPE_SE_GENERIC: c_uint = 0x3;
pub const ICE_AQC_ELEM_TYPE_ENTRY_POINT: c_uint = 0x4;
pub const ICE_AQC_ELEM_TYPE_LEAF: c_uint = 0x5;
pub const ICE_AQC_ELEM_TYPE_SE_PADDED: c_uint = 0x6;
    pub valid_sections: u8,

    pub generic: u8,
pub const ICE_AQC_ELEM_GENERIC_MODE_M: c_uint = 0x1;
pub const ICE_AQC_ELEM_GENERIC_PRIO_S: c_uint = 0x1;

pub const ICE_AQC_ELEM_GENERIC_SP_S: c_uint = 0x4;

pub const ICE_AQC_ELEM_GENERIC_ADJUST_VAL_S: c_uint = 0x5;

    pub /: *mut *mut u8 flags; / Special field, reserved for some aq calls,
pub const ICE_AQC_ELEM_FLAG_SUSPEND_M: c_uint = 0x1;
    pub cir_bw: ice_aqc_elem_info_bw,
    pub eir_bw: ice_aqc_elem_info_bw,
    pub srl_id: __le16,
    pub reserved2: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_txsched_elem_data {
    pub parent_teid: __le32,
    pub node_teid: __le32,
    pub data: ice_aqc_txsched_elem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_txsched_topo_grp_info_hdr {
    pub parent_teid: __le32,
    pub num_elems: __le16,
    pub reserved2: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_elem {
    pub hdr: ice_aqc_txsched_topo_grp_info_hdr,
    pub generic: [ice_aqc_txsched_elem_data; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_topo_elem {
    pub hdr: ice_aqc_txsched_topo_grp_info_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_delete_elem {
    pub hdr: ice_aqc_txsched_topo_grp_info_hdr,
    pub teid: [__le32; ],
}

// Query Port ETS (indirect 0x040E)
//
// This indirect command is used to query port TC node configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_query_port_ets {
    pub port_teid: __le32,
    pub reserved: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_port_ets_elem {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 3],
// 3 bits for UP per TC 0-7, 4th byte reserved
    pub up2tc: __le32,
    pub tc_bw_share: [u8; 8],
    pub port_eir_prof_id: __le32,
    pub port_cir_prof_id: __le32,
// 3 bits per Node priority to TC 0-7, 4th byte reserved
    pub tc_node_prio: __le32,
pub const ICE_TC_NODE_PRIO_S: c_uint = 0x4;
    pub reserved1: [u8; 4],
    pub /: *mut *mut __le32 tc_node_teid[8]; / Used for response, reserved in command,
}

// Rate limiting profile for
// Add RL profile (indirect 0x0410)
// Query RL profile (indirect 0x0411)
// Remove RL profile (indirect 0x0415)
// These indirect commands acts on single or multiple
// RL profiles with specified data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_rl_profile {
    pub num_profiles: __le16,
    pub /: *mut *mut __le16 num_processed; / Only for response. Reserved in Command.,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_rl_profile_elem {
    pub level: u8,
    pub flags: u8,
pub const ICE_AQC_RL_PROFILE_TYPE_S: c_uint = 0x0;

pub const ICE_AQC_RL_PROFILE_TYPE_CIR: c_int = 0;
pub const ICE_AQC_RL_PROFILE_TYPE_EIR: c_int = 1;
pub const ICE_AQC_RL_PROFILE_TYPE_SRL: c_int = 2;
// The following flag is used for Query RL Profile Data
pub const ICE_AQC_RL_PROFILE_INVAL_S: c_uint = 0x7;

    pub profile_id: __le16,
    pub max_burst_size: __le16,
    pub rl_multiply: __le16,
    pub wake_up_calc: __le16,
    pub rl_encode: __le16,
}

// Query Scheduler Resource Allocation (indirect 0x0412)
// This indirect command retrieves the scheduler resources allocated by
// EMP Firmware to the given PF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_query_txsched_res {
    pub reserved: [u8; 8],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_generic_sched_props {
    pub phys_levels: __le16,
    pub logical_levels: __le16,
    pub flattening_bitmap: u8,
    pub max_device_cgds: u8,
    pub max_pf_cgds: u8,
    pub rsvd0: u8,
    pub rdma_qsets: __le16,
    pub rsvd1: [u8; 22],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_layer_props {
    pub logical_layer: u8,
    pub chunk_size: u8,
    pub max_device_nodes: __le16,
    pub max_pf_nodes: __le16,
    pub rsvd0: [u8; 4],
    pub max_sibl_grp_sz: __le16,
    pub max_cir_rl_profiles: __le16,
    pub max_eir_rl_profiles: __le16,
    pub max_srl_profiles: __le16,
    pub rsvd1: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_query_txsched_res_resp {
    pub sched_props: ice_aqc_generic_sched_props,
    pub layer_props: [ice_aqc_layer_props; ICE_AQC_TOPO_MAX_LEVEL_NUM],
}

// Get PHY capabilities (indirect 0x0600)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_phy_caps {
    pub lport_num: u8,
    pub reserved: u8,
    pub param0: __le16,
// 18.0 - Report qualified modules

// 18.1 - 18.3 : Report mode
// 000b - Report NVM capabilities
// 001b - Report topology capabilities
// 010b - Report SW configured
// 100b - Report default capabilities
//
pub const ICE_AQC_REPORT_MODE_S: c_int = 1;

pub const ICE_AQC_REPORT_TOPO_CAP_NO_MEDIA: c_int = 0;

    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// This is #define of PHY type (Extended):
// The first set of defines is for phy_type_low.
//

pub const ICE_PHY_TYPE_LOW_MAX_INDEX: c_int = 63;
// The second set of defines is for phy_type_high.

pub const ICE_PHY_TYPE_HIGH_MAX_INDEX: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_phy_caps_data {
    pub /: *mut *mut *mut __le64 phy_type_low; / Use values from ICE_PHY_TYPE_LOW_,
    pub /: *mut *mut *mut __le64 phy_type_high; / Use values from ICE_PHY_TYPE_HIGH_,
    pub caps: u8,

    pub low_power_ctrl_an: u8,

    pub eee_cap: __le16,

    pub eeer_value: __le16,
    pub /: *mut *mut u8 phy_id_oui[4]; / PHY/Module ID connected on the port,
    pub phy_fw_ver: [u8; 8],
    pub link_fec_options: u8,

    pub module_compliance_enforcement: u8,

    pub extended_compliance_code: u8,
pub const ICE_MODULE_TYPE_TOTAL_BYTE: c_int = 3;
    pub module_type: [u8; ICE_MODULE_TYPE_TOTAL_BYTE],
pub const ICE_AQC_MOD_TYPE_BYTE0_SFP_PLUS: c_uint = 0xA0;
pub const ICE_AQC_MOD_TYPE_BYTE0_QSFP_PLUS: c_uint = 0x80;
pub const ICE_AQC_MOD_TYPE_IDENT: c_int = 1;

pub const ICE_AQC_MOD_TYPE_BYTE2_SFP_PLUS: c_uint = 0xA0;
pub const ICE_AQC_MOD_TYPE_BYTE2_QSFP_PLUS: c_uint = 0x86;
    pub qualified_module_count: u8,
    pub /: *mut *mut u8 rsvd2[7]; / Bytes 47:41 reserved,
pub const ICE_AQC_QUAL_MOD_COUNT_MAX: c_int = 16;
    pub v_oui: [u8; 3],
    pub rsvd3: u8,
    pub v_part: [u8; 16],
    pub v_rev: __le32,
    pub rsvd4: __le64,
    pub qual_modules: [}; ICE_AQC_QUAL_MOD_COUNT_MAX],
}

// Set PHY capabilities (direct 0x0601)
// NOTE: This command must be followed by setup link and restart auto-neg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_phy_cfg {
    pub lport_num: u8,
    pub reserved: [u8; 7],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Set PHY config command data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_phy_cfg_data {
    pub /: *mut *mut *mut __le64 phy_type_low; / Use values from ICE_PHY_TYPE_LOW_,
    pub /: *mut *mut *mut __le64 phy_type_high; / Use values from ICE_PHY_TYPE_HIGH_,
    pub caps: u8,

    pub low_power_ctrl_an: u8,
    pub /: *mut *mut __le16 eee_cap; / Value from ice_aqc_get_phy_caps,
    pub eeer_value: __le16,
    pub /: *mut *mut u8 link_fec_opt; / Use defines from ice_aqc_get_phy_caps,
    pub module_compliance_enforcement: u8,
}

// Set MAC Config command data structure (direct 0x0603)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_mac_cfg {
    pub max_frame_size: __le16,
    pub params: u8,
pub const ICE_AQ_SET_MAC_PACE_S: c_int = 3;

pub const ICE_AQ_SET_MAC_PACE_TYPE_RATE: c_int = 0;

    pub tx_tmr_priority: u8,
    pub tx_tmr_value: __le16,
    pub fc_refresh_threshold: __le16,
    pub drop_opts: u8,

pub const ICE_AQ_SET_MAC_AUTO_DROP_NONE: c_int = 0;
    pub reserved: [u8; 7],
}

// Restart AN command data structure (direct 0x0605)
// Also used for response, with only the lport_num field present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_restart_an {
    pub lport_num: u8,
    pub reserved: u8,
    pub cmd_flags: u8,

pub const ICE_AQC_RESTART_AN_REFCLK_NOCHANGE: c_int = 0;
    pub reserved2: [u8; 13],
}

// Get link status (indirect 0x0607), also used for Link Status Event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_link_status {
    pub lport_num: u8,
    pub reserved: u8,
    pub cmd_flags: __le16,
pub const ICE_AQ_LSE_M: c_uint = 0x3;
pub const ICE_AQ_LSE_NOP: c_uint = 0x0;
pub const ICE_AQ_LSE_DIS: c_uint = 0x2;
pub const ICE_AQ_LSE_ENA: c_uint = 0x3;
// only response uses this flag
pub const ICE_AQ_LSE_IS_ENABLED: c_uint = 0x1;
    pub reserved2: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get link status response data structure, also used for Link Status Event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_link_status_data {
    pub topo_media_conflict: u8,

    pub link_cfg_err: u8,

    pub link_info: u8,

    pub an_info: u8,

    pub ext_info: u8,

// Port Tx Suspended
pub const ICE_AQ_LINK_TX_S: c_int = 2;

pub const ICE_AQ_LINK_TX_ACTIVE: c_int = 0;
pub const ICE_AQ_LINK_TX_DRAINED: c_int = 1;
pub const ICE_AQ_LINK_TX_FLUSHED: c_int = 3;
    pub reserved2: u8,
    pub max_frame_size: __le16,
    pub cfg: u8,

// Pacing Config
pub const ICE_AQ_CFG_PACING_S: c_int = 3;

pub const ICE_AQ_CFG_PACING_TYPE_AVG: c_int = 0;

// External Device Power Ability
    pub power_desc: u8,
pub const ICE_AQ_PWR_CLASS_M: c_uint = 0x3F;
pub const ICE_AQ_LINK_PWR_BASET_LOW_HIGH: c_int = 0;
pub const ICE_AQ_LINK_PWR_BASET_HIGH: c_int = 1;
pub const ICE_AQ_LINK_PWR_QSFP_CLASS_1: c_int = 0;
pub const ICE_AQ_LINK_PWR_QSFP_CLASS_2: c_int = 1;
pub const ICE_AQ_LINK_PWR_QSFP_CLASS_3: c_int = 2;
pub const ICE_AQ_LINK_PWR_QSFP_CLASS_4: c_int = 3;
    pub link_speed: __le16,

// Aligns next field to 8-byte boundary
    pub reserved3: __le16,
    pub ext_fec_status: u8,
// RS 272 FEC enabled

    pub reserved4: u8,
// Use values from ICE_PHY_TYPE_LOW_*
    pub phy_type_low: __le64,
// Use values from ICE_PHY_TYPE_HIGH_*
    pub phy_type_high: __le64,

// Get link status v2 link partner data
    pub lp_phy_type_low: __le64,
    pub lp_phy_type_high: __le64,
    pub lp_fec_adv: u8,

    pub lp_fec_req: u8,

    pub lp_flowcontrol: u8,
    pub reserved5: [u8; 5],
    pub __packed: },
// Set event mask command (direct 0x0613)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_event_mask {
    pub lport_num: u8,
    pub reserved: [u8; 7],
    pub event_mask: __le16,
    pub reserved1: [u8; 6],
}

// Set MAC Loopback command (direct 0x0620)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_mac_lb {
    pub lb_mode: u8,
    pub reserved: [u8; 15],
}

// Set PHY recovered clock output (direct 0x0630)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_phy_rec_clk_out {
    pub phy_output: u8,
    pub port_num: u8,
pub const ICE_AQC_SET_PHY_REC_CLK_OUT_CURR_PORT: c_uint = 0xFF;
    pub flags: u8,

    pub rsvd: u8,
    pub freq: __le32,
    pub rsvd2: [u8; 6],
    pub node_handle: __le16,
}

// Get PHY recovered clock output (direct 0x0631)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_phy_rec_clk_out {
    pub phy_output: u8,
    pub port_num: u8,
pub const ICE_AQC_GET_PHY_REC_CLK_OUT_CURR_PORT: c_uint = 0xFF;
    pub flags: u8,
    pub rsvd: [u8; 11],
    pub node_handle: __le16,
}

// Get sensor reading (direct 0x0632)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_sensor_reading {
    pub sensor: u8,
    pub format: u8,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get sensor reading response (direct 0x0632)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_sensor_reading_resp {
    pub raw: [u8; 8],
// Output data for sensor 0x00, format 0x00
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _packed {
    pub temp: i8,
    pub temp_warning_threshold: u8,
    pub temp_critical_threshold: u8,
    pub temp_fatal_threshold: u8,
    pub reserved: [u8; 4],
    pub s0f0: },
    pub data: },
}

// DNL call command (indirect 0x0682)
// Struct is used for both command and response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_dnl_call_command {
    pub /: *mut *mut u8 ctx; / Used in command, reserved in response,
    pub reserved: u8,
    pub activity_id: __le16,
pub const ICE_AQC_ACT_ID_DNL: c_uint = 0x1129;
    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_dnl_equa_param {
    pub data_in: __le16,
pub const ICE_AQC_RX_EQU_SHIFT: c_int = 8;

pub const ICE_AQC_TX_EQU_PRE1: c_uint = 0x0;
pub const ICE_AQC_TX_EQU_PRE3: c_uint = 0x3;
pub const ICE_AQC_TX_EQU_ATTEN: c_uint = 0x4;
pub const ICE_AQC_TX_EQU_POST1: c_uint = 0x8;
pub const ICE_AQC_TX_EQU_PRE2: c_uint = 0xC;
    pub op_code_serdes_sel: __le16,
pub const ICE_AQC_OP_CODE_SHIFT: c_int = 4;
    pub reserved: [__le32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_dnl_equa_respon {
// Equalization value can be negative
    pub val: c_int,
    pub reserved: [__le32; 3],
}

// DNL call command/response buffer (indirect 0x0682)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_dnl_call {
    pub txrx_equa_reqs: ice_aqc_dnl_equa_param,
    pub stores: [__le32; 4],
    pub txrx_equa_resp: ice_aqc_dnl_equa_respon,
    pub sto: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_link_topo_params {
    pub lport_num: u8,
    pub lport_num_valid: u8,

    pub node_type_ctx: u8,
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_S: c_int = 0;

pub const ICE_AQC_LINK_TOPO_NODE_TYPE_PHY: c_int = 0;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_GPIO_CTRL: c_int = 1;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_MUX_CTRL: c_int = 2;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_LED_CTRL: c_int = 3;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_LED: c_int = 4;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_THERMAL: c_int = 5;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_CAGE: c_int = 6;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_MEZZ: c_int = 7;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_ID_EEPROM: c_int = 8;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_CLK_CTRL: c_int = 9;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_CLK_MUX: c_int = 10;
pub const ICE_AQC_LINK_TOPO_NODE_TYPE_GPS: c_int = 11;
pub const ICE_AQC_LINK_TOPO_NODE_CTX_S: c_int = 4;

pub const ICE_AQC_LINK_TOPO_NODE_CTX_GLOBAL: c_int = 0;
pub const ICE_AQC_LINK_TOPO_NODE_CTX_BOARD: c_int = 1;
pub const ICE_AQC_LINK_TOPO_NODE_CTX_PORT: c_int = 2;
pub const ICE_AQC_LINK_TOPO_NODE_CTX_NODE: c_int = 3;
pub const ICE_AQC_LINK_TOPO_NODE_CTX_PROVIDED: c_int = 4;
pub const ICE_AQC_LINK_TOPO_NODE_CTX_OVERRIDE: c_int = 5;
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_link_topo_addr {
    pub topo_params: ice_aqc_link_topo_params,
    pub handle: __le16,
pub const ICE_AQC_LINK_TOPO_HANDLE_S: c_int = 0;

// Used to decode the handle field

pub const ICE_AQC_LINK_TOPO_HANDLE_BRD_TYPE_MEZZ: c_int = 0;
pub const ICE_AQC_LINK_TOPO_HANDLE_NODE_S: c_int = 0;
// In case of a Mezzanine type

pub const ICE_AQC_LINK_TOPO_HANDLE_MEZZ_S: c_int = 6;

// In case of a LOM type

}

// Get Link Topology Handle (direct, 0x06E0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_link_topo {
    pub addr: ice_aqc_link_topo_addr,
    pub node_part_num: u8,
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_PCA9575: c_uint = 0x21;
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_ZL30632_80032: c_uint = 0x24;
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_SI5383_5384: c_uint = 0x25;
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_E822_PHY: c_uint = 0x30;
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_C827: c_uint = 0x31;
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_GEN_CLK_MUX: c_uint = 0x47;
pub const ICE_AQC_GET_LINK_TOPO_NODE_NR_GEN_GPS: c_uint = 0x48;
    pub rsvd: [u8; 9],
}

// Read/Write I2C (direct, 0x06E2/0x06E3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_i2c {
    pub topo_addr: ice_aqc_link_topo_addr,
    pub i2c_addr: __le16,
    pub i2c_params: u8,

    pub rsvd: u8,
    pub i2c_bus_addr: __le16,
    pub /: *mut *mut u8 i2c_data[4]; / Used only by write command, reserved in read.,
}

// Read I2C Response (direct, 0x06E2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_read_i2c_resp {
    pub i2c_data: [u8; 16],
}

// Set Port Identification LED (direct, 0x06E9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_port_id_led {
    pub lport_num: u8,
    pub lport_num_valid: u8,
    pub ident_mode: u8,

pub const ICE_AQC_PORT_IDENT_LED_ORIG: c_int = 0;
    pub rsvd: [u8; 13],
}

// Get Port Options (indirect, 0x06EA)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_port_options {
    pub lport_num: u8,
    pub lport_num_valid: u8,
    pub port_options_count: u8,

pub const ICE_AQC_PORT_OPT_MAX: c_int = 16;
    pub innermost_phy_index: u8,
    pub port_options: u8,

    pub pending_port_option_status: u8,
    pub rsvd: [u8; 2],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_port_options_elem {
    pub pmd: u8,

    pub max_lane_speed: u8,

pub const ICE_AQC_PORT_OPT_MAX_LANE_100M: c_int = 0;
pub const ICE_AQC_PORT_OPT_MAX_LANE_1G: c_int = 1;
pub const ICE_AQC_PORT_OPT_MAX_LANE_2500M: c_int = 2;
pub const ICE_AQC_PORT_OPT_MAX_LANE_5G: c_int = 3;
pub const ICE_AQC_PORT_OPT_MAX_LANE_10G: c_int = 4;
pub const ICE_AQC_PORT_OPT_MAX_LANE_25G: c_int = 5;
pub const ICE_AQC_PORT_OPT_MAX_LANE_50G: c_int = 6;
pub const ICE_AQC_PORT_OPT_MAX_LANE_100G: c_int = 7;
pub const ICE_AQC_PORT_OPT_MAX_LANE_200G: c_int = 8;
pub const ICE_AQC_PORT_OPT_MAX_LANE_40G: c_int = 9;
    pub global_scid: [u8; 2],
    pub phy_scid: [u8; 2],
    pub pf2port_cid: [u8; 2],
}

// Set Port Option (direct, 0x06EB)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_port_option {
    pub lport_num: u8,
    pub lport_num_valid: u8,
    pub selected_port_option: u8,
    pub rsvd: [u8; 13],
}

// Set/Get GPIO (direct, 0x06EC/0x06ED)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_gpio {
    pub gpio_ctrl_handle: __le16,
pub const ICE_AQC_GPIO_HANDLE_S: c_int = 0;

    pub gpio_num: u8,
    pub gpio_val: u8,
    pub rsvd: [u8; 12],
}

// Read/Write SFF EEPROM command (indirect 0x06EE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_sff_eeprom {
    pub lport_num: u8,
    pub lport_num_valid: u8,

    pub i2c_bus_addr: __le16,
pub const ICE_AQC_SFF_I2CBUS_7BIT_M: c_uint = 0x7F;
pub const ICE_AQC_SFF_I2CBUS_10BIT_M: c_uint = 0x3FF;

pub const ICE_AQC_SFF_I2CBUS_TYPE_7BIT: c_int = 0;

pub const ICE_AQC_SFF_SET_EEPROM_PAGE_S: c_int = 11;

pub const ICE_AQC_SFF_NO_PAGE_CHANGE: c_int = 0;
pub const ICE_AQC_SFF_SET_23_ON_MISMATCH: c_int = 1;
pub const ICE_AQC_SFF_SET_22_ON_MISMATCH: c_int = 2;

    pub i2c_mem_addr: __le16,
    pub eeprom_page: __le16,
pub const ICE_AQC_SFF_EEPROM_BANK_S: c_int = 0;

pub const ICE_AQC_SFF_EEPROM_PAGE_S: c_int = 8;

    pub addr_high: __le32,
    pub addr_low: __le32,
}

// NVM Read command (indirect 0x0701)
// NVM Erase commands (direct 0x0702)
// NVM Update commands (indirect 0x0703)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_nvm {
pub const ICE_AQC_NVM_MAX_OFFSET: c_uint = 0xFFFFFF;
    pub offset_low: __le16,
    pub offset_high: u8,
    pub cmd_flags: u8,

pub const ICE_AQC_NVM_PRESERVATION_S: c_int = 1;

pub const ICE_AQC_NVM_POR_FLAG: c_int = 0;
pub const ICE_AQC_NVM_PERST_FLAG: c_int = 1;
pub const ICE_AQC_NVM_EMPR_FLAG: c_int = 2;

// For Write Activate, several flags are sent as part of a separate
// flags2 field using a separate byte. For simplicity of the software
// interface, we pass the flags as a 16 bit value so these flags are
// all offset by 8 bits
//

    pub module_typeid: __le16,
    pub length: __le16,
pub const ICE_AQC_NVM_ERASE_LEN: c_uint = 0xFFFF;
    pub addr_high: __le32,
    pub addr_low: __le32,
}

pub const ICE_AQC_NVM_START_POINT: c_int = 0;
pub const ICE_AQC_NVM_SECTOR_UNIT: c_int = 4096;
pub const ICE_AQC_NVM_SDP_AC_PTR_OFFSET: c_uint = 0xD8;

pub const ICE_AQC_NVM_SDP_AC_PTR_INVAL: c_uint = 0x7FFF;

pub const ICE_AQC_NVM_SDP_AC_MAX_SIZE: c_int = 7;
pub const ICE_AQC_NVM_TX_TOPO_MOD_ID: c_uint = 0x14B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_nvm_tx_topo_user_sel {
    pub length: __le16,
    pub data: u8,

    pub reserved: u8,
}

// NVM Checksum Command (direct, 0x0706)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_nvm_checksum {
    pub flags: u8,

    pub rsvd: u8,
    pub /: *mut *mut __le16 checksum; / Used only by response,
pub const ICE_AQC_NVM_CHECKSUM_CORRECT: c_uint = 0xBABA;
    pub rsvd2: [u8; 12],
}

// Used for NVM Set Package Data command - 0x070A
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_nvm_pkg_data {
    pub reserved: [u8; 3],
    pub cmd_flags: u8,

    pub reserved1: u32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Used for Pass Component Table command - 0x070B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_nvm_pass_comp_tbl {
    pub /: *mut *mut u8 component_response; / Response only,
pub const ICE_AQ_NVM_PASS_COMP_CAN_BE_UPDATED: c_uint = 0x0;
pub const ICE_AQ_NVM_PASS_COMP_CAN_MAY_BE_UPDATEABLE: c_uint = 0x1;
pub const ICE_AQ_NVM_PASS_COMP_CAN_NOT_BE_UPDATED: c_uint = 0x2;
pub const ICE_AQ_NVM_PASS_COMP_PARTIAL_CHECK: c_uint = 0x3;
    pub /: *mut *mut u8 component_response_code; / Response only,
pub const ICE_AQ_NVM_PASS_COMP_CAN_BE_UPDATED_CODE: c_uint = 0x0;
pub const ICE_AQ_NVM_PASS_COMP_STAMP_IDENTICAL_CODE: c_uint = 0x1;
pub const ICE_AQ_NVM_PASS_COMP_STAMP_LOWER: c_uint = 0x2;
pub const ICE_AQ_NVM_PASS_COMP_INVALID_STAMP_CODE: c_uint = 0x3;
pub const ICE_AQ_NVM_PASS_COMP_CONFLICT_CODE: c_uint = 0x4;
pub const ICE_AQ_NVM_PASS_COMP_PRE_REQ_NOT_MET_CODE: c_uint = 0x5;
pub const ICE_AQ_NVM_PASS_COMP_NOT_SUPPORTED_CODE: c_uint = 0x6;
pub const ICE_AQ_NVM_PASS_COMP_CANNOT_DOWNGRADE_CODE: c_uint = 0x7;
pub const ICE_AQ_NVM_PASS_COMP_INCOMPLETE_IMAGE_CODE: c_uint = 0x8;
pub const ICE_AQ_NVM_PASS_COMP_VER_STR_IDENTICAL_CODE: c_uint = 0xA;
pub const ICE_AQ_NVM_PASS_COMP_VER_STR_LOWER_CODE: c_uint = 0xB;
    pub reserved: u8,
    pub transfer_flag: u8,
pub const ICE_AQ_NVM_PASS_COMP_TBL_START: c_uint = 0x1;
pub const ICE_AQ_NVM_PASS_COMP_TBL_MIDDLE: c_uint = 0x2;
pub const ICE_AQ_NVM_PASS_COMP_TBL_END: c_uint = 0x4;
pub const ICE_AQ_NVM_PASS_COMP_TBL_START_AND_END: c_uint = 0x5;
    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_nvm_comp_tbl {
    pub comp_class: __le16,
pub const NVM_COMP_CLASS_ALL_FW: c_uint = 0x000A;
    pub comp_id: __le16,
pub const NVM_COMP_ID_OROM: c_uint = 0x5;
pub const NVM_COMP_ID_NVM: c_uint = 0x6;
pub const NVM_COMP_ID_NETLIST: c_uint = 0x8;
    pub comp_class_idx: u8,
pub const FWU_COMP_CLASS_IDX_NOT_USE: c_uint = 0x0;
    pub comp_cmp_stamp: __le32,
    pub cvs_type: u8,
pub const NVM_CVS_TYPE_ASCII: c_uint = 0x1;
    pub cvs_len: u8,
    pub /: *mut *mut u8 cvs[]; / Component Version String,
    pub __packed: },
// Send to PF command (indirect 0x0801) ID is only used by PF
//
// Send to VF command (indirect 0x0802) ID is only used by PF
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_pf_vf_msg {
    pub id: __le32,
    pub reserved: u32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get LLDP MIB (indirect 0x0A00)
// Note: This is also used by the LLDP MIB Change Event (0x0A01)
// as the format is the same.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_get_mib {
    pub type: u8,
pub const ICE_AQ_LLDP_MIB_TYPE_S: c_int = 0;

pub const ICE_AQ_LLDP_MIB_LOCAL: c_int = 0;
pub const ICE_AQ_LLDP_MIB_REMOTE: c_int = 1;
pub const ICE_AQ_LLDP_MIB_LOCAL_AND_REMOTE: c_int = 2;
pub const ICE_AQ_LLDP_BRID_TYPE_S: c_int = 2;

pub const ICE_AQ_LLDP_BRID_TYPE_NEAREST_BRID: c_int = 0;
pub const ICE_AQ_LLDP_BRID_TYPE_NON_TPMR: c_int = 1;
// Tx pause flags in the 0xA01 event use ICE_AQ_LLDP_TX_*
pub const ICE_AQ_LLDP_TX_S: c_uint = 0x4;

pub const ICE_AQ_LLDP_TX_ACTIVE: c_int = 0;
pub const ICE_AQ_LLDP_TX_SUSPENDED: c_int = 1;
pub const ICE_AQ_LLDP_TX_FLUSHED: c_int = 3;
// DCBX mode

pub const ICE_AQ_LLDP_DCBX_NA: c_int = 0;
pub const ICE_AQ_LLDP_DCBX_CEE: c_int = 1;
pub const ICE_AQ_LLDP_DCBX_IEEE: c_int = 2;
    pub state: u8,

pub const ICE_AQ_LLDP_MIB_CHANGE_EXECUTED: c_int = 0;
pub const ICE_AQ_LLDP_MIB_CHANGE_PENDING: c_int = 1;
// The following bytes are reserved for the Get LLDP MIB command (0x0A00)
// and in the LLDP MIB Change Event (0x0A01). They are valid for the
// Get LLDP MIB (0x0A00) response only.
//
    pub local_len: __le16,
    pub remote_len: __le16,
    pub reserved: [u8; 2],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Configure LLDP MIB Change Event (direct 0x0A01)
// For MIB Change Event use ice_aqc_lldp_get_mib structure above
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_set_mib_change {
    pub command: u8,
pub const ICE_AQ_LLDP_MIB_UPDATE_ENABLE: c_uint = 0x0;
pub const ICE_AQ_LLDP_MIB_UPDATE_DIS: c_uint = 0x1;

pub const ICE_AQ_LLDP_MIB_PENDING_DISABLE: c_int = 0;
pub const ICE_AQ_LLDP_MIB_PENDING_ENABLE: c_int = 1;
    pub reserved: [u8; 15],
}

// Stop LLDP (direct 0x0A05)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_stop {
    pub command: u8,

pub const ICE_AQ_LLDP_AGENT_STOP: c_uint = 0x0;
    pub reserved: [u8; 15],
}

// Start LLDP (direct 0x0A06)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_start {
    pub command: u8,
    pub reserved: [u8; 15],
}

// Get CEE DCBX Oper Config (0x0A07)
// The command uses the generic descriptor struct and
// returns the struct below as an indirect response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cee_dcb_cfg_resp {
    pub oper_num_tc: u8,
    pub oper_prio_tc: [u8; 4],
    pub oper_tc_bw: [u8; 8],
    pub oper_pfc_en: u8,
    pub oper_app_prio: __le16,
pub const ICE_AQC_CEE_APP_FCOE_S: c_int = 0;

pub const ICE_AQC_CEE_APP_ISCSI_S: c_int = 3;

pub const ICE_AQC_CEE_APP_FIP_S: c_int = 8;

    pub tlv_status: __le32,
pub const ICE_AQC_CEE_PG_STATUS_S: c_int = 0;

pub const ICE_AQC_CEE_PFC_STATUS_S: c_int = 3;

pub const ICE_AQC_CEE_FCOE_STATUS_S: c_int = 8;

pub const ICE_AQC_CEE_ISCSI_STATUS_S: c_int = 11;

pub const ICE_AQC_CEE_FIP_STATUS_S: c_int = 16;
    pub reserved: [u8; 12],
}

// Set Local LLDP MIB (indirect 0x0A08)
// Used to replace the local MIB of a given LLDP agent. e.g. DCBX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_set_local_mib {
    pub type: u8,

pub const SET_LOCAL_MIB_TYPE_LOCAL_MIB: c_int = 0;

pub const SET_LOCAL_MIB_TYPE_CEE_WILLING: c_int = 0;

    pub reserved0: u8,
    pub length: __le16,
    pub reserved1: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Stop/Start LLDP Agent (direct 0x0A09)
// Used for stopping/starting specific LLDP agent. e.g. DCBX.
// The same structure is used for the response, with the command field
// being used as the status field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_stop_start_specific_agent {
    pub command: u8,

pub const ICE_AQC_START_STOP_AGENT_STOP_DCBX: c_int = 0;
    pub reserved: [u8; 15],
}

// LLDP Filter Control (direct 0x0A0A)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_lldp_filter_ctrl {
    pub cmd_flags: u8,
pub const ICE_AQC_LLDP_FILTER_ACTION_ADD: c_uint = 0x0;
pub const ICE_AQC_LLDP_FILTER_ACTION_DELETE: c_uint = 0x1;
    pub reserved1: u8,
    pub vsi_num: __le16,
    pub reserved2: [u8; 12],
}

// Get/Set RSS key (indirect 0x0B04/0x0B02)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_set_rss_key {
    pub vsi_id: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

pub const ICE_AQC_GET_SET_RSS_KEY_DATA_RSS_KEY_SIZE: c_uint = 0x28;
pub const ICE_AQC_GET_SET_RSS_KEY_DATA_HASH_KEY_SIZE: c_uint = 0xC;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_set_rss_keys {
    pub standard_rss_key: [u8; ICE_AQC_GET_SET_RSS_KEY_DATA_RSS_KEY_SIZE],
    pub extended_hash_key: [u8; ICE_AQC_GET_SET_RSS_KEY_DATA_HASH_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_lut_type {
    ICE_LUT_VSI = 0,
    ICE_LUT_PF = 1,
    ICE_LUT_GLOBAL = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_lut_size {
    ICE_LUT_VSI_SIZE = 64,
    ICE_LUT_GLOBAL_SIZE = 512,
    ICE_LUT_PF_SIZE = 2048,
}

// enum ice_aqc_lut_flags combines constants used to fill
// &ice_aqc_get_set_rss_lut ::flags, which is an amalgamation of global LUT ID,
// LUT size and LUT type, last of which does not need neither shift nor mask.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aqc_lut_flags {
    ICE_AQC_LUT_SIZE_SMALL = 0, /* size = 64 or 128 */
    ICE_AQC_LUT_SIZE_512 = BIT(2),
    ICE_AQC_LUT_SIZE_2K = BIT(3),

    ICE_AQC_LUT_GLOBAL_IDX = GENMASK(7, 4),
}

// Get/Set RSS LUT (indirect 0x0B05/0x0B03)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_set_rss_lut {
    pub vsi_id: __le16,
    pub flags: __le16,
    pub reserved: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Sideband Control Interface Commands
// Neighbor Device Request (indirect 0x0C00); also used for the response.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_neigh_dev_req {
    pub sb_data_len: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Add Tx LAN Queues (indirect 0x0C30)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_txqs {
    pub num_qgrps: u8,
    pub reserved: [u8; 3],
    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// This is the descriptor of each queue entry for the Add Tx LAN Queues
// command (0x0C30). Only used within struct ice_aqc_add_tx_qgrp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_txqs_perq {
    pub txq_id: __le16,
    pub rsvd: [u8; 2],
    pub q_teid: __le32,
    pub txq_ctx: ice_txq_ctx_buf_t,
    pub rsvd2: [u8; 2],
    pub info: ice_aqc_txsched_elem,
    pub __packed: },
// The format of the command buffer for Add Tx LAN Queues (0x0C30)
// is an array of the following structs. Please note that the length of
// each struct ice_aqc_add_tx_qgrp is variable due
// to the variable number of queues in each group!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_tx_qgrp {
    pub parent_teid: __le32,
    pub num_txqs: u8,
    pub rsvd: [u8; 3],
    pub txqs: [ice_aqc_add_txqs_perq; ],
}

// Disable Tx LAN Queues (indirect 0x0C31)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_dis_txqs {
    pub cmd_type: u8,
pub const ICE_AQC_Q_DIS_CMD_S: c_int = 0;

    pub num_entries: u8,
    pub vmvf_and_timeout: __le16,
pub const ICE_AQC_Q_DIS_VMVF_NUM_S: c_int = 0;

pub const ICE_AQC_Q_DIS_TIMEOUT_S: c_int = 10;

    pub blocked_cgds: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// The buffer for Disable Tx LAN Queues (indirect 0x0C31)
// contains the following structures, arrayed one after the
// other.
// Note: Since the q_id is 16 bits wide, if the
// number of queues is even, then 2 bytes of alignment MUST be
// added before the start of the next group, to allow correct
// alignment of the parent_teid field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_dis_txq_item {
    pub parent_teid: __le32,
    pub num_qs: u8,
    pub rsvd: u8,
// The length of the q_id array varies according to num_qs
pub const ICE_AQC_Q_DIS_BUF_ELEM_TYPE_S: c_int = 15;
    pub q_id: [__le16; ],
    pub __packed: },
// Move/Reconfigure Tx queue (indirect 0x0C32)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_cfg_txqs {
    pub cmd_type: u8,
pub const ICE_AQC_Q_CFG_MOVE_NODE: c_uint = 0x1;
pub const ICE_AQC_Q_CFG_TC_CHNG: c_uint = 0x2;
pub const ICE_AQC_Q_CFG_MOVE_TC_CHNG: c_uint = 0x3;

    pub num_qs: u8,
    pub port_num_chng: u8,
pub const ICE_AQC_Q_CFG_SRC_PRT_M: c_uint = 0x7;
pub const ICE_AQC_Q_CFG_DST_PRT_S: c_int = 3;

pub const ICE_AQC_Q_CFG_MODE_SAME_PF: c_uint = 0x0;
pub const ICE_AQC_Q_CFG_MODE_GIVE_OWN: c_uint = 0x1;
pub const ICE_AQC_Q_CFG_MODE_KEEP_OWN: c_uint = 0x2;
    pub time_out: u8,
pub const ICE_AQC_Q_CFG_TIMEOUT_S: c_int = 2;

    pub blocked_cgds: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Per Q struct for Move/Reconfigure Tx LAN Queues (indirect 0x0C32)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_cfg_txq_perq {
    pub q_handle: __le16,
    pub tc: u8,
    pub rsvd: u8,
    pub q_teid: __le32,
}

// The buffer for Move/Reconfigure Tx LAN Queues (indirect 0x0C32)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_cfg_txqs_buf {
    pub src_parent_teid: __le32,
    pub dst_parent_teid: __le32,
    pub queue_info: [ice_aqc_cfg_txq_perq; ],
}

// Add Tx RDMA Queue Set (indirect 0x0C33)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_rdma_qset {
    pub num_qset_grps: u8,
    pub reserved: [u8; 7],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// This is the descriptor of each Qset entry for the Add Tx RDMA Queue Set
// command (0x0C33). Only used within struct ice_aqc_add_rdma_qset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_tx_rdma_qset_entry {
    pub tx_qset_id: __le16,
    pub rsvd: [u8; 2],
    pub qset_teid: __le32,
    pub info: ice_aqc_txsched_elem,
}

// The format of the command buffer for Add Tx RDMA Queue Set(0x0C33)
// is an array of the following structs. Please note that the length of
// each struct ice_aqc_add_rdma_qset is variable due to the variable
// number of queues in each group!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_add_rdma_qset_data {
    pub parent_teid: __le32,
    pub num_qsets: __le16,
    pub rsvd: [u8; 2],
    pub rdma_qsets: [ice_aqc_add_tx_rdma_qset_entry; ],
}

// Set Tx Time LAN Queue (indirect 0x0C35)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_txtimeqs {
    pub q_id: __le16,
    pub q_amount: __le16,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// This is the descriptor of each queue entry for the Set Tx Time Queue
// command (0x0C35). Only used within struct ice_aqc_set_txtime_qgrp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_txtimeqs_perq {
    pub reserved: [u8; 4],
    pub txtime_ctx: ice_txtime_ctx_buf_t,
    pub reserved1: [u8; 3],
}

// The format of the command buffer for Set Tx Time Queue (0x0C35)
// is an array of the following structs. Please note that the length of
// each struct ice_aqc_set_txtime_qgrp is variable due to the variable
// number of queues in each group!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_txtime_qgrp {
    pub reserved: [u8; 8],
    pub txtimeqs: [ice_aqc_set_txtimeqs_perq; ],
}

// Download Package (indirect 0x0C40)
// Also used for Update Package (indirect 0x0C41 and 0x0C42)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_download_pkg {
    pub flags: u8,
pub const ICE_AQC_DOWNLOAD_PKG_LAST_BUF: c_uint = 0x01;
    pub reserved: [u8; 3],
    pub reserved1: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_download_pkg_resp {
    pub error_offset: __le32,
    pub error_info: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get Package Info List (indirect 0x0C43)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_pkg_info_list {
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Version format for packages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pkg_ver {
    pub major: u8,
    pub minor: u8,
    pub update: u8,
    pub draft: u8,
}

pub const ICE_PKG_NAME_SIZE: c_int = 32;
pub const ICE_SEG_ID_SIZE: c_int = 28;
pub const ICE_SEG_NAME_SIZE: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_pkg_info {
    pub ver: ice_pkg_ver,
    pub name: [c_char; ICE_SEG_NAME_SIZE],
    pub track_id: __le32,
    pub is_in_nvm: u8,
    pub is_active: u8,
    pub is_active_at_boot: u8,
    pub is_modified: u8,
}

// Get Package Info List response buffer format (0x0C43)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_pkg_info_resp {
    pub count: __le32,
    pub pkg_info: [ice_aqc_get_pkg_info; ],
}

pub const ICE_CGU_INPUT_PHASE_OFFSET_BYTES: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_cgu_input_measure {
    pub phase_offset: [u8; ICE_CGU_INPUT_PHASE_OFFSET_BYTES],
    pub freq: __le32,
    pub __aligned(sizeof(__le16)): } __packed,

// Get CGU input measure command response data structure (indirect 0x0C59)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_input_measure {
    pub dpll_idx_opt: u8,
    pub length: u8,
    pub rsvd: [u8; 6],
}

// Get CGU abilities command response data structure (indirect 0x0C61)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_abilities {
    pub num_inputs: u8,
    pub num_outputs: u8,
    pub pps_dpll_idx: u8,
    pub eec_dpll_idx: u8,
    pub max_in_freq: __le32,
    pub max_in_phase_adj: __le32,
    pub max_out_freq: __le32,
    pub max_out_phase_adj: __le32,
    pub cgu_part_num: u8,
    pub rsvd: [u8; 3],
}

// Set CGU input config (direct 0x0C62)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_cgu_input_config {
    pub input_idx: u8,
    pub flags1: u8,

    pub flags2: u8,

    pub rsvd: u8,
    pub freq: __le32,
    pub phase_delay: __le32,
    pub rsvd2: [u8; 2],
    pub node_handle: __le16,
}

// Get CGU input config response descriptor structure (direct 0x0C63)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_input_config {
    pub input_idx: u8,
    pub status: u8,

    pub type: u8,

    pub flags1: u8,

    pub freq: __le32,
    pub phase_delay: __le32,
    pub flags2: u8,
    pub rsvd: [u8; 1],
    pub node_handle: __le16,
}

// Set CGU output config (direct 0x0C64)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_cgu_output_config {
    pub output_idx: u8,
    pub flags: u8,

    pub src_sel: u8,

    pub rsvd: u8,
    pub freq: __le32,
    pub phase_delay: __le32,
    pub rsvd2: [u8; 2],
    pub node_handle: __le16,
}

// Get CGU output config (direct 0x0C65)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_output_config {
    pub output_idx: u8,
    pub flags: u8,

    pub src_sel: u8,
pub const ICE_AQC_GET_CGU_OUT_CFG_DPLL_SRC_SEL_SHIFT: c_int = 0;

pub const ICE_AQC_GET_CGU_OUT_CFG_DPLL_MODE_SHIFT: c_int = 5;

    pub rsvd: u8,
    pub freq: __le32,
    pub src_freq: __le32,
    pub rsvd2: [u8; 2],
    pub node_handle: __le16,
}

// Get CGU DPLL status (direct 0x0C66)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_dpll_status {
    pub dpll_num: u8,
    pub ref_state: u8,

    pub dpll_state: u8,

    pub config: u8,

pub const ICE_AQC_GET_CGU_DPLL_CONFIG_MODE_SHIFT: c_int = 5;

pub const ICE_AQC_GET_CGU_DPLL_CONFIG_MODE_FREERUN: c_int = 0;

    pub phase_offset_h: __le32,
    pub phase_offset_l: __le32,
    pub eec_mode: u8,
pub const ICE_AQC_GET_CGU_DPLL_STATUS_EEC_MODE_1: c_uint = 0xA;
pub const ICE_AQC_GET_CGU_DPLL_STATUS_EEC_MODE_2: c_uint = 0xB;
pub const ICE_AQC_GET_CGU_DPLL_STATUS_EEC_MODE_UNKNOWN: c_uint = 0xF;
    pub rsvd: [u8; 1],
    pub node_handle: __le16,
}

// Set CGU DPLL config (direct 0x0C67)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_cgu_dpll_config {
    pub dpll_num: u8,
    pub ref_state: u8,

    pub rsvd: u8,
    pub config: u8,

pub const ICE_AQC_SET_CGU_DPLL_CONFIG_MODE_SHIFT: c_int = 5;

pub const ICE_AQC_SET_CGU_DPLL_CONFIG_MODE_FREERUN: c_int = 0;
    pub rsvd2: [u8; 8],
    pub eec_mode: u8,
    pub rsvd3: [u8; 1],
    pub node_handle: __le16,
}

// Set CGU reference priority (direct 0x0C68)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_cgu_ref_prio {
    pub dpll_num: u8,
    pub ref_idx: u8,
    pub ref_priority: u8,
    pub rsvd: [u8; 11],
    pub node_handle: __le16,
}

// Get CGU reference priority (direct 0x0C69)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_ref_prio {
    pub dpll_num: u8,
    pub ref_idx: u8,
    pub /: *mut *mut u8 ref_priority; / Valid only in response,
    pub rsvd: [u8; 13],
}

// Get CGU info (direct 0x0C6A)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_cgu_info {
    pub cgu_id: __le32,
    pub cgu_cfg_ver: __le32,
    pub cgu_fw_ver: __le32,
    pub node_part_num: u8,
    pub dev_rev: u8,
    pub node_handle: __le16,
}

// Driver Shared Parameters (direct, 0x0C90)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_driver_shared_params {
    pub set_or_get_op: u8,

pub const ICE_AQC_DRIVER_PARAM_SET: c_int = 0;
pub const ICE_AQC_DRIVER_PARAM_GET: c_int = 1;
    pub param_indx: u8,
pub const ICE_AQC_DRIVER_PARAM_MAX_IDX: c_int = 15;
    pub rsvd: [u8; 2],
    pub param_val: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Lan Queue Overflow Event (direct, 0x1001)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_event_lan_overflow {
    pub prtdcb_ruptq: __le32,
    pub qtx_ctl: __le32,
    pub reserved: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aqc_health_status_mask {
    ICE_AQC_HEALTH_STATUS_SET_PF_SPECIFIC_MASK = BIT(0),
    ICE_AQC_HEALTH_STATUS_SET_ALL_PF_MASK      = BIT(1),
    ICE_AQC_HEALTH_STATUS_SET_GLOBAL_MASK      = BIT(2),
}

// Set Health Status (direct 0xFF20)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_set_health_status_cfg {
    pub event_source: u8,
    pub reserved: [u8; 15],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aqc_health_status {
    ICE_AQC_HEALTH_STATUS_ERR_UNKNOWN_MOD_STRICT		= 0x101,
    ICE_AQC_HEALTH_STATUS_ERR_MOD_TYPE			= 0x102,
    ICE_AQC_HEALTH_STATUS_ERR_MOD_QUAL			= 0x103,
    ICE_AQC_HEALTH_STATUS_ERR_MOD_COMM			= 0x104,
    ICE_AQC_HEALTH_STATUS_ERR_MOD_CONFLICT			= 0x105,
    ICE_AQC_HEALTH_STATUS_ERR_MOD_NOT_PRESENT		= 0x106,
    ICE_AQC_HEALTH_STATUS_INFO_MOD_UNDERUTILIZED		= 0x107,
    ICE_AQC_HEALTH_STATUS_ERR_UNKNOWN_MOD_LENIENT		= 0x108,
    ICE_AQC_HEALTH_STATUS_ERR_MOD_DIAGNOSTIC_FEATURE	= 0x109,
    ICE_AQC_HEALTH_STATUS_ERR_INVALID_LINK_CFG		= 0x10B,
    ICE_AQC_HEALTH_STATUS_ERR_PORT_ACCESS			= 0x10C,
    ICE_AQC_HEALTH_STATUS_ERR_PORT_UNREACHABLE		= 0x10D,
    ICE_AQC_HEALTH_STATUS_INFO_PORT_SPEED_MOD_LIMITED	= 0x10F,
    ICE_AQC_HEALTH_STATUS_ERR_PARALLEL_FAULT		= 0x110,
    ICE_AQC_HEALTH_STATUS_INFO_PORT_SPEED_PHY_LIMITED	= 0x111,
    ICE_AQC_HEALTH_STATUS_ERR_NETLIST_TOPO			= 0x112,
    ICE_AQC_HEALTH_STATUS_ERR_NETLIST			= 0x113,
    ICE_AQC_HEALTH_STATUS_ERR_TOPO_CONFLICT			= 0x114,
    ICE_AQC_HEALTH_STATUS_ERR_LINK_HW_ACCESS		= 0x115,
    ICE_AQC_HEALTH_STATUS_ERR_LINK_RUNTIME			= 0x116,
    ICE_AQC_HEALTH_STATUS_ERR_DNL_INIT			= 0x117,
    ICE_AQC_HEALTH_STATUS_ERR_PHY_NVM_PROG			= 0x120,
    ICE_AQC_HEALTH_STATUS_ERR_PHY_FW_LOAD			= 0x121,
    ICE_AQC_HEALTH_STATUS_INFO_RECOVERY			= 0x500,
    ICE_AQC_HEALTH_STATUS_ERR_FLASH_ACCESS			= 0x501,
    ICE_AQC_HEALTH_STATUS_ERR_NVM_AUTH			= 0x502,
    ICE_AQC_HEALTH_STATUS_ERR_OROM_AUTH			= 0x503,
    ICE_AQC_HEALTH_STATUS_ERR_DDP_AUTH			= 0x504,
    ICE_AQC_HEALTH_STATUS_ERR_NVM_COMPAT			= 0x505,
    ICE_AQC_HEALTH_STATUS_ERR_OROM_COMPAT			= 0x506,
    ICE_AQC_HEALTH_STATUS_ERR_NVM_SEC_VIOLATION		= 0x507,
    ICE_AQC_HEALTH_STATUS_ERR_OROM_SEC_VIOLATION		= 0x508,
    ICE_AQC_HEALTH_STATUS_ERR_DCB_MIB			= 0x509,
    ICE_AQC_HEALTH_STATUS_ERR_MNG_TIMEOUT			= 0x50A,
    ICE_AQC_HEALTH_STATUS_ERR_BMC_RESET			= 0x50B,
    ICE_AQC_HEALTH_STATUS_ERR_LAST_MNG_FAIL			= 0x50C,
    ICE_AQC_HEALTH_STATUS_ERR_RESOURCE_ALLOC_FAIL		= 0x50D,
    ICE_AQC_HEALTH_STATUS_ERR_FW_LOOP			= 0x1000,
    ICE_AQC_HEALTH_STATUS_ERR_FW_PFR_FAIL			= 0x1001,
    ICE_AQC_HEALTH_STATUS_ERR_LAST_FAIL_AQ			= 0x1002,
}

// Get Health Status (indirect 0xFF22)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_get_health_status {
    pub health_status_count: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aqc_health_status_scope {
    ICE_AQC_HEALTH_STATUS_PF	= 0x1,
    ICE_AQC_HEALTH_STATUS_PORT	= 0x2,
    ICE_AQC_HEALTH_STATUS_GLOBAL	= 0x3,
}

pub const ICE_AQC_HEALTH_STATUS_UNDEFINED_DATA: c_uint = 0xDEADBEEF;
// Get Health Status event buffer entry (0xFF22),
// repeated per reported health status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_health_status_elem {
    pub health_status_code: __le16,
    pub event_source: __le16,
    pub internal_data1: __le32,
    pub internal_data2: __le32,
}

// Admin Queue command opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_adminq_opc {
// AQ commands
    ice_aqc_opc_get_ver				= 0x0001,
    ice_aqc_opc_driver_ver				= 0x0002,
    ice_aqc_opc_q_shutdown				= 0x0003,

// resource ownership
    ice_aqc_opc_req_res				= 0x0008,
    ice_aqc_opc_release_res				= 0x0009,

// device/function capabilities
    ice_aqc_opc_list_func_caps			= 0x000A,
    ice_aqc_opc_list_dev_caps			= 0x000B,

// manage MAC address
    ice_aqc_opc_manage_mac_read			= 0x0107,
    ice_aqc_opc_manage_mac_write			= 0x0108,

// PXE
    ice_aqc_opc_clear_pxe_mode			= 0x0110,

// internal switch commands
    ice_aqc_opc_get_sw_cfg				= 0x0200,
    ice_aqc_opc_set_port_params			= 0x0203,

// Alloc/Free/Get Resources
    ice_aqc_opc_alloc_res				= 0x0208,
    ice_aqc_opc_free_res				= 0x0209,
    ice_aqc_opc_share_res				= 0x020B,
    ice_aqc_opc_set_vlan_mode_parameters		= 0x020C,
    ice_aqc_opc_get_vlan_mode_parameters		= 0x020D,

// VSI commands
    ice_aqc_opc_add_vsi				= 0x0210,
    ice_aqc_opc_update_vsi				= 0x0211,
    ice_aqc_opc_free_vsi				= 0x0213,

// recipe commands
    ice_aqc_opc_add_recipe				= 0x0290,
    ice_aqc_opc_recipe_to_profile			= 0x0291,
    ice_aqc_opc_get_recipe				= 0x0292,
    ice_aqc_opc_get_recipe_to_profile		= 0x0293,

// switch rules population commands
    ice_aqc_opc_add_sw_rules			= 0x02A0,
    ice_aqc_opc_update_sw_rules			= 0x02A1,
    ice_aqc_opc_remove_sw_rules			= 0x02A2,

    ice_aqc_opc_clear_pf_cfg			= 0x02A4,

// DCB commands
    ice_aqc_opc_query_pfc_mode			= 0x0302,
    ice_aqc_opc_set_pfc_mode			= 0x0303,

// transmit scheduler commands
    ice_aqc_opc_get_dflt_topo			= 0x0400,
    ice_aqc_opc_add_sched_elems			= 0x0401,
    ice_aqc_opc_cfg_sched_elems			= 0x0403,
    ice_aqc_opc_get_sched_elems			= 0x0404,
    ice_aqc_opc_move_sched_elems			= 0x0408,
    ice_aqc_opc_suspend_sched_elems			= 0x0409,
    ice_aqc_opc_resume_sched_elems			= 0x040A,
    ice_aqc_opc_query_port_ets			= 0x040E,
    ice_aqc_opc_delete_sched_elems			= 0x040F,
    ice_aqc_opc_add_rl_profiles			= 0x0410,
    ice_aqc_opc_query_sched_res			= 0x0412,
    ice_aqc_opc_remove_rl_profiles			= 0x0415,

// tx topology commands
    ice_aqc_opc_set_tx_topo				= 0x0417,
    ice_aqc_opc_get_tx_topo				= 0x0418,

// PHY commands
    ice_aqc_opc_get_phy_caps			= 0x0600,
    ice_aqc_opc_set_phy_cfg				= 0x0601,
    ice_aqc_opc_set_mac_cfg				= 0x0603,
    ice_aqc_opc_restart_an				= 0x0605,
    ice_aqc_opc_get_link_status			= 0x0607,
    ice_aqc_opc_set_event_mask			= 0x0613,
    ice_aqc_opc_set_mac_lb				= 0x0620,
    ice_aqc_opc_set_phy_rec_clk_out			= 0x0630,
    ice_aqc_opc_get_phy_rec_clk_out			= 0x0631,
    ice_aqc_opc_get_sensor_reading			= 0x0632,
    ice_aqc_opc_dnl_call                            = 0x0682,
    ice_aqc_opc_get_link_topo			= 0x06E0,
    ice_aqc_opc_read_i2c				= 0x06E2,
    ice_aqc_opc_write_i2c				= 0x06E3,
    ice_aqc_opc_set_port_id_led			= 0x06E9,
    ice_aqc_opc_get_port_options			= 0x06EA,
    ice_aqc_opc_set_port_option			= 0x06EB,
    ice_aqc_opc_set_gpio				= 0x06EC,
    ice_aqc_opc_get_gpio				= 0x06ED,
    ice_aqc_opc_sff_eeprom				= 0x06EE,

// NVM commands
    ice_aqc_opc_nvm_read				= 0x0701,
    ice_aqc_opc_nvm_erase				= 0x0702,
    ice_aqc_opc_nvm_write				= 0x0703,
    ice_aqc_opc_nvm_checksum			= 0x0706,
    ice_aqc_opc_nvm_write_activate			= 0x0707,
    ice_aqc_opc_nvm_update_empr			= 0x0709,
    ice_aqc_opc_nvm_pkg_data			= 0x070A,
    ice_aqc_opc_nvm_pass_component_tbl		= 0x070B,

// PF/VF mailbox commands
    ice_mbx_opc_send_msg_to_pf			= 0x0801,
    ice_mbx_opc_send_msg_to_vf			= 0x0802,
// LLDP commands
    ice_aqc_opc_lldp_get_mib			= 0x0A00,
    ice_aqc_opc_lldp_set_mib_change			= 0x0A01,
    ice_aqc_opc_lldp_stop				= 0x0A05,
    ice_aqc_opc_lldp_start				= 0x0A06,
    ice_aqc_opc_get_cee_dcb_cfg			= 0x0A07,
    ice_aqc_opc_lldp_set_local_mib			= 0x0A08,
    ice_aqc_opc_lldp_stop_start_specific_agent	= 0x0A09,
    ice_aqc_opc_lldp_filter_ctrl			= 0x0A0A,
    ice_aqc_opc_lldp_execute_pending_mib		= 0x0A0B,

// RSS commands
    ice_aqc_opc_set_rss_key				= 0x0B02,
    ice_aqc_opc_set_rss_lut				= 0x0B03,
    ice_aqc_opc_get_rss_key				= 0x0B04,
    ice_aqc_opc_get_rss_lut				= 0x0B05,

// Sideband Control Interface commands
    ice_aqc_opc_neighbour_device_request		= 0x0C00,

// Tx queue handling commands/events
    ice_aqc_opc_add_txqs				= 0x0C30,
    ice_aqc_opc_dis_txqs				= 0x0C31,
    ice_aqc_opc_cfg_txqs				= 0x0C32,
    ice_aqc_opc_add_rdma_qset			= 0x0C33,

// Tx Time queue commands
    ice_aqc_opc_set_txtimeqs			= 0x0C35,

// package commands
    ice_aqc_opc_download_pkg			= 0x0C40,
    ice_aqc_opc_upload_section			= 0x0C41,
    ice_aqc_opc_update_pkg				= 0x0C42,
    ice_aqc_opc_get_pkg_info_list			= 0x0C43,

// 1588/SyncE commands/events
    ice_aqc_opc_get_cgu_input_measure		= 0x0C59,
    ice_aqc_opc_get_cgu_abilities			= 0x0C61,
    ice_aqc_opc_set_cgu_input_config		= 0x0C62,
    ice_aqc_opc_get_cgu_input_config		= 0x0C63,
    ice_aqc_opc_set_cgu_output_config		= 0x0C64,
    ice_aqc_opc_get_cgu_output_config		= 0x0C65,
    ice_aqc_opc_get_cgu_dpll_status			= 0x0C66,
    ice_aqc_opc_set_cgu_dpll_config			= 0x0C67,
    ice_aqc_opc_set_cgu_ref_prio			= 0x0C68,
    ice_aqc_opc_get_cgu_ref_prio			= 0x0C69,
    ice_aqc_opc_get_cgu_info			= 0x0C6A,

    ice_aqc_opc_driver_shared_params		= 0x0C90,

// Standalone Commands/Events
    ice_aqc_opc_event_lan_overflow			= 0x1001,

// System Diagnostic commands
    ice_aqc_opc_set_health_status_cfg		= 0xFF20,
    ice_aqc_opc_get_health_status			= 0xFF22,

// FW Logging Commands
    ice_aqc_opc_fw_logs_config			= 0xFF30,
    ice_aqc_opc_fw_logs_register			= 0xFF31,
    ice_aqc_opc_fw_logs_query			= 0xFF32,
    ice_aqc_opc_fw_logs_event			= 0xFF33,
}
