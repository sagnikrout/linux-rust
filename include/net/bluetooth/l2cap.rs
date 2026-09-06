//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/l2cap.h
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

// L2CAP defaults
pub const L2CAP_DEFAULT_MTU: c_int = 672;
pub const L2CAP_DEFAULT_MIN_MTU: c_int = 48;

pub const L2CAP_DEFAULT_FLUSH_TO: c_uint = 0xFFFF;
pub const L2CAP_EFS_DEFAULT_FLUSH_TO: c_uint = 0xFFFFFFFF;
pub const L2CAP_DEFAULT_TX_WINDOW: c_int = 63;
pub const L2CAP_DEFAULT_EXT_WINDOW: c_uint = 0x3FFF;
pub const L2CAP_DEFAULT_MAX_TX: c_int = 3;

pub const L2CAP_DEFAULT_ACK_TO: c_int = 200;
pub const L2CAP_DEFAULT_MAX_SDU_SIZE: c_uint = 0xFFFF;
pub const L2CAP_DEFAULT_SDU_ITIME: c_uint = 0xFFFFFFFF;
pub const L2CAP_DEFAULT_ACC_LAT: c_uint = 0xFFFFFFFF;

pub const L2CAP_LE_MIN_MTU: c_int = 23;
pub const L2CAP_ECRED_CONN_SCID_MAX: c_int = 5;

// L2CAP socket address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_l2 {
    pub l2_family: sa_family_t,
    pub l2_psm: __le16,
    pub l2_bdaddr: bdaddr_t,
    pub l2_cid: __le16,
    pub l2_bdaddr_type: __u8,
}

// L2CAP socket options
pub const L2CAP_OPTIONS: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_options {
    pub omtu: __u16,
    pub imtu: __u16,
    pub flush_to: __u16,
    pub mode: __u8,
    pub fcs: __u8,
    pub max_tx: __u8,
    pub txwin_size: __u16,
}

pub const L2CAP_CONNINFO: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conninfo {
    pub hci_handle: __u16,
    pub dev_class: [__u8; 3],
}

