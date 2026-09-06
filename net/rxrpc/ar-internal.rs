//! Automatically rewritten from C Header to Rust Module
//! Source: net/rxrpc/ar-internal.h
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
// AF_RXRPC internal definitions
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

pub const FCRYPT_ROUNDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcrypt_key {
    pub sched: [__be32; FCRYPT_ROUNDS],
}

pub const FCRYPT_BSIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_crypt {
    pub x: [u8; FCRYPT_BSIZE],
    pub n: [__be32; 2],
}

extern "C" {
    pub fn fcrypt_preparekey(key: *mut fcrypt_key, raw_key[FCRYPT_BSIZE]: u8);
}

//
// Mark applied to socket buffers in skb->mark.  skb->priority is used
// to pass supplementary information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_skb_mark {
    RXRPC_SKB_MARK_PACKET,		/* Received packet */
    RXRPC_SKB_MARK_ERROR,		/* Error notification */
    RXRPC_SKB_MARK_CHALLENGE,	/* Challenge notification */
    RXRPC_SKB_MARK_SERVICE_CONN_SECURED, /* Service connection response has been verified */
    RXRPC_SKB_MARK_REJECT_BUSY,	/* Reject with BUSY */
    RXRPC_SKB_MARK_REJECT_ABORT,	/* Reject with ABORT (code in skb->priority) */
    RXRPC_SKB_MARK_REJECT_CONN_ABORT, /* Reject with connection ABORT (code in skb->priority) */
}

//
// sk_state for RxRPC sockets
//
// Per-network namespace data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_net {
    pub /: *mut *mut *mut proc_dir_entry proc_net; / Subdir in /proc/net,
    pub /: *mut *mut u32 epoch; / Local epoch for detecting local-end reset,
    pub /: *mut *mut list_head calls; / List of calls active in this namespace,
    pub /: *mut *mut spinlock_t call_lock; / Lock for ->calls,
    pub /: *mut *mut atomic_t nr_calls; / Count of allocated calls,
    pub nr_conns: core::sync::atomic::AtomicI32,
    pub /: *mut *mut list_head bundle_proc_list; / List of bundles for proc,
    pub /: *mut *mut list_head conn_proc_list; / List of conns in this namespace for proc,
    pub /: *mut *mut list_head service_conns; / Service conns in this namespace,
    pub /: *mut *mut rwlock_t conn_lock; / Lock for ->conn_proc_list, ->service_conns,
    pub service_conn_reaper: work_struct,
    pub service_conn_reap_timer: timer_list,
    pub live: bool,
    pub nr_client_conns: core::sync::atomic::AtomicI32,
    pub local_endpoints: hlist_head,
    pub /: *mut *mut mutex local_mutex; / Lock for ->local_endpoints,
    pub 10): DECLARE_HASHTABLE (peer_hash,,
    pub /: *mut *mut spinlock_t peer_hash_lock; / Lock for ->peer_hash,

    pub peer_keepalive_cursor: u8,
    pub peer_keepalive_base: time64_t,
    pub peer_keepalive: [list_head; 32],
    pub peer_keepalive_new: list_head,
    pub peer_keepalive_timer: timer_list,
    pub peer_keepalive_work: work_struct,
    pub stat_tx_data: core::sync::atomic::AtomicI32,
    pub stat_tx_data_retrans: core::sync::atomic::AtomicI32,
    pub stat_tx_data_send: core::sync::atomic::AtomicI32,
    pub stat_tx_data_send_frag: core::sync::atomic::AtomicI32,
    pub stat_tx_data_send_fail: core::sync::atomic::AtomicI32,
    pub stat_tx_data_send_msgsize: core::sync::atomic::AtomicI32,
    pub stat_tx_data_underflow: core::sync::atomic::AtomicI32,
    pub stat_tx_data_cwnd_reset: core::sync::atomic::AtomicI32,
    pub stat_rx_data: core::sync::atomic::AtomicI32,
    pub stat_rx_data_reqack: core::sync::atomic::AtomicI32,
    pub stat_rx_data_jumbo: core::sync::atomic::AtomicI32,
    pub stat_tx_ack_fill: core::sync::atomic::AtomicI32,
    pub stat_tx_ack_send: core::sync::atomic::AtomicI32,
    pub stat_tx_ack_skip: core::sync::atomic::AtomicI32,
    pub stat_tx_acks: [core::sync::atomic::AtomicI32; 256],
    pub stat_rx_acks: [core::sync::atomic::AtomicI32; 256],
    pub stat_tx_jumbo: [core::sync::atomic::AtomicI32; 10],
    pub stat_rx_jumbo: [core::sync::atomic::AtomicI32; 10],
    pub stat_why_req_ack: [core::sync::atomic::AtomicI32; 9],
    pub stat_io_loop: core::sync::atomic::AtomicI32,
}

//
// Service backlog preallocation.
//
// This contains circular buffers of preallocated peers, connections and calls
// for incoming service calls and their head and tail pointers.  This allows
// calls to be set up in the data_ready handler, thereby avoiding the need to
// shuffle packets around so much.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_backlog {
    pub peer_backlog_head: c_ushort,
    pub peer_backlog_tail: c_ushort,
    pub conn_backlog_head: c_ushort,
    pub conn_backlog_tail: c_ushort,
    pub call_backlog_head: c_ushort,
    pub call_backlog_tail: c_ushort,
pub const RXRPC_BACKLOG_MAX: c_int = 32;
    pub peer_backlog: [*mut rxrpc_peer; RXRPC_BACKLOG_MAX],
    pub conn_backlog: [*mut rxrpc_connection; RXRPC_BACKLOG_MAX],
    pub call_backlog: [*mut rxrpc_call; RXRPC_BACKLOG_MAX],
}

//
// RxRPC socket definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_sock {
// WARNING: sk has to be the first member
    pub sk: sock,
    pub /: *const *const *const rxrpc_kernel_ops app_ops; / Table of kernel app notification funcs,
    pub /: *mut *mut *mut rxrpc_local local; / local endpoint,
    pub /: *mut *mut *mut rxrpc_backlog backlog; / Preallocation for services,
    pub /: *mut *mut sk_buff_head recvmsg_oobq; / OOB messages for recvmsg to pick up,
    pub /: *mut *mut rb_root pending_oobq; / OOB messages awaiting userspace to respond to,
    pub /: *mut *mut u64 oob_id_counter; / OOB message ID counter,
    pub /: *mut *mut spinlock_t incoming_lock; / Incoming call vs service shutdown lock,
    pub /: *mut *mut list_head sock_calls; / List of calls owned by this socket,
    pub /: *mut *mut list_head to_be_accepted; / calls awaiting acceptance,
    pub /: *mut *mut list_head recvmsg_q; / Calls awaiting recvmsg's attention,
    pub /: *mut *mut spinlock_t recvmsg_lock; / Lock for recvmsg_q,
    pub /: *mut *mut *mut key key; / security for this socket,
    pub /: *mut *mut *mut key securities; / list of server security descriptors,
    pub /: *mut *mut rb_root calls; / User ID -> call mapping,
    pub flags: c_ulong,

    pub /: *mut *mut rwlock_t call_lock; / lock for calls,
    pub /: *mut *mut u32 min_sec_level; / minimum security level,

    pub /: *mut *mut bool exclusive; / Exclusive connection for a client socket,
    pub /: *mut *mut u16 second_service; / Additional service bound to the endpoint,
