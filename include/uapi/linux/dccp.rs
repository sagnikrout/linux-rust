//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dccp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// struct dccp_hdr - generic part of DCCP packet header
//
// @dccph_sport - Relevant port on the endpoint that sent this packet
// @dccph_dport - Relevant port on the other endpoint
// @dccph_doff - Data Offset from the start of the DCCP header, in 32-bit words
// @dccph_ccval - Used by the HC-Sender CCID
// @dccph_cscov - Parts of the packet that are covered by the Checksum field
// @dccph_checksum - Internet checksum, depends on dccph_cscov
// @dccph_x - 0 = 24 bit sequence number, 1 = 48
// @dccph_type - packet type, see DCCP_PKT_ prefixed macros
// @dccph_seq - sequence number high or low order 24 bits, depends on dccph_x
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr {
    pub dccph_doff: __u8,

    pub dccph_checksum: __sum16,

    pub dccph_seq2: __u8,
    pub dccph_seq: __be16,
}

//
// struct dccp_hdr_ext - the low bits of a 48 bit seq packet
//
// @dccph_seq_low - low 24 bits of a 48 bit seq packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr_ext {
    pub dccph_seq_low: __be32,
}

//
// struct dccp_hdr_request - Connection initiation request header
//
// @dccph_req_service - Service to which the client app wants to connect
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr_request {
    pub dccph_req_service: __be32,
}

//
// struct dccp_hdr_ack_bits - acknowledgment bits common to most packets
//
// @dccph_resp_ack_nr_high - 48 bit ack number high order bits, contains GSR
// @dccph_resp_ack_nr_low - 48 bit ack number low order bits, contains GSR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr_ack_bits {
    pub dccph_reserved1: __be16,
    pub dccph_ack_nr_high: __be16,
    pub dccph_ack_nr_low: __be32,
}

//
// struct dccp_hdr_response - Connection initiation response header
//
// @dccph_resp_ack - 48 bit Acknowledgment Number Subheader (5.3)
// @dccph_resp_service - Echoes the Service Code on a received DCCP-Request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr_response {
    pub dccph_resp_ack: dccp_hdr_ack_bits,
    pub dccph_resp_service: __be32,
}

//
// struct dccp_hdr_reset - Unconditionally shut down a connection
//
// @dccph_reset_ack - 48 bit Acknowledgment Number Subheader (5.6)
// @dccph_reset_code - one of %dccp_reset_codes
// @dccph_reset_data - the Data 1 ... Data 3 fields from 5.6
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr_reset {
    pub dccph_reset_ack: dccp_hdr_ack_bits,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dccp_pkt_type {
    DCCP_PKT_REQUEST = 0,
    DCCP_PKT_RESPONSE,
    DCCP_PKT_DATA,
    DCCP_PKT_ACK,
    DCCP_PKT_DATAACK,
    DCCP_PKT_CLOSEREQ,
    DCCP_PKT_CLOSE,
    DCCP_PKT_RESET,
    DCCP_PKT_SYNC,
    DCCP_PKT_SYNCACK,
    DCCP_PKT_INVALID,
}

extern "C" {
    pub fn sizeof(dccp_hdr_ack_bits: struct) -> return;
}
extern "C" {
    pub fn sizeof(dccp_hdr_request: struct) -> return;
}
extern "C" {
    pub fn sizeof(dccp_hdr_response: struct) -> return;
}
extern "C" {
    pub fn sizeof(dccp_hdr_reset: struct) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dccp_reset_codes {
    DCCP_RESET_CODE_UNSPECIFIED = 0,
    DCCP_RESET_CODE_CLOSED,
    DCCP_RESET_CODE_ABORTED,
    DCCP_RESET_CODE_NO_CONNECTION,
    DCCP_RESET_CODE_PACKET_ERROR,
    DCCP_RESET_CODE_OPTION_ERROR,
    DCCP_RESET_CODE_MANDATORY_ERROR,
    DCCP_RESET_CODE_CONNECTION_REFUSED,
    DCCP_RESET_CODE_BAD_SERVICE_CODE,
    DCCP_RESET_CODE_TOO_BUSY,
    DCCP_RESET_CODE_BAD_INIT_COOKIE,
    DCCP_RESET_CODE_AGGRESSION_PENALTY,

    DCCP_MAX_RESET_CODES		/* Leave at the end!  */
}

// DCCP options
// maximum size of a single TLV-encoded DCCP option (sans type/len bytes)
pub const DCCP_SINGLE_OPT_MAXLEN: c_int = 253;
// DCCP CCIDS
// DCCP features (RFC 4340 section 6.4)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dccp_feature_numbers {
    DCCPF_RESERVED = 0,
    DCCPF_CCID = 1,
    DCCPF_SHORT_SEQNOS = 2,
    DCCPF_SEQUENCE_WINDOW = 3,
    DCCPF_ECN_INCAPABLE = 4,
    DCCPF_ACK_RATIO = 5,
    DCCPF_SEND_ACK_VECTOR = 6,
    DCCPF_SEND_NDP_COUNT = 7,
    DCCPF_MIN_CSUM_COVER = 8,
    DCCPF_DATA_CHECKSUM = 9,
// 10-127 reserved
    DCCPF_MIN_CCID_SPECIFIC = 128,
    DCCPF_SEND_LEV_RATE = 192,	/* RFC 4342, sec. 8.4 */
    DCCPF_MAX_CCID_SPECIFIC = 255,
}

// DCCP socket control message types for cmsg
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dccp_cmsg_type {
    DCCP_SCM_PRIORITY = 1,
    DCCP_SCM_QPOLICY_MAX = 0xFFFF,
// ^-- Up to here reserved exclusively for qpolicy parameters
    DCCP_SCM_MAX
}

// DCCP priorities for outgoing/queued packets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dccp_packet_dequeueing_policy {
    DCCPQ_POLICY_SIMPLE,
    DCCPQ_POLICY_PRIO,
    DCCPQ_POLICY_MAX
}

// DCCP socket options

pub const DCCP_SOCKOPT_SERVICE: c_int = 2;
pub const DCCP_SOCKOPT_CHANGE_L: c_int = 3;
pub const DCCP_SOCKOPT_CHANGE_R: c_int = 4;
pub const DCCP_SOCKOPT_GET_CUR_MPS: c_int = 5;
pub const DCCP_SOCKOPT_SERVER_TIMEWAIT: c_int = 6;
pub const DCCP_SOCKOPT_SEND_CSCOV: c_int = 10;
pub const DCCP_SOCKOPT_RECV_CSCOV: c_int = 11;
pub const DCCP_SOCKOPT_AVAILABLE_CCIDS: c_int = 12;
pub const DCCP_SOCKOPT_CCID: c_int = 13;
pub const DCCP_SOCKOPT_TX_CCID: c_int = 14;
pub const DCCP_SOCKOPT_RX_CCID: c_int = 15;
pub const DCCP_SOCKOPT_QPOLICY_ID: c_int = 16;
pub const DCCP_SOCKOPT_QPOLICY_TXQLEN: c_int = 17;
pub const DCCP_SOCKOPT_CCID_RX_INFO: c_int = 128;
pub const DCCP_SOCKOPT_CCID_TX_INFO: c_int = 192;
// maximum number of services provided on the same listening port
pub const DCCP_SERVICE_LIST_MAX_LEN: c_int = 32;
