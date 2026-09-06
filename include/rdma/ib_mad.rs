//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_mad.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2004 Mellanox Technologies Ltd.  All rights reserved.
// Copyright (c) 2004 Infinicon Corporation.  All rights reserved.
// Copyright (c) 2004 Intel Corporation.  All rights reserved.
// Copyright (c) 2004 Topspin Corporation.  All rights reserved.
// Copyright (c) 2004-2006 Voltaire Corporation.  All rights reserved.
//

// Management base versions
pub const IB_MGMT_BASE_VERSION: c_int = 1;
pub const OPA_MGMT_BASE_VERSION: c_uint = 0x80;
pub const OPA_SM_CLASS_VERSION: c_uint = 0x80;
// Management classes
pub const IB_MGMT_CLASS_SUBN_LID_ROUTED: c_uint = 0x01;
pub const IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE: c_uint = 0x81;
pub const IB_MGMT_CLASS_SUBN_ADM: c_uint = 0x03;
pub const IB_MGMT_CLASS_PERF_MGMT: c_uint = 0x04;
pub const IB_MGMT_CLASS_BM: c_uint = 0x05;
pub const IB_MGMT_CLASS_DEVICE_MGMT: c_uint = 0x06;
pub const IB_MGMT_CLASS_CM: c_uint = 0x07;
pub const IB_MGMT_CLASS_SNMP: c_uint = 0x08;
pub const IB_MGMT_CLASS_DEVICE_ADM: c_uint = 0x10;
pub const IB_MGMT_CLASS_BOOT_MGMT: c_uint = 0x11;
pub const IB_MGMT_CLASS_BIS: c_uint = 0x12;
pub const IB_MGMT_CLASS_CONG_MGMT: c_uint = 0x21;
pub const IB_MGMT_CLASS_VENDOR_RANGE2_START: c_uint = 0x30;
pub const IB_MGMT_CLASS_VENDOR_RANGE2_END: c_uint = 0x4F;

// Management methods
pub const IB_MGMT_METHOD_GET: c_uint = 0x01;
pub const IB_MGMT_METHOD_SET: c_uint = 0x02;
pub const IB_MGMT_METHOD_GET_RESP: c_uint = 0x81;
pub const IB_MGMT_METHOD_SEND: c_uint = 0x03;
pub const IB_MGMT_METHOD_TRAP: c_uint = 0x05;
pub const IB_MGMT_METHOD_REPORT: c_uint = 0x06;
pub const IB_MGMT_METHOD_REPORT_RESP: c_uint = 0x86;
pub const IB_MGMT_METHOD_TRAP_REPRESS: c_uint = 0x07;
pub const IB_MGMT_METHOD_GET_TABLE: c_uint = 0x12;
pub const IB_MGMT_METHOD_RESP: c_uint = 0x80;

pub const IB_MGMT_MAX_METHODS: c_int = 128;
// MAD Status field bit masks
pub const IB_MGMT_MAD_STATUS_SUCCESS: c_uint = 0x0000;
pub const IB_MGMT_MAD_STATUS_BUSY: c_uint = 0x0001;
pub const IB_MGMT_MAD_STATUS_REDIRECT_REQD: c_uint = 0x0002;
pub const IB_MGMT_MAD_STATUS_BAD_VERSION: c_uint = 0x0004;
pub const IB_MGMT_MAD_STATUS_UNSUPPORTED_METHOD: c_uint = 0x0008;
pub const IB_MGMT_MAD_STATUS_UNSUPPORTED_METHOD_ATTRIB: c_uint = 0x000c;
pub const IB_MGMT_MAD_STATUS_INVALID_ATTRIB_VALUE: c_uint = 0x001c;
// RMPP information
pub const IB_MGMT_RMPP_VERSION: c_int = 1;
pub const IB_MGMT_RMPP_TYPE_DATA: c_int = 1;
pub const IB_MGMT_RMPP_TYPE_ACK: c_int = 2;
pub const IB_MGMT_RMPP_TYPE_STOP: c_int = 3;
pub const IB_MGMT_RMPP_TYPE_ABORT: c_int = 4;
pub const IB_MGMT_RMPP_FLAG_ACTIVE: c_int = 1;

