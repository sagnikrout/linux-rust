//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sock.h
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
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the AF_INET socket handler.
//
// Version:	@(#)sock.h	1.0.4	05/13/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Corey Minyard <wf-rch!minyard@relay.EU.net>
// Florian La Roche <flla@stud.uni-sb.de>
//
// Fixes:
// Alan Cox	:	Volatiles in skbuff pointers. See
// skbuff comments. May be overdone,
// better to prove they can be removed
// than the reverse.
// Alan Cox	:	Added a zapped field for tcp to note
// a socket is reset and must stay shut up
// Alan Cox	:	New fields for options
// Pauline Middelink	:	identd support
// Alan Cox	:	Eliminate low level recv/recvfrom
// David S. Miller	:	New socket lookup architecture.
// Steve Whitehouse:       Default routines for sock_ops
// Arnaldo C. Melo :	removed net_pinfo, tp_pinfo and made
// protinfo be just a void pointer, as the
// protocol specific parts were moved to
// respective headers and ipv4/v6, etc now
// use private slabcaches for its socks
// Pedro Hortas	:	New flags field for socket options
//

//
// This structure really needs to be cleaned up.
// Most of it is for TCP, and not used by any of
// the other protocols.
//
// This is the per-socket lock.  The spinlock provides a synchronization
// between user contexts and software interrupt processing, whereas the
// mini-semaphore synchronizes multiple users amongst themselves.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slock_owned {
    pub owned: c_int,
    pub slock: spinlock_t,
}

//
// We express the mutex-alike socket_lock semantics
// to the lock validator by explicitly managing
// the slock as a lock variant (in addition to
// the slock itself):
//

pub type __portpair = __u32 ;
pub type __addrpair = __u64 ;
//
// struct sock_common - minimal network layer representation of sockets
// @skc_daddr: Foreign IPv4 addr
// @skc_rcv_saddr: Bound local IPv4 addr
// @skc_addrpair: 8-byte-aligned __u64 union of @skc_daddr & @skc_rcv_saddr
// @skc_hash: hash value used with various protocol lookup tables
// @skc_u16hashes: two u16 hash values used by UDP lookup tables
// @skc_dport: placeholder for inet_dport/tw_dport
// @skc_num: placeholder for inet_num/tw_num
// @skc_portpair: __u32 union of @skc_dport & @skc_num
// @skc_family: network address family
// @skc_state: Connection state
// @skc_reuse: %SO_REUSEADDR setting
// @skc_reuseport: %SO_REUSEPORT setting
// @skc_ipv6only: socket is IPV6 only
// @skc_net_refcnt: socket is using net ref counting
// @skc_bypass_prot_mem: bypass the per-protocol memory accounting for skb
// @skc_bound_dev_if: bound device index if != 0
// @skc_bind_node: bind hash linkage for various protocol lookup tables
// @skc_portaddr_node: second hash linkage for UDP
// @skc_prot: protocol handlers inside a network family
// @skc_net: reference to the network namespace of this socket
// @skc_v6_daddr: IPV6 destination address
// @skc_v6_rcv_saddr: IPV6 source address
// @skc_cookie: socket's cookie value
// @skc_node: main hash linkage for various protocol lookup tables
// @skc_nulls_node: main hash linkage for TCP
// @skc_tx_queue_mapping: tx queue number for this connection
// @skc_rx_queue_mapping: rx queue number for this connection
// @skc_flags: place holder for sk_flags
// %SO_LINGER (l_onoff), %SO_BROADCAST, %SO_KEEPALIVE,
// %SO_OOBINLINE settings, %SO_TIMESTAMPING settings
// @skc_listener: connection request listener socket (aka rsk_listener)
// [union with @skc_flags]
// @skc_tw_dr: (aka tw_dr) ptr to &struct inet_timewait_death_row
// [union with @skc_flags]
// @skc_incoming_cpu: record/match cpu processing incoming packets
// @skc_rcv_wnd: (aka rsk_rcv_wnd) TCP receive window size (possibly scaled)
// [union with @skc_incoming_cpu]
// @skc_tw_rcv_nxt: (aka tw_rcv_nxt) TCP window next expected seq number
// [union with @skc_incoming_cpu]
// @skc_refcnt: reference count
//
// This is the minimal network layer representation of sockets, the header
// for struct sock and struct inet_timewait_sock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_common {
    pub skc_addrpair: __addrpair,
    pub skc_daddr: __be32,
    pub skc_rcv_saddr: __be32,
}

// skc_dport && skc_num must be grouped as well

// following fields are padding to force
// offset(struct sock, sk_refcnt) == 128 on 64bit arches
// assuming IPV6 is enabled. We use this padding differently
// for different kind of 'sockets'
//
// fields between dontcopy_begin/dontcopy_end
// are not copied in sock_copy()
//
// private:
// public:

// private:
// public:
//
// struct sock - network layer representation of sockets
// @__sk_common: shared layout with inet_timewait_sock
// @sk_shutdown: mask of %SEND_SHUTDOWN and/or %RCV_SHUTDOWN
// @sk_userlocks: %SO_SNDBUF and %SO_RCVBUF settings
// @sk_lock:	synchronizer
// @sk_kern_sock: True if sock is using kernel lock classes
// @sk_rcvbuf: size of receive buffer in bytes
// @sk_wq: sock wait queue and async head
// @sk_rx_dst: receive input route used by early demux
// @sk_rx_dst_ifindex: ifindex for @sk_rx_dst
// @sk_rx_dst_cookie: cookie for @sk_rx_dst
// @sk_dst_cache: destination cache
// @sk_dst_pending_confirm: need to confirm neighbour
// @sk_policy: flow policy
// @psp_assoc: PSP association, if socket is PSP-secured
// @sk_receive_queue: incoming packets
// @sk_wmem_alloc: transmit queue bytes committed
// @sk_tsq_flags: TCP Small Queues flags
// @sk_write_queue: Packet sending queue
// @sk_omem_alloc: "o" is "option" or "other"
// @sk_wmem_queued: persistent queue size
// @sk_forward_alloc: space allocated forward
// @sk_reserved_mem: space reserved and non-reclaimable for the socket
// @sk_napi_id: id of the last napi context to receive data for sk
// @sk_ll_usec: usecs to busypoll when there is no data
// @sk_allocation: allocation mode
// @sk_pacing_rate: Pacing rate (if supported by transport/packet scheduler)
// @sk_pacing_status: Pacing status (requested, handled by sch_fq)
// @sk_max_pacing_rate: Maximum pacing rate (%SO_MAX_PACING_RATE)
// @sk_sndbuf: size of send buffer in bytes
// @sk_no_check_tx: %SO_NO_CHECK setting, set checksum in TX packets
// @sk_no_check_rx: allow zero checksum in RX packets
// @sk_route_caps: route capabilities (e.g. %NETIF_F_TSO)
// @sk_gso_disabled: if set, NETIF_F_GSO_MASK is forbidden.
// @sk_gso_type: GSO type (e.g. %SKB_GSO_TCPV4)
// @sk_gso_max_size: Maximum GSO segment size to build
// @sk_gso_max_segs: Maximum number of GSO segments
// @sk_pacing_shift: scaling factor for TCP Small Queues
// @sk_lingertime: %SO_LINGER l_linger setting
// @sk_backlog: always used with the per-socket spinlock held
// @sk_callback_lock: used with the callbacks in the end of this struct
// @sk_error_queue: rarely used
// @sk_prot_creator: sk_prot of original sock creator (see ipv6_setsockopt,
// IPV6_ADDRFORM for instance)
// @sk_err: last error
// @sk_err_soft: errors that don't cause failure but are the cause of a
// persistent failure not just 'timed out'
// @sk_drops: raw/udp drops counter
// @sk_drop_counters: optional pointer to numa_drop_counters
// @sk_ack_backlog: current listen backlog
// @sk_max_ack_backlog: listen backlog set in listen()
// @sk_uid: user id of owner
// @sk_ino: inode number (zero if orphaned)
// @sk_prefer_busy_poll: prefer busypolling over softirq processing
// @sk_busy_poll_budget: napi processing budget when busypolling
// @sk_priority: %SO_PRIORITY setting
// @sk_type: socket type (%SOCK_STREAM, etc)
// @sk_protocol: which protocol this socket belongs in this network family
// @sk_peer_lock: lock protecting @sk_peer_pid and @sk_peer_cred
// @sk_peer_pid: &struct pid for this socket's peer
// @sk_peer_cred: %SO_PEERCRED setting
// @sk_rcvlowat: %SO_RCVLOWAT setting
// @sk_rcvtimeo: %SO_RCVTIMEO setting
// @sk_sndtimeo: %SO_SNDTIMEO setting
// @sk_txhash: computed flow hash for use on transmit
// @sk_txrehash: enable TX hash rethink
// @sk_filter: socket filtering instructions
// @sk_timer: sock cleanup timer
// @tcp_retransmit_timer: tcp retransmit timer
// @mptcp_retransmit_timer: mptcp retransmit timer
// @sk_stamp: time stamp of last packet received
// @sk_stamp_seq: lock for accessing sk_stamp on 32 bit architectures only
// @sk_tsflags: SO_TIMESTAMPING flags
// @sk_bpf_cb_flags: used in bpf_setsockopt()
// @sk_use_task_frag: allow sk_page_frag() to use current->task_frag.
// Sockets that can be used under memory reclaim should
// set this to false.
// @sk_bind_phc: SO_TIMESTAMPING bind PHC index of PTP virtual clock
// for timestamping
// @sk_tskey: counter to disambiguate concurrent tstamp requests
// @sk_tx_queue_mapping_jiffies: time in jiffies of last @sk_tx_queue_mapping refresh.
// @sk_zckey: counter to order MSG_ZEROCOPY notifications
// @sk_socket: Identd and reporting IO signals
// @sk_user_data: RPC layer private data. Write-protected by @sk_callback_lock.
// @sk_frag: cached page frag
// @sk_peek_off: current peek_offset value
// @sk_send_head: front of stuff to transmit
// @tcp_rtx_queue: TCP re-transmit queue [union with @sk_send_head]
// @sk_security: used by security modules
// @sk_mark: generic packet mark
// @sk_cgrp_data: cgroup data for this cgroup
// @sk_memcg: this socket's memory cgroup association
// @sk_write_pending: a write to stream socket waits to start
// @sk_disconnects: number of disconnect operations performed on this sock
// @sk_state_change: callback to indicate change in the state of the sock
// @sk_data_ready: callback to indicate there is data to be processed
// @sk_write_space: callback to indicate there is bf sending space available
// @sk_error_report: callback to indicate errors (e.g. %MSG_ERRQUEUE)
// @sk_backlog_rcv: callback to process the backlog
// @sk_validate_xmit_skb: ptr to an optional validate function
// @sk_destruct: called at sock freeing time, i.e. when all refcnt == 0
// @sk_reuseport_cb: reuseport group container
// @sk_bpf_storage: ptr to cache and control for bpf_sk_storage
// @sk_rcu: used during RCU grace period
// @sk_freeptr: used for SLAB_TYPESAFE_BY_RCU managed sockets
// @sk_clockid: clockid used by time-based scheduling (SO_TXTIME)
// @sk_txtime_deadline_mode: set deadline mode for SO_TXTIME
// @sk_txtime_report_errors: set report errors mode for SO_TXTIME
// @sk_txtime_unused: unused txtime flags
// @sk_scm_recv_flags: all flags used by scm_recv()
// @sk_scm_credentials: flagged by SO_PASSCRED to recv SCM_CREDENTIALS
// @sk_scm_security: flagged by SO_PASSSEC to recv SCM_SECURITY
// @sk_scm_pidfd: flagged by SO_PASSPIDFD to recv SCM_PIDFD
// @sk_scm_rights: flagged by SO_PASSRIGHTS to recv SCM_RIGHTS
// @sk_scm_unused: unused flags for scm_recv()
// @ns_tracker: tracker for netns reference
// @sk_user_frags: xarray of pages the user is holding a reference on.
// @sk_owner: reference to the real owner of the socket that calls
// sock_lock_init_class_and_name().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
//
// Now struct inet_timewait_sock also uses sock_common, so please just
// don't add nothing before this first member (__sk_common) --acme
//
    pub __sk_common: sock_common,

    pub sk_drops: core::sync::atomic::AtomicI32,
    pub sk_peek_off: __s32,
    pub sk_error_queue: sk_buff_head,
    pub sk_receive_queue: sk_buff_head,
