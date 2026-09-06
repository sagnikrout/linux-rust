//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sctp.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
// SCTP kernel implementation
// (C) Copyright IBM Corp. 2001, 2004
// Copyright (c) 1999-2000 Cisco, Inc.
// Copyright (c) 1999-2001 Motorola, Inc.
// Copyright (c) 2002 Intel Corp.
//
// This file is part of the SCTP kernel implementation
//
// This header represents the structures and constants needed to support
// the SCTP Extension to the Sockets API.
//
// This SCTP implementation is free software;
// you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by
// the Free Software Foundation; either version 2, or (at your option)
// any later version.
//
// This SCTP implementation is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY; without even the implied
//
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNU CC; see the file COPYING.  If not, see
// <http://www.gnu.org/licenses/>.
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Or submit a bug report through the following website:
// http://www.sf.net/projects/lksctp
//
// Written or modified by:
// La Monte H.P. Yarroll    <piggy@acm.org>
// R. Stewart               <randall@sctp.chicago.il.us>
// K. Morneau               <kmorneau@cisco.com>
// Q. Xie                   <qxie1@email.mot.com>
// Karl Knutson             <karl@athena.chicago.il.us>
// Jon Grimm                <jgrimm@us.ibm.com>
// Daisy Chang              <daisyc@us.ibm.com>
// Ryan Layer               <rmlayer@us.ibm.com>
// Ardelle Fan              <ardelle.fan@intel.com>
// Sridhar Samudrala        <sri@us.ibm.com>
// Inaky Perez-Gonzalez     <inaky.gonzalez@intel.com>
// Vlad Yasevich            <vladislav.yasevich@hp.com>
//
// Any bugs reported given to us we will try to fix... any fixes shared will
// be incorporated into the next SCTP release.
//

pub type sctp_assoc_t = __s32;
pub const SCTP_FUTURE_ASSOC: c_int = 0;
pub const SCTP_CURRENT_ASSOC: c_int = 1;
pub const SCTP_ALL_ASSOC: c_int = 2;
// The following symbols come from the Sockets API Extensions for
// SCTP <draft-ietf-tsvwg-sctpsocket-07.txt>.
//
pub const SCTP_RTOINFO: c_int = 0;
pub const SCTP_ASSOCINFO: c_int = 1;
pub const SCTP_INITMSG: c_int = 2;

pub const SCTP_AUTOCLOSE: c_int = 4;
pub const SCTP_SET_PEER_PRIMARY_ADDR: c_int = 5;
pub const SCTP_PRIMARY_ADDR: c_int = 6;
pub const SCTP_ADAPTATION_LAYER: c_int = 7;
pub const SCTP_DISABLE_FRAGMENTS: c_int = 8;
pub const SCTP_PEER_ADDR_PARAMS: c_int = 9;
pub const SCTP_DEFAULT_SEND_PARAM: c_int = 10;
pub const SCTP_EVENTS: c_int = 11;

pub const SCTP_STATUS: c_int = 14;
pub const SCTP_GET_PEER_ADDR_INFO: c_int = 15;
pub const SCTP_DELAYED_ACK_TIME: c_int = 16;

pub const SCTP_CONTEXT: c_int = 17;
pub const SCTP_FRAGMENT_INTERLEAVE: c_int = 18;

pub const SCTP_HMAC_IDENT: c_int = 22;
pub const SCTP_AUTH_KEY: c_int = 23;
pub const SCTP_AUTH_ACTIVE_KEY: c_int = 24;
pub const SCTP_AUTH_DELETE_KEY: c_int = 25;

pub const SCTP_AUTO_ASCONF: c_int = 30;
pub const SCTP_PEER_ADDR_THLDS: c_int = 31;
pub const SCTP_RECVRCVINFO: c_int = 32;
pub const SCTP_RECVNXTINFO: c_int = 33;
pub const SCTP_DEFAULT_SNDINFO: c_int = 34;
pub const SCTP_AUTH_DEACTIVATE_KEY: c_int = 35;
pub const SCTP_REUSE_PORT: c_int = 36;
pub const SCTP_PEER_ADDR_THLDS_V2: c_int = 37;
// Internal Socket Options. Some of the sctp library functions are
// implemented using these socket options.
//

// Options 104-106 are deprecated and removed. Do not use this space

pub const SCTP_PR_SUPPORTED: c_int = 113;
pub const SCTP_DEFAULT_PRINFO: c_int = 114;
pub const SCTP_PR_ASSOC_STATUS: c_int = 115;
pub const SCTP_PR_STREAM_STATUS: c_int = 116;
pub const SCTP_RECONFIG_SUPPORTED: c_int = 117;
pub const SCTP_ENABLE_STREAM_RESET: c_int = 118;
pub const SCTP_RESET_STREAMS: c_int = 119;
pub const SCTP_RESET_ASSOC: c_int = 120;
pub const SCTP_ADD_STREAMS: c_int = 121;
pub const SCTP_SOCKOPT_PEELOFF_FLAGS: c_int = 122;
pub const SCTP_STREAM_SCHEDULER: c_int = 123;
pub const SCTP_STREAM_SCHEDULER_VALUE: c_int = 124;
pub const SCTP_INTERLEAVING_SUPPORTED: c_int = 125;
pub const SCTP_SENDMSG_CONNECT: c_int = 126;
pub const SCTP_EVENT: c_int = 127;
pub const SCTP_ASCONF_SUPPORTED: c_int = 128;
pub const SCTP_AUTH_SUPPORTED: c_int = 129;
pub const SCTP_ECN_SUPPORTED: c_int = 130;
pub const SCTP_EXPOSE_POTENTIALLY_FAILED_STATE: c_int = 131;

