//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/be2iscsi/be_cmds.h
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
// Copyright 2017 Broadcom. All Rights Reserved.
// The term "Broadcom" refers to Broadcom Limited and/or its subsidiaries.
//
// Contact Information:
// linux-drivers@broadcom.com
//
// The driver sends configuration and managements command requests to the
// firmware in the BE. These requests are communicated to the processor
// using Work Request Blocks (WRBs) submitted to the MCC-WRB ring or via one
// WRB inside a MAILBOX.
// The commands are serviced by the ARM processor in the OneConnect's MPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_sge {
    pub pa_lo: __le32,
    pub pa_hi: __le32,
    pub len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_wrb {
    pub /: *mut *mut u32 emb_sgecnt_special; / dword 0,
// bits 0 - embedded
// bits 1 - 2 reserved
// bits 3 - 7 sge count
// bits 8 - 23 reserved
// bits 24 - 31 special
pub const MCC_WRB_EMBEDDED_MASK: c_int = 1;
pub const MCC_WRB_SGE_CNT_SHIFT: c_int = 3;
pub const MCC_WRB_SGE_CNT_MASK: c_uint = 0x1F;
    pub /: *mut *mut u32 payload_length; / dword 1,
    pub /: *mut *mut u32 tag0; / dword 2,
    pub /: *mut *mut u32 tag1; / dword 3,
    pub /: *mut *mut u32 rsvd; / dword 4,
pub const EMBED_MBX_MAX_PAYLOAD_SIZE: c_int = 220;
    pub /: *mut *mut u8 embedded_payload[236]; / used by embedded cmds,
    pub /: *mut *mut be_sge sgl[19]; / used by non-embedded cmds,
    pub payload: },
}

// Completion Status
pub const MCC_STATUS_SUCCESS: c_uint = 0x0;
pub const MCC_STATUS_FAILED: c_uint = 0x1;
pub const MCC_STATUS_ILLEGAL_REQUEST: c_uint = 0x2;
pub const MCC_STATUS_ILLEGAL_FIELD: c_uint = 0x3;
pub const MCC_STATUS_INSUFFICIENT_BUFFER: c_uint = 0x4;
pub const MCC_STATUS_INVALID_LENGTH: c_uint = 0x74;
pub const CQE_STATUS_COMPL_MASK: c_uint = 0xFFFF;

pub const CQE_STATUS_EXTD_MASK: c_uint = 0xFFFF;

pub const CQE_STATUS_ADDL_MASK: c_uint = 0xFF00;
pub const CQE_STATUS_ADDL_SHIFT: c_int = 8;
pub const CQE_STATUS_MASK: c_uint = 0xFF;
pub const CQE_STATUS_WRB_MASK: c_uint = 0xFF0000;
pub const CQE_STATUS_WRB_SHIFT: c_int = 16;

pub const BEISCSI_FW_MBX_TIMEOUT: c_int = 100;
// MBOX Command VER
pub const MBX_CMD_VER1: c_uint = 0x01;
pub const MBX_CMD_VER2: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_compl {
    pub /: *mut *mut u32 status; / dword 0,
    pub /: *mut *mut u32 tag0; / dword 1,
    pub /: *mut *mut u32 tag1; / dword 2,
    pub /: *mut *mut u32 flags; / dword 3,
}

// Mailbox door bell
//
// Used for driver communication with the FW.
// The software must write this register twice to post any command. First,
// it writes the register with hi=1 and the upper bits of the physical address
// for the MAILBOX structure. Software must poll the ready bit until this
// is acknowledged. Then, sotware writes the register with hi=0 with the lower
// bits in the address. It must poll the ready bit until the command is
// complete. Upon completion, the MAILBOX will contain a valid completion
// queue entry.
//
pub const MPU_MAILBOX_DB_OFFSET: c_uint = 0x160;
pub const MPU_MAILBOX_DB_RDY_MASK: c_uint = 0x1	/* bit 0 */;
pub const MPU_MAILBOX_DB_HI_MASK: c_uint = 0x2	/* bit 1 */;
// MPU semphore: used for SH & BE
pub const SLIPORT_SOFTRESET_OFFSET: c_uint = 0x5c	/* CSR BAR offset */;
pub const SLIPORT_SEMAPHORE_OFFSET_BEx: c_uint = 0xac	/* CSR BAR offset */;
pub const SLIPORT_SEMAPHORE_OFFSET_SH: c_uint = 0x94	/* PCI-CFG offset */;
pub const POST_STAGE_MASK: c_uint = 0x0000FFFF;
pub const POST_ERROR_BIT: c_uint = 0x80000000;
pub const POST_ERR_RECOVERY_CODE_MASK: c_uint = 0xF000;
// Soft Reset register masks
pub const SLIPORT_SOFTRESET_SR_MASK: c_uint = 0x00000080	/* SR bit */;
// MPU semphore POST stage values
pub const POST_STAGE_AWAITING_HOST_RDY: c_uint = 0x1 /* FW awaiting goahead from host */;
pub const POST_STAGE_HOST_RDY: c_uint = 0x2 /* Host has given go-ahed to FW */;
pub const POST_STAGE_BE_RESET: c_uint = 0x3 /* Host wants to reset chip */;
pub const POST_STAGE_ARMFW_RDY: c_uint = 0xC000 /* FW is done with POST */;
pub const POST_STAGE_RECOVERABLE_ERR: c_uint = 0xE000 /* Recoverable err detected */;
// MCC door bell
pub const DB_MCCQ_OFFSET: c_uint = 0x140;
pub const DB_MCCQ_RING_ID_MASK: c_uint = 0xFFFF		/* bits 0 - 15 */;
// Number of entries posted

//
// When the async bit of mcc_compl is set, the last 4 bytes of
// mcc_compl is interpreted as follows:
//

pub const ASYNC_TRAILER_EVENT_CODE_MASK: c_uint = 0xFF;
pub const ASYNC_EVENT_CODE_LINK_STATE: c_uint = 0x1;
pub const ASYNC_EVENT_CODE_ISCSI: c_uint = 0x4;
pub const ASYNC_EVENT_CODE_SLI: c_uint = 0x11;

