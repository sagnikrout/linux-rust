//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_adminq_cmd.h
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
// Copyright(c) 2013 - 2021 Intel Corporation.

// This header file defines the i40e Admin Queue commands and is shared between
// i40e Firmware and Software.
//
// This file needs to comply with the Linux Kernel coding style.
//
pub const I40E_FW_API_VERSION_MAJOR: c_uint = 0x0001;
pub const I40E_FW_API_VERSION_MINOR_X722: c_uint = 0x000C;
pub const I40E_FW_API_VERSION_MINOR_X710: c_uint = 0x000F;

// API version 1.7 implements additional link and PHY-specific APIs
pub const I40E_MINOR_VER_GET_LINK_INFO_XL710: c_uint = 0x0007;
// API version 1.9 for X722 implements additional link and PHY-specific APIs
pub const I40E_MINOR_VER_GET_LINK_INFO_X722: c_uint = 0x0009;
// API version 1.6 for X722 devices adds ability to stop FW LLDP agent
pub const I40E_MINOR_VER_FW_LLDP_STOPPABLE_X722: c_uint = 0x0006;
// API version 1.10 for X722 devices adds ability to request FEC encoding
pub const I40E_MINOR_VER_FW_REQUEST_FEC_X722: c_uint = 0x000A;
// Admin Queue command opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_admin_queue_opc {
// aq commands
    i40e_aqc_opc_get_version	= 0x0001,
    i40e_aqc_opc_driver_version	= 0x0002,
    i40e_aqc_opc_queue_shutdown	= 0x0003,
    i40e_aqc_opc_set_pf_context	= 0x0004,

// resource ownership
    i40e_aqc_opc_request_resource	= 0x0008,
    i40e_aqc_opc_release_resource	= 0x0009,

    i40e_aqc_opc_list_func_capabilities	= 0x000A,
    i40e_aqc_opc_list_dev_capabilities	= 0x000B,

// Proxy commands
    i40e_aqc_opc_set_proxy_config		= 0x0104,
    i40e_aqc_opc_set_ns_proxy_table_entry	= 0x0105,

// LAA
    i40e_aqc_opc_mac_address_read	= 0x0107,
    i40e_aqc_opc_mac_address_write	= 0x0108,

// PXE
    i40e_aqc_opc_clear_pxe_mode	= 0x0110,

// WoL commands
    i40e_aqc_opc_set_wol_filter	= 0x0120,
    i40e_aqc_opc_get_wake_reason	= 0x0121,

// internal switch commands
    i40e_aqc_opc_get_switch_config		= 0x0200,
    i40e_aqc_opc_add_statistics		= 0x0201,
    i40e_aqc_opc_remove_statistics		= 0x0202,
    i40e_aqc_opc_set_port_parameters	= 0x0203,
    i40e_aqc_opc_get_switch_resource_alloc	= 0x0204,
    i40e_aqc_opc_set_switch_config		= 0x0205,
    i40e_aqc_opc_rx_ctl_reg_read		= 0x0206,
    i40e_aqc_opc_rx_ctl_reg_write		= 0x0207,

    i40e_aqc_opc_add_vsi			= 0x0210,
    i40e_aqc_opc_update_vsi_parameters	= 0x0211,
    i40e_aqc_opc_get_vsi_parameters		= 0x0212,

    i40e_aqc_opc_add_pv			= 0x0220,
    i40e_aqc_opc_update_pv_parameters	= 0x0221,
    i40e_aqc_opc_get_pv_parameters		= 0x0222,

    i40e_aqc_opc_add_veb			= 0x0230,
    i40e_aqc_opc_update_veb_parameters	= 0x0231,
    i40e_aqc_opc_get_veb_parameters		= 0x0232,

    i40e_aqc_opc_delete_element		= 0x0243,

    i40e_aqc_opc_add_macvlan		= 0x0250,
    i40e_aqc_opc_remove_macvlan		= 0x0251,
    i40e_aqc_opc_add_vlan			= 0x0252,
    i40e_aqc_opc_remove_vlan		= 0x0253,
    i40e_aqc_opc_set_vsi_promiscuous_modes	= 0x0254,
    i40e_aqc_opc_add_tag			= 0x0255,
    i40e_aqc_opc_remove_tag			= 0x0256,
    i40e_aqc_opc_add_multicast_etag		= 0x0257,
    i40e_aqc_opc_remove_multicast_etag	= 0x0258,
    i40e_aqc_opc_update_tag			= 0x0259,
    i40e_aqc_opc_add_control_packet_filter	= 0x025A,
    i40e_aqc_opc_remove_control_packet_filter	= 0x025B,
    i40e_aqc_opc_add_cloud_filters		= 0x025C,
    i40e_aqc_opc_remove_cloud_filters	= 0x025D,
    i40e_aqc_opc_clear_wol_switch_filters	= 0x025E,

    i40e_aqc_opc_add_mirror_rule	= 0x0260,
    i40e_aqc_opc_delete_mirror_rule	= 0x0261,

// Dynamic Device Personalization
    i40e_aqc_opc_write_personalization_profile	= 0x0270,
    i40e_aqc_opc_get_personalization_profile_list	= 0x0271,

// DCB commands
    i40e_aqc_opc_dcb_ignore_pfc	= 0x0301,
    i40e_aqc_opc_dcb_updated	= 0x0302,
    i40e_aqc_opc_set_dcb_parameters = 0x0303,

// TX scheduler
    i40e_aqc_opc_configure_vsi_bw_limit		= 0x0400,
    i40e_aqc_opc_configure_vsi_ets_sla_bw_limit	= 0x0406,
    i40e_aqc_opc_configure_vsi_tc_bw		= 0x0407,
    i40e_aqc_opc_query_vsi_bw_config		= 0x0408,
    i40e_aqc_opc_query_vsi_ets_sla_config		= 0x040A,
    i40e_aqc_opc_configure_switching_comp_bw_limit	= 0x0410,

    i40e_aqc_opc_enable_switching_comp_ets			= 0x0413,
    i40e_aqc_opc_modify_switching_comp_ets			= 0x0414,
    i40e_aqc_opc_disable_switching_comp_ets			= 0x0415,
    i40e_aqc_opc_configure_switching_comp_ets_bw_limit	= 0x0416,
    i40e_aqc_opc_configure_switching_comp_bw_config		= 0x0417,
    i40e_aqc_opc_query_switching_comp_ets_config		= 0x0418,
    i40e_aqc_opc_query_port_ets_config			= 0x0419,
    i40e_aqc_opc_query_switching_comp_bw_config		= 0x041A,
    i40e_aqc_opc_suspend_port_tx				= 0x041B,
    i40e_aqc_opc_resume_port_tx				= 0x041C,
    i40e_aqc_opc_configure_partition_bw			= 0x041D,
// hmc
    i40e_aqc_opc_query_hmc_resource_profile	= 0x0500,
    i40e_aqc_opc_set_hmc_resource_profile	= 0x0501,

// phy commands
    i40e_aqc_opc_get_phy_abilities		= 0x0600,
    i40e_aqc_opc_set_phy_config		= 0x0601,
    i40e_aqc_opc_set_mac_config		= 0x0603,
    i40e_aqc_opc_set_link_restart_an	= 0x0605,
    i40e_aqc_opc_get_link_status		= 0x0607,
    i40e_aqc_opc_set_phy_int_mask		= 0x0613,
    i40e_aqc_opc_get_local_advt_reg		= 0x0614,
    i40e_aqc_opc_set_local_advt_reg		= 0x0615,
    i40e_aqc_opc_get_partner_advt		= 0x0616,
    i40e_aqc_opc_set_lb_modes		= 0x0618,
    i40e_aqc_opc_get_phy_wol_caps		= 0x0621,
    i40e_aqc_opc_set_phy_debug		= 0x0622,
    i40e_aqc_opc_upload_ext_phy_fm		= 0x0625,
    i40e_aqc_opc_run_phy_activity		= 0x0626,
    i40e_aqc_opc_set_phy_register		= 0x0628,
    i40e_aqc_opc_get_phy_register		= 0x0629,

// NVM commands
    i40e_aqc_opc_nvm_read			= 0x0701,
    i40e_aqc_opc_nvm_erase			= 0x0702,
    i40e_aqc_opc_nvm_update			= 0x0703,
    i40e_aqc_opc_nvm_config_read		= 0x0704,
    i40e_aqc_opc_nvm_config_write		= 0x0705,
    i40e_aqc_opc_oem_post_update		= 0x0720,
    i40e_aqc_opc_thermal_sensor		= 0x0721,

// virtualization commands
    i40e_aqc_opc_send_msg_to_pf		= 0x0801,
    i40e_aqc_opc_send_msg_to_vf		= 0x0802,
    i40e_aqc_opc_send_msg_to_peer		= 0x0803,

// alternate structure
    i40e_aqc_opc_alternate_write		= 0x0900,
    i40e_aqc_opc_alternate_write_indirect	= 0x0901,
    i40e_aqc_opc_alternate_read		= 0x0902,
    i40e_aqc_opc_alternate_read_indirect	= 0x0903,
    i40e_aqc_opc_alternate_write_done	= 0x0904,
    i40e_aqc_opc_alternate_set_mode		= 0x0905,
    i40e_aqc_opc_alternate_clear_port	= 0x0906,