pub const SCTP_REMOTE_UDP_ENCAPS_PORT: c_int = 132;
pub const SCTP_PLPMTUD_PROBE_INTERVAL: c_int = 133;
// PR-SCTP policies
pub const SCTP_PR_SCTP_NONE: c_uint = 0x0000;
pub const SCTP_PR_SCTP_TTL: c_uint = 0x0010;
pub const SCTP_PR_SCTP_RTX: c_uint = 0x0020;
pub const SCTP_PR_SCTP_PRIO: c_uint = 0x0030;

pub const SCTP_PR_SCTP_MASK: c_uint = 0x0030;

// For enable stream reset
pub const SCTP_ENABLE_RESET_STREAM_REQ: c_uint = 0x01;
pub const SCTP_ENABLE_RESET_ASSOC_REQ: c_uint = 0x02;
pub const SCTP_ENABLE_CHANGE_ASSOC_REQ: c_uint = 0x04;
pub const SCTP_ENABLE_STRRESET_MASK: c_uint = 0x07;
pub const SCTP_STREAM_RESET_INCOMING: c_uint = 0x01;
pub const SCTP_STREAM_RESET_OUTGOING: c_uint = 0x02;
// These are bit fields for msghdr->msg_flags.  See section 5.1.
// On user space Linux, these live in <bits/socket.h> as an enum.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_msg_flags {
    MSG_NOTIFICATION = 0x8000,

}

// 5.3.1 SCTP Initiation Structure (SCTP_INIT)
//
// This cmsghdr structure provides information for initializing new
// SCTP associations with sendmsg().  The SCTP_INITMSG socket option
// uses this same data structure.  This structure is not used for
// recvmsg().
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   ----------------------
// IPPROTO_SCTP  SCTP_INIT      struct sctp_initmsg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_initmsg {
    pub sinit_num_ostreams: __u16,
    pub sinit_max_instreams: __u16,
    pub sinit_max_attempts: __u16,
    pub sinit_max_init_timeo: __u16,
}

// 5.3.2 SCTP Header Information Structure (SCTP_SNDRCV)
//
// This cmsghdr structure specifies SCTP options for sendmsg() and
// describes SCTP header information about a received message through
// recvmsg().
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   ----------------------
// IPPROTO_SCTP  SCTP_SNDRCV    struct sctp_sndrcvinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sndrcvinfo {
    pub sinfo_stream: __u16,
    pub sinfo_ssn: __u16,
    pub sinfo_flags: __u16,
    pub sinfo_ppid: __u32,
    pub sinfo_context: __u32,
    pub sinfo_timetolive: __u32,
    pub sinfo_tsn: __u32,
    pub sinfo_cumtsn: __u32,
    pub sinfo_assoc_id: sctp_assoc_t,
}

// 5.3.4 SCTP Send Information Structure (SCTP_SNDINFO)
//
// This cmsghdr structure specifies SCTP options for sendmsg().
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   -------------------
// IPPROTO_SCTP  SCTP_SNDINFO   struct sctp_sndinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sndinfo {
    pub snd_sid: __u16,
    pub snd_flags: __u16,
    pub snd_ppid: __u32,
    pub snd_context: __u32,
    pub snd_assoc_id: sctp_assoc_t,
}

// 5.3.5 SCTP Receive Information Structure (SCTP_RCVINFO)
//
// This cmsghdr structure describes SCTP receive information
// about a received message through recvmsg().
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   -------------------
// IPPROTO_SCTP  SCTP_RCVINFO   struct sctp_rcvinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_rcvinfo {
    pub rcv_sid: __u16,
    pub rcv_ssn: __u16,
    pub rcv_flags: __u16,
    pub rcv_ppid: __u32,
    pub rcv_tsn: __u32,
    pub rcv_cumtsn: __u32,
    pub rcv_context: __u32,
    pub rcv_assoc_id: sctp_assoc_t,
}

// 5.3.6 SCTP Next Receive Information Structure (SCTP_NXTINFO)
//
// This cmsghdr structure describes SCTP receive information
// of the next message that will be delivered through recvmsg()
// if this information is already available when delivering
// the current message.
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   -------------------
// IPPROTO_SCTP  SCTP_NXTINFO   struct sctp_nxtinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_nxtinfo {
    pub nxt_sid: __u16,
    pub nxt_flags: __u16,
    pub nxt_ppid: __u32,
    pub nxt_length: __u32,
    pub nxt_assoc_id: sctp_assoc_t,
}