//
// The backlog queue is special, it is always used with
// the per-socket spinlock held and requires low latency
// access. Therefore we special case it's implementation.
// Note : rmem_alloc is in this structure to fill a hole
// on 64bit arches, not because its logically part of
// backlog.
//
    pub rmem_alloc: core::sync::atomic::AtomicI32,
    pub len: c_int,
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub sk_backlog: },

// early demux fields
    pub sk_rx_dst: *mut dst_entry __rcu,
    pub sk_rx_dst_ifindex: c_int,
    pub sk_rx_dst_cookie: u32,

    pub sk_ll_usec: c_uint,
    pub sk_napi_id: c_uint,
    pub sk_busy_poll_budget: u16,
    pub sk_prefer_busy_poll: u8,

    pub sk_userlocks: u8,
    pub sk_rcvbuf: c_int,
    pub sk_filter: *mut sk_filter __rcu,
    pub sk_wq: *mut socket_wq __rcu,
// private:
    pub sk_wq_raw: *mut socket_wq,
// public:
}

//
// Because of non atomicity rules, all
// changes are protected by socket lock.
//

// sockets using SLAB_TYPESAFE_BY_RCU can use sk_freeptr.
// By the time kfree() is called, sk_rcu can not be in
// use and can be mangled.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_bh_locked {
    pub sock: *mut sock,
    pub bh_lock: local_lock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sk_pacing {
    SK_PACING_NONE		= 0,
    SK_PACING_NEEDED	= 1,
    SK_PACING_FQ		= 2,
}

// flag bits in sk_user_data
//
// - SK_USER_DATA_NOCOPY:      Pointer stored in sk_user_data might
// not be suitable for copying when cloning the socket. For instance,
// it can point to a reference counted object. sk_user_data bottom
// bit is set if pointer must not be copied.
//
// - SK_USER_DATA_BPF:         Mark whether sk_user_data field is
// managed/owned by a BPF reuseport array. This bit should be set
// when sk_user_data's sk is added to the bpf's reuseport_array.
//
// - SK_USER_DATA_PSOCK:       Mark whether pointer stored in
// sk_user_data points to psock type. This bit should be set
// when sk_user_data is assigned to a psock object.
//

//
// sk_user_data_is_nocopy - Test if sk_user_data pointer must not be copied
// @sk: socket
//

//
// __locked_read_sk_user_data_with_flags - return the pointer
// only if argument flags all has been set in sk_user_data. Otherwise
// return NULL
//
// @sk: socket
// @flags: flag bits
//
// The caller must be holding sk->sk_callback_lock.
//
// __rcu_dereference_sk_user_data_with_flags - return the pointer
// only if argument flags all has been set in sk_user_data. Otherwise
// return NULL
//
// @sk: socket
// @flags: flag bits
//

extern "C" {
    pub fn read_pnet(_arg: &sk->sk_net) -> return;
}
//
// SK_CAN_REUSE and SK_NO_REUSE on a socket mean that the socket is OK
// or not whether his port will be reused by someone else. SK_FORCE_REUSE
// on a socket means that the socket will reuse everybody else's port
// without looking at the other's sk_reuse value.
//
pub const SK_NO_REUSE: c_int = 0;
pub const SK_CAN_REUSE: c_int = 1;
pub const SK_FORCE_REUSE: c_int = 2;
extern "C" {
    pub fn sk_set_peek_off(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn READ_ONCE(_arg: sk->sk_peek_off) -> return;
}
//
// Hashed lists helper routines
//
extern "C" {
    pub fn hlist_entry(_arg: node, sock: struct, _arg: sk_node) -> return;
}
extern "C" {
    pub fn hlist_entry(_arg: head->first, sock: struct, _arg: sk_node) -> return;
}
extern "C" {
    pub fn hlist_empty(__sk_head(head: head) ? NULL :) -> return;
}
extern "C" {
    pub fn hlist_nulls_entry(_arg: head->first, sock: struct, _arg: sk_nulls_node) -> return;
}
extern "C" {
    pub fn hlist_nulls_empty(__sk_nulls_head(head: head) ? NULL :) -> return;
}
extern "C" {
    pub fn hlist_entry_safe(_arg: sk->sk_node.next, sock: struct, _arg: sk_node) -> return;
}
extern "C" {
    pub fn hlist_unhashed(_arg: &sk->sk_node) -> return;
}
// NB: equivalent to hlist_del_init_rcu
// Grab socket reference count. This operation is valid only
//
// Ungrab socket in the context, which assumes that socket refcnt
//

//
// sk_for_each_entry_offset_rcu - iterate over a list at a given struct offset
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_node to use as a loop cursor.
// @head:	the head for your list.
// @offset:	offset of hlist_node within the struct.
//

// Careful only use this in a context where these parameters
// can not change and must all be valid, such as recvmsg from
// userspace.
//
// Sock flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sock_flags {
    SOCK_DEAD,
    SOCK_DONE,
    SOCK_URGINLINE,
    SOCK_KEEPOPEN,
    SOCK_LINGER,
    SOCK_DESTROY,
    SOCK_BROADCAST,
    SOCK_TIMESTAMP,
    SOCK_ZAPPED,
    SOCK_USE_WRITE_QUEUE, /* whether to call sk->sk_write_space in sock_wfree */
    SOCK_DBG, /* %SO_DEBUG setting */
    SOCK_RCVTSTAMP, /* %SO_TIMESTAMP setting */
    SOCK_RCVTSTAMPNS, /* %SO_TIMESTAMPNS setting */
    SOCK_LOCALROUTE, /* route locally only, %SO_DONTROUTE setting */
    SOCK_MEMALLOC, /* VM depends on this socket for swapping */
    SOCK_TIMESTAMPING_RX_SOFTWARE,  /* %SOF_TIMESTAMPING_RX_SOFTWARE */
    SOCK_FASYNC, /* fasync() active */
    SOCK_RXQ_OVFL,
    SOCK_ZEROCOPY, /* buffers from userspace */
    SOCK_WIFI_STATUS, /* push wifi status to userspace */
    SOCK_NOFCS, /* Tell NIC not to do the Ethernet FCS.
// Will use last 4 bytes of packet sent from
// user-space instead.
//
    SOCK_FILTER_LOCKED, /* Filter cannot be changed anymore */
    SOCK_SELECT_ERR_QUEUE, /* Wake select on error queue */
    SOCK_RCU_FREE, /* wait rcu grace period in sk_destruct() */
    SOCK_TXTIME,
    SOCK_XDP, /* XDP is attached */
    SOCK_TSTAMP_NEW, /* Indicates 64 bit timestamps always */
    SOCK_RCVMARK, /* Receive SO_MARK  ancillary data with packet */
    SOCK_RCVPRIORITY, /* Receive SO_PRIORITY ancillary data with packet */
    SOCK_TIMESTAMPING_ANY, /* Copy of sk_tsflags & TSFLAGS_ANY */
}

