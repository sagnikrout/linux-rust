//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/structs.h
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
// email addresses:
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Randall Stewart	    <randall@sctp.chicago.il.us>
// Ken Morneau	    <kmorneau@cisco.com>
// Qiaobing Xie	    <qxie1@email.mot.com>
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson	    <karl@athena.chicago.il.us>
// Jon Grimm		    <jgrimm@us.ibm.com>
// Xingang Guo	    <xingang.guo@intel.com>
// Hui Huang		    <hui.huang@nokia.com>
// Sridhar Samudrala	    <sri@us.ibm.com>
// Daisy Chang	    <daisyc@us.ibm.com>
// Dajiang Zhang	    <dajiang.zhang@nokia.com>
// Ardelle Fan	    <ardelle.fan@intel.com>
// Ryan Layer	    <rmlayer@us.ibm.com>
// Anup Pemmaiah	    <pemmaiah@cc.usu.edu>
// Kevin Gao             <kevin.gao@intel.com>
//

// Macro flag: #define __sctp_structs_h__

// A convenience structure for handling sockaddr structures.
// We should wean ourselves off this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_addr {
    pub /: *mut *mut sockaddr_inet sa; / Large enough for both address families,
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}

// Forward declarations for data structures.

// Structures useful for managing bind/connect.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_bind_bucket {
    pub port: c_ushort,
    pub fastreuse: signed char,
    pub fastreuseport: signed char,
    pub fastuid: kuid_t,
    pub node: hlist_node,
    pub owner: hlist_head,
    pub net: *mut net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_bind_hashbucket {
    pub lock: spinlock_t,
    pub chain: hlist_head,
}

// Used for hashing all associations.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_hashbucket {
    pub lock: rwlock_t,
    pub chain: hlist_head,
    pub __attribute__((__aligned__(8))): },
// The SCTP globals structure.
// This is a list of groups of functions for each address
// family that we support.
//
    pub address_families: list_head,
// This is the hash of all endpoints.
    pub ep_hashtable: *mut sctp_hashbucket,
// This is the sctp port control hash.
    pub port_hashtable: *mut sctp_bind_hashbucket,
// This is the hash of all transports.
    pub transport_hashtable: rhltable,
// Sizes of above hashtables.
    pub ep_hashsize: c_int,
    pub port_hashsize: c_int,
// Default initialization values to be applied to new associations.
    pub max_instreams: __u16,
    pub max_outstreams: __u16,
// Flag to indicate whether computing and verifying checksum
// is disabled.
    pub checksum_disable: bool,
    pub sctp_globals: },

// SCTP Socket type: UDP or TCP style.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_socket_type {
    SCTP_SOCKET_UDP = 0,
    SCTP_SOCKET_UDP_HIGH_BANDWIDTH,
    SCTP_SOCKET_TCP
}

// Per socket SCTP information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sock {
// inet_sock has to be the first member of sctp_sock
    pub inet: inet_sock,
// What kind of a socket is this?
    pub type: sctp_socket_type,
// PF_ family specific functions.
    pub pf: *mut sctp_pf,
// What is our base endpointer?
    pub ep: *mut sctp_endpoint,
    pub bind_hash: *mut sctp_bind_bucket,
// Various Socket Options.
    pub default_stream: __u16,
    pub default_ppid: __u32,
    pub default_flags: __u16,
    pub default_context: __u32,
    pub default_timetolive: __u32,
    pub default_rcv_context: __u32,
    pub max_burst: c_int,
// Heartbeat interval: The endpoint sends out a Heartbeat chunk to
// the destination address every heartbeat interval. This value
// will be inherited by all new associations.
//
    pub hbinterval: __u32,
    pub probe_interval: __u32,
    pub udp_port: __be16,
    pub encap_port: __be16,
// This is the max_retrans value for new associations.
    pub pathmaxrxt: __u16,
    pub flowlabel: __u32,
    pub dscp: __u8,
    pub pf_retrans: __u16,
    pub ps_retrans: __u16,
// The initial Path MTU to use for new associations.
    pub pathmtu: __u32,
// The default SACK delay timeout for new associations.
    pub sackdelay: __u32,
    pub sackfreq: __u32,
// Flags controlling Heartbeat, SACK delay, and Path MTU Discovery.
    pub param_flags: __u32,
    pub default_ss: __u32,
    pub rtoinfo: sctp_rtoinfo,
    pub paddrparam: sctp_paddrparams,
    pub assocparams: sctp_assocparams,
//
// These two structures must be grouped together for the usercopy
// whitelist region.
//
    pub subscribe: __u16,
    pub initmsg: sctp_initmsg,
    pub user_frag: c_int,
    pub autoclose: __u32,
    pub adaptation_ind: __u32,
    pub pd_point: __u32,
    pub pd_mode: core::sync::atomic::AtomicI32,
// Receive to here while partial delivery is in effect.
    pub pd_lobby: sk_buff_head,
    pub auto_asconf_list: list_head,
    pub do_auto_asconf: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp6_sock {
    pub sctp: sctp_sock,
    pub inet6: ipv6_pinfo,
}

// This is our APPLICATION-SPECIFIC state cookie.
// THIS IS NOT DICTATED BY THE SPECIFICATION.
//
// These are the parts of an association which we send in the cookie.
// Most of these are straight out of:
// RFC2960 12.2 Parameters necessary per association (i.e. the TCB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cookie {
// My	       : Tag expected in every inbound packet and sent
// Verification: in the INIT or INIT ACK chunk.
// Tag	       :
//
    pub my_vtag: __u32,
// Peer's      : Tag expected in every outbound packet except
// Verification: in the INIT chunk.
// Tag	       :
//
    pub peer_vtag: __u32,
// The rest of these are not from the spec, but really need to
// be in the cookie.
//
// My Tie Tag  : Assist in discovering a restarting association.
    pub my_ttag: __u32,
// Peer's Tie Tag: Assist in discovering a restarting association.
    pub peer_ttag: __u32,
// When does this cookie expire?
    pub expiration: ktime_t,
// Number of inbound/outbound streams which are set
// and negotiated during the INIT process.
//
    pub sinit_num_ostreams: __u16,
    pub sinit_max_instreams: __u16,
// This is the first sequence number I used.
    pub initial_tsn: __u32,
// This holds the originating address of the INIT packet.
    pub peer_addr: sctp_addr,
// IG Section 2.35.3
// Include the source port of the INIT-ACK
//
    pub my_port: __u16,
    pub prsctp_capable: __u8,
// Padding for future use
    pub padding: __u8,
    pub adaptation_ind: __u32,
    pub sizeof(__u16)]: *mut *mut SCTP_AUTH_NUM_HMACS,
    pub SCTP_AUTH_MAX_CHUNKS]: __u8 auth_chunks[sizeof(struct sctp_paramhdr) +,
// This is a shim for my peer's INIT packet, followed by
// a copy of the raw address list of the association.
// The length of the raw address list is saved in the
// raw_addr_list_len field, which will be used at the time when
// the association TCB is re-constructed from the cookie.
//
    pub raw_addr_list_len: __u32,
// struct sctp_init_chunk peer_init[];
}