// 5.3.7 SCTP PR-SCTP Information Structure (SCTP_PRINFO)
//
// This cmsghdr structure specifies SCTP options for sendmsg().
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   -------------------
// IPPROTO_SCTP  SCTP_PRINFO    struct sctp_prinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_prinfo {
    pub pr_policy: __u16,
    pub pr_value: __u32,
}

// 5.3.8 SCTP AUTH Information Structure (SCTP_AUTHINFO)
//
// This cmsghdr structure specifies SCTP options for sendmsg().
//
// cmsg_level    cmsg_type      cmsg_data[]
// ------------  ------------   -------------------
// IPPROTO_SCTP  SCTP_AUTHINFO  struct sctp_authinfo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authinfo {
    pub auth_keynumber: __u16,
}

//
// sinfo_flags: 16 bits (unsigned integer)
//
// This field may contain any of the following flags and is composed of
// a bitwise OR of these values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_sinfo_flags {
    SCTP_UNORDERED		= (1 << 0), /* Send/receive message unordered. */
    SCTP_ADDR_OVER		= (1 << 1), /* Override the primary destination. */
    SCTP_ABORT		= (1 << 2), /* Send an ABORT message to the peer. */
    SCTP_SACK_IMMEDIATELY	= (1 << 3), /* SACK should be sent without delay. */
// 2 bits here have been used by SCTP_PR_SCTP_MASK
    SCTP_SENDALL		= (1 << 6),
    SCTP_PR_SCTP_ALL	= (1 << 7),
    SCTP_NOTIFICATION	= MSG_NOTIFICATION, /* Next message is not user msg but notification. */
    SCTP_EOF		= MSG_FIN,  /* Initiate graceful shutdown process. */
}

// These are cmsg_types.

//
// 5.3.1.1 SCTP_ASSOC_CHANGE
//
// Communication notifications inform the ULP that an SCTP association
// has either begun or ended. The identifier for a new association is
// provided by this notificaion. The notification information has the
// following format:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_assoc_change {
    pub sac_type: __u16,
    pub sac_flags: __u16,
    pub sac_length: __u32,
    pub sac_state: __u16,
    pub sac_error: __u16,
    pub sac_outbound_streams: __u16,
    pub sac_inbound_streams: __u16,
    pub sac_assoc_id: sctp_assoc_t,
    pub sac_info: [__u8; ],
}

//
// sac_state: 32 bits (signed integer)
//
// This field holds one of a number of values that communicate the
// event that happened to the association.  They include:
//
// Note:  The following state names deviate from the API draft as
// the names clash too easily with other kernel symbols.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_sac_state {
    SCTP_COMM_UP,
    SCTP_COMM_LOST,
    SCTP_RESTART,
    SCTP_SHUTDOWN_COMP,
    SCTP_CANT_STR_ASSOC,
}

//
// 5.3.1.2 SCTP_PEER_ADDR_CHANGE
//
// When a destination address on a multi-homed peer encounters a change
// an interface details event is sent.  The information has the
// following structure:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_paddr_change {
    pub spc_type: __u16,
    pub spc_flags: __u16,
    pub spc_length: __u32,
    pub spc_aaddr: sockaddr_storage,
    pub spc_state: c_int,
    pub spc_error: c_int,
    pub spc_assoc_id: sctp_assoc_t,
// C attribute field omitted
//
// spc_state:  32 bits (signed integer)
//
// This field holds one of a number of values that communicate the
// event that happened to the address.  They include:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_spc_state {
    SCTP_ADDR_AVAILABLE,
    SCTP_ADDR_UNREACHABLE,
    SCTP_ADDR_REMOVED,
    SCTP_ADDR_ADDED,
    SCTP_ADDR_MADE_PRIM,
    SCTP_ADDR_CONFIRMED,
    SCTP_ADDR_POTENTIALLY_FAILED,

}

//
// 5.3.1.3 SCTP_REMOTE_ERROR
//
// A remote peer may send an Operational Error message to its peer.
// This message indicates a variety of error conditions on an
// association. The entire error TLV as it appears on the wire is
// included in a SCTP_REMOTE_ERROR event.  Please refer to the SCTP
// specification [SCTP] and any extensions for a list of possible
// error formats. SCTP error TLVs have the format:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_remote_error {
    pub sre_type: __u16,
    pub sre_flags: __u16,
    pub sre_length: __u32,
    pub sre_error: __be16,
    pub sre_assoc_id: sctp_assoc_t,
    pub sre_data: [__u8; ],
}

//
// 5.3.1.4 SCTP_SEND_FAILED
//
// If SCTP cannot deliver a message it may return the message as a
// notification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_send_failed {
    pub ssf_type: __u16,
    pub ssf_flags: __u16,
    pub ssf_length: __u32,
    pub ssf_error: __u32,
    pub ssf_info: sctp_sndrcvinfo,
    pub ssf_assoc_id: sctp_assoc_t,
    pub ssf_data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_send_failed_event {
    pub ssf_type: __u16,
    pub ssf_flags: __u16,
    pub ssf_length: __u32,
    pub ssf_error: __u32,
    pub ssfe_info: sctp_sndinfo,
    pub ssf_assoc_id: sctp_assoc_t,
    pub ssf_data: [__u8; ],
}

