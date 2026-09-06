//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/svc_xprt.h
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
// linux/include/linux/sunrpc/svc_xprt.h
//
// RPC server transport I/O
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_xprt_ops {
    pub ): *mut *mut *mut svc_xprt (xpo_accept)(svc_xprt,
    pub ): *mut *mut int (xpo_has_wspace)(struct svc_xprt,
    pub ): *mut *mut int (xpo_recvfrom)(struct svc_rqst,
    pub ): *mut *mut int (xpo_sendto)(struct svc_rqst,
    pub int): unsigned,
    pub ctxt): *mut *mut *mut void (xpo_release_ctxt)(struct svc_xprt xprt, void,
    pub ): *mut *mut void (xpo_detach)(struct svc_xprt,
    pub ): *mut *mut void (xpo_free)(struct svc_xprt,
    pub ): *mut *mut void (xpo_kill_temp_xprt)(struct svc_xprt,
    pub xprt): *mut *mut void (xpo_handshake)(struct svc_xprt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_xprt_class {
    pub xcl_name: *const c_char,
    pub xcl_owner: *mut module,
    pub xcl_ops: *const svc_xprt_ops,
    pub xcl_list: list_head,
    pub xcl_max_payload: u32,
    pub xcl_ident: c_int,
}

//
// This is embedded in an object that wants a callback before deleting
// an xprt; intended for use by NFSv4.1, which needs to know when a
// client's tcp connection (and hence possibly a backchannel) goes away.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_xpt_user {
    pub list: list_head,
    pub ): *mut *mut void (callback)(struct svc_xpt_user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_xprt {
    pub xpt_class: *mut svc_xprt_class,
    pub xpt_ops: *const svc_xprt_ops,
    pub xpt_ref: kref,
    pub xpt_qtime: ktime_t,
    pub xpt_list: list_head,
    pub xpt_ready: lwq_node,
    pub xpt_flags: c_ulong,
    pub /: *mut *mut *mut svc_serv xpt_server; / service for transport,
    pub /: *mut *mut atomic_t xpt_reserved; / space on outq that is rsvd,
    pub /: *mut *mut atomic_t xpt_nr_rqsts; / Number of requests,
    pub /: *mut *mut mutex xpt_mutex; / to serialize sending data,
    pub sk_deferred: *mut *mut spinlock_t xpt_lock; / protects,
// and xpt_auth_cache
    pub /: *mut *mut *mut void xpt_auth_cache;/ auth cache,
    pub need: *mut *mut list_head xpt_deferred; / deferred requests that,
// to be revisted
    pub /: *mut *mut sockaddr_storage xpt_local; / local address,
    pub /: *mut *mut size_t xpt_locallen; / length of address,
    pub /: *mut *mut sockaddr_storage xpt_remote; / remote peer's address,
    pub /: *mut *mut size_t xpt_remotelen; / length of address,
    pub 10]: char xpt_remotebuf[INET6_ADDRSTRLEN +,
    pub /: *mut *mut list_head xpt_users; / callbacks on free,
    pub xpt_net: *mut net,
    pub ns_tracker: netns_tracker,
    pub xpt_cred: *const cred,
    pub /: *mut *mut *mut rpc_xprt xpt_bc_xprt; / NFSv4.1 backchannel,
    pub /: *mut *mut *mut rpc_xprt_switch xpt_bc_xps; / NFSv4.1 backchannel,
}

// flag bits for xpt_flags
// it has access to.  It is NOT counted
// in ->sv_tmpcnt.
//
// with rpcbind (TCP, UDP) on destroy
//
// Maximum number of "tmp" connections - those without XPT_PEER_VALID -
// permitted on any service.
//
pub const XPT_MAX_TMP_CONN: c_int = 64;
//
// The connection is about to be deleted soon (or,
// worse, may already be deleted--in which case we've
// already notified the xpt_users).
//
extern "C" {
    pub fn svc_reg_xprt_class(: *mut svc_xprt_class) -> c_int;
}
extern "C" {
    pub fn svc_unreg_xprt_class(: *mut svc_xprt_class);
}
extern "C" {
    pub fn svc_xprt_received(xprt: *mut svc_xprt);
}
extern "C" {
    pub fn svc_xprt_enqueue(xprt: *mut svc_xprt);
}
extern "C" {
    pub fn svc_xprt_put(xprt: *mut svc_xprt);
}
extern "C" {
    pub fn svc_xprt_copy_addrs(rqstp: *mut svc_rqst, xprt: *mut svc_xprt);
}
extern "C" {
    pub fn svc_xprt_close(xprt: *mut svc_xprt);
}
extern "C" {
    pub fn svc_port_is_privileged(sin: *mut sockaddr) -> c_int;
}
extern "C" {
    pub fn svc_print_xprts(buf: *mut c_char, maxlen: c_int) -> c_int;
}
extern "C" {
    pub fn svc_xprt_names(serv: *mut svc_serv, buf: *mut c_char, buflen: c_int) -> c_int;
}
extern "C" {
    pub fn svc_add_new_perm_xprt(serv: *mut svc_serv, xprt: *mut svc_xprt);
}
extern "C" {
    pub fn svc_age_temp_xprts_now(: *mut svc_serv, : *mut sockaddr);
}
extern "C" {
    pub fn svc_xprt_deferred_close(xprt: *mut svc_xprt);
}
extern "C" {
    pub fn ntohs(_arg: sin->sin_port) -> return;
}
extern "C" {
    pub fn ntohs(_arg: sin6->sin6_port) -> return;
}
extern "C" {
    pub fn sizeof(sockaddr_in: struct) -> return;
}
extern "C" {
    pub fn sizeof(sockaddr_in6: struct) -> return;
}
extern "C" {
    pub fn svc_addr_port()&xprt->xpt_local: *const (struct sockaddr) -> return;
}
extern "C" {
    pub fn svc_addr_port()&xprt->xpt_remote: *const (struct sockaddr) -> return;
}