// The format of our cookie that we send to our peer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_signed_cookie {
    pub mac: [__u8; SCTP_COOKIE_MAC_SIZE],
    pub /: *mut *mut __u32 __pad; / force sctp_cookie alignment to 64 bits,
    pub c: sctp_cookie,
    pub __packed: },
// This is another convenience type to allocate memory for address
// params for the maximum size and pass such structures around
// internally.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_addr_param {
    pub p: sctp_paramhdr,
    pub v4: sctp_ipv4addr_param,
    pub v6: sctp_ipv6addr_param,
}

// A convenience type to allow walking through the various
// parameters and avoid casting all over the place.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sctp_params {
    pub v: *mut c_void,
    pub p: *mut sctp_paramhdr,
    pub life: *mut sctp_cookie_preserve_param,
    pub dns: *mut sctp_hostname_param,
    pub cookie: *mut sctp_cookie_param,
    pub sat: *mut sctp_supported_addrs_param,
    pub v4: *mut sctp_ipv4addr_param,
    pub v6: *mut sctp_ipv6addr_param,
    pub addr: *mut sctp_addr_param,
    pub aind: *mut sctp_adaptation_ind_param,
    pub ext: *mut sctp_supported_ext_param,
    pub random: *mut sctp_random_param,
    pub chunks: *mut sctp_chunks_param,
    pub hmac_algo: *mut sctp_hmac_algo_param,
    pub addip: *mut sctp_addip_param,
}

// RFC 2960.  Section 3.3.5 Heartbeat.
// Heartbeat Information: variable length
// The Sender-specific Heartbeat Info field should normally include
// information about the sender's current time when this HEARTBEAT
// chunk is sent and the destination transport address to which this
// HEARTBEAT is sent (see Section 8.3).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sender_hb_info {
    pub param_hdr: sctp_paramhdr,
    pub daddr: sctp_addr,
    pub sent_at: c_ulong,
    pub hb_nonce: __u64,
    pub probe_size: __u32,
}

extern "C" {
    pub fn sctp_stream_init_ext(stream: *mut sctp_stream, sid: __u16) -> c_int;
}
extern "C" {
    pub fn sctp_stream_free(stream: *mut sctp_stream);
}
extern "C" {
    pub fn sctp_stream_clear(stream: *mut sctp_stream);
}
extern "C" {
    pub fn sctp_stream_update(stream: *mut sctp_stream, new: *mut sctp_stream);
}
// What is the current SSN number for this stream?

// Return the next SSN number for this stream.

// Skip over this ssn and all below.

// What is the current MID number for this stream?

// Return the next MID number for this stream.

// Skip over this mid and all below.

// What is the current MID_uo number for this stream?

// Return the next MID_uo number for this stream.

//
// Pointers to address related SCTP functions.
// (i.e. things that depend on the address family.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_af {
    pub ): *mut sctp_transport,
    pub optlen): c_uint,
    pub optlen): *mut int __user,
    pub sk): *mut sock,
    pub fl): *mut flowi,
    pub ): *mut net_device,
    pub addr2): *const sctp_addr,
    pub src): *mut sctp_addr,
    pub saddr): c_int,
    pub sk): *mut sock,
    pub iif): __be16 port, int,
    pub ): *mut sctp_addr_param,
    pub ): *const sk_buff,
    pub ): *mut *mut sctp_scope (scope)(union sctp_addr,
    pub __be16): *mut *mut *mut void (inaddr_any) (union sctp_addr ,,
    pub ): *const *const int (is_any) (union sctp_addr,
    pub ): *mut sctp_sock,
    pub sk): *const *const int (skb_iif) (struct sk_buff,
    pub sk): *const *const int (skb_sdif)(struct sk_buff,
    pub sk): *const *const int (is_ce) (struct sk_buff,
    pub addr): *mut sctp_addr,
    pub sk): *mut *mut void (ecn_capable)(struct sock,
    pub net_header_len: __u16,
    pub sockaddr_len: c_int,
    pub sk): *mut *mut int (ip_options_len)(struct sock,
    pub sa_family: sa_family_t,
    pub list: list_head,
}

extern "C" {
    pub fn sctp_register_af(: *mut sctp_af) -> c_int;
}
// Protocol family functions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_pf {
    pub ): *mut *mut *mut *mut void (event_msgname)(struct sctp_ulpevent , char , int,
    pub ): *mut *mut *mut *mut void (skb_msgname) (struct sk_buff , char , int,
    pub ): *mut *mut int (af_supported) (sa_family_t, struct sctp_sock,
    pub ): *mut sctp_sock,
    pub ): *mut *mut *mut int (bind_verify) (struct sctp_sock , union sctp_addr,
    pub ): *mut *mut *mut int (send_verify) (struct sctp_sock , union sctp_addr,
    pub ): *const *const *const int (supported_addrs)(struct sctp_sock , __be16,
    pub addr): *mut *mut *mut int (addr_to_user)(struct sctp_sock sk, union sctp_addr,
    pub sk): *mut *mut *mut void (to_sk_saddr)(union sctp_addr , struct sock,
    pub sk): *mut *mut *mut void (to_sk_daddr)(union sctp_addr , struct sock,
    pub newsk): *mut *mut *mut void (copy_ip_options)(struct sock sk, struct sock,
    pub af: *mut sctp_af,
}

// Structure to track chunk fragments that have been acked, but peer
// fragments of the same message have not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_datamsg {
// Chunks waiting to be submitted to lower layer.
    pub chunks: list_head,
// Reference counting.
    pub refcnt: refcount_t,
// When is this message no longer interesting to the peer?
    pub expires_at: c_ulong,
// Did the message fail to send?
    pub send_error: c_int,
    pub /: *mut *mut abandoned:1; / should this message be abandoned,
}

extern "C" {
    pub fn sctp_datamsg_free(: *mut sctp_datamsg);
}
extern "C" {
    pub fn sctp_datamsg_put(: *mut sctp_datamsg);
}
extern "C" {
    pub fn sctp_chunk_fail(: *mut sctp_chunk, error: c_int);
}
extern "C" {
    pub fn sctp_chunk_abandoned(: *mut sctp_chunk) -> c_int;
}
// RFC2960 1.4 Key Terms
//
// o Chunk: A unit of information within an SCTP packet, consisting of
// a chunk header and chunk-specific content.
//
// As a matter of convenience, we remember the SCTP common header for
// each chunk as well as a few other header pointers...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_chunk {
    pub list: list_head,
    pub refcnt: refcount_t,
// How many times this chunk have been sent, for prsctp RTX policy
    pub sent_count: c_int,
// This is our link to the per-transport transmitted list.
    pub transmitted_list: list_head,
// List in specific stream outq
    pub stream_list: list_head,
}