//
// ssf_flags: 16 bits (unsigned integer)
//
// The flag value will take one of the following values
//
// SCTP_DATA_UNSENT  - Indicates that the data was never put on
// the wire.
//
// SCTP_DATA_SENT    - Indicates that the data was put on the wire.
// Note that this does not necessarily mean that the
// data was (or was not) successfully delivered.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_ssf_flags {
    SCTP_DATA_UNSENT,
    SCTP_DATA_SENT,
}

//
// 5.3.1.5 SCTP_SHUTDOWN_EVENT
//
// When a peer sends a SHUTDOWN, SCTP delivers this notification to
// inform the application that it should cease sending data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_shutdown_event {
    pub sse_type: __u16,
    pub sse_flags: __u16,
    pub sse_length: __u32,
    pub sse_assoc_id: sctp_assoc_t,
}

//
// 5.3.1.6 SCTP_ADAPTATION_INDICATION
//
// When a peer sends a Adaptation Layer Indication parameter , SCTP
// delivers this notification to inform the application
// that of the peers requested adaptation layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_adaptation_event {
    pub sai_type: __u16,
    pub sai_flags: __u16,
    pub sai_length: __u32,
    pub sai_adaptation_ind: __u32,
    pub sai_assoc_id: sctp_assoc_t,
}

//
// 5.3.1.7 SCTP_PARTIAL_DELIVERY_EVENT
//
// When a receiver is engaged in a partial delivery of a
// message this notification will be used to indicate
// various events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_pdapi_event {
    pub pdapi_type: __u16,
    pub pdapi_flags: __u16,
    pub pdapi_length: __u32,
    pub pdapi_indication: __u32,
    pub pdapi_assoc_id: sctp_assoc_t,
    pub pdapi_stream: __u32,
    pub pdapi_seq: __u32,
}

//
// 5.3.1.8.  SCTP_AUTHENTICATION_EVENT
//
// When a receiver is using authentication this message will provide
// notifications regarding new keys being made active as well as errors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authkey_event {
    pub auth_type: __u16,
    pub auth_flags: __u16,
    pub auth_length: __u32,
    pub auth_keynumber: __u16,
    pub auth_altkeynumber: __u16,
    pub auth_indication: __u32,
    pub auth_assoc_id: sctp_assoc_t,
}

//
// 6.1.9. SCTP_SENDER_DRY_EVENT
//
// When the SCTP stack has no more user data to send or retransmit, this
// notification is given to the user. Also, at the time when a user app
// subscribes to this event, if there is no data to be sent or
// retransmit, the stack will immediately send up this notification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sender_dry_event {
    pub sender_dry_type: __u16,
    pub sender_dry_flags: __u16,
    pub sender_dry_length: __u32,
    pub sender_dry_assoc_id: sctp_assoc_t,
}

pub const SCTP_STREAM_RESET_INCOMING_SSN: c_uint = 0x0001;
pub const SCTP_STREAM_RESET_OUTGOING_SSN: c_uint = 0x0002;
pub const SCTP_STREAM_RESET_DENIED: c_uint = 0x0004;
pub const SCTP_STREAM_RESET_FAILED: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_reset_event {
    pub strreset_type: __u16,
    pub strreset_flags: __u16,
    pub strreset_length: __u32,
    pub strreset_assoc_id: sctp_assoc_t,
    pub strreset_stream_list: [__u16; ],
}

pub const SCTP_ASSOC_RESET_DENIED: c_uint = 0x0004;
pub const SCTP_ASSOC_RESET_FAILED: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_assoc_reset_event {
    pub assocreset_type: __u16,
    pub assocreset_flags: __u16,
    pub assocreset_length: __u32,
    pub assocreset_assoc_id: sctp_assoc_t,
    pub assocreset_local_tsn: __u32,
    pub assocreset_remote_tsn: __u32,
}

pub const SCTP_ASSOC_CHANGE_DENIED: c_uint = 0x0004;
pub const SCTP_ASSOC_CHANGE_FAILED: c_uint = 0x0008;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_change_event {
    pub strchange_type: __u16,
    pub strchange_flags: __u16,
    pub strchange_length: __u32,
    pub strchange_assoc_id: sctp_assoc_t,
    pub strchange_instrms: __u16,
    pub strchange_outstrms: __u16,
}

//
// Described in Section 7.3
// Ancillary Data and Notification Interest Options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_event_subscribe {
    pub sctp_data_io_event: __u8,
    pub sctp_association_event: __u8,
    pub sctp_address_event: __u8,
    pub sctp_send_failure_event: __u8,
    pub sctp_peer_error_event: __u8,
    pub sctp_shutdown_event: __u8,
    pub sctp_partial_delivery_event: __u8,
    pub sctp_adaptation_layer_event: __u8,
    pub sctp_authentication_event: __u8,
    pub sctp_sender_dry_event: __u8,
    pub sctp_stream_reset_event: __u8,
    pub sctp_assoc_reset_event: __u8,
    pub sctp_stream_change_event: __u8,
    pub sctp_send_failure_event_event: __u8,
}

