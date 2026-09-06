//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_pptp.h
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
// PPTP constants and structs

// state of the control session
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pptp_ctrlsess_state {
    PPTP_SESSION_NONE,			/* no session present */
    PPTP_SESSION_ERROR,			/* some session error */
    PPTP_SESSION_STOPREQ,			/* stop_sess request seen */
    PPTP_SESSION_REQUESTED,			/* start_sess request seen */
    PPTP_SESSION_CONFIRMED,			/* session established */
}

// state of the call inside the control session
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pptp_ctrlcall_state {
    PPTP_CALL_NONE,
    PPTP_CALL_ERROR,
    PPTP_CALL_OUT_REQ,
    PPTP_CALL_OUT_CONF,
    PPTP_CALL_IN_REQ,
    PPTP_CALL_IN_REP,
    PPTP_CALL_IN_CONF,
    PPTP_CALL_CLEAR_REQ,
}

// conntrack private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_pptp_master {
    pub /: *mut *mut pptp_ctrlsess_state sstate; / session state,
    pub /: *mut *mut pptp_ctrlcall_state cstate; / call state,
    pub /: *mut *mut __be16 pac_call_id; / call id of PAC,
    pub /: *mut *mut __be16 pns_call_id; / call id of PNS,
// in pre-2.6.11 this used to be per-expect. Now it is per-conntrack
// and therefore imposes a fixed limit on the number of maps
    pub keymap: [*mut nf_ct_gre_keymap; IP_CT_DIR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_pptp {
    pub /: *mut *mut __be16 pns_call_id; / NAT'ed PNS call id,
    pub /: *mut *mut __be16 pac_call_id; / NAT'ed PAC call id,
}

pub const PPTP_PACKET_CONTROL: c_int = 1;
pub const PPTP_PACKET_MGMT: c_int = 2;
pub const PPTP_MAGIC_COOKIE: c_uint = 0x1a2b3c4d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pptp_pkt_hdr {
    pub packetLength: __u16,
    pub packetType: __be16,
    pub magicCookie: __be32,
}

// PptpControlMessageType values
pub const PPTP_START_SESSION_REQUEST: c_int = 1;
pub const PPTP_START_SESSION_REPLY: c_int = 2;
pub const PPTP_STOP_SESSION_REQUEST: c_int = 3;
pub const PPTP_STOP_SESSION_REPLY: c_int = 4;
pub const PPTP_ECHO_REQUEST: c_int = 5;
pub const PPTP_ECHO_REPLY: c_int = 6;
pub const PPTP_OUT_CALL_REQUEST: c_int = 7;
pub const PPTP_OUT_CALL_REPLY: c_int = 8;
pub const PPTP_IN_CALL_REQUEST: c_int = 9;
pub const PPTP_IN_CALL_REPLY: c_int = 10;
pub const PPTP_IN_CALL_CONNECT: c_int = 11;
pub const PPTP_CALL_CLEAR_REQUEST: c_int = 12;
pub const PPTP_CALL_DISCONNECT_NOTIFY: c_int = 13;
pub const PPTP_WAN_ERROR_NOTIFY: c_int = 14;
pub const PPTP_SET_LINK_INFO: c_int = 15;
pub const PPTP_MSG_MAX: c_int = 15;
// PptpGeneralError values
pub const PPTP_ERROR_CODE_NONE: c_int = 0;
pub const PPTP_NOT_CONNECTED: c_int = 1;
pub const PPTP_BAD_FORMAT: c_int = 2;
pub const PPTP_BAD_VALUE: c_int = 3;
pub const PPTP_NO_RESOURCE: c_int = 4;
pub const PPTP_BAD_CALLID: c_int = 5;
pub const PPTP_REMOVE_DEVICE_ERROR: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpControlHeader {
    pub messageType: __be16,
    pub reserved: __u16,
}

// FramingCapability Bitmap Values
pub const PPTP_FRAME_CAP_ASYNC: c_uint = 0x1;
pub const PPTP_FRAME_CAP_SYNC: c_uint = 0x2;
// BearerCapability Bitmap Values
pub const PPTP_BEARER_CAP_ANALOG: c_uint = 0x1;
pub const PPTP_BEARER_CAP_DIGITAL: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpStartSessionRequest {
    pub protocolVersion: __be16,
    pub reserved1: __u16,
    pub framingCapability: __be32,
    pub bearerCapability: __be32,
    pub maxChannels: __be16,
    pub firmwareRevision: __be16,
    pub hostName: [__u8; 64],
    pub vendorString: [__u8; 64],
}

// PptpStartSessionResultCode Values
pub const PPTP_START_OK: c_int = 1;
pub const PPTP_START_GENERAL_ERROR: c_int = 2;
pub const PPTP_START_ALREADY_CONNECTED: c_int = 3;
pub const PPTP_START_NOT_AUTHORIZED: c_int = 4;
pub const PPTP_START_UNKNOWN_PROTOCOL: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpStartSessionReply {
    pub protocolVersion: __be16,
    pub resultCode: __u8,
    pub generalErrorCode: __u8,
    pub framingCapability: __be32,
    pub bearerCapability: __be32,
    pub maxChannels: __be16,
    pub firmwareRevision: __be16,
    pub hostName: [__u8; 64],
    pub vendorString: [__u8; 64],
}

// PptpStopReasons
pub const PPTP_STOP_NONE: c_int = 1;
pub const PPTP_STOP_PROTOCOL: c_int = 2;
pub const PPTP_STOP_LOCAL_SHUTDOWN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpStopSessionRequest {
    pub reason: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
}

// PptpStopSessionResultCode
pub const PPTP_STOP_OK: c_int = 1;
pub const PPTP_STOP_GENERAL_ERROR: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpStopSessionReply {
    pub resultCode: __u8,
    pub generalErrorCode: __u8,
    pub reserved1: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpEchoRequest {
    pub identNumber: __be32,
}

// PptpEchoReplyResultCode
pub const PPTP_ECHO_OK: c_int = 1;
pub const PPTP_ECHO_GENERAL_ERROR: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpEchoReply {
    pub identNumber: __be32,
    pub resultCode: __u8,
    pub generalErrorCode: __u8,
    pub reserved: __u16,
}

// PptpFramingType
pub const PPTP_ASYNC_FRAMING: c_int = 1;
pub const PPTP_SYNC_FRAMING: c_int = 2;
pub const PPTP_DONT_CARE_FRAMING: c_int = 3;
// PptpCallBearerType
pub const PPTP_ANALOG_TYPE: c_int = 1;
pub const PPTP_DIGITAL_TYPE: c_int = 2;
pub const PPTP_DONT_CARE_BEARER_TYPE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpOutCallRequest {
    pub callID: __be16,
    pub callSerialNumber: __be16,
    pub minBPS: __be32,
    pub maxBPS: __be32,
    pub bearerType: __be32,
    pub framingType: __be32,
    pub packetWindow: __be16,
    pub packetProcDelay: __be16,
    pub phoneNumberLength: __be16,
    pub reserved1: __u16,
    pub phoneNumber: [__u8; 64],
    pub subAddress: [__u8; 64],
}

// PptpCallResultCode
pub const PPTP_OUTCALL_CONNECT: c_int = 1;
pub const PPTP_OUTCALL_GENERAL_ERROR: c_int = 2;
pub const PPTP_OUTCALL_NO_CARRIER: c_int = 3;
pub const PPTP_OUTCALL_BUSY: c_int = 4;
pub const PPTP_OUTCALL_NO_DIAL_TONE: c_int = 5;
pub const PPTP_OUTCALL_TIMEOUT: c_int = 6;
pub const PPTP_OUTCALL_DONT_ACCEPT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpOutCallReply {
    pub callID: __be16,
    pub peersCallID: __be16,
    pub resultCode: __u8,
    pub generalErrorCode: __u8,
    pub causeCode: __be16,
    pub connectSpeed: __be32,
    pub packetWindow: __be16,
    pub packetProcDelay: __be16,
    pub physChannelID: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpInCallRequest {
    pub callID: __be16,
    pub callSerialNumber: __be16,
    pub callBearerType: __be32,
    pub physChannelID: __be32,
    pub dialedNumberLength: __be16,
    pub dialingNumberLength: __be16,
    pub dialedNumber: [__u8; 64],
    pub dialingNumber: [__u8; 64],
    pub subAddress: [__u8; 64],
}

// PptpInCallResultCode
pub const PPTP_INCALL_ACCEPT: c_int = 1;
pub const PPTP_INCALL_GENERAL_ERROR: c_int = 2;
pub const PPTP_INCALL_DONT_ACCEPT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpInCallReply {
    pub callID: __be16,
    pub peersCallID: __be16,
    pub resultCode: __u8,
    pub generalErrorCode: __u8,
    pub packetWindow: __be16,
    pub packetProcDelay: __be16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpInCallConnected {
    pub peersCallID: __be16,
    pub reserved: __u16,
    pub connectSpeed: __be32,
    pub packetWindow: __be16,
    pub packetProcDelay: __be16,
    pub callFramingType: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpClearCallRequest {
    pub callID: __be16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpCallDisconnectNotify {
    pub callID: __be16,
    pub resultCode: __u8,
    pub generalErrorCode: __u8,
    pub causeCode: __be16,
    pub reserved: __u16,
    pub callStatistics: [__u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpWanErrorNotify {
    pub peersCallID: __be16,
    pub reserved: __u16,
    pub crcErrors: __be32,
    pub framingErrors: __be32,
    pub hardwareOverRuns: __be32,
    pub bufferOverRuns: __be32,
    pub timeoutErrors: __be32,
    pub alignmentErrors: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpSetLinkInfo {
    pub peersCallID: __be16,
    pub reserved: __u16,
    pub sendAccm: __be32,
    pub recvAccm: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pptp_ctrl_union {
    pub sreq: PptpStartSessionRequest,
    pub srep: PptpStartSessionReply,
    pub streq: PptpStopSessionRequest,
    pub strep: PptpStopSessionReply,
    pub ocreq: PptpOutCallRequest,
    pub ocack: PptpOutCallReply,
    pub icreq: PptpInCallRequest,
    pub icack: PptpInCallReply,
    pub iccon: PptpInCallConnected,
    pub clrreq: PptpClearCallRequest,
    pub disc: PptpCallDisconnectNotify,
    pub wanerr: PptpWanErrorNotify,
    pub setlink: PptpSetLinkInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_pptp_hook {
    pub pptpReq): *mut pptp_ctrl_union,
    pub pptpReq): *mut pptp_ctrl_union,
    pub exp_reply): *mut nf_conntrack_expect,
    pub exp): *mut nf_conntrack_expect,
}