pub const IB_MGMT_RMPP_NO_RESPTIME: c_uint = 0x1F;
pub const IB_MGMT_RMPP_STATUS_SUCCESS: c_int = 0;
pub const IB_MGMT_RMPP_STATUS_RESX: c_int = 1;
pub const IB_MGMT_RMPP_STATUS_ABORT_MIN: c_int = 118;
pub const IB_MGMT_RMPP_STATUS_T2L: c_int = 118;
pub const IB_MGMT_RMPP_STATUS_BAD_LEN: c_int = 119;
pub const IB_MGMT_RMPP_STATUS_BAD_SEG: c_int = 120;
pub const IB_MGMT_RMPP_STATUS_BADT: c_int = 121;
pub const IB_MGMT_RMPP_STATUS_W2S: c_int = 122;
pub const IB_MGMT_RMPP_STATUS_S2B: c_int = 123;
pub const IB_MGMT_RMPP_STATUS_BAD_STATUS: c_int = 124;
pub const IB_MGMT_RMPP_STATUS_UNV: c_int = 125;
pub const IB_MGMT_RMPP_STATUS_TMR: c_int = 126;
pub const IB_MGMT_RMPP_STATUS_UNSPEC: c_int = 127;
pub const IB_MGMT_RMPP_STATUS_ABORT_MAX: c_int = 127;
pub const IB_QP0: c_int = 0;

