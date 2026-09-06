//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_adminq_cmd.h
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

// This header file defines the iavf Admin Queue commands and is shared between
// iavf Firmware and Software.
//
// This file needs to comply with the Linux Kernel coding style.
//
pub const IAVF_FW_API_VERSION_MAJOR: c_uint = 0x0001;
pub const IAVF_FW_API_VERSION_MINOR_X722: c_uint = 0x0005;
pub const IAVF_FW_API_VERSION_MINOR_X710: c_uint = 0x0008;

// API version 1.7 implements additional link and PHY-specific APIs
pub const IAVF_MINOR_VER_GET_LINK_INFO_XL710: c_uint = 0x0007;
// Admin Queue command opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_admin_queue_opc {
// aq commands
    iavf_aqc_opc_get_version	= 0x0001,
    iavf_aqc_opc_driver_version	= 0x0002,
    iavf_aqc_opc_queue_shutdown	= 0x0003,
    iavf_aqc_opc_set_pf_context	= 0x0004,

// resource ownership
    iavf_aqc_opc_request_resource	= 0x0008,
    iavf_aqc_opc_release_resource	= 0x0009,

    iavf_aqc_opc_list_func_capabilities	= 0x000A,
    iavf_aqc_opc_list_dev_capabilities	= 0x000B,

// Proxy commands
    iavf_aqc_opc_set_proxy_config		= 0x0104,
    iavf_aqc_opc_set_ns_proxy_table_entry	= 0x0105,

// LAA
    iavf_aqc_opc_mac_address_read	= 0x0107,
    iavf_aqc_opc_mac_address_write	= 0x0108,

// PXE
    iavf_aqc_opc_clear_pxe_mode	= 0x0110,

// WoL commands
    iavf_aqc_opc_set_wol_filter	= 0x0120,
    iavf_aqc_opc_get_wake_reason	= 0x0121,

// internal switch commands
    iavf_aqc_opc_get_switch_config		= 0x0200,
    iavf_aqc_opc_add_statistics		= 0x0201,
    iavf_aqc_opc_remove_statistics		= 0x0202,
    iavf_aqc_opc_set_port_parameters	= 0x0203,
    iavf_aqc_opc_get_switch_resource_alloc	= 0x0204,
    iavf_aqc_opc_set_switch_config		= 0x0205,
    iavf_aqc_opc_rx_ctl_reg_read		= 0x0206,
    iavf_aqc_opc_rx_ctl_reg_write		= 0x0207,

    iavf_aqc_opc_add_vsi			= 0x0210,
    iavf_aqc_opc_update_vsi_parameters	= 0x0211,
    iavf_aqc_opc_get_vsi_parameters		= 0x0212,

    iavf_aqc_opc_add_pv			= 0x0220,
    iavf_aqc_opc_update_pv_parameters	= 0x0221,
    iavf_aqc_opc_get_pv_parameters		= 0x0222,

    iavf_aqc_opc_add_veb			= 0x0230,
    iavf_aqc_opc_update_veb_parameters	= 0x0231,
    iavf_aqc_opc_get_veb_parameters		= 0x0232,

    iavf_aqc_opc_delete_element		= 0x0243,

    iavf_aqc_opc_add_macvlan		= 0x0250,
    iavf_aqc_opc_remove_macvlan		= 0x0251,
    iavf_aqc_opc_add_vlan			= 0x0252,
    iavf_aqc_opc_remove_vlan		= 0x0253,
    iavf_aqc_opc_set_vsi_promiscuous_modes	= 0x0254,
    iavf_aqc_opc_add_tag			= 0x0255,
    iavf_aqc_opc_remove_tag			= 0x0256,
    iavf_aqc_opc_add_multicast_etag		= 0x0257,
    iavf_aqc_opc_remove_multicast_etag	= 0x0258,
    iavf_aqc_opc_update_tag			= 0x0259,
    iavf_aqc_opc_add_control_packet_filter	= 0x025A,
    iavf_aqc_opc_remove_control_packet_filter	= 0x025B,
    iavf_aqc_opc_add_cloud_filters		= 0x025C,
    iavf_aqc_opc_remove_cloud_filters	= 0x025D,
    iavf_aqc_opc_clear_wol_switch_filters	= 0x025E,

    iavf_aqc_opc_add_mirror_rule	= 0x0260,
    iavf_aqc_opc_delete_mirror_rule	= 0x0261,

