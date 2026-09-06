//! Automatically rewritten from C Header to Rust Module
//! Source: net/rds/rds.h
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
// RDS Network protocol version
//
pub const RDS_PROTOCOL_3_0: c_uint = 0x0300;
pub const RDS_PROTOCOL_3_1: c_uint = 0x0301;
pub const RDS_PROTOCOL_4_0: c_uint = 0x0400;
pub const RDS_PROTOCOL_4_1: c_uint = 0x0401;

// The following ports, 16385, 18634, 18635, are registered with IANA as
// the ports to be used for RDS over TCP and UDP.  Currently, only RDS over
// TCP and RDS over IB/RDMA are implemented.  18634 is the historical value
// used for the RDMA_CM listener port.  RDS/TCP uses port 16385.  After
// IPv6 work, RDMA_CM also uses 16385 as the listener port.  18634 is kept
// to ensure compatibility with older RDS modules.  Those ports are defined
// in each transport's header file.
//
pub const RDS_PORT: c_int = 18634;

// Macro flag: #define KERNEL_HAS_ATOMIC64

// sigh, pr_debug() causes unused variable warnings

pub const RDS_FRAG_SHIFT: c_int = 12;

// Used to limit both RDMA and non-RDMA RDS message to 1MB

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_cong_map {
    pub m_rb_node: rb_node,
    pub m_addr: in6_addr,
    pub m_waitq: wait_queue_head_t,
    pub m_conn_list: list_head,
    pub m_page_addrs: [c_ulong; RDS_CONG_MAP_PAGES],
}

//
// This is how we will track the connection state:
// A connection is always in one of the following
// states. Updates to the state are atomic and imply
// a memory barrier.
//
// Bits for c_flags
pub const RDS_LL_SEND_FULL: c_int = 0;
pub const RDS_RECONNECT_PENDING: c_int = 1;
pub const RDS_IN_XMIT: c_int = 2;
pub const RDS_RECV_REFILL: c_int = 3;
pub const RDS_DESTROY_PENDING: c_int = 4;
// Max number of multipaths per RDS connection. Must be a power of 2
pub const RDS_MPATH_WORKERS: c_int = 8;

// Per mpath connection state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_conn_path {
    pub cp_conn: *mut rds_connection,
    pub cp_xmit_rm: *mut rds_message,
    pub cp_xmit_sg: c_ulong,
    pub cp_xmit_hdr_off: c_uint,
    pub cp_xmit_data_off: c_uint,
    pub cp_xmit_atomic_sent: c_uint,
    pub cp_xmit_rdma_sent: c_uint,
    pub cp_xmit_data_sent: c_uint,
    pub /: *mut *mut spinlock_t cp_lock; / protect msg queues,
    pub cp_next_tx_seq: u64,
    pub cp_send_queue: list_head,
    pub cp_retrans: list_head,
    pub cp_next_rx_seq: u64,
    pub cp_transport_data: *mut c_void,
    pub cp_wq: *mut workqueue_struct,
    pub cp_state: core::sync::atomic::AtomicI32,
    pub cp_send_gen: c_ulong,
    pub cp_flags: c_ulong,
    pub cp_reconnect_jiffies: c_ulong,
    pub cp_send_w: delayed_work,
    pub cp_recv_w: delayed_work,
    pub cp_conn_w: delayed_work,
    pub cp_down_w: work_struct,
    pub /: *mut *mut mutex cp_cm_lock; / protect cp_state & cm,
    pub cp_waitq: wait_queue_head_t,
    pub cp_unacked_packets: c_uint,
    pub cp_unacked_bytes: c_uint,
    pub cp_index: c_uint,
}

// One rds_connection per RDS address pair
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_connection {
    pub c_hash_node: hlist_node,
    pub c_laddr: in6_addr,
    pub c_faddr: in6_addr,
    pub /: *mut *mut int c_dev_if; / ifindex used for this conn,
    pub /: *mut *mut int c_bound_if; / ifindex of c_laddr,
    pub c_npaths: c_int,
    pub c_with_sport_idx: bool,
    pub c_passive: *mut rds_connection,
    pub c_trans: *mut rds_transport,
    pub c_lcong: *mut rds_cong_map,
    pub c_fcong: *mut rds_cong_map,
// Protocol version
    pub c_proposed_version: c_uint,
    pub c_version: c_uint,
    pub c_net: possible_net_t,
// TOS
    pub c_tos: u8,
    pub c_map_item: list_head,
    pub c_map_queued: c_ulong,
    pub c_path: *mut rds_conn_path,
    pub /: *mut *mut wait_queue_head_t c_hs_waitq; / handshake waitq,
    pub c_my_gen_num: u32,
    pub c_peer_gen_num: u32,
    pub c_cp0_mprds_catchup_tx_seq: u64,
}

