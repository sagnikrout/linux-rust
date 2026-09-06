//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sctp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// SCTP kernel reference Implementation
// (C) Copyright IBM Corp. 2001, 2004
// Copyright (c) 1999-2000 Cisco, Inc.
// Copyright (c) 1999-2001 Motorola, Inc.
// Copyright (c) 2001 Intel Corp.
// Copyright (c) 2001 Nokia, Inc.
// Copyright (c) 2001 La Monte H.P. Yarroll
//
// This file is part of the SCTP kernel reference Implementation
//
// Various protocol defined structures.
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Or submit a bug report through the following website:
// http://www.sf.net/projects/lksctp
//
// Written or modified by:
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson <karl@athena.chicago.il.us>
// Jon Grimm <jgrimm@us.ibm.com>
// Xingang Guo <xingang.guo@intel.com>
// randall@sctp.chicago.il.us
// kmorneau@cisco.com
// qxie1@email.mot.com
// Sridhar Samudrala <sri@us.ibm.com>
// Kevin Gao <kevin.gao@intel.com>
//
// Any bugs reported given to us we will try to fix... any fixes shared will
// be incorporated into the next SCTP release.
//

// Section 3.1.  SCTP Common Header Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctphdr {
    pub source: __be16,
    pub dest: __be16,
    pub vtag: __be32,
    pub checksum: __le32,
}

// Section 3.2.  Chunk Field Descriptions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_chunkhdr {
    pub type: __u8,
    pub flags: __u8,
    pub length: __be16,
}

// Section 3.2.  Chunk Type Values.
// [Chunk Type] identifies the type of information contained in the Chunk
// Value field. It takes a value from 0 to 254. The value of 255 is
// reserved for future use as an extension field.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_cid {
    SCTP_CID_DATA			= 0,
    SCTP_CID_INIT			= 1,
    SCTP_CID_INIT_ACK		= 2,
    SCTP_CID_SACK			= 3,
    SCTP_CID_HEARTBEAT		= 4,
    SCTP_CID_HEARTBEAT_ACK		= 5,
    SCTP_CID_ABORT			= 6,
    SCTP_CID_SHUTDOWN		= 7,
    SCTP_CID_SHUTDOWN_ACK		= 8,
    SCTP_CID_ERROR			= 9,
    SCTP_CID_COOKIE_ECHO		= 10,
    SCTP_CID_COOKIE_ACK	        = 11,
    SCTP_CID_ECN_ECNE		= 12,
    SCTP_CID_ECN_CWR		= 13,
    SCTP_CID_SHUTDOWN_COMPLETE	= 14,

// AUTH Extension Section 4.1
    SCTP_CID_AUTH			= 0x0F,

// sctp ndata 5.1. I-DATA
    SCTP_CID_I_DATA			= 0x40,

// PR-SCTP Sec 3.2
    SCTP_CID_FWD_TSN		= 0xC0,

// Use hex, as defined in ADDIP sec. 3.1
    SCTP_CID_ASCONF			= 0xC1,
    SCTP_CID_I_FWD_TSN		= 0xC2,
    SCTP_CID_ASCONF_ACK		= 0x80,
    SCTP_CID_RECONF			= 0x82,
    SCTP_CID_PAD			= 0x84,
}

// Section 3.2
// Chunk Types are encoded such that the highest-order two bits specify
// the action that must be taken if the processing endpoint does not
// recognize the Chunk Type.
//
// This flag is used in Chunk Flags for ABORT and SHUTDOWN COMPLETE.
//
// 3.3.7 Abort Association (ABORT) (6):
// The T bit is set to 0 if the sender had a TCB that it destroyed.
// If the sender did not have a TCB it should set this bit to 1.
//
// Set the T bit
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Type = 14   |Reserved     |T|      Length = 4               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Chunk Flags: 8 bits
//
// Reserved:  7 bits
// Set to 0 on transmit and ignored on receipt.
//
// T bit:  1 bit
// The T bit is set to 0 if the sender had a TCB that it destroyed. If
// the sender did NOT have a TCB it should set this bit to 1.
//
// Note: Special rules apply to this chunk for verification, please
// see Section 8.5.1 for details.
//