// LLDP commands
    i40e_aqc_opc_lldp_get_mib	= 0x0A00,
    i40e_aqc_opc_lldp_update_mib	= 0x0A01,
    i40e_aqc_opc_lldp_add_tlv	= 0x0A02,
    i40e_aqc_opc_lldp_update_tlv	= 0x0A03,
    i40e_aqc_opc_lldp_delete_tlv	= 0x0A04,
    i40e_aqc_opc_lldp_stop		= 0x0A05,
    i40e_aqc_opc_lldp_start		= 0x0A06,
    i40e_aqc_opc_get_cee_dcb_cfg	= 0x0A07,
    i40e_aqc_opc_lldp_set_local_mib	= 0x0A08,
    i40e_aqc_opc_lldp_stop_start_spec_agent	= 0x0A09,
    i40e_aqc_opc_lldp_restore		= 0x0A0A,

// Tunnel commands
    i40e_aqc_opc_add_udp_tunnel	= 0x0B00,
    i40e_aqc_opc_del_udp_tunnel	= 0x0B01,
    i40e_aqc_opc_set_rss_key	= 0x0B02,
    i40e_aqc_opc_set_rss_lut	= 0x0B03,
    i40e_aqc_opc_get_rss_key	= 0x0B04,
    i40e_aqc_opc_get_rss_lut	= 0x0B05,

// Async Events
    i40e_aqc_opc_event_lan_overflow		= 0x1001,

// OEM commands
    i40e_aqc_opc_oem_parameter_change	= 0xFE00,
    i40e_aqc_opc_oem_device_status_change	= 0xFE01,
    i40e_aqc_opc_oem_ocsd_initialize	= 0xFE02,
    i40e_aqc_opc_oem_ocbb_initialize	= 0xFE03,

// debug commands
    i40e_aqc_opc_debug_read_reg		= 0xFF03,
    i40e_aqc_opc_debug_write_reg		= 0xFF04,
    i40e_aqc_opc_debug_modify_reg		= 0xFF07,
    i40e_aqc_opc_debug_dump_internals	= 0xFF08,
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

// internal (0x00XX) commands
// Get version (direct 0x0001)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_version {
    pub rom_ver: __le32,
    pub fw_build: __le32,
    pub fw_major: __le16,
    pub fw_minor: __le16,
    pub api_major: __le16,
    pub api_minor: __le16,
}

// Queue Shutdown (direct 0x0003)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_queue_shutdown {
    pub driver_unloading: __le32,
pub const I40E_AQ_DRIVER_UNLOADING: c_uint = 0x1;
    pub reserved: [u8; 12],
}

// Set PF context (0x0004, direct)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_pf_context {
    pub pf_id: u8,
    pub reserved: [u8; 15],
}

// Set CPPM Configuration (direct 0x0103)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_cppm_configuration {
    pub command_flags: __le16,
    pub ttlx: __le16,
    pub dmacr: __le32,
    pub dmcth: __le16,
    pub hptc: u8,
    pub reserved: u8,
    pub pfltrc: __le32,
}

// Set ARP Proxy command / response (indirect 0x0104)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_arp_proxy_data {
    pub command_flags: __le16,
    pub table_id: __le16,
    pub enabled_offloads: __le32,
    pub ip_addr: __le32,
    pub mac_addr: [u8; 6],
    pub reserved: [u8; 2],
}

// Set NS Proxy Table Entry Command (indirect 0x0105)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_ns_proxy_data {
    pub table_idx_mac_addr_0: __le16,
    pub table_idx_mac_addr_1: __le16,
    pub table_idx_ipv6_0: __le16,
    pub table_idx_ipv6_1: __le16,
    pub control: __le16,
    pub mac_addr_0: [u8; 6],
    pub mac_addr_1: [u8; 6],
    pub local_mac_addr: [u8; 6],
    pub /: *mut *mut u8 ipv6_addr_0[16]; / Warning! spec specifies BE byte order,
    pub ipv6_addr_1: [u8; 16],
}

// Manage LAA Command (0x0106) - obsolete
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_mng_laa {
    pub command_flags: __le16,
    pub reserved: [u8; 2],
    pub sal: __le32,
    pub sah: __le16,
    pub reserved2: [u8; 6],
}

// Manage MAC Address Read Command (indirect 0x0107)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_mac_address_read {
    pub command_flags: __le16,
pub const I40E_AQC_LAN_ADDR_VALID: c_uint = 0x10;
pub const I40E_AQC_PORT_ADDR_VALID: c_uint = 0x40;
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_mac_address_read_data {
    pub pf_lan_mac: [u8; 6],
    pub pf_san_mac: [u8; 6],
    pub port_mac: [u8; 6],
    pub pf_wol_mac: [u8; 6],
}

// Manage MAC Address Write Command (0x0108)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_mac_address_write {
    pub command_flags: __le16,
pub const I40E_AQC_MC_MAG_EN: c_uint = 0x0100;
pub const I40E_AQC_WOL_PRESERVE_ON_PFR: c_uint = 0x0200;
pub const I40E_AQC_WRITE_TYPE_LAA_ONLY: c_uint = 0x0000;
pub const I40E_AQC_WRITE_TYPE_LAA_WOL: c_uint = 0x4000;
pub const I40E_AQC_WRITE_TYPE_UPDATE_MC_MAG: c_uint = 0xC000;
    pub mac_sah: __le16,
    pub mac_sal: __le32,
    pub reserved: [u8; 8],
}

// PXE commands (0x011x)
// Clear PXE Command and response  (direct 0x0110)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_clear_pxe {
    pub rx_cnt: u8,
    pub reserved: [u8; 15],
}

// Set WoL Filter (0x0120)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_wol_filter {
    pub filter_index: __le16,
    pub cmd_flags: __le16,
    pub valid_flags: __le16,
    pub reserved: [u8; 2],
    pub address_high: __le32,
    pub address_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_wol_filter_data {
    pub filter: [u8; 128],
    pub mask: [u8; 16],
}

// Get Wake Reason (0x0121)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_wake_reason_completion {
    pub reserved_1: [u8; 2],
    pub wake_reason: __le16,
    pub reserved_2: [u8; 12],
}

// Switch configuration commands (0x02xx)
// Used by many indirect commands that only pass an seid and a buffer in the
// command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_switch_seid {
    pub seid: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Get Switch Configuration command (indirect 0x0200)
// uses i40e_aqc_switch_seid for the descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_switch_config_header_resp {
    pub num_reported: __le16,
    pub num_total: __le16,
    pub reserved: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_switch_config_element_resp {
    pub element_type: u8,
    pub revision: u8,
    pub seid: __le16,
    pub uplink_seid: __le16,
    pub downlink_seid: __le16,
    pub reserved: [u8; 3],
    pub connection_type: u8,
    pub scheduler_id: __le16,
    pub element_info: __le16,
}

// Get Switch Configuration (indirect 0x0200)
// an array of elements are returned in the response buffer
// the first in the array is the header, remainder are elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_switch_config_resp {
    pub header: i40e_aqc_get_switch_config_header_resp,
    pub element: [i40e_aqc_switch_config_element_resp; 1],
}

// Add Statistics (direct 0x0201)
// Remove Statistics (direct 0x0202)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_statistics {
    pub seid: __le16,
    pub vlan: __le16,
    pub stat_index: __le16,
    pub reserved: [u8; 10],
}

// Set Port Parameters command (direct 0x0203)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_port_parameters {
    pub command_flags: __le16,
    pub bad_frame_vsi: __le16,
    pub /: *mut *mut __le16 default_seid; / reserved for command,
    pub reserved: [u8; 10],
}

// Get Switch Resource Allocation (indirect 0x0204)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_switch_resource_alloc {
    pub /: *mut *mut u8 num_entries; / reserved for command,
    pub reserved: [u8; 7],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// expect an array of these structs in the response buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_switch_resource_alloc_element_resp {
    pub resource_type: u8,
    pub reserved1: u8,
    pub guaranteed: __le16,
    pub total: __le16,
    pub used: __le16,
    pub total_unalloced: __le16,
    pub reserved2: [u8; 6],
}

// Set Switch Configuration (direct 0x0205)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_switch_config {
    pub flags: __le16,
// flags used for both fields below
pub const I40E_AQ_SET_SWITCH_CFG_PROMISC: c_uint = 0x0001;
    pub valid_flags: __le16,
// The ethertype in switch_tag is dropped on ingress and used
// internally by the switch. Set this to zero for the default
// of 0x88a8 (802.1ad). Should be zero for firmware API
// versions lower than 1.7.
//
    pub switch_tag: __le16,
// The ethertypes in first_tag and second_tag are used to
// match the outer and inner VLAN tags (respectively) when HW
// double VLAN tagging is enabled via the set port parameters
// AQ command. Otherwise these are both ignored. Set them to
// zero for their defaults of 0x8100 (802.1Q). Should be zero
// for firmware API versions lower than 1.7.
//
    pub first_tag: __le16,
    pub second_tag: __le16,
