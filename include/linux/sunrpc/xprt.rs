//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/xprt.h
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
// linux/include/linux/sunrpc/xprt.h
//
// Declarations for the RPC transport interface.
//
// Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_display_format_t {
    RPC_DISPLAY_ADDR = 0,
    RPC_DISPLAY_PORT,
    RPC_DISPLAY_PROTO,
    RPC_DISPLAY_HEX_ADDR,
    RPC_DISPLAY_HEX_PORT,
    RPC_DISPLAY_NETID,
    RPC_DISPLAY_MAX,
}

//
// This describes a complete RPC request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_rqst {
//
// This is the user-visible part
//
    pub /: *mut *mut *mut rpc_xprt  rq_xprt; / RPC client,
    pub /: *mut *mut xdr_buf rq_snd_buf; / send buffer,
    pub /: *mut *mut xdr_buf rq_rcv_buf; / recv buffer,
//
// This is the private part
//
    pub /: *mut *mut *mut rpc_task  rq_task; / RPC task data,
    pub /: *mut *mut *mut rpc_cred  rq_cred; / Bound cred,
    pub /: *mut *mut __be32 rq_xid; / request XID,
    pub /: *mut *mut int rq_cong; / has incremented xprt->cong,
    pub /: *mut *mut u32 rq_seqnos[RPC_GSS_SEQNO_ARRAY_SIZE]; / past gss req seq nos.,
    pub /: *mut *mut unsigned int rq_seqno_count; / number of entries in rq_seqnos,
    pub rq_enc_pages_num: c_int,
    pub by: *mut *mut *mut *mut page rq_enc_pages; / scratch pages for use,
    pub /: *mut *mut *mut *mut void (rq_release_snd_buf)(struct rpc_rqst ); / release rq_enc_pages,
    pub /: *mut *mut list_head rq_list; / Slot allocation list,
    pub /: *mut *mut rb_node rq_recv; / Receive queue,
}

// received
// used in the softirq.
//
// A cookie used to track the
//
// Partial send handling
//

// Shift array to make room for the newest element at the beginning
// RPC transport layer security policies
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xprtsec_policies {
    RPC_XPRTSEC_NONE = 0,
    RPC_XPRTSEC_TLS_ANON,
    RPC_XPRTSEC_TLS_X509,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xprtsec_parms {
    pub policy: xprtsec_policies,
// authentication material
    pub cert_serial: key_serial_t,
    pub privkey_serial: key_serial_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_xprt_ops {
    pub rcvsize): *mut *mut *mut void (set_buffer_size)(struct rpc_xprt xprt, size_t sndsize, size_t,
    pub task): *mut *mut *mut int (reserve_xprt)(struct rpc_xprt xprt, struct rpc_task,
    pub task): *mut *mut *mut void (release_xprt)(struct rpc_xprt xprt, struct rpc_task,
    pub task): *mut *mut *mut void (alloc_slot)(struct rpc_xprt xprt, struct rpc_task,
    pub req): *mut rpc_rqst,
    pub task): *mut *mut void (rpcbind)(struct rpc_task,
    pub port): *mut *mut *mut void (set_port)(struct rpc_xprt xprt, unsigned short,
    pub task): *mut *mut *mut void (connect)(struct rpc_xprt xprt, struct rpc_task,
    pub buflen): usize,
    pub xprt): *mut *mut unsigned short (get_srcport)(struct rpc_xprt,
    pub task): *mut *mut int (buf_alloc)(struct rpc_task,
    pub task): *mut *mut void (buf_free)(struct rpc_task,
    pub buf): *mut xdr_buf,
    pub req): *mut *mut int (send_request)(struct rpc_rqst,
    pub req): *mut *mut void (abort_send_request)(struct rpc_rqst,
    pub task): *mut *mut void (wait_for_reply_request)(struct rpc_task,
    pub task): *mut *mut *mut void (timer)(struct rpc_xprt xprt, struct rpc_task,
    pub task): *mut *mut void (release_request)(struct rpc_task,
    pub xprt): *mut *mut void (close)(struct rpc_xprt,
    pub xprt): *mut *mut void (destroy)(struct rpc_xprt,
    pub reconnect_timeout): c_ulong,
    pub seq): *mut *mut *mut void (print_stats)(struct rpc_xprt xprt, struct seq_file,
    pub xprt): *mut *mut int (enable_swap)(struct rpc_xprt,
    pub xprt): *mut *mut void (disable_swap)(struct rpc_xprt,
    pub xprt): *mut *mut void (inject_disconnect)(struct rpc_xprt,
    pub min_reqs): c_uint,
    pub xprt): *mut *mut size_t (bc_maxpayload)(struct rpc_xprt,
    pub xprt): *mut *mut unsigned int (bc_num_slots)(struct rpc_xprt,
    pub rqst): *mut *mut void (bc_free_rqst)(struct rpc_rqst,
    pub max_reqs): c_uint,
}