// RFC 2960
// Section 3.2.1 Optional/Variable-length Parmaeter Format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_paramhdr {
    pub type: __be16,
    pub length: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_param {

// RFC 2960 Section 3.3.5
    SCTP_PARAM_HEARTBEAT_INFO		= cpu_to_be16(1),
// RFC 2960 Section 3.3.2.1
    SCTP_PARAM_IPV4_ADDRESS			= cpu_to_be16(5),
    SCTP_PARAM_IPV6_ADDRESS			= cpu_to_be16(6),
    SCTP_PARAM_STATE_COOKIE			= cpu_to_be16(7),
    SCTP_PARAM_UNRECOGNIZED_PARAMETERS	= cpu_to_be16(8),
    SCTP_PARAM_COOKIE_PRESERVATIVE		= cpu_to_be16(9),
    SCTP_PARAM_HOST_NAME_ADDRESS		= cpu_to_be16(11),
    SCTP_PARAM_SUPPORTED_ADDRESS_TYPES	= cpu_to_be16(12),
    SCTP_PARAM_ECN_CAPABLE			= cpu_to_be16(0x8000),

// AUTH Extension Section 3
    SCTP_PARAM_RANDOM			= cpu_to_be16(0x8002),
    SCTP_PARAM_CHUNKS			= cpu_to_be16(0x8003),
    SCTP_PARAM_HMAC_ALGO			= cpu_to_be16(0x8004),

// Add-IP: Supported Extensions, Section 4.2
    SCTP_PARAM_SUPPORTED_EXT	= cpu_to_be16(0x8008),

// PR-SCTP Sec 3.1
    SCTP_PARAM_FWD_TSN_SUPPORT	= cpu_to_be16(0xc000),

// Add-IP Extension. Section 3.2
    SCTP_PARAM_ADD_IP		= cpu_to_be16(0xc001),
    SCTP_PARAM_DEL_IP		= cpu_to_be16(0xc002),
    SCTP_PARAM_ERR_CAUSE		= cpu_to_be16(0xc003),
    SCTP_PARAM_SET_PRIMARY		= cpu_to_be16(0xc004),
    SCTP_PARAM_SUCCESS_REPORT	= cpu_to_be16(0xc005),
    SCTP_PARAM_ADAPTATION_LAYER_IND = cpu_to_be16(0xc006),

// RE-CONFIG. Section 4
    SCTP_PARAM_RESET_OUT_REQUEST		= cpu_to_be16(0x000d),
    SCTP_PARAM_RESET_IN_REQUEST		= cpu_to_be16(0x000e),
    SCTP_PARAM_RESET_TSN_REQUEST		= cpu_to_be16(0x000f),
    SCTP_PARAM_RESET_RESPONSE		= cpu_to_be16(0x0010),
    SCTP_PARAM_RESET_ADD_OUT_STREAMS	= cpu_to_be16(0x0011),
    SCTP_PARAM_RESET_ADD_IN_STREAMS		= cpu_to_be16(0x0012),
}

// RFC 2960 Section 3.2.1
// The Parameter Types are encoded such that the highest-order two bits
// specify the action that must be taken if the processing endpoint does
// not recognize the Parameter Type.
//
// RFC 2960 Section 3.3.1 Payload Data (DATA) (0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_datahdr {
    pub tsn: __be32,
    pub stream: __be16,
    pub ssn: __be16,
    pub ppid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_data_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub data_hdr: sctp_datahdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_idatahdr {
    pub tsn: __be32,
    pub stream: __be16,
    pub reserved: __be16,
    pub mid: __be32,
    pub ppid: __u32,
    pub fsn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_idata_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub data_hdr: sctp_idatahdr,
}