// Next byte is split into following:
// Bit 7    : 0 : No action, 1: Switch to mode defined by bits 6:0
// Bit 6    : 0 : Destination Port, 1: source port
// Bit 5..4 : L4 type
// 0: rsvd
// 1: TCP
// 2: UDP
// 3: Both TCP and UDP
// Bits 3:0 Mode
// 0: default mode
// 1: L4 port only mode
// 2: non-tunneled mode
// 3: tunneled mode
//
pub const I40E_AQ_SET_SWITCH_BIT7_VALID: c_uint = 0x80;
pub const I40E_AQ_SET_SWITCH_L4_TYPE_TCP: c_uint = 0x10;
pub const I40E_AQ_SET_SWITCH_MODE_NON_TUNNEL: c_uint = 0x02;
    pub mode: u8,
    pub rsvd5: [u8; 5],
}

// Read Receive control registers  (direct 0x0206)
// Write Receive control registers (direct 0x0207)
// used for accessing Rx control registers that can be
// slow and need special handling when under high Rx load
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_rx_ctl_reg_read_write {
    pub reserved1: __le32,
    pub address: __le32,
    pub reserved2: __le32,
    pub value: __le32,
}

// Add VSI (indirect 0x0210)
// this indirect command uses struct i40e_aqc_vsi_properties_data
// as the indirect buffer (128 bytes)
//
// Update VSI (indirect 0x211)
// uses the same data structure as Add VSI
//
// Get VSI (indirect 0x0212)
// uses the same completion and data structure as Add VSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_get_update_vsi {
    pub uplink_seid: __le16,
    pub connection_type: u8,
pub const I40E_AQ_VSI_CONN_TYPE_NORMAL: c_uint = 0x1;
    pub reserved1: u8,
    pub vf_id: u8,
    pub reserved2: u8,
    pub vsi_flags: __le16,
