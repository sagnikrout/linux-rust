//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/constants.h
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
// SCTP kernel implementation
// (C) Copyright IBM Corp. 2001, 2004
// Copyright (c) 1999-2000 Cisco, Inc.
// Copyright (c) 1999-2001 Motorola, Inc.
// Copyright (c) 2001 Intel Corp.
//
// This file is part of the SCTP kernel implementation
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson          <karl@athena.chicago.il.us>
// Randall Stewart       <randall@stewart.chicago.il.us>
// Ken Morneau           <kmorneau@cisco.com>
// Qiaobing Xie          <qxie1@motorola.com>
// Xingang Guo           <xingang.guo@intel.com>
// Sridhar Samudrala     <samudrala@us.ibm.com>
// Daisy Chang           <daisyc@us.ibm.com>
//

// Macro flag: #define __sctp_constants_h__

// Value used for stream negotiation.
// Since CIDs are sparse, we need all four of the following
// symbols.  CIDs are dense through SCTP_CID_BASE_MAX.
//

pub const SCTP_NUM_ADDIP_CHUNK_TYPES: c_int = 2;
pub const SCTP_NUM_PRSCTP_CHUNK_TYPES: c_int = 1;
pub const SCTP_NUM_RECONF_CHUNK_TYPES: c_int = 1;
pub const SCTP_NUM_AUTH_CHUNK_TYPES: c_int = 1;

// These are the different flavours of event.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_event_type {
    SCTP_EVENT_T_CHUNK = 1,
    SCTP_EVENT_T_TIMEOUT,
    SCTP_EVENT_T_OTHER,
    SCTP_EVENT_T_PRIMITIVE
}

// As a convenience for the state machine, we append SCTP_EVENT_* and
// SCTP_ULP_* to the list of possible chunks.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_event_timeout {
    SCTP_EVENT_TIMEOUT_NONE = 0,
    SCTP_EVENT_TIMEOUT_T1_COOKIE,
    SCTP_EVENT_TIMEOUT_T1_INIT,
    SCTP_EVENT_TIMEOUT_T2_SHUTDOWN,
    SCTP_EVENT_TIMEOUT_T3_RTX,
    SCTP_EVENT_TIMEOUT_T4_RTO,
    SCTP_EVENT_TIMEOUT_T5_SHUTDOWN_GUARD,
    SCTP_EVENT_TIMEOUT_HEARTBEAT,
    SCTP_EVENT_TIMEOUT_RECONF,
    SCTP_EVENT_TIMEOUT_PROBE,
    SCTP_EVENT_TIMEOUT_SACK,
    SCTP_EVENT_TIMEOUT_AUTOCLOSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_event_other {
    SCTP_EVENT_NO_PENDING_TSN = 0,
    SCTP_EVENT_ICMP_PROTO_UNREACH,
}

// These are primitive requests from the ULP.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_event_primitive {
    SCTP_PRIMITIVE_ASSOCIATE = 0,
    SCTP_PRIMITIVE_SHUTDOWN,
    SCTP_PRIMITIVE_ABORT,
    SCTP_PRIMITIVE_SEND,
    SCTP_PRIMITIVE_REQUESTHEARTBEAT,
    SCTP_PRIMITIVE_ASCONF,
    SCTP_PRIMITIVE_RECONF,
}

// We define here a utility type for manipulating subtypes.
// The subtype constructors all work like this:
//
// union sctp_subtype foo = SCTP_ST_CHUNK(SCTP_CID_INIT);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_subtype {
    pub chunk: sctp_cid,
    pub timeout: sctp_event_timeout,
    pub other: sctp_event_other,
    pub primitive: sctp_event_primitive,
}