//
// RPC transport identifiers
//
// To preserve compatibility with the historical use of raw IP protocol
// id's for transport selection, UDP and TCP identifiers are specified
// with the previous values. No such restriction exists for new transports,
// except that they may not collide with these values (17 and 6,
// respectively).
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xprt_transports {
    XPRT_TRANSPORT_UDP	= IPPROTO_UDP,
    XPRT_TRANSPORT_TCP	= IPPROTO_TCP,
    XPRT_TRANSPORT_BC_TCP	= IPPROTO_TCP | XPRT_TRANSPORT_BC,
    XPRT_TRANSPORT_RDMA	= 256,
    XPRT_TRANSPORT_BC_RDMA	= XPRT_TRANSPORT_RDMA | XPRT_TRANSPORT_BC,
    XPRT_TRANSPORT_LOCAL	= 257,
    XPRT_TRANSPORT_TCP_TLS	= 258,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_xprt {
    pub /: *mut *mut kref kref; / Reference count,
    pub /: *const *const *const rpc_xprt_ops ops; / transport methods,
    pub /: *mut *mut unsigned int id; / transport id,
    pub /: *const *const *const rpc_timeout timeout; / timeout parms,
    pub /: *mut *mut sockaddr_storage addr; / server address,
    pub /: *mut *mut size_t addrlen; / size of server address,
    pub /: *mut *mut int prot; / IP protocol,
    pub /: *mut *mut unsigned long cong; / current congestion,
    pub /: *mut *mut unsigned long cwnd; / congestion window,
    pub size,: *mut *mut size_t max_payload; / largest RPC payload,
    pub /: *mut *mut rpc_wait_queue binding; / requests waiting on rpcbind,
    pub /: *mut *mut rpc_wait_queue sending; / requests waiting to send,
    pub /: *mut *mut rpc_wait_queue pending; / requests in flight,
    pub /: *mut *mut rpc_wait_queue backlog; / waiting for slot,
    pub /: *mut *mut list_head free; / free slots,
    pub /: *mut *mut unsigned int max_reqs; / max number of slots,
    pub /: *mut *mut unsigned int min_reqs; / min number of slots,
    pub /: *mut *mut unsigned int num_reqs; / total slots,
    pub /: *mut *mut unsigned long state; / transport state,
    pub /: *mut *mut reuseport : 1; / reuse port on reconnect,
    pub this: *mut *mut atomic_t swapper; / we're swapping over,
    pub /: *mut *mut unsigned int bind_index; / bind function index,
//
// Multipath
//
    pub xprt_switch: list_head,
//
// Connection of transports
//
    pub xprtsec: xprtsec_parms,
    pub bumped: *mut *mut unsigned int connect_cookie; / A cookie that gets,
//
// Disconnection of idle transports
//
    pub task_cleanup: work_struct,
    pub timer: timer_list,
//
// Send stuff
//
    pub queuelen: atomic_long_t,
    pub /: *mut *mut spinlock_t transport_lock; / lock transport info,
    pub /: *mut *mut spinlock_t reserve_lock; / lock slot table,
    pub /: *mut *mut spinlock_t queue_lock; / send/receive queue lock,
    pub /: *mut *mut u32 xid; / Next XID value to use,
    pub /: *mut *mut *mut rpc_task  snd_task; / Task blocked in send,
    pub /: *mut *mut list_head xmit_queue; / Send queue,
    pub xmit_queuelen: atomic_long_t,
    pub /: *mut *mut *mut svc_xprt bc_xprt; / NFSv4.1 backchannel,

    pub /: *mut *mut *mut svc_serv bc_serv; / The RPC service which will,
// process the callback
    pub bc_alloc_max: c_uint,
    pub /: *mut *mut unsigned int bc_alloc_count; / Total number of preallocs,
    pub /: *mut *mut atomic_t bc_slot_count; / Number of allocated slots,
    pub preallocated: *mut *mut spinlock_t bc_pa_lock; / Protects the,
// items
    pub preallocated: *mut *mut list_head bc_pa_list; / List of,
// backchannel rpc_rqst's

    pub /: *mut *mut rb_root recv_queue; / Receive queue,
    pub /: *mut *mut max_slots; / max rpc_slots used,
    pub /: *mut *mut pending_u; / pend q utilization,
    pub stat: },
    pub xprt_net: *mut net,
    pub ns_tracker: netns_tracker,
    pub servername: *const c_char,
    pub address_strings: [*const c_char; RPC_DISPLAY_MAX],
    pub /: *mut *mut *mut dentry debugfs; / debugfs directory,

    pub rcu: rcu_head,
    pub xprt_class: *const xprt_class,
    pub xprt_sysfs: *mut rpc_sysfs_xprt,
    pub /: *mut *mut bool main; /mark if this is the 1st transport,
}