// Service upgrade information
    pub /: *mut *mut u16 from; / Service ID to upgrade (if not 0),
    pub /: *mut *mut u16 to; / service ID to upgrade to,
    pub service_upgrade: },
    pub /: *mut *mut sa_family_t family; / Protocol family created with,
    pub /: *mut *mut sockaddr_rxrpc srx; / Primary Service/local addresses,
    pub /: *mut *mut sockaddr_rxrpc connect_srx; / Default client address from connect(),
}

//
// CPU-byteorder normalised Rx packet header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_host_header {
    pub /: *mut *mut u32 epoch; / client boot timestamp,
    pub /: *mut *mut u32 cid; / connection and channel ID,
    pub /: *mut *mut u32 callNumber; / call ID (0 for connection-level packets),
    pub /: *mut *mut u32 seq; / sequence number of pkt in call stream,
    pub /: *mut *mut u32 serial; / serial number of pkt sent to network,
    pub /: *mut *mut u8 type; / packet type,
    pub /: *mut *mut u8 flags; / packet flags,
    pub /: *mut *mut u8 userStatus; / app-layer defined status,
    pub /: *mut *mut u8 securityIndex; / security protocol ID,
    pub /: *mut *mut u16 _rsvd; / reserved,
    pub /: *mut *mut u16 cksum; / kerberos security checksum,
}

//
// RxRPC socket buffer private variables
// - max 48 bytes (struct sk_buff::cb)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_skb_priv {
    pub /: *mut *mut *mut rxrpc_connection poke_conn; / Conn referred to (poke packet),
    pub /: *mut *mut u16 offset; / Offset of data,
    pub /: *mut *mut u16 len; / Length of data,
}

//
// RxRPC security module interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_security {
    pub /: *const *const *const char name; / name of this service,
    pub /: *mut *mut u8 security_index; / security type provided,
    pub /: *mut *mut u32 no_key_abort; / Abort code indicating no key,
// Initialise a security service
    pub (*init)(void): *mut c_int,
// Clean up a security service
    pub (*exit)(void): *mut c_void,
// Parse the information from a server key
    pub ): *mut *mut int (preparse_server_key)(struct key_preparsed_payload,
// Clean up the preparse buffer after parsing a server key
    pub ): *mut *mut void (free_preparse_server_key)(struct key_preparsed_payload,
// Destroy the payload of a server key
    pub ): *mut *mut void (destroy_server_key)(struct key,
// Describe a server key
    pub ): *const *const *const void (describe_server_key)(struct key , struct seq_file,
// initialise a connection's security
    pub ): *mut rxrpc_key_token,
// Work out how much data we can store in a packet, given an estimate
// of the amount of data remaining and allocate a data buffer.
//
    pub gfp): *mut *mut *mut *mut rxrpc_txbuf (alloc_txbuf)(rxrpc_call call, size_t remaining, gfp_t,
// impose security on a packet
    pub ): *mut *mut *mut int (secure_packet)(struct rxrpc_call , struct rxrpc_txbuf,
// verify the security on a received packet
    pub ): *mut *mut *mut int (verify_packet)(struct rxrpc_call , struct sk_buff,
// Free crypto request on a call
    pub ): *mut *mut void (free_call_crypto)(struct rxrpc_call,
// issue a challenge
    pub ): *mut *mut int (issue_challenge)(struct rxrpc_connection,
// Validate a challenge packet
    pub skb): *mut sk_buff,
// Fill out the cmsg for recvmsg() to pass on a challenge to userspace.
// The security class gets to add additional information.
//
    pub msg): *mut msghdr,
// Parse sendmsg() control message and respond to challenge.
    pub msg): *mut msghdr,
// respond to a challenge
    pub challenge): *mut sk_buff,
// verify a response
    pub len): *mut *mut void response, unsigned int,
// clear connection security
    pub ): *mut *mut void (clear)(struct rxrpc_connection,
// Default ticket -> key decoder
    pub _key): *mut key,
}

//
// RxRPC local transport endpoint description
// - owned by a single AF_RXRPC socket
// - pointed to by transport socket struct sk_user_data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_local {
    pub rcu: rcu_head,
    pub /: *mut *mut atomic_t active_users; / Number of users of the local endpoint,
    pub /: *mut *mut refcount_t ref; / Number of references to the structure,
    pub /: *mut *mut *mut net net; / The network namespace,
    pub /: *mut *mut *mut rxrpc_net rxnet; / Our bits in the network namespace,
    pub link: hlist_node,
    pub /: *mut *mut *mut socket socket; / my UDP socket,
    pub io_thread: *mut task_struct,
    pub /: *mut *mut completion io_thread_ready; / Indication that the I/O thread started,
    pub /: *mut *mut page_frag_cache tx_alloc; / Tx control packet allocation (I/O thread only),
    pub /: *mut *mut *mut rxrpc_sock service; / Service(s) listening on this endpoint,

    pub /: *mut *mut sk_buff_head rx_delay_queue; / Delay injection queue,

    pub /: *mut *mut sk_buff_head rx_queue; / Received packets,
    pub /: *mut *mut list_head conn_attend_q; / Conns requiring immediate attention,
    pub /: *mut *mut list_head call_attend_q; / Calls requiring immediate attention,
    pub /: *mut *mut rb_root client_bundles; / Client connection bundles by socket params,
    pub /: *mut *mut spinlock_t client_bundles_lock; / Lock for client_bundles,
    pub kill_all_client_conns: bool,
    pub idle_client_conns: list_head,
    pub client_conn_reap_timer: timer_list,
    pub client_conn_flags: c_ulong,

    pub /: *mut *mut spinlock_t lock; / access lock,
    pub /: *mut *mut rwlock_t services_lock; / lock for services list,
    pub /: *mut *mut int debug_id; / debug ID for printks,
    pub dead: bool,
    pub /: *mut *mut bool service_closed; / Service socket closed,
    pub /: *mut *mut idr conn_ids; / List of connection IDs,
    pub /: *mut *mut list_head new_client_calls; / Newly created client calls need connection,
    pub /: *mut *mut spinlock_t client_call_lock; / Lock for ->new_client_calls,
    pub /: *mut *mut sockaddr_rxrpc srx; / local address,
// Provide a kvec table sufficiently large to manage either a
// DATA packet with a maximum set of jumbo subpackets or a PING
// ACK padded out to 64K with zeropages for PMTUD.
//
    pub 16]: 1 + RXRPC_MAX_NR_JUMBO : 3 +,
    pub 16]: bio_vec bvec[3 +,
}