pub const I40E_AQ_VSI_TYPE_VF: c_uint = 0x0;
pub const I40E_AQ_VSI_TYPE_VMDQ2: c_uint = 0x1;
pub const I40E_AQ_VSI_TYPE_PF: c_uint = 0x2;
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_get_update_vsi_completion {
    pub seid: __le16,
    pub vsi_number: __le16,
    pub vsi_used: __le16,
    pub vsi_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_vsi_properties_data {
// first 96 byte are written by SW
    pub valid_sections: __le16,
pub const I40E_AQ_VSI_PROP_SWITCH_VALID: c_uint = 0x0001;
pub const I40E_AQ_VSI_PROP_SECURITY_VALID: c_uint = 0x0002;
pub const I40E_AQ_VSI_PROP_VLAN_VALID: c_uint = 0x0004;
pub const I40E_AQ_VSI_PROP_QUEUE_MAP_VALID: c_uint = 0x0040;
pub const I40E_AQ_VSI_PROP_QUEUE_OPT_VALID: c_uint = 0x0080;
pub const I40E_AQ_VSI_PROP_SCHED_VALID: c_uint = 0x0200;
// switch section
    pub /: *mut *mut __le16 switch_id; / 12bit id combined with flags below,
pub const I40E_AQ_VSI_SW_ID_SHIFT: c_uint = 0x0000;

pub const I40E_AQ_VSI_SW_ID_FLAG_ALLOW_LB: c_uint = 0x2000;
pub const I40E_AQ_VSI_SW_ID_FLAG_LOCAL_LB: c_uint = 0x4000;
    pub sw_reserved: [u8; 2],
// security section
    pub sec_flags: u8,
pub const I40E_AQ_VSI_SEC_FLAG_ENABLE_VLAN_CHK: c_uint = 0x02;
pub const I40E_AQ_VSI_SEC_FLAG_ENABLE_MAC_CHK: c_uint = 0x04;
    pub sec_reserved: u8,
// VLAN section
    pub /: *mut *mut __le16 pvid; / VLANS include priority bits,
    pub fcoe_pvid: __le16,
    pub port_vlan_flags: u8,
pub const I40E_AQ_VSI_PVLAN_MODE_SHIFT: c_uint = 0x00;

pub const I40E_AQ_VSI_PVLAN_MODE_TAGGED: c_uint = 0x01;
pub const I40E_AQ_VSI_PVLAN_MODE_ALL: c_uint = 0x03;
pub const I40E_AQ_VSI_PVLAN_INSERT_PVID: c_uint = 0x04;
pub const I40E_AQ_VSI_PVLAN_EMOD_SHIFT: c_uint = 0x03;

pub const I40E_AQ_VSI_PVLAN_EMOD_STR_BOTH: c_uint = 0x0;
pub const I40E_AQ_VSI_PVLAN_EMOD_STR: c_uint = 0x10;
pub const I40E_AQ_VSI_PVLAN_EMOD_NOTHING: c_uint = 0x18;
    pub pvlan_reserved: [u8; 3],
// ingress egress up sections
    pub /: *mut *mut __le32 ingress_table; / bitmap, 3 bits per up,
    pub /: *mut *mut __le32 egress_table; / same defines as for ingress table,
// cascaded PV section
    pub cas_pv_tag: __le16,
    pub cas_pv_flags: u8,
    pub cas_pv_reserved: u8,
// queue mapping section
    pub mapping_flags: __le16,
pub const I40E_AQ_VSI_QUE_MAP_CONTIG: c_uint = 0x0;
pub const I40E_AQ_VSI_QUE_MAP_NONCONTIG: c_uint = 0x1;
    pub queue_mapping: [__le16; 16],
    pub tc_mapping: [__le16; 8],
pub const I40E_AQ_VSI_TC_QUE_OFFSET_SHIFT: c_int = 0;
pub const I40E_AQ_VSI_TC_QUE_NUMBER_SHIFT: c_int = 9;
// queueing option section
    pub queueing_opt_flags: u8,
pub const I40E_AQ_VSI_QUE_OPT_TCP_ENA: c_uint = 0x10;
pub const I40E_AQ_VSI_QUE_OPT_RSS_LUT_VSI: c_uint = 0x40;
    pub queueing_opt_reserved: [u8; 3],
// scheduler section
    pub up_enable_bits: u8,
    pub sched_reserved: u8,
// outer up section
    pub /: *mut *mut __le32 outer_up_table; / same structure and defines as ingress tbl,
    pub cmd_reserved: [u8; 8],
// last 32 bytes are written by FW
    pub qs_handle: [__le16; 8],
pub const I40E_AQ_VSI_QS_HANDLE_INVALID: c_uint = 0xFFFF;
    pub stat_counter_idx: __le16,
    pub sched_id: __le16,
    pub resp_reserved: [u8; 12],
}

// Add Port Virtualizer (direct 0x0220)
// also used for update PV (direct 0x0221) but only flags are used
// (IS_CTRL_PORT only works on add PV)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_update_pv {
    pub command_flags: __le16,
    pub uplink_seid: __le16,
    pub connected_seid: __le16,
    pub reserved: [u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_update_pv_completion {
// reserved for update; for add also encodes error if rc == ENOSPC
    pub pv_seid: __le16,
    pub reserved: [u8; 14],
}

// Get PV Params (direct 0x0222)
// uses i40e_aqc_switch_seid for the descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_pv_params_completion {
    pub seid: __le16,
    pub default_stag: __le16,
    pub /: *mut *mut __le16 pv_flags; / same flags as add_pv,
    pub reserved: [u8; 8],
    pub default_port_seid: __le16,
}

// Add VEB (direct 0x0230)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_veb {
    pub uplink_seid: __le16,
    pub downlink_seid: __le16,
    pub veb_flags: __le16,
pub const I40E_AQC_ADD_VEB_FLOATING: c_uint = 0x1;
pub const I40E_AQC_ADD_VEB_PORT_TYPE_DEFAULT: c_uint = 0x2;
pub const I40E_AQC_ADD_VEB_PORT_TYPE_DATA: c_uint = 0x4;
pub const I40E_AQC_ADD_VEB_ENABLE_DISABLE_STATS: c_uint = 0x10;
    pub enable_tcs: u8,
    pub reserved: [u8; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_veb_completion {
    pub reserved: [u8; 6],
    pub switch_seid: __le16,
// also encodes error if rc == ENOSPC; codes are the same as add_pv
    pub veb_seid: __le16,
    pub statistic_index: __le16,
    pub vebs_used: __le16,
    pub vebs_free: __le16,
}

// Get VEB Parameters (direct 0x0232)
// uses i40e_aqc_switch_seid for the descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_veb_parameters_completion {
    pub seid: __le16,
    pub switch_id: __le16,
    pub /: *mut *mut __le16 veb_flags; / only the first/last flags from 0x0230 is valid,
    pub statistic_index: __le16,
    pub vebs_used: __le16,
    pub vebs_free: __le16,
    pub reserved: [u8; 4],
}

// Delete Element (direct 0x0243)
// uses the generic i40e_aqc_switch_seid
//
// Add MAC-VLAN (indirect 0x0250)
// used for the command for most vlan commands
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_macvlan {
    pub num_addresses: __le16,
    pub seid: [__le16; 3],
pub const I40E_AQC_MACVLAN_CMD_SEID_VALID: c_uint = 0x8000;
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// indirect data for command and response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_macvlan_element_data {
    pub mac_addr: [u8; 6],
    pub vlan_tag: __le16,
    pub flags: __le16,
pub const I40E_AQC_MACVLAN_ADD_PERFECT_MATCH: c_uint = 0x0001;
pub const I40E_AQC_MACVLAN_ADD_IGNORE_VLAN: c_uint = 0x0004;
pub const I40E_AQC_MACVLAN_ADD_USE_SHARED_MAC: c_uint = 0x0010;
    pub queue_number: __le16,
// response section
    pub match_method: u8,
pub const I40E_AQC_MM_ERR_NO_RES: c_uint = 0xFF;
    pub reserved1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_macvlan_completion {
    pub perfect_mac_used: __le16,
    pub perfect_mac_free: __le16,
    pub unicast_hash_free: __le16,
    pub multicast_hash_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Remove MAC-VLAN (indirect 0x0251)
// uses i40e_aqc_macvlan for the descriptor
// data points to an array of num_addresses of elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_remove_macvlan_element_data {
    pub mac_addr: [u8; 6],
    pub vlan_tag: __le16,
    pub flags: u8,
pub const I40E_AQC_MACVLAN_DEL_PERFECT_MATCH: c_uint = 0x01;
pub const I40E_AQC_MACVLAN_DEL_IGNORE_VLAN: c_uint = 0x08;
    pub reserved: [u8; 3],
// reply section
    pub error_code: u8,
    pub reply_reserved: [u8; 3],
}

// Add VLAN (indirect 0x0252)
// Remove VLAN (indirect 0x0253)
// use the generic i40e_aqc_macvlan for the command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_vlan_element_data {
    pub vlan_tag: __le16,
    pub vlan_flags: u8,
    pub reserved: u8,
    pub result: u8,
    pub reserved1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_vlan_completion {
    pub reserved: [u8; 4],
    pub vlans_used: __le16,
    pub vlans_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Set VSI Promiscuous Modes (direct 0x0254)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_vsi_promiscuous_modes {
    pub promiscuous_flags: __le16,
    pub valid_flags: __le16,
// flags used for both fields above
pub const I40E_AQC_SET_VSI_PROMISC_UNICAST: c_uint = 0x01;
pub const I40E_AQC_SET_VSI_PROMISC_MULTICAST: c_uint = 0x02;
pub const I40E_AQC_SET_VSI_PROMISC_BROADCAST: c_uint = 0x04;
pub const I40E_AQC_SET_VSI_DEFAULT: c_uint = 0x08;
pub const I40E_AQC_SET_VSI_PROMISC_VLAN: c_uint = 0x10;
pub const I40E_AQC_SET_VSI_PROMISC_RX_ONLY: c_uint = 0x8000;
    pub seid: __le16,
    pub vlan_tag: __le16,
pub const I40E_AQC_SET_VSI_VLAN_VALID: c_uint = 0x8000;
    pub reserved: [u8; 8],
}

// Add S/E-tag command (direct 0x0255)
// Uses generic i40e_aqc_add_remove_tag_completion for completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_tag {
    pub flags: __le16,
    pub seid: __le16,
    pub tag: __le16,
    pub queue_number: __le16,
    pub reserved: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_tag_completion {
    pub reserved: [u8; 12],
    pub tags_used: __le16,
    pub tags_free: __le16,
}

// Remove S/E-tag command (direct 0x0256)
// Uses generic i40e_aqc_add_remove_tag_completion for completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_remove_tag {
    pub seid: __le16,
    pub tag: __le16,
    pub reserved: [u8; 12],
}

// Add multicast E-Tag (direct 0x0257)
// del multicast E-Tag (direct 0x0258) only uses pv_seid and etag fields
// and no external data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_mcast_etag {
    pub pv_seid: __le16,
    pub etag: __le16,
    pub num_unicast_etags: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut __le32 addr_high; / address of array of 2-byte s-tags,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_mcast_etag_completion {
    pub reserved: [u8; 4],
    pub mcast_etags_used: __le16,
    pub mcast_etags_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Update S/E-Tag (direct 0x0259)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_update_tag {
    pub seid: __le16,
    pub old_tag: __le16,
    pub new_tag: __le16,
    pub reserved: [u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_update_tag_completion {
    pub reserved: [u8; 12],
    pub tags_used: __le16,
    pub tags_free: __le16,
}

// Add Control Packet filter (direct 0x025A)
// Remove Control Packet filter (direct 0x025B)
// uses the i40e_aqc_add_oveb_cloud,
// and the generic direct completion structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_control_packet_filter {
    pub mac: [u8; 6],
    pub etype: __le16,
    pub flags: __le16,
pub const I40E_AQC_ADD_CONTROL_PACKET_FLAGS_IGNORE_MAC: c_uint = 0x0001;
pub const I40E_AQC_ADD_CONTROL_PACKET_FLAGS_DROP: c_uint = 0x0002;
pub const I40E_AQC_ADD_CONTROL_PACKET_FLAGS_TX: c_uint = 0x0008;
pub const I40E_AQC_ADD_CONTROL_PACKET_FLAGS_RX: c_uint = 0x0000;
    pub seid: __le16,
    pub queue: __le16,
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_control_packet_filter_completion {
    pub mac_etype_used: __le16,
    pub etype_used: __le16,
    pub mac_etype_free: __le16,
    pub etype_free: __le16,
    pub reserved: [u8; 8],
}

// Add Cloud filters (indirect 0x025C)
// Remove Cloud filters (indirect 0x025D)
// uses the i40e_aqc_add_remove_cloud_filters,
// and the generic indirect completion structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_remove_cloud_filters {
    pub num_filters: u8,
    pub reserved: u8,
    pub seid: __le16,
    pub big_buffer_flag: u8,
pub const I40E_AQC_ADD_CLOUD_CMD_BB: c_int = 1;
    pub reserved2: [u8; 3],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_cloud_filters_element_data {
    pub outer_mac: [u8; 6],
    pub inner_mac: [u8; 6],
    pub inner_vlan: __le16,
    pub reserved: [u8; 12],
    pub data: [u8; 4],
    pub v4: },
    pub data: [u8; 16],
    pub v6: },
    pub data: [__le16; 8],
    pub raw_v6: },
    pub ipaddr: },
    pub flags: __le16,
// 0x0000 reserved
// 0x0001 reserved
// 0x0002 reserved
pub const I40E_AQC_ADD_CLOUD_FILTER_IMAC_IVLAN: c_uint = 0x0003;
pub const I40E_AQC_ADD_CLOUD_FILTER_IMAC_IVLAN_TEN_ID: c_uint = 0x0004;
// 0x0005 reserved
pub const I40E_AQC_ADD_CLOUD_FILTER_IMAC_TEN_ID: c_uint = 0x0006;
// 0x0007 reserved
// 0x0008 reserved
pub const I40E_AQC_ADD_CLOUD_FILTER_OMAC: c_uint = 0x0009;
pub const I40E_AQC_ADD_CLOUD_FILTER_IMAC: c_uint = 0x000A;
pub const I40E_AQC_ADD_CLOUD_FILTER_OMAC_TEN_ID_IMAC: c_uint = 0x000B;
pub const I40E_AQC_ADD_CLOUD_FILTER_IIP: c_uint = 0x000C;
// 0x000D reserved
// 0x000E reserved
// 0x000F reserved
// 0x0010 to 0x0017 is for custom filters
pub const I40E_AQC_ADD_CLOUD_FILTER_IP_PORT: c_uint = 0x0010 /* Dest IP + L4 Port */;
pub const I40E_AQC_ADD_CLOUD_FILTER_MAC_PORT: c_uint = 0x0011 /* Dest MAC + L4 Port */;
pub const I40E_AQC_ADD_CLOUD_FILTER_MAC_VLAN_PORT: c_uint = 0x0012 /* Dest MAC + VLAN + L4 Port */;
pub const I40E_AQC_ADD_CLOUD_FLAGS_IPV4: c_int = 0;
pub const I40E_AQC_ADD_CLOUD_FLAGS_IPV6: c_uint = 0x0100;
pub const I40E_AQC_ADD_CLOUD_TNL_TYPE_SHIFT: c_int = 9;
pub const I40E_AQC_ADD_CLOUD_TNL_TYPE_MASK: c_uint = 0x1E00;
pub const I40E_AQC_ADD_CLOUD_TNL_TYPE_GENEVE: c_int = 2;
    pub tenant_id: __le32,
    pub reserved: [u8; 4],
    pub queue_number: __le16,
    pub reserved2: [u8; 14],
// response section
    pub allocation_result: u8,
    pub response_reserved: [u8; 7],
}

// i40e_aqc_cloud_filters_element_bb is used when
// I40E_AQC_CLOUD_CMD_BB flag is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_cloud_filters_element_bb {
    pub element: i40e_aqc_cloud_filters_element_data,
    pub general_fields: [u16; 32],
pub const I40E_AQC_ADD_CLOUD_FV_FLU_0X16_WORD0: c_int = 15;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_remove_cloud_filters_completion {
    pub perfect_ovlan_used: __le16,
    pub perfect_ovlan_free: __le16,
    pub vlan_used: __le16,
    pub vlan_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Replace filter Command 0x025F
// uses the i40e_aqc_replace_cloud_filters,
// and the generic indirect completion structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_filter_data {
    pub filter_type: u8,
    pub input: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_replace_cloud_filters_cmd {
    pub valid_flags: u8,
    pub old_filter_type: u8,
    pub new_filter_type: u8,
    pub tr_bit: u8,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_replace_cloud_filters_cmd_buf {
    pub data: [u8; 32],
    pub filters: [i40e_filter_data; 8],
}

// Add Mirror Rule (indirect or direct 0x0260)
// Delete Mirror Rule (indirect or direct 0x0261)
// note: some rule types (4,5) do not use an external buffer.
// take care to set the flags correctly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_delete_mirror_rule {
    pub seid: __le16,
    pub rule_type: __le16,
pub const I40E_AQC_MIRROR_RULE_TYPE_SHIFT: c_int = 0;

pub const I40E_AQC_MIRROR_RULE_TYPE_VLAN: c_int = 3;
pub const I40E_AQC_MIRROR_RULE_TYPE_ALL_INGRESS: c_int = 4;
pub const I40E_AQC_MIRROR_RULE_TYPE_ALL_EGRESS: c_int = 5;
    pub num_entries: __le16,
    pub /: *mut *mut __le16 destination; / VSI for add, rule id for delete,
    pub /: *mut *mut __le32 addr_high; / address of array of 2-byte VSI or VLAN ids,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_delete_mirror_rule_completion {
    pub reserved: [u8; 2],
    pub /: *mut *mut __le16 rule_id; / only used on add,
    pub mirror_rules_used: __le16,
    pub mirror_rules_free: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Dynamic Device Personalization
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_write_personalization_profile {
    pub flags: u8,
    pub reserved: [u8; 3],
    pub profile_track_id: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_write_ddp_resp {
    pub error_offset: __le32,
    pub error_info: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_applied_profiles {
    pub flags: u8,
    pub rsv: [u8; 3],
    pub reserved: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// DCB 0x03xx
// PFC Ignore (direct 0x0301)
// the command and response use the same descriptor structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_pfc_ignore {
    pub tc_bitmap: u8,
    pub /: *mut *mut u8 command_flags; / unused on response,
    pub reserved: [u8; 14],
}

// DCB Update (direct 0x0302) uses the i40e_aq_desc structure
// with no parameters
//
// TX scheduler 0x04xx
// Almost all the indirect commands use
// this generic struct to pass the SEID in param0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_tx_sched_ind {
    pub vsi_seid: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Several commands respond with a set of queue set handles
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_qs_handles_resp {
    pub qs_handles: [__le16; 8],
}

// Configure VSI BW limits (direct 0x0400)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_vsi_bw_limit {
    pub vsi_seid: __le16,
    pub reserved: [u8; 2],
    pub credit: __le16,
    pub reserved1: [u8; 2],
    pub /: *mut *mut u8 max_credit; / 0-3, limit = 2^max,
    pub reserved2: [u8; 7],
}

// Configure VSI Bandwidth Limit per Traffic Type (indirect 0x0406)
// responds with i40e_aqc_qs_handles_resp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_vsi_ets_sla_bw_data {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 15],
    pub /: *mut *mut __le16 tc_bw_credits[8]; / FW writesback QS handles here,
// 4 bits per tc 0-7, 4th bit is reserved, limit = 2^max
    pub tc_bw_max: [__le16; 2],
    pub reserved1: [u8; 28],
}

// Configure VSI Bandwidth Allocation per Traffic Type (indirect 0x0407)
// responds with i40e_aqc_qs_handles_resp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_vsi_tc_bw_data {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 3],
    pub tc_bw_credits: [u8; 8],
    pub reserved1: [u8; 4],
    pub qs_handles: [__le16; 8],
}

// Query vsi bw configuration (indirect 0x0408)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_query_vsi_bw_config_resp {
    pub tc_valid_bits: u8,
    pub tc_suspended_bits: u8,
    pub reserved: [u8; 14],
    pub qs_handles: [__le16; 8],
    pub reserved1: [u8; 4],
    pub port_bw_limit: __le16,
    pub reserved2: [u8; 2],
    pub /: *mut *mut u8 max_bw; / 0-3, limit = 2^max,
    pub reserved3: [u8; 23],
}

// Query VSI Bandwidth Allocation per Traffic Type (indirect 0x040A)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_query_vsi_ets_sla_config_resp {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 3],
    pub share_credits: [u8; 8],
    pub credits: [__le16; 8],
// 4 bits per tc 0-7, 4th bit is reserved, limit = 2^max
    pub tc_bw_max: [__le16; 2],
}

// Configure Switching Component Bandwidth Limit (direct 0x0410)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_switching_comp_bw_limit {
    pub seid: __le16,
    pub reserved: [u8; 2],
    pub credit: __le16,
    pub reserved1: [u8; 2],
    pub /: *mut *mut u8 max_bw; / 0-3, limit = 2^max,
    pub reserved2: [u8; 7],
}

