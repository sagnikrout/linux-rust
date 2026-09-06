//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nfc/nci.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// The NFC Controller Interface is the communication protocol between an
// NFC Controller (NFCC) and a Device Host (DH).
//
// Copyright (C) 2014 Marvell International Ltd.
// Copyright (C) 2011 Texas Instruments, Inc.
//
// Written by Ilan Elias <ilane@ti.com>
//
// Acknowledgements:
// This file is based on hci.h, which was written
// by Maxim Krasnyansky.
//

// NCI constants
pub const NCI_MAX_NUM_MAPPING_CONFIGS: c_int = 10;
pub const NCI_MAX_NUM_RF_CONFIGS: c_int = 10;
pub const NCI_MAX_NUM_CONN: c_int = 10;
pub const NCI_MAX_PARAM_LEN: c_int = 251;
pub const NCI_MAX_PAYLOAD_SIZE: c_int = 255;
pub const NCI_MAX_PACKET_SIZE: c_int = 258;
pub const NCI_MAX_LARGE_PARAMS_NCI_v2: c_int = 15;
pub const NCI_VER_2_MASK: c_uint = 0x20;
// NCI Status Codes
pub const NCI_STATUS_OK: c_uint = 0x00;
pub const NCI_STATUS_REJECTED: c_uint = 0x01;
pub const NCI_STATUS_RF_FRAME_CORRUPTED: c_uint = 0x02;
pub const NCI_STATUS_FAILED: c_uint = 0x03;
pub const NCI_STATUS_NOT_INITIALIZED: c_uint = 0x04;
pub const NCI_STATUS_SYNTAX_ERROR: c_uint = 0x05;
pub const NCI_STATUS_SEMANTIC_ERROR: c_uint = 0x06;
pub const NCI_STATUS_UNKNOWN_GID: c_uint = 0x07;
pub const NCI_STATUS_UNKNOWN_OID: c_uint = 0x08;
pub const NCI_STATUS_INVALID_PARAM: c_uint = 0x09;
pub const NCI_STATUS_MESSAGE_SIZE_EXCEEDED: c_uint = 0x0a;
// Discovery Specific Status Codes
pub const NCI_STATUS_DISCOVERY_ALREADY_STARTED: c_uint = 0xa0;
pub const NCI_STATUS_DISCOVERY_TARGET_ACTIVATION_FAILED: c_uint = 0xa1;
pub const NCI_STATUS_DISCOVERY_TEAR_DOWN: c_uint = 0xa2;
// RF Interface Specific Status Codes
pub const NCI_STATUS_RF_TRANSMISSION_ERROR: c_uint = 0xb0;
pub const NCI_STATUS_RF_PROTOCOL_ERROR: c_uint = 0xb1;
pub const NCI_STATUS_RF_TIMEOUT_ERROR: c_uint = 0xb2;
// NFCEE Interface Specific Status Codes
pub const NCI_STATUS_NFCEE_INTERFACE_ACTIVATION_FAILED: c_uint = 0xc0;
pub const NCI_STATUS_NFCEE_TRANSMISSION_ERROR: c_uint = 0xc1;
pub const NCI_STATUS_NFCEE_PROTOCOL_ERROR: c_uint = 0xc2;
pub const NCI_STATUS_NFCEE_TIMEOUT_ERROR: c_uint = 0xc3;
// NFCEE Interface/Protocols
pub const NCI_NFCEE_INTERFACE_APDU: c_uint = 0x00;
pub const NCI_NFCEE_INTERFACE_HCI_ACCESS: c_uint = 0x01;
pub const NCI_NFCEE_INTERFACE_TYPE3_CMD_SET: c_uint = 0x02;
pub const NCI_NFCEE_INTERFACE_TRANSPARENT: c_uint = 0x03;
// Destination type
pub const NCI_DESTINATION_NFCC_LOOPBACK: c_uint = 0x01;
pub const NCI_DESTINATION_REMOTE_NFC_ENDPOINT: c_uint = 0x02;
pub const NCI_DESTINATION_NFCEE: c_uint = 0x03;
// Destination-specific parameters type
pub const NCI_DESTINATION_SPECIFIC_PARAM_RF_TYPE: c_uint = 0x00;
pub const NCI_DESTINATION_SPECIFIC_PARAM_NFCEE_TYPE: c_uint = 0x01;
// NFCEE Discovery Action
pub const NCI_NFCEE_DISCOVERY_ACTION_DISABLE: c_uint = 0x00;
pub const NCI_NFCEE_DISCOVERY_ACTION_ENABLE: c_uint = 0x01;
// NCI RF Technology and Mode
pub const NCI_NFC_A_PASSIVE_POLL_MODE: c_uint = 0x00;
pub const NCI_NFC_B_PASSIVE_POLL_MODE: c_uint = 0x01;
pub const NCI_NFC_F_PASSIVE_POLL_MODE: c_uint = 0x02;
pub const NCI_NFC_A_ACTIVE_POLL_MODE: c_uint = 0x03;
pub const NCI_NFC_F_ACTIVE_POLL_MODE: c_uint = 0x05;
pub const NCI_NFC_V_PASSIVE_POLL_MODE: c_uint = 0x06;
pub const NCI_NFC_A_PASSIVE_LISTEN_MODE: c_uint = 0x80;
pub const NCI_NFC_B_PASSIVE_LISTEN_MODE: c_uint = 0x81;
pub const NCI_NFC_F_PASSIVE_LISTEN_MODE: c_uint = 0x82;
pub const NCI_NFC_A_ACTIVE_LISTEN_MODE: c_uint = 0x83;
pub const NCI_NFC_F_ACTIVE_LISTEN_MODE: c_uint = 0x85;
pub const NCI_RF_TECH_MODE_LISTEN_MASK: c_uint = 0x80;
// NCI RF Technologies
pub const NCI_NFC_RF_TECHNOLOGY_A: c_uint = 0x00;
pub const NCI_NFC_RF_TECHNOLOGY_B: c_uint = 0x01;
pub const NCI_NFC_RF_TECHNOLOGY_F: c_uint = 0x02;
pub const NCI_NFC_RF_TECHNOLOGY_V: c_uint = 0x03;
// NCI Bit Rates
pub const NCI_NFC_BIT_RATE_106: c_uint = 0x00;
pub const NCI_NFC_BIT_RATE_212: c_uint = 0x01;
pub const NCI_NFC_BIT_RATE_424: c_uint = 0x02;
pub const NCI_NFC_BIT_RATE_848: c_uint = 0x03;
pub const NCI_NFC_BIT_RATE_1695: c_uint = 0x04;
pub const NCI_NFC_BIT_RATE_3390: c_uint = 0x05;
pub const NCI_NFC_BIT_RATE_6780: c_uint = 0x06;
pub const NCI_NFC_BIT_RATE_26: c_uint = 0x20;
// NCI RF Protocols
pub const NCI_RF_PROTOCOL_UNKNOWN: c_uint = 0x00;
pub const NCI_RF_PROTOCOL_T1T: c_uint = 0x01;
pub const NCI_RF_PROTOCOL_T2T: c_uint = 0x02;
pub const NCI_RF_PROTOCOL_T3T: c_uint = 0x03;
pub const NCI_RF_PROTOCOL_ISO_DEP: c_uint = 0x04;
pub const NCI_RF_PROTOCOL_NFC_DEP: c_uint = 0x05;
pub const NCI_RF_PROTOCOL_T5T: c_uint = 0x06;
// NCI RF Interfaces
pub const NCI_RF_INTERFACE_NFCEE_DIRECT: c_uint = 0x00;
pub const NCI_RF_INTERFACE_FRAME: c_uint = 0x01;
pub const NCI_RF_INTERFACE_ISO_DEP: c_uint = 0x02;
pub const NCI_RF_INTERFACE_NFC_DEP: c_uint = 0x03;
// NCI Configuration Parameter Tags
pub const NCI_PN_ATR_REQ_GEN_BYTES: c_uint = 0x29;
pub const NCI_LN_ATR_RES_GEN_BYTES: c_uint = 0x61;
pub const NCI_LA_SEL_INFO: c_uint = 0x32;
pub const NCI_LF_PROTOCOL_TYPE: c_uint = 0x50;
pub const NCI_LF_CON_BITR_F: c_uint = 0x54;
// NCI Configuration Parameters masks
pub const NCI_LA_SEL_INFO_ISO_DEP_MASK: c_uint = 0x20;
pub const NCI_LA_SEL_INFO_NFC_DEP_MASK: c_uint = 0x40;
pub const NCI_LF_PROTOCOL_TYPE_NFC_DEP_MASK: c_uint = 0x02;
pub const NCI_LF_CON_BITR_F_212: c_uint = 0x02;
pub const NCI_LF_CON_BITR_F_424: c_uint = 0x04;
// NCI 2.x Feature Enable Bit
pub const NCI_FEATURE_DISABLE: c_uint = 0x00;
// NCI Reset types
pub const NCI_RESET_TYPE_KEEP_CONFIG: c_uint = 0x00;
pub const NCI_RESET_TYPE_RESET_CONFIG: c_uint = 0x01;
// NCI Static RF connection ID
pub const NCI_STATIC_RF_CONN_ID: c_uint = 0x00;
// NCI Data Flow Control
pub const NCI_DATA_FLOW_CONTROL_NOT_USED: c_uint = 0xff;
// NCI RF_DISCOVER_MAP_CMD modes
pub const NCI_DISC_MAP_MODE_POLL: c_uint = 0x01;
pub const NCI_DISC_MAP_MODE_LISTEN: c_uint = 0x02;
// NCI Discover Notification Type
pub const NCI_DISCOVER_NTF_TYPE_LAST: c_uint = 0x00;
pub const NCI_DISCOVER_NTF_TYPE_LAST_NFCC: c_uint = 0x01;
pub const NCI_DISCOVER_NTF_TYPE_MORE: c_uint = 0x02;
// NCI Deactivation Type
pub const NCI_DEACTIVATE_TYPE_IDLE_MODE: c_uint = 0x00;
pub const NCI_DEACTIVATE_TYPE_SLEEP_MODE: c_uint = 0x01;
pub const NCI_DEACTIVATE_TYPE_SLEEP_AF_MODE: c_uint = 0x02;
pub const NCI_DEACTIVATE_TYPE_DISCOVERY: c_uint = 0x03;
// Message Type (MT)
pub const NCI_MT_DATA_PKT: c_uint = 0x00;
pub const NCI_MT_CMD_PKT: c_uint = 0x01;
pub const NCI_MT_RSP_PKT: c_uint = 0x02;
pub const NCI_MT_NTF_PKT: c_uint = 0x03;