// This field is used by chunks that hold fragmented data.
// For the first fragment this is the list that holds the rest of
// fragments. For the remaining fragments, this is the link to the
// frag_list maintained in the first fragment.
//
// This points to the sk_buff containing the actual data.
// In case of GSO packets, this will store the head one
// In case of auth enabled, this will point to the shkey
// These are the SCTP headers by reverse order in a packet.
// Note that some of these may happen more than once.  In that
// case, we point at the "current" one, whatever that means
// for that level of header.
//
// We point this at the FIRST TLV parameter to chunk_hdr.
// This needs to be recoverable for SCTP_SEND_FAILED events.
// Which association does this belong to?
// What endpoint received this chunk?
// We fill this in if we are calculating RTT.
// What is the origin IP address for this chunk?
// Destination address for this chunk.
// For outbound message, track all fragments for SEND_FAILED.
// For an inbound chunk, this tells us where it came from.
// For an outbound chunk, it tells us where we'd like it to
// go.	It is NULL if we have no preference.
//
// SCTP-AUTH:  For the special case inbound processing of COOKIE-ECHO
// we need save a pointer to the AUTH chunk, since the SCTP-AUTH
// spec violates the principle premis that all chunks are processed
// in order.
//
pub const SCTP_CAN_FRTX: c_uint = 0x0;
pub const SCTP_NEED_FRTX: c_uint = 0x1;
pub const SCTP_DONT_FRTX: c_uint = 0x2;

extern "C" {
    pub fn sctp_chunk_hold(: *mut sctp_chunk);
}
extern "C" {
    pub fn sctp_chunk_put(: *mut sctp_chunk);
}
extern "C" {
    pub fn sctp_chunk_free(: *mut sctp_chunk);
}
extern "C" {
    pub fn ntohs(_arg: ch->subh.data_hdr->stream) -> return;
}
// This is a structure for holding either an IPv6 or an IPv4 address.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sockaddr_entry {
    pub list: list_head,
    pub rcu: rcu_head,
    pub a: sctp_addr,
    pub state: __u8,
    pub valid: __u8,
}

pub const SCTP_ADDRESS_TICK_DELAY: c_int = 500;
// This structure holds lists of chunks as we are assembling for
// transmission.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_packet {
// These are the SCTP header values (host order) for the packet.
    pub source_port: __u16,
    pub destination_port: __u16,
    pub vtag: __u32,
// This contains the payload chunks.
    pub chunk_list: list_head,
// This is the overhead of the sctp and ip headers.
    pub overhead: usize,
// This is the total size of all chunks INCLUDING padding.
    pub size: usize,
// This is the maximum size this packet may have
    pub max_size: usize,
// The packet is destined for this transport address.
// The function we finally use to pass down to the next lower
// layer lives in the transport structure.
//
    pub transport: *mut sctp_transport,
// pointer to the auth chunk for this packet
    pub auth: *mut sctp_chunk,
    pub /: *mut *mut ipfragok:1; / So let ip fragment this packet,
}

extern "C" {
    pub fn sctp_packet_config(: *mut sctp_packet, vtag: __u32, _arg: c_int);
}
extern "C" {
    pub fn sctp_packet_transmit(: *mut sctp_packet, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn sctp_packet_free(: *mut sctp_packet);
}
// This represents a remote transport address.
// For local transport addresses, we just use union sctp_addr.
//
// RFC2960 Section 1.4 Key Terms
//
// o	Transport address:  A Transport Address is traditionally defined
// by Network Layer address, Transport Layer protocol and Transport
// Layer port number.  In the case of SCTP running over IP, a
// transport address is defined by the combination of an IP address
// and an SCTP port number (where SCTP is the Transport protocol).
//
// RFC2960 Section 7.1 SCTP Differences from TCP Congestion control
//
// o	The sender keeps a separate congestion control parameter set for
// each of the destination addresses it can send to (not each
// source-destination pair but for each destination).  The parameters
// should decay if the address is not used for a long enough time
// period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_transport {
// A list of transports.
    pub transports: list_head,
    pub node: rhlist_head,
// Reference counting.
    pub refcnt: refcount_t,
// RTO-Pending : A flag used to track if one of the DATA
// chunks sent to this address is currently being
// used to compute a RTT. If this flag is 0,
// the next DATA chunk sent to this destination
// should be used to compute a RTT and this flag
// should be set. Every time the RTT
// calculation completes (i.e. the DATA chunk
// is SACK'd) clear this flag.
//
// hb_sent : a flag that signals that we have a pending
// heartbeat.
//
// Is the Path MTU update pending on this transport
// Has this transport moved the ctsn since we last sacked
    pub dst_cookie: u32,
    pub fl: flowi,
// This is the peer's IP address and port.
    pub ipaddr: sctp_addr,
// These are the functions we call to handle LLP stuff.
    pub af_specific: *mut sctp_af,
// Which association do we belong to?
    pub asoc: *mut sctp_association,
// RFC2960
//
// 12.3 Per Transport Address Data
//
// For each destination transport address in the peer's
// address list derived from the INIT or INIT ACK chunk, a
// number of data elements needs to be maintained including:
//
// RTO	       : The current retransmission timeout value.
    pub rto: c_ulong,
    pub /: *mut *mut __u32 rtt; / This is the most recent RTT.,
// RTTVAR      : The current RTT variation.
    pub rttvar: __u32,
// SRTT	       : The current smoothed round trip time.
    pub srtt: __u32,
//
// These are the congestion stats.
//
// cwnd	       : The current congestion window.
    pub /: *mut *mut __u32 cwnd; / This is the actual cwnd.,
// ssthresh    : The current slow start threshold value.
    pub ssthresh: __u32,
// partial     : The tracking method for increase of cwnd when in
// bytes acked : congestion avoidance mode (see Section 6.2.2)
//
    pub partial_bytes_acked: __u32,
// Data that has been sent, but not acknowledged.
    pub flight_size: __u32,
    pub /: *mut *mut __u32 burst_limited; / Holds old cwnd when max.burst is applied,
// Destination
    pub dst: *mut dst_entry,
// Source address.
    pub saddr: sctp_addr,
// Heartbeat interval: The endpoint sends out a Heartbeat chunk to
// the destination address every heartbeat interval.
//
    pub hbinterval: c_ulong,
    pub probe_interval: c_ulong,
// SACK delay timeout
    pub sackdelay: c_ulong,
    pub sackfreq: __u32,
    pub mtu_info: core::sync::atomic::AtomicI32,
// When was the last time that we heard from this transport? We use
// this to pick new active and retran paths.
//
    pub last_time_heard: ktime_t,
// When was the last time that we sent a chunk using this
// transport? We use this to check for idle transports
//
    pub last_time_sent: c_ulong,
// Last time(in jiffies) when cwnd is reduced due to the congestion
// indication based on ECNE chunk.
//
    pub last_time_ecne_reduced: c_ulong,
    pub encap_port: __be16,
// This is the max_retrans value for the transport and will
// be initialized from the assocs value.  This can be changed
// using the SCTP_SET_PEER_ADDR_PARAMS socket option.
//
    pub pathmaxrxt: __u16,
    pub flowlabel: __u32,
    pub dscp: __u8,
// This is the partially failed retrans value for the transport
// and will be initialized from the assocs value.  This can be changed
// using the SCTP_PEER_ADDR_THLDS socket option
//
    pub pf_retrans: __u16,
// Used for primary path switchover.
    pub ps_retrans: __u16,
// PMTU	      : The current known path MTU.
    pub pathmtu: __u32,
// Flags controlling Heartbeat, SACK delay, and Path MTU Discovery.
    pub param_flags: __u32,
// The number of times INIT has been sent on this transport.
    pub init_sent_count: c_int,
// state       : The current state of this destination,
// : i.e. SCTP_ACTIVE, SCTP_INACTIVE, SCTP_UNKNOWN.
//
    pub state: c_int,
// These are the error stats for this destination.
// Error count : The current error count for this destination.
    pub error_count: c_ushort,
// Per	       : A timer used by each destination.
// Destination :
// Timer       :
//
// [Everywhere else in the text this is called T3-rtx. -ed]
//
    pub T3_rtx_timer: timer_list,
// Heartbeat timer is per destination.
    pub hb_timer: timer_list,
// Timer to handle ICMP proto unreachable envets
    pub proto_unreach_timer: timer_list,
// Timer to handler reconf chunk rtx
    pub reconf_timer: timer_list,
// Timer to send a probe HB packet for PLPMTUD
    pub probe_timer: timer_list,
// Since we're using per-destination retransmission timers
// (see above), we're also using per-destination "transmitted"
// queues.  This probably ought to be a private struct
// accessible only within the outqueue, but it's not, yet.
//
    pub transmitted: list_head,
// We build bundle-able packets for this transport here.
    pub packet: sctp_packet,
// This is the list of transports that have chunks to send.
    pub send_ready: list_head,
// State information saved for SFR_CACC algorithm. The key
// idea in SFR_CACC is to maintain state at the sender on a
// per-destination basis when a changeover happens.
// char changeover_active;
// char cycling_changeover;
// __u32 next_tsn_at_change;
// char cacc_saw_newack;
//
// An unsigned integer, which stores the next TSN to be
// used by the sender, at the moment of changeover.
//
    pub next_tsn_at_change: __u32,
// A flag which indicates the occurrence of a changeover
    pub changeover_active: c_char,
// A flag which indicates whether the change of primary is
// the first switch to this destination address during an
// active switch.
//
    pub cycling_changeover: c_char,
// A temporary flag, which is used during the processing of
// a SACK to estimate the causative TSN(s)'s group.
//
    pub cacc_saw_newack: c_char,
    pub cacc: },
    pub pmtu: __u16,
    pub probe_size: __u16,
    pub probe_high: __u16,
    pub probe_count: __u8,
    pub state: __u8,
    pub /: *mut *mut } pl; / plpmtud related,