//
// RxRPC remote transport endpoint definition
// - matched by local endpoint, remote port, address and protocol type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_peer {
    pub /: *mut *mut rcu_head rcu; / This must be first,
    pub ref: refcount_t,
    pub hash_key: c_ulong,
    pub hash_link: hlist_node,
    pub local: *mut rxrpc_local,
    pub /: *mut *mut hlist_head error_targets; / targets for net error distribution,
    pub /: *mut *mut rb_root service_conns; / Service connections,
    pub /: *mut *mut list_head keepalive_link; / Link in net->peer_keepalive[],
    pub /: *mut *mut unsigned long app_data; / Application data (e.g. afs_server),
    pub /: *mut *mut unsigned int last_tx_at; / Last time packet sent here (time64_t LSW),
    pub service_conn_lock: seqlock_t,
    pub /: *mut *mut spinlock_t lock; / access lock,
    pub /: *mut *mut int debug_id; / debug ID for printks,
    pub /: *mut *mut sockaddr_rxrpc srx; / remote address,
// Path MTU discovery [RFC8899]
    pub /: *mut *mut unsigned int pmtud_trial; / Current MTU probe size,
    pub /: *mut *mut unsigned int pmtud_good; / Largest working MTU probe we've tried,
    pub /: *mut *mut unsigned int pmtud_bad; / Smallest non-working MTU probe we've tried,
    pub /: *mut *mut bool pmtud_lost; / T if MTU probe was lost,
    pub /: *mut *mut bool pmtud_probing; / T if we have an active probe outstanding,
    pub /: *mut *mut bool pmtud_pending; / T if a call to this peer should send a probe,
    pub /: *mut *mut u8 pmtud_jumbo; / Max jumbo packets for the MTU,
    pub /: *mut *mut bool ackr_adv_pmtud; / T if the peer advertises path-MTU,
    pub /: *mut *mut unsigned int ackr_max_data; / Maximum data advertised by peer,
    pub /: *mut *mut unsigned int if_mtu; / Local interface MTU (- hdrsize) for this peer,
    pub /: *mut *mut unsigned int max_data; / Maximum packet data capacity for this peer,
    pub /: *mut *mut unsigned short hdrsize; / header size (IP + UDP + RxRPC),
    pub /: *mut *mut unsigned short tx_seg_max; / Maximum number of transmissable segments,
// Calculated RTT cache
    pub recent_srtt_us: c_uint,
    pub recent_rto_us: c_uint,
    pub /: *mut *mut u8 cong_ssthresh; / Congestion slow-start threshold,
}

//
// Keys for matching a connection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_conn_proto {
    pub /: *mut *mut u32 epoch; / epoch of this connection,
    pub /: *mut *mut u32 cid; / connection ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_conn_parameters {
    pub /: *mut *mut *mut rxrpc_local local; / Representation of local endpoint,
    pub /: *mut *mut *mut rxrpc_peer peer; / Representation of remote endpoint,
    pub /: *mut *mut *mut key key; / Security details,
    pub /: *mut *mut bool exclusive; / T if conn is exclusive,
    pub /: *mut *mut bool upgrade; / T if service ID can be upgraded,
    pub /: *mut *mut u16 service_id; / Service ID for this connection,
    pub /: *mut *mut u32 security_level; / Security level selected,
}

//
// Call completion condition (state == RXRPC_CALL_COMPLETE).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_call_completion {
    RXRPC_CALL_SUCCEEDED,		/* - Normal termination */
    RXRPC_CALL_REMOTELY_ABORTED,	/* - call aborted by peer */
    RXRPC_CALL_LOCALLY_ABORTED,	/* - call aborted locally on error or close */
    RXRPC_CALL_LOCAL_ERROR,		/* - call failed due to local error */
    RXRPC_CALL_NETWORK_ERROR,	/* - call terminated by network error */
    NR__RXRPC_CALL_COMPLETIONS
}

//
// Bits in the connection flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_conn_flag {
    RXRPC_CONN_IN_SERVICE_CONNS,	/* Conn is in peer->service_conns */
    RXRPC_CONN_DONT_REUSE,		/* Don't reuse this connection */
    RXRPC_CONN_PROBING_FOR_UPGRADE,	/* Probing for service upgrade */
    RXRPC_CONN_FINAL_ACK_0,		/* Need final ACK for channel 0 */
    RXRPC_CONN_FINAL_ACK_1,		/* Need final ACK for channel 1 */
    RXRPC_CONN_FINAL_ACK_2,		/* Need final ACK for channel 2 */
    RXRPC_CONN_FINAL_ACK_3,		/* Need final ACK for channel 3 */
}

//
// Events that can be raised upon a connection.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_conn_event {
    RXRPC_CONN_EV_CHALLENGE,	/* Send challenge packet */
    RXRPC_CONN_EV_ABORT_CALLS,	/* Abort attached calls */
}

//
// The connection protocol state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_conn_proto_state {
    RXRPC_CONN_UNUSED,		/* Connection not yet attempted */
    RXRPC_CONN_CLIENT_UNSECURED,	/* Client connection needs security init */
    RXRPC_CONN_CLIENT,		/* Client connection */
    RXRPC_CONN_SERVICE_PREALLOC,	/* Service connection preallocation */
    RXRPC_CONN_SERVICE_UNSECURED,	/* Service unsecured connection */
    RXRPC_CONN_SERVICE_CHALLENGING,	/* Service challenging for security */
    RXRPC_CONN_SERVICE,		/* Service secured connection */
    RXRPC_CONN_ABORTED,		/* Conn aborted */
    RXRPC_CONN__NR_STATES
}

//
// RxRPC client connection bundle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_bundle {
    pub /: *mut *mut *mut rxrpc_local local; / Representation of local endpoint,
    pub /: *mut *mut *mut rxrpc_peer peer; / Remote endpoint,
    pub /: *mut *mut *mut key key; / Security details,
    pub /: *mut *mut list_head proc_link; / Link in net->bundle_proc_list,
    pub /: *const *const *const rxrpc_security security; / applied security module,
    pub ref: refcount_t,
    pub /: *mut *mut atomic_t active; / Number of active users,
    pub debug_id: c_uint,
    pub /: *mut *mut u32 security_level; / Security level selected,
    pub /: *mut *mut u16 service_id; / Service ID for this connection,
    pub /: *mut *mut bool try_upgrade; / True if the bundle is attempting upgrade,
    pub /: *mut *mut bool exclusive; / T if conn is exclusive,
    pub /: *mut *mut bool upgrade; / T if service ID can be upgraded,
    pub /: *mut *mut unsigned short alloc_error; / Error from last conn allocation,
    pub /: *mut *mut rb_node local_node; / Node in local->client_conns,
    pub /: *mut *mut list_head waiting_calls; / Calls waiting for channels,
    pub /: *mut *mut unsigned long avail_chans; / Mask of available channels,
    pub /: *mut *mut unsigned int conn_ids[4]; / Connection IDs.,
    pub /: *mut *mut *mut rxrpc_connection conns[4]; / The connections in the bundle (max 4),
}