// Enable  Physical Port ETS (indirect 0x0413)
// Modify  Physical Port ETS (indirect 0x0414)
// Disable Physical Port ETS (indirect 0x0415)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_switching_comp_ets_data {
    pub reserved: [u8; 4],
    pub tc_valid_bits: u8,
    pub seepage: u8,
    pub tc_strict_priority_flags: u8,
    pub reserved1: [u8; 17],
    pub tc_bw_share_credits: [u8; 8],
    pub reserved2: [u8; 96],
}

// Configure Switching Component Bandwidth Limits per Tc (indirect 0x0416)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_switching_comp_ets_bw_limit_data {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 15],
    pub tc_bw_credit: [__le16; 8],
// 4 bits per tc 0-7, 4th bit is reserved, limit = 2^max
    pub tc_bw_max: [__le16; 2],
    pub reserved1: [u8; 28],
}

// Configure Switching Component Bandwidth Allocation per Tc
// (indirect 0x0417)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_switching_comp_bw_config_data {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 2],
    pub /: *mut *mut u8 absolute_credits; / bool,
    pub tc_bw_share_credits: [u8; 8],
    pub reserved1: [u8; 20],
}

// Query Switching Component Configuration (indirect 0x0418)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_query_switching_comp_ets_config_resp {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 35],
    pub port_bw_limit: __le16,
    pub reserved1: [u8; 2],
    pub /: *mut *mut u8 tc_bw_max; / 0-3, limit = 2^max,
    pub reserved2: [u8; 23],
}

// Query PhysicalPort ETS Configuration (indirect 0x0419)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_query_port_ets_config_resp {
    pub reserved: [u8; 4],
    pub tc_valid_bits: u8,
    pub reserved1: u8,
    pub tc_strict_priority_bits: u8,
    pub reserved2: u8,
    pub tc_bw_share_credits: [u8; 8],
    pub tc_bw_limits: [__le16; 8],
// 4 bits per tc 0-7, 4th bit reserved, limit = 2^max
    pub tc_bw_max: [__le16; 2],
    pub reserved3: [u8; 32],
}

// Query Switching Component Bandwidth Allocation per Traffic Type
// (indirect 0x041A)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_query_switching_comp_bw_config_resp {
    pub tc_valid_bits: u8,
    pub reserved: [u8; 2],
    pub /: *mut *mut u8 absolute_credits_enable; / bool,
    pub tc_bw_share_credits: [u8; 8],
    pub tc_bw_limits: [__le16; 8],
// 4 bits per tc 0-7, 4th bit is reserved, limit = 2^max
    pub tc_bw_max: [__le16; 2],
}

// Suspend/resume port TX traffic
// (direct 0x041B and 0x041C) uses the generic SEID struct
//
// Configure partition BW
// (indirect 0x041D)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_configure_partition_bw_data {
    pub pf_valid_bits: __le16,
    pub /: *mut *mut u8 min_bw[16]; / guaranteed bandwidth,
    pub /: *mut *mut u8 max_bw[16]; / bandwidth limit,
}

// Get and set the active HMC resource profile and status.
// (direct 0x0500) and (direct 0x0501)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aq_get_set_hmc_resource_profile {
    pub pm_profile: u8,
    pub pe_vf_enabled: u8,
    pub reserved: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_aq_hmc_profile {
// I40E_HMC_PROFILE_NO_CHANGE	= 0, reserved
    I40E_HMC_PROFILE_DEFAULT	= 1,
    I40E_HMC_PROFILE_FAVOR_VF	= 2,
    I40E_HMC_PROFILE_EQUAL		= 3,
}