// 64-bit random number sent with heartbeat.
    pub hb_nonce: __u64,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn sctp_transport_pmtu(: *mut sctp_transport, sk: *mut sock);
}
extern "C" {
    pub fn sctp_transport_free(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_reset_t3_rtx(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_reset_hb_timer(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_reset_reconf_timer(transport: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_reset_probe_timer(transport: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_reset_raise_timer(transport: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_hold(: *mut sctp_transport) -> c_int;
}
extern "C" {
    pub fn sctp_transport_put(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_update_rto(: *mut sctp_transport, _arg: __u32);
}
extern "C" {
    pub fn sctp_transport_raise_cwnd(: *mut sctp_transport, _arg: __u32, _arg: __u32);
}
extern "C" {
    pub fn sctp_transport_burst_limited(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_burst_reset(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_timeout(: *mut sctp_transport) -> c_ulong;
}
extern "C" {
    pub fn sctp_transport_reset(t: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_update_pmtu(t: *mut sctp_transport, pmtu: u32) -> bool;
}
extern "C" {
    pub fn sctp_transport_immediate_rtx(: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_dst_release(t: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_dst_confirm(t: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_pl_send(t: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_transport_pl_recv(t: *mut sctp_transport) -> bool;
}
// This is the structure we use to queue packets as they come into
// SCTP.  We write packets to it and read chunks from it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_inq {
// This is actually a queue of sctp_chunk each
// containing a partially decoded packet.
//
    pub in_chunk_list: list_head,
// This is the packet which is currently off the in queue and is
// being worked on through the inbound chunk processing.
//
    pub in_progress: *mut sctp_chunk,
// This is the delayed task to finish delivering inbound
// messages.
//
    pub immediate: work_struct,
}

extern "C" {
    pub fn sctp_inq_init(: *mut sctp_inq);
}
extern "C" {
    pub fn sctp_inq_free(: *mut sctp_inq);
}
extern "C" {
    pub fn sctp_inq_push(: *mut sctp_inq, packet: *mut sctp_chunk);
}
extern "C" {
    pub fn sctp_inq_set_th_handler(: *mut sctp_inq, _arg: work_func_t);
}
// This is the structure we use to hold outbound chunks.  You push
// chunks in and they automatically pop out the other end as bundled
// packets (it calls (*output_handler)()).
//
// This structure covers sections 6.3, 6.4, 6.7, 6.8, 6.10, 7., 8.1,
// and 8.2 of the v13 draft.
//
// It handles retransmissions.	The connection to the timeout portion
// of the state machine is through sctp_..._timeout() and timeout_handler.
//
// If you feed it SACKs, it will eat them.
//
// If you give it big chunks, it will fragment them.
//
// It assigns TSN's to data chunks.  This happens at the last possible
// instant before transmission.
//
// When free()'d, it empties itself out via output_handler().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_outq {
    pub asoc: *mut sctp_association,
// Data pending that has never been transmitted.
    pub out_chunk_list: list_head,
// Stream scheduler being used
    pub sched: *const sctp_sched_ops,
    pub /: *mut *mut unsigned int out_qlen; / Total length of queued data chunks.,
// Error of send failed, may used in SCTP_SEND_FAILED event.
    pub error: c_uint,
// These are control chunks we want to send.
    pub control_chunk_list: list_head,
// These are chunks that have been sacked but are above the
// CTSN, or cumulative tsn ack point.
//
    pub sacked: list_head,
// Put chunks on this list to schedule them for
// retransmission.
//
    pub retransmit: list_head,
// Put chunks on this list to save them for FWD TSN processing as
// they were abandoned.
//
    pub abandoned: list_head,
// How many unackd bytes do we have in-flight?
    pub outstanding_bytes: __u32,
// Are we doing fast-rtx on this queue
    pub fast_rtx: c_char,
// Corked?
    pub cork: c_char,
}

extern "C" {
    pub fn sctp_outq_init(: *mut sctp_association, : *mut sctp_outq);
}
extern "C" {
    pub fn sctp_outq_teardown(: *mut sctp_outq);
}
extern "C" {
    pub fn sctp_outq_free(sctp_outq*: *mut struct);
}
extern "C" {
    pub fn sctp_outq_tail(: *mut sctp_outq, chunk: *mut sctp_chunk, _arg: gfp_t);
}
extern "C" {
    pub fn sctp_outq_sack(: *mut sctp_outq, : *mut sctp_chunk) -> c_int;
}
extern "C" {
    pub fn sctp_outq_is_empty(: *const sctp_outq) -> c_int;
}
extern "C" {
    pub fn sctp_retransmit_mark(: *mut sctp_outq, : *mut sctp_transport, _arg: __u8);
}
extern "C" {
    pub fn sctp_outq_uncork(: *mut sctp_outq, gfp: gfp_t);
}
extern "C" {
    pub fn sctp_generate_fwdtsn(q: *mut sctp_outq, sack_ctsn: __u32);
}
// Uncork and flush an outqueue.
// SCTP skb control block.
// sctp_input_cb is currently used on rx and sock rx queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_input_cb {
    pub h4: inet_skb_parm,

    pub h6: inet6_skb_parm,

    pub header: },
    pub chunk: *mut sctp_chunk,
    pub af: *mut sctp_af,
    pub encap_port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_output_cb {
    pub last: *mut sk_buff,
}

// These bind address data fields common between endpoints and associations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_bind_addr {
// RFC 2960 12.1 Parameters necessary for the SCTP instance
//
// SCTP Port:	The local SCTP port number the endpoint is
// bound to.
//
    pub port: __u16,
// RFC 2960 12.1 Parameters necessary for the SCTP instance
//
// Address List: The list of IP addresses that this instance
// has bound.  This information is passed to one's
// peer(s) in INIT and INIT ACK chunks.
//
    pub address_list: list_head,
}

extern "C" {
    pub fn sctp_bind_addr_init(: *mut sctp_bind_addr, port: __u16);
}
extern "C" {
    pub fn sctp_bind_addr_free(: *mut sctp_bind_addr);
}
extern "C" {
    pub fn sctp_del_bind_addr(: *mut sctp_bind_addr, : *mut sctp_addr) -> c_int;
}
extern "C" {
    pub fn sctp_scope(addr: *const sctp_addr) -> sctp_scope;
}
extern "C" {
    pub fn sctp_is_any(sk: *mut sock, addr: *const sctp_addr) -> c_int;
}
extern "C" {
    pub fn sctp_is_ep_boundall(sk: *mut sock) -> c_int;
}
// What type of endpoint?
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_endpoint_type {
    SCTP_EP_TYPE_SOCKET,
    SCTP_EP_TYPE_ASSOCIATION,
}

//
// A common base class to bridge the implementation view of a
// socket (usually listening) endpoint versus an association's
// local endpoint.
// This common structure is useful for several purposes:
// 1) Common interface for lookup routines.
// a) Subfunctions work for either endpoint or association
// b) Single interface to lookup allows hiding the lookup lock rather
// than acquiring it externally.
// 2) Common interface for the inbound chunk handling/state machine.
// 3) Common object handling routines for reference counting, etc.
// 4) Disentangle association lookup from endpoint lookup, where we
// do not have to find our endpoint to find our association.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ep_common {
// Runtime type information.  What kind of endpoint is this?
    pub type: sctp_endpoint_type,
// Some fields to help us manage this object.
// refcnt   - Reference count access to this object.
// dead     - Do not attempt to use this object.
//
    pub refcnt: refcount_t,
    pub dead: bool,
// What socket does this endpoint belong to?
    pub sk: *mut sock,
// Cache netns and it won't change once set
    pub net: *mut net,
// This is where we receive inbound chunks.
    pub inqueue: sctp_inq,
// This substructure includes the defining parameters of the
// endpoint:
// bind_addr.port is our shared port number.
// bind_addr.address_list is our set of local IP addresses.
//
    pub bind_addr: sctp_bind_addr,
}

// RFC Section 1.4 Key Terms
//
// o SCTP endpoint: The logical sender/receiver of SCTP packets. On a
// multi-homed host, an SCTP endpoint is represented to its peers as a
// combination of a set of eligible destination transport addresses to
// which SCTP packets can be sent and a set of eligible source
// transport addresses from which SCTP packets can be received.
// All transport addresses used by an SCTP endpoint must use the
// same port number, but can use multiple IP addresses. A transport
// address used by an SCTP endpoint must not be used by another
// SCTP endpoint. In other words, a transport address is unique
// to an SCTP endpoint.
//
// From an implementation perspective, each socket has one of these.
// A TCP-style socket will have exactly one association on one of
// these.  An UDP-style socket will have multiple associations hanging
// off one of these.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_endpoint {
// Common substructure for endpoint and association.
    pub base: sctp_ep_common,
// Fields to help us manage our entries in the hash tables.
    pub node: hlist_node,
    pub hashent: c_int,
// Associations: A list of current associations and mappings
// to the data consumers for each association. This
// may be in the form of a hash table or other
// implementation dependent structure. The data
// consumers may be process identification
// information such as file descriptors, named pipe
// pointer, or table pointers dependent on how SCTP
// is implemented.
//
// This is really a list of struct sctp_association entries.
    pub asocs: list_head,
// Cookie authentication key used by this endpoint
    pub cookie_auth_key: hmac_sha256_key,
// sendbuf acct. policy.
    pub sndbuf_policy: __u32,
// rcvbuf acct. policy.
    pub rcvbuf_policy: __u32,
// SCTP-AUTH: hmacs for the endpoint encoded into parameter
    pub auth_hmacs_list: *mut sctp_hmac_algo_param,
// SCTP-AUTH: chunks to authenticate encoded into parameter
    pub auth_chunk_list: *mut sctp_chunks_param,
// SCTP-AUTH: endpoint shared keys
    pub endpoint_shared_keys: list_head,
    pub active_key_id: __u16,
    pub strreset_enable: __u8,
    pub rcu: rcu_head,
}

// Recover the outer endpoint structure.
// These are function signatures for manipulating endpoints.
extern "C" {
    pub fn sctp_endpoint_free(: *mut sctp_endpoint);
}
extern "C" {
    pub fn sctp_endpoint_put(: *mut sctp_endpoint);
}
extern "C" {
    pub fn sctp_endpoint_hold(ep: *mut sctp_endpoint) -> c_int;
}
extern "C" {
    pub fn sctp_endpoint_add_asoc(: *mut sctp_endpoint, : *mut sctp_association);
}
extern "C" {
    pub fn sctp_generate_tag(: *const sctp_endpoint) -> __u32;
}
extern "C" {
    pub fn sctp_generate_tsn(: *const sctp_endpoint) -> __u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_inithdr_host {
    pub init_tag: __u32,
    pub a_rwnd: __u32,
    pub num_outbound_streams: __u16,
    pub num_inbound_streams: __u16,
    pub initial_tsn: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_priorities {
// List of priorities scheduled
    pub prio_sched: list_head,
// List of streams scheduled
    pub active: list_head,
// The next stream in line
    pub next: *mut sctp_stream_out_ext,
    pub prio: __u16,
    pub users: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_out_ext {
    pub 1]: __u64 abandoned_unsent[SCTP_PR_INDEX(MAX) +,
    pub 1]: __u64 abandoned_sent[SCTP_PR_INDEX(MAX) +,
    pub /: *mut *mut list_head outq; / chunks enqueued by this stream,
// Scheduled streams list
    pub prio_list: list_head,
    pub prio_head: *mut sctp_stream_priorities,
}

// Fields used by RR scheduler
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_out {
    pub mid: __u32,
    pub ssn: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_in {
    pub mid: __u32,
    pub ssn: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream {
    pub out: GENRADIX(struct sctp_stream_out),
    pub in: GENRADIX(struct sctp_stream_in),
    pub outcnt: __u16,
    pub incnt: __u16,
// Current stream being sent, if any
    pub out_curr: *mut sctp_stream_out,
// Fields used by priority scheduler
// List of priorities scheduled
    pub prio_list: list_head,
}

// Fields used by RR scheduler
// List of streams scheduled
// The next stream in line
extern "C" {
    pub fn genradix_ptr(_arg: &stream->out, _arg: sid) -> return;
}
extern "C" {
    pub fn genradix_ptr(_arg: &stream->in, _arg: sid) -> return;
}

pub const SCTP_STREAM_CLOSED: c_uint = 0x00;
pub const SCTP_STREAM_OPEN: c_uint = 0x01;
// SCTP_GET_ASSOC_STATS counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_priv_assoc_stats {
// Maximum observed rto in the association during subsequent
// observations. Value is set to 0 if no RTO measurement took place
// The transport where the max_rto was observed is returned in
// obs_rto_ipaddr
//
    pub obs_rto_ipaddr: sockaddr_storage,
    pub max_obs_rto: __u64,
// Total In and Out SACKs received and sent
    pub isacks: __u64,
    pub osacks: __u64,
// Total In and Out packets received and sent
    pub opackets: __u64,
    pub ipackets: __u64,
// Total retransmitted chunks
    pub rtxchunks: __u64,
// TSN received > next expected
    pub outofseqtsns: __u64,
// Duplicate Chunks received
    pub idupchunks: __u64,
// Gap Ack Blocks received
    pub gapcnt: __u64,
// Unordered data chunks sent and received
    pub ouodchunks: __u64,
    pub iuodchunks: __u64,
// Ordered data chunks sent and received
    pub oodchunks: __u64,
    pub iodchunks: __u64,
// Control chunks sent and received
    pub octrlchunks: __u64,
    pub ictrlchunks: __u64,
}

// RFC2960
//
// 12. Recommended Transmission Control Block (TCB) Parameters
//
// This section details a recommended set of parameters that should
// be contained within the TCB for an implementation. This section is
// for illustrative purposes and should not be deemed as requirements
// on an implementation or as an exhaustive list of all parameters
// inside an SCTP TCB. Each implementation may need its own additional
// parameters for optimization.
//
// Here we have information about each individual association.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_association {
// A base structure common to endpoint and association.
// In this context, it represents the associations's view
// of the local endpoint of the association.
//
    pub base: sctp_ep_common,
// Associations on the same socket.
    pub asocs: list_head,
// association id.
    pub assoc_id: sctp_assoc_t,
// This is our parent endpoint.
    pub ep: *mut sctp_endpoint,
// These are those association elements needed in the cookie.
    pub c: sctp_cookie,
// This is all information about our peer.
// transport_addr_list
//
// Peer	       : A list of SCTP transport addresses that the
// Transport   : peer is bound to. This information is derived
// Address     : from the INIT or INIT ACK and is used to
// List	       : associate an inbound packet with a given
// : association. Normally this information is
// : hashed or keyed for quick lookup and access
// : of the TCB.
// : The list is also initialized with the list
// : of addresses passed with the sctp_connectx()
// : call.
//
// It is a list of SCTP_transport's.
//
    pub transport_addr_list: list_head,
// rwnd
//
// Peer Rwnd   : Current calculated value of the peer's rwnd.
//
    pub rwnd: __u32,
// transport_count
//
// Peer        : A count of the number of peer addresses
// Transport   : in the Peer Transport Address List.
// Address     :
// Count       :
//
    pub transport_count: __u16,
// port
// The transport layer port number.
//
    pub port: __u16,
// primary_path
//
// Primary     : This is the current primary destination
// Path	       : transport address of the peer endpoint.  It
// : may also specify a source transport address
// : on this endpoint.
//
// All of these paths live on transport_addr_list.
//
// At the bakeoffs, we discovered that the intent of
// primaryPath is that it only changes when the ULP
// asks to have it changed.  We add the activePath to
// designate the connection we are currently using to
// transmit new data and most control chunks.
//
    pub primary_path: *mut sctp_transport,
// Cache the primary path address here, when we
// need a an address for msg_name.
//
    pub primary_addr: sctp_addr,
// active_path
// The path that we are currently using to
// transmit new data and most control chunks.
//
    pub active_path: *mut sctp_transport,
// retran_path
//
// RFC2960 6.4 Multi-homed SCTP Endpoints
// ...
// Furthermore, when its peer is multi-homed, an
// endpoint SHOULD try to retransmit a chunk to an
// active destination transport address that is
// different from the last destination address to
// which the DATA chunk was sent.
//
    pub retran_path: *mut sctp_transport,
// Pointer to last transport I have sent on.
    pub last_sent_to: *mut sctp_transport,
// This is the last transport I have received DATA on.
    pub last_data_from: *mut sctp_transport,
//
// Mapping  An array of bits or bytes indicating which out of
// Array    order TSN's have been received (relative to the
// Last Rcvd TSN). If no gaps exist, i.e. no out of
// order packets have been received, this array
// will be set to all zero. This structure may be
// in the form of a circular buffer or bit array.
//
// Last Rcvd   : This is the last TSN received in
// TSN	       : sequence. This value is set initially by
// : taking the peer's Initial TSN, received in
// : the INIT or INIT ACK chunk, and subtracting
// : one from it.
//
// Throughout most of the specification this is called the
// "Cumulative TSN ACK Point".	In this case, we
// ignore the advice in 12.2 in favour of the term
// used in the bulk of the text.  This value is hidden
// in tsn_map--we get it by calling sctp_tsnmap_get_ctsn().
//
    pub tsn_map: sctp_tsnmap,
// This mask is used to disable sending the ASCONF chunk
// with specified parameter to peer.
//
    pub addip_disabled_mask: __be16,
// These are capabilities which our peer advertised.
// sack_needed:
// This flag indicates if the next received
// packet is to be responded to with a
// SACK. This is initialized to 0.  When a packet
// is received sack_cnt is incremented. If this value
// reaches 2 or more, a SACK is sent and the
// value is reset to 0. Note: This is used only
// when no DATA chunks are received out of
// order.  When DATA chunks are out of order,
// SACK's are not delayed (see Section 6).
//
    pub sack_cnt: __u32,
    pub /: *mut *mut __u32 adaptation_ind; / Adaptation Code point.,
    pub i: sctp_inithdr_host,
    pub cookie: *mut c_void,
    pub cookie_len: c_int,
// ADDIP Section 4.2 Upon reception of an ASCONF Chunk.
// C1) ... "Peer-Serial-Number'. This value MUST be initialized to the
// Initial TSN Value minus 1
//
    pub addip_serial: __u32,
// SCTP-AUTH: We need to know pears random number, hmac list
// and authenticated chunk list.  All that is part of the
// cookie and these are just pointers to those locations
//
    pub peer_random: *mut sctp_random_param,
    pub peer_chunks: *mut sctp_chunks_param,
    pub peer_hmacs: *mut sctp_hmac_algo_param,
    pub peer: },
// State       : A state variable indicating what state the
// : association is in, i.e. COOKIE-WAIT,
// : COOKIE-ECHOED, ESTABLISHED, SHUTDOWN-PENDING,
// : SHUTDOWN-SENT, SHUTDOWN-RECEIVED, SHUTDOWN-ACK-SENT.
//
// Note: No "CLOSED" state is illustrated since if a
// association is "CLOSED" its TCB SHOULD be removed.
//
// In this implementation we DO have a CLOSED
// state which is used during initiation and shutdown.
//
// State takes values from SCTP_STATE_*.
//
    pub state: sctp_state,
// Overall     : The overall association error count.
// Error Count : [Clear this any time I get something.]
//
    pub overall_error_count: c_int,
// The cookie life I award for any cookie.
    pub cookie_life: ktime_t,
// These are the association's initial, max, and min RTO values.
// These values will be initialized by system defaults, but can
// be modified via the SCTP_RTOINFO socket option.
//
    pub rto_initial: c_ulong,
    pub rto_max: c_ulong,
    pub rto_min: c_ulong,
// Maximum number of new data packets that can be sent in a burst.
    pub max_burst: c_int,
// This is the max_retrans value for the association.  This value will
// be initialized from system defaults, but can be
// modified by the SCTP_ASSOCINFO socket option.
//
    pub max_retrans: c_int,
// This is the partially failed retrans value for the transport
// and will be initialized from the assocs value.  This can be
// changed using the SCTP_PEER_ADDR_THLDS socket option
//
    pub pf_retrans: __u16,
// Used for primary path switchover.
    pub ps_retrans: __u16,
// Maximum number of times the endpoint will retransmit INIT
    pub max_init_attempts: __u16,
// How many times have we resent an INIT?
    pub init_retries: __u16,
// The largest timeout or RTO value to use in attempting an INIT
    pub max_init_timeo: c_ulong,
// Heartbeat interval: The endpoint sends out a Heartbeat chunk to
// the destination address every heartbeat interval. This value
// will be inherited by all new transports.
//
    pub hbinterval: c_ulong,
    pub probe_interval: c_ulong,
    pub encap_port: __be16,
// This is the max_retrans value for new transports in the
// association.
//
    pub pathmaxrxt: __u16,
    pub flowlabel: __u32,
    pub dscp: __u8,
// Flag that path mtu update is pending
    pub pmtu_pending: __u8,
// Association : The smallest PMTU discovered for all of the
// PMTU	       : peer's transport addresses.
//
    pub pathmtu: __u32,
// Flags controlling Heartbeat, SACK delay, and Path MTU Discovery.
    pub param_flags: __u32,
    pub sackfreq: __u32,
// SACK delay timeout
    pub sackdelay: c_ulong,
    pub timeouts: [c_ulong; SCTP_NUM_TIMEOUT_TYPES],
    pub timers: [timer_list; SCTP_NUM_TIMEOUT_TYPES],
// Transport to which SHUTDOWN chunk was last sent.
    pub shutdown_last_sent_to: *mut sctp_transport,
// Transport to which INIT chunk was last sent.
    pub init_last_sent_to: *mut sctp_transport,
// How many times have we resent a SHUTDOWN
    pub shutdown_retries: c_int,
// Next TSN    : The next TSN number to be assigned to a new
// : DATA chunk.  This is sent in the INIT or INIT
// : ACK chunk to the peer and incremented each
// : time a DATA chunk is assigned a TSN
// : (normally just prior to transmit or during
// : fragmentation).
//
    pub next_tsn: __u32,
//
// Last Rcvd   : This is the last TSN received in sequence.  This value
// TSN	       : is set initially by taking the peer's Initial TSN,
// : received in the INIT or INIT ACK chunk, and
// : subtracting one from it.
//
// Most of RFC 2960 refers to this as the Cumulative TSN Ack Point.
//
    pub ctsn_ack_point: __u32,
// PR-SCTP Advanced.Peer.Ack.Point
    pub adv_peer_ack_point: __u32,
// Highest TSN that is acknowledged by incoming SACKs.
    pub highest_sacked: __u32,
// TSN marking the fast recovery exit point
    pub fast_recovery_exit: __u32,
// Flag to track the current fast recovery state
    pub fast_recovery: __u8,
// The number of unacknowledged data chunks.  Reported through
// the SCTP_STATUS sockopt.
//
    pub unack_data: __u16,
// The total number of data chunks that we've had to retransmit
// as the result of a T3 timer expiration
//
    pub rtx_data_chunks: __u32,
// This is the association's receive buffer space.  This value is used
// to set a_rwnd field in an INIT or a SACK chunk.
//
    pub rwnd: __u32,
// This is the last advertised value of rwnd over a SACK chunk.
    pub a_rwnd: __u32,
// Number of bytes by which the rwnd has slopped.  The rwnd is allowed
// to slop over a maximum of the association's frag_point.
//
    pub rwnd_over: __u32,
// Keeps treack of rwnd pressure.  This happens when we have
// a window, but not receive buffer (i.e small packets).  This one
// is releases slowly (1 PMTU at a time ).
//
    pub rwnd_press: __u32,
// This is the sndbuf size in use for the association.
// This corresponds to the sndbuf size for the association,
// as specified in the sk->sndbuf.
//
    pub sndbuf_used: c_int,
// This is the amount of memory that this association has allocated
// in the receive path at any given time.
//
    pub rmem_alloc: core::sync::atomic::AtomicI32,
// This is the wait queue head for send requests waiting on
// the association sndbuf space.
//
    pub wait: wait_queue_head_t,
// The message size at which SCTP fragmentation will occur.
    pub frag_point: __u32,
    pub user_frag: __u32,
// Counter used to count INIT errors.
    pub init_err_counter: c_int,
// Count the number of INIT cycles (for doubling timeout).
    pub init_cycle: c_int,
// Default send parameters.
    pub default_stream: __u16,
    pub default_flags: __u16,
    pub default_ppid: __u32,
    pub default_context: __u32,
    pub default_timetolive: __u32,
// Default receive parameters
    pub default_rcv_context: __u32,
// Stream arrays
    pub stream: sctp_stream,
// All outbound chunks go through this structure.
    pub outqueue: sctp_outq,
// A smart pipe that will handle reordering and fragmentation,
// as well as handle passing events up to the ULP.
//
    pub ulpq: sctp_ulpq,
// Last TSN that caused an ECNE Chunk to be sent.
    pub last_ecne_tsn: __u32,
// Last TSN that caused a CWR Chunk to be sent.
    pub last_cwr_tsn: __u32,
// How many duplicated TSNs have we seen?
    pub numduptsns: c_int,
// These are to support
// "SCTP Extensions for Dynamic Reconfiguration of IP Addresses
// and Enforcement of Flow and Message Limits"
// <draft-ietf-tsvwg-addip-sctp-02.txt>
// or "ADDIP" for short.
//
// ADDIP Section 4.1.1 Congestion Control of ASCONF Chunks
//
// R1) One and only one ASCONF Chunk MAY be in transit and
// unacknowledged at any one time.  If a sender, after sending
// an ASCONF chunk, decides it needs to transfer another
// ASCONF Chunk, it MUST wait until the ASCONF-ACK Chunk
// returns from the previous ASCONF Chunk before sending a
// subsequent ASCONF. Note this restriction binds each side,
// so at any time two ASCONF may be in-transit on any given
// association (one sent from each endpoint).
//
// [This is our one-and-only-one ASCONF in flight.  If we do
// not have an ASCONF in flight, this is NULL.]
//
    pub addip_last_asconf: *mut sctp_chunk,
// ADDIP Section 5.2 Upon reception of an ASCONF Chunk.
//
// This is needed to implement items E1 - E4 of the updated
// spec.  Here is the justification:
//
// Since the peer may bundle multiple ASCONF chunks toward us,
// we now need the ability to cache multiple ACKs.  The section
// describes in detail how they are cached and cleaned up.
//
    pub asconf_ack_list: list_head,
// These ASCONF chunks are waiting to be sent.
//
// These chunks can't be pushed to outqueue until receiving
// ASCONF_ACK for the previous ASCONF indicated by
// addip_last_asconf, so as to guarantee that only one ASCONF
// is in flight at any time.
//
// ADDIP Section 4.1.1 Congestion Control of ASCONF Chunks
//
// In defining the ASCONF Chunk transfer procedures, it is
// essential that these transfers MUST NOT cause congestion
// within the network.	To achieve this, we place these
// restrictions on the transfer of ASCONF Chunks:
//
// R1) One and only one ASCONF Chunk MAY be in transit and
// unacknowledged at any one time.  If a sender, after sending
// an ASCONF chunk, decides it needs to transfer another
// ASCONF Chunk, it MUST wait until the ASCONF-ACK Chunk
// returns from the previous ASCONF Chunk before sending a
// subsequent ASCONF. Note this restriction binds each side,
// so at any time two ASCONF may be in-transit on any given
// association (one sent from each endpoint).
//
// [I really think this is EXACTLY the sort of intelligence
// which already resides in sctp_outq.	 Please move this
// queue and its supporting logic down there.	--piggy]
//
    pub addip_chunk_list: list_head,
// ADDIP Section 4.1 ASCONF Chunk Procedures
//
// A2) A serial number should be assigned to the Chunk. The
// serial number SHOULD be a monotonically increasing
// number. The serial number SHOULD be initialized at
// the start of the association to the same value as the
// Initial TSN and every time a new ASCONF chunk is created
// it is incremented by one after assigning the serial number
// to the newly created chunk.
//
// ADDIP
// 3.1.1  Address/Stream Configuration Change Chunk (ASCONF)
//
// Serial Number : 32 bits (unsigned integer)
//
// This value represents a Serial Number for the ASCONF
// Chunk. The valid range of Serial Number is from 0 to
// 4294967295 (2^32 - 1).  Serial Numbers wrap back to 0
// after reaching 4294967295.
//
    pub addip_serial: __u32,
    pub src_out_of_asoc_ok: c_int,
    pub asconf_addr_del_pending: *mut sctp_addr,
    pub new_transport: *mut sctp_transport,
// SCTP AUTH: list of the endpoint shared keys.  These
// keys are provided out of band by the user application
// and can't change during the lifetime of the association
//
    pub endpoint_shared_keys: list_head,
// SCTP AUTH:
// The current generated association shared key (secret)
//
    pub asoc_shared_key: *mut sctp_auth_bytes,
    pub shkey: *mut sctp_shared_key,
// SCTP AUTH: hmac id of the first peer requested algorithm
// that we support.
//
    pub default_hmac_id: __u16,
    pub active_key_id: __u16,
    pub strreset_enable: __u8,
    pub /: *mut *mut __u8 strreset_outstanding; / request param bitmask on the fly,
    pub /: *mut *mut __u32 strreset_outseq; / Update after receiving response,
    pub /: *mut *mut __u32 strreset_inseq; / Update after receiving request,
    pub /: *mut *mut __u32 strreset_result[2]; / save the results of last 2 responses,
    pub /: *mut *mut *mut sctp_chunk strreset_chunk; / save request chunk,
    pub stats: sctp_priv_assoc_stats,
    pub sent_cnt_removable: c_int,
    pub subscribe: __u16,
    pub 1]: __u64 abandoned_unsent[SCTP_PR_INDEX(MAX) +,
    pub 1]: __u64 abandoned_sent[SCTP_PR_INDEX(MAX) +,
// Security identifiers from incoming (INIT). These are set by
// security_sctp_assoc_request(). These will only be used by
// SCTP TCP type sockets and peeled off connections as they
// cause a new socket to be generated. security_sctp_sk_clone()
// will then plug these into the new socket.
//
    pub secid: u32,
    pub peer_secid: u32,
    pub rcu: rcu_head,
}

// An eyecatcher for determining if we are really looking at an
// association data structure.
//
// Recover the outer association structure.
// These are function signatures for manipulating associations.
extern "C" {
    pub fn sctp_association_free(: *mut sctp_association);
}
extern "C" {
    pub fn sctp_association_put(: *mut sctp_association);
}
extern "C" {
    pub fn sctp_association_hold(: *mut sctp_association);
}
extern "C" {
    pub fn sctp_assoc_update_retran_path(: *mut sctp_association);
}
extern "C" {
    pub fn sctp_assoc_migrate(: *mut sctp_association, : *mut sock);
}
extern "C" {
    pub fn sctp_association_get_next_tsn(: *mut sctp_association) -> __u32;
}
extern "C" {
    pub fn sctp_assoc_update_frag_point(asoc: *mut sctp_association);
}
extern "C" {
    pub fn sctp_assoc_set_pmtu(asoc: *mut sctp_association, pmtu: __u32);
}
extern "C" {
    pub fn sctp_assoc_sync_pmtu(asoc: *mut sctp_association);
}
extern "C" {
    pub fn sctp_assoc_rwnd_increase(: *mut sctp_association, int: unsigned);
}
extern "C" {
    pub fn sctp_assoc_rwnd_decrease(: *mut sctp_association, int: unsigned);
}
extern "C" {
    pub fn sctp_assoc_set_id(: *mut sctp_association, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn sctp_assoc_clean_asconf_ack_cache(asoc: *const sctp_association);
}
extern "C" {
    pub fn sctp_asconf_queue_teardown(asoc: *mut sctp_association);
}
// A convenience structure to parse out SCTP specific CMSGs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_cmsgs {
    pub init: *mut sctp_initmsg,
    pub srinfo: *mut sctp_sndrcvinfo,
    pub sinfo: *mut sctp_sndinfo,
    pub prinfo: *mut sctp_prinfo,
    pub authinfo: *mut sctp_authinfo,
    pub addrs_msg: *mut msghdr,
}

// Structure for tracking memory objects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_dbg_objcnt_entry {
    pub label: *mut c_char,
    pub counter: *mut core::sync::atomic::AtomicI32,
}
