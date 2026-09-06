//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/sctp.h
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
// Copyright (c) 2001-2003 Intel Corp.
//
// This file is part of the SCTP kernel implementation
//
// The base lksctp header.
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// La Monte H.P. Yarroll <piggy@acm.org>
// Xingang Guo           <xingang.guo@intel.com>
// Jon Grimm             <jgrimm@us.ibm.com>
// Daisy Chang           <daisyc@us.ibm.com>
// Sridhar Samudrala     <sri@us.ibm.com>
// Ardelle Fan           <ardelle.fan@intel.com>
// Ryan Layer            <rmlayer@us.ibm.com>
// Kevin Gao             <kevin.gao@intel.com>
//

// Macro flag: #define __net_sctp_h__
// Header Strategy.
// Start getting some control over the header file dependencies:
// includes
// constants
// structs
// prototypes
// macros, externs, and inlines
//
// Move test_frame specific items out of the kernel headers
// and into the test frame headers.   This is not perfect in any sense
// and will continue to evolve.
//

pub const SCTP_PROTOSW_FLAG: c_int = 0;

//
// Function declarations.
//
// sctp/protocol.c
//
extern "C" {
    pub fn sctp_register_pf(: *mut sctp_pf, _arg: sa_family_t) -> c_int;
}
extern "C" {
    pub fn sctp_addr_wq_mgmt(: *mut net, : *mut sctp_sockaddr_entry, _arg: c_int);
}
extern "C" {
    pub fn sctp_udp_sock_start(net: *mut net) -> c_int;
}
extern "C" {
    pub fn sctp_udp_sock_stop(net: *mut net);
}
//
// sctp/socket.c
//
extern "C" {
    pub fn sctp_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn sctp_inet_listen(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn sctp_write_space(sk: *mut sock);
}
extern "C" {
    pub fn sctp_data_ready(sk: *mut sock);
}
extern "C" {
    pub fn sctp_sock_rfree(skb: *mut sk_buff);
}
extern "C" {
    pub fn sctp_asconf_mgmt(: *mut sctp_sock, : *mut sctp_sockaddr_entry) -> c_int;
}
extern "C" {
    pub fn int(: *mut *mut sctp_callback_t)(struct sctp_endpoint, : *mut sctp_transport, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn sctp_transport_walk_start(iter: *mut rhashtable_iter);
}
extern "C" {
    pub fn sctp_transport_walk_stop(iter: *mut rhashtable_iter);
}
//
// sctp/primitive.c
//
extern "C" {
    pub fn sctp_primitive_ASSOCIATE(: *mut net, : *mut sctp_association, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sctp_primitive_SHUTDOWN(: *mut net, : *mut sctp_association, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sctp_primitive_ABORT(: *mut net, : *mut sctp_association, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sctp_primitive_SEND(: *mut net, : *mut sctp_association, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sctp_primitive_REQUESTHEARTBEAT(: *mut net, : *mut sctp_association, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sctp_primitive_ASCONF(: *mut net, : *mut sctp_association, arg: *mut c_void) -> c_int;
}
//
// sctp/input.c
//
extern "C" {
    pub fn sctp_rcv(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn sctp_v4_err(skb: *mut sk_buff, info: u32) -> c_int;
}
extern "C" {
    pub fn sctp_hash_endpoint(ep: *mut sctp_endpoint) -> c_int;
}
extern "C" {
    pub fn sctp_unhash_endpoint(: *mut sctp_endpoint);
}
extern "C" {
    pub fn sctp_err_finish(: *mut sock, : *mut sctp_transport);
}
extern "C" {
    pub fn sctp_udp_v4_err(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn sctp_udp_v6_err(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn sctp_transport_hashtable_init() -> c_int;
}
extern "C" {
    pub fn sctp_transport_hashtable_destroy();
}
extern "C" {
    pub fn sctp_hash_transport(t: *mut sctp_transport) -> c_int;
}
extern "C" {
    pub fn sctp_unhash_transport(t: *mut sctp_transport);
}
extern "C" {
    pub fn sctp_sk_bound_dev_eq(net: *mut net, bound_dev_if: c_int, dif: c_int, sdif: c_int) -> bool;
}
//
// sctp/proc.c
//
extern "C" {
    pub fn sctp_proc_init(net: *mut net) -> int __net_init;
}
//
// sctp/offload.c
//
extern "C" {
    pub fn sctp_offload_init() -> c_int;
}
//
// sctp/stream_sched.c
//
extern "C" {
    pub fn sctp_sched_ops_init();
}
//
// sctp/stream.c
//
extern "C" {
    pub fn sctp_send_reset_assoc(asoc: *mut sctp_association) -> c_int;
}
//
// Module global variables
//
// sctp/protocol.c
//
// Section:  Macros, externs, and inlines
//
// SCTP SNMP MIB stats handlers

// sctp mib definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_mib {
    pub mibs: [c_ulong; SCTP_MIB_MAX],
}

// helper function to track stats about max rto and related transport
//
// Macros for keeping a global reference of object allocations.
//

// Macros to atomically increment/decrement objcnt counters.

// Macro to help create new entries in the global array of
// objcnt counters.
//

extern "C" {
    pub fn sctp_dbg_objcnt_init(: *mut net);
}

// Macro flag: #define SCTP_DBG_OBJCNT_INC(name)
// Macro flag: #define SCTP_DBG_OBJCNT_DEC(name)

extern "C" {
    pub fn sctp_sysctl_register();
}
extern "C" {
    pub fn sctp_sysctl_unregister();
}
extern "C" {
    pub fn sctp_sysctl_net_register(net: *mut net) -> c_int;
}
extern "C" {
    pub fn sctp_sysctl_net_unregister(net: *mut net);
}

// Size of Supported Address Parameter for 'x' address types.

extern "C" {
    pub fn sctp_v6_pf_init();
}
extern "C" {
    pub fn sctp_v6_pf_exit();
}
extern "C" {
    pub fn sctp_v6_protosw_init() -> c_int;
}
extern "C" {
    pub fn sctp_v6_protosw_exit();
}
extern "C" {
    pub fn sctp_v6_add_protocol() -> c_int;
}
extern "C" {
    pub fn sctp_v6_del_protocol();
}

// Map an association to an assoc_id.
// SCTP's uapi always had SCTP_EMPTY(=0) as a dummy state, but we
// got rid of it in kernel space. Therefore SCTP_CLOSED et al
// start at =1 in user space, but actually as =0 in kernel space.
// Now that we can not break user space and SCTP_EMPTY is exposed
// there, we need to fix it up with an ugly offset not to break
// applications. :(
//
// Look up the association by its id.
// A macro to walk a list of skbs.

//
// sctp_list_dequeue - remove from the head of the queue
// @list: list to dequeue from
//
// Remove the head of the list. The head item is
// returned or %NULL if the list is empty.
//
// SCTP version of skb_set_owner_r.  We need this one because
// of the way we have to do receive buffer accounting on bundled
// chunks.
//
// This mimics the behavior of skb_set_owner_r
//
// Tests if the list has one and only one entry.
extern "C" {
    pub fn list_is_singular(_arg: head) -> return;
}
// Walk through a list of TLV parameters.  Don't trust the
// individual parameter lengths and instead depend on
// the chunk length to indicate when to stop.  Make sure
// there is room for a param header too.
//

// External references.
extern "C" {
    pub fn sctp_put_port(sk: *mut sock);
}
// Static inline functions.
// Convert from an IP version number to an Address Family symbol.
// Convert from an address parameter type to an address family.
// Warning: The following hash functions assume a power of two 'size'.
// This is the hash function for the SCTP port hash table.
// This is the hash function for the endpoint hash table.

// Is a socket of this style?

// Is the association in this state?

// Is the socket in this state?

// Map v4-mapped v6 address back to v4 address
// Map v4 address to v4-mapped v6 address
// The cookie is always 0 since this is how it's used in the
// pmtu code.
//
// Calculate max payload size given a MTU, or the total overhead if
// given MTU is zero
//
extern "C" {
    pub fn __sctp_mtu_payload(_arg: sp, _arg: NULL, _arg: mtu, _arg: extra) -> return;
}
extern "C" {
    pub fn sctp_mtu_payload(_arg: sp, _arg: SCTP_DEFAULT_MINSEGMENT, _arg: datasize) -> return;
}