// Get PHY Abilities (indirect 0x0600) uses the generic indirect struct
// set in param0 for get phy abilities to report qualified modules
pub const I40E_AQ_PHY_REPORT_QUALIFIED_MODULES: c_uint = 0x0001;
pub const I40E_AQ_PHY_REPORT_INITIAL_VALUES: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_aq_phy_type {
    I40E_PHY_TYPE_SGMII			= 0x0,
    I40E_PHY_TYPE_1000BASE_KX		= 0x1,
    I40E_PHY_TYPE_10GBASE_KX4		= 0x2,
    I40E_PHY_TYPE_10GBASE_KR		= 0x3,
    I40E_PHY_TYPE_40GBASE_KR4		= 0x4,
    I40E_PHY_TYPE_XAUI			= 0x5,
    I40E_PHY_TYPE_XFI			= 0x6,
    I40E_PHY_TYPE_SFI			= 0x7,
    I40E_PHY_TYPE_XLAUI			= 0x8,
    I40E_PHY_TYPE_XLPPI			= 0x9,
    I40E_PHY_TYPE_40GBASE_CR4_CU		= 0xA,
    I40E_PHY_TYPE_10GBASE_CR1_CU		= 0xB,
    I40E_PHY_TYPE_10GBASE_AOC		= 0xC,
    I40E_PHY_TYPE_40GBASE_AOC		= 0xD,
    I40E_PHY_TYPE_UNRECOGNIZED		= 0xE,
    I40E_PHY_TYPE_UNSUPPORTED		= 0xF,
    I40E_PHY_TYPE_100BASE_TX		= 0x11,
    I40E_PHY_TYPE_1000BASE_T		= 0x12,
    I40E_PHY_TYPE_10GBASE_T			= 0x13,
    I40E_PHY_TYPE_10GBASE_SR		= 0x14,
    I40E_PHY_TYPE_10GBASE_LR		= 0x15,
    I40E_PHY_TYPE_10GBASE_SFPP_CU		= 0x16,
    I40E_PHY_TYPE_10GBASE_CR1		= 0x17,
    I40E_PHY_TYPE_40GBASE_CR4		= 0x18,
    I40E_PHY_TYPE_40GBASE_SR4		= 0x19,
    I40E_PHY_TYPE_40GBASE_LR4		= 0x1A,
    I40E_PHY_TYPE_1000BASE_SX		= 0x1B,
    I40E_PHY_TYPE_1000BASE_LX		= 0x1C,
    I40E_PHY_TYPE_1000BASE_T_OPTICAL	= 0x1D,
    I40E_PHY_TYPE_20GBASE_KR2		= 0x1E,
    I40E_PHY_TYPE_25GBASE_KR		= 0x1F,
    I40E_PHY_TYPE_25GBASE_CR		= 0x20,
    I40E_PHY_TYPE_25GBASE_SR		= 0x21,
    I40E_PHY_TYPE_25GBASE_LR		= 0x22,
    I40E_PHY_TYPE_25GBASE_AOC		= 0x23,
    I40E_PHY_TYPE_25GBASE_ACC		= 0x24,
    I40E_PHY_TYPE_2_5GBASE_T		= 0x26,
    I40E_PHY_TYPE_5GBASE_T			= 0x27,
    I40E_PHY_TYPE_2_5GBASE_T_LINK_STATUS	= 0x30,
    I40E_PHY_TYPE_5GBASE_T_LINK_STATUS	= 0x31,
    I40E_PHY_TYPE_MAX,
    I40E_PHY_TYPE_NOT_SUPPORTED_HIGH_TEMP	= 0xFD,
    I40E_PHY_TYPE_EMPTY			= 0xFE,
    I40E_PHY_TYPE_DEFAULT			= 0xFF,
}

pub const I40E_LINK_SPEED_2_5GB_SHIFT: c_uint = 0x0;
pub const I40E_LINK_SPEED_100MB_SHIFT: c_uint = 0x1;
pub const I40E_LINK_SPEED_1000MB_SHIFT: c_uint = 0x2;
pub const I40E_LINK_SPEED_10GB_SHIFT: c_uint = 0x3;
pub const I40E_LINK_SPEED_40GB_SHIFT: c_uint = 0x4;
pub const I40E_LINK_SPEED_20GB_SHIFT: c_uint = 0x5;
pub const I40E_LINK_SPEED_25GB_SHIFT: c_uint = 0x6;
pub const I40E_LINK_SPEED_5GB_SHIFT: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_aq_link_speed {
    I40E_LINK_SPEED_UNKNOWN	= 0,
    I40E_LINK_SPEED_100MB	= BIT(I40E_LINK_SPEED_100MB_SHIFT),
    I40E_LINK_SPEED_1GB	= BIT(I40E_LINK_SPEED_1000MB_SHIFT),
    I40E_LINK_SPEED_2_5GB	= (1 << I40E_LINK_SPEED_2_5GB_SHIFT),
    I40E_LINK_SPEED_5GB	= (1 << I40E_LINK_SPEED_5GB_SHIFT),
    I40E_LINK_SPEED_10GB	= BIT(I40E_LINK_SPEED_10GB_SHIFT),
    I40E_LINK_SPEED_40GB	= BIT(I40E_LINK_SPEED_40GB_SHIFT),
    I40E_LINK_SPEED_20GB	= BIT(I40E_LINK_SPEED_20GB_SHIFT),
    I40E_LINK_SPEED_25GB	= BIT(I40E_LINK_SPEED_25GB_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_module_desc {
    pub oui: [u8; 3],
    pub reserved1: u8,
    pub part_number: [u8; 16],
    pub revision: [u8; 4],
    pub reserved2: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aq_get_phy_abilities_resp {
    pub /: *mut *mut __le32 phy_type; / bitmap using the above enum for offsets,
    pub /: *mut *mut u8 link_speed; / bitmap using the above enum bit patterns,
    pub abilities: u8,
pub const I40E_AQ_PHY_FLAG_PAUSE_TX: c_uint = 0x01;
pub const I40E_AQ_PHY_FLAG_PAUSE_RX: c_uint = 0x02;
    pub eee_capability: __le16,
    pub eeer_val: __le32,
    pub d3_lpan: u8,
    pub phy_type_ext: u8,

pub const I40E_AQ_PHY_TYPE_EXT_25G_SR: c_uint = 0x04;
pub const I40E_AQ_PHY_TYPE_EXT_25G_LR: c_uint = 0x08;
    pub fec_cfg_curr_mod_ext_info: u8,
pub const I40E_AQ_REQUEST_FEC_KR: c_uint = 0x04;
pub const I40E_AQ_REQUEST_FEC_RS: c_uint = 0x08;
pub const I40E_AQ_ENABLE_FEC_AUTO: c_uint = 0x10;
    pub ext_comp_code: u8,
    pub phy_id: [u8; 4],
    pub module_type: [u8; 3],
    pub qualified_module_count: u8,
pub const I40E_AQ_PHY_MAX_QMS: c_int = 16;
    pub qualified_module: [i40e_aqc_module_desc; I40E_AQ_PHY_MAX_QMS],
}

// Set PHY Config (direct 0x0601)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aq_set_phy_config {
    pub phy_type: __le32,
    pub link_speed: u8,
    pub abilities: u8,
// bits 0-2 use the values from get_phy_abilities_resp
pub const I40E_AQ_PHY_ENABLE_LINK: c_uint = 0x08;
pub const I40E_AQ_PHY_ENABLE_AN: c_uint = 0x10;
pub const I40E_AQ_PHY_ENABLE_ATOMIC_LINK: c_uint = 0x20;
    pub eee_capability: __le16,
    pub eeer: __le32,
    pub low_power_ctrl: u8,
    pub phy_type_ext: u8,

pub const I40E_AQ_PHY_TYPE_EXT_25G_SR: c_uint = 0x04;
pub const I40E_AQ_PHY_TYPE_EXT_25G_LR: c_uint = 0x08;
    pub fec_config: u8,

pub const I40E_AQ_PHY_FEC_CONFIG_SHIFT: c_uint = 0x0;

    pub reserved: u8,
}

// Set MAC Config command data structure (direct 0x0603)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aq_set_mac_config {
    pub max_frame_size: __le16,
    pub params: u8,

    pub /: *mut *mut u8 tx_timer_priority; / bitmap,
    pub tx_timer_value: __le16,
    pub fc_refresh_threshold: __le16,
    pub reserved: [u8; 8],
}

// Restart Auto-Negotiation (direct 0x605)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_link_restart_an {
    pub command: u8,
pub const I40E_AQ_PHY_RESTART_AN: c_uint = 0x02;
pub const I40E_AQ_PHY_LINK_ENABLE: c_uint = 0x04;
    pub reserved: [u8; 15],
}

// Get Link Status cmd & response data structure (direct 0x0607)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_link_status {
    pub /: *mut *mut __le16 command_flags; / only field set on command,
pub const I40E_AQ_LSE_DISABLE: c_uint = 0x2;
pub const I40E_AQ_LSE_ENABLE: c_uint = 0x3;
// only response uses this flag
pub const I40E_AQ_LSE_IS_ENABLED: c_uint = 0x1;
    pub /: *mut *mut u8 phy_type; / i40e_aq_phy_type,
    pub /: *mut *mut u8 link_speed; / i40e_aq_link_speed,
    pub link_info: u8,
pub const I40E_AQ_LINK_UP: c_uint = 0x01    /* obsolete */;
pub const I40E_AQ_MEDIA_AVAILABLE: c_uint = 0x40;
    pub an_info: u8,
pub const I40E_AQ_AN_COMPLETED: c_uint = 0x01;
pub const I40E_AQ_LINK_PAUSE_TX: c_uint = 0x20;
pub const I40E_AQ_LINK_PAUSE_RX: c_uint = 0x40;
pub const I40E_AQ_QUALIFIED_MODULE: c_uint = 0x80;
    pub ext_info: u8,
    pub /: *mut *mut u8 loopback; / use defines from i40e_aqc_set_lb_mode,
// Since firmware API 1.7 loopback field keeps power class info as well
pub const I40E_AQ_LOOPBACK_MASK: c_uint = 0x07;
    pub max_frame_size: __le16,
    pub config: u8,