// Internal error codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_ierror {
    SCTP_IERROR_NO_ERROR	        = 0,
    SCTP_IERROR_BASE		= 1000,
    SCTP_IERROR_NO_COOKIE,
    SCTP_IERROR_BAD_SIG,
    SCTP_IERROR_STALE_COOKIE,
    SCTP_IERROR_NOMEM,
    SCTP_IERROR_MALFORMED,
    SCTP_IERROR_BAD_TAG,
    SCTP_IERROR_BIG_GAP,
    SCTP_IERROR_DUP_TSN,
    SCTP_IERROR_HIGH_TSN,
    SCTP_IERROR_IGNORE_TSN,
    SCTP_IERROR_NO_DATA,
    SCTP_IERROR_BAD_STREAM,
    SCTP_IERROR_BAD_PORTS,
    SCTP_IERROR_AUTH_BAD_HMAC,
    SCTP_IERROR_AUTH_BAD_KEYID,
    SCTP_IERROR_PROTO_VIOLATION,
    SCTP_IERROR_ERROR,
    SCTP_IERROR_ABORT,
}

// SCTP state defines for internal state machine
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_state {

    SCTP_STATE_CLOSED		= 0,
    SCTP_STATE_COOKIE_WAIT		= 1,
    SCTP_STATE_COOKIE_ECHOED	= 2,
    SCTP_STATE_ESTABLISHED		= 3,
    SCTP_STATE_SHUTDOWN_PENDING	= 4,
    SCTP_STATE_SHUTDOWN_SENT	= 5,
    SCTP_STATE_SHUTDOWN_RECEIVED	= 6,
    SCTP_STATE_SHUTDOWN_ACK_SENT	= 7,

}

// These are values for sk->state.
// For a UDP-style SCTP socket, the states are defined as follows
// - A socket in SCTP_SS_CLOSED state indicates that it is not willing to
// accept new associations, but it can initiate the creation of new ones.
// - A socket in SCTP_SS_LISTENING state indicates that it is willing to
// accept new  associations and can initiate the creation of new ones.
// - A socket in SCTP_SS_ESTABLISHED state indicates that it is a peeled off
// socket with one association.
// For a TCP-style SCTP socket, the states are defined as follows
// - A socket in SCTP_SS_CLOSED state indicates that it is not willing to
// accept new associations, but it can initiate the creation of new ones.
// - A socket in SCTP_SS_LISTENING state indicates that it is willing to
// accept new associations, but cannot initiate the creation of new ones.
// - A socket in SCTP_SS_ESTABLISHED state indicates that it has a single
// association.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_sock_state {
    SCTP_SS_CLOSED         = TCP_CLOSE,
    SCTP_SS_LISTENING      = TCP_LISTEN,
    SCTP_SS_ESTABLISHING   = TCP_SYN_SENT,
    SCTP_SS_ESTABLISHED    = TCP_ESTABLISHED,
    SCTP_SS_CLOSING        = TCP_CLOSE_WAIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_plpmtud_state {
    SCTP_PL_DISABLED,
    SCTP_PL_BASE,
    SCTP_PL_SEARCH,
    SCTP_PL_COMPLETE,
    SCTP_PL_ERROR,
}

pub const SCTP_BASE_PLPMTU: c_int = 1200;
pub const SCTP_MAX_PLPMTU: c_int = 9000;
pub const SCTP_MIN_PLPMTU: c_int = 512;
pub const SCTP_MAX_PROBES: c_int = 3;
pub const SCTP_PL_BIG_STEP: c_int = 32;
pub const SCTP_PL_MIN_STEP: c_int = 4;
// These functions map various type to printable names.
// This is a table of printable names of sctp_state_t's.
// Maximum chunk length considering padding requirements.
// Encourage Cookie-Echo bundling by pre-fragmenting chunks a little
// harder (until reaching ESTABLISHED state).
//
// Guess at how big to make the TSN mapping array.
// We guarantee that we can handle at least this big a gap between the
// cumulative ACK and the highest TSN.  In practice, we can often
// handle up to twice this value.
//
// NEVER make this more than 32767 (2^15-1).  The Gap Ack Blocks in a
// SACK (see  section 3.3.4) are only 16 bits, so 2*SCTP_TSN_MAP_SIZE
// must be less than 65535 (2^16 - 1), or we will have overflow
// problems creating SACK's.
//

pub const SCTP_TSN_MAP_SIZE: c_int = 4096;
// We will not record more than this many duplicate TSNs between two
// SACKs.  The minimum PMTU is 512.  Remove all the headers and there
// is enough room for 117 duplicate reports.  Round down to the
// nearest power of 2.
//
// Heartbeat interval - 30 secs