//
// 5.3.1 SCTP Notification Structure
//
// The notification structure is defined as the union of all
// notification types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_notification {
    pub /: *mut *mut __u16 sn_type; / Notification type.,
    pub sn_flags: __u16,
    pub sn_length: __u32,
    pub sn_header: },
    pub sn_assoc_change: sctp_assoc_change,
    pub sn_paddr_change: sctp_paddr_change,
    pub sn_remote_error: sctp_remote_error,
    pub sn_send_failed: sctp_send_failed,
    pub sn_shutdown_event: sctp_shutdown_event,
    pub sn_adaptation_event: sctp_adaptation_event,
    pub sn_pdapi_event: sctp_pdapi_event,
    pub sn_authkey_event: sctp_authkey_event,
    pub sn_sender_dry_event: sctp_sender_dry_event,
    pub sn_strreset_event: sctp_stream_reset_event,
    pub sn_assocreset_event: sctp_assoc_reset_event,
    pub sn_strchange_event: sctp_stream_change_event,
    pub sn_send_failed_event: sctp_send_failed_event,
}

// Section 5.3.1
// All standard values for sn_type flags are greater than 2^15.
// Values from 2^15 and down are reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_sn_type {
    SCTP_SN_TYPE_BASE	= (1<<15),
    SCTP_DATA_IO_EVENT	= SCTP_SN_TYPE_BASE,

    SCTP_ASSOC_CHANGE,

    SCTP_PEER_ADDR_CHANGE,

    SCTP_SEND_FAILED,

    SCTP_REMOTE_ERROR,

    SCTP_SHUTDOWN_EVENT,

    SCTP_PARTIAL_DELIVERY_EVENT,

    SCTP_ADAPTATION_INDICATION,

    SCTP_AUTHENTICATION_EVENT,

    SCTP_SENDER_DRY_EVENT,

    SCTP_STREAM_RESET_EVENT,

    SCTP_ASSOC_RESET_EVENT,

    SCTP_STREAM_CHANGE_EVENT,

    SCTP_SEND_FAILED_EVENT,

    SCTP_SN_TYPE_MAX	= SCTP_SEND_FAILED_EVENT,

}

// Notification error codes used to fill up the error fields in some
// notifications.
// SCTP_PEER_ADDRESS_CHAGE 	: spc_error
// SCTP_ASSOC_CHANGE		: sac_error
// These names should be potentially included in the draft 04 of the SCTP
// sockets API specification.
//
// 7.1.1 Retransmission Timeout Parameters (SCTP_RTOINFO)
//
// The protocol parameters used to initialize and bound retransmission
// timeout (RTO) are tunable.  See [SCTP] for more information on how
// these parameters are used in RTO calculation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_rtoinfo {
    pub srto_assoc_id: sctp_assoc_t,
    pub srto_initial: __u32,
    pub srto_max: __u32,
    pub srto_min: __u32,
}

//
// 7.1.2 Association Parameters (SCTP_ASSOCINFO)
//
// This option is used to both examine and set various association and
// endpoint parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_assocparams {
    pub sasoc_assoc_id: sctp_assoc_t,
    pub sasoc_asocmaxrxt: __u16,
    pub sasoc_number_peer_destinations: __u16,
    pub sasoc_peer_rwnd: __u32,
    pub sasoc_local_rwnd: __u32,
    pub sasoc_cookie_life: __u32,
}

//
// 7.1.9 Set Peer Primary Address (SCTP_SET_PEER_PRIMARY_ADDR)
//
// Requests that the peer mark the enclosed address as the association
// primary. The enclosed address must be one of the association's
// locally bound addresses. The following structure is used to make a
// set primary request:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_setpeerprim {
    pub sspp_assoc_id: sctp_assoc_t,
    pub sspp_addr: sockaddr_storage,
// C attribute field omitted
//
// 7.1.10 Set Primary Address (SCTP_PRIMARY_ADDR)
//
// Requests that the local SCTP stack use the enclosed peer address as
// the association primary. The enclosed address must be one of the
// association peer's addresses. The following structure is used to
// make a set peer primary request:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_prim {
    pub ssp_assoc_id: sctp_assoc_t,
    pub ssp_addr: sockaddr_storage,
// C attribute field omitted
// For backward compatibility use, define the old name too

//
// 7.1.11 Set Adaptation Layer Indicator (SCTP_ADAPTATION_LAYER)
//
// Requests that the local endpoint set the specified Adaptation Layer
// Indication parameter for all future INIT and INIT-ACK exchanges.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_setadaptation {
    pub ssb_adaptation_ind: __u32,
}