pub const L2CAP_LM: c_uint = 0x03;
pub const L2CAP_LM_MASTER: c_uint = 0x0001;
pub const L2CAP_LM_AUTH: c_uint = 0x0002;
pub const L2CAP_LM_ENCRYPT: c_uint = 0x0004;
pub const L2CAP_LM_TRUSTED: c_uint = 0x0008;
pub const L2CAP_LM_RELIABLE: c_uint = 0x0010;
pub const L2CAP_LM_SECURE: c_uint = 0x0020;
pub const L2CAP_LM_FIPS: c_uint = 0x0040;
// L2CAP command codes
pub const L2CAP_COMMAND_REJ: c_uint = 0x01;
pub const L2CAP_CONN_REQ: c_uint = 0x02;
pub const L2CAP_CONN_RSP: c_uint = 0x03;
pub const L2CAP_CONF_REQ: c_uint = 0x04;
pub const L2CAP_CONF_RSP: c_uint = 0x05;
pub const L2CAP_DISCONN_REQ: c_uint = 0x06;
pub const L2CAP_DISCONN_RSP: c_uint = 0x07;
pub const L2CAP_ECHO_REQ: c_uint = 0x08;
pub const L2CAP_ECHO_RSP: c_uint = 0x09;
pub const L2CAP_INFO_REQ: c_uint = 0x0a;
pub const L2CAP_INFO_RSP: c_uint = 0x0b;
pub const L2CAP_CONN_PARAM_UPDATE_REQ: c_uint = 0x12;
pub const L2CAP_CONN_PARAM_UPDATE_RSP: c_uint = 0x13;
pub const L2CAP_LE_CONN_REQ: c_uint = 0x14;
pub const L2CAP_LE_CONN_RSP: c_uint = 0x15;
pub const L2CAP_LE_CREDITS: c_uint = 0x16;
pub const L2CAP_ECRED_CONN_REQ: c_uint = 0x17;
pub const L2CAP_ECRED_CONN_RSP: c_uint = 0x18;
pub const L2CAP_ECRED_RECONF_REQ: c_uint = 0x19;
pub const L2CAP_ECRED_RECONF_RSP: c_uint = 0x1a;
// L2CAP extended feature mask
pub const L2CAP_FEAT_FLOWCTL: c_uint = 0x00000001;
pub const L2CAP_FEAT_RETRANS: c_uint = 0x00000002;
pub const L2CAP_FEAT_BIDIR_QOS: c_uint = 0x00000004;
pub const L2CAP_FEAT_ERTM: c_uint = 0x00000008;
pub const L2CAP_FEAT_STREAMING: c_uint = 0x00000010;
pub const L2CAP_FEAT_FCS: c_uint = 0x00000020;
pub const L2CAP_FEAT_EXT_FLOW: c_uint = 0x00000040;
pub const L2CAP_FEAT_FIXED_CHAN: c_uint = 0x00000080;
pub const L2CAP_FEAT_EXT_WINDOW: c_uint = 0x00000100;
pub const L2CAP_FEAT_UCD: c_uint = 0x00000200;
// L2CAP checksum option
pub const L2CAP_FCS_NONE: c_uint = 0x00;
pub const L2CAP_FCS_CRC16: c_uint = 0x01;
// L2CAP fixed channels
pub const L2CAP_FC_SIG_BREDR: c_uint = 0x02;
pub const L2CAP_FC_CONNLESS: c_uint = 0x04;
pub const L2CAP_FC_ATT: c_uint = 0x10;
pub const L2CAP_FC_SIG_LE: c_uint = 0x20;
pub const L2CAP_FC_SMP_LE: c_uint = 0x40;
pub const L2CAP_FC_SMP_BREDR: c_uint = 0x80;
// L2CAP Control Field bit masks
pub const L2CAP_CTRL_SAR: c_uint = 0xC000;
pub const L2CAP_CTRL_REQSEQ: c_uint = 0x3F00;
pub const L2CAP_CTRL_TXSEQ: c_uint = 0x007E;
pub const L2CAP_CTRL_SUPERVISE: c_uint = 0x000C;
pub const L2CAP_CTRL_RETRANS: c_uint = 0x0080;
pub const L2CAP_CTRL_FINAL: c_uint = 0x0080;
pub const L2CAP_CTRL_POLL: c_uint = 0x0010;
pub const L2CAP_CTRL_FRAME_TYPE: c_uint = 0x0001 /* I- or S-Frame */;
pub const L2CAP_CTRL_TXSEQ_SHIFT: c_int = 1;
pub const L2CAP_CTRL_SUPER_SHIFT: c_int = 2;
pub const L2CAP_CTRL_POLL_SHIFT: c_int = 4;
pub const L2CAP_CTRL_FINAL_SHIFT: c_int = 7;
pub const L2CAP_CTRL_REQSEQ_SHIFT: c_int = 8;
pub const L2CAP_CTRL_SAR_SHIFT: c_int = 14;
// L2CAP Extended Control Field bit mask
pub const L2CAP_EXT_CTRL_TXSEQ: c_uint = 0xFFFC0000;
pub const L2CAP_EXT_CTRL_SAR: c_uint = 0x00030000;
pub const L2CAP_EXT_CTRL_SUPERVISE: c_uint = 0x00030000;
pub const L2CAP_EXT_CTRL_REQSEQ: c_uint = 0x0000FFFC;
pub const L2CAP_EXT_CTRL_POLL: c_uint = 0x00040000;
pub const L2CAP_EXT_CTRL_FINAL: c_uint = 0x00000002;
pub const L2CAP_EXT_CTRL_FRAME_TYPE: c_uint = 0x00000001 /* I- or S-Frame */;
pub const L2CAP_EXT_CTRL_FINAL_SHIFT: c_int = 1;
pub const L2CAP_EXT_CTRL_REQSEQ_SHIFT: c_int = 2;
pub const L2CAP_EXT_CTRL_SAR_SHIFT: c_int = 16;
pub const L2CAP_EXT_CTRL_SUPER_SHIFT: c_int = 16;
pub const L2CAP_EXT_CTRL_POLL_SHIFT: c_int = 18;
pub const L2CAP_EXT_CTRL_TXSEQ_SHIFT: c_int = 18;
// L2CAP Supervisory Function
pub const L2CAP_SUPER_RR: c_uint = 0x00;
pub const L2CAP_SUPER_REJ: c_uint = 0x01;
pub const L2CAP_SUPER_RNR: c_uint = 0x02;
pub const L2CAP_SUPER_SREJ: c_uint = 0x03;
// L2CAP Segmentation and Reassembly
pub const L2CAP_SAR_UNSEGMENTED: c_uint = 0x00;
pub const L2CAP_SAR_START: c_uint = 0x01;
pub const L2CAP_SAR_END: c_uint = 0x02;
pub const L2CAP_SAR_CONTINUE: c_uint = 0x03;
// L2CAP Command rej. reasons
pub const L2CAP_REJ_NOT_UNDERSTOOD: c_uint = 0x0000;
pub const L2CAP_REJ_MTU_EXCEEDED: c_uint = 0x0001;
pub const L2CAP_REJ_INVALID_CID: c_uint = 0x0002;
// L2CAP structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_hdr {
    pub len: __le16,
    pub cid: __le16,
    pub __packed: },