// Packet Boundary Flag (PBF)
pub const NCI_PBF_LAST: c_uint = 0x00;
pub const NCI_PBF_CONT: c_uint = 0x01;

// Control Opcode manipulation

// Payload Length

// Connection ID

// GID values
pub const NCI_GID_CORE: c_uint = 0x0;
pub const NCI_GID_RF_MGMT: c_uint = 0x1;
pub const NCI_GID_NFCEE_MGMT: c_uint = 0x2;
pub const NCI_GID_PROPRIETARY: c_uint = 0xf;
// ----- NCI over SPI head/crc(tail) room needed for outgoing frames -----
pub const NCI_SPI_HDR_LEN: c_int = 4;
pub const NCI_SPI_CRC_LEN: c_int = 2;
// ---- NCI Packet structures ----
pub const NCI_CTRL_HDR_SIZE: c_int = 3;
pub const NCI_DATA_HDR_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_ctrl_hdr {
    pub /: *mut *mut __u8 gid; / MT & PBF & GID,
    pub oid: __u8,
    pub plen: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_data_hdr {
    pub /: *mut *mut __u8 conn_id; / MT & PBF & ConnID,
    pub rfu: __u8,
    pub plen: __u8,
    pub __packed: },
// ------------------------
// -----  NCI Commands ----
// ------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_reset_cmd {
    pub reset_type: __u8,
    pub __packed: },