extern "C" {
    pub fn read_pnet(_arg: &conn->c_net) -> return;
}
pub const RDS_FLAG_CONG_BITMAP: c_uint = 0x01;
pub const RDS_FLAG_ACK_REQUIRED: c_uint = 0x02;
pub const RDS_FLAG_RETRANSMITTED: c_uint = 0x04;
pub const RDS_FLAG_EXTHDR_EXTENSION: c_uint = 0x20;
pub const RDS_MAX_ADV_CREDIT: c_int = 255;
// RDS_FLAG_PROBE_PORT is the reserved sport used for sending a ping
// probe to exchange control information before establishing a connection.
// Currently the control information that is exchanged is the number of
// supported paths. If the peer is a legacy (older kernel revision) peer,
// it would return a pong message without additional control information
// that would then alert the sender that the peer was an older rev.
//
pub const RDS_FLAG_PROBE_PORT: c_int = 1;

//
// Maximum space available for extension headers.
//
pub const RDS_HEADER_EXT_SPACE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_header {
    pub h_sequence: __be64,
    pub h_ack: __be64,
    pub h_len: __be32,
    pub h_sport: __be16,
    pub h_dport: __be16,
    pub h_flags: u8,
    pub h_credit: u8,
    pub h_padding: [u8; 4],
    pub h_csum: __sum16,
    pub h_exthdr: [u8; RDS_HEADER_EXT_SPACE],
}

//
// Reserved - indicates end of extensions
//
pub const RDS_EXTHDR_NONE: c_int = 0;
//
// This extension header is included in the very
// first message that is sent on a new connection,
// and identifies the protocol level. This will help
// rolling updates if a future change requires breaking
// the protocol.
// NB: This is no longer true for IB, where we do a version
// negotiation during the connection setup phase (protocol
// version information is included in the RDMA CM private data).
//
pub const RDS_EXTHDR_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ext_header_version {
    pub h_version: __be32,
}

//
// This extension header is included in the RDS message
// chasing an RDMA operation.
//
pub const RDS_EXTHDR_RDMA: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ext_header_rdma {
    pub h_rdma_rkey: __be32,
}

//
// This extension header tells the peer about the
// destination <R_Key,offset> of the requested RDMA
// operation.
//
pub const RDS_EXTHDR_RDMA_DEST: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ext_header_rdma_dest {
    pub h_rdma_rkey: __be32,
    pub h_rdma_offset: __be32,
}

//
// This extension header tells the peer about delivered RDMA byte count.
//
pub const RDS_EXTHDR_RDMA_BYTES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ext_header_rdma_bytes {
    pub /: *mut *mut __be32 h_rdma_bytes; / byte count,
    pub /: *mut *mut u8 h_rflags; / direction of RDMA, write or read,
    pub h_pad: [u8; 3],
}

pub const RDS_FLAG_RDMA_WR_BYTES: c_uint = 0x01;
pub const RDS_FLAG_RDMA_RD_BYTES: c_uint = 0x02;
// Extension header announcing number of paths.
// Implicit length = 2 bytes.
//
pub const RDS_EXTHDR_NPATHS: c_int = 5;
pub const RDS_EXTHDR_GEN_NUM: c_int = 6;
pub const RDS_EXTHDR_SPORT_IDX: c_int = 8;

