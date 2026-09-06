//! Automatically rewritten from C Header to Rust Module
//! Source: net/l2tp/l2tp_core.h
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


// SPDX-License-Identifier: GPL-2.0-only
// L2TP internal definitions.
//
// Copyright (c) 2008,2009 Katalix Systems Ltd
//

// Random numbers used for internal consistency checks of tunnel and session structures
pub const L2TP_SESSION_MAGIC: c_uint = 0x0C04EB7D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_stats {
    pub tx_packets: atomic_long_t,
    pub tx_bytes: atomic_long_t,
    pub tx_errors: atomic_long_t,
    pub rx_packets: atomic_long_t,
    pub rx_bytes: atomic_long_t,
    pub rx_seq_discards: atomic_long_t,
    pub rx_oos_packets: atomic_long_t,
    pub rx_errors: atomic_long_t,
    pub rx_cookie_discards: atomic_long_t,
    pub rx_invalid: atomic_long_t,
}

// L2TP session configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_session_cfg {
    pub pw_type: l2tp_pwtype,
    pub /: *mut *mut unsigned int recv_seq:1; / expect receive packets with sequence numbers?,
    pub /: *mut *mut unsigned int send_seq:1; / send packets with sequence numbers?,
    pub LNS?: *mut *mut unsigned int lns_mode:1; / behave as,
// LAC enables sequence numbers under LNS control.
//
    pub /: *mut *mut u16 l2specific_type; / Layer 2 specific type,
    pub /: *mut *mut u8 cookie[8]; / optional cookie,
    pub /: *mut *mut int cookie_len; / 0, 4 or 8 bytes,
    pub /: *mut *mut u8 peer_cookie[8]; / peer's cookie,
    pub /: *mut *mut int peer_cookie_len; / 0, 4 or 8 bytes,
    pub /: *mut *mut int reorder_timeout; / configured reorder timeout (in jiffies),
    pub ifname: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_session_coll_list {
    pub /: *mut *mut spinlock_t lock; / for access to list,
    pub list: list_head,
    pub ref_count: refcount_t,
}

// Represents a session (pseudowire) instance.
// Tracks runtime state including cookies, dataplane packet sequencing, and IO statistics.
// Is linked into a per-tunnel session list and a per-net ("global") IDR tree.
//
pub const L2TP_SESSION_NAME_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_session {
    pub /: *mut *mut int magic; / should be L2TP_SESSION_MAGIC,
    pub dead: c_long,
    pub rcu: rcu_head,
    pub /: *mut *mut *mut l2tp_tunnel tunnel; / back pointer to tunnel context,
    pub session_id: u32,
    pub peer_session_id: u32,
    pub cookie: [u8; 8],
    pub cookie_len: c_int,
    pub peer_cookie: [u8; 8],
    pub peer_cookie_len: c_int,
    pub l2specific_type: u16,
    pub hdr_len: u16,
    pub /: *mut *mut u32 nr; / session NR state (receive),
    pub /: *mut *mut u32 ns; / session NR state (send),
    pub /: *mut *mut sk_buff_head reorder_q; / receive reorder queue,
    pub /: *mut *mut u32 nr_max; / max NR. Depends on tunnel,
    pub /: *mut *mut u32 nr_window_size; / NR window size,
    pub /: *mut *mut u32 nr_oos; / NR of last OOS packet,
    pub /: *mut *mut int nr_oos_count; / for OOS recovery,
    pub nr_oos_count_max: c_int,
    pub /: *mut *mut list_head list; / per-tunnel list node,
    pub ref_count: refcount_t,
    pub /: *mut *mut hlist_node hlist; / per-net session hlist,
    pub /: *mut *mut unsigned long hlist_key; / key for session hlist,
    pub /: *mut *mut *mut l2tp_session_coll_list coll_list; / session collision list,
    pub /: *mut *mut list_head clist; / for coll_list,
    pub /: *mut *mut char name[L2TP_SESSION_NAME_MAX]; / for logging,
    pub ifname: [c_char; IFNAMSIZ],
    pub /: *mut *mut unsigned int recv_seq:1; / expect receive packets with sequence numbers?,
    pub /: *mut *mut unsigned int send_seq:1; / send packets with sequence numbers?,
    pub LNS?: *mut *mut unsigned int lns_mode:1; / behave as,
// LAC enables sequence numbers under LNS control.
//
    pub /: *mut *mut int reorder_timeout; / configured reorder timeout (in jiffies),
    pub /: *mut *mut int reorder_skip; / set if skip to next nr,
    pub pwtype: l2tp_pwtype,
    pub stats: l2tp_stats,
    pub del_work: work_struct,
// Session receive handler for data packets.
// Each pseudowire implementation should implement this callback in order to
// handle incoming packets.  Packets are passed to the pseudowire handler after
// reordering, if data sequence numbers are enabled for the session.
//
    pub data_len): *mut *mut *mut *mut void (recv_skb)(struct l2tp_session session, struct sk_buff skb, int,
// Session close handler.
// Each pseudowire implementation may implement this callback in order to carry
// out pseudowire-specific shutdown actions.
// The callback is called by core after unlisting the session and purging its
// reorder queue.
//
    pub session): *mut *mut void (session_close)(struct l2tp_session,
// Session show handler.
// Pseudowire-specific implementation of debugfs session rendering.
// The callback is called by l2tp_debugfs.c after rendering core session
// information.
//
    pub priv): *mut *mut *mut void (show)(struct seq_file m, void,
    pub /: *mut *mut u8 priv[]; / private data,
}