// Dynamic Device Personalization
    iavf_aqc_opc_write_personalization_profile	= 0x0270,
    iavf_aqc_opc_get_personalization_profile_list	= 0x0271,

// DCB commands
    iavf_aqc_opc_dcb_ignore_pfc	= 0x0301,
    iavf_aqc_opc_dcb_updated	= 0x0302,
    iavf_aqc_opc_set_dcb_parameters = 0x0303,

// TX scheduler
    iavf_aqc_opc_configure_vsi_bw_limit		= 0x0400,
    iavf_aqc_opc_configure_vsi_ets_sla_bw_limit	= 0x0406,
    iavf_aqc_opc_configure_vsi_tc_bw		= 0x0407,
    iavf_aqc_opc_query_vsi_bw_config		= 0x0408,
    iavf_aqc_opc_query_vsi_ets_sla_config		= 0x040A,
    iavf_aqc_opc_configure_switching_comp_bw_limit	= 0x0410,

    iavf_aqc_opc_enable_switching_comp_ets			= 0x0413,
    iavf_aqc_opc_modify_switching_comp_ets			= 0x0414,
    iavf_aqc_opc_disable_switching_comp_ets			= 0x0415,
    iavf_aqc_opc_configure_switching_comp_ets_bw_limit	= 0x0416,
    iavf_aqc_opc_configure_switching_comp_bw_config		= 0x0417,
    iavf_aqc_opc_query_switching_comp_ets_config		= 0x0418,
    iavf_aqc_opc_query_port_ets_config			= 0x0419,
    iavf_aqc_opc_query_switching_comp_bw_config		= 0x041A,
    iavf_aqc_opc_suspend_port_tx				= 0x041B,
    iavf_aqc_opc_resume_port_tx				= 0x041C,
    iavf_aqc_opc_configure_partition_bw			= 0x041D,
// hmc
    iavf_aqc_opc_query_hmc_resource_profile	= 0x0500,
    iavf_aqc_opc_set_hmc_resource_profile	= 0x0501,

// phy commands
    iavf_aqc_opc_get_phy_abilities		= 0x0600,
    iavf_aqc_opc_set_phy_config		= 0x0601,
    iavf_aqc_opc_set_mac_config		= 0x0603,
    iavf_aqc_opc_set_link_restart_an	= 0x0605,
    iavf_aqc_opc_get_link_status		= 0x0607,
    iavf_aqc_opc_set_phy_int_mask		= 0x0613,
    iavf_aqc_opc_get_local_advt_reg		= 0x0614,
    iavf_aqc_opc_set_local_advt_reg		= 0x0615,
    iavf_aqc_opc_get_partner_advt		= 0x0616,
    iavf_aqc_opc_set_lb_modes		= 0x0618,
    iavf_aqc_opc_get_phy_wol_caps		= 0x0621,
    iavf_aqc_opc_set_phy_debug		= 0x0622,
    iavf_aqc_opc_upload_ext_phy_fm		= 0x0625,
    iavf_aqc_opc_run_phy_activity		= 0x0626,
    iavf_aqc_opc_set_phy_register		= 0x0628,
    iavf_aqc_opc_get_phy_register		= 0x0629,

// NVM commands
    iavf_aqc_opc_nvm_read			= 0x0701,
    iavf_aqc_opc_nvm_erase			= 0x0702,
    iavf_aqc_opc_nvm_update			= 0x0703,
    iavf_aqc_opc_nvm_config_read		= 0x0704,
    iavf_aqc_opc_nvm_config_write		= 0x0705,
    iavf_aqc_opc_oem_post_update		= 0x0720,
    iavf_aqc_opc_thermal_sensor		= 0x0721,

// virtualization commands
    iavf_aqc_opc_send_msg_to_pf		= 0x0801,
    iavf_aqc_opc_send_msg_to_vf		= 0x0802,
    iavf_aqc_opc_send_msg_to_peer		= 0x0803,

// alternate structure
    iavf_aqc_opc_alternate_write		= 0x0900,
    iavf_aqc_opc_alternate_write_indirect	= 0x0901,
    iavf_aqc_opc_alternate_read		= 0x0902,
    iavf_aqc_opc_alternate_read_indirect	= 0x0903,
    iavf_aqc_opc_alternate_write_done	= 0x0904,
    iavf_aqc_opc_alternate_set_mode		= 0x0905,
    iavf_aqc_opc_alternate_clear_port	= 0x0906,