pub const ASYNC_TRAILER_EVENT_TYPE_MASK: c_uint = 0xFF;
// iSCSI events
pub const ASYNC_EVENT_NEW_ISCSI_TGT_DISC: c_uint = 0x4;
pub const ASYNC_EVENT_NEW_ISCSI_CONN: c_uint = 0x5;
pub const ASYNC_EVENT_NEW_TCP_CONN: c_uint = 0x7;
// SLI events
pub const ASYNC_SLI_EVENT_TYPE_MISCONFIGURED: c_uint = 0x9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_trailer {
    pub code: u32,
}

//
// When the event code of an async trailer is link-state, the mcc_compl
// must be interpreted as follows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_link_state {
    pub physical_port: u8,
    pub port_link_status: u8,
//
// ASYNC_EVENT_LINK_DOWN		0x0
// ASYNC_EVENT_LINK_UP			0x1
// ASYNC_EVENT_LINK_LOGICAL_DOWN	0x2
// ASYNC_EVENT_LINK_LOGICAL_UP		0x3
//
pub const BE_ASYNC_LINK_UP_MASK: c_uint = 0x01;
    pub port_duplex: u8,
    pub port_speed: u8,
// BE2ISCSI_LINK_SPEED_ZERO	0x00 - no link
pub const BE2ISCSI_LINK_SPEED_10MBPS: c_uint = 0x01;
pub const BE2ISCSI_LINK_SPEED_100MBPS: c_uint = 0x02;
pub const BE2ISCSI_LINK_SPEED_1GBPS: c_uint = 0x03;
pub const BE2ISCSI_LINK_SPEED_10GBPS: c_uint = 0x04;
pub const BE2ISCSI_LINK_SPEED_25GBPS: c_uint = 0x06;
pub const BE2ISCSI_LINK_SPEED_40GBPS: c_uint = 0x07;
    pub port_fault: u8,
    pub event_reason: u8,
    pub qos_link_speed: u16,
    pub event_tag: u32,
    pub trailer: be_async_event_trailer,
    pub __packed: },
