//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mctp.h
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
// Management Component Transport Protocol (MCTP)
//
// Copyright (c) 2021 Code Construct
// Copyright (c) 2021 Google
//

// MCTP packet definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_hdr {
    pub ver: u8,
    pub dest: u8,
    pub src: u8,
    pub flags_seq_tag: u8,
}

pub const MCTP_VER_MIN: c_int = 1;
pub const MCTP_VER_MAX: c_int = 1;
// Definitions for ver field

// Definitions for flags_seq_tag field

pub const MCTP_HDR_SEQ_SHIFT: c_int = 4;

pub const MCTP_HDR_TAG_SHIFT: c_int = 0;

pub const MCTP_INITIAL_DEFAULT_NET: c_int = 1;
// socket implementation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_sock {
    pub sk: sock,
// bind() params
    pub bind_net: c_uint,
    pub bind_local_addr: mctp_eid_t,
    pub bind_peer_addr: mctp_eid_t,
    pub bind_peer_net: c_uint,
    pub bind_peer_set: bool,
    pub bind_type: __u8,
// sendmsg()/recvmsg() uses struct sockaddr_mctp_ext
    pub addr_ext: bool,
// list of mctp_sk_key, for incoming tag lookup. updates protected
// by sk->net->keys_lock
//
    pub keys: hlist_head,
// mechanism for expiring allocated keys; will release an allocated
// tag, and any netdev state for a request/response pairing
//
    pub key_expiry: timer_list,
}

// Key for matching incoming packets to sockets or reassembly contexts.
// Packets are matched on (peer EID, local EID, tag).
//
// Lifetime / locking requirements:
//
// - individual key data (ie, the struct itself) is protected by key->lock;
// changes must be made with that lock held.
//
// - the lookup fields: peer_addr, local_addr and tag are set before the
// key is added to lookup lists, and never updated.
//
// - A ref to the key must be held (throuh key->refs) if a pointer to the
// key is to be accessed after key->lock is released.
//
// - a mctp_sk_key contains a reference to a struct sock; this is valid
// for the life of the key. On sock destruction (through unhash), the key is
// removed from lists (see below), and marked invalid.
//
// - these mctp_sk_keys appear on two lists:
// 1) the struct mctp_sock->keys list
// 2) the struct netns_mctp->keys list
//
// presences on these lists requires a (single) refcount to be held; both
// lists are updated as a single operation.
//
// Updates and lookups in either list are performed under the
// netns_mctp->keys lock. Lookup functions will need to lock the key and
// take a reference before unlocking the keys_lock. Consequently, the list's
// keys_lock *cannot* be acquired with the individual key->lock held.
//
// - a key may have a sk_buff attached as part of an in-progress message
// reassembly (->reasm_head). The reasm data is protected by the individual
// key->lock.
//
// - there are two destruction paths for a mctp_sk_key:
//
// - through socket unhash (see mctp_sk_unhash). This performs the list
// removal under keys_lock.
//
// - where a key is established to receive a reply message: after receiving
// the (complete) reply, or during reassembly errors. Here, we clean up
// the reassembly context (marking reasm_dead, to prevent another from
// starting), and remove the socket from the netns & socket lists.
//
// - through an expiry timeout, on a per-socket timer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_sk_key {
    pub net: c_uint,
    pub peer_addr: mctp_eid_t,
    pub /: *mut *mut mctp_eid_t local_addr; / MCTP_ADDR_ANY for local owned tags,
    pub /: *mut *mut __u8 tag; / incoming tag match; invert TO for local,
// we hold a ref to sk when set
    pub sk: *mut sock,
// routing lookup list
    pub hlist: hlist_node,
// per-socket list
    pub sklist: hlist_node,
// lock protects against concurrent updates to the reassembly and
// expiry data below.
//
    pub lock: spinlock_t,
// Keys are referenced during the output path, which may sleep
    pub refs: refcount_t,
// incoming fragment reassembly context
    pub reasm_head: *mut sk_buff,
    pub reasm_tailp: *mut sk_buff,
    pub reasm_dead: bool,
    pub last_seq: u8,
// key validity
    pub valid: bool,
// expiry timeout; valid (above) cleared on expiry
    pub expiry: c_ulong,
// free to use for device flow state tracking. Initialised to
// zero on initial key creation
//
    pub dev_flow_state: c_ulong,
    pub dev: *mut mctp_dev,
// a tag allocated with SIOCMCTPALLOCTAG ioctl will not expire
// automatically on timeout or response, instead SIOCMCTPDROPTAG
// is used.
//
    pub manual_alloc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_skb_cb {
    pub magic: c_uint,
    pub net: c_uint,
// fields below provide extended addressing for ingress to recvmsg()
    pub ifindex: c_int,
    pub halen: c_uchar,
    pub haddr: [c_uchar; MAX_ADDR_LEN],
}