// LLDP commands
    iavf_aqc_opc_lldp_get_mib	= 0x0A00,
    iavf_aqc_opc_lldp_update_mib	= 0x0A01,
    iavf_aqc_opc_lldp_add_tlv	= 0x0A02,
    iavf_aqc_opc_lldp_update_tlv	= 0x0A03,
    iavf_aqc_opc_lldp_delete_tlv	= 0x0A04,
    iavf_aqc_opc_lldp_stop		= 0x0A05,
    iavf_aqc_opc_lldp_start		= 0x0A06,

// Tunnel commands
    iavf_aqc_opc_add_udp_tunnel	= 0x0B00,
    iavf_aqc_opc_del_udp_tunnel	= 0x0B01,
    iavf_aqc_opc_set_rss_key	= 0x0B02,
    iavf_aqc_opc_set_rss_lut	= 0x0B03,
    iavf_aqc_opc_get_rss_key	= 0x0B04,
    iavf_aqc_opc_get_rss_lut	= 0x0B05,

// Async Events
    iavf_aqc_opc_event_lan_overflow		= 0x1001,

// OEM commands
    iavf_aqc_opc_oem_parameter_change	= 0xFE00,
    iavf_aqc_opc_oem_device_status_change	= 0xFE01,
    iavf_aqc_opc_oem_ocsd_initialize	= 0xFE02,
    iavf_aqc_opc_oem_ocbb_initialize	= 0xFE03,

// debug commands
    iavf_aqc_opc_debug_read_reg		= 0xFF03,
    iavf_aqc_opc_debug_write_reg		= 0xFF04,
    iavf_aqc_opc_debug_modify_reg		= 0xFF07,
    iavf_aqc_opc_debug_dump_internals	= 0xFF08,
}

// command structures and indirect data structures
// Structure naming conventions:
// - no suffix for direct command descriptor structures
// - _data for indirect sent data
// - _resp for indirect return data (data which is both will use _data)
// - _completion for direct return data
// - _element_ for repeated elements (may also be _data or _resp)
//
// Command structures are expected to overlay the params.raw member of the basic
// descriptor, and as such cannot exceed 16 bytes in length.
//
// This macro is used to generate a compilation error if a structure
// is not exactly the correct length. It gives a divide by zero error if the
// structure is not of the correct size, otherwise it creates an enum that is
// never used.
//

// This macro is used extensively to ensure that command structures are 16
// bytes in length as they have to map to the raw array of that size.
//

// Queue Shutdown (direct 0x0003)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_queue_shutdown {
    pub driver_unloading: __le32,