pub const I40E_AQ_CONFIG_FEC_KR_ENA: c_uint = 0x01;
pub const I40E_AQ_CONFIG_FEC_RS_ENA: c_uint = 0x02;
pub const I40E_AQ_CONFIG_CRC_ENA: c_uint = 0x04;
pub const I40E_AQ_CONFIG_PACING_MASK: c_uint = 0x78;
    pub power_desc: u8,
    pub reserved: [u8; 4],
}

// Set event mask command (direct 0x613)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_phy_int_mask {
    pub reserved: [u8; 8],
    pub event_mask: __le16,
pub const I40E_AQ_EVENT_LINK_UPDOWN: c_uint = 0x0002;
pub const I40E_AQ_EVENT_MEDIA_NA: c_uint = 0x0004;
pub const I40E_AQ_EVENT_MODULE_QUAL_FAIL: c_uint = 0x0100;
    pub reserved1: [u8; 6],
}

// Get Local AN advt register (direct 0x0614)
// Set Local AN advt register (direct 0x0615)
// Get Link Partner AN advt register (direct 0x0616)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_an_advt_reg {
    pub local_an_reg0: __le32,
    pub local_an_reg1: __le16,
    pub reserved: [u8; 10],
}

// Set Loopback mode (0x0618)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_lb_mode {
    pub lb_mode: __le16,
pub const I40E_LEGACY_LOOPBACK_NVM_VER: c_uint = 0x6000;
pub const I40E_AQ_LB_MAC_LOCAL: c_uint = 0x01;
pub const I40E_AQ_LB_PHY_LOCAL: c_uint = 0x05;
pub const I40E_AQ_LB_PHY_REMOTE: c_uint = 0x06;
pub const I40E_AQ_LB_MAC_LOCAL_LEGACY: c_uint = 0x04;
    pub reserved: [u8; 14],
}

// Set PHY Debug command (0x0622)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_phy_debug {
    pub command_flags: u8,
// Disable link manageability on a single port
pub const I40E_AQ_PHY_DEBUG_DISABLE_LINK_FW: c_uint = 0x10;
// Disable link manageability on all ports
pub const I40E_AQ_PHY_DEBUG_DISABLE_ALL_LINK_FW: c_uint = 0x20;
    pub reserved: [u8; 15],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_aq_phy_reg_type {
    I40E_AQC_PHY_REG_INTERNAL	= 0x1,
    I40E_AQC_PHY_REG_EXERNAL_BASET	= 0x2,
    I40E_AQC_PHY_REG_EXERNAL_MODULE	= 0x3
}

// Run PHY Activity (0x0626)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_run_phy_activity {
    pub activity_id: __le16,
    pub flags: u8,
    pub reserved1: u8,
    pub control: __le32,
    pub data: __le32,
    pub reserved2: [u8; 4],
}

// Set PHY Register command (0x0628)
// Get PHY Register command (0x0629)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_phy_register_access {
    pub phy_interface: u8,
pub const I40E_AQ_PHY_REG_ACCESS_EXTERNAL: c_int = 1;
pub const I40E_AQ_PHY_REG_ACCESS_EXTERNAL_MODULE: c_int = 2;
    pub dev_address: u8,
    pub cmd_flags: u8,
pub const I40E_AQ_PHY_REG_ACCESS_DONT_CHANGE_QSFP_PAGE: c_uint = 0x01;
pub const I40E_AQ_PHY_REG_ACCESS_SET_MDIO_IF_NUMBER: c_uint = 0x02;
pub const I40E_AQ_PHY_REG_ACCESS_MDIO_IF_NUMBER_SHIFT: c_int = 2;

    pub reserved1: u8,
    pub reg_address: __le32,
    pub reg_value: __le32,
    pub reserved2: [u8; 4],
}

// NVM Read command (indirect 0x0701)
// NVM Erase commands (direct 0x0702)
// NVM Update commands (indirect 0x0703)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_update {
    pub command_flags: u8,
pub const I40E_AQ_NVM_LAST_CMD: c_uint = 0x01;
pub const I40E_AQ_NVM_REARRANGE_TO_FLAT: c_uint = 0x20;
pub const I40E_AQ_NVM_REARRANGE_TO_STRUCT: c_uint = 0x40;
pub const I40E_AQ_NVM_PRESERVATION_FLAGS_SHIFT: c_int = 1;
pub const I40E_AQ_NVM_PRESERVATION_FLAGS_SELECTED: c_uint = 0x03;
pub const I40E_AQ_NVM_PRESERVATION_FLAGS_ALL: c_uint = 0x01;
    pub module_pointer: u8,
    pub length: __le16,
    pub offset: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// NVM Config Read (indirect 0x0704)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_config_read {
    pub cmd_flags: __le16,
    pub element_count: __le16,
    pub /: *mut *mut __le16 element_id; / Feature/field ID,
    pub /: *mut *mut __le16 element_id_msw; / MSWord of field ID,
    pub address_high: __le32,
    pub address_low: __le32,
}

// NVM Config Write (indirect 0x0705)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_config_write {
    pub cmd_flags: __le16,
    pub element_count: __le16,
    pub reserved: [u8; 4],
    pub address_high: __le32,
    pub address_low: __le32,
}

// Used for 0x0704 as well as for 0x0705 commands
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_config_data_feature {
    pub feature_id: __le16,
    pub feature_options: __le16,
    pub feature_selection: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_config_data_immediate_field {
    pub field_id: __le32,
    pub field_value: __le32,
    pub field_options: __le16,
    pub reserved: __le16,
}

// OEM Post Update (indirect 0x0720)
// no command data struct used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_oem_post_update {
    pub sel_data: u8,
    pub reserved: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_nvm_oem_post_update_buffer {
    pub str_len: u8,
    pub dev_addr: u8,
    pub eeprom_addr: __le16,
    pub data: [u8; 36],
}

// Thermal Sensor (indirect 0x0721)
// read or set thermal sensor configs and values
// takes a sensor and command specific data buffer, not detailed here
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_thermal_sensor {
    pub sensor_action: u8,
    pub reserved: [u8; 7],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Send to PF command (indirect 0x0801) id is only used by PF
// Send to VF command (indirect 0x0802) id is only used by PF
// Send to Peer PF command (indirect 0x0803)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_pf_vf_message {
    pub id: __le32,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Alternate structure
// Direct write (direct 0x0900)
// Direct read (direct 0x0902)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_alternate_write {
    pub address0: __le32,
    pub data0: __le32,
    pub address1: __le32,
    pub data1: __le32,
}

// Indirect write (indirect 0x0901)
// Indirect read (indirect 0x0903)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_alternate_ind_write {
    pub address: __le32,
    pub length: __le32,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Done alternate write (direct 0x0904)
// uses i40e_aq_desc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_alternate_write_done {
    pub cmd_flags: __le16,
    pub reserved: [u8; 14],
}

// Set OEM mode (direct 0x0905)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_alternate_set_mode {
    pub mode: __le32,
    pub reserved: [u8; 12],
}

// Clear port Alternate RAM (direct 0x0906) uses i40e_aq_desc
// async events 0x10xx
// Lan Queue Overflow Event (direct, 0x1001)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lan_overflow {
    pub prtdcb_rupto: __le32,
    pub otx_ctl: __le32,
    pub reserved: [u8; 8],
}

// Get LLDP MIB (indirect 0x0A00)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_get_mib {
    pub type: u8,
    pub reserved1: u8,
pub const I40E_AQ_LLDP_MIB_TYPE_MASK: c_uint = 0x3;
pub const I40E_AQ_LLDP_MIB_LOCAL: c_uint = 0x0;
pub const I40E_AQ_LLDP_MIB_REMOTE: c_uint = 0x1;
pub const I40E_AQ_LLDP_BRIDGE_TYPE_MASK: c_uint = 0xC;
pub const I40E_AQ_LLDP_BRIDGE_TYPE_SHIFT: c_uint = 0x2;
pub const I40E_AQ_LLDP_BRIDGE_TYPE_NEAREST_BRIDGE: c_uint = 0x0;
// TX pause flags use I40E_AQ_LINK_TX_* above
    pub local_len: __le16,
    pub remote_len: __le16,
    pub reserved2: [u8; 2],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Configure LLDP MIB Change Event (direct 0x0A01)
// also used for the event (with type in the command field)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_update_mib {
    pub command: u8,
pub const I40E_AQ_LLDP_MIB_UPDATE_DISABLE: c_uint = 0x1;
    pub reserved: [u8; 7],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Add LLDP TLV (indirect 0x0A02)
// Delete LLDP TLV (indirect 0x0A04)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_add_tlv {
    pub /: *mut *mut u8 type; / only nearest bridge and non-TPMR from 0x0A00,
    pub reserved1: [u8; 1],
    pub len: __le16,
    pub reserved2: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Update LLDP TLV (indirect 0x0A03)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_update_tlv {
    pub /: *mut *mut u8 type; / only nearest bridge and non-TPMR from 0x0A00,
    pub reserved: u8,
    pub old_len: __le16,
    pub new_offset: __le16,
    pub new_len: __le16,
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// Stop LLDP (direct 0x0A05)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_stop {
    pub command: u8,
pub const I40E_AQ_LLDP_AGENT_SHUTDOWN: c_uint = 0x1;
pub const I40E_AQ_LLDP_AGENT_STOP_PERSIST: c_uint = 0x2;
    pub reserved: [u8; 15],
}