// To support NCI 2.x
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_init_v2_cmd {
    pub feature1: u8,
    pub feature2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_config_param {
    pub id: __u8,
    pub len: __u8,
    pub val: [__u8; NCI_MAX_PARAM_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_set_config_cmd {
    pub num_params: __u8,
    pub /: *mut *mut set_config_param param; / support 1 param per cmd is enough,
    pub __packed: },

pub const DEST_SPEC_PARAMS_ID_INDEX: c_int = 0;
pub const DEST_SPEC_PARAMS_PROTOCOL_INDEX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dest_spec_params {
    pub id: __u8,
    pub protocol: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_conn_create_dest_spec_params {
    pub type: __u8,
    pub length: __u8,
    pub value: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_conn_create_cmd {
    pub destination_type: __u8,
    pub number_destination_params: __u8,
    pub params: [core_conn_create_dest_spec_params; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disc_map_config {
    pub rf_protocol: __u8,
    pub mode: __u8,
    pub rf_interface: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_disc_map_cmd {
    pub num_mapping_configs: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disc_config {
    pub rf_tech_and_mode: __u8,
    pub frequency: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_disc_cmd {
    pub num_disc_configs: __u8,
    pub disc_configs: [disc_config; NCI_MAX_NUM_RF_CONFIGS],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_discover_select_cmd {
    pub rf_discovery_id: __u8,
    pub rf_protocol: __u8,
    pub rf_interface: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_deactivate_cmd {
    pub type: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_nfcee_discover_cmd {
    pub discovery_action: __u8,
    pub __packed: },

pub const NCI_NFCEE_DISABLE: c_uint = 0x00;
pub const NCI_NFCEE_ENABLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_nfcee_mode_set_cmd {
    pub nfcee_id: __u8,
    pub nfcee_mode: __u8,
    pub __packed: },

// -----------------------
// ---- NCI Responses ----
// -----------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_reset_rsp {
    pub status: __u8,
    pub nci_ver: __u8,
    pub config_status: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_init_rsp_1 {
    pub status: __u8,
    pub nfcc_features: __le32,
    pub num_supported_rf_interfaces: __u8,
    pub /: *mut *mut __u8 supported_rf_interfaces[]; / variable size array,
// continued in nci_core_init_rsp_2
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_init_rsp_2 {
    pub max_logical_connections: __u8,
    pub max_routing_table_size: __le16,
    pub max_ctrl_pkt_payload_len: __u8,
    pub max_size_for_large_params: __le16,
    pub manufact_id: __u8,
    pub manufact_specific_info: __le32,
    pub __packed: },
// To support NCI ver 2.x
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_init_rsp_nci_ver2 {
    pub status: u8,
    pub nfcc_features: __le32,
    pub max_logical_connections: u8,
    pub max_routing_table_size: __le16,
    pub max_ctrl_pkt_payload_len: u8,
    pub max_data_pkt_hci_payload_len: u8,
    pub number_of_hci_credit: u8,
    pub max_nfc_v_frame_size: __le16,
    pub num_supported_rf_interfaces: u8,
    pub supported_rf_interfaces: [u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_set_config_rsp {
    pub status: __u8,
    pub num_params: __u8,
    pub /: *mut *mut __u8 params_id[]; / variable size array,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_conn_create_rsp {
    pub status: __u8,
    pub max_ctrl_pkt_payload_len: __u8,
    pub credits_cnt: __u8,
    pub conn_id: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_nfcee_discover_rsp {
    pub status: __u8,
    pub num_nfcee: __u8,
    pub __packed: },

// ---------------------------
// ---- NCI Notifications ----
// ---------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_reset_ntf {
    pub reset_trigger: u8,
    pub config_status: u8,
    pub nci_ver: u8,
    pub manufact_id: u8,
    pub manufacturer_specific_len: u8,
    pub manufact_specific_info: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conn_credit_entry {
    pub conn_id: __u8,
    pub credits: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_conn_credit_ntf {
    pub num_entries: __u8,
    pub conn_entries: [conn_credit_entry; NCI_MAX_NUM_CONN],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_core_intf_error_ntf {
    pub status: __u8,
    pub conn_id: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_tech_specific_params_nfca_poll {
    pub sens_res: __u16,
    pub /: *mut *mut __u8 nfcid1_len; / 0, 4, 7, or 10 Bytes,
    pub nfcid1: [__u8; NFC_NFCID1_MAXSIZE],
    pub /: *mut *mut __u8 sel_res_len; / 0 or 1 Bytes,
    pub sel_res: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_tech_specific_params_nfcb_poll {
    pub sensb_res_len: __u8,
    pub /: *mut *mut __u8 sensb_res[NFC_SENSB_RES_MAXSIZE]; / 11 or 12 Bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_tech_specific_params_nfcf_poll {
    pub bit_rate: __u8,
    pub sensf_res_len: __u8,
    pub /: *mut *mut __u8 sensf_res[NFC_SENSF_RES_MAXSIZE]; / 16 or 18 Bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_tech_specific_params_nfcv_poll {
    pub res_flags: __u8,
    pub dsfid: __u8,
    pub /: *mut *mut __u8 uid[NFC_ISO15693_UID_MAXSIZE]; / 8 Bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_tech_specific_params_nfcf_listen {
    pub local_nfcid2_len: __u8,
    pub /: *mut *mut __u8 local_nfcid2[NFC_NFCID2_MAXSIZE]; / 0 or 8 Bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_discover_ntf {
    pub rf_discovery_id: __u8,
    pub rf_protocol: __u8,
    pub rf_tech_and_mode: __u8,
    pub rf_tech_specific_params_len: __u8,
    pub nfca_poll: rf_tech_specific_params_nfca_poll,
    pub nfcb_poll: rf_tech_specific_params_nfcb_poll,
    pub nfcf_poll: rf_tech_specific_params_nfcf_poll,
    pub nfcv_poll: rf_tech_specific_params_nfcv_poll,
    pub rf_tech_specific_params: },
    pub ntf_type: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct activation_params_nfca_poll_iso_dep {
    pub rats_res_len: __u8,
    pub rats_res: [__u8; NFC_ATS_MAXSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct activation_params_nfcb_poll_iso_dep {
    pub attrib_res_len: __u8,
    pub attrib_res: [__u8; 50],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct activation_params_poll_nfc_dep {
    pub atr_res_len: __u8,
    pub /: *mut *mut __u8 atr_res[NFC_ATR_RES_MAXSIZE - 2]; / ATR_RES from byte 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct activation_params_listen_nfc_dep {
    pub atr_req_len: __u8,
    pub /: *mut *mut __u8 atr_req[NFC_ATR_REQ_MAXSIZE - 2]; / ATR_REQ from byte 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_intf_activated_ntf {
    pub rf_discovery_id: __u8,
    pub rf_interface: __u8,
    pub rf_protocol: __u8,
    pub activation_rf_tech_and_mode: __u8,
    pub max_data_pkt_payload_size: __u8,
    pub initial_num_credits: __u8,
    pub rf_tech_specific_params_len: __u8,
    pub nfca_poll: rf_tech_specific_params_nfca_poll,
    pub nfcb_poll: rf_tech_specific_params_nfcb_poll,
    pub nfcf_poll: rf_tech_specific_params_nfcf_poll,
    pub nfcv_poll: rf_tech_specific_params_nfcv_poll,
    pub nfcf_listen: rf_tech_specific_params_nfcf_listen,
    pub rf_tech_specific_params: },
    pub data_exch_rf_tech_and_mode: __u8,
    pub data_exch_tx_bit_rate: __u8,
    pub data_exch_rx_bit_rate: __u8,
    pub activation_params_len: __u8,
    pub nfca_poll_iso_dep: activation_params_nfca_poll_iso_dep,
    pub nfcb_poll_iso_dep: activation_params_nfcb_poll_iso_dep,
    pub poll_nfc_dep: activation_params_poll_nfc_dep,
    pub listen_nfc_dep: activation_params_listen_nfc_dep,
    pub activation_params: },
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_deactivate_ntf {
    pub type: __u8,
    pub reason: __u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_rf_nfcee_action_ntf {
    pub nfcee_id: __u8,
    pub trigger: __u8,
    pub supported_data_length: __u8,
    pub supported_data: [__u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_nfcee_supported_protocol {
    pub num_protocol: __u8,
    pub supported_protocol: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_nfcee_information_tlv {
    pub num_tlv: __u8,
    pub information_tlv: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_nfcee_discover_ntf {
    pub nfcee_id: __u8,
    pub nfcee_status: __u8,
    pub supported_protocols: nci_nfcee_supported_protocol,
    pub information_tlv: nci_nfcee_information_tlv,
    pub __packed: },