pub const L2CAP_LEN_SIZE: c_int = 2;
pub const L2CAP_HDR_SIZE: c_int = 4;
pub const L2CAP_ENH_HDR_SIZE: c_int = 6;
pub const L2CAP_EXT_HDR_SIZE: c_int = 8;
pub const L2CAP_FCS_SIZE: c_int = 2;
pub const L2CAP_SDULEN_SIZE: c_int = 2;
pub const L2CAP_PSMLEN_SIZE: c_int = 2;
pub const L2CAP_ENH_CTRL_SIZE: c_int = 2;
pub const L2CAP_EXT_CTRL_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_cmd_hdr {
    pub code: __u8,
    pub ident: __u8,
    pub len: __le16,
    pub __packed: },
pub const L2CAP_CMD_HDR_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_cmd_rej_unk {
    pub reason: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_cmd_rej_mtu {
    pub reason: __le16,
    pub max_mtu: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_cmd_rej_cid {
    pub reason: __le16,
    pub scid: __le16,
    pub dcid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conn_req {
    pub psm: __le16,
    pub scid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conn_rsp {
    pub dcid: __le16,
    pub scid: __le16,
    pub result: __le16,
    pub status: __le16,
    pub __packed: },
// protocol/service multiplexer (PSM)
pub const L2CAP_PSM_SDP: c_uint = 0x0001;
pub const L2CAP_PSM_RFCOMM: c_uint = 0x0003;
pub const L2CAP_PSM_3DSP: c_uint = 0x0021;
pub const L2CAP_PSM_IPSP: c_uint = 0x0023 /* 6LoWPAN */;
pub const L2CAP_PSM_DYN_START: c_uint = 0x1001;
pub const L2CAP_PSM_DYN_END: c_uint = 0xffff;
pub const L2CAP_PSM_AUTO_END: c_uint = 0x10ff;
pub const L2CAP_PSM_LE_DYN_START: c_uint = 0x0080;
pub const L2CAP_PSM_LE_DYN_END: c_uint = 0x00ff;
// channel identifier
pub const L2CAP_CID_SIGNALING: c_uint = 0x0001;
pub const L2CAP_CID_CONN_LESS: c_uint = 0x0002;
pub const L2CAP_CID_ATT: c_uint = 0x0004;
pub const L2CAP_CID_LE_SIGNALING: c_uint = 0x0005;
pub const L2CAP_CID_SMP: c_uint = 0x0006;
pub const L2CAP_CID_SMP_BREDR: c_uint = 0x0007;
pub const L2CAP_CID_DYN_START: c_uint = 0x0040;
pub const L2CAP_CID_DYN_END: c_uint = 0xffff;
pub const L2CAP_CID_LE_DYN_END: c_uint = 0x007f;
// connect/create channel results
pub const L2CAP_CR_SUCCESS: c_uint = 0x0000;
pub const L2CAP_CR_PEND: c_uint = 0x0001;
pub const L2CAP_CR_BAD_PSM: c_uint = 0x0002;
pub const L2CAP_CR_SEC_BLOCK: c_uint = 0x0003;
pub const L2CAP_CR_NO_MEM: c_uint = 0x0004;
pub const L2CAP_CR_INVALID_SCID: c_uint = 0x0006;
pub const L2CAP_CR_SCID_IN_USE: c_uint = 0x0007;
// credit based connect results
pub const L2CAP_CR_LE_SUCCESS: c_uint = 0x0000;
pub const L2CAP_CR_LE_BAD_PSM: c_uint = 0x0002;
pub const L2CAP_CR_LE_NO_MEM: c_uint = 0x0004;
pub const L2CAP_CR_LE_AUTHENTICATION: c_uint = 0x0005;
pub const L2CAP_CR_LE_AUTHORIZATION: c_uint = 0x0006;
pub const L2CAP_CR_LE_BAD_KEY_SIZE: c_uint = 0x0007;
pub const L2CAP_CR_LE_ENCRYPTION: c_uint = 0x0008;
pub const L2CAP_CR_LE_INVALID_SCID: c_uint = 0x0009;
pub const L2CAP_CR_LE_SCID_IN_USE: c_uint = 0x000A;
pub const L2CAP_CR_LE_UNACCEPT_PARAMS: c_uint = 0x000B;
pub const L2CAP_CR_LE_INVALID_PARAMS: c_uint = 0x000C;
// connect/create channel status
pub const L2CAP_CS_NO_INFO: c_uint = 0x0000;
pub const L2CAP_CS_AUTHEN_PEND: c_uint = 0x0001;
pub const L2CAP_CS_AUTHOR_PEND: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conf_req {
    pub dcid: __le16,
    pub flags: __le16,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conf_rsp {
    pub scid: __le16,
    pub flags: __le16,
    pub result: __le16,
    pub data: [__u8; ],
    pub __packed: },
pub const L2CAP_CONF_SUCCESS: c_uint = 0x0000;
pub const L2CAP_CONF_UNACCEPT: c_uint = 0x0001;
pub const L2CAP_CONF_REJECT: c_uint = 0x0002;
pub const L2CAP_CONF_UNKNOWN: c_uint = 0x0003;
pub const L2CAP_CONF_PENDING: c_uint = 0x0004;
pub const L2CAP_CONF_EFS_REJECT: c_uint = 0x0005;
// configuration req/rsp continuation flag
pub const L2CAP_CONF_FLAG_CONTINUATION: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conf_opt {
    pub type: __u8,
    pub len: __u8,
    pub val: [__u8; ],
    pub __packed: },
pub const L2CAP_CONF_OPT_SIZE: c_int = 2;
pub const L2CAP_CONF_HINT: c_uint = 0x80;
pub const L2CAP_CONF_MASK: c_uint = 0x7f;
pub const L2CAP_CONF_MTU: c_uint = 0x01;
pub const L2CAP_CONF_FLUSH_TO: c_uint = 0x02;
pub const L2CAP_CONF_QOS: c_uint = 0x03;
pub const L2CAP_CONF_RFC: c_uint = 0x04;
pub const L2CAP_CONF_FCS: c_uint = 0x05;
pub const L2CAP_CONF_EFS: c_uint = 0x06;
pub const L2CAP_CONF_EWS: c_uint = 0x07;
pub const L2CAP_CONF_MAX_SIZE: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conf_rfc {
    pub mode: __u8,
    pub txwin_size: __u8,
    pub max_transmit: __u8,
    pub retrans_timeout: __le16,
    pub monitor_timeout: __le16,
    pub max_pdu_size: __le16,
    pub __packed: },
pub const L2CAP_MODE_BASIC: c_uint = 0x00;
pub const L2CAP_MODE_RETRANS: c_uint = 0x01;
pub const L2CAP_MODE_FLOWCTL: c_uint = 0x02;
pub const L2CAP_MODE_ERTM: c_uint = 0x03;
pub const L2CAP_MODE_STREAMING: c_uint = 0x04;
// Unlike the above this one doesn't actually map to anything that would
// ever be sent over the air. Therefore, use a value that's unlikely to
// ever be used in the BR/EDR configuration phase.
//
pub const L2CAP_MODE_LE_FLOWCTL: c_uint = 0x80;
pub const L2CAP_MODE_EXT_FLOWCTL: c_uint = 0x81;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conf_efs {
    pub id: __u8,
    pub stype: __u8,
    pub msdu: __le16,
    pub sdu_itime: __le32,
    pub acc_lat: __le32,
    pub flush_to: __le32,
    pub __packed: },
pub const L2CAP_SERV_NOTRAFIC: c_uint = 0x00;
pub const L2CAP_SERV_BESTEFFORT: c_uint = 0x01;
pub const L2CAP_SERV_GUARANTEED: c_uint = 0x02;
pub const L2CAP_BESTEFFORT_ID: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_disconn_req {
    pub dcid: __le16,
    pub scid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_disconn_rsp {
    pub dcid: __le16,
    pub scid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_info_req {
    pub type: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_info_rsp {
    pub type: __le16,
    pub result: __le16,
    pub data: [__u8; ],
    pub __packed: },
pub const L2CAP_MR_SUCCESS: c_uint = 0x0000;
pub const L2CAP_MR_PEND: c_uint = 0x0001;
pub const L2CAP_MR_BAD_ID: c_uint = 0x0002;
pub const L2CAP_MR_SAME_ID: c_uint = 0x0003;
pub const L2CAP_MR_NOT_SUPP: c_uint = 0x0004;
pub const L2CAP_MR_COLLISION: c_uint = 0x0005;
pub const L2CAP_MR_NOT_ALLOWED: c_uint = 0x0006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_move_chan_cfm {
    pub icid: __le16,
    pub result: __le16,
    pub __packed: },
pub const L2CAP_MC_CONFIRMED: c_uint = 0x0000;
pub const L2CAP_MC_UNCONFIRMED: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_move_chan_cfm_rsp {
    pub icid: __le16,
    pub __packed: },
// info type
pub const L2CAP_IT_CL_MTU: c_uint = 0x0001;
pub const L2CAP_IT_FEAT_MASK: c_uint = 0x0002;
pub const L2CAP_IT_FIXED_CHAN: c_uint = 0x0003;
// info result
pub const L2CAP_IR_SUCCESS: c_uint = 0x0000;
pub const L2CAP_IR_NOTSUPP: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conn_param_update_req {
    pub min: __le16,
    pub max: __le16,
    pub latency: __le16,
    pub to_multiplier: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conn_param_update_rsp {
    pub result: __le16,
    pub __packed: },
// Connection Parameters result
pub const L2CAP_CONN_PARAM_ACCEPTED: c_uint = 0x0000;
pub const L2CAP_CONN_PARAM_REJECTED: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_le_conn_req {
    pub psm: __le16,
    pub scid: __le16,
    pub mtu: __le16,
    pub mps: __le16,
    pub credits: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_le_conn_rsp {
    pub dcid: __le16,
    pub mtu: __le16,
    pub mps: __le16,
    pub credits: __le16,
    pub result: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_le_credits {
    pub cid: __le16,
    pub credits: __le16,
    pub __packed: },
pub const L2CAP_ECRED_MIN_MTU: c_int = 64;
pub const L2CAP_ECRED_MIN_MPS: c_int = 64;
pub const L2CAP_ECRED_MAX_CID: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_ecred_conn_req {
// New members must be added within the struct_group() macro below.
    pub psm: __le16,
    pub mtu: __le16,
    pub mps: __le16,
    pub credits: __le16,
    pub scid: [__le16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_ecred_conn_rsp {
// New members must be added within the struct_group() macro below.
    pub mtu: __le16,
    pub mps: __le16,
    pub credits: __le16,
    pub result: __le16,
    pub dcid: [__le16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_ecred_reconf_req {
    pub mtu: __le16,
    pub mps: __le16,
    pub scid: [__le16; ],
    pub __packed: },
pub const L2CAP_RECONF_SUCCESS: c_uint = 0x0000;
pub const L2CAP_RECONF_INVALID_MTU: c_uint = 0x0001;
pub const L2CAP_RECONF_INVALID_MPS: c_uint = 0x0002;
pub const L2CAP_RECONF_INVALID_CID: c_uint = 0x0003;
pub const L2CAP_RECONF_INVALID_PARAMS: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_ecred_reconf_rsp {
    pub result: __le16,
    pub __packed: },
// ----- L2CAP channels and connections -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_seq_list {
    pub head: __u16,
    pub tail: __u16,
    pub mask: __u16,
    pub list: *mut __u16,
}

pub const L2CAP_SEQ_LIST_CLEAR: c_uint = 0xFFFF;
pub const L2CAP_SEQ_LIST_TAIL: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_chan {
    pub conn: *mut l2cap_conn,
    pub kref: kref,
    pub nesting: core::sync::atomic::AtomicI32,
    pub state: __u8,
    pub dst: bdaddr_t,
    pub dst_type: __u8,
    pub src: bdaddr_t,
    pub src_type: __u8,
    pub psm: __le16,
    pub sport: __le16,
    pub dcid: __u16,
    pub scid: __u16,
    pub imtu: __u16,
    pub omtu: __u16,
    pub flush_to: __u16,
    pub mode: __u8,
    pub chan_type: __u8,
    pub chan_policy: __u8,
    pub sec_level: __u8,
    pub ident: __u8,
    pub conf_req: [__u8; 64],
    pub conf_len: __u8,
    pub num_conf_req: __u8,
    pub num_conf_rsp: __u8,
    pub fcs: __u8,
    pub tx_win: __u16,
    pub tx_win_max: __u16,
    pub ack_win: __u16,
    pub max_tx: __u8,
    pub retrans_timeout: __u16,
    pub monitor_timeout: __u16,
    pub mps: __u16,
    pub tx_credits: __u16,
    pub rx_credits: __u16,
// estimated available receive buffer space or -1 if unknown
    pub rx_avail: isize,
    pub tx_state: __u8,
    pub rx_state: __u8,
    pub conf_state: c_ulong,
    pub conn_state: c_ulong,
    pub flags: c_ulong,
    pub next_tx_seq: __u16,
    pub expected_ack_seq: __u16,
    pub expected_tx_seq: __u16,
    pub buffer_seq: __u16,
    pub srej_save_reqseq: __u16,
    pub last_acked_seq: __u16,
    pub frames_sent: __u16,
    pub unacked_frames: __u16,
    pub retry_count: __u8,
    pub sdu_len: __u16,
    pub sdu: *mut sk_buff,
    pub sdu_last_frag: *mut sk_buff,
    pub remote_tx_win: __u16,
    pub remote_max_tx: __u8,
    pub remote_mps: __u16,
    pub local_id: __u8,
    pub local_stype: __u8,
    pub local_msdu: __u16,
    pub local_sdu_itime: __u32,
    pub local_acc_lat: __u32,
    pub local_flush_to: __u32,
    pub remote_id: __u8,
    pub remote_stype: __u8,
    pub remote_msdu: __u16,
    pub remote_sdu_itime: __u32,
    pub remote_acc_lat: __u32,
    pub remote_flush_to: __u32,
    pub chan_timer: delayed_work,
    pub retrans_timer: delayed_work,
    pub monitor_timer: delayed_work,
    pub ack_timer: delayed_work,
    pub tx_send_head: *mut sk_buff,
    pub tx_q: sk_buff_head,
    pub srej_q: sk_buff_head,
    pub srej_list: l2cap_seq_list,
    pub retrans_list: l2cap_seq_list,
    pub list: list_head,
    pub global_l: list_head,
    pub data: *mut c_void,
    pub ops: *const l2cap_ops,
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_ops {
    pub name: *mut c_char,
    pub new_chan): *mut l2cap_chan,
    pub skb): *mut sk_buff,
    pub err): *mut *mut *mut void (teardown) (struct l2cap_chan chan, int,
    pub chan): *mut *mut void (close) (struct l2cap_chan,
    pub err): int state, int,
    pub chan): *mut *mut void (ready) (struct l2cap_chan,
    pub chan): *mut *mut void (defer) (struct l2cap_chan,
    pub chan): *mut *mut void (resume) (struct l2cap_chan,
    pub chan): *mut *mut void (suspend) (struct l2cap_chan,
    pub chan): *mut *mut void (set_shutdown) (struct l2cap_chan,
    pub chan): *mut *mut long (get_sndtimeo) (struct l2cap_chan,
    pub chan): *mut *mut *mut pid (get_peer_pid) (l2cap_chan,
    pub nb): unsigned long len, int,
    pub skb): *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_conn {
    pub hcon: *mut hci_conn,
    pub hchan: *mut hci_chan,
    pub mtu: c_uint,
    pub feat_mask: __u32,
    pub remote_fixed_chan: __u8,
    pub local_fixed_chan: __u8,
    pub info_state: __u8,
    pub info_ident: __u8,
    pub info_timer: delayed_work,
    pub rx_skb: *mut sk_buff,
    pub rx_len: __u32,
    pub tx_ida: ida,
    pub tx_ident: __u8,
    pub pending_rx: sk_buff_head,
    pub pending_rx_work: work_struct,
    pub id_addr_timer: delayed_work,
    pub disc_reason: __u8,
    pub smp: *mut l2cap_chan,
    pub chan_l: list_head,
    pub lock: mutex,
    pub ref: kref,
    pub users: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_user {
    pub list: list_head,
    pub user): *mut *mut *mut int (probe) (struct l2cap_conn conn, struct l2cap_user,
    pub user): *mut *mut *mut void (remove) (struct l2cap_conn conn, struct l2cap_user,
}

pub const L2CAP_INFO_CL_MTU_REQ_SENT: c_uint = 0x01;
pub const L2CAP_INFO_FEAT_MASK_REQ_SENT: c_uint = 0x04;
pub const L2CAP_INFO_FEAT_MASK_REQ_DONE: c_uint = 0x08;
pub const L2CAP_CHAN_RAW: c_int = 1;
pub const L2CAP_CHAN_CONN_LESS: c_int = 2;
pub const L2CAP_CHAN_CONN_ORIENTED: c_int = 3;
pub const L2CAP_CHAN_FIXED: c_int = 4;
// ----- L2CAP socket info -----

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_rx_busy {
    pub list: list_head,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_pinfo {
    pub bt: bt_sock,
// With owning sk_socket chan may be read without lock, other access
// should hold lock_sock.
//
    pub chan: *mut l2cap_chan,
    pub rx_busy: list_head,
}

pub const L2CAP_CONF_MAX_CONF_REQ: c_int = 2;
pub const L2CAP_CONF_MAX_CONF_RSP: c_int = 2;
// Definitions for flags in l2cap_chan
// Lock nesting levels for L2CAP channels. We need these because lockdep
// otherwise considers all channels equal and will e.g. complain about a
// connection oriented channel triggering SMP procedures or a listening
// channel creating and locking a child channel.
//
extern "C" {
    pub fn l2cap_chan_hold(c: *mut l2cap_chan);
}
extern "C" {
    pub fn l2cap_chan_put(c: *mut l2cap_chan);
}
// If delayed work cancelled do not hold(chan)
// put(chan) if delayed work cancelled otherwise it

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn l2cap_init_sockets() -> c_int;
}
extern "C" {
    pub fn l2cap_cleanup_sockets();
}
extern "C" {
    pub fn l2cap_is_socket(sock: *mut socket) -> bool;
}
extern "C" {
    pub fn __l2cap_le_connect_rsp_defer(chan: *mut l2cap_chan);
}
extern "C" {
    pub fn __l2cap_ecred_conn_rsp_defer(chan: *mut l2cap_chan);
}
extern "C" {
    pub fn __l2cap_connect_rsp_defer(chan: *mut l2cap_chan);
}
extern "C" {
    pub fn l2cap_add_psm(chan: *mut l2cap_chan, src: *mut bdaddr_t, psm: __le16) -> c_int;
}
extern "C" {
    pub fn l2cap_add_scid(chan: *mut l2cap_chan, scid: __u16) -> c_int;
}
extern "C" {
    pub fn l2cap_chan_close(chan: *mut l2cap_chan, reason: c_int);
}
extern "C" {
    pub fn l2cap_chan_reconfigure(chan: *mut l2cap_chan, mtu: __u16) -> c_int;
}
extern "C" {
    pub fn l2cap_chan_busy(chan: *mut l2cap_chan, busy: c_int);
}
extern "C" {
    pub fn l2cap_chan_rx_avail(chan: *mut l2cap_chan, rx_avail: isize);
}
extern "C" {
    pub fn l2cap_chan_check_security(chan: *mut l2cap_chan, initiator: bool) -> c_int;
}
extern "C" {
    pub fn l2cap_chan_set_defaults(chan: *mut l2cap_chan, pchan: *mut l2cap_chan);
}
extern "C" {
    pub fn l2cap_ertm_init(chan: *mut l2cap_chan) -> c_int;
}
extern "C" {
    pub fn l2cap_chan_add(conn: *mut l2cap_conn, chan: *mut l2cap_chan);
}
extern "C" {
    pub fn __l2cap_chan_add(conn: *mut l2cap_conn, chan: *mut l2cap_chan);
}
extern "C" {
    pub fn void(chan: *mut *mut l2cap_chan_func_t)(struct l2cap_chan, data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn l2cap_chan_del(chan: *mut l2cap_chan, err: c_int);
}
extern "C" {
    pub fn l2cap_send_conn_req(chan: *mut l2cap_chan);
}
extern "C" {
    pub fn l2cap_conn_put(conn: *mut l2cap_conn);
}
extern "C" {
    pub fn l2cap_register_user(conn: *mut l2cap_conn, user: *mut l2cap_user) -> c_int;
}
extern "C" {
    pub fn l2cap_unregister_user(conn: *mut l2cap_conn, user: *mut l2cap_user);
}