pub const IB_QP1_QKEY: c_uint = 0x80010000;
pub const IB_QP_SET_QKEY: c_uint = 0x80000000;
pub const IB_DEFAULT_PKEY_PARTIAL: c_uint = 0x7FFF;
pub const IB_DEFAULT_PKEY_FULL: c_uint = 0xFFFF;
//
// Generic trap/notice types
//
pub const IB_NOTICE_TYPE_FATAL: c_uint = 0x80;
pub const IB_NOTICE_TYPE_URGENT: c_uint = 0x81;
pub const IB_NOTICE_TYPE_SECURITY: c_uint = 0x82;
pub const IB_NOTICE_TYPE_SM: c_uint = 0x83;
pub const IB_NOTICE_TYPE_INFO: c_uint = 0x84;
//
// Generic trap/notice producers
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_hdr {
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: __be16,
    pub class_specific: __be16,
    pub tid: __be64,
    pub attr_id: __be16,
    pub resv: __be16,
    pub attr_mod: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rmpp_hdr {
    pub rmpp_version: u8,
    pub rmpp_type: u8,
    pub rmpp_rtime_flags: u8,
    pub rmpp_status: u8,
    pub seg_num: __be32,
    pub paylen_newwin: __be32,
}

pub type ib_sa_comp_mask = u64 ;

//
// ib_sa_hdr and ib_sa_mad structures must be packed because they have
// 64-bit fields that are only 32-bit aligned. 64-bit architectures will
// lay them out wrong otherwise.  (And unfortunately they are sent on
// the wire so we can't change the layout)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sa_hdr {
    pub sm_key: __be64,
    pub attr_offset: __be16,
    pub reserved: __be16,
    pub comp_mask: ib_sa_comp_mask,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad {
    pub mad_hdr: ib_mad_hdr,
    pub data: [u8; IB_MGMT_MAD_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_mad {
    pub mad_hdr: ib_mad_hdr,
    pub data: [u8; OPA_MGMT_MAD_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rmpp_mad {
    pub mad_hdr: ib_mad_hdr,
    pub rmpp_hdr: ib_rmpp_hdr,
    pub data: [u8; IB_MGMT_RMPP_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_rmpp_mad {
    pub mad_hdr: ib_mad_hdr,
    pub rmpp_hdr: ib_rmpp_hdr,
    pub data: [u8; OPA_MGMT_RMPP_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sa_mad {
    pub mad_hdr: ib_mad_hdr,
    pub rmpp_hdr: ib_rmpp_hdr,
    pub sa_hdr: ib_sa_hdr,
    pub data: [u8; IB_MGMT_SA_DATA],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_vendor_mad {
    pub mad_hdr: ib_mad_hdr,
    pub rmpp_hdr: ib_rmpp_hdr,
    pub reserved: u8,
    pub oui: [u8; 3],
    pub data: [u8; IB_MGMT_VENDOR_DATA],
}

pub const IB_CLASS_PORT_INFO_RESP_TIME_MASK: c_uint = 0x1F;
pub const IB_CLASS_PORT_INFO_RESP_TIME_FIELD_SIZE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_class_port_info {
    pub base_version: u8,
    pub class_version: u8,
    pub capability_mask: __be16,
// 27 bits for cap_mask2, 5 bits for resp_time
    pub cap_mask2_resp_time: __be32,
    pub redirect_gid: [u8; 16],
    pub redirect_tcslfl: __be32,
    pub redirect_lid: __be16,
    pub redirect_pkey: __be16,
    pub redirect_qp: __be32,
    pub redirect_qkey: __be32,
    pub trap_gid: [u8; 16],
    pub trap_tcslfl: __be32,
    pub trap_lid: __be16,
    pub trap_pkey: __be16,
    pub trap_hlqp: __be32,
    pub trap_qkey: __be32,
}

// PortInfo CapabilityMask
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_capability_mask_bits {
    IB_PORT_SM = 1 << 1,
    IB_PORT_NOTICE_SUP = 1 << 2,
    IB_PORT_TRAP_SUP = 1 << 3,
    IB_PORT_OPT_IPD_SUP = 1 << 4,
    IB_PORT_AUTO_MIGR_SUP = 1 << 5,
    IB_PORT_SL_MAP_SUP = 1 << 6,
    IB_PORT_MKEY_NVRAM = 1 << 7,
    IB_PORT_PKEY_NVRAM = 1 << 8,
    IB_PORT_LED_INFO_SUP = 1 << 9,
    IB_PORT_SM_DISABLED = 1 << 10,
    IB_PORT_SYS_IMAGE_GUID_SUP = 1 << 11,
    IB_PORT_PKEY_SW_EXT_PORT_TRAP_SUP = 1 << 12,
    IB_PORT_EXTENDED_SPEEDS_SUP = 1 << 14,
    IB_PORT_CAP_MASK2_SUP = 1 << 15,
    IB_PORT_CM_SUP = 1 << 16,
    IB_PORT_SNMP_TUNNEL_SUP = 1 << 17,
    IB_PORT_REINIT_SUP = 1 << 18,
    IB_PORT_DEVICE_MGMT_SUP = 1 << 19,
    IB_PORT_VENDOR_CLASS_SUP = 1 << 20,
    IB_PORT_DR_NOTICE_SUP = 1 << 21,
    IB_PORT_CAP_MASK_NOTICE_SUP = 1 << 22,
    IB_PORT_BOOT_MGMT_SUP = 1 << 23,
    IB_PORT_LINK_LATENCY_SUP = 1 << 24,
    IB_PORT_CLIENT_REG_SUP = 1 << 25,
    IB_PORT_OTHER_LOCAL_CHANGES_SUP = 1 << 26,
    IB_PORT_LINK_SPEED_WIDTH_TABLE_SUP = 1 << 27,
    IB_PORT_VENDOR_SPECIFIC_MADS_TABLE_SUP = 1 << 28,
    IB_PORT_MCAST_PKEY_TRAP_SUPPRESSION_SUP = 1 << 29,
    IB_PORT_MCAST_FDB_TOP_SUP = 1 << 30,
    IB_PORT_HIERARCHY_INFO_SUP = 1ULL << 31,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_capability_mask2_bits {
    IB_PORT_SET_NODE_DESC_SUP		= 1 << 0,
    IB_PORT_EX_PORT_INFO_EX_SUP		= 1 << 1,
    IB_PORT_VIRT_SUP			= 1 << 2,
    IB_PORT_SWITCH_PORT_STATE_TABLE_SUP	= 1 << 3,
    IB_PORT_LINK_WIDTH_2X_SUP		= 1 << 4,
    IB_PORT_LINK_SPEED_HDR_SUP		= 1 << 5,
    IB_PORT_LINK_SPEED_NDR_SUP		= 1 << 10,
    IB_PORT_EXTENDED_SPEEDS2_SUP            = 1 << 11,
    IB_PORT_LINK_SPEED_XDR_SUP              = 1 << 12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_class_port_info {
    pub base_version: u8,
    pub class_version: u8,
    pub cap_mask: __be16,
    pub cap_mask2_resp_time: __be32,
    pub redirect_gid: [u8; 16],
    pub redirect_tc_fl: __be32,
    pub redirect_lid: __be32,
    pub redirect_sl_qp: __be32,
    pub redirect_qkey: __be32,
    pub trap_gid: [u8; 16],
    pub trap_tc_fl: __be32,
    pub trap_lid: __be32,
    pub trap_hl_qp: __be32,
    pub trap_qkey: __be32,
    pub trap_pkey: __be16,
    pub redirect_pkey: __be16,
    pub trap_sl_rsvd: u8,
    pub reserved: [u8; 3],
    pub __packed: },
//
// ib_get_cpi_resp_time - Returns the resp_time value from
// cap_mask2_resp_time in ib_class_port_info.
// @cpi: A struct ib_class_port_info mad.
//
// ib_set_cpi_resptime - Sets the response time in an
// ib_class_port_info mad.
// @cpi: A struct ib_class_port_info.
// @rtime: The response time to set.
//
    pub IB_CLASS_PORT_INFO_RESP_TIME_MASK): cpu_to_be32(rtime &,
//
// ib_get_cpi_capmask2 - Returns the capmask2 value from
// cap_mask2_resp_time in ib_class_port_info.
// @cpi: A struct ib_class_port_info mad.
//
// ib_set_cpi_capmask2 - Sets the capmask2 in an
// ib_class_port_info mad.
// @cpi: A struct ib_class_port_info.
// @capmask2: The capmask2 to set.
//
// opa_get_cpi_capmask2 - Returns the capmask2 value from
// cap_mask2_resp_time in ib_class_port_info.
// @cpi: A struct opa_class_port_info mad.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_notice_attr {
    pub generic_type: u8,
    pub prod_type_msb: u8,
    pub prod_type_lsb: __be16,
    pub trap_num: __be16,
    pub issuer_lid: __be16,
    pub toggle_count: __be16,
    pub details: [u8; 54],
    pub raw_data: },
    pub reserved: __be16,
    pub /: *mut *mut __be16 lid; / where violation happened,
    pub /: *mut *mut u8 port_num; / where violation happened,
    pub ntc_129_131: } __packed,
    pub reserved: __be16,
    pub /: *mut *mut __be16 lid; / LID where change occurred,
    pub reserved2: u8,
    pub /: *mut *mut u8 local_changes; / low bit - local changes,
    pub /: *mut *mut __be32 new_cap_mask; / new capability mask,
    pub reserved3: u8,
    pub /: *mut *mut u8 change_flags; / low 3 bits only,
    pub ntc_144: } __packed,
    pub reserved: __be16,
    pub /: *mut *mut __be16 lid; / lid where sys guid changed,
    pub reserved2: __be16,
    pub new_sys_guid: __be64,
    pub ntc_145: } __packed,
    pub reserved: __be16,
    pub lid: __be16,
    pub dr_slid: __be16,
    pub method: u8,
    pub reserved2: u8,
    pub attr_id: __be16,
    pub attr_mod: __be32,
    pub mkey: __be64,
    pub reserved3: u8,
    pub dr_trunc_hop: u8,
    pub dr_rtn_path: [u8; 30],
    pub ntc_256: } __packed,
    pub reserved: __be16,
    pub lid1: __be16,
    pub lid2: __be16,
    pub key: __be32,
    pub /: *mut *mut __be32 sl_qp1; / SL: high 4 bits,
    pub /: *mut *mut __be32 qp2; / high 8 bits reserved,
    pub gid1: ib_gid,
    pub gid2: ib_gid,
    pub ntc_257_258: } __packed,
    pub details: },
}

//
// ib_mad_send_buf - MAD data buffer and work request for sends.
// @next: A pointer used to chain together MADs for posting.
// @mad: References an allocated MAD data buffer for MADs that do not have
// RMPP active.  For MADs using RMPP, references the common and management
// class specific headers.
// @mad_agent: MAD agent that allocated the buffer.
// @ah: The address handle to use when sending the MAD.
// @context: User-controlled context fields.
// @hdr_len: Indicates the size of the data header of the MAD.  This length
// includes the common MAD, RMPP, and class specific headers.
// @data_len: Indicates the total size of user-transferred data.
// @seg_count: The number of RMPP segments allocated for this send.
// @seg_size: Size of the data in each RMPP segment.  This does not include
// class specific headers.
// @seg_rmpp_size: Size of each RMPP segment including the class specific
// headers.
// @timeout_ms: Time to wait for a response.
// @retries: Number of times to retry a request for a response.  For MADs
// using RMPP, this applies per window.  On completion, returns the number
// of retries needed to complete the transfer.
//
// Users are responsible for initializing the MAD buffer itself, with the
// exception of any RMPP header.  Additional segment buffer space allocated
// beyond data_len is padding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_send_buf {
    pub next: *mut ib_mad_send_buf,
    pub mad: *mut c_void,
    pub mad_agent: *mut ib_mad_agent,
    pub ah: *mut ib_ah,
    pub context: [*mut c_void; 2],
    pub hdr_len: c_int,
    pub data_len: c_int,
    pub seg_count: c_int,
    pub seg_size: c_int,
    pub seg_rmpp_size: c_int,
    pub timeout_ms: c_int,
    pub retries: c_int,
}

//
// ib_response_mad - Returns if the specified MAD has been generated in
// response to a sent request or trap.
//
extern "C" {
    pub fn ib_response_mad(hdr: *const ib_mad_hdr) -> c_int;
}
//
// ib_get_rmpp_resptime - Returns the RMPP response time.
// @rmpp_hdr: An RMPP header.
//
// ib_get_rmpp_flags - Returns the RMPP flags.
// @rmpp_hdr: An RMPP header.
//
// ib_set_rmpp_resptime - Sets the response time in an RMPP header.
// @rmpp_hdr: An RMPP header.
// @rtime: The response time to set.
//
// ib_set_rmpp_flags - Sets the flags in an RMPP header.
// @rmpp_hdr: An RMPP header.
// @flags: The flags to set.
//
// ib_mad_send_handler - callback handler for a sent MAD.
// @mad_agent: MAD agent that sent the MAD.
// @mad_send_wc: Send work completion information on the sent MAD.
//
// ib_mad_recv_handler - callback handler for a received MAD.
// @mad_agent: MAD agent requesting the received MAD.
// @send_buf: Send buffer if found, else NULL
// @mad_recv_wc: Received work completion information on the received MAD.
//
// MADs received in response to a send request operation will be handed to
// the user before the send operation completes.  All data buffers given
// to registered agents through this routine are owned by the receiving
// client.
//
// ib_mad_agent - Used to track MAD registration with the access layer.
// @device: Reference to device registration is on.
// @qp: Reference to QP used for sending and receiving MADs.
// @mr: Memory region for system memory usable for DMA.
// @recv_handler: Callback handler for a received MAD.
// @send_handler: Callback handler for a sent MAD.
// @context: User-specified context associated with this registration.
// @hi_tid: Access layer assigned transaction ID for this client.
// Unsolicited MADs sent by this client will have the upper 32-bits
// of their TID set to this value.
// @flags: registration flags
// @port_num: Port number on which QP is registered
// @rmpp_version: If set, indicates the RMPP version used by this agent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_agent {
    pub device: *mut ib_device,
    pub qp: *mut ib_qp,
    pub recv_handler: ib_mad_recv_handler,
    pub send_handler: ib_mad_send_handler,
    pub context: *mut c_void,
    pub hi_tid: u32,
    pub flags: u32,
    pub security: *mut c_void,
    pub mad_agent_sec_list: list_head,
    pub port_num: u8,
    pub rmpp_version: u8,
    pub smp_allowed: bool,
}

//
// ib_mad_send_wc - MAD send completion information.
// @send_buf: Send MAD data buffer associated with the send MAD request.
// @status: Completion status.
// @vendor_err: Optional vendor error information returned with a failed
// request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_send_wc {
    pub send_buf: *mut ib_mad_send_buf,
    pub status: ib_wc_status,
    pub vendor_err: u32,
}

//
// ib_mad_recv_buf - received MAD buffer information.
// @list: Reference to next data buffer for a received RMPP MAD.
// @grh: References a data buffer containing the global route header.
// The data refereced by this buffer is only valid if the GRH is
// valid.
// @mad: References the start of the received MAD.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_recv_buf {
    pub list: list_head,
    pub grh: *mut ib_grh,
    pub mad: *mut ib_mad,
    pub opa_mad: *mut opa_mad,
}

