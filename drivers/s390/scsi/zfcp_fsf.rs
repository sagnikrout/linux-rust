//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_fsf.h
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
//
// zfcp device driver
//
// Interface to the FSF support functions.
//
// Copyright IBM Corp. 2002, 2020
//

pub const FSF_QTCB_CURRENT_VERSION: c_uint = 0x00000001;
// FSF commands
pub const FSF_QTCB_FCP_CMND: c_uint = 0x00000001;
pub const FSF_QTCB_ABORT_FCP_CMND: c_uint = 0x00000002;
pub const FSF_QTCB_OPEN_PORT_WITH_DID: c_uint = 0x00000005;
pub const FSF_QTCB_OPEN_LUN: c_uint = 0x00000006;
pub const FSF_QTCB_CLOSE_LUN: c_uint = 0x00000007;
pub const FSF_QTCB_CLOSE_PORT: c_uint = 0x00000008;
pub const FSF_QTCB_CLOSE_PHYSICAL_PORT: c_uint = 0x00000009;
pub const FSF_QTCB_SEND_ELS: c_uint = 0x0000000B;
pub const FSF_QTCB_SEND_GENERIC: c_uint = 0x0000000C;
pub const FSF_QTCB_EXCHANGE_CONFIG_DATA: c_uint = 0x0000000D;
pub const FSF_QTCB_EXCHANGE_PORT_DATA: c_uint = 0x0000000E;
pub const FSF_QTCB_DOWNLOAD_CONTROL_FILE: c_uint = 0x00000012;
pub const FSF_QTCB_UPLOAD_CONTROL_FILE: c_uint = 0x00000013;
// FSF QTCB types
pub const FSF_IO_COMMAND: c_uint = 0x00000001;
pub const FSF_SUPPORT_COMMAND: c_uint = 0x00000002;
pub const FSF_CONFIG_COMMAND: c_uint = 0x00000003;
pub const FSF_PORT_COMMAND: c_uint = 0x00000004;
// FSF protocol states
pub const FSF_PROT_GOOD: c_uint = 0x00000001;
pub const FSF_PROT_QTCB_VERSION_ERROR: c_uint = 0x00000010;
pub const FSF_PROT_SEQ_NUMB_ERROR: c_uint = 0x00000020;
pub const FSF_PROT_UNSUPP_QTCB_TYPE: c_uint = 0x00000040;
pub const FSF_PROT_HOST_CONNECTION_INITIALIZING: c_uint = 0x00000080;
pub const FSF_PROT_FSF_STATUS_PRESENTED: c_uint = 0x00000100;
pub const FSF_PROT_DUPLICATE_REQUEST_ID: c_uint = 0x00000200;
pub const FSF_PROT_LINK_DOWN: c_uint = 0x00000400;
pub const FSF_PROT_REEST_QUEUE: c_uint = 0x00000800;
pub const FSF_PROT_ERROR_STATE: c_uint = 0x01000000;
// FSF states
pub const FSF_GOOD: c_uint = 0x00000000;
pub const FSF_PORT_ALREADY_OPEN: c_uint = 0x00000001;
pub const FSF_LUN_ALREADY_OPEN: c_uint = 0x00000002;
pub const FSF_PORT_HANDLE_NOT_VALID: c_uint = 0x00000003;
pub const FSF_LUN_HANDLE_NOT_VALID: c_uint = 0x00000004;
pub const FSF_HANDLE_MISMATCH: c_uint = 0x00000005;
pub const FSF_SERVICE_CLASS_NOT_SUPPORTED: c_uint = 0x00000006;
pub const FSF_FCPLUN_NOT_VALID: c_uint = 0x00000009;
pub const FSF_LUN_SHARING_VIOLATION: c_uint = 0x00000012;
pub const FSF_FCP_COMMAND_DOES_NOT_EXIST: c_uint = 0x00000022;
pub const FSF_DIRECTION_INDICATOR_NOT_VALID: c_uint = 0x00000030;
pub const FSF_CMND_LENGTH_NOT_VALID: c_uint = 0x00000033;
pub const FSF_MAXIMUM_NUMBER_OF_PORTS_EXCEEDED: c_uint = 0x00000040;
pub const FSF_MAXIMUM_NUMBER_OF_LUNS_EXCEEDED: c_uint = 0x00000041;
pub const FSF_ELS_COMMAND_REJECTED: c_uint = 0x00000050;
pub const FSF_GENERIC_COMMAND_REJECTED: c_uint = 0x00000051;
pub const FSF_PORT_BOXED: c_uint = 0x00000059;
pub const FSF_LUN_BOXED: c_uint = 0x0000005A;
pub const FSF_EXCHANGE_CONFIG_DATA_INCOMPLETE: c_uint = 0x0000005B;
pub const FSF_PAYLOAD_SIZE_MISMATCH: c_uint = 0x00000060;
pub const FSF_REQUEST_SIZE_TOO_LARGE: c_uint = 0x00000061;
pub const FSF_RESPONSE_SIZE_TOO_LARGE: c_uint = 0x00000062;
pub const FSF_SBAL_MISMATCH: c_uint = 0x00000063;
pub const FSF_INCONSISTENT_PROT_DATA: c_uint = 0x00000070;
pub const FSF_INVALID_PROT_PARM: c_uint = 0x00000071;
pub const FSF_BLOCK_GUARD_CHECK_FAILURE: c_uint = 0x00000081;
pub const FSF_APP_TAG_CHECK_FAILURE: c_uint = 0x00000082;
pub const FSF_REF_TAG_CHECK_FAILURE: c_uint = 0x00000083;
pub const FSF_SECURITY_ERROR: c_uint = 0x00000090;
pub const FSF_ADAPTER_STATUS_AVAILABLE: c_uint = 0x000000AD;
pub const FSF_FCP_RSP_AVAILABLE: c_uint = 0x000000AF;
pub const FSF_UNKNOWN_COMMAND: c_uint = 0x000000E2;
pub const FSF_UNKNOWN_OP_SUBTYPE: c_uint = 0x000000E3;
pub const FSF_INVALID_COMMAND_OPTION: c_uint = 0x000000E5;
pub const FSF_PROT_STATUS_QUAL_SIZE: c_int = 16;
pub const FSF_STATUS_QUALIFIER_SIZE: c_int = 16;
// FSF status qualifier, recommendations
pub const FSF_SQ_NO_RECOM: c_uint = 0x00;
pub const FSF_SQ_FCP_RSP_AVAILABLE: c_uint = 0x01;
pub const FSF_SQ_RETRY_IF_POSSIBLE: c_uint = 0x02;
pub const FSF_SQ_ULP_DEPENDENT_ERP_REQUIRED: c_uint = 0x03;
pub const FSF_SQ_INVOKE_LINK_TEST_PROCEDURE: c_uint = 0x04;
pub const FSF_SQ_COMMAND_ABORTED: c_uint = 0x06;
pub const FSF_SQ_NO_RETRY_POSSIBLE: c_uint = 0x07;
// FSF status qualifier (most significant 4 bytes), local link down
pub const FSF_PSQ_LINK_NO_LIGHT: c_uint = 0x00000004;
pub const FSF_PSQ_LINK_WRAP_PLUG: c_uint = 0x00000008;
pub const FSF_PSQ_LINK_NO_FCP: c_uint = 0x00000010;
pub const FSF_PSQ_LINK_FIRMWARE_UPDATE: c_uint = 0x00000020;
pub const FSF_PSQ_LINK_INVALID_WWPN: c_uint = 0x00000100;
pub const FSF_PSQ_LINK_NO_NPIV_SUPPORT: c_uint = 0x00000200;
pub const FSF_PSQ_LINK_NO_FCP_RESOURCES: c_uint = 0x00000400;
pub const FSF_PSQ_LINK_NO_FABRIC_RESOURCES: c_uint = 0x00000800;
pub const FSF_PSQ_LINK_FABRIC_LOGIN_UNABLE: c_uint = 0x00001000;
pub const FSF_PSQ_LINK_WWPN_ASSIGNMENT_CORRUPTED: c_uint = 0x00002000;
pub const FSF_PSQ_LINK_MODE_TABLE_CURRUPTED: c_uint = 0x00004000;
pub const FSF_PSQ_LINK_NO_WWPN_ASSIGNMENT: c_uint = 0x00008000;
// FSF status qualifier, security error
pub const FSF_SQ_SECURITY_REQUIRED: c_uint = 0x00000001;
pub const FSF_SQ_SECURITY_TIMEOUT: c_uint = 0x00000002;
pub const FSF_SQ_SECURITY_KM_UNAVAILABLE: c_uint = 0x00000003;
pub const FSF_SQ_SECURITY_RKM_UNAVAILABLE: c_uint = 0x00000004;
pub const FSF_SQ_SECURITY_AUTH_FAILURE: c_uint = 0x00000005;
pub const FSF_SQ_SECURITY_ENC_FAILURE: c_uint = 0x00000010;
// payload size in status read buffer
pub const FSF_STATUS_READ_PAYLOAD_SIZE: c_int = 4032;
// number of status read buffers that should be sent by ULP
pub const FSF_STATUS_READS_RECOM: c_int = 16;
// status types in status read buffer
pub const FSF_STATUS_READ_PORT_CLOSED: c_uint = 0x00000001;
pub const FSF_STATUS_READ_INCOMING_ELS: c_uint = 0x00000002;
pub const FSF_STATUS_READ_SENSE_DATA_AVAIL: c_uint = 0x00000003;
pub const FSF_STATUS_READ_BIT_ERROR_THRESHOLD: c_uint = 0x00000004;
pub const FSF_STATUS_READ_LINK_DOWN: c_uint = 0x00000005;
pub const FSF_STATUS_READ_LINK_UP: c_uint = 0x00000006;
pub const FSF_STATUS_READ_NOTIFICATION_LOST: c_uint = 0x00000009;
pub const FSF_STATUS_READ_FEATURE_UPDATE_ALERT: c_uint = 0x0000000C;
pub const FSF_STATUS_READ_VERSION_CHANGE: c_uint = 0x0000000D;
// status subtypes for link down
pub const FSF_STATUS_READ_SUB_NO_PHYSICAL_LINK: c_uint = 0x00000000;
pub const FSF_STATUS_READ_SUB_FDISC_FAILED: c_uint = 0x00000001;
pub const FSF_STATUS_READ_SUB_FIRMWARE_UPDATE: c_uint = 0x00000002;
// status subtypes for unsolicited status notification lost
pub const FSF_STATUS_READ_SUB_INCOMING_ELS: c_uint = 0x00000001;
pub const FSF_STATUS_READ_SUB_VERSION_CHANGE: c_uint = 0x00000100;
// status subtypes for version change
pub const FSF_STATUS_READ_SUB_LIC_CHANGE: c_uint = 0x00000001;
// topologie that is detected by the adapter
pub const FSF_TOPO_P2P: c_uint = 0x00000001;
pub const FSF_TOPO_FABRIC: c_uint = 0x00000002;
pub const FSF_TOPO_AL: c_uint = 0x00000003;
// data direction for FCP commands
pub const FSF_DATADIR_WRITE: c_uint = 0x00000001;
pub const FSF_DATADIR_READ: c_uint = 0x00000002;
pub const FSF_DATADIR_CMND: c_uint = 0x00000004;
pub const FSF_DATADIR_DIF_WRITE_INSERT: c_uint = 0x00000009;
pub const FSF_DATADIR_DIF_READ_STRIP: c_uint = 0x0000000a;
pub const FSF_DATADIR_DIF_WRITE_CONVERT: c_uint = 0x0000000b;