//
// The highest bit of sk_tsflags is reserved for kernel-internal
// SOCKCM_FLAG_TS_OPT_ID. There is a check in core/sock.c to control that
// SOF_TIMESTAMPING* values do not reach this reserved area
//

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &sk->sk_flags) -> return;
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &memalloc_socks_key) -> return;
}
extern "C" {
    pub fn __receive_sock(file: *mut file);
}

// Note: If you think the test should be:
// return READ_ONCE(sk->sk_ack_backlog) >= READ_ONCE(sk->sk_max_ack_backlog);
// Then please take a look at commit 64a146513f8f ("[NET]: Revert incorrect accept queue backlog changes.")
//
extern "C" {
    pub fn READ_ONCE(READ_ONCE(sk->sk_max_ack_backlog: sk->sk_ack_backlog) >) -> return;
}
//
// Compute minimal free write space needed to queue new packets.
//
extern "C" {
    pub fn READ_ONCE(READ_ONCE(sk->sk_wmem_queued: sk->sk_sndbuf) -) -> return;
}
// Paired with lockless reads of sk->sk_forward_alloc
extern "C" {
    pub fn sk_stream_write_space(sk: *mut sock);
}
// OOB backlog add
// dont let skb dst not refcounted, we are going to leave rcu lock
//
// Take into account size of receive queue and backlog queue
// Do not take into account this skb truesize,
// to allow even a single big packet to come.
//
// The per-socket spinlock must be held here.
//
// If the skb was allocated from pfmemalloc reserves, only
// allow SOCK_MEMALLOC sockets to use it as this socket is
// helping free memory
//
extern "C" {
    pub fn __sk_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __sk_backlog_rcv(_arg: sk, _arg: skb) -> return;
}

// The following WRITE_ONCE() is paired with the READ_ONCE()
// here, and another one in sock_rps_record_flow().
//

// Paired with READ_ONCE() in sock_rps_record_flow()

// (__timeo) = wait_woken(__wait,			\
// (__timeo));		\
extern "C" {
    pub fn sk_stream_wait_connect(sk: *mut sock, timeo_p: *mut c_long) -> c_int;
}
extern "C" {
    pub fn sk_stream_wait_memory(sk: *mut sock, timeo_p: *mut c_long) -> c_int;
}
extern "C" {
    pub fn sk_stream_wait_close(sk: *mut sock, timeo_p: c_long);
}
extern "C" {
    pub fn sk_stream_error(sk: *mut sock, flags: c_int, err: c_int) -> c_int;
}
extern "C" {
    pub fn sk_stream_kill_queues(sk: *mut sock);
}
extern "C" {
    pub fn sk_set_memalloc(sk: *mut sock);
}
extern "C" {
    pub fn sk_clear_memalloc(sk: *mut sock);
}
extern "C" {
    pub fn __sk_flush_backlog(sk: *mut sock);
}
extern "C" {
    pub fn sk_wait_data(sk: *mut sock, timeo: *mut c_long, skb: *const sk_buff) -> c_int;
}
//
// caches using SLAB_TYPESAFE_BY_RCU should let .next pointer from nulls nodes
// un-modified. Special care is taken when initializing object to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto_accept_arg {
    pub flags: c_int,
    pub err: c_int,
    pub is_empty: c_int,
    pub kern: bool,
}

// Networking protocol blocks we attach to sockets.
// socket layer -> transport layer interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto {
    pub timeout): c_long,
    pub addr_len): c_int,
    pub addr_len): c_int,
    pub flags): *mut *mut *mut int (disconnect)(struct sock sk, int,
    pub arg): *mut proto_accept_arg,
    pub karg): *mut c_int,
    pub sk): *mut *mut int (init)(struct sock,
    pub sk): *mut *mut void (destroy)(struct sock,
    pub how): *mut *mut *mut void (shutdown)(struct sock sk, int,
    pub optlen): c_uint,
    pub option): *mut int __user,
    pub valbool): *mut *mut *mut void (keepalive)(struct sock sk, int,

    pub arg): unsigned int cmd, unsigned long,

    pub len): usize,
    pub flags): size_t len, int,
    pub sock): *mut *mut void (splice_eof)(struct socket,
    pub addr_len): *mut *mut sockaddr_unsized addr, int,
    pub addr_len): *mut *mut sockaddr_unsized addr, int,
    pub skb): *mut sk_buff,
    pub optname): c_int,
    pub sk): *mut *mut void (release_cb)(struct sock,
// Keeping track of sk's, looking them up, and port selection methods.
    pub sk): *mut *mut int (hash)(struct sock,
    pub sk): *mut *mut void (unhash)(struct sock,
    pub sk): *mut *mut void (rehash)(struct sock,
    pub snum): *mut *mut *mut int (get_port)(struct sock sk, unsigned short,
    pub sk): *mut *mut void (put_port)(struct sock,

    pub restore): bool,