//
// ib_mad_recv_wc - received MAD information.
// @wc: Completion information for the received data.
// @recv_buf: Specifies the location of the received data buffer(s).
// @rmpp_list: Specifies a list of RMPP reassembled received MAD buffers.
// @mad_len: The length of the received MAD, without duplicated headers.
// @mad_seg_size: The size of individual MAD segments
//
// For received response, the wr_id contains a pointer to the ib_mad_send_buf
// for the corresponding send request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_recv_wc {
    pub wc: *mut ib_wc,
    pub recv_buf: ib_mad_recv_buf,
    pub rmpp_list: list_head,
    pub mad_len: c_int,
    pub mad_seg_size: usize,
}

//
// ib_mad_reg_req - MAD registration request
// @mgmt_class: Indicates which management class of MADs should be receive
// by the caller.  This field is only required if the user wishes to
// receive unsolicited MADs, otherwise it should be 0.
// @mgmt_class_version: Indicates which version of MADs for the given
// management class to receive.
// @oui: Indicates IEEE OUI when mgmt_class is a vendor class
// in the range from 0x30 to 0x4f. Otherwise not used.
// @method_mask: The caller will receive unsolicited MADs for any method
// where @method_mask = 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_reg_req {
    pub mgmt_class: u8,
    pub mgmt_class_version: u8,
    pub oui: [u8; 3],
    pub IB_MGMT_MAX_METHODS): DECLARE_BITMAP(method_mask,,
}