//
// RxRPC connection definition
// - matched by { local, peer, epoch, conn_id, direction }
// - each connection can only handle four simultaneous calls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_connection {
    pub proto: rxrpc_conn_proto,
    pub /: *mut *mut *mut rxrpc_local local; / Representation of local endpoint,
    pub /: *mut *mut *mut rxrpc_peer peer; / Remote endpoint,
    pub /: *mut *mut *mut rxrpc_net rxnet; / Network namespace to which call belongs,
    pub /: *mut *mut *mut key key; / Security details,
    pub /: *mut *mut list_head attend_link; / Link in local->conn_attend_q,
    pub ref: refcount_t,
    pub /: *mut *mut atomic_t active; / Active count for service conns,
    pub rcu: rcu_head,
    pub cache_link: list_head,
    pub /: *mut *mut unsigned char act_chans; / Mask of active channels,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_channel {
    pub /: *mut *mut unsigned long final_ack_at; / Time at which to issue final ACK,
    pub /: *mut *mut *mut rxrpc_call call; / Active call,
    pub /: *mut *mut unsigned int call_debug_id; / call->debug_id,
    pub /: *mut *mut u32 call_id; / ID of current call,
    pub /: *mut *mut u32 call_counter; / Call ID counter,
    pub /: *mut *mut u32 last_call; / ID of last call,
    pub /: *mut *mut u8 last_type; / Type of last packet,
    pub last_seq: u32,
    pub last_abort: u32,
}

//
// Flags in call->flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_call_flag {
    RXRPC_CALL_RELEASED,		/* call has been released - no more message to userspace */
    RXRPC_CALL_HAS_USERID,		/* has a user ID attached */
    RXRPC_CALL_IS_SERVICE,		/* Call is service call */
    RXRPC_CALL_EXPOSED,		/* The call was exposed to the world */
    RXRPC_CALL_RX_LAST,		/* Received the last packet (at rxtx_top) */
    RXRPC_CALL_TX_LAST,		/* Last packet in Tx buffer (at rxtx_top) */
    RXRPC_CALL_TX_ALL_ACKED,	/* Last packet has been hard-acked */
    RXRPC_CALL_TX_NO_MORE,		/* No more data to transmit (MSG_MORE deasserted) */
    RXRPC_CALL_SEND_PING,		/* A ping will need to be sent */
    RXRPC_CALL_RETRANS_TIMEOUT,	/* Retransmission due to timeout occurred */
    RXRPC_CALL_BEGAN_RX_TIMER,	/* We began the expect_rx_by timer */
    RXRPC_CALL_RX_HEARD,		/* The peer responded at least once to this call */
    RXRPC_CALL_DISCONNECTED,	/* The call has been disconnected */
    RXRPC_CALL_KERNEL,		/* The call was made by the kernel */
    RXRPC_CALL_UPGRADE,		/* Service upgrade was requested for the call */
    RXRPC_CALL_EXCLUSIVE,		/* The call uses a once-only connection */
    RXRPC_CALL_RX_IS_IDLE,		/* recvmsg() is idle - send an ACK */
    RXRPC_CALL_RECVMSG_READ_ALL,	/* recvmsg() read all of the received data */
    RXRPC_CALL_CONN_CHALLENGING,	/* The connection is being challenged */
}

//
// Events that can be raised on a call.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_call_event {
    RXRPC_CALL_EV_ACK_LOST,		/* ACK may be lost, send ping */
    RXRPC_CALL_EV_INITIAL_PING,	/* Send initial ping for a new service call */
}

//
// The states that a call can be in.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_call_state {
    RXRPC_CALL_UNINITIALISED,
    RXRPC_CALL_CLIENT_AWAIT_CONN,	/* - client waiting for connection to become available */
    RXRPC_CALL_CLIENT_PRE_SEND,	/* - client is connected, but hasn't sent anything yet */
    RXRPC_CALL_CLIENT_SEND_REQUEST,	/* - client sending request phase */
    RXRPC_CALL_CLIENT_AWAIT_ACK,	/* - client awaiting ACKs of request */
    RXRPC_CALL_CLIENT_AWAIT_REPLY,	/* - client awaiting reply */
    RXRPC_CALL_CLIENT_RECV_REPLY,	/* - client receiving reply phase */
    RXRPC_CALL_SERVER_PREALLOC,	/* - service preallocation */
    RXRPC_CALL_SERVER_RECV_REQUEST,	/* - server receiving request */
    RXRPC_CALL_SERVER_ACK_REQUEST,	/* - server pending ACK of request */
    RXRPC_CALL_SERVER_SEND_REPLY,	/* - server sending reply */
    RXRPC_CALL_SERVER_AWAIT_ACK,	/* - server awaiting final ACK */
    RXRPC_CALL_COMPLETE,		/* - call complete */
    NR__RXRPC_CALL_STATES
}

//
// Call Tx congestion management modes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_ca_state {
    RXRPC_CA_SLOW_START,
    RXRPC_CA_CONGEST_AVOIDANCE,
    RXRPC_CA_PACKET_LOSS,
    RXRPC_CA_FAST_RETRANSMIT,
    NR__RXRPC_CA_STATES
    } __mode(byte);

//
// Current purpose of call RACK timer.  According to the RACK-TLP protocol
// [RFC8985], the transmission timer (call->rack_timo_at) may only be used for
// one of these at once.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_rack_timer_mode {
    RXRPC_CALL_RACKTIMER_OFF,		/* Timer not running */
    RXRPC_CALL_RACKTIMER_RACK_REORDER,	/* RACK reordering timer */
    RXRPC_CALL_RACKTIMER_TLP_PTO,		/* TLP timeout */
    RXRPC_CALL_RACKTIMER_RTO,		/* Retransmission timeout */
    } __mode(byte);