// data protection control flags
pub const FSF_APP_TAG_CHECK_ENABLE: c_uint = 0x10;
// fc service class
pub const FSF_CLASS_3: c_uint = 0x00000003;
// logging space behind QTCB
pub const FSF_QTCB_LOG_SIZE: c_int = 1024;
// channel features
pub const FSF_FEATURE_NOTIFICATION_LOST: c_uint = 0x00000008;
pub const FSF_FEATURE_HBAAPI_MANAGEMENT: c_uint = 0x00000010;
pub const FSF_FEATURE_ELS_CT_CHAINED_SBALS: c_uint = 0x00000020;
pub const FSF_FEATURE_UPDATE_ALERT: c_uint = 0x00000100;
pub const FSF_FEATURE_MEASUREMENT_DATA: c_uint = 0x00000200;
pub const FSF_FEATURE_REQUEST_SFP_DATA: c_uint = 0x00000200;
pub const FSF_FEATURE_REPORT_SFP_DATA: c_uint = 0x00000800;
pub const FSF_FEATURE_FC_SECURITY: c_uint = 0x00001000;
pub const FSF_FEATURE_DIF_PROT_TYPE1: c_uint = 0x00010000;
pub const FSF_FEATURE_DIX_PROT_TCPIP: c_uint = 0x00020000;
// host connection features
pub const FSF_FEATURE_NPIV_MODE: c_uint = 0x00000001;
// option
pub const FSF_OPEN_LUN_SUPPRESS_BOXING: c_uint = 0x00000001;
// FC security algorithms
pub const FSF_FC_SECURITY_AUTH: c_uint = 0x00000001;
pub const FSF_FC_SECURITY_ENC_FCSP2: c_uint = 0x00000002;
pub const FSF_FC_SECURITY_ENC_ERAS: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_queue_designator {
    pub cssid: u8,
    pub chpid: u8,
    pub hla: u8,
    pub ua: u8,
    pub res1: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_bit_error_payload {
    pub res1: u32,
    pub link_failure_error_count: u32,
    pub loss_of_sync_error_count: u32,
    pub loss_of_signal_error_count: u32,
    pub primitive_sequence_error_count: u32,
    pub invalid_transmission_word_error_count: u32,
    pub crc_error_count: u32,
    pub primitive_sequence_event_timeout_count: u32,
    pub elastic_buffer_overrun_error_count: u32,
    pub fcal_arbitration_timeout_count: u32,
    pub advertised_receive_b2b_credit: u32,
    pub current_receive_b2b_credit: u32,
    pub advertised_transmit_b2b_credit: u32,
    pub current_transmit_b2b_credit: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_link_down_info {
    pub error_code: u32,
    pub res1: u32,
    pub res2: [u8; 2],
    pub primary_status: u8,
    pub ioerr_code: u8,
    pub action_code: u8,
    pub reason_code: u8,
    pub explanation_code: u8,
    pub vendor_specific_code: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_version_change {
    pub current_version: u32,
    pub previous_version: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_status_read_buffer {
    pub status_type: u32,
    pub status_subtype: u32,
    pub length: u32,
    pub res1: u32,
    pub queue_designator: fsf_queue_designator,
    pub res2: u8,
    pub d_id: [u8; 3],
    pub class: u32,
    pub fcp_lun: u64,
    pub res3: u8,
    pub s_id: [u8; 3],
    pub res4: [u8; 20],
    pub data: [u8; FSF_STATUS_READ_PAYLOAD_SIZE],
    pub word: [u32; FSF_STATUS_READ_PAYLOAD_SIZE/sizeof(u32)],
    pub link_down_info: fsf_link_down_info,
    pub bit_error: fsf_bit_error_payload,
    pub version_change: fsf_version_change,
    pub payload: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qual_version_error {
    pub fsf_version: u32,
    pub res1: [u32; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qual_sequence_error {
    pub exp_req_seq_no: u32,
    pub res1: [u32; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qual_latency_info {
    pub channel_lat: u32,
    pub fabric_lat: u32,
    pub res1: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub union fsf_prot_status_qual {
    pub sizeof(u32)]: u32 word[FSF_PROT_STATUS_QUAL_SIZE /,
    pub sizeof(u64)]: u64 doubleword[FSF_PROT_STATUS_QUAL_SIZE /,
    pub version_error: fsf_qual_version_error,
    pub sequence_error: fsf_qual_sequence_error,
    pub link_down_info: fsf_link_down_info,
    pub latency_info: fsf_qual_latency_info,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb_prefix {
    pub req_id: u64,
    pub qtcb_version: u32,
    pub ulp_info: u32,
    pub qtcb_type: u32,
    pub req_seq_no: u32,
    pub prot_status: u32,
    pub prot_status_qual: fsf_prot_status_qual,
    pub res1: [u8; 20],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_statistics_info {
    pub input_req: u64,
    pub output_req: u64,
    pub control_req: u64,
    pub input_mb: u64,
    pub output_mb: u64,
    pub seconds_act: u64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub union fsf_status_qual {
    pub byte: [u8; FSF_STATUS_QUALIFIER_SIZE],
    pub (u16)]: u16 halfword[FSF_STATUS_QUALIFIER_SIZE / sizeof,
    pub (u32)]: u32 word[FSF_STATUS_QUALIFIER_SIZE / sizeof,
    pub sizeof(u64)]: u64 doubleword[FSF_STATUS_QUALIFIER_SIZE /,
    pub fsf_queue_designator: fsf_queue_designator,
    pub link_down_info: fsf_link_down_info,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb_header {
    pub req_handle: u64,
    pub fsf_command: u32,
    pub res1: u32,
    pub port_handle: u32,
    pub lun_handle: u32,
    pub res2: u32,
    pub fsf_status: u32,
    pub fsf_status_qual: fsf_status_qual,
    pub res3: [u8; 28],
    pub log_start: u16,
    pub log_length: u16,
    pub res4: [u8; 16],
// C attribute field omitted
pub const FSF_PLOGI_MIN_LEN: c_int = 112;
pub const FSF_FCP_CMND_SIZE: c_int = 288;
pub const FSF_FCP_RSP_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb_bottom_io {
    pub data_direction: u32,
    pub service_class: u32,
    pub res1: u8,
    pub data_prot_flags: u8,
    pub app_tag_value: u16,
    pub ref_tag_value: u32,
    pub fcp_cmnd_length: u32,
    pub data_block_length: u32,
    pub prot_data_length: u32,
    pub res2: [u8; 4],
    pub byte: [u8; FSF_FCP_CMND_SIZE],
    pub iu: fcp_cmnd,
    pub fcp_cmnd: },
    pub byte: [u8; FSF_FCP_RSP_SIZE],
    pub iu: fcp_resp_with_ext,
    pub fcp_rsp: },
    pub res3: [u8; 64],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb_bottom_support {
    pub operation_subtype: u32,
    pub res1: [u8; 13],
    pub d_id: [u8; 3],
    pub option: u32,
    pub fcp_lun: u64,
    pub res2: u64,
    pub req_handle: u64,
    pub service_class: u32,
    pub res3: [u8; 3],
    pub timeout: u8,
    pub lun_access_info: u32,
    pub connection_info: u32,
    pub res4: [u8; 176],
    pub els1_length: u32,
    pub els2_length: u32,
    pub req_buf_length: u32,
    pub resp_buf_length: u32,
    pub els: [u8; 256],
// C attribute field omitted
pub const ZFCP_FSF_TIMER_INT_MASK: c_uint = 0x3FFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb_bottom_config {
    pub lic_version: u32,
    pub feature_selection: u32,
    pub high_qtcb_version: u32,
    pub low_qtcb_version: u32,
    pub max_qtcb_size: u32,
    pub max_data_transfer_size: u32,
    pub adapter_features: u32,
    pub connection_features: u32,
    pub fc_topology: u32,
    pub /: *mut *mut *mut u32 fc_link_speed; / one of ZFCP_FSF_PORTSPEED_,
    pub adapter_type: u32,
    pub res0: u8,
    pub peer_d_id: [u8; 3],
    pub status_read_buf_num: u16,
    pub timer_interval: u16,
    pub res2: [u8; 9],
    pub s_id: [u8; 3],
    pub nport_serv_param: [u8; 128],
    pub res3: [u8; 8],
    pub adapter_ports: u32,
    pub hardware_version: u32,
    pub serial_number: [u8; 32],
    pub plogi_payload: [u8; 112],
    pub stat_info: fsf_statistics_info,
    pub res4: [u8; 112],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb_bottom_port {
    pub wwpn: u64,
    pub fc_port_id: u32,
    pub port_type: u32,
    pub port_state: u32,
    pub /: *mut *mut u32 class_of_service; / should be 0x00000006 for class 2 and 3,
    pub /: *mut *mut u8 supported_fc4_types[32]; / should be 0x00000100 for scsi fcp,
    pub active_fc4_types: [u8; 32],
    pub /: *mut *mut *mut u32 supported_speed; / any combination of ZFCP_FSF_PORTSPEED_,
    pub /: *mut *mut u32 maximum_frame_size; / fixed value of 2112,
    pub seconds_since_last_reset: u64,
    pub tx_frames: u64,
    pub tx_words: u64,
    pub rx_frames: u64,
    pub rx_words: u64,
    pub /: *mut *mut u64 lip; / 0,
    pub /: *mut *mut u64 nos; / currently 0,
    pub /: *mut *mut u64 error_frames; / currently 0,
    pub /: *mut *mut u64 dumped_frames; / currently 0,
    pub link_failure: u64,
    pub loss_of_sync: u64,
    pub loss_of_signal: u64,
    pub psp_error_counts: u64,
    pub invalid_tx_words: u64,
    pub invalid_crcs: u64,
    pub input_requests: u64,
    pub output_requests: u64,
    pub control_requests: u64,
    pub /: *mut *mut u64 input_mb; / where 1 MByte == 1.000.000 Bytes,
    pub /: *mut *mut u64 output_mb; / where 1 MByte == 1.000.000 Bytes,
    pub cp_util: u8,
    pub cb_util: u8,
    pub a_util: u8,
    pub res2: u8,
    pub temperature: i16,
    pub vcc: u16,
    pub tx_bias: u16,
    pub tx_power: u16,
    pub rx_power: u16,
    pub raw: u16,
    pub :1: u16 fec_active,
    pub :2: u16 connector_type,
    pub :1: u16 sfp_invalid,
    pub :1: u16 optical_port,
    pub :4: u16 port_tx_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fsf_qtcb_bottom {
    pub io: fsf_qtcb_bottom_io,
    pub support: fsf_qtcb_bottom_support,
    pub config: fsf_qtcb_bottom_config,
    pub port: fsf_qtcb_bottom_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsf_qtcb {
    pub prefix: fsf_qtcb_prefix,
    pub header: fsf_qtcb_header,
    pub bottom: fsf_qtcb_bottom,
    pub log: [u8; FSF_QTCB_LOG_SIZE],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_blk_drv_data {
pub const ZFCP_BLK_DRV_DATA_MAGIC: c_uint = 0x1;
    pub magic: u32,
pub const ZFCP_BLK_LAT_VALID: c_uint = 0x1;
pub const ZFCP_BLK_REQ_ERROR: c_uint = 0x2;
    pub flags: u16,
    pub inb_usage: u8,
    pub outb_usage: u8,
    pub channel_lat: u64,
    pub fabric_lat: u64,
// C attribute field omitted
//
// struct zfcp_fsf_ct_els - zfcp data for ct or els request
// @req: scatter-gather list for request, points to &zfcp_fc_req.sg_req or BSG
// @resp: scatter-gather list for response, points to &zfcp_fc_req.sg_rsp or BSG
// @handler: handler function (called for response to the request)
// @handler_data: data passed to handler function
// @port: Optional pointer to port for zfcp internal ELS (only test link ADISC)
// @status: used to pass error status to calling function
// @d_id: Destination ID of either open WKA port for CT or of D_ID for ELS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fsf_ct_els {
    pub req: *mut scatterlist,
    pub resp: *mut scatterlist,
    pub ): *mut *mut void (handler)(void,
    pub handler_data: *mut c_void,
    pub port: *mut zfcp_port,
    pub status: c_int,
    pub d_id: u32,
}