// L2TP tunnel configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_tunnel_cfg {
    pub encap: l2tp_encap_type,
// Used only for kernel-created sockets
    pub local_ip: in_addr,
    pub peer_ip: in_addr,

    pub local_ip6: *mut in6_addr,
    pub peer_ip6: *mut in6_addr,

    pub local_udp_port: u16,
    pub peer_udp_port: u16,
}

// Represents a tunnel instance.
// Tracks runtime state including IO statistics.
// Holds the tunnel socket (either passed from userspace or directly created by the kernel).
// Maintains a list of sessions belonging to the tunnel instance.
// Is linked into a per-net list of tunnels.
//
pub const L2TP_TUNNEL_NAME_MAX: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_tunnel {
    pub dead: c_ulong,
    pub rcu: rcu_head,
    pub /: *mut *mut spinlock_t list_lock; / write-protection for session_list,
    pub accepts: *mut *mut bool acpt_newsess; / indicates whether this tunnel,
// new sessions. Protected by list_lock.
//
    pub /: *mut *mut list_head session_list; / list of sessions,
    pub tunnel_id: u32,
    pub peer_tunnel_id: u32,
    pub /: *mut *mut int version; / 2=>L2TPv2, 3=>L2TPv3,
    pub /: *mut *mut char name[L2TP_TUNNEL_NAME_MAX]; / for logging,
    pub encap: l2tp_encap_type,
    pub stats: l2tp_stats,
    pub /: *mut *mut *mut net l2tp_net; / the net we belong to,
    pub ref_count: refcount_t,
    pub /: *mut *mut *mut sock sock; / parent socket,
    pub created: *mut *mut int fd; / parent fd, if tunnel socket was,
// by userspace
//
    pub del_work: work_struct,
}

// Pseudowire ops callbacks for use with the l2tp genetlink interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2tp_nl_cmd_ops {
// The pseudowire session create callback is responsible for creating a session
// instance for a specific pseudowire type.
// It must call l2tp_session_create and l2tp_session_register to register the
// session instance, as well as carry out any pseudowire-specific initialisation.
// It must return >= 0 on success, or an appropriate negative errno value on failure.
//
    pub cfg): *mut l2tp_session_cfg,
// The pseudowire session delete callback is responsible for initiating the deletion
// of a session instance.
// It must call l2tp_session_delete, as well as carry out any pseudowire-specific
// teardown actions.
//
    pub session): *mut *mut void (session_delete)(struct l2tp_session,
}

// Tunnel and session refcounts
extern "C" {
    pub fn l2tp_tunnel_put(tunnel: *mut l2tp_tunnel);
}
extern "C" {
    pub fn l2tp_session_put(session: *mut l2tp_session);
}
// Tunnel and session lookup.
// These functions take a reference on the instances they return, so
// the caller must ensure that the reference is dropped appropriately.
//
// Tunnel and session lifetime management.
// Creation of a new instance is a two-step process: create, then register.
// Destruction is triggered using the *_delete functions, and completes asynchronously.
//
extern "C" {
    pub fn l2tp_tunnel_delete(tunnel: *mut l2tp_tunnel);
}
extern "C" {
    pub fn l2tp_session_delete(session: *mut l2tp_session);
}
// Receive path helpers.  If data sequencing is enabled for the session these
// functions handle queuing and reordering prior to passing packets to the
// pseudowire code to be passed to userspace.
//
extern "C" {
    pub fn l2tp_udp_encap_recv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
// Transmit path helpers for sending packets over the tunnel socket.
extern "C" {
    pub fn l2tp_xmit_skb(session: *mut l2tp_session, skb: *mut sk_buff) -> c_int;
}
// Pseudowire management.
// Pseudowires should register with l2tp core on module init, and unregister
// on module exit.
//
extern "C" {
    pub fn l2tp_nl_register_ops(pw_type: l2tp_pwtype, ops: *const l2tp_nl_cmd_ops) -> c_int;
}
extern "C" {
    pub fn l2tp_nl_unregister_ops(pw_type: l2tp_pwtype);
}
// IOCTL helper for IP encap modules.
extern "C" {
    pub fn l2tp_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int;
}

// optr = skb->data;
// ptr = skb->data + off;