//
// RxRPC call definition
// - matched by { connection, call_id }
//
    struct rxrpc_call {
    struct rcu_head		rcu;
    struct rxrpc_connection	*conn;		/* connection carrying call */
    struct rxrpc_bundle	*bundle;	/* Connection bundle to use */
    struct rxrpc_peer	*peer;		/* Peer record for remote address */
    struct rxrpc_local	*local;		/* Representation of local endpoint */
    struct rxrpc_sock __rcu	*socket;	/* socket responsible */
    struct rxrpc_net	*rxnet;		/* Network namespace to which call belongs */
    struct key		*key;		/* Security details */
    const struct rxrpc_security *security;	/* applied security module */
    struct mutex		user_mutex;	/* User access mutex */
    struct sockaddr_rxrpc	dest_srx;	/* Destination address */
    ktime_t			delay_ack_at;	/* When DELAY ACK needs to happen */
    ktime_t			rack_timo_at;	/* When ACK is figured as lost */
    ktime_t			ping_at;	/* When next to send a ping */
    ktime_t			keepalive_at;	/* When next to send a keepalive ping */
    ktime_t			expect_rx_by;	/* When we expect to get a packet by */
    ktime_t			expect_req_by;	/* When we expect to get a request DATA packet by */
    ktime_t			expect_term_by;	/* When we expect call termination by */
    u32			next_rx_timo;	/* Timeout for next Rx packet (ms) */
    u32			next_req_timo;	/* Timeout for next Rx request packet (ms) */
    u32			hard_timo;	/* Maximum lifetime or 0 (s) */
    struct timer_list	timer;		/* Combined event timer */
    struct work_struct	destroyer;	/* In-process-context destroyer */
    rxrpc_notify_rx_t	notify_rx;	/* kernel service Rx notification function */
    struct list_head	link;		/* link in master call list */
    struct list_head	wait_link;	/* Link in local->new_client_calls */
    struct hlist_node	error_link;	/* link in error distribution list */
    struct list_head	accept_link;	/* Link in rx->acceptq */
    struct list_head	recvmsg_link;	/* Link in rx->recvmsg_q */
    struct list_head	sock_link;	/* Link in rx->sock_calls */
    struct rb_node		sock_node;	/* Node in rx->calls */
    struct list_head	attend_link;	/* Link in local->call_attend_q */
    struct rxrpc_txbuf	*tx_pending;	/* Tx buffer being filled */
    wait_queue_head_t	waitq;		/* Wait queue for channel or Tx */
    s64			tx_total_len;	/* Total length left to be transmitted (or -1) */
    unsigned long		user_call_ID;	/* user-defined call ID */
    unsigned long		flags;
    unsigned long		events;
    spinlock_t		notify_lock;	/* Kernel notification lock */
    unsigned int		send_abort_why; /* Why the abort [enum rxrpc_abort_reason] */
    s32			send_abort;	/* Abort code to be sent */
    short			send_abort_err;	/* Error to be associated with the abort */
    rxrpc_seq_t		send_abort_seq;	/* DATA packet that incurred the abort (or 0) */
    s32			abort_code;	/* Local/remote abort code */
    int			error;		/* Local error incurred */
    enum rxrpc_call_state	_state;		/* Current state of call (needs barrier) */
    enum rxrpc_call_completion completion;	/* Call completion condition */
    refcount_t		ref;
    u8			security_ix;	/* Security type */
    enum rxrpc_interruptibility interruptibility; /* At what point call may be interrupted */
    u32			call_id;	/* call ID on connection  */
    u32			cid;		/* connection ID plus channel index */
    u32			security_level;	/* Security level selected */
    u32			security_enctype; /* Security-specific encoding type (or 0) */
    int			debug_id;	/* debug ID for printks */
    unsigned short		rx_pkt_offset;	/* Current recvmsg packet offset */
    unsigned short		rx_pkt_len;	/* Current recvmsg packet len */

// Sendmsg data tracking.
    rxrpc_seq_t		send_top;	/* Highest Tx slot filled by sendmsg. */
    struct rxrpc_txqueue	*send_queue;	/* Queue that sendmsg is writing into */

// Transmitted data tracking.
    struct rxrpc_txqueue	*tx_queue;	/* Start of transmission buffers */
    struct rxrpc_txqueue	*tx_qtail;	/* End of transmission buffers */
    rxrpc_seq_t		tx_qbase;	/* First slot in tx_queue */
    rxrpc_seq_t		tx_bottom;	/* First packet in buffer */
    rxrpc_seq_t		tx_transmitted;	/* Highest packet transmitted */
    rxrpc_seq_t		tx_top;		/* Highest Tx slot allocated. */
    rxrpc_serial_t		tx_last_serial;	/* Serial of last DATA transmitted */
    u16			tx_backoff;	/* Delay to insert due to Tx failure (ms) */
    u16			tx_nr_sent;	/* Number of packets sent, but unacked */
    u16			tx_nr_lost;	/* Number of packets marked lost */
    u16			tx_nr_resent;	/* Number of packets resent, but unacked */
    u16			tx_winsize;	/* Maximum size of Tx window */
pub const RXRPC_TX_MAX_WINDOW: c_int = 128;
    u8			tx_jumbo_max;	/* Maximum subpkts peer will accept */
    ktime_t			tx_last_sent;	/* Last time a transmission occurred */

// Received data tracking
    struct sk_buff_head	recvmsg_queue;	/* Queue of packets ready for recvmsg() */
    struct sk_buff_head	rx_queue;	/* Queue of packets for this call to receive */
    struct sk_buff_head	rx_oos_queue;	/* Queue of out of sequence packets */
    void			*rx_dec_buffer;	/* Decryption buffer */
    unsigned short		rx_dec_bsize;	/* rx_dec_buffer size */
    unsigned short		rx_dec_offset;	/* Decrypted packet data offset */
    unsigned short		rx_dec_len;	/* Decrypted packet data len */
    rxrpc_seq_t		rx_dec_seq;	/* Packet in decryption buffer */

    rxrpc_seq_t		rx_highest_seq;	/* Higest sequence number received */
    rxrpc_seq_t		rx_consumed;	/* Highest packet consumed */
    rxrpc_serial_t		rx_serial;	/* Highest serial received for this call */
    u8			rx_winsize;	/* Size of Rx window */

// TCP-style slow-start congestion control [RFC5681].  Since the SMSS
// is fixed, we keep these numbers in terms of segments (ie. DATA
// packets) rather than bytes.
//

pub const RXRPC_MIN_CWND: c_int = 4;
    enum rxrpc_ca_state	cong_ca_state;	/* Congestion control state */
    u8			cong_extra;	/* Extra to send for congestion management */
    u16			cong_cwnd;	/* Congestion window size */
    u16			cong_ssthresh;	/* Slow-start threshold */
    u16			cong_dup_acks;	/* Count of ACKs showing missing packets */
    u16			cong_cumul_acks; /* Cumulative ACK count */
    ktime_t			cong_tstamp;	/* Last time cwnd was changed */

// RACK-TLP [RFC8985] state.
    ktime_t			rack_xmit_ts;	/* Latest transmission timestamp */
    ktime_t			rack_rtt;	/* RTT of most recently ACK'd segment */
    ktime_t			rack_rtt_ts;	/* Timestamp of rack_rtt */
    ktime_t			rack_reo_wnd;	/* Reordering window */
    unsigned int		rack_reo_wnd_mult; /* Multiplier applied to rack_reo_wnd */
    int			rack_reo_wnd_persist; /* Num loss recoveries before reset reo_wnd */
    rxrpc_seq_t		rack_fack;	/* Highest sequence so far ACK'd */
    rxrpc_seq_t		rack_end_seq;	/* Highest sequence seen */
    rxrpc_seq_t		rack_dsack_round; /* DSACK opt recv'd in latest roundtrip */
    bool			rack_dsack_round_none; /* T if dsack_round is "None" */
    bool			rack_reordering_seen; /* T if detected reordering event */
    enum rxrpc_rack_timer_mode rack_timer_mode; /* Current mode of RACK timer */
    bool			tlp_is_retrans;	/* T if unacked TLP retransmission */
    rxrpc_serial_t		tlp_serial;	/* Serial of TLP probe (or 0 if none in progress) */
    rxrpc_seq_t		tlp_seq;	/* Sequence of TLP probe */
    unsigned int		tlp_rtt_taken;	/* Last time RTT taken */
    ktime_t			tlp_max_ack_delay; /* Sender budget for max delayed ACK interval */

// Receive-phase ACK management (ACKs we send).
    u8			ackr_reason;	/* reason to ACK */
    u16			ackr_sack_base;	/* Starting slot in SACK table ring */
    rxrpc_seq_t		ackr_window;	/* Base of SACK window */
    rxrpc_seq_t		ackr_wtop;	/* Base of SACK window */
    unsigned int		ackr_nr_unacked; /* Number of unacked packets */
    atomic_t		ackr_nr_consumed; /* Number of packets needing hard ACK */
    struct {
pub const RXRPC_SACK_SIZE: c_int = 256;
// SACK table for soft-acked packets
    u8		ackr_sack_table[RXRPC_SACK_SIZE];
    } __aligned(8);

// RTT management
    rxrpc_serial_t		rtt_serial[4];	/* Serial number of DATA or PING sent */
    ktime_t			rtt_sent_at[4];	/* Time packet sent */
    unsigned long		rtt_avail;	/* Mask of available slots in bits 0-3,
// Mask of pending samples in 8-11
pub const RXRPC_CALL_RTT_AVAIL_MASK: c_uint = 0xf;
pub const RXRPC_CALL_RTT_PEND_SHIFT: c_int = 8;

// Transmission-phase ACK management (ACKs we've received).
    ktime_t			acks_latest_ts;	/* Timestamp of latest ACK received */
    rxrpc_seq_t		acks_hard_ack;	/* Highest sequence hard acked */
    rxrpc_seq_t		acks_prev_seq;	/* Highest previousPacket received */
    rxrpc_seq_t		acks_lowest_nak; /* Lowest NACK in the buffer (or ==tx_hard_ack) */
    rxrpc_serial_t		acks_highest_serial; /* Highest serial number ACK'd */
    unsigned short		acks_nr_sacks;	/* Number of soft acks recorded */
    unsigned short		acks_nr_snacks;	/* Number of soft nacks recorded */

// Calculated RTT cache
    ktime_t			rtt_last_req;	/* Time of last RTT request */
    unsigned int		rtt_count;	/* Number of samples we've got */
    unsigned int		rtt_taken;	/* Number of samples taken (wrapping) */
    struct minmax		min_rtt;	/* Estimated minimum RTT */
    u32			srtt_us;	/* smoothed round trip time << 3 in usecs */
    u32			mdev_us;	/* medium deviation			*/
    u32			mdev_max_us;	/* maximal mdev for the last rtt period	*/
    u32			rttvar_us;	/* smoothed mdev_max			*/
    u32			rto_us;		/* Retransmission timeout in usec */
    u8			backoff;	/* Backoff timeout (as shift) */
}