//
// Backchannel flags
//
pub const RPC_BC_PA_IN_USE: c_uint = 0x0001		/* Preallocated backchannel */;
// buffer in use

extern "C" {
    pub fn test_bit(_arg: RPC_BC_PA_IN_USE, _arg: &req->rq_bc_pa_state) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xprt_create {
    pub /: *mut *mut int ident; / XPRT_TRANSPORT identifier,
    pub net: *mut *mut net,
    pub /: *mut *mut *mut sockaddr  srcaddr; / optional local address,
    pub /: *mut *mut *mut sockaddr  dstaddr; / remote peer address,
    pub addrlen: usize,
    pub servername: *const c_char,
    pub /: *mut *mut *mut svc_xprt bc_xprt; / NFSv4.1 backchannel,
    pub bc_xps: *mut rpc_xprt_switch,
    pub flags: c_uint,
    pub xprtsec: xprtsec_parms,
    pub connect_timeout: c_ulong,
    pub reconnect_timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xprt_class {
    pub list: list_head,
    pub /: *mut *mut int ident; / XPRT_TRANSPORT identifier,
    pub ): *mut *mut *mut rpc_xprt  (setup)(xprt_create,
    pub owner: *mut module,
    pub name: [c_char; 32],
    pub netid: [*const *const c_char; ],
}

//
// Generic internal transport functions
//
extern "C" {
    pub fn xprt_connect(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_reconnect_delay(xprt: *const rpc_xprt) -> c_ulong;
}
extern "C" {
    pub fn xprt_reserve(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_retry_reserve(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_reserve_xprt(xprt: *mut rpc_xprt, task: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn xprt_reserve_xprt_cong(xprt: *mut rpc_xprt, task: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn xprt_alloc_slot(xprt: *mut rpc_xprt, task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_prepare_transmit(task: *mut rpc_task) -> bool;
}
extern "C" {
    pub fn xprt_request_enqueue_transmit(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_request_enqueue_receive(task: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn xprt_request_wait_receive(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_request_dequeue_xprt(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_request_need_retransmit(task: *mut rpc_task) -> bool;
}
extern "C" {
    pub fn xprt_transmit(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_end_transmit(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_adjust_timeout(req: *mut rpc_rqst) -> c_int;
}
extern "C" {
    pub fn xprt_release_xprt(xprt: *mut rpc_xprt, task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_release_xprt_cong(xprt: *mut rpc_xprt, task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_release(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_get(xprt: *mut rpc_xprt) -> *mut rpc_xprt;
}
extern "C" {
    pub fn xprt_put(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_free(: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_add_backlog(xprt: *mut rpc_xprt, task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_wake_up_backlog(xprt: *mut rpc_xprt, req: *mut rpc_rqst) -> bool;
}
extern "C" {
    pub fn xprt_cleanup_ids();
}
//
// Transport switch helper functions
//
extern "C" {
    pub fn xprt_register_transport(type: *mut xprt_class) -> c_int;
}
extern "C" {
    pub fn xprt_unregister_transport(type: *mut xprt_class) -> c_int;
}
extern "C" {
    pub fn xprt_find_transport_ident(: *const c_char) -> c_int;
}
extern "C" {
    pub fn xprt_wait_for_reply_request_def(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_wait_for_reply_request_rtt(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_wake_pending_tasks(xprt: *mut rpc_xprt, status: c_int);
}
extern "C" {
    pub fn xprt_wait_for_buffer_space(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_write_space(xprt: *mut rpc_xprt) -> bool;
}
extern "C" {
    pub fn xprt_adjust_cwnd(xprt: *mut rpc_xprt, task: *mut rpc_task, result: c_int);
}
extern "C" {
    pub fn xprt_lookup_rqst(xprt: *mut rpc_xprt, xid: __be32) -> *mut rpc_rqst;
}
extern "C" {
    pub fn xprt_update_rtt(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_complete_rqst(task: *mut rpc_task, copied: c_int);
}
extern "C" {
    pub fn xprt_pin_rqst(req: *mut rpc_rqst);
}
extern "C" {
    pub fn xprt_unpin_rqst(req: *mut rpc_rqst);
}
extern "C" {
    pub fn xprt_release_rqst_cong(task: *mut rpc_task);
}
extern "C" {
    pub fn xprt_request_get_cong(xprt: *mut rpc_xprt, req: *mut rpc_rqst) -> bool;
}
extern "C" {
    pub fn xprt_disconnect_done(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_force_disconnect(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_conditional_disconnect(xprt: *mut rpc_xprt, cookie: c_uint);
}
extern "C" {
    pub fn xprt_lock_connect(: *mut rpc_xprt, : *mut rpc_task, : *mut c_void) -> bool;
}
extern "C" {
    pub fn xprt_unlock_connect(: *mut rpc_xprt, : *mut c_void);
}
extern "C" {
    pub fn xprt_release_write(: *mut rpc_xprt, : *mut rpc_task);
}
//
// Reserved bit positions in xprt->state
//

extern "C" {
    pub fn test_bit(_arg: XPRT_CONNECTED, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: XPRT_CONNECTED, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: XPRT_CONNECTED, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn test_bit(_arg: XPRT_CONNECTING, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: XPRT_CONNECTING, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn test_bit(_arg: XPRT_BOUND, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: XPRT_BINDING, _arg: &xprt->state) -> return;
}
extern "C" {
    pub fn xprt_set_offline_locked(xprt: *mut rpc_xprt, xps: *mut rpc_xprt_switch);
}
extern "C" {
    pub fn xprt_set_online_locked(xprt: *mut rpc_xprt, xps: *mut rpc_xprt_switch);
}
extern "C" {
    pub fn xprt_delete_locked(xprt: *mut rpc_xprt, xps: *mut rpc_xprt_switch);
}