pub const RDS_MSG_RX_HDR: c_int = 0;
pub const RDS_MSG_RX_START: c_int = 1;
pub const RDS_MSG_RX_END: c_int = 2;
pub const RDS_MSG_RX_CMSG: c_int = 3;
// The following values are whitelisted for usercopy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_inc_usercopy {
    pub rdma_cookie: rds_rdma_cookie_t,
    pub rx_tstamp: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_incoming {
    pub i_refcount: refcount_t,
    pub i_item: list_head,
    pub i_conn: *mut rds_connection,
    pub i_conn_path: *mut rds_conn_path,
    pub i_hdr: rds_header,
    pub i_rx_jiffies: c_ulong,
    pub i_saddr: in6_addr,
    pub i_usercopy: rds_inc_usercopy,
    pub i_rx_lat_trace: [u64; RDS_RX_MAX_TRACES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_mr {
    pub r_rb_node: rb_node,
    pub r_kref: kref,
    pub r_key: u32,
// A copy of the creation flags
    pub r_use_once:1: c_uint,
    pub r_invalidate:1: c_uint,
    pub r_write:1: c_uint,
    pub counted: *mut *mut *mut rds_sock r_sock; / socket that owns us;,
// reference, dropped by
// __rds_put_mr_final()
//
    pub r_trans: *mut rds_transport,
    pub r_trans_private: *mut c_void,
}

// atomic operation types
pub const RDS_ATOMIC_TYPE_CSWP: c_int = 0;
pub const RDS_ATOMIC_TYPE_FADD: c_int = 1;
//
// m_sock_item and m_conn_item are on lists that are serialized under
// conn->c_lock.  m_sock_item has additional meaning in that once it is empty
// the message will not be put back on the retransmit list after being sent.
// messages that are canceled while being sent rely on this.
//
// m_inc is used by loopback so that it can pass an incoming message straight
// back up into the rx path.  It embeds a wire header which is also used by
// the send path, which is kind of awkward.
//
// m_sock_item indicates the message's presence on a socket's send or receive
// queue.  m_rs will point to that socket.
//
// m_daddr is used by cancellation to prune messages to a given destination.
//
// The RDS_MSG_ON_SOCK and RDS_MSG_ON_CONN flags are used to avoid lock
// nesting.  As paths iterate over messages on a sock, or conn, they must
// also lock the conn, or sock, to remove the message from those lists too.
// Testing the flag to determine if the message is still on the lists lets
// us avoid testing the list_head directly.  That means each path can use
// the message's list_head to keep it on a local list while juggling locks
// without confusing the other path.
//
// m_ack_seq is an optional field set by transports who need a different
// sequence number range to invalidate.  They can use this in a callback
// that they pass to rds_send_drop_acked() to see if each message has been
// acked.  The HAS_ACK_SEQ flag can be used to detect messages which haven't
// had ack_seq set yet.
//
pub const RDS_MSG_ON_SOCK: c_int = 1;
pub const RDS_MSG_ON_CONN: c_int = 2;
pub const RDS_MSG_HAS_ACK_SEQ: c_int = 3;
pub const RDS_MSG_ACK_REQUIRED: c_int = 4;
pub const RDS_MSG_RETRANSMITTED: c_int = 5;
pub const RDS_MSG_MAPPED: c_int = 6;
pub const RDS_MSG_PAGEVEC: c_int = 7;
pub const RDS_MSG_FLUSH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_znotifier {
    pub z_mmp: mmpin,
    pub z_cookie: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_msg_zcopy_info {
    pub rs_zcookie_next: list_head,
    pub znotif: rds_znotifier,
    pub zcookies: rds_zcopy_cookies,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_msg_zcopy_queue {
    pub zcookie_head: list_head,
    pub /: *mut *mut spinlock_t lock; / protects zcookie_head queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_iov_vector {
    pub iov: *mut rds_iovec,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_iov_vector_arr {
    pub vec: *mut rds_iov_vector,
    pub len: c_int,
    pub indx: c_int,
    pub incr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_message {
    pub m_refcount: refcount_t,
    pub m_sock_item: list_head,
    pub m_conn_item: list_head,
    pub m_inc: rds_incoming,
    pub m_ack_seq: u64,
    pub m_daddr: in6_addr,
    pub m_flags: c_ulong,
// Never access m_rs without holding m_rs_lock.
// Lock nesting is
// rm->m_rs_lock
// -> rs->rs_lock
//
    pub m_rs_lock: spinlock_t,
    pub m_flush_wait: wait_queue_head_t,
    pub m_rs: *mut rds_sock,
// cookie to send to remote, in rds header
    pub m_rdma_cookie: rds_rdma_cookie_t,
    pub m_used_sgs: c_uint,
    pub m_total_sgs: c_uint,
    pub m_final_op: *mut c_void,
// Unpins the ops' user pages and frees the message from
// process context when the final put happens in atomic
// context: dirtying the pages on unpin can sleep.
//
    pub m_unpin_work: work_struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rm_atomic_op {
    pub op_type: c_int,
    pub compare: u64,
    pub swap: u64,
    pub compare_mask: u64,
    pub swap_mask: u64,
    pub op_m_cswp: },
    pub add: u64,
    pub nocarry_mask: u64,
    pub op_m_fadd: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rm_rdma_op {
    pub op_rkey: u32,
    pub op_remote_addr: u64,
    pub op_write:1: c_uint,
    pub op_fence:1: c_uint,
    pub op_notify:1: c_uint,
    pub op_recverr:1: c_uint,
    pub op_mapped:1: c_uint,
    pub op_silent:1: c_uint,
    pub op_active:1: c_uint,
    pub op_unpin_deferred:1: c_uint,
    pub op_bytes: c_uint,
    pub op_nents: c_uint,
    pub op_count: c_uint,
    pub op_sg: *mut scatterlist,
    pub op_notifier: *mut rds_notifier,
    pub op_rdma_mr: *mut rds_mr,
    pub op_odp_addr: u64,
    pub op_odp_mr: *mut rds_mr,
    pub rdma: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rm_data_op {
    pub op_active:1: c_uint,
    pub op_nents: c_uint,
    pub op_count: c_uint,
    pub op_dmasg: c_uint,
    pub op_dmaoff: c_uint,
    pub op_mmp_znotifier: *mut rds_znotifier,
    pub op_sg: *mut scatterlist,
    pub data: },
}

//
// The RDS notifier is used (optionally) to tell the application about
// completed RDMA operations. Rather than keeping the whole rds message
// around on the queue, we allocate a small notifier that is put on the
// socket's notifier_list. Notifications are delivered to the application
// through control messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_notifier {
    pub n_list: list_head,
    pub n_user_token: u64,
    pub n_status: c_int,
}

// Available as part of RDS core, so doesn't need to participate
// in get_preferred transport etc
//
pub const RDS_TRANS_LOOP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_transport {
    pub t_name: [c_char; TRANSNAMSIZ],
    pub t_item: list_head,
    pub t_owner: *mut module,
    pub t_type: c_uint,
    pub scope_id): __u32,
    pub gfp): *mut *mut *mut int (conn_alloc)(struct rds_connection conn, gfp_t,
    pub data): *mut *mut void (conn_free)(void,
//
// conn_slots_available is invoked when a previously unavailable
// connection slot becomes available again. rds_tcp_accept_one_path may
// return -ENOBUFS if it cannot find an available slot, and then stashes
// the new socket in "rds_tcp_accepted_sock". This function re-issues
// `rds_tcp_accept_one_path`, which picks up the stashed socket and
// continuing where it left with "-ENOBUFS" last time.  This ensures
// messages received on the new socket are not discarded when no
// connection path was available at the time.
//
    pub fan_out): *mut *mut *mut void (conn_slots_available)(struct rds_connection conn, bool,
    pub cp): *mut *mut int (conn_path_connect)(struct rds_conn_path,
//
// conn_shutdown stops traffic on the given connection.  Once
// it returns the connection can not call rds_recv_incoming().
// This will only be called once after conn_connect returns
// non-zero success and will The caller serializes this with
// the send and connecting paths (xmit_* and conn_*).  The
// transport is responsible for other serialization, including
// rds_recv_incoming().  This is called in process context but
// should try hard not to block.
//
    pub conn): *mut *mut void (conn_path_shutdown)(struct rds_conn_path,
    pub cp): *mut *mut void (xmit_path_prepare)(struct rds_conn_path,
    pub cp): *mut *mut void (xmit_path_complete)(struct rds_conn_path,
//
// .xmit is called by rds_send_xmit() to tell the transport to send
// part of a message.  The caller serializes on the send_sem so this
// doesn't need to be reentrant for a given conn.  The header must be
// sent before the data payload.  .xmit must be prepared to send a
// message with no data payload.  .xmit should return the number of
// bytes that were sent down the connection, including header bytes.
// Returning 0 tells the caller that it doesn't need to perform any
// additional work now.  This is usually the case when the transport has
// filled the sending queue for its connection and will handle
// triggering the rds thread to continue the send when space becomes
// available.  Returning -EAGAIN tells the caller to retry the send
// immediately.  Returning -ENOMEM tells the caller to retry the send at
// some point in the future.
//
    pub off): unsigned int hdr_off, unsigned int sg, unsigned int,
    pub op): *mut *mut *mut int (xmit_rdma)(struct rds_connection conn, struct rm_rdma_op,
    pub op): *mut *mut *mut int (xmit_atomic)(struct rds_connection conn, struct rm_atomic_op,
    pub cp): *mut *mut int (recv_path)(struct rds_conn_path,
    pub to): *mut *mut *mut int (inc_copy_to_user)(struct rds_incoming inc, struct iov_iter,
    pub inc): *mut *mut void (inc_free)(struct rds_incoming,
    pub isv6): *mut *mut rdma_cm_event event, bool,
    pub isv6): *mut *mut *mut int (cm_initiate_connect)(struct rdma_cm_id cm_id, bool,
    pub event): *mut rdma_cm_event,
    pub avail): c_uint,
    pub (*exit)(void): *mut c_void,
    pub need_odp): u64 start, u64 length, int,
    pub direction): *mut *mut *mut void (sync_mr)(void trans_private, int,
    pub invalidate): *mut *mut *mut void (free_mr)(void trans_private, int,
    pub (*flush_mrs)(void): *mut c_void,
    pub conn): *mut *mut bool (t_unloading)(struct rds_connection,
    pub tos): *mut *mut u8 (get_tos_map)(u8,
}

// Bind hash table key length.  It is the sum of the size of a struct
// in6_addr, a scope_id  and a port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_sock {
    pub rs_sk: sock,
    pub rs_user_addr: u64,
    pub rs_user_bytes: u64,
//
// bound_addr used for both incoming and outgoing, no INADDR_ANY
// support.
//
    pub rs_bound_node: rhash_head,
    pub rs_bound_key: [u8; RDS_BOUND_KEY_LEN],
    pub rs_bound_sin6: sockaddr_in6,

    pub rs_conn_addr: in6_addr,

    pub rs_conn_port: __be16,
    pub rs_transport: *mut rds_transport,
//
// rds_sendmsg caches the conn it used the last time around.
// This helps avoid costly lookups.
//
    pub rs_conn: *mut rds_connection,
// flag indicating we were congested or not
    pub rs_congested: c_int,
// seen congestion (ENOBUFS) when sending?
    pub rs_seen_congestion: c_int,
// rs_lock protects all these adjacent members before the newline
    pub rs_lock: spinlock_t,
    pub rs_send_queue: list_head,
    pub rs_snd_bytes: u32,
    pub rs_rcv_bytes: c_int,
    pub /: *mut *mut list_head rs_notify_queue; / currently used for failed RDMAs,
// Congestion wake_up. If rs_cong_monitor is set, we use cong_mask
// to decide whether the application should be woken up.
// If not set, we use rs_cong_track to find out whether a cong map
// update arrived.
//
    pub rs_cong_mask: u64,
    pub rs_cong_notify: u64,
    pub rs_cong_list: list_head,
    pub rs_cong_track: c_ulong,
//
// rs_recv_lock protects the receive queue, and is
// used to serialize with rds_release.
//
    pub rs_recv_lock: rwlock_t,
    pub rs_recv_queue: list_head,
// just for stats reporting
    pub rs_item: list_head,
// these have their own lock
    pub rs_rdma_lock: spinlock_t,
    pub rs_rdma_keys: rb_root,
// Socket options - in case there will be more
    pub rs_hash_initval: u32,
// Socket receive path trace points
    pub rs_rx_traces: u8,
    pub rs_rx_trace: [u8; RDS_MSG_RX_DGRAM_TRACE_MAX],
    pub rs_zcookie_queue: rds_msg_zcopy_queue,
    pub rs_tos: u8,
}