// Delayed sack timer - 200ms

// RTO.Initial              - 3  seconds
// RTO.Min                  - 1  second
// RTO.Max                  - 60 seconds
// RTO.Alpha                - 1/8
// RTO.Beta                 - 1/4
//

// Maximum number of new data packets that can be sent in a burst.
pub const SCTP_DEFAULT_MAX_BURST: c_int = 4;

// rcvbuf, which is 1/8 of initial
// window
//

// to which we will raise the P-MTU.
//

// functions simpler to write.
//

// These are the values for pf exposure, UNUSED is to keep compatible with old
// applications by default.
//

pub const SCTP_PS_RETRANS_MAX: c_uint = 0xffff;
// These return values describe the success or failure of a number of
// routines which form the lower interface to SCTP_outqueue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_xmit {
    SCTP_XMIT_OK,
    SCTP_XMIT_PMTU_FULL,
    SCTP_XMIT_RWND_FULL,
    SCTP_XMIT_DELAY,
}

// These are the commands for manipulating transports.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_transport_cmd {
    SCTP_TRANSPORT_UP,
    SCTP_TRANSPORT_DOWN,
    SCTP_TRANSPORT_PF,
}

// These are the address scopes defined mainly for IPv4 addresses
// based on draft of SCTP IPv4 scoping <draft-stewart-tsvwg-sctp-ipv4-00.txt>.
// These scopes are hopefully generic enough to be used on scoping both
// IPv4 and IPv6 addresses in SCTP.
// At this point, the IPv6 scopes will be mapped to these internal scopes
// as much as possible.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_scope {
    SCTP_SCOPE_GLOBAL,		/* IPv4 global addresses */
    SCTP_SCOPE_PRIVATE,		/* IPv4 private addresses */
    SCTP_SCOPE_LINK,		/* IPv4 link local address */
    SCTP_SCOPE_LOOPBACK,		/* IPv4 loopback address */
    SCTP_SCOPE_UNUSABLE,		/* IPv4 unusable addresses */
}

// Based on IPv4 scoping <draft-stewart-tsvwg-sctp-ipv4-00.txt>,
// SCTP IPv4 unusable addresses: 0.0.0.0/8, 224.0.0.0/4, 192.88.99.0/24.
// Also, RFC 8.4, non-unicast addresses are not considered valid SCTP
// addresses.
//

// Flags used for the bind address copy functions.
pub const SCTP_ADDR4_ALLOWED: c_uint = 0x00000001	/* IPv4 address is allowed by;
pub const SCTP_ADDR6_ALLOWED: c_uint = 0x00000002	/* IPv6 address is allowed by;
pub const SCTP_ADDR4_PEERSUPP: c_uint = 0x00000004	/* IPv4 address is supported by;
pub const SCTP_ADDR6_PEERSUPP: c_uint = 0x00000008	/* IPv6 address is supported by;
// Reasons to retransmit.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_retransmit_reason {
    SCTP_RTXR_T3_RTX,
    SCTP_RTXR_FAST_RTX,
    SCTP_RTXR_PMTUD,
    SCTP_RTXR_T1_RTX,
}

// Reasons to lower cwnd.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_lower_cwnd {
    SCTP_LOWER_CWND_T3_RTX,
    SCTP_LOWER_CWND_FAST_RTX,
    SCTP_LOWER_CWND_ECNE,
    SCTP_LOWER_CWND_INACTIVE,
}

// SCTP-AUTH Necessary constants
// SCTP-AUTH, Section 3.3
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

// SCTP-AUTH, Section 3.2
// The chunk types for INIT, INIT-ACK, SHUTDOWN-COMPLETE and AUTH chunks
// MUST NOT be listed in the CHUNKS parameter
//
pub const SCTP_NUM_NOAUTH_CHUNKS: c_int = 4;

// SCTP-AUTH Section 6.1
// The RANDOM parameter MUST contain a 32 byte random number.
//
pub const SCTP_AUTH_RANDOM_LENGTH: c_int = 32;
pub const SCTP_PROBE_TIMER_MIN: c_int = 5000;