// DATA Chuck Specific Flags
// RFC 2960 Section 3.3.2 Initiation (INIT) (1)
//
// This chunk is used to initiate a SCTP association between two
// endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_inithdr {
    pub init_tag: __be32,
    pub a_rwnd: __be32,
    pub num_outbound_streams: __be16,
    pub num_inbound_streams: __be16,
    pub initial_tsn: __be32,
// __u8  params[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_init_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub init_hdr: sctp_inithdr,
}

// Section 3.3.2.1. IPv4 Address Parameter (5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ipv4addr_param {
    pub param_hdr: sctp_paramhdr,
    pub addr: in_addr,
}

// Section 3.3.2.1. IPv6 Address Parameter (6)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ipv6addr_param {
    pub param_hdr: sctp_paramhdr,
    pub addr: in6_addr,
}

// Section 3.3.2.1 Cookie Preservative (9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cookie_preserve_param {
    pub param_hdr: sctp_paramhdr,
    pub lifespan_increment: __be32,
}

// Section 3.3.2.1 Host Name Address (11)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_hostname_param {
    pub param_hdr: sctp_paramhdr,
    pub hostname: [u8; ],
}

// Section 3.3.2.1 Supported Address Types (12)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_supported_addrs_param {
    pub param_hdr: sctp_paramhdr,
    pub types: [__be16; ],
}

// ADDIP Section 3.2.6 Adaptation Layer Indication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_adaptation_ind_param {
    pub param_hdr: sctp_paramhdr,
    pub adaptation_ind: __be32,
}

// ADDIP Section 4.2.7 Supported Extensions Parameter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_supported_ext_param {
    pub param_hdr: sctp_paramhdr,
    pub chunks: [__u8; ],
}

// AUTH Section 3.1 Random
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_random_param {
    pub param_hdr: sctp_paramhdr,
    pub random_val: [__u8; ],
}

// AUTH Section 3.2 Chunk List
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_chunks_param {
    pub param_hdr: sctp_paramhdr,
    pub chunks: [__u8; ],
}

// AUTH Section 3.3 HMAC Algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_hmac_algo_param {
    pub param_hdr: sctp_paramhdr,
    pub hmac_ids: [__be16; ],
}

// RFC 2960.  Section 3.3.3 Initiation Acknowledgement (INIT ACK) (2):
// The INIT ACK chunk is used to acknowledge the initiation of an SCTP
// association.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_initack_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub init_hdr: sctp_inithdr,
}

// Section 3.3.3.1 State Cookie (7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cookie_param {
    pub p: sctp_paramhdr,
    pub body: [__u8; ],
}

// Section 3.3.3.1 Unrecognized Parameters (8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_unrecognized_param {
    pub param_hdr: sctp_paramhdr,
    pub unrecognized: sctp_paramhdr,
}

//
// 3.3.4 Selective Acknowledgement (SACK) (3):
//
// This chunk is sent to the peer endpoint to acknowledge received DATA
// chunks and to inform the peer endpoint of gaps in the received
// subsequences of DATA chunks as represented by their TSNs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_gap_ack_block {
    pub start: __be16,
    pub end: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_sack_variable {
    pub gab: sctp_gap_ack_block,
    pub dup: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sackhdr {
    pub cum_tsn_ack: __be32,
    pub a_rwnd: __be32,
    pub num_gap_ack_blocks: __be16,
    pub num_dup_tsns: __be16,
// union sctp_sack_variable variable[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sack_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub sack_hdr: sctp_sackhdr,
}

// RFC 2960.  Section 3.3.5 Heartbeat Request (HEARTBEAT) (4):
//
// An endpoint should send this chunk to its peer endpoint to probe the
// reachability of a particular destination transport address defined in
// the present association.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_heartbeathdr {
    pub info: sctp_paramhdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_heartbeat_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub hb_hdr: sctp_heartbeathdr,
}

// PAD chunk could be bundled with heartbeat chunk to probe pmtu
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_pad_chunk {
    pub uh: sctp_chunkhdr,
}