pub const IAVF_AQ_DRIVER_UNLOADING: c_uint = 0x1;
    pub reserved: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_vsi_properties_data {
// first 96 byte are written by SW
    pub valid_sections: __le16,
pub const IAVF_AQ_VSI_PROP_SWITCH_VALID: c_uint = 0x0001;
pub const IAVF_AQ_VSI_PROP_SECURITY_VALID: c_uint = 0x0002;
pub const IAVF_AQ_VSI_PROP_VLAN_VALID: c_uint = 0x0004;
pub const IAVF_AQ_VSI_PROP_CAS_PV_VALID: c_uint = 0x0008;
pub const IAVF_AQ_VSI_PROP_INGRESS_UP_VALID: c_uint = 0x0010;
pub const IAVF_AQ_VSI_PROP_EGRESS_UP_VALID: c_uint = 0x0020;
pub const IAVF_AQ_VSI_PROP_QUEUE_MAP_VALID: c_uint = 0x0040;
pub const IAVF_AQ_VSI_PROP_QUEUE_OPT_VALID: c_uint = 0x0080;
pub const IAVF_AQ_VSI_PROP_OUTER_UP_VALID: c_uint = 0x0100;
pub const IAVF_AQ_VSI_PROP_SCHED_VALID: c_uint = 0x0200;
// switch section
    pub /: *mut *mut __le16 switch_id; / 12bit id combined with flags below,
pub const IAVF_AQ_VSI_SW_ID_SHIFT: c_uint = 0x0000;

pub const IAVF_AQ_VSI_SW_ID_FLAG_NOT_STAG: c_uint = 0x1000;
pub const IAVF_AQ_VSI_SW_ID_FLAG_ALLOW_LB: c_uint = 0x2000;
pub const IAVF_AQ_VSI_SW_ID_FLAG_LOCAL_LB: c_uint = 0x4000;
    pub sw_reserved: [u8; 2],
// security section
    pub sec_flags: u8,
pub const IAVF_AQ_VSI_SEC_FLAG_ALLOW_DEST_OVRD: c_uint = 0x01;
pub const IAVF_AQ_VSI_SEC_FLAG_ENABLE_VLAN_CHK: c_uint = 0x02;
pub const IAVF_AQ_VSI_SEC_FLAG_ENABLE_MAC_CHK: c_uint = 0x04;
    pub sec_reserved: u8,
// VLAN section
    pub /: *mut *mut __le16 pvid; / VLANS include priority bits,
    pub fcoe_pvid: __le16,
    pub port_vlan_flags: u8,
pub const IAVF_AQ_VSI_PVLAN_MODE_SHIFT: c_uint = 0x00;

pub const IAVF_AQ_VSI_PVLAN_MODE_TAGGED: c_uint = 0x01;
pub const IAVF_AQ_VSI_PVLAN_MODE_UNTAGGED: c_uint = 0x02;
pub const IAVF_AQ_VSI_PVLAN_MODE_ALL: c_uint = 0x03;
pub const IAVF_AQ_VSI_PVLAN_INSERT_PVID: c_uint = 0x04;
pub const IAVF_AQ_VSI_PVLAN_EMOD_SHIFT: c_uint = 0x03;

pub const IAVF_AQ_VSI_PVLAN_EMOD_STR_BOTH: c_uint = 0x0;
pub const IAVF_AQ_VSI_PVLAN_EMOD_STR_UP: c_uint = 0x08;
pub const IAVF_AQ_VSI_PVLAN_EMOD_STR: c_uint = 0x10;
pub const IAVF_AQ_VSI_PVLAN_EMOD_NOTHING: c_uint = 0x18;
    pub pvlan_reserved: [u8; 3],
// ingress egress up sections
    pub /: *mut *mut __le32 ingress_table; / bitmap, 3 bits per up,
pub const IAVF_AQ_VSI_UP_TABLE_UP0_SHIFT: c_int = 0;

pub const IAVF_AQ_VSI_UP_TABLE_UP1_SHIFT: c_int = 3;

pub const IAVF_AQ_VSI_UP_TABLE_UP2_SHIFT: c_int = 6;

pub const IAVF_AQ_VSI_UP_TABLE_UP3_SHIFT: c_int = 9;

pub const IAVF_AQ_VSI_UP_TABLE_UP4_SHIFT: c_int = 12;

pub const IAVF_AQ_VSI_UP_TABLE_UP5_SHIFT: c_int = 15;

pub const IAVF_AQ_VSI_UP_TABLE_UP6_SHIFT: c_int = 18;

pub const IAVF_AQ_VSI_UP_TABLE_UP7_SHIFT: c_int = 21;

    pub /: *mut *mut __le32 egress_table; / same defines as for ingress table,
// cascaded PV section
    pub cas_pv_tag: __le16,
    pub cas_pv_flags: u8,
pub const IAVF_AQ_VSI_CAS_PV_TAGX_SHIFT: c_uint = 0x00;

pub const IAVF_AQ_VSI_CAS_PV_TAGX_LEAVE: c_uint = 0x00;
pub const IAVF_AQ_VSI_CAS_PV_TAGX_REMOVE: c_uint = 0x01;
pub const IAVF_AQ_VSI_CAS_PV_TAGX_COPY: c_uint = 0x02;
pub const IAVF_AQ_VSI_CAS_PV_INSERT_TAG: c_uint = 0x10;
pub const IAVF_AQ_VSI_CAS_PV_ETAG_PRUNE: c_uint = 0x20;
pub const IAVF_AQ_VSI_CAS_PV_ACCEPT_HOST_TAG: c_uint = 0x40;
    pub cas_pv_reserved: u8,
// queue mapping section
    pub mapping_flags: __le16,
pub const IAVF_AQ_VSI_QUE_MAP_CONTIG: c_uint = 0x0;
pub const IAVF_AQ_VSI_QUE_MAP_NONCONTIG: c_uint = 0x1;
    pub queue_mapping: [__le16; 16],
pub const IAVF_AQ_VSI_QUEUE_SHIFT: c_uint = 0x0;
    pub tc_mapping: [__le16; 8],
pub const IAVF_AQ_VSI_TC_QUE_OFFSET_SHIFT: c_int = 0;

pub const IAVF_AQ_VSI_TC_QUE_NUMBER_SHIFT: c_int = 9;

// queueing option section
    pub queueing_opt_flags: u8,
pub const IAVF_AQ_VSI_QUE_OPT_MULTICAST_UDP_ENA: c_uint = 0x04;
pub const IAVF_AQ_VSI_QUE_OPT_UNICAST_UDP_ENA: c_uint = 0x08;
pub const IAVF_AQ_VSI_QUE_OPT_TCP_ENA: c_uint = 0x10;
pub const IAVF_AQ_VSI_QUE_OPT_FCOE_ENA: c_uint = 0x20;
pub const IAVF_AQ_VSI_QUE_OPT_RSS_LUT_PF: c_uint = 0x00;
pub const IAVF_AQ_VSI_QUE_OPT_RSS_LUT_VSI: c_uint = 0x40;
    pub queueing_opt_reserved: [u8; 3],
// scheduler section
    pub up_enable_bits: u8,
    pub sched_reserved: u8,
// outer up section
    pub /: *mut *mut __le32 outer_up_table; / same structure and defines as ingress tbl,
    pub cmd_reserved: [u8; 8],
// last 32 bytes are written by FW
    pub qs_handle: [__le16; 8],
pub const IAVF_AQ_VSI_QS_HANDLE_INVALID: c_uint = 0xFFFF;
    pub stat_counter_idx: __le16,
    pub sched_id: __le16,
    pub resp_reserved: [u8; 12],
}