// skb control-block accessors with a little extra debugging for initial
// development.
//
// TODO: remove checks & mctp_skb_cb->magic; replace callers of __mctp_cb
// with mctp_cb().
//
// __mctp_cb() is only for the initial ingress code; we should see ->magic set
// at all times after this.
//
// If CONFIG_MCTP_FLOWS, we may add one of these as a SKB extension,
// indicating the flow to the device driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_flow {
    pub key: *mut mctp_sk_key,
}

// Route definition.
//
// These are held in the pernet->mctp.routes list, with RCU protection for
// removed routes. We hold a reference to the netdev; routes need to be
// dropped on NETDEV_UNREGISTER events.
//
// Updates to the route table are performed under rtnl; all reads under RCU,
// so routes cannot be referenced over a RCU grace period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_route {
    pub max: mctp_eid_t min,,
    pub type: c_uchar,
    pub mtu: c_uint,
    pub dst_type: },
    pub dev: *mut mctp_dev,
    pub gateway: mctp_fq_addr,
}

// Route lookup result: dst. Represents the results of a routing decision,
// but is only held over the individual routing operation.
//
// Will typically be stored on the caller stack, and must be released after
// usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_dst {
    pub dev: *mut mctp_dev,
    pub mtu: c_uint,
    pub nexthop: mctp_eid_t,
    pub saddr: mctp_eid_t,
// set for direct addressing
    pub halen: c_uchar,
    pub haddr: [c_uchar; MAX_ADDR_LEN],
    pub skb): *mut *mut *mut int (output)(struct mctp_dst dst, struct sk_buff,
}

// route interfaces
extern "C" {
    pub fn mctp_dst_release(dst: *mut mctp_dst);
}
// always takes ownership of skb
extern "C" {
    pub fn mctp_key_unref(key: *mut mctp_sk_key);
}
// routing <--> device interface
extern "C" {
    pub fn mctp_default_net(net: *mut net) -> c_uint;
}
extern "C" {
    pub fn mctp_default_net_set(net: *mut net, index: c_uint) -> c_int;
}
extern "C" {
    pub fn mctp_route_add_local(mdev: *mut mctp_dev, addr: mctp_eid_t) -> c_int;
}
extern "C" {
    pub fn mctp_route_remove_local(mdev: *mut mctp_dev, addr: mctp_eid_t) -> c_int;
}
extern "C" {
    pub fn mctp_route_remove_dev(mdev: *mut mctp_dev);
}
// neighbour definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mctp_neigh_source {
    MCTP_NEIGH_STATIC,
    MCTP_NEIGH_DISCOVER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_neigh {
    pub dev: *mut mctp_dev,
    pub eid: mctp_eid_t,
    pub source: mctp_neigh_source,
    pub ha: [c_uchar; MAX_ADDR_LEN],
    pub list: list_head,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn mctp_neigh_init() -> c_int;
}
extern "C" {
    pub fn mctp_neigh_exit();
}
// ret_hwaddr may be NULL, otherwise must have space for MAX_ADDR_LEN
extern "C" {
    pub fn mctp_neigh_remove_dev(mdev: *mut mctp_dev);
}
extern "C" {
    pub fn mctp_routes_init() -> c_int;
}
extern "C" {
    pub fn mctp_routes_exit();
}
extern "C" {
    pub fn mctp_device_init() -> c_int;
}
extern "C" {
    pub fn mctp_device_exit();
}
// MCTP IDs and Codes from DMTF specification
// "DSP0239 Management Component Transport Protocol (MCTP) IDs and Codes"
// https://www.dmtf.org/sites/default/files/standards/documents/DSP0239_1.11.1.pdf
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mctp_phys_binding {
    MCTP_PHYS_BINDING_UNSPEC	= 0x00,
    MCTP_PHYS_BINDING_SMBUS		= 0x01,
    MCTP_PHYS_BINDING_PCIE_VDM	= 0x02,
    MCTP_PHYS_BINDING_USB		= 0x03,
    MCTP_PHYS_BINDING_KCS		= 0x04,
    MCTP_PHYS_BINDING_SERIAL	= 0x05,
    MCTP_PHYS_BINDING_I3C		= 0x06,
    MCTP_PHYS_BINDING_MMBI		= 0x07,
    MCTP_PHYS_BINDING_PCC		= 0x08,
    MCTP_PHYS_BINDING_UCIE		= 0x09,
    MCTP_PHYS_BINDING_VENDOR	= 0xFF,
}