//
// Summary of a new ACK and the changes it made to the Tx buffer packet states.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_ack_summary {
    pub /: *mut *mut rxrpc_serial_t ack_serial; / Serial number of ACK,
    pub /: *mut *mut rxrpc_serial_t acked_serial; / Serial number ACK'd,
    pub /: *mut *mut u16 in_flight; / Number of unreceived transmissions,
    pub /: *mut *mut u16 nr_new_hacks; / Number of rotated new ACKs,
    pub /: *mut *mut u16 nr_new_sacks; / Number of new soft ACKs in packet,
    pub /: *mut *mut u16 nr_new_snacks; / Number of new soft nacks in packet,
    pub ack_reason: u8,
    pub /: *mut *mut bool new_low_snack:1; / T if new low soft NACK found,
    pub /: *mut *mut bool retrans_timeo:1; / T if reTx due to timeout happened,
    pub /: *mut *mut bool need_retransmit:1; / T if we need transmission,
    pub /: *mut *mut bool rtt_sample_avail:1; / T if RTT sample available,
    pub in_fast_or_rto_recovery:1: bool,
    pub exiting_fast_or_rto_recovery:1: bool,
    pub /: *mut *mut bool tlp_probe_acked:1; / T if the TLP probe seq was acked,
    pub change: *mut *mut *mut u8 /enum rxrpc_congest_change/,
}

//
// sendmsg() cmsg-specified parameters.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_command {
    RXRPC_CMD_SEND_DATA,		/* send data message */
    RXRPC_CMD_SEND_ABORT,		/* request abort generation */
    RXRPC_CMD_REJECT_BUSY,		/* [server] reject a call as busy */
    RXRPC_CMD_CHARGE_ACCEPT,	/* [server] charge accept preallocation */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_call_params {
    pub /: *mut *mut s64 tx_total_len; / Total Tx data length (if send data),
    pub /: *mut *mut unsigned long user_call_ID; / User's call ID,
    pub /: *mut *mut u32 hard; / Maximum lifetime (sec),
    pub /: *mut *mut u32 idle; / Max time since last data packet (msec),
    pub /: *mut *mut u32 normal; / Max time since last call packet (msec),
    pub timeouts: },
    pub /: *mut *mut u8 nr_timeouts; / Number of timeouts specified,
    pub /: *mut *mut bool kernel; / T if kernel is making the call,
    pub /: *mut *mut rxrpc_interruptibility interruptibility; / How is interruptible is the call?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_send_params {
    pub call: rxrpc_call_params,
    pub /: *mut *mut u32 abort_code; / Abort code to Tx (if abort),
    pub /: *mut *mut rxrpc_command command : 8; / The command to implement,
    pub /: *mut *mut bool exclusive; / Shared or exclusive call,
    pub /: *mut *mut bool upgrade; / If the connection is upgradeable,
}

//
// Buffer of data to be output as a packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_txbuf {
    pub ref: refcount_t,
    pub /: *mut *mut rxrpc_seq_t seq; / Sequence number of this packet,
    pub /: *mut *mut rxrpc_serial_t serial; / Last serial number transmitted with,
    pub call_debug_id: c_uint,
    pub debug_id: c_uint,
    pub /: *mut *mut unsigned short len; / Amount of data in buffer,
    pub /: *mut *mut unsigned short space; / Remaining data space,
    pub /: *mut *mut unsigned short offset; / Offset of fill point,
    pub /: *mut *mut unsigned short crypto_header; / Size of crypto header,
    pub /: *mut *mut unsigned short sec_header; / Size of security header,
    pub /: *mut *mut unsigned short pkt_len; / Size of packet content,
    pub /: *mut *mut unsigned short alloc_size; / Amount of bufferage allocated,
    pub flags: c_uint,
pub const RXRPC_TXBUF_WIRE_FLAGS: c_uint = 0xff		/* The wire protocol flags */;
pub const RXRPC_TXBUF_RESENT: c_uint = 0x100		/* Set if has been resent */;
    pub /: *mut *mut __be16 cksum; / Checksum to go in header,
    pub /: *mut *mut bool jumboable; / Can be non-terminal jumbo subpacket,
    pub /: *mut *mut *mut void data; / Data with preceding jumbo header,
}

//
// Transmit queue element, including RACK [RFC8985] per-segment metadata.  The
// transmission timestamp is in usec from the base.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_txqueue {
// Start with the members we want to prefetch.
    pub next: *mut rxrpc_txqueue,
    pub xmit_ts_base: ktime_t,
    pub qbase: rxrpc_seq_t,
    pub /: *mut *mut u8 nr_reported_acks; / Number of segments explicitly acked/nacked,
    pub /: *mut *mut unsigned long segment_acked; / Bit-per-buf: Set if ACK'd,
    pub /: *mut *mut unsigned long segment_lost; / Bit-per-buf: Set if declared lost,
    pub /: *mut *mut unsigned long segment_retransmitted; / Bit-per-buf: Set if retransmitted,
    pub /: *mut *mut unsigned long rtt_samples; / Bit-per-buf: Set if available for RTT,
    pub /: *mut *mut unsigned long ever_retransmitted; / Bit-per-buf: Set if ever retransmitted,