// For the abort and shutdown ACK we must carry the init tag in the
// common header. Just the common header is all that is needed with a
// chunk descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_abort_chunk {
    pub uh: sctp_chunkhdr,
}

// For the graceful shutdown we must carry the tag (in common header)
// and the highest consecutive acking value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_shutdownhdr {
    pub cum_tsn_ack: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_shutdown_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub shutdown_hdr: sctp_shutdownhdr,
}

// RFC 2960.  Section 3.3.10 Operation Error (ERROR) (9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_errhdr {
    pub cause: __be16,
    pub length: __be16,
// __u8  variable[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_operr_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub err_hdr: sctp_errhdr,
}

// RFC 2960 3.3.10 - Operation Error
//
// Cause Code: 16 bits (unsigned integer)
//
// Defines the type of error conditions being reported.
// Cause Code
// Value           Cause Code
// ---------      ----------------
// 1              Invalid Stream Identifier
// 2              Missing Mandatory Parameter
// 3              Stale Cookie Error
// 4              Out of Resource
// 5              Unresolvable Address
// 6              Unrecognized Chunk Type
// 7              Invalid Mandatory Parameter
// 8              Unrecognized Parameters
// 9              No User Data
// 10              Cookie Received While Shutting Down
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_error {

    SCTP_ERROR_NO_ERROR	   = cpu_to_be16(0x00),
    SCTP_ERROR_INV_STRM	   = cpu_to_be16(0x01),
    SCTP_ERROR_MISS_PARAM 	   = cpu_to_be16(0x02),
    SCTP_ERROR_STALE_COOKIE	   = cpu_to_be16(0x03),
    SCTP_ERROR_NO_RESOURCE 	   = cpu_to_be16(0x04),
    SCTP_ERROR_DNS_FAILED      = cpu_to_be16(0x05),
    SCTP_ERROR_UNKNOWN_CHUNK   = cpu_to_be16(0x06),
    SCTP_ERROR_INV_PARAM       = cpu_to_be16(0x07),
    SCTP_ERROR_UNKNOWN_PARAM   = cpu_to_be16(0x08),
    SCTP_ERROR_NO_DATA         = cpu_to_be16(0x09),
    SCTP_ERROR_COOKIE_IN_SHUTDOWN = cpu_to_be16(0x0a),


// SCTP Implementation Guide:
// 11  Restart of an association with new addresses
// 12  User Initiated Abort
// 13  Protocol Violation
// 14  Restart of an Association with New Encapsulation Port
//

    SCTP_ERROR_RESTART         = cpu_to_be16(0x0b),
    SCTP_ERROR_USER_ABORT      = cpu_to_be16(0x0c),
    SCTP_ERROR_PROTO_VIOLATION = cpu_to_be16(0x0d),
    SCTP_ERROR_NEW_ENCAP_PORT  = cpu_to_be16(0x0e),

// ADDIP Section 3.3  New Error Causes
//
// Four new Error Causes are added to the SCTP Operational Errors,
// primarily for use in the ASCONF-ACK chunk.
//
// Value          Cause Code
// ---------      ----------------
// 0x00A0          Request to Delete Last Remaining IP Address.
// 0x00A1          Operation Refused Due to Resource Shortage.
// 0x00A2          Request to Delete Source IP Address.
// 0x00A3          Association Aborted due to illegal ASCONF-ACK
// 0x00A4          Request refused - no authorization.
//
    SCTP_ERROR_DEL_LAST_IP	= cpu_to_be16(0x00A0),
    SCTP_ERROR_RSRC_LOW	= cpu_to_be16(0x00A1),
    SCTP_ERROR_DEL_SRC_IP	= cpu_to_be16(0x00A2),
    SCTP_ERROR_ASCONF_ACK   = cpu_to_be16(0x00A3),
    SCTP_ERROR_REQ_REFUSED	= cpu_to_be16(0x00A4),

// AUTH Section 4.  New Error Cause
//
// This section defines a new error cause that will be sent if an AUTH
// chunk is received with an unsupported HMAC identifier.
// illustrates the new error cause.
//
// Cause Code      Error Cause Name
// --------------------------------------------------------------
// 0x0105          Unsupported HMAC Identifier
//
    SCTP_ERROR_UNSUP_HMAC	= cpu_to_be16(0x0105)
}