//
// When async-trailer is SLI event, mcc_compl is interpreted as
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_async_event_sli {
    pub event_data1: u32,
    pub event_data2: u32,
    pub reserved: u32,
    pub trailer: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_mailbox {
    pub wrb: be_mcc_wrb,
    pub compl: be_mcc_compl,
}

// Type of subsystems supported by FW
pub const CMD_SUBSYSTEM_COMMON: c_uint = 0x1;
pub const CMD_SUBSYSTEM_ISCSI: c_uint = 0x2;
pub const CMD_SUBSYSTEM_ETH: c_uint = 0x3;
pub const CMD_SUBSYSTEM_ISCSI_INI: c_uint = 0x6;
pub const CMD_COMMON_TCP_UPLOAD: c_uint = 0x1;
//
// List of common opcodes subsystem  CMD_SUBSYSTEM_COMMON
// These opcodes are unique for each subsystem defined above
//
pub const OPCODE_COMMON_CQ_CREATE: c_int = 12;
pub const OPCODE_COMMON_EQ_CREATE: c_int = 13;
pub const OPCODE_COMMON_MCC_CREATE: c_int = 21;
pub const OPCODE_COMMON_MCC_CREATE_EXT: c_int = 90;
pub const OPCODE_COMMON_ADD_TEMPLATE_HEADER_BUFFERS: c_int = 24;
pub const OPCODE_COMMON_REMOVE_TEMPLATE_HEADER_BUFFERS: c_int = 25;
pub const OPCODE_COMMON_GET_CNTL_ATTRIBUTES: c_int = 32;
pub const OPCODE_COMMON_GET_FW_VERSION: c_int = 35;
pub const OPCODE_COMMON_MODIFY_EQ_DELAY: c_int = 41;
pub const OPCODE_COMMON_FIRMWARE_CONFIG: c_int = 42;
pub const OPCODE_COMMON_MCC_DESTROY: c_int = 53;
pub const OPCODE_COMMON_CQ_DESTROY: c_int = 54;
pub const OPCODE_COMMON_EQ_DESTROY: c_int = 55;
pub const OPCODE_COMMON_QUERY_FIRMWARE_CONFIG: c_int = 58;
pub const OPCODE_COMMON_FUNCTION_RESET: c_int = 61;
pub const OPCODE_COMMON_GET_PORT_NAME: c_int = 77;
pub const OPCODE_COMMON_SET_HOST_DATA: c_int = 93;
pub const OPCODE_COMMON_SET_FEATURES: c_int = 191;
//
// LIST of opcodes that are common between Initiator and Target
// used by CMD_SUBSYSTEM_ISCSI
// These opcodes are unique for each subsystem defined above
//
pub const OPCODE_COMMON_ISCSI_CFG_POST_SGL_PAGES: c_int = 2;
pub const OPCODE_COMMON_ISCSI_CFG_REMOVE_SGL_PAGES: c_int = 3;
pub const OPCODE_COMMON_ISCSI_NTWK_GET_NIC_CONFIG: c_int = 7;
pub const OPCODE_COMMON_ISCSI_NTWK_SET_VLAN: c_int = 14;
pub const OPCODE_COMMON_ISCSI_NTWK_CONFIG_STATELESS_IP_ADDR: c_int = 17;
pub const OPCODE_COMMON_ISCSI_NTWK_REL_STATELESS_IP_ADDR: c_int = 18;
pub const OPCODE_COMMON_ISCSI_NTWK_MODIFY_IP_ADDR: c_int = 21;
pub const OPCODE_COMMON_ISCSI_NTWK_GET_DEFAULT_GATEWAY: c_int = 22;
pub const OPCODE_COMMON_ISCSI_NTWK_MODIFY_DEFAULT_GATEWAY: c_int = 23;
pub const OPCODE_COMMON_ISCSI_NTWK_GET_ALL_IF_ID: c_int = 24;
pub const OPCODE_COMMON_ISCSI_NTWK_GET_IF_INFO: c_int = 25;
pub const OPCODE_COMMON_ISCSI_SET_FRAGNUM_BITS_FOR_SGL_CRA: c_int = 61;
pub const OPCODE_COMMON_ISCSI_DEFQ_CREATE: c_int = 64;
pub const OPCODE_COMMON_ISCSI_DEFQ_DESTROY: c_int = 65;
pub const OPCODE_COMMON_ISCSI_WRBQ_CREATE: c_int = 66;
pub const OPCODE_COMMON_ISCSI_WRBQ_DESTROY: c_int = 67;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_hdr {
    pub /: *mut *mut u8 opcode; / dword 0,
    pub /: *mut *mut u8 subsystem; / dword 0,
    pub /: *mut *mut u8 port_number; / dword 0,
    pub /: *mut *mut u8 domain; / dword 0,
    pub /: *mut *mut u32 timeout; / dword 1,
    pub /: *mut *mut u32 request_length; / dword 2,
    pub /: *mut *mut u8 version; / dword 3,
    pub /: *mut *mut u8 rsvd0[3]; / dword 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_hdr {
    pub /: *mut *mut u32 info; / dword 0,
    pub /: *mut *mut u32 status; / dword 1,
    pub /: *mut *mut u32 response_length; / dword 2,
    pub /: *mut *mut u32 actual_resp_len; / dword 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phys_addr {
    pub lo: u32,
    pub hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virt_addr {
    pub lo: u32,
    pub hi: u32,
}

//
// BE Command definitions
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte - used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eq_context {
    pub /: *mut *mut u8 cidx[13]; / dword 0,
    pub /: *mut *mut u8 rsvd0[3]; / dword 0,
    pub /: *mut *mut u8 epidx[13]; / dword 0,
    pub /: *mut *mut u8 valid; / dword 0,
    pub /: *mut *mut u8 rsvd1; / dword 0,
    pub /: *mut *mut u8 size; / dword 0,
    pub /: *mut *mut u8 pidx[13]; / dword 1,
    pub /: *mut *mut u8 rsvd2[3]; / dword 1,
    pub /: *mut *mut u8 pd[10]; / dword 1,
    pub /: *mut *mut u8 count[3]; / dword 1,
    pub /: *mut *mut u8 solevent; / dword 1,
    pub /: *mut *mut u8 stalled; / dword 1,
    pub /: *mut *mut u8 armed; / dword 1,
    pub /: *mut *mut u8 rsvd3[4]; / dword 2,
    pub /: *mut *mut u8 func[8]; / dword 2,
    pub /: *mut *mut u8 rsvd4; / dword 2,
    pub /: *mut *mut u8 delaymult[10]; / dword 2,
    pub /: *mut *mut u8 rsvd5[2]; / dword 2,
    pub /: *mut *mut u8 phase[2]; / dword 2,
    pub /: *mut *mut u8 nodelay; / dword 2,
    pub /: *mut *mut u8 rsvd6[4]; / dword 2,
    pub /: *mut *mut u8 rsvd7[32]; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_eq_create {
    pub /: *mut *mut be_cmd_req_hdr hdr; / dw[4],
    pub /: *mut *mut u16 num_pages; / sword,
    pub /: *mut *mut u16 rsvd0; / sword,
    pub /: *mut *mut u8 context[sizeof(struct amap_eq_context) / 8]; / dw[4],
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_eq_create {
    pub resp_hdr: be_cmd_resp_hdr,
    pub /: *mut *mut u16 eq_id; / sword,
    pub /: *mut *mut u16 rsvd0; / sword,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_set_eqd {
    pub eq_id: u32,
    pub phase: u32,
    pub delay_multiplier: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_chap_format {
    pub flags: u32,
    pub intr_chap_name: [u8; 256],
    pub intr_secret: [u8; 16],
    pub target_chap_name: [u8; 256],
    pub target_secret: [u8; 16],
    pub intr_chap_name_length: u16,
    pub intr_secret_length: u16,
    pub target_chap_name_length: u16,
    pub target_secret_length: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_auth_method_format {
    pub auth_method_type: u8,
    pub padding: [u8; 3],
    pub chap: mgmt_chap_format,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_logout_fw_sess {
    pub /: *mut *mut be_cmd_req_hdr hdr; / dw[4],
    pub session_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_logout_fw_sess {
    pub /: *mut *mut be_cmd_resp_hdr hdr; / dw[4],
    pub session_status: u32,
pub const BE_SESS_STATUS_CLOSE: c_uint = 0x20;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_conn_login_options {
    pub flags: u8,
    pub header_digest: u8,
    pub data_digest: u8,
    pub rsvd0: u8,
    pub max_recv_datasegment_len_ini: u32,
    pub max_recv_datasegment_len_tgt: u32,
    pub tcp_mss: u32,
    pub tcp_window_size: u32,
    pub auth_data: mgmt_auth_method_format,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_addr_format {
    pub size_of_structure: u16,
    pub reserved: u8,
    pub ip_type: u8,
pub const BEISCSI_IP_TYPE_V4: c_uint = 0x1;
pub const BEISCSI_IP_TYPE_STATIC_V4: c_uint = 0x3;
pub const BEISCSI_IP_TYPE_DHCP_V4: c_uint = 0x5;
// type v4 values < type v6 values
pub const BEISCSI_IP_TYPE_V6: c_uint = 0x10;
pub const BEISCSI_IP_TYPE_ROUTABLE_V6: c_uint = 0x30;
pub const BEISCSI_IP_TYPE_LINK_LOCAL_V6: c_uint = 0x50;
pub const BEISCSI_IP_TYPE_AUTO_V6: c_uint = 0x90;
    pub addr: [u8; 16],
    pub rsvd0: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_conn_info {
    pub connection_handle: u32,
    pub connection_status: u32,
    pub src_port: u16,
    pub dest_port: u16,
    pub dest_port_redirected: u16,
    pub cid: u16,
    pub estimated_throughput: u32,
    pub src_ipaddr: ip_addr_format,
    pub dest_ipaddr: ip_addr_format,
    pub dest_ipaddr_redirected: ip_addr_format,
    pub negotiated_login_options: mgmt_conn_login_options,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_session_login_options {
    pub flags: u8,
    pub error_recovery_level: u8,
    pub rsvd0: u16,
    pub first_burst_length: u32,
    pub max_burst_length: u32,
    pub max_connections: u16,
    pub max_outstanding_r2t: u16,
    pub default_time2wait: u16,
    pub default_time2retain: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_session_info {
    pub session_handle: u32,
    pub status: u32,
    pub isid: [u8; 6],
    pub tsih: u16,
    pub session_flags: u32,
    pub conn_count: u16,
    pub pad: u16,
    pub target_name: [u8; 224],
    pub initiator_iscsiname: [u8; 224],
    pub negotiated_login_options: mgmt_session_login_options,
    pub conn_list: [mgmt_conn_info; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_session_req {
    pub hdr: be_cmd_req_hdr,
    pub session_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_session_resp {
    pub hdr: be_cmd_resp_hdr,
    pub session_info: mgmt_session_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr {
    pub size_of_structure: u16,
    pub addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_boot_target_req {
    pub hdr: be_cmd_req_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_boot_target_resp {
    pub hdr: be_cmd_resp_hdr,
    pub boot_session_count: u32,
    pub boot_session_handle: u32,
//
// FW returns 0xffffffff if it couldn't establish connection with
// configured boot target.
//
pub const BE_BOOT_INVALID_SHANDLE: c_uint = 0xffffffff;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_reopen_session_req {
    pub hdr: be_cmd_req_hdr,
pub const BE_REOPEN_ALL_SESSIONS: c_uint = 0x00;
pub const BE_REOPEN_BOOT_SESSIONS: c_uint = 0x01;
pub const BE_REOPEN_A_SESSION: c_uint = 0x02;
    pub reopen_type: u16,
    pub rsvd: u16,
    pub session_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_reopen_session_resp {
    pub hdr: be_cmd_resp_hdr,
    pub rsvd: u32,
    pub session_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_mac_query_req {
    pub hdr: be_cmd_req_hdr,
    pub type: u8,
    pub permanent: u8,
    pub if_id: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_mac_resp {
    pub hdr: be_cmd_resp_hdr,
    pub mac: mac_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ip_addr_subnet_format {
    pub size_of_structure: u16,
    pub ip_type: u8,
    pub ipv6_prefix_length: u8,
    pub addr: [u8; 16],
    pub subnet_mask: [u8; 16],
    pub rsvd0: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_if_info_req {
    pub hdr: be_cmd_req_hdr,
    pub interface_hndl: u32,
    pub ip_type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_if_info_resp {
    pub hdr: be_cmd_req_hdr,
    pub interface_hndl: u32,
    pub vlan_priority: u32,
    pub ip_addr_count: u32,
    pub dhcp_state: u32,
    pub ip_addr: be_ip_addr_subnet_format,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ip_addr_record {
    pub action: u32,
    pub interface_hndl: u32,
    pub ip_addr: be_ip_addr_subnet_format,
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ip_addr_record_params {
    pub record_entry_count: u32,
    pub ip_record: be_ip_addr_record,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_set_ip_addr_req {
    pub hdr: be_cmd_req_hdr,
    pub ip_params: be_ip_addr_record_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_set_dhcp_req {
    pub hdr: be_cmd_req_hdr,
    pub interface_hndl: u32,
    pub ip_type: u32,
    pub flags: u32,
    pub retry_count: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_rel_dhcp_req {
    pub hdr: be_cmd_req_hdr,
    pub interface_hndl: u32,
    pub ip_type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_set_def_gateway_req {
    pub hdr: be_cmd_req_hdr,
    pub action: u32,
    pub ip_addr: ip_addr_format,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_def_gateway_req {
    pub hdr: be_cmd_req_hdr,
    pub ip_type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_def_gateway_resp {
    pub hdr: be_cmd_req_hdr,
    pub ip_addr: ip_addr_format,
    pub __packed: },
pub const BEISCSI_VLAN_DISABLE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_set_vlan_req {
    pub hdr: be_cmd_req_hdr,
    pub interface_hndl: u32,
    pub vlan_priority: u32,
    pub __packed: },
// Create CQ
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte - used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_cq_context {
    pub /: *mut *mut u8 cidx[11]; / dword 0,
    pub /: *mut *mut u8 rsvd0; / dword 0,
    pub /: *mut *mut u8 coalescwm[2]; / dword 0,
    pub /: *mut *mut u8 nodelay; / dword 0,
    pub /: *mut *mut u8 epidx[11]; / dword 0,
    pub /: *mut *mut u8 rsvd1; / dword 0,
    pub /: *mut *mut u8 count[2]; / dword 0,
    pub /: *mut *mut u8 valid; / dword 0,
    pub /: *mut *mut u8 solevent; / dword 0,
    pub /: *mut *mut u8 eventable; / dword 0,
    pub /: *mut *mut u8 pidx[11]; / dword 1,
    pub /: *mut *mut u8 rsvd2; / dword 1,
    pub /: *mut *mut u8 pd[10]; / dword 1,
    pub /: *mut *mut u8 eqid[8]; / dword 1,
    pub /: *mut *mut u8 stalled; / dword 1,
    pub /: *mut *mut u8 armed; / dword 1,
    pub /: *mut *mut u8 rsvd3[4]; / dword 2,
    pub /: *mut *mut u8 func[8]; / dword 2,
    pub /: *mut *mut u8 rsvd4[20]; / dword 2,
    pub /: *mut *mut u8 rsvd5[32]; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_cq_context_v2 {
    pub /: *mut *mut u8 rsvd0[12]; / dword 0,
    pub /: *mut *mut u8 coalescwm[2]; / dword 0,
    pub /: *mut *mut u8 nodelay; / dword 0,
    pub /: *mut *mut u8 rsvd1[12]; / dword 0,
    pub /: *mut *mut u8 count[2]; / dword 0,
    pub /: *mut *mut u8 valid; / dword 0,
    pub /: *mut *mut u8 rsvd2; / dword 0,
    pub /: *mut *mut u8 eventable; / dword 0,
    pub /: *mut *mut u8 eqid[16]; / dword 1,
    pub /: *mut *mut u8 rsvd3[15]; / dword 1,
    pub /: *mut *mut u8 armed; / dword 1,
    pub /: *mut *mut u8 cqecount[16];/ dword 2,
    pub /: *mut *mut u8 rsvd4[16]; / dword 2,
    pub /: *mut *mut u8 rsvd5[32]; / dword 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_cq_create {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub page_size: u8,
    pub rsvd0: u8,
    pub 8]: u8 context[sizeof(struct amap_cq_context) /,
    pub pages: [phys_addr; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_cq_create {
    pub hdr: be_cmd_resp_hdr,
    pub cq_id: u16,
    pub rsvd0: u16,
    pub __packed: },
// Create MCCQ
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte - used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_mcc_context {
    pub con_index: [u8; 14],
    pub rsvd0: [u8; 2],
    pub ring_size: [u8; 4],
    pub fetch_wrb: u8,
    pub fetch_r2t: u8,
    pub cq_id: [u8; 10],
    pub prod_index: [u8; 14],
    pub fid: [u8; 8],
    pub pdid: [u8; 9],
    pub valid: u8,
    pub rsvd1: [u8; 32],
    pub rsvd2: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_mcc_create_ext {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub rsvd0: u16,
    pub async_evt_bitmap: u32,
    pub 8]: u8 context[sizeof(struct amap_mcc_context) /,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_resp_mcc_create {
    pub hdr: be_cmd_resp_hdr,
    pub id: u16,
    pub rsvd0: u16,
    pub __packed: },
// Q Destroy
// Type of Queue to be destroyed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_q_destroy {
    pub hdr: be_cmd_req_hdr,
    pub id: u16,
    pub /: *mut *mut u16 bypass_flush; / valid only for rx q destroy,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macaddr {
    pub byte: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_mcast_mac_config {
    pub hdr: be_cmd_req_hdr,
    pub num_mac: u16,
    pub promiscuous: u8,
    pub interface_id: u8,
    pub mac: [macaddr; 32],
    pub __packed: },
    pub wrb->payload.embedded_payload: return,
    pub &wrb->payload.sgl[0]: return,
// Modify EQ Delay
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_req_modify_eq_delay {
    pub hdr: be_cmd_req_hdr,
    pub num_eq: __le32,
    pub eq_id: __le32,
    pub phase: __le32,
    pub delay_multiplier: __le32,
    pub delay: [}; MAX_CPUS],
    pub __packed: },
// Get MAC ADDR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_nic_conf_resp {
    pub hdr: be_cmd_resp_hdr,
    pub nic_port_count: u32,
    pub speed: u32,
    pub max_speed: u32,
    pub link_state: u32,
    pub max_frame_size: u32,
    pub size_of_structure: u16,
    pub mac_address: [u8; ETH_ALEN],
    pub __packed: },
// Get HBA NAME
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_hba_name {
    pub hdr: be_cmd_req_hdr,
    pub flags: u16,
    pub rsvd0: u16,
    pub initiator_name: [u8; ISCSI_NAME_LEN],
pub const BE_INI_ALIAS_LEN: c_int = 32;
    pub initiator_alias: [u8; BE_INI_ALIAS_LEN],
    pub __packed: },
// COMMON SET HOST DATA
pub const BE_CMD_SET_HOST_PARAM_ID: c_uint = 0x2;
pub const BE_CMD_MAX_DRV_VERSION: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_sethost_req {
    pub param_id: u32,
    pub param_len: u32,
    pub param_data: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_sethost_resp {
    pub rsvd0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_set_host_data {
    pub req_hdr: be_cmd_req_hdr,
    pub resp_hdr: be_cmd_resp_hdr,
    pub h: },
    pub req: be_sethost_req,
    pub resp: be_sethost_resp,
    pub param: },
    pub __packed: },
// COMMON SET Features
pub const BE_CMD_SET_FEATURE_UER: c_uint = 0x10;
pub const BE_CMD_UER_SUPP_BIT: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_uer_req {
    pub uer: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_uer_resp {
    pub uer: u32,
    pub ue2rp: u16,
    pub ue2sr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_set_features {
    pub req_hdr: be_cmd_req_hdr,
    pub resp_hdr: be_cmd_resp_hdr,
    pub h: },
    pub feature: u32,
    pub param_len: u32,
    pub req: be_uer_req,
    pub resp: be_uer_resp,
    pub rsvd: [u32; 2],
    pub param: },
    pub __packed: },
    pub phba): *mut int beiscsi_cmd_function_reset(struct beiscsi_hba,
    pub load): *mut *mut int beiscsi_cmd_special_wrb(struct be_ctrl_info ctrl, u32,
    pub phba): *mut int beiscsi_check_fw_rdy(struct beiscsi_hba,
    pub phba): *mut int beiscsi_init_sliport(struct beiscsi_hba,
    pub ulp_num): *mut *mut int beiscsi_cmd_iscsi_cleanup(struct beiscsi_hba phba, unsigned short,
    pub phba): *mut int beiscsi_detect_ue(struct beiscsi_hba,
    pub phba): *mut int beiscsi_detect_tpe(struct beiscsi_hba,
    pub eq_delay): *mut *mut be_queue_info eq, int,
    pub num_cqe_dma_coalesce): c_int,
    pub type): c_int,
    pub cq): *mut be_queue_info,
    pub tag): *mut *mut void free_mcc_wrb(struct be_ctrl_info ctrl, unsigned int,
    pub num): c_int,
    pub mbx_cmd_mem): *mut be_dma_mem,
    pub mbx_cmd_mem): *mut be_dma_mem,
    pub mbox_mem): *mut *mut be_mcc_wrb wrb_from_mbox(be_dma_mem,
    pub tag): *mut *mut void be_mcc_notify(struct beiscsi_hba phba, unsigned int,
    pub ref_tag): *mut c_uint,
    pub compl): *mut be_mcc_compl,
    pub compl): *mut be_mcc_compl,
    pub ulp_num): u8,
    pub q_mem): *mut be_dma_mem,
    pub ctrl): *mut int be_cmd_iscsi_remove_template_hdr(struct be_ctrl_info,
    pub num_pages): u32,
    pub ulp_num): u8,
// Configuration Functions
    pub vlan_tag): *mut *mut int be_cmd_set_vlan(struct beiscsi_hba phba, uint16_t,
    pub phba): *mut beiscsi_hba,
    pub phba): *mut *mut int beiscsi_get_fw_config(struct be_ctrl_info ctrl, struct beiscsi_hba,
    pub phba): *mut *mut int beiscsi_get_port_name(struct be_ctrl_info ctrl, struct beiscsi_hba,
    pub phba): *mut int beiscsi_set_uer_feature(struct beiscsi_hba,
    pub phba): *mut int beiscsi_set_host_data(struct beiscsi_hba,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_default_pdu_context {
    pub dw: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_be_default_pdu_context {
    pub /: *mut *mut u8 dbuf_cindex[13]; / dword 0,
    pub /: *mut *mut u8 rsvd0[3]; / dword 0,
    pub /: *mut *mut u8 ring_size[4]; / dword 0,
    pub /: *mut *mut u8 ring_state[4]; / dword 0,
    pub /: *mut *mut u8 rsvd1[8]; / dword 0,
    pub /: *mut *mut u8 dbuf_pindex[13]; / dword 1,
    pub /: *mut *mut u8 rsvd2; / dword 1,
    pub /: *mut *mut u8 pci_func_id[8]; / dword 1,
    pub /: *mut *mut u8 rx_pdid[9]; / dword 1,
    pub /: *mut *mut u8 rx_pdid_valid; / dword 1,
    pub /: *mut *mut u8 default_buffer_size[16]; / dword 2,
    pub /: *mut *mut u8 cq_id_recv[10]; / dword 2,
    pub /: *mut *mut u8 rx_pdid_not_valid; / dword 2,
    pub /: *mut *mut u8 rsvd3[5]; / dword 2,
    pub /: *mut *mut u8 rsvd4[32]; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_default_pdu_context_ext {
    pub /: *mut *mut u8 rsvd0[16]; / dword 0,
    pub /: *mut *mut u8 ring_size[4]; / dword 0,
    pub /: *mut *mut u8 rsvd1[12]; / dword 0,
    pub /: *mut *mut u8 rsvd2[22]; / dword 1,
    pub /: *mut *mut u8 rx_pdid[9]; / dword 1,
    pub /: *mut *mut u8 rx_pdid_valid; / dword 1,
    pub /: *mut *mut u8 default_buffer_size[16]; / dword 2,
    pub /: *mut *mut u8 cq_id_recv[16]; / dword 2,
    pub /: *mut *mut u8 rsvd3[32]; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_defq_create_req {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub ulp_num: u8,

    pub dua_feature: u8,
    pub context: be_default_pdu_context,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_defq_create_resp {
    pub hdr: be_cmd_req_hdr,
    pub id: u16,
    pub rsvd0: u8,
    pub ulp_num: u8,
    pub doorbell_offset: u32,
    pub register_set: u16,
    pub doorbell_format: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_post_template_pages_req {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
pub const BEISCSI_TEMPLATE_HDR_TYPE_ISCSI: c_uint = 0x1;
    pub type: u16,
    pub scratch_pa: phys_addr,
    pub scratch_va: virt_addr,
    pub pages_va: virt_addr,
    pub pages: [phys_addr; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_remove_template_pages_req {
    pub hdr: be_cmd_req_hdr,
    pub type: u16,
    pub rsvd0: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_post_sgl_pages_req {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub page_offset: u16,
    pub rsvd0: u32,
    pub pages: [phys_addr; 26],
    pub rsvd1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_wrbq_create_req {
    pub hdr: be_cmd_req_hdr,
    pub num_pages: u16,
    pub ulp_num: u8,
    pub dua_feature: u8,
    pub pages: [phys_addr; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_wrbq_create_resp {
    pub resp_hdr: be_cmd_resp_hdr,
    pub cid: u16,
    pub rsvd0: u8,
    pub ulp_num: u8,
    pub doorbell_offset: u32,
    pub register_set: u16,
    pub doorbell_format: u16,
    pub __packed: },
pub const SOL_CID_MASK: c_uint = 0x0000FFC0;
pub const SOL_CODE_MASK: c_uint = 0x0000003F;
pub const SOL_WRB_INDEX_MASK: c_uint = 0x00FF0000;
pub const SOL_CMD_WND_MASK: c_uint = 0xFF000000;
pub const SOL_RES_CNT_MASK: c_uint = 0x7FFFFFFF;
pub const SOL_EXP_CMD_SN_MASK: c_uint = 0xFFFFFFFF;
pub const SOL_HW_STS_MASK: c_uint = 0x000000FF;
pub const SOL_STS_MASK: c_uint = 0x0000FF00;
pub const SOL_RESP_MASK: c_uint = 0x00FF0000;
pub const SOL_FLAGS_MASK: c_uint = 0x7F000000;
pub const SOL_S_MASK: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sol_cqe {
    pub dw: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_sol_cqe {
    pub /: *mut *mut u8 hw_sts[8]; / dword 0,
    pub /: *mut *mut u8 i_sts[8]; / dword 0,
    pub /: *mut *mut u8 i_resp[8]; / dword 0,
    pub /: *mut *mut u8 i_flags[7]; / dword 0,
    pub /: *mut *mut u8 s; / dword 0,
    pub /: *mut *mut u8 i_exp_cmd_sn[32]; / dword 1,
    pub /: *mut *mut u8 code[6]; / dword 2,
    pub /: *mut *mut u8 cid[10]; / dword 2,
    pub /: *mut *mut u8 wrb_index[8]; / dword 2,
    pub /: *mut *mut u8 i_cmd_wnd[8]; / dword 2,
    pub /: *mut *mut u8 i_res_cnt[31]; / dword 3,
    pub /: *mut *mut u8 valid; / dword 3,
    pub __packed: },
pub const SOL_ICD_INDEX_MASK: c_uint = 0x0003FFC0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_sol_cqe_ring {
    pub /: *mut *mut u8 hw_sts[8]; / dword 0,
    pub /: *mut *mut u8 i_sts[8]; / dword 0,
    pub /: *mut *mut u8 i_resp[8]; / dword 0,
    pub /: *mut *mut u8 i_flags[7]; / dword 0,
    pub /: *mut *mut u8 s; / dword 0,
    pub /: *mut *mut u8 i_exp_cmd_sn[32]; / dword 1,
    pub /: *mut *mut u8 code[6]; / dword 2,
    pub /: *mut *mut u8 icd_index[12]; / dword 2,
    pub /: *mut *mut u8 rsvd[6]; / dword 2,
    pub /: *mut *mut u8 i_cmd_wnd[8]; / dword 2,
    pub /: *mut *mut u8 i_res_cnt[31]; / dword 3,
    pub /: *mut *mut u8 valid; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_sol_cqe_v2 {
    pub /: *mut *mut u8 hw_sts[8]; / dword 0,
    pub /: *mut *mut u8 i_sts[8]; / dword 0,
    pub /: *mut *mut u8 wrb_index[16]; / dword 0,
    pub /: *mut *mut u8 i_exp_cmd_sn[32]; / dword 1,
    pub /: *mut *mut u8 code[6]; / dword 2,
    pub /: *mut *mut u8 cmd_cmpl; / dword 2,
    pub /: *mut *mut u8 rsvd0; / dword 2,
    pub /: *mut *mut u8 i_cmd_wnd[8]; / dword 2,
    pub /: *mut *mut u8 cid[13]; / dword 2,
    pub /: *mut *mut u8 u; / dword 2,
    pub /: *mut *mut u8 o; / dword 2,
    pub /: *mut *mut u8 s; / dword 2,
    pub /: *mut *mut u8 i_res_cnt[31]; / dword 3,
    pub /: *mut *mut u8 valid; / dword 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_sol_cqe {
    pub exp_cmdsn: u32,
    pub res_cnt: u32,
    pub wrb_index: u16,
    pub cid: u16,
    pub hw_sts: u8,
    pub cmd_wnd: u8,
    pub /: *mut *mut u8 res_flag; / the s feild of structure,
    pub /: *mut *mut u8 i_resp; / for skh if cmd_complete is set then i_sts is response,
    pub /: *mut *mut u8 i_flags; / for skh or the u and o feilds,
    pub /: *mut *mut u8 i_sts; / for skh if cmd_complete is not-set then i_sts is status,
}

// iSCSI ack/driver message completions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_it_dmsg_cqe {
    pub /: *mut *mut u8 ack_num[32]; / DWORD 0,
    pub /: *mut *mut u8 pdu_bytes_rcvd[32]; / DWORD 1,
    pub /: *mut *mut u8 code[6]; / DWORD 2,
    pub /: *mut *mut u8 cid[10]; / DWORD 2,
    pub /: *mut *mut u8 wrb_idx[8]; / DWORD 2,
    pub 2*/: *mut *mut u8 rsvd0[8]; / DWORD,
    pub 3*/: *mut *mut u8 rsvd1[31]; / DWORD,
    pub /: *mut *mut u8 valid; / DWORD 3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_it_dmsg_cqe_v2 {
    pub /: *mut *mut u8 ack_num[32]; / DWORD 0,
    pub /: *mut *mut u8 pdu_bytes_rcvd[32]; / DWORD 1,
    pub /: *mut *mut u8 code[6]; / DWORD 2,
    pub /: *mut *mut u8 rsvd0[10]; / DWORD 2,
    pub /: *mut *mut u8 wrb_idx[16]; / DWORD 2,
    pub /: *mut *mut u8 rsvd1[16]; / DWORD 3,
    pub /: *mut *mut u8 cid[13]; / DWORD 3,
    pub /: *mut *mut u8 rsvd2[2]; / DWORD 3,
    pub /: *mut *mut u8 valid; / DWORD 3,
    pub __packed: },
//
// Post WRB Queue Doorbell Register used by the host Storage
// stack to notify the
// controller of a posted Work Request Block
//
pub const DB_WRB_POST_CID_MASK: c_uint = 0xFFFF	/* bits 0 - 16 */;
pub const DB_DEF_PDU_WRB_INDEX_MASK: c_uint = 0xFF	/* bits 0 - 9 */;
pub const DB_DEF_PDU_WRB_INDEX_SHIFT: c_int = 16;
pub const DB_DEF_PDU_NUM_POSTED_SHIFT: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fragnum_bits_for_sgl_cra_in {
    pub hdr: be_cmd_req_hdr,
    pub num_bits: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cleanup_req {
    pub hdr: be_cmd_req_hdr,
    pub chute: u16,
    pub hdr_ring_id: u8,
    pub data_ring_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cleanup_req_v1 {
    pub hdr: be_cmd_req_hdr,
    pub chute: u16,
    pub rsvd1: u16,
    pub hdr_ring_id: u16,
    pub rsvd2: u16,
    pub data_ring_id: u16,
    pub rsvd3: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq_delay {
    pub eq_id: u32,
    pub phase: u32,
    pub delay_multiplier: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eq_delay_params_in {
    pub hdr: be_cmd_req_hdr,
    pub num_eq: u32,
    pub delay: [eq_delay; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_connect_and_offload_in {
    pub hdr: be_cmd_req_hdr,
    pub ip_address: ip_addr_format,
    pub tcp_port: u16,
    pub cid: u16,
    pub cq_id: u16,
    pub defq_id: u16,
    pub dataout_template_pa: phys_addr,
    pub hdr_ring_id: u16,
    pub data_ring_id: u16,
    pub do_offload: u8,
    pub rsvd0: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_connect_and_offload_in_v1 {
    pub hdr: be_cmd_req_hdr,
    pub ip_address: ip_addr_format,
    pub tcp_port: u16,
    pub cid: u16,
    pub cq_id: u16,
    pub defq_id: u16,
    pub dataout_template_pa: phys_addr,
    pub hdr_ring_id: u16,
    pub data_ring_id: u16,
    pub do_offload: u8,
    pub ifd_state: u8,
    pub rsvd0: [u8; 2],
    pub tcp_window_size: u16,
    pub tcp_window_scale_count: u8,
    pub rsvd1: u8,
    pub tcp_mss:24: u32,
    pub rsvd2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_connect_and_offload_out {
    pub hdr: be_cmd_resp_hdr,
    pub connection_handle: u32,
    pub cid: u16,
    pub rsvd0: u16,
    pub __packed: },
pub const DB_DEF_PDU_RING_ID_MASK: c_uint = 0x3FFF	/* bits 0 - 13 */;
pub const DB_DEF_PDU_CQPROC_MASK: c_uint = 0x3FFF	/* bits 16 - 29 */;
pub const DB_DEF_PDU_REARM_SHIFT: c_int = 14;
pub const DB_DEF_PDU_EVENT_SHIFT: c_int = 15;
pub const DB_DEF_PDU_CQPROC_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_invalidate_connection_params_in {
    pub hdr: be_cmd_req_hdr,
    pub session_handle: u32,
    pub cid: u16,
    pub unused: u16,
pub const BE_CLEANUP_TYPE_INVALIDATE: c_uint = 0x8001;
pub const BE_CLEANUP_TYPE_ISSUE_TCP_RST: c_uint = 0x8002;
    pub cleanup_type: u16,
    pub save_cfg: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_invalidate_connection_params_out {
    pub session_handle: u32,
    pub cid: u16,
    pub unused: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union be_invalidate_connection_params {
    pub req: be_invalidate_connection_params_in,
    pub resp: be_invalidate_connection_params_out,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_tcp_upload_params_in {
    pub hdr: be_cmd_req_hdr,
    pub id: u16,
pub const BE_UPLOAD_TYPE_GRACEFUL: c_int = 1;
// abortive upload with reset
pub const BE_UPLOAD_TYPE_ABORT_RESET: c_int = 2;
// abortive upload without reset
pub const BE_UPLOAD_TYPE_ABORT: c_int = 3;
// abortive upload with reset, sequence number by driver
pub const BE_UPLOAD_TYPE_ABORT_WITH_SEQ: c_int = 4;
    pub upload_type: u16,
    pub reset_seq: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_tcp_upload_params_out {
    pub dw: [u32; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union be_tcp_upload_params {
    pub request: be_tcp_upload_params_in,
    pub response: be_tcp_upload_params_out,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ulp_fw_cfg {
pub const BEISCSI_ULP_ISCSI_INI_MODE: c_uint = 0x10;
    pub ulp_mode: u32,
    pub etx_base: u32,
    pub etx_count: u32,
    pub sq_base: u32,
    pub sq_count: u32,
    pub rq_base: u32,
    pub rq_count: u32,
    pub dq_base: u32,
    pub dq_count: u32,
    pub lro_base: u32,
    pub lro_count: u32,
    pub icd_base: u32,
    pub icd_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ulp_chain_icd {
    pub chain_base: u32,
    pub chain_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_fw_cfg {
    pub hdr: be_cmd_req_hdr,
    pub be_config_number: u32,
    pub asic_revision: u32,
    pub phys_port: u32,
pub const BEISCSI_FUNC_ISCSI_INI_MODE: c_uint = 0x10;
pub const BEISCSI_FUNC_DUA_MODE: c_uint = 0x800;
    pub function_mode: u32,
    pub ulp: [be_ulp_fw_cfg; 2],
    pub function_caps: u32,
    pub cqid_base: u32,
    pub cqid_count: u32,
    pub eqid_base: u32,
    pub eqid_count: u32,
    pub chain_icd: [be_ulp_chain_icd; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_all_if_id_req {
    pub hdr: be_cmd_req_hdr,
    pub if_count: u32,
    pub if_hndl_list: [u32; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_get_port_name {
    pub req_hdr: be_cmd_req_hdr,
    pub resp_hdr: be_cmd_resp_hdr,
    pub h: },
    pub reserved: u32,
    pub req: },
    pub port_names: u32,
    pub resp: },
    pub p: },
    pub __packed: },
pub const ISCSI_OPCODE_SCSI_DATA_OUT: c_int = 5;
pub const OPCODE_COMMON_NTWK_LINK_STATUS_QUERY: c_int = 5;
pub const OPCODE_COMMON_MODIFY_EQ_DELAY: c_int = 41;
pub const OPCODE_COMMON_ISCSI_CLEANUP: c_int = 59;
pub const OPCODE_COMMON_TCP_UPLOAD: c_int = 56;
pub const OPCODE_COMMON_ISCSI_TCP_CONNECT_AND_OFFLOAD: c_int = 70;
pub const OPCODE_COMMON_ISCSI_ERROR_RECOVERY_INVALIDATE_COMMANDS: c_int = 1;
pub const OPCODE_ISCSI_INI_CFG_GET_HBA_NAME: c_int = 6;
pub const OPCODE_ISCSI_INI_CFG_SET_HBA_NAME: c_int = 7;
pub const OPCODE_ISCSI_INI_SESSION_GET_A_SESSION: c_int = 14;
pub const OPCODE_ISCSI_INI_SESSION_LOGOUT_TARGET: c_int = 24;
pub const OPCODE_ISCSI_INI_DRIVER_REOPEN_ALL_SESSIONS: c_int = 36;
pub const OPCODE_ISCSI_INI_DRIVER_OFFLOAD_SESSION: c_int = 41;
pub const OPCODE_ISCSI_INI_DRIVER_INVALIDATE_CONNECTION: c_int = 42;
pub const OPCODE_ISCSI_INI_BOOT_GET_BOOT_TARGET: c_int = 52;
pub const OPCODE_COMMON_WRITE_FLASH: c_int = 96;
pub const OPCODE_COMMON_READ_FLASH: c_int = 97;
pub const CMD_ISCSI_COMMAND_INVALIDATE: c_int = 1;

// a read command
//

// prepared by driver should not
// be touched.
//
// Returns the number of items in the field array.

//
// Different types of iSCSI completions to host driver for both initiator
// and taget mode
// of operation.
//

// normally
//

// invalidated internally due
// to Data Digest error
//

// internally
// due to a received PDU
// size > DSL
//

// internally due ti received
// PDU sequence size >
// FBL/MBL.
//

// internally due to a received
// PDU Hdr that has
// AHS

// internally due to Hdr Digest
// error
//

// internally
// due to a bad opcode in the
// pdu hdr
//

// internally due to a received
// ITT/TTT that does not belong
// to this Connection
//

// internally due to received
// ITT/TTT value > Max
// Supported ITTs/TTTs
//

// internally due to an
// incoming TCP RST
//

// internally due to timeout on
// tcp segment 12 retransmit
// attempts failed
//

// internally due to TCP RST
// sent by the Tx side
//

// internally due to an
// incoming TCP FIN.
//

// internally due to bad
// unsolicited PDU Unsolicited
// PDUs are PDUs with
// ITT=0xffffffff
//

// internally due to bad WRB
// index.
//

// internally due to received
// command has residual
// over run bytes.
//

// internally due to received
// command has residual under
// run bytes.
//

// internally due to a received
// PDU has an invalid StatusSN
//

// internally due to a received
// an R2T with some invalid
// fields in it
//

// internally due to received
// PDU has an invalid LUN.
//

// internally due to the
// corresponding ICD not in a
// valid state
//

// to received PDU has an
// invalid ITT.
//

// to received sequence buffer
// offset is out of order.
//

// internally due to a
// received PDU has an invalid
// DataSN
//

// completion notify.
//

// completion
// with data PDU index.
//

// completionnotifify.
//

// error notify.
//

// notification.
//

// internally due to command
// and data are not on same
// connection.
//

// invalidated internally due
// to DIF error
//

// internally due to incoming
// TCP SYN
//

// internally due to an
// incoming Unsolicited PDU
// that has immediate data on
// the cxn
//
    pub sge_cnt): bool embedded, u8,
    pub cmd_len): u8 subsystem, u8 opcode, u32,