// The arrays we want to pack into as few cache lines as possible.
    pub bufs: [*mut rxrpc_txbuf; RXRPC_NR_TXQUEUE],
    pub segment_serial: [c_uint; RXRPC_NR_TXQUEUE],
    pub segment_xmit_ts: [c_uint; RXRPC_NR_TXQUEUE],
    pub ____cacheline_aligned: },
}

//
// Data transmission request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_send_data_req {
    pub /: *mut *mut ktime_t now; / Current time,
    pub /: *mut *mut *mut rxrpc_txqueue tq; / Tx queue segment holding first DATA,
    pub /: *mut *mut rxrpc_seq_t seq; / Sequence of first data,
    pub /: *mut *mut int n; / Number of DATA packets to glue into jumbo,
    pub /: *mut *mut bool retrans; / T if this is a retransmission,
    pub /: *mut *mut bool did_send; / T if did actually send,
    pub /: *mut *mut bool tlp_probe; / T if this is a TLP probe,
    pub trace: *mut *mut *mut int / enum rxrpc_txdata_trace /,
}

//
// Allocate the next serial number on a connection.  0 must be skipped.
//
// Allocate the next serial n numbers on a connection.  0 must be skipped.
//
// af_rxrpc.c
//
// call_accept.c
//
extern "C" {
    pub fn rxrpc_service_prealloc(: *mut rxrpc_sock, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn rxrpc_discard_prealloc(: *mut rxrpc_sock);
}
extern "C" {
    pub fn rxrpc_user_charge_accept(: *mut rxrpc_sock, long: unsigned) -> c_int;
}
//
// call_event.c
//
extern "C" {
    pub fn rxrpc_resend_tlp(call: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_input_call_event(call: *mut rxrpc_call) -> bool;
}
//
// call_object.c
//
extern "C" {
    pub fn rxrpc_poke_call(call: *mut rxrpc_call, what: rxrpc_call_poke_trace);
}
extern "C" {
    pub fn rxrpc_start_call_timer(call: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_release_call(: *mut rxrpc_sock, : *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_release_calls_on_socket(: *mut rxrpc_sock);
}
extern "C" {
    pub fn rxrpc_see_call(: *mut rxrpc_call, rxrpc_call_trace: enum);
}
extern "C" {
    pub fn rxrpc_get_call(: *mut rxrpc_call, rxrpc_call_trace: enum);
}
extern "C" {
    pub fn rxrpc_put_call(: *mut rxrpc_call, rxrpc_call_trace: enum);
}
extern "C" {
    pub fn rxrpc_cleanup_call(: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_destroy_all_calls(: *mut rxrpc_net);
}
extern "C" {
    pub fn test_bit(_arg: RXRPC_CALL_IS_SERVICE, _arg: &call->flags) -> return;
}
//
// call_state.c
//
extern "C" {
    pub fn rxrpc_call_completed(call: *mut rxrpc_call) -> bool;
}
// Order write of completion info before write of ->state.
// Order read ->state before read of completion info.
extern "C" {
    pub fn smp_load_acquire(_arg: &call->_state) -> return;
}
//
// conn_client.c
//
extern "C" {
    pub fn rxrpc_purge_client_connections(local: *mut rxrpc_local);
}
extern "C" {
    pub fn rxrpc_put_bundle(: *mut rxrpc_bundle, rxrpc_bundle_trace: enum);
}
extern "C" {
    pub fn rxrpc_look_up_bundle(call: *mut rxrpc_call, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn rxrpc_connect_client_calls(local: *mut rxrpc_local);
}
extern "C" {
    pub fn rxrpc_expose_client_call(: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_disconnect_client_call(: *mut rxrpc_bundle, : *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_deactivate_bundle(bundle: *mut rxrpc_bundle);
}
extern "C" {
    pub fn rxrpc_discard_expired_client_conns(local: *mut rxrpc_local);
}
extern "C" {
    pub fn rxrpc_clean_up_local_conns(: *mut rxrpc_local);
}
//
// conn_event.c
//
extern "C" {
    pub fn rxrpc_process_connection(: *mut work_struct);
}
extern "C" {
    pub fn rxrpc_process_delayed_final_acks(: *mut rxrpc_connection, _arg: bool);
}
extern "C" {
    pub fn rxrpc_input_conn_packet(conn: *mut rxrpc_connection, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn rxrpc_input_conn_event(conn: *mut rxrpc_connection, skb: *mut sk_buff);
}
// Order reading the abort info after the state check.
//
// conn_object.c
//
extern "C" {
    pub fn rxrpc_poke_conn(conn: *mut rxrpc_connection, why: rxrpc_conn_trace);
}
extern "C" {
    pub fn __rxrpc_disconnect_call(: *mut rxrpc_connection, : *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_disconnect_call(: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_kill_client_conn(: *mut rxrpc_connection);
}
extern "C" {
    pub fn rxrpc_queue_conn(: *mut rxrpc_connection, rxrpc_conn_trace: enum);
}
extern "C" {
    pub fn rxrpc_see_connection(: *mut rxrpc_connection, rxrpc_conn_trace: enum);
}
extern "C" {
    pub fn rxrpc_put_connection(: *mut rxrpc_connection, rxrpc_conn_trace: enum);
}
extern "C" {
    pub fn rxrpc_service_connection_reaper(: *mut work_struct);
}
extern "C" {
    pub fn rxrpc_destroy_all_connections(: *mut rxrpc_net);
}
//
// conn_service.c
//
extern "C" {
    pub fn rxrpc_unpublish_service_conn(: *mut rxrpc_connection);
}
//
// input.c
//
extern "C" {
    pub fn rxrpc_congestion_degrade(: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_input_call_packet(: *mut rxrpc_call, : *mut sk_buff);
}
extern "C" {
    pub fn rxrpc_implicit_end_call(: *mut rxrpc_call, : *mut sk_buff);
}
//
// input_rack.c
//
extern "C" {
    pub fn rxrpc_tlp_calc_pto(call: *mut rxrpc_call, now: ktime_t) -> ktime_t;
}
extern "C" {
    pub fn rxrpc_tlp_send_probe(call: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_tlp_process_ack(call: *mut rxrpc_call, summary: *mut rxrpc_ack_summary);
}
extern "C" {
    pub fn rxrpc_rack_timer_expired(call: *mut rxrpc_call, overran_by: ktime_t);
}
// Initialise TLP state [RFC8958 7.1].
//
// io_thread.c
//
extern "C" {
    pub fn rxrpc_encap_rcv(: *mut sock, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rxrpc_error_report(: *mut sock);
}
extern "C" {
    pub fn rxrpc_io_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rxrpc_post_response(conn: *mut rxrpc_connection, skb: *mut sk_buff);
}
extern "C" {
    pub fn rxrpc_direct_abort(_arg: skb, _arg: why, _arg: RX_PROTOCOL_ERROR, _arg: -EPROTO) -> return;
}
//
// insecure.c
//
// key.c
//
extern "C" {
    pub fn rxrpc_request_key(: *mut rxrpc_sock, _arg: sockptr_t, _arg: c_int) -> c_int;
}
//
// local_event.c
//
extern "C" {
    pub fn rxrpc_gen_version_string();
}
//
// local_object.c
//
extern "C" {
    pub fn rxrpc_local_dont_fragment(local: *const rxrpc_local, set: bool);
}
extern "C" {
    pub fn rxrpc_put_local(: *mut rxrpc_local, rxrpc_local_trace: enum);
}
extern "C" {
    pub fn rxrpc_unuse_local(: *mut rxrpc_local, rxrpc_local_trace: enum);
}
extern "C" {
    pub fn rxrpc_destroy_local(local: *mut rxrpc_local);
}
extern "C" {
    pub fn rxrpc_destroy_all_locals(: *mut rxrpc_net);
}
//
// misc.c
//

//
// net_ns.c
//
extern "C" {
    pub fn net_generic(_arg: net, _arg: rxrpc_net_id) -> return;
}
//
// oob.c
//
extern "C" {
    pub fn rxrpc_notify_socket_oob(call: *mut rxrpc_call, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn rxrpc_add_pending_oob(rx: *mut rxrpc_sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn rxrpc_sendmsg_oob(rx: *mut rxrpc_sock, msg: *mut msghdr, len: usize) -> c_int;
}
//
// output.c
//
extern "C" {
    pub fn do_udp_sendmsg(socket: *mut socket, msg: *mut msghdr, len: usize) -> isize;
}
extern "C" {
    pub fn rxrpc_send_probe_for_pmtud(call: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_send_abort_packet(: *mut rxrpc_call) -> c_int;
}
extern "C" {
    pub fn rxrpc_send_data_packet(call: *mut rxrpc_call, req: *mut rxrpc_send_data_req);
}
extern "C" {
    pub fn rxrpc_send_conn_abort(conn: *mut rxrpc_connection);
}
extern "C" {
    pub fn rxrpc_reject_packet(local: *mut rxrpc_local, skb: *mut sk_buff);
}
extern "C" {
    pub fn rxrpc_send_keepalive(: *mut rxrpc_peer);
}
extern "C" {
    pub fn rxrpc_send_response(conn: *mut rxrpc_connection, skb: *mut sk_buff);
}
//
// peer_event.c
//
extern "C" {
    pub fn rxrpc_input_error(: *mut rxrpc_local, : *mut sk_buff);
}
extern "C" {
    pub fn rxrpc_peer_keepalive_worker(: *mut work_struct);
}
// Update the last transmission time on a peer for keepalive purposes.
// To avoid tearing on 32-bit systems, we only keep the LSW.
//
// peer_object.c
//
extern "C" {
    pub fn rxrpc_assess_MTU_size(local: *mut rxrpc_local, peer: *mut rxrpc_peer);
}
extern "C" {
    pub fn rxrpc_new_incoming_peer(local: *mut rxrpc_local, peer: *mut rxrpc_peer);
}
extern "C" {
    pub fn rxrpc_destroy_all_peers(: *mut rxrpc_net);
}
extern "C" {
    pub fn rxrpc_put_peer(: *mut rxrpc_peer, rxrpc_peer_trace: enum);
}
//
// proc.c
//
// recvmsg.c
//
extern "C" {
    pub fn rxrpc_notify_socket(: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_recvmsg(: *mut socket, : *mut msghdr, _arg: usize, _arg: c_int) -> c_int;
}
//
// Abort a call due to a protocol error.
//
// rtt.c
//
extern "C" {
    pub fn rxrpc_get_rto_backoff(call: *mut rxrpc_call, retrans: bool) -> ktime_t;
}
extern "C" {
    pub fn rxrpc_call_init_rtt(call: *mut rxrpc_call);
}
//
// rxgk.c
//
// rxkad.c
//

//
// security.c
//
extern "C" {
    pub fn rxrpc_init_security() -> int __init;
}
extern "C" {
    pub fn rxrpc_exit_security();
}
extern "C" {
    pub fn rxrpc_init_client_call_security(: *mut rxrpc_call) -> c_int;
}
extern "C" {
    pub fn rxrpc_init_client_conn_security(: *mut rxrpc_connection) -> c_int;
}
//
// sendmsg.c
//
extern "C" {
    pub fn rxrpc_do_sendmsg(: *mut rxrpc_sock, : *mut msghdr, _arg: usize) -> c_int;
}
//
// server_key.c
//
extern "C" {
    pub fn rxrpc_server_keyring(: *mut rxrpc_sock, _arg: sockptr_t, _arg: c_int) -> c_int;
}
//
// skbuff.c
//
extern "C" {
    pub fn rxrpc_kernel_data_consumed(: *mut rxrpc_call, : *mut sk_buff);
}
extern "C" {
    pub fn rxrpc_new_skb(: *mut sk_buff, rxrpc_skb_trace: enum);
}
extern "C" {
    pub fn rxrpc_see_skb(: *mut sk_buff, rxrpc_skb_trace: enum);
}
extern "C" {
    pub fn rxrpc_get_skb(: *mut sk_buff, rxrpc_skb_trace: enum);
}
extern "C" {
    pub fn rxrpc_free_skb(: *mut sk_buff, rxrpc_skb_trace: enum);
}
extern "C" {
    pub fn rxrpc_purge_queue(: *mut sk_buff_head);
}
//
// stats.c
//
extern "C" {
    pub fn rxrpc_stats_show(seq: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rxrpc_stats_clear(file: *mut file, buf: *mut c_char, size: usize) -> c_int;
}

//
// sysctl.c
//

extern "C" {
    pub fn rxrpc_sysctl_init() -> int __init;
}
extern "C" {
    pub fn rxrpc_sysctl_exit();
}

//
// txbuf.c
//
extern "C" {
    pub fn rxrpc_see_txbuf(txb: *mut rxrpc_txbuf, what: rxrpc_txbuf_trace);
}
extern "C" {
    pub fn rxrpc_put_txbuf(txb: *mut rxrpc_txbuf, what: rxrpc_txbuf_trace);
}
//
// utils.c
//
extern "C" {
    pub fn rxrpc_extract_addr_from_skb(: *mut sockaddr_rxrpc, : *mut sk_buff) -> c_int;
}
//
// Calculate how much space there is for transmitting more DATA packets.
//
extern "C" {
    pub fn max(transmitted: winsize -, _arg: 0) -> return;
}
//
// Calculate the number of transmitted DATA packets assumed to be in flight
// [approx RFC6675].
//
// debug tracing
//

pub const RXRPC_DEBUG_KENTER: c_uint = 0x01;
pub const RXRPC_DEBUG_KLEAVE: c_uint = 0x02;
pub const RXRPC_DEBUG_KDEBUG: c_uint = 0x04;

//
// debug assertion checking
//