//
// 7.1.13 Peer Address Parameters  (SCTP_PEER_ADDR_PARAMS)
//
// Applications can enable or disable heartbeats for any peer address
// of an association, modify an address's heartbeat interval, force a
// heartbeat to be sent immediately, and adjust the address's maximum
// number of retransmissions sent before an address is considered
// unreachable. The following structure is used to access and modify an
// address's parameters:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_spp_flags {
    SPP_HB_ENABLE = 1<<0,		/*Enable heartbeats*/
    SPP_HB_DISABLE = 1<<1,		/*Disable heartbeats*/
    SPP_HB = SPP_HB_ENABLE | SPP_HB_DISABLE,
    SPP_HB_DEMAND = 1<<2,		/*Send heartbeat immediately*/
    SPP_PMTUD_ENABLE = 1<<3,	/*Enable PMTU discovery*/
    SPP_PMTUD_DISABLE = 1<<4,	/*Disable PMTU discovery*/
    SPP_PMTUD = SPP_PMTUD_ENABLE | SPP_PMTUD_DISABLE,
    SPP_SACKDELAY_ENABLE = 1<<5,	/*Enable SACK*/
    SPP_SACKDELAY_DISABLE = 1<<6,	/*Disable SACK*/
    SPP_SACKDELAY = SPP_SACKDELAY_ENABLE | SPP_SACKDELAY_DISABLE,
    SPP_HB_TIME_IS_ZERO = 1<<7,	/* Set HB delay to 0 */
    SPP_IPV6_FLOWLABEL = 1<<8,
    SPP_DSCP = 1<<9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_paddrparams {
    pub spp_assoc_id: sctp_assoc_t,
    pub spp_address: sockaddr_storage,
    pub spp_hbinterval: __u32,
    pub spp_pathmaxrxt: __u16,
    pub spp_pathmtu: __u32,
    pub spp_sackdelay: __u32,
    pub spp_flags: __u32,
    pub spp_ipv6_flowlabel: __u32,
    pub spp_dscp: __u8,
// C attribute field omitted
//
// 7.1.18.  Add a chunk that must be authenticated (SCTP_AUTH_CHUNK)
//
// This set option adds a chunk type that the user is requesting to be
// received only in an authenticated way.  Changes to the list of chunks
// will only effect future associations on the socket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authchunk {
    pub sauth_chunk: __u8,
}

//
// 7.1.19.  Get or set the list of supported HMAC Identifiers (SCTP_HMAC_IDENT)
//
// This option gets or sets the list of HMAC algorithms that the local
// endpoint requires the peer to use.
//
// This here is only used by user space as is. It might not be a good idea
// to export/reveal the whole structure with reserved fields etc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_hmacalgo {
    pub shmac_num_idents: __u32,
    pub shmac_idents: [__u16; ],
}

// Sadly, user and kernel space have different names for
// this structure member, so this is to not break anything.
//

//
// 7.1.20.  Set a shared key (SCTP_AUTH_KEY)
//
// This option will set a shared secret key which is used to build an
// association shared key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authkey {
    pub sca_assoc_id: sctp_assoc_t,
    pub sca_keynumber: __u16,
    pub sca_keylength: __u16,
    pub sca_key: [__u8; ],
}

//
// 7.1.21.  Get or set the active shared key (SCTP_AUTH_ACTIVE_KEY)
//
// This option will get or set the active shared key to be used to build
// the association shared key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authkeyid {
    pub scact_assoc_id: sctp_assoc_t,
    pub scact_keynumber: __u16,
}

//
// 7.1.23.  Get or set delayed ack timer (SCTP_DELAYED_SACK)
//
// This option will effect the way delayed acks are performed.  This
// option allows you to get or set the delayed ack time, in
// milliseconds.  It also allows changing the delayed ack frequency.
// Changing the frequency to 1 disables the delayed sack algorithm.  If
// the assoc_id is 0, then this sets or gets the endpoints default
// values.  If the assoc_id field is non-zero, then the set or get
// effects the specified association for the one to many model (the
// assoc_id field is ignored by the one to one model).  Note that if
// sack_delay or sack_freq are 0 when setting this option, then the
// current values will remain unchanged.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sack_info {
    pub sack_assoc_id: sctp_assoc_t,
    pub sack_delay: u32,
    pub sack_freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_assoc_value {
    pub assoc_id: sctp_assoc_t,
    pub assoc_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_value {
    pub assoc_id: sctp_assoc_t,
    pub stream_id: u16,
    pub stream_value: u16,
}

//
// 7.2.2 Peer Address Information
//
// Applications can retrieve information about a specific peer address
// of an association, including its reachability state, congestion
// window, and retransmission timer values.  This information is
// read-only. The following structure is used to access this
// information:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_paddrinfo {
    pub spinfo_assoc_id: sctp_assoc_t,
    pub spinfo_address: sockaddr_storage,
    pub spinfo_state: __s32,
    pub spinfo_cwnd: __u32,
    pub spinfo_srtt: __u32,
    pub spinfo_rto: __u32,
    pub spinfo_mtu: __u32,
// C attribute field omitted
// Peer addresses's state.
// UNKNOWN: Peer address passed by the upper layer in sendmsg or connect[x]
// calls.
// UNCONFIRMED: Peer address received in INIT/INIT-ACK address parameters.
// Not yet confirmed by a heartbeat and not available for data
// transfers.
// ACTIVE : Peer address confirmed, active and available for data transfers.
// INACTIVE: Peer address inactive and not available for data transfers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_spinfo_state {
    SCTP_INACTIVE,
    SCTP_PF,

    SCTP_ACTIVE,
    SCTP_UNCONFIRMED,
    SCTP_UNKNOWN = 0xffff  /* Value used for transport state unknown */
}