// RFC 2960.  Appendix A.  Explicit Congestion Notification.
// Explicit Congestion Notification Echo (ECNE) (12)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ecnehdr {
    pub lowest_tsn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ecne_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub ence_hdr: sctp_ecnehdr,
}

// RFC 2960.  Appendix A.  Explicit Congestion Notification.
// Congestion Window Reduced (CWR) (13)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cwrhdr {
    pub lowest_tsn: __be32,
}

// PR-SCTP
// 3.2 Forward Cumulative TSN Chunk Definition (FORWARD TSN)
//
// Forward Cumulative TSN chunk has the following format:
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Type = 192  |  Flags = 0x00 |        Length = Variable      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                      New Cumulative TSN                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |         Stream-1              |       Stream Sequence-1       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \
// /                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |         Stream-N              |       Stream Sequence-N       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Chunk Flags:
//
// Set to all zeros on transmit and ignored on receipt.
//
// New Cumulative TSN: 32 bit u_int
//
// This indicates the new cumulative TSN to the data receiver. Upon
// the reception of this value, the data receiver MUST consider
// any missing TSNs earlier than or equal to this value as received
// and stop reporting them as gaps in any subsequent SACKs.
//
// Stream-N: 16 bit u_int
//
// This field holds a stream number that was skipped by this
// FWD-TSN.
//
// Stream Sequence-N: 16 bit u_int
// This field holds the sequence number associated with the stream
// that was skipped. The stream sequence field holds the largest stream
// sequence number in this stream being skipped.  The receiver of
// the FWD-TSN's can use the Stream-N and Stream Sequence-N fields
// to enable delivery of any stranded TSN's that remain on the stream
// re-ordering queues. This field MUST NOT report TSN's corresponding
// to DATA chunk that are marked as unordered. For ordered DATA
// chunks this field MUST be filled in.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_fwdtsn_skip {
    pub stream: __be16,
    pub ssn: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_fwdtsn_hdr {
    pub new_cum_tsn: __be32,
// struct sctp_fwdtsn_skip skip[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_fwdtsn_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub fwdtsn_hdr: sctp_fwdtsn_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ifwdtsn_skip {
    pub stream: __be16,
    pub reserved: __u8,
    pub flags: __u8,
    pub mid: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ifwdtsn_hdr {
    pub new_cum_tsn: __be32,
// struct sctp_ifwdtsn_skip skip[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ifwdtsn_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub fwdtsn_hdr: sctp_ifwdtsn_hdr,
}

// ADDIP
// Section 3.1.1 Address Configuration Change Chunk (ASCONF)
//
// Serial Number: 32 bits (unsigned integer)
// This value represents a Serial Number for the ASCONF Chunk. The
// valid range of Serial Number is from 0 to 2^32-1.
// Serial Numbers wrap back to 0 after reaching 2^32 -1.
//
// Address Parameter: 8 or 20 bytes (depending on type)
// The address is an address of the sender of the ASCONF chunk,
// the address MUST be considered part of the association by the
// peer endpoint. This field may be used by the receiver of the
// ASCONF to help in finding the association. This parameter MUST
// be present in every ASCONF message i.e. it is a mandatory TLV
// parameter.
//
// ASCONF Parameter: TLV format
// Each Address configuration change is represented by a TLV
// parameter as defined in Section 3.2. One or more requests may
// be present in an ASCONF Chunk.
//
// Section 3.1.2 Address Configuration Acknowledgement Chunk (ASCONF-ACK)
//
// Serial Number: 32 bits (unsigned integer)
// This value represents the Serial Number for the received ASCONF
// Chunk that is acknowledged by this chunk. This value is copied
// from the received ASCONF Chunk.
//
// ASCONF Parameter Response: TLV format
// The ASCONF Parameter Response is used in the ASCONF-ACK to
// report status of ASCONF processing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_addip_param {
    pub param_hdr: sctp_paramhdr,
    pub crr_id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_addiphdr {
    pub serial: __be32,
// __u8	params[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_addip_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub addip_hdr: sctp_addiphdr,
}

// AUTH
// Section 4.1  Authentication Chunk (AUTH)
//
// This chunk is used to hold the result of the HMAC calculation.
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | Type = 0x0F   |   Flags=0     |             Length            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Shared Key Identifier     |   HMAC Identifier             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// \                             HMAC
// /                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Type: 1 byte (unsigned integer)
// This value MUST be set to 0x0F for  all AUTH-chunks.
//
// Flags: 1 byte (unsigned integer)
// Set to zero on transmit and ignored on receipt.
//
// Length: 2 bytes (unsigned integer)
// This value holds the length of the HMAC in bytes plus 8.
//
// Shared Key Identifier: 2 bytes (unsigned integer)
// This value describes which endpoint pair shared key is used.
//
// HMAC Identifier: 2 bytes (unsigned integer)
// This value describes which message digest is being used.  Table 2
// shows the currently defined values.
//
// The following Table 2 shows the currently defined values for HMAC
// identifiers.
//
// +-----------------+--------------------------+
// | HMAC Identifier | Message Digest Algorithm |
// +-----------------+--------------------------+
// | 0               | Reserved                 |
// | 1               | SHA-1 defined in [8]     |
// | 2               | Reserved                 |
// | 3               | SHA-256 defined in [8]   |
// +-----------------+--------------------------+
//
// HMAC: n bytes (unsigned integer) This hold the result of the HMAC
// calculation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authhdr {
    pub shkey_id: __be16,
    pub hmac_id: __be16,
// __u8   hmac[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_auth_chunk {
    pub chunk_hdr: sctp_chunkhdr,
    pub auth_hdr: sctp_authhdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_infox {
    pub sctpinfo: *mut sctp_info,
    pub asoc: *mut sctp_association,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_reconf_chunk {
    pub chunk_hdr: sctp_chunkhdr,
// __u8 params[];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_strreset_outreq {
    pub param_hdr: sctp_paramhdr,
    pub request_seq: __be32,
    pub response_seq: __be32,
    pub send_reset_at_tsn: __be32,
    pub list_of_streams: [__be16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_strreset_inreq {
    pub param_hdr: sctp_paramhdr,
    pub request_seq: __be32,
    pub list_of_streams: [__be16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_strreset_tsnreq {
    pub param_hdr: sctp_paramhdr,
    pub request_seq: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_strreset_addstrm {
    pub param_hdr: sctp_paramhdr,
    pub request_seq: __be32,
    pub number_of_streams: __be16,
    pub reserved: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_strreset_resp {
    pub param_hdr: sctp_paramhdr,
    pub response_seq: __be32,
    pub result: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_strreset_resptsn {
    pub param_hdr: sctp_paramhdr,
    pub response_seq: __be32,
    pub result: __be32,
    pub senders_next_tsn: __be32,
    pub receivers_next_tsn: __be32,
}

// UDP Encapsulation
// draft-tuexen-tsvwg-sctp-udp-encaps-cons-03.html#section-4-4
//
// The error cause indicating an "Restart of an Association with
// New Encapsulation Port"
//
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Cause Code = 14        |       Cause Length = 8        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Current Encapsulation Port  |     New Encapsulation Port    |
// +-------------------------------+-------------------------------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_new_encap_port_hdr {
    pub cur_port: __be16,
    pub new_port: __be16,
}

// Round an int up to the next multiple of 4.

// Truncate to the previous multiple of 4.