// Get VEB Parameters (direct 0x0232)
// uses iavf_aqc_switch_seid for the descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_get_veb_parameters_completion {
    pub seid: __le16,
    pub switch_id: __le16,
    pub /: *mut *mut __le16 veb_flags; / only the first/last flags from 0x0230 is valid,
    pub statistic_index: __le16,
    pub vebs_used: __le16,
    pub vebs_free: __le16,
    pub reserved: [u8; 4],
}

pub const IAVF_LINK_SPEED_100MB_SHIFT: c_uint = 0x1;
pub const IAVF_LINK_SPEED_1000MB_SHIFT: c_uint = 0x2;
pub const IAVF_LINK_SPEED_10GB_SHIFT: c_uint = 0x3;
pub const IAVF_LINK_SPEED_40GB_SHIFT: c_uint = 0x4;
pub const IAVF_LINK_SPEED_20GB_SHIFT: c_uint = 0x5;
pub const IAVF_LINK_SPEED_25GB_SHIFT: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_aq_link_speed {
    IAVF_LINK_SPEED_UNKNOWN	= 0,
    IAVF_LINK_SPEED_100MB	= BIT(IAVF_LINK_SPEED_100MB_SHIFT),
    IAVF_LINK_SPEED_1GB	= BIT(IAVF_LINK_SPEED_1000MB_SHIFT),
    IAVF_LINK_SPEED_10GB	= BIT(IAVF_LINK_SPEED_10GB_SHIFT),
    IAVF_LINK_SPEED_40GB	= BIT(IAVF_LINK_SPEED_40GB_SHIFT),
    IAVF_LINK_SPEED_20GB	= BIT(IAVF_LINK_SPEED_20GB_SHIFT),
    IAVF_LINK_SPEED_25GB	= BIT(IAVF_LINK_SPEED_25GB_SHIFT),
}

// Send to PF command (indirect 0x0801) id is only used by PF
// Send to VF command (indirect 0x0802) id is only used by PF
// Send to Peer PF command (indirect 0x0803)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_pf_vf_message {
    pub id: __le32,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_get_set_rss_key {

pub const IAVF_AQC_SET_RSS_KEY_VSI_ID_SHIFT: c_int = 0;

    pub vsi_id: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_get_set_rss_key_data {
    pub standard_rss_key: [u8; 0x28],
    pub extended_hash_key: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_aqc_get_set_rss_lut {

pub const IAVF_AQC_SET_RSS_LUT_VSI_ID_SHIFT: c_int = 0;

    pub vsi_id: __le16,
pub const IAVF_AQC_SET_RSS_LUT_TABLE_TYPE_SHIFT: c_int = 0;

pub const IAVF_AQC_SET_RSS_LUT_TABLE_TYPE_VSI: c_int = 0;
pub const IAVF_AQC_SET_RSS_LUT_TABLE_TYPE_PF: c_int = 1;
    pub flags: __le16,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}