//
// 7.2.1 Association Status (SCTP_STATUS)
//
// Applications can retrieve current status information about an
// association, including association state, peer receiver window size,
// number of unacked data chunks, and number of data chunks pending
// receipt.  This information is read-only.  The following structure is
// used to access this information:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_status {
    pub sstat_assoc_id: sctp_assoc_t,
    pub sstat_state: __s32,
    pub sstat_rwnd: __u32,
    pub sstat_unackdata: __u16,
    pub sstat_penddata: __u16,
    pub sstat_instrms: __u16,
    pub sstat_outstrms: __u16,
    pub sstat_fragmentation_point: __u32,
    pub sstat_primary: sctp_paddrinfo,
}

//
// 7.2.3.  Get the list of chunks the peer requires to be authenticated
// (SCTP_PEER_AUTH_CHUNKS)
//
// This option gets a list of chunks for a specified association that
// the peer requires to be received authenticated only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_authchunks {
    pub gauth_assoc_id: sctp_assoc_t,
    pub gauth_number_of_chunks: __u32,
    pub gauth_chunks: [u8; ],
}

// The broken spelling has been released already in lksctp-tools header,
// so don't break anyone, now that it's fixed.
//

// Association states.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_sstat_state {
    SCTP_EMPTY                = 0,
    SCTP_CLOSED               = 1,
    SCTP_COOKIE_WAIT          = 2,
    SCTP_COOKIE_ECHOED        = 3,
    SCTP_ESTABLISHED          = 4,
    SCTP_SHUTDOWN_PENDING     = 5,
    SCTP_SHUTDOWN_SENT        = 6,
    SCTP_SHUTDOWN_RECEIVED    = 7,
    SCTP_SHUTDOWN_ACK_SENT    = 8,
}

//
// 8.2.6. Get the Current Identifiers of Associations
// (SCTP_GET_ASSOC_ID_LIST)
//
// This option gets the current list of SCTP association identifiers of
// the SCTP associations handled by a one-to-many style socket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_assoc_ids {
    pub gaids_number_of_ids: __u32,
    pub gaids_assoc_id: [sctp_assoc_t; ],
}

//
// 8.3, 8.5 get all peer/local addresses in an association.
// This parameter struct is used by SCTP_GET_PEER_ADDRS and
// SCTP_GET_LOCAL_ADDRS socket options used internally to implement
// sctp_getpaddrs() and sctp_getladdrs() API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_getaddrs_old {
    pub assoc_id: sctp_assoc_t,
    pub addr_num: c_int,

    pub addrs: *mut sockaddr __user,

    pub addrs: *mut sockaddr,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_getaddrs {
    pub /*input*/: *mut sctp_assoc_t assoc_id;,
    pub /*output*/: *mut __u32 addr_num;,
    pub size*/: *mut *mut __u8 addrs[]; /output, variable,
}

// A socket user request obtained via SCTP_GET_ASSOC_STATS that retrieves
// association stats. All stats are counts except sas_maxrto and
// sas_obs_rto_ipaddr. maxrto is the max observed rto + transport since
// the last call. Will return 0 when RTO was not update since last call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_assoc_stats {
    pub /: *mut *mut sctp_assoc_t sas_assoc_id; / Input,
// Transport of observed max RTO
    pub sas_obs_rto_ipaddr: sockaddr_storage,
    pub /: *mut *mut __u64 sas_maxrto; / Maximum Observed RTO for period,
    pub /: *mut *mut __u64 sas_isacks; / SACKs received,
    pub /: *mut *mut __u64 sas_osacks; / SACKs sent,
    pub /: *mut *mut __u64 sas_opackets; / Packets sent,
    pub /: *mut *mut __u64 sas_ipackets; / Packets received,
    pub /: *mut *mut __u64 sas_rtxchunks; / Retransmitted Chunks,
    pub /: *mut *mut __u64 sas_outofseqtsns;/ TSN received > next expected,
    pub /: *mut *mut __u64 sas_idupchunks; / Dups received (ordered+unordered),
    pub /: *mut *mut __u64 sas_gapcnt; / Gap Acknowledgements Received,
    pub /: *mut *mut __u64 sas_ouodchunks; / Unordered data chunks sent,
    pub /: *mut *mut __u64 sas_iuodchunks; / Unordered data chunks received,
    pub /: *mut *mut __u64 sas_oodchunks; / Ordered data chunks sent,
    pub /: *mut *mut __u64 sas_iodchunks; / Ordered data chunks received,
    pub /: *mut *mut __u64 sas_octrlchunks; / Control chunks sent,
    pub /: *mut *mut __u64 sas_ictrlchunks; / Control chunks received,
}

//
// 8.1 sctp_bindx()
//
// The flags parameter is formed from the bitwise OR of zero or more of the
// following currently defined flags:
//
pub const SCTP_BINDX_ADD_ADDR: c_uint = 0x01;
pub const SCTP_BINDX_REM_ADDR: c_uint = 0x02;
// This is the structure that is passed as an argument(optval) to
// getsockopt(SCTP_SOCKOPT_PEELOFF).
//
// Peer Address Thresholds socket option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_paddrthlds {
    pub spt_assoc_id: sctp_assoc_t,
    pub spt_address: sockaddr_storage,
    pub spt_pathmaxrxt: __u16,
    pub spt_pathpfthld: __u16,
}