extern "C" {
    pub fn container_of(_arg: sk, rds_sock: struct, _arg: rs_sk) -> return;
}
//
// The stack assigns sk_sndbuf and sk_rcvbuf to twice the specified value
// to account for overhead.  We don't account for overhead, we just apply
// the number of payload bytes to the specified value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_statistics {
    pub s_conn_reset: u64,
    pub s_recv_drop_bad_checksum: u64,
    pub s_recv_drop_old_seq: u64,
    pub s_recv_drop_no_sock: u64,
    pub s_recv_drop_dead_sock: u64,
    pub s_recv_deliver_raced: u64,
    pub s_recv_delivered: u64,
    pub s_recv_queued: u64,
    pub s_recv_immediate_retry: u64,
    pub s_recv_delayed_retry: u64,
    pub s_recv_ack_required: u64,
    pub s_recv_rdma_bytes: u64,
    pub s_recv_ping: u64,
    pub s_send_queue_empty: u64,
    pub s_send_queue_full: u64,
    pub s_send_lock_contention: u64,
    pub s_send_lock_queue_raced: u64,
    pub s_send_immediate_retry: u64,
    pub s_send_delayed_retry: u64,
    pub s_send_drop_acked: u64,
    pub s_send_ack_required: u64,
    pub s_send_queued: u64,
    pub s_send_rdma: u64,
    pub s_send_rdma_bytes: u64,
    pub s_send_pong: u64,
    pub s_page_remainder_hit: u64,
    pub s_page_remainder_miss: u64,
    pub s_copy_to_user: u64,
    pub s_copy_from_user: u64,
    pub s_cong_update_queued: u64,
    pub s_cong_update_received: u64,
    pub s_cong_send_error: u64,
    pub s_cong_send_blocked: u64,
    pub s_recv_bytes_added_to_socket: u64,
    pub s_recv_bytes_removed_from_socket: u64,
    pub s_send_stuck_rm: u64,
    pub s_mprds_catchup_tx0_retries: u64,
}