// Start LLDP (direct 0x0A06)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_start {
    pub command: u8,
pub const I40E_AQ_LLDP_AGENT_START: c_uint = 0x1;
pub const I40E_AQ_LLDP_AGENT_START_PERSIST: c_uint = 0x2;
    pub reserved: [u8; 15],
}

// Set DCB (direct 0x0303)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_set_dcb_parameters {
    pub command: u8,
pub const I40E_AQ_DCB_SET_AGENT: c_uint = 0x1;
pub const I40E_DCB_VALID: c_uint = 0x1;
    pub valid_flags: u8,
    pub reserved: [u8; 14],
}

// Get CEE DCBX Oper Config (0x0A07)
// uses the generic descriptor struct
// returns below as indirect response
//
pub const I40E_AQC_CEE_APP_FCOE_SHIFT: c_uint = 0x0;

pub const I40E_AQC_CEE_APP_ISCSI_SHIFT: c_uint = 0x3;

pub const I40E_AQC_CEE_APP_FIP_SHIFT: c_uint = 0x8;

pub const I40E_AQC_CEE_PG_STATUS_SHIFT: c_uint = 0x0;

pub const I40E_AQC_CEE_PFC_STATUS_SHIFT: c_uint = 0x3;

pub const I40E_AQC_CEE_APP_STATUS_SHIFT: c_uint = 0x8;

pub const I40E_AQC_CEE_FCOE_STATUS_SHIFT: c_uint = 0x8;

pub const I40E_AQC_CEE_ISCSI_STATUS_SHIFT: c_uint = 0xB;

pub const I40E_AQC_CEE_FIP_STATUS_SHIFT: c_uint = 0x10;

// struct i40e_aqc_get_cee_dcb_cfg_v1_resp was originally defined with
// word boundary layout issues, which the Linux compilers silently deal
// with by adding padding, making the actual struct larger than designed.
// However, the FW compiler for the NIC is less lenient and complains
// about the struct.  Hence, the struct defined here has an extra byte in
// fields reserved3 and reserved4 to directly acknowledge that padding,
// and the new length is used in the length check macro.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_cee_dcb_cfg_v1_resp {
    pub reserved1: u8,
    pub oper_num_tc: u8,
    pub oper_prio_tc: [u8; 4],
    pub reserved2: u8,
    pub oper_tc_bw: [u8; 8],
    pub oper_pfc_en: u8,
    pub reserved3: [u8; 2],
    pub oper_app_prio: __le16,
    pub reserved4: [u8; 2],
    pub tlv_status: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_cee_dcb_cfg_resp {
    pub oper_num_tc: u8,
    pub oper_prio_tc: [u8; 4],
    pub oper_tc_bw: [u8; 8],
    pub oper_pfc_en: u8,
    pub oper_app_prio: __le16,
pub const I40E_AQC_CEE_APP_FCOE_SHIFT: c_uint = 0x0;

pub const I40E_AQC_CEE_APP_ISCSI_SHIFT: c_uint = 0x3;

pub const I40E_AQC_CEE_APP_FIP_SHIFT: c_uint = 0x8;

    pub tlv_status: __le32,
pub const I40E_AQC_CEE_PG_STATUS_SHIFT: c_uint = 0x0;

pub const I40E_AQC_CEE_PFC_STATUS_SHIFT: c_uint = 0x3;

pub const I40E_AQC_CEE_APP_STATUS_SHIFT: c_uint = 0x8;
    pub reserved: [u8; 12],
}

// Set Local LLDP MIB (indirect 0x0A08)
// Used to replace the local MIB of a given LLDP agent. e.g. DCBx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_set_local_mib {
pub const SET_LOCAL_MIB_AC_TYPE_DCBX_SHIFT: c_int = 0;

pub const SET_LOCAL_MIB_AC_TYPE_LOCAL_MIB: c_uint = 0x0;

pub const SET_LOCAL_MIB_AC_TYPE_NON_WILLING_APPS: c_uint = 0x1;
    pub type: u8,
    pub reserved0: u8,
    pub length: __le16,
    pub reserved1: [u8; 4],
    pub address_high: __le32,
    pub address_low: __le32,
}

// Stop/Start LLDP Agent (direct 0x0A09)
// Used for stopping/starting specific LLDP agent. e.g. DCBx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_stop_start_specific_agent {
    pub command: u8,
    pub reserved: [u8; 15],
}

// Restore LLDP Agent factory settings (direct 0x0A0A)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_lldp_restore {
    pub command: u8,
pub const I40E_AQ_LLDP_AGENT_RESTORE: c_uint = 0x1;
    pub reserved: [u8; 15],
}

// Add Udp Tunnel command and completion (direct 0x0B00)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_udp_tunnel {
    pub udp_port: __le16,
    pub reserved0: [u8; 3],
    pub protocol_type: u8,
pub const I40E_AQC_TUNNEL_TYPE_VXLAN: c_uint = 0x00;
pub const I40E_AQC_TUNNEL_TYPE_NGE: c_uint = 0x01;
    pub reserved1: [u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_add_udp_tunnel_completion {
    pub udp_port: __le16,
    pub filter_entry_index: u8,
    pub multiple_pfs: u8,
    pub total_filters: u8,
    pub reserved: [u8; 11],
}

// remove UDP Tunnel command (0x0B01)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_remove_udp_tunnel {
    pub reserved: [u8; 2],
    pub /: *mut *mut u8 index; / 0 to 15,
    pub reserved2: [u8; 13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_del_udp_tunnel_completion {
    pub udp_port: __le16,
    pub /: *mut *mut u8 index; / 0 to 15,
    pub multiple_pfs: u8,
    pub total_filters_used: u8,
    pub reserved1: [u8; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_set_rss_key {

pub const I40E_AQC_SET_RSS_KEY_VSI_ID_SHIFT: c_int = 0;

    pub vsi_id: __le16,
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_set_rss_key_data {
    pub standard_rss_key: [u8; 0x28],
    pub extended_hash_key: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_get_set_rss_lut {

pub const I40E_AQC_SET_RSS_LUT_VSI_ID_SHIFT: c_int = 0;

    pub vsi_id: __le16,
pub const I40E_AQC_SET_RSS_LUT_TABLE_TYPE_SHIFT: c_int = 0;

pub const I40E_AQC_SET_RSS_LUT_TABLE_TYPE_VSI: c_int = 0;
pub const I40E_AQC_SET_RSS_LUT_TABLE_TYPE_PF: c_int = 1;
    pub flags: __le16,
    pub reserved: [u8; 4],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

// tunnel key structure 0x0B10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_tunnel_key_structure {
    pub key1_off: u8,
    pub key2_off: u8,
    pub /: *mut *mut u8 key1_len; / 0 to 15,
    pub /: *mut *mut u8 key2_len; / 0 to 15,
    pub flags: u8,
    pub network_key_index: u8,
    pub reserved: [u8; 10],
}

// OEM mode commands (direct 0xFE0x)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_oem_param_change {
    pub param_type: __le32,
    pub param_value1: __le32,
    pub param_value2: __le16,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_oem_state_change {
    pub state: __le32,
    pub reserved: [u8; 12],
}

// Initialize OCSD (0xFE02, direct)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_opc_oem_ocsd_initialize {
    pub type_status: u8,
    pub reserved1: [u8; 3],
    pub ocsd_memory_block_addr_high: __le32,
    pub ocsd_memory_block_addr_low: __le32,
    pub requested_update_interval: __le32,
}

// Initialize OCBB  (0xFE03, direct)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_opc_oem_ocbb_initialize {
    pub type_status: u8,
    pub reserved1: [u8; 3],
    pub ocbb_memory_block_addr_high: __le32,
    pub ocbb_memory_block_addr_low: __le32,
    pub reserved2: [u8; 4],
}

// debug commands
// get device id (0xFF00) uses the generic structure
// set test more (0xFF01, internal)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_acq_set_test_mode {
    pub mode: u8,
    pub reserved: [u8; 3],
    pub command: u8,
    pub reserved2: [u8; 3],
    pub address_high: __le32,
    pub address_low: __le32,
}

// Debug Read Register command (0xFF03)
// Debug Write Register command (0xFF04)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_debug_reg_read_write {
    pub reserved: __le32,
    pub address: __le32,
    pub value_high: __le32,
    pub value_low: __le32,
}

// Scatter/gather Reg Read  (indirect 0xFF05)
// Scatter/gather Reg Write (indirect 0xFF06)
//
// i40e_aq_desc is used for the command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_debug_reg_sg_element_data {
    pub address: __le32,
    pub value: __le32,
}

// Debug Modify register (direct 0xFF07)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_debug_modify_reg {
    pub address: __le32,
    pub value: __le32,
    pub clear_mask: __le32,
    pub set_mask: __le32,
}

// dump internal data (0xFF08, indirect)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_debug_dump_internals {
    pub cluster_id: u8,
    pub table_id: u8,
    pub data_size: __le16,
    pub idx: __le32,
    pub address_high: __le32,
    pub address_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_aqc_debug_modify_internals {
    pub cluster_id: u8,
    pub cluster_specific_params: [u8; 7],
    pub address_high: __le32,
    pub address_low: __le32,
}