// Use a new structure with spt_pathcpthld for back compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_paddrthlds_v2 {
    pub spt_assoc_id: sctp_assoc_t,
    pub spt_address: sockaddr_storage,
    pub spt_pathmaxrxt: __u16,
    pub spt_pathpfthld: __u16,
    pub spt_pathcpthld: __u16,
}

//
// Socket Option for Getting the Association/Stream-Specific PR-SCTP Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_prstatus {
    pub sprstat_assoc_id: sctp_assoc_t,
    pub sprstat_sid: __u16,
    pub sprstat_policy: __u16,
    pub sprstat_abandoned_unsent: __u64,
    pub sprstat_abandoned_sent: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_default_prinfo {
    pub pr_assoc_id: sctp_assoc_t,
    pub pr_value: __u32,
    pub pr_policy: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_info {
    pub sctpi_tag: __u32,
    pub sctpi_state: __u32,
    pub sctpi_rwnd: __u32,
    pub sctpi_unackdata: __u16,
    pub sctpi_penddata: __u16,
    pub sctpi_instrms: __u16,
    pub sctpi_outstrms: __u16,
    pub sctpi_fragmentation_point: __u32,
    pub sctpi_inqueue: __u32,
    pub sctpi_outqueue: __u32,
    pub sctpi_overall_error: __u32,
    pub sctpi_max_burst: __u32,
    pub sctpi_maxseg: __u32,
    pub sctpi_peer_rwnd: __u32,
    pub sctpi_peer_tag: __u32,
    pub sctpi_peer_capable: __u8,
    pub sctpi_peer_sack: __u8,
    pub __reserved1: __u16,
// assoc status info
    pub sctpi_isacks: __u64,
    pub sctpi_osacks: __u64,
    pub sctpi_opackets: __u64,
    pub sctpi_ipackets: __u64,
    pub sctpi_rtxchunks: __u64,
    pub sctpi_outofseqtsns: __u64,
    pub sctpi_idupchunks: __u64,
    pub sctpi_gapcnt: __u64,
    pub sctpi_ouodchunks: __u64,
    pub sctpi_iuodchunks: __u64,
    pub sctpi_oodchunks: __u64,
    pub sctpi_iodchunks: __u64,
    pub sctpi_octrlchunks: __u64,
    pub sctpi_ictrlchunks: __u64,
// primary transport info
    pub sctpi_p_address: sockaddr_storage,
    pub sctpi_p_state: __s32,
    pub sctpi_p_cwnd: __u32,
    pub sctpi_p_srtt: __u32,
    pub sctpi_p_rto: __u32,
    pub sctpi_p_hbinterval: __u32,
    pub sctpi_p_pathmaxrxt: __u32,
    pub sctpi_p_sackdelay: __u32,
    pub sctpi_p_sackfreq: __u32,
    pub sctpi_p_ssthresh: __u32,
    pub sctpi_p_partial_bytes_acked: __u32,
    pub sctpi_p_flight_size: __u32,
    pub sctpi_p_error: __u16,
    pub __reserved2: __u16,
// sctp sock info
    pub sctpi_s_autoclose: __u32,
    pub sctpi_s_adaptation_ind: __u32,
    pub sctpi_s_pd_point: __u32,
    pub sctpi_s_nodelay: __u8,
    pub sctpi_s_disable_fragments: __u8,
    pub sctpi_s_v4mapped: __u8,
    pub sctpi_s_frag_interleave: __u8,
    pub sctpi_s_type: __u32,
    pub __reserved3: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_reset_streams {
    pub srs_assoc_id: sctp_assoc_t,
    pub srs_flags: u16,
    pub /: *mut *mut uint16_t srs_number_streams; / 0 == ALL,
    pub /: *mut *mut uint16_t srs_stream_list[]; / list if srs_num_streams is not 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_add_streams {
    pub sas_assoc_id: sctp_assoc_t,
    pub sas_instrms: u16,
    pub sas_outstrms: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_event {
    pub se_assoc_id: sctp_assoc_t,
    pub se_type: u16,
    pub se_on: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_udpencaps {
    pub sue_assoc_id: sctp_assoc_t,
    pub sue_address: sockaddr_storage,
    pub sue_port: u16,
}

// SCTP Stream schedulers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_sched_type {
    SCTP_SS_FCFS,
    SCTP_SS_DEFAULT = SCTP_SS_FCFS,
    SCTP_SS_PRIO,
    SCTP_SS_RR,
    SCTP_SS_FC,
    SCTP_SS_WFQ,
    SCTP_SS_MAX = SCTP_SS_WFQ
}

// Probe Interval socket option
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_probeinterval {
    pub spi_assoc_id: sctp_assoc_t,
    pub spi_address: sockaddr_storage,
    pub spi_interval: __u32,
}