//
// ib_register_mad_agent - Register to send/receive MADs.
// @device: The device to register with.
// @port_num: The port on the specified device to use.
// @qp_type: Specifies which QP to access.  Must be either
// IB_QPT_SMI or IB_QPT_GSI.
// @mad_reg_req: Specifies which unsolicited MADs should be received
// by the caller.  This parameter may be NULL if the caller only
// wishes to receive solicited responses.
// @rmpp_version: If set, indicates that the client will send
// and receive MADs that contain the RMPP header for the given version.
// If set to 0, indicates that RMPP is not used by this client.
// @send_handler: The completion callback routine invoked after a send
// request has completed.
// @recv_handler: The completion callback routine invoked for a received
// MAD.
// @context: User specified context associated with the registration.
// @registration_flags: Registration flags to set for this agent
//
// ib_unregister_mad_agent - Unregisters a client from using MAD services.
// @mad_agent: Corresponding MAD registration request to deregister.
//
// After invoking this routine, MAD services are no longer usable by the
// client on the associated QP.
//
extern "C" {
    pub fn ib_unregister_mad_agent(mad_agent: *mut ib_mad_agent);
}
//
// ib_post_send_mad - Posts MAD(s) to the send queue of the QP associated
// with the registered client.
// @send_buf: Specifies the information needed to send the MAD(s).
// @bad_send_buf: Specifies the MAD on which an error was encountered.  This
// parameter is optional if only a single MAD is posted.
//
// Sent MADs are not guaranteed to complete in the order that they were posted.
//
// If the MAD requires RMPP, the data buffer should contain a single copy
// of the common MAD, RMPP, and class specific headers, followed by the class
// defined data.  If the class defined data would not divide evenly into
// RMPP segments, then space must be allocated at the end of the referenced
// buffer for any required padding.  To indicate the amount of class defined
// data being transferred, the paylen_newwin field in the RMPP header should
// be set to the size of the class specific header plus the amount of class
// defined data being transferred.  The paylen_newwin field should be
// specified in network-byte order.
//
// ib_free_recv_mad - Returns data buffers used to receive a MAD.
// @mad_recv_wc: Work completion information for a received MAD.
//
// Clients receiving MADs through their ib_mad_recv_handler must call this
// routine to return the work completion buffers to the access layer.
//
extern "C" {
    pub fn ib_free_recv_mad(mad_recv_wc: *mut ib_mad_recv_wc);
}
//
// ib_modify_mad - Modifies an outstanding send MAD operation.
// @send_buf: Indicates the MAD to modify.
// @timeout_ms: New timeout value for sent MAD.
//
// This call will reset the timeout value for a sent MAD to the specified
// value.
//
extern "C" {
    pub fn ib_modify_mad(send_buf: *mut ib_mad_send_buf, timeout_ms: u32) -> c_int;
}
//
// ib_cancel_mad - Cancels an outstanding send MAD operation.
// @send_buf: Indicates the MAD to cancel.
//
// MADs will be returned to the user through the corresponding
// ib_mad_send_handler.
//
// ib_create_send_mad - Allocate and initialize a data buffer and work request
// for sending a MAD.
// @mad_agent: Specifies the registered MAD service to associate with the MAD.
// @remote_qpn: Specifies the QPN of the receiving node.
// @pkey_index: Specifies which PKey the MAD will be sent using.  This field
// is valid only if the remote_qpn is QP 1.
// @rmpp_active: Indicates if the send will enable RMPP.
// @hdr_len: Indicates the size of the data header of the MAD.  This length
// should include the common MAD header, RMPP header, plus any class
// specific header.
// @data_len: Indicates the size of any user-transferred data.  The call will
// automatically adjust the allocated buffer size to account for any
// additional padding that may be necessary.
// @gfp_mask: GFP mask used for the memory allocation.
// @base_version: Base Version of this MAD
//
// This routine allocates a MAD for sending.  The returned MAD send buffer
// will reference a data buffer usable for sending a MAD, along
// with an initialized work request structure.  Users may modify the returned
// MAD data buffer before posting the send.
//
// The returned MAD header, class specific headers, and any padding will be
// cleared.  Users are responsible for initializing the common MAD header,
// any class specific header, and MAD data area.
// If @rmpp_active is set, the RMPP header will be initialized for sending.
//
// ib_is_mad_class_rmpp - returns whether given management class
// supports RMPP.
// @mgmt_class: management class
//
// This routine returns whether the management class supports RMPP.
//
extern "C" {
    pub fn ib_is_mad_class_rmpp(mgmt_class: u8) -> c_int;
}
//
// ib_get_mad_data_offset - returns the data offset for a given
// management class.
// @mgmt_class: management class
//
// This routine returns the data offset in the MAD for the management
// class requested.
//
extern "C" {
    pub fn ib_get_mad_data_offset(mgmt_class: u8) -> c_int;
}
//
// ib_get_rmpp_segment - returns the data buffer for a given RMPP segment.
// @send_buf: Previously allocated send data buffer.
// @seg_num: number of segment to return
//
// This routine returns a pointer to the data buffer of an RMPP MAD.
// Users must provide synchronization to @send_buf around this call.
//
// ib_free_send_mad - Returns data buffers used to send a MAD.
// @send_buf: Previously allocated send data buffer.
//
extern "C" {
    pub fn ib_free_send_mad(send_buf: *mut ib_mad_send_buf);
}
//
// ib_mad_kernel_rmpp_agent - Returns if the agent is performing RMPP.
// @agent: the agent in question
// @return: true if agent is performing rmpp, false otherwise.
//
extern "C" {
    pub fn ib_mad_kernel_rmpp_agent(agent: *const ib_mad_agent) -> c_int;
}