// af_rds.c
extern "C" {
    pub fn rds_sock_addref(rs: *mut rds_sock);
}
extern "C" {
    pub fn rds_sock_put(rs: *mut rds_sock);
}
extern "C" {
    pub fn rds_wake_sk_sleep(rs: *mut rds_sock);
}
// bind.c
extern "C" {
    pub fn rds_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn rds_remove_bound(rs: *mut rds_sock);
}
extern "C" {
    pub fn rds_bind_lock_init() -> c_int;
}
extern "C" {
    pub fn rds_bind_lock_destroy();
}
// cong.c
extern "C" {
    pub fn rds_cong_get_maps(conn: *mut rds_connection) -> c_int;
}
extern "C" {
    pub fn rds_cong_add_conn(conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_cong_remove_conn(conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_cong_set_bit(map: *mut rds_cong_map, port: __be16);
}
extern "C" {
    pub fn rds_cong_clear_bit(map: *mut rds_cong_map, port: __be16);
}
extern "C" {
    pub fn rds_cong_wait(map: *mut rds_cong_map, port: __be16, nonblock: c_int, rs: *mut rds_sock) -> c_int;
}
extern "C" {
    pub fn rds_cong_queue_updates(map: *mut rds_cong_map);
}
extern "C" {
    pub fn rds_cong_map_updated(map: *mut rds_cong_map, _arg: u64);
}
extern "C" {
    pub fn rds_cong_updated_since(recent: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn rds_cong_add_socket(: *mut rds_sock);
}
extern "C" {
    pub fn rds_cong_remove_socket(: *mut rds_sock);
}
extern "C" {
    pub fn rds_cong_exit();
}
// connection.c
extern "C" {
    pub fn rds_conn_init() -> c_int;
}
extern "C" {
    pub fn rds_conn_exit();
}
extern "C" {
    pub fn rds_conn_shutdown(cpath: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_conn_destroy(conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_conn_drop(conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_conn_path_drop(cpath: *mut rds_conn_path, destroy: bool);
}
extern "C" {
    pub fn rds_conn_connect_if_down(conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_conn_path_connect_if_down(cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_check_all_paths(conn: *mut rds_connection);
}
extern "C" {
    pub fn __rds_conn_path_error(cp: *mut rds_conn_path, : *const c_char, ...);
}

extern "C" {
    pub fn rds_conn_path_transition(_arg: &conn->c_path[0], _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &cp->cp_state) -> return;
}
extern "C" {
    pub fn rds_conn_path_state(_arg: &conn->c_path[0]) -> return;
}
extern "C" {
    pub fn rds_conn_path_up(_arg: &conn->c_path[0]) -> return;
}
extern "C" {
    pub fn rds_conn_path_connecting(_arg: &conn->c_path[0]) -> return;
}
// message.c
extern "C" {
    pub fn rds_message_add_rdma_dest_extension(hdr: *mut rds_header, r_key: u32, offset: u32) -> c_int;
}
extern "C" {
    pub fn rds_message_inc_copy_to_user(inc: *mut rds_incoming, to: *mut iov_iter) -> c_int;
}
extern "C" {
    pub fn rds_message_addref(rm: *mut rds_message);
}
extern "C" {
    pub fn rds_message_put(rm: *mut rds_message);
}
extern "C" {
    pub fn rds_message_wait(rm: *mut rds_message);
}
extern "C" {
    pub fn rds_message_unmapped(rm: *mut rds_message);
}
extern "C" {
    pub fn rds_notify_msg_zcopy_purge(info: *mut rds_msg_zcopy_queue);
}
// page.c
extern "C" {
    pub fn rds_page_exit();
}
// recv.c
extern "C" {
    pub fn rds_inc_put(inc: *mut rds_incoming);
}
extern "C" {
    pub fn rds_clear_recv_queue(rs: *mut rds_sock);
}
extern "C" {
    pub fn rds_notify_queue_get(rs: *mut rds_sock, msg: *mut msghdr) -> c_int;
}
// send.c
extern "C" {
    pub fn rds_sendmsg(sock: *mut socket, msg: *mut msghdr, payload_len: usize) -> c_int;
}
extern "C" {
    pub fn rds_send_path_reset(conn: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_send_xmit(cp: *mut rds_conn_path) -> c_int;
}
extern "C" {
    pub fn rds_send_drop_to(rs: *mut rds_sock, dest: *mut sockaddr_in6);
}
extern "C" {
    pub fn int(rm: *mut *mut is_acked_func)(struct rds_message, ack: u64) -> typedef;
}
extern "C" {
    pub fn rds_send_ping(conn: *mut rds_connection, cp_index: c_int);
}
extern "C" {
    pub fn rds_send_pong(cp: *mut rds_conn_path, dport: __be16) -> c_int;
}
// rdma.c
extern "C" {
    pub fn rds_rdma_unuse(rs: *mut rds_sock, r_key: u32, force: c_int);
}
extern "C" {
    pub fn rds_get_mr(rs: *mut rds_sock, optval: sockptr_t, optlen: c_int) -> c_int;
}
extern "C" {
    pub fn rds_get_mr_for_dest(rs: *mut rds_sock, optval: sockptr_t, optlen: c_int) -> c_int;
}
extern "C" {
    pub fn rds_free_mr(rs: *mut rds_sock, optval: sockptr_t, optlen: c_int) -> c_int;
}
extern "C" {
    pub fn rds_rdma_drop_keys(rs: *mut rds_sock);
}
extern "C" {
    pub fn rds_rdma_free_op(ro: *mut rm_rdma_op);
}
extern "C" {
    pub fn rds_atomic_free_op(ao: *mut rm_atomic_op);
}
extern "C" {
    pub fn rds_rdma_op_unpin_pages(ro: *mut rm_rdma_op);
}
extern "C" {
    pub fn rds_atomic_op_unpin_page(ao: *mut rm_atomic_op);
}
extern "C" {
    pub fn rds_rdma_send_complete(rm: *mut rds_message, wc_status: c_int);
}
extern "C" {
    pub fn rds_atomic_send_complete(rm: *mut rds_message, wc_status: c_int);
}
extern "C" {
    pub fn __rds_put_mr_final(kref: *mut kref);
}
// stats.c

extern "C" {
    pub fn rds_stats_init() -> c_int;
}
extern "C" {
    pub fn rds_stats_exit();
}
// sysctl.c
extern "C" {
    pub fn rds_sysctl_init() -> c_int;
}
extern "C" {
    pub fn rds_sysctl_exit();
}
// threads.c
extern "C" {
    pub fn rds_threads_init() -> c_int;
}
extern "C" {
    pub fn rds_threads_exit();
}
extern "C" {
    pub fn rds_queue_reconnect(cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_connect_worker(: *mut work_struct);
}
extern "C" {
    pub fn rds_shutdown_worker(: *mut work_struct);
}
extern "C" {
    pub fn rds_send_worker(: *mut work_struct);
}
extern "C" {
    pub fn rds_recv_worker(: *mut work_struct);
}
extern "C" {
    pub fn rds_connect_path_complete(conn: *mut rds_conn_path, curr: c_int);
}
extern "C" {
    pub fn rds_connect_complete(conn: *mut rds_connection);
}
extern "C" {
    pub fn rds_addr_cmp(a1: *const in6_addr, a2: *const in6_addr) -> c_int;
}
// transport.c
extern "C" {
    pub fn rds_trans_register(trans: *mut rds_transport);
}
extern "C" {
    pub fn rds_trans_unregister(trans: *mut rds_transport);
}
extern "C" {
    pub fn rds_trans_put(trans: *mut rds_transport);
}