// Keeping track of sockets in use

    pub inuse_idx: c_uint,

    pub wake): *const *const *const bool (stream_memory_free)(struct sock sk, int,
    pub sk): *mut *mut bool (sock_is_readable)(struct sock,
// Memory pressure
    pub sk): *mut *mut void (enter_memory_pressure)(struct sock,
    pub sk): *mut *mut void (leave_memory_pressure)(struct sock,
    pub /: *mut *mut *mut atomic_long_t memory_allocated; / Current allocated memory.,
    pub per_cpu_fw_alloc: *mut int __percpu,
    pub /: *mut *mut *mut percpu_counter sockets_allocated; / Current number of sockets.,
//
// Pressure flag: try to collapse.
// Technical note: it is used by multiple contexts non atomically.
// Make sure to use READ_ONCE()/WRITE_ONCE() for all reads/writes.
// All the __sk_mem_schedule() is of this nature: accounting
// is strict, actions are advisory and have some latency.
//
    pub memory_pressure: *mut c_ulong,
    pub sysctl_mem: *mut c_long,
    pub sysctl_wmem: *mut c_int,
    pub sysctl_rmem: *mut c_int,
    pub sysctl_wmem_offset: u32,
    pub sysctl_rmem_offset: u32,
    pub max_header: c_int,
    pub no_autobind: bool,
    pub slab: *mut kmem_cache,
    pub obj_size: c_uint,
    pub freeptr_offset: c_uint,
    pub ipv6_pinfo_offset: c_uint,
    pub slab_flags: slab_flags_t,
    pub /: *mut *mut unsigned int useroffset; / Usercopy region offset,
    pub /: *mut *mut unsigned int usersize; / Usercopy region size,
    pub rsk_prot: *mut request_sock_ops,
    pub twsk_prot: *mut timewait_sock_ops,
    pub hashinfo: *mut inet_hashinfo,
    pub raw_hash: *mut raw_hashinfo,
    pub smc_hash: *mut smc_hashinfo,
    pub h: },
    pub owner: *mut module,
    pub name: [c_char; 32],
    pub node: list_head,
    pub err): *mut *mut *mut int (diag_destroy)(struct sock sk, int,
    pub __randomize_layout: },
    pub alloc_slab): *mut *mut int proto_register(struct proto prot, int,
    pub prot): *mut void proto_unregister(struct proto,
    pub protocol): int sock_load_diag_module(int family, int,
    pub wake)): *const *const INDIRECT_CALLABLE_DECLARE(bool tcp_stream_memory_free(struct sock sk, int,
    pub false: return,
    pub true: tcp_stream_memory_free, sk, wake) :,
    pub 0): return __sk_stream_memory_free(sk,,
    pub wake): __sk_stream_memory_free(sk,,
    pub 0): return __sk_stream_is_writeable(sk,,

    pub -ENOTSUPP: return,

pub const SK_ALLOC_PERCPU_COUNTER_BATCH: c_int = 16;
    pub percpu_counter_read_positive(sk->sk_prot->sockets_allocated): return,
    pub percpu_counter_sum_positive(prot->sockets_allocated): return,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prot_inuse {
    pub all: c_int,
    pub val: [c_int; PROTO_INUSE_NR],
}

extern "C" {
    pub fn sock_prot_inuse_get(net: *mut net, proto: *mut proto) -> c_int;
}
extern "C" {
    pub fn sock_inuse_get(net: *mut net) -> c_int;
}

// With per-bucket locks this operation is not-atomic, so that
// this version is not worse.
//
// About 10 seconds

// Sockets 0-1023 can't be bound to unless you are superuser
pub const PROT_SOCK: c_int = 1024;
pub const SHUTDOWN_MASK: c_int = 3;
pub const RCV_SHUTDOWN: c_int = 1;
pub const SEND_SHUTDOWN: c_int = 2;
pub const SOCK_BINDADDR_LOCK: c_int = 4;
pub const SOCK_BINDPORT_LOCK: c_int = 8;
//
// define SOCK_CONNECT_BIND - &sock->sk_userlocks flag for auto-bind at connect() time
//
pub const SOCK_CONNECT_BIND: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_alloc {
    pub socket: socket,
    pub vfs_inode: inode,
}

//
// Functions for memory accounting
//
extern "C" {
    pub fn __sk_mem_raise_allocated(sk: *mut sock, size: c_int, amt: c_int, kind: c_int) -> c_int;
}
extern "C" {
    pub fn __sk_mem_schedule(sk: *mut sock, size: c_int, kind: c_int) -> c_int;
}
extern "C" {
    pub fn __sk_mem_reduce_allocated(sk: *mut sock, amount: c_int);
}
extern "C" {
    pub fn __sk_mem_reclaim(sk: *mut sock, amount: c_int);
}
pub const SK_MEM_SEND: c_int = 0;
pub const SK_MEM_RECV: c_int = 1;
// sysctl_mem values are in pages
extern "C" {
    pub fn READ_ONCE(_arg: sk->sk_prot->sysctl_mem[index]) -> return;
}
// return true if protocol supports memory accounting
extern "C" {
    pub fn __sk_rmem_schedule(_arg: sk, _arg: size, _arg: skb_pfmemalloc(skb)) -> return;
}
extern "C" {
    pub fn __sk_charge(sk: *mut sock, gfp: gfp_t);
}

//
// Macro so as to not evaluate some arguments when
// lockdep is not enabled.
//
// Mark both the sk_lock and the sk_lock.slock as a
// per-address-family lock class.
//

extern "C" {
    pub fn lock_sock_nested(sk: *mut sock, subclass: c_int);
}
extern "C" {
    pub fn __release_sock(sk: *mut sock);
}
extern "C" {
    pub fn release_sock(sk: *mut sock);
}
// BH context may only use the following locking interface.

extern "C" {
    pub fn __lock_sock_fast(__acquires(&sk->sk_lock.slock: *mut *mut sock sk)) -> bool;
}
//
// lock_sock_fast - fast version of lock_sock
// @sk: socket
//
// This version should be used for very small section, where process won't block
// return false if fast path is taken:
//
// sk_lock.slock locked, owned = 0, BH disabled
//
// return true if slow path is taken:
//
// sk_lock.slock unlocked, owned = 1, BH enabled
//
// The sk_lock has mutex_lock() semantics here.
extern "C" {
    pub fn __lock_sock_fast(_arg: sk) -> return;
}
// fast socket lock variant for caller already holding a [different] socket lock
extern "C" {
    pub fn __lock_sock_fast(_arg: sk) -> return;
}
//
// unlock_sock_fast - complement of lock_sock_fast
// @sk: socket
// @slow: slow mode
//
// fast unlock socket for user context.
// If slow mode is on, we call regular release_sock()
//
extern "C" {
    pub fn sockopt_lock_sock(sk: *mut sock);
}
extern "C" {
    pub fn sockopt_release_sock(sk: *mut sock);
}
extern "C" {
    pub fn sockopt_ns_capable(ns: *mut user_namespace, cap: c_int) -> bool;
}
extern "C" {
    pub fn sockopt_capable(cap: c_int) -> bool;
}
// Used by processes to "lock" a socket state, so that
// interrupts and bottom half handlers won't change it
// from under us. It essentially blocks any incoming
// packets, so that we won't get any new data or any
// packets that change the state of the socket.
//
// While locked, BH processing will add new packets to
// the backlog queue.  This queue is processed by the
// owner of the socket lock right before it is released.
//
// Since ~2.3.5 it is also exclusive sleep lock serializing
// accesses from user process context.
//

// The sk_lock has mutex_unlock() semantics:
// no reclassification while locks are held
extern "C" {
    pub fn sk_free(sk: *mut sock);
}
extern "C" {
    pub fn sk_net_refcnt_upgrade(sk: *mut sock);
}
extern "C" {
    pub fn sk_destruct(sk: *mut sock);
}
extern "C" {
    pub fn sk_clone(_arg: sk, _arg: priority, _arg: true) -> return;
}
extern "C" {
    pub fn sock_wfree(skb: *mut sk_buff);
}
extern "C" {
    pub fn __sock_wfree(skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_wfree(skb: *mut sk_buff);
}
extern "C" {
    pub fn skb_orphan_partial(skb: *mut sk_buff);
}
extern "C" {
    pub fn sock_rfree(skb: *mut sk_buff);
}
extern "C" {
    pub fn sock_rmem_free(skb: *mut sk_buff);
}
extern "C" {
    pub fn sock_efree(skb: *mut sk_buff);
}

extern "C" {
    pub fn sock_edemux(skb: *mut sk_buff);
}
extern "C" {
    pub fn sock_pfree(skb: *mut sk_buff);
}

extern "C" {
    pub fn sock_alloc_send_pskb(_arg: sk, _arg: size, _arg: 0, _arg: noblock, _arg: errcode, _arg: 0) -> return;
}
extern "C" {
    pub fn sock_kfree_s(sk: *mut sock, mem: *mut c_void, size: c_int);
}
extern "C" {
    pub fn sock_kzfree_s(sk: *mut sock, mem: *mut c_void, size: c_int);
}
extern "C" {
    pub fn sk_send_sigurg(sk: *mut sock);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockcm_cookie {
    pub transmit_time: u64,
    pub mark: u32,
    pub tsflags: u32,
    pub ts_opt_id: u32,
    pub priority: u32,
    pub dmabuf_id: u32,
}

// sockc = (struct sockcm_cookie) {
//
// Functions to fill in entries in struct proto_ops when a protocol
// does not implement a particular function.
//
extern "C" {
    pub fn sock_no_bind(sock: *mut socket, saddr: *mut sockaddr_unsized, len: c_int) -> c_int;
}
extern "C" {
    pub fn sock_no_connect(sock: *mut socket, saddr: *mut sockaddr_unsized, len: c_int, flags: c_int) -> c_int;
}
extern "C" {
    pub fn sock_no_socketpair(: *mut socket, : *mut socket) -> c_int;
}
extern "C" {
    pub fn sock_no_accept(: *mut socket, : *mut socket, : *mut proto_accept_arg) -> c_int;
}
extern "C" {
    pub fn sock_no_getname(: *mut socket, : *mut sockaddr, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sock_no_ioctl(: *mut socket, int: unsigned, long: unsigned) -> c_int;
}
extern "C" {
    pub fn sock_no_listen(: *mut socket, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sock_no_shutdown(: *mut socket, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sock_no_sendmsg(: *mut socket, : *mut msghdr, _arg: usize) -> c_int;
}
extern "C" {
    pub fn sock_no_sendmsg_locked(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int;
}
extern "C" {
    pub fn sock_no_recvmsg(: *mut socket, : *mut msghdr, _arg: usize, _arg: c_int) -> c_int;
}
//
// Functions to fill in entries in struct proto_ops when a protocol
// uses the inet style.
//
extern "C" {
    pub fn sk_common_release(sk: *mut sock);
}
//
// Default socket callbacks and setup code
//
// Initialise core socket variables using an explicit uid.
extern "C" {
    pub fn sock_init_data_uid(sock: *mut socket, sk: *mut sock, uid: kuid_t);
}
// Initialise core socket variables.
// Assumes struct socket *sock is embedded in a struct socket_alloc.
//
extern "C" {
    pub fn sock_init_data(sock: *mut socket, sk: *mut sock);
}
//
// Socket reference counting postulates.
//
// * Each user of socket SHOULD hold a reference count.
// * Each access point to socket (an hash table bucket, reference from a list,
// running timer, skb in flight MUST hold a reference count.
// * When reference count hits 0, it means it will never increase back.
// * When reference count hits 0, it means that no references from
// outside exist to this socket and current process on current CPU
// is last user and may/should destroy this socket.
// * sk_free is called from any context: process, BH, IRQ. When
// it is called, socket has no references from outside -> sk_free
// may release descendant resources allocated by the socket, but
// to the time when it is called, socket is NOT referenced by any
// hash tables, lists etc.
// * Packets, delivered from outside (from network or from another process)
// and enqueued on receive/error queues SHOULD NOT grab reference count,
// when they sit in queue. Otherwise, packets will leak to hole, when
// socket is looked up by one cpu and unhasing is made by another CPU.
// It is true for udp/raw, netlink (leak to receive and error queues), tcp
// (leak to backlog). Packet socket does all the processing inside
// BR_NETPROTO_LOCK, so that it has not this race condition. UNIX sockets
// use separate SMP lock, so that they are prone too.
//
// Ungrab socket and destroy it, if it was the last reference.
// Generic version of sock_put(), dealing with all sockets
// (TCP_TIMEWAIT, TCP_NEW_SYN_RECV, ESTABLISHED...)
//
extern "C" {
    pub fn sock_gen_put(sk: *mut sock);
}
extern "C" {
    pub fn __sk_receive_skb(_arg: sk, _arg: skb, _arg: nested, _arg: 1, _arg: true) -> return;
}
// sk_tx_queue_mapping accept only upto a 16-bit value
// Paired with READ_ONCE() in sk_tx_queue_get() and
// other WRITE_ONCE() because socket lock might be not held.
//
// Refresh sk_tx_queue_mapping_jiffies if too old.

// Paired with READ_ONCE() in sk_tx_queue_get() and
// other WRITE_ONCE() because socket lock might be not held.
//
extern "C" {
    pub fn sk_tx_queue_get(sk: *const sock) -> c_int;
}

// Note: sk_uid is unchanged.
// Detach socket from process context.
// Announce socket dead, detach it from wait queue and inode.
// Note that parent inode held reference count on this struct sock,
// we do not release it in this function, because protocol
// probably wants some additional cleanups or even continuing
// to work with this socket (TCP).
//
// Paired with WRITE_ONCE() in sock_graft() and sock_orphan()
extern "C" {
    pub fn READ_ONCE(_arg: sk->sk_ino) -> return;
}
// Paired with WRITE_ONCE() in sockfs_setattr()
extern "C" {
    pub fn READ_ONCE(_arg: sk->sk_uid) -> return;
}
// This pairs with READ_ONCE() in skb_set_hash_from_sk()
// Re-roll the socket txhash.  On a rehash, IPv6 also drops the cached route
// so the next transmit re-selects an ECMP path; IPv4 keeps its route, since
// IPv4 ECMP path selection does not use sk_txhash.
//
extern "C" {
    pub fn sk_mc_loop(sk: *const sock) -> bool;
}
extern "C" {
    pub fn net_gso_ok(_arg: sk->sk_route_caps, _arg: sk->sk_gso_type) -> return;
}
extern "C" {
    pub fn sk_setup_caps(sk: *mut sock, dst: *mut dst_entry);
}
pub const SK_WMEM_ALLOC_BIAS: c_int = 1;
//
// sk_wmem_alloc_get - returns write allocations
// @sk: socket
//
// Return: sk_wmem_alloc minus initial offset of one
//
// sk_rmem_alloc_get - returns read allocations
// @sk: socket
//
// Return: sk_rmem_alloc
//
extern "C" {
    pub fn atomic_read(_arg: &sk->sk_rmem_alloc) -> return;
}
//
// sk_has_allocations - check if allocations are outstanding
// @sk: socket
//
// Return: true if socket has write or read allocations
//
extern "C" {
    pub fn sk_wmem_alloc_get(sk_rmem_alloc_get(sk: sk) ||) -> return;
}
//
// skwq_has_sleeper - check if there are any waiting processes
// @wq: struct socket_wq
//
// Return: true if socket_wq has waiting processes
//
// The purpose of the skwq_has_sleeper and sock_poll_wait is to wrap the memory
// barrier call. They were added due to the race found within the tcp code.
//
// Consider following tcp code paths::
//
// CPU1                CPU2
// sys_select          receive packet
// ...                 ...
// __add_wait_queue    update tp->rcv_nxt
// ...                 ...
// tp->rcv_nxt check   sock_def_readable
// ...                 {
// schedule               rcu_read_lock();
// wq = rcu_dereference(sk->sk_wq);
// if (wq && waitqueue_active(&wq->wait))
// wake_up_interruptible(&wq->wait)
// ...
// }
//
// The race for tcp fires when the __add_wait_queue changes done by CPU1 stay
// in its cache, and so does the tp->rcv_nxt update on CPU2 side.  The CPU1
// could then endup calling schedule and sleep forever if there are no more
// data on the socket.
//
// sock_poll_wait - wrapper for the poll_wait call.
// @filp:           file
// @sock:           socket to wait on
// @p:              poll_table
//
// See the comments in the wq_has_sleeper function.
//
// Provides a barrier we need to be sure we are in sync
// with the socket flags modification.
//
// This memory barrier is paired in the wq_has_sleeper.
//
// This pairs with WRITE_ONCE() in sk_set_txhash()
extern "C" {
    pub fn skb_set_owner_w(skb: *mut sk_buff, sk: *mut sock);
}
//
// Queue a received datagram if it will fit. Stream and sequenced
// protocols can't normally use this as they need to fit buffers in
// and play with them.
//
// Inlined as it's very short and called for pretty much every
// packet ever received.
//
extern "C" {
    pub fn sk_stop_timer(sk: *mut sock, timer: *mut timer_list);
}
extern "C" {
    pub fn sk_stop_timer_sync(sk: *mut sock, timer: *mut timer_list);
}
extern "C" {
    pub fn __sock_queue_rcv_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn sock_queue_err_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
//
// Recover an error report and clear atomically
//
// Avoid an atomic operation for the common case.
// This is racy since another cpu/thread can change sk_err under us.
//
extern "C" {
    pub fn sk_error_report(sk: *mut sock);
}
// Note:
// We use sk->sk_wq_raw, from contexts knowing this
// pointer is not NULL and cannot disappear/change.
//
// Since sk_{r,w}mem_alloc sums skb->truesize, even a small frame might
// need sizeof(sk_buff) + MTU + padding, unless net driver perform copybreak.
// Note: for send buffers, TCP works better if we can build two skbs at
// minimum.
//

//
// sk_page_frag - return an appropriate page_frag
// @sk: socket
//
// Use the per task page_frag instead of the per socket one for
// optimization when we know that we're in process context and own
// everything that's associated with %current.
//
// Both direct reclaim and page faults can nest inside other
// socket operations and end up recursing into sk_page_frag()
// while it's already in use: explicitly avoid task page_frag
// when users disable sk_use_task_frag.
//
// Return: a per task page_frag if context allows that,
// otherwise a per socket one.
//
extern "C" {
    pub fn sk_page_frag_refill(sk: *mut sock, pfrag: *mut page_frag) -> bool;
}
//
// Default write policy as shown to user space via poll/select/SIGIO
//
extern "C" {
    pub fn __sock_writeable(_arg: sk, _arg: refcount_read(&sk->sk_wmem_alloc)) -> return;
}

// Alas, with timeout socket operations are not restartable.
// Compare this to poll().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_skb_cb {
    pub dropcount: u32,
}

// Store sock_skb_cb at the end of skb->cb[] so protocol families
// using skb->cb[] would keep using it directly and utilize its
// alignment guarantee.
//

extern "C" {
    pub fn numa_drop_read(_arg: ndc) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &sk->sk_drops) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: sk->sk_stamp) -> return;
}

extern "C" {
    pub fn skb_has_tx_timestamp(skb: *mut sk_buff, sk: *const sock) -> bool;
}
//
// generate control messages if
// - receive time stamping in software requested
// - software time stamp available and wanted
// - hardware time stamps available and wanted
//

extern "C" {
    pub fn __sock_tx_timestamp(tsflags: __u32, tx_flags: *mut __u8);
}
//
// _sock_tx_timestamp - checks whether the outgoing packet is to be time stamped
// @sk:		socket sending this packet
// @sockc:	pointer to socket cmsg cookie to get timestamping info
// @tx_flags:	completed with instructions for time stamping
// @tskey:      filled in with next sk_tskey (not for TCP, which uses seqno)
//
// Note: callers should take care of initial ``*tx_flags`` value (usually 0)
//
// tskey = sockc->ts_opt_id;
// tskey = atomic_inc_return(&sk->sk_tskey) - 1;
//
// sk_eat_skb - Release a skb if it is no longer needed
// @sk: socket to eat this skb from
// @skb: socket buffer to eat
//
// This routine must be called with interrupts disabled or with the socket
// locked so that the sk_buff queue operation is ok.
//

// This helper checks if a socket is a full socket,
// ie _not_ a timewait or request socket.
//
// Only full sockets have sk->sk_flags.
// This helper checks if a socket is a LISTEN or NEW_SYN_RECV
// SYNACK messages can be attached to either ones (depending on SYNCOOKIE)
//
// This helper checks if a socket is a LISTEN or NEW_SYN_RECV or TIME_WAIT
// TCP SYNACK messages can be attached to LISTEN or NEW_SYN_RECV (depending on SYNCOOKIE)
// TCP RST and ACK can be attached to TIME_WAIT.
//
extern "C" {
    pub fn sock_enable_timestamp(sk: *mut sock, flag: sock_flags);
}
extern "C" {
    pub fn sk_capable(sk: *const sock, cap: c_int) -> bool;
}
extern "C" {
    pub fn sk_net_capable(sk: *const sock, cap: c_int) -> bool;
}
extern "C" {
    pub fn sk_get_meminfo(sk: *const sock, meminfo: *mut u32);
}
// Take into consideration the size of the struct sk_buff overhead in the
// determination of these values, since that is non-constant across
// platforms.  This makes socket queueing behavior and performance
// not depend upon such differences.
//
pub const _SK_MEM_PACKETS: c_int = 256;

// Does this proto have per netns sysctl_wmem ?
extern "C" {
    pub fn READ_ONCE(proto->sysctl_wmem_offset): *mut *mut *mut *mut (int )((void )sock_net(sk) +) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: *mut proto->sysctl_wmem) -> return;
}
// Does this proto have per netns sysctl_rmem ?
extern "C" {
    pub fn READ_ONCE(proto->sysctl_rmem_offset): *mut *mut *mut *mut (int )((void )sock_net(sk) +) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: *mut proto->sysctl_rmem) -> return;
}
// Default TCP Small queue budget is ~1 ms of data (1sec >> 10)
// Some wifi drivers need to tweak it to get more chunks.
// They can use this helper from their ndo_start_xmit()
//
// if a socket is bound to a device, check that the given device
// index is either the same or that the socket is bound to an L3
// master device and the given device index is also enslaved to
// that L3 master
//
extern "C" {
    pub fn sock_def_readable(sk: *mut sock);
}
extern "C" {
    pub fn sock_bindtoindex(sk: *mut sock, ifindex: c_int, lock_sk: bool) -> c_int;
}
extern "C" {
    pub fn sock_set_timestamp(sk: *mut sock, optname: c_int, valbool: bool);
}

extern "C" {
    pub fn bpf_skops_tx_timestamping(sk: *mut sock, skb: *mut sk_buff, op: c_int);
}

extern "C" {
    pub fn sock_no_linger(sk: *mut sock);
}
extern "C" {
    pub fn sock_set_keepalive(sk: *mut sock);
}
extern "C" {
    pub fn sock_set_priority(sk: *mut sock, priority: u32);
}
extern "C" {
    pub fn sock_set_rcvbuf(sk: *mut sock, val: c_int);
}
extern "C" {
    pub fn sock_set_mark(sk: *mut sock, val: u32);
}
extern "C" {
    pub fn sock_set_reuseaddr(sk: *mut sock);
}
extern "C" {
    pub fn sock_set_reuseport(sk: *mut sock);
}
extern "C" {
    pub fn sock_set_sndtimeo(sk: *mut sock, secs: i64);
}
extern "C" {
    pub fn sock_bind_add(sk: *mut sock, addr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn sock_get_timeout(timeo: c_long, optval: *mut c_void, old_timeval: bool) -> c_int;
}
extern "C" {
    pub fn sk_ioctl(sk: *mut sock, cmd: c_uint, arg: *mut void __user) -> c_int;
}
