//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/ipset/ip_set.h
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
// Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
// Patrick Schaaf <bof@bof.de>
// Martin Josefsson <gandalf@wlug.westbo.se>
// Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org>
//

// Set features
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_set_feature {
    IPSET_TYPE_IP_FLAG = 0,
    IPSET_TYPE_IP = (1 << IPSET_TYPE_IP_FLAG),
    IPSET_TYPE_PORT_FLAG = 1,
    IPSET_TYPE_PORT = (1 << IPSET_TYPE_PORT_FLAG),
    IPSET_TYPE_MAC_FLAG = 2,
    IPSET_TYPE_MAC = (1 << IPSET_TYPE_MAC_FLAG),
    IPSET_TYPE_IP2_FLAG = 3,
    IPSET_TYPE_IP2 = (1 << IPSET_TYPE_IP2_FLAG),
    IPSET_TYPE_NAME_FLAG = 4,
    IPSET_TYPE_NAME = (1 << IPSET_TYPE_NAME_FLAG),
    IPSET_TYPE_IFACE_FLAG = 5,
    IPSET_TYPE_IFACE = (1 << IPSET_TYPE_IFACE_FLAG),
    IPSET_TYPE_MARK_FLAG = 6,
    IPSET_TYPE_MARK = (1 << IPSET_TYPE_MARK_FLAG),
    IPSET_TYPE_NOMATCH_FLAG = 7,
    IPSET_TYPE_NOMATCH = (1 << IPSET_TYPE_NOMATCH_FLAG),
// Strictly speaking not a feature, but a flag for dumping:
// this settype must be dumped last
    IPSET_DUMP_LAST_FLAG = 8,
    IPSET_DUMP_LAST = (1 << IPSET_DUMP_LAST_FLAG),
}

// Set extensions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_set_extension {
    IPSET_EXT_BIT_TIMEOUT = 0,
    IPSET_EXT_TIMEOUT = (1 << IPSET_EXT_BIT_TIMEOUT),
    IPSET_EXT_BIT_COUNTER = 1,
    IPSET_EXT_COUNTER = (1 << IPSET_EXT_BIT_COUNTER),
    IPSET_EXT_BIT_COMMENT = 2,
    IPSET_EXT_COMMENT = (1 << IPSET_EXT_BIT_COMMENT),
    IPSET_EXT_BIT_SKBINFO = 3,
    IPSET_EXT_SKBINFO = (1 << IPSET_EXT_BIT_SKBINFO),
// Mark set with an extension which needs to call destroy
    IPSET_EXT_BIT_DESTROY = 7,
    IPSET_EXT_DESTROY = (1 << IPSET_EXT_BIT_DESTROY),
}

// Extension id, in size order
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_set_ext_id {
    IPSET_EXT_ID_COUNTER = 0,
    IPSET_EXT_ID_TIMEOUT,
    IPSET_EXT_ID_SKBINFO,
    IPSET_EXT_ID_COMMENT,
    IPSET_EXT_ID_MAX,
}

// Extension type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_ext_type {
// Destroy extension private data (can be NULL)
    pub ext): *mut *mut *mut void (destroy)(struct ip_set set, void,
    pub type: ip_set_extension,
    pub flag: ipset_cadt_flags,
// Size and minimal alignment
    pub len: u8,
    pub align: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_counter {
    pub bytes: core::sync::atomic::AtomicI64,
    pub packets: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_comment_rcu {
    pub rcu: rcu_head,
    pub str: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_comment {
    pub c: *mut ip_set_comment_rcu __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_skbinfo {
    pub skbmark: u32,
    pub skbmarkmask: u32,
    pub skbprio: u32,
    pub skbqueue: u16,
    pub __pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_ext {
    pub skbinfo: ip_set_skbinfo,
    pub packets: u64,
    pub bytes: u64,
    pub comment: *mut c_char,
    pub timeout: u32,
    pub packets_op: u8,
    pub bytes_op: u8,
    pub target: bool,
}

// Kernel API function options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_adt_opt {
    pub /: *mut *mut u8 family; / Actual protocol family,
    pub /: *mut *mut u8 dim; / Dimension of match/target,
    pub /: *mut *mut u8 flags; / Direction and negation flags,
    pub /: *mut *mut u32 cmdflags; / Command-like flags,
    pub /: *mut *mut ip_set_ext ext; / Extensions,
}

// Set type, variant-specific part
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_type_variant {
// Kernelspace: test/add/del entries
// returns negative error code,
// zero for no match/success to add/delete
// positive for matching element
    pub opt): *mut ipset_adt adt, struct ip_set_adt_opt,
// Userspace: test/add/del entries
// returns negative error code,
// zero for no match/success to add/delete
// positive for matching element
    pub retried): *mut *mut ipset_adt adt, u32 lineno, u32 flags, bool,
// Low level add/del/test functions
    pub adt: [ipset_adtfn; IPSET_ADT_MAX],
// When adding entries and set is full, try to resize the set
    pub retried): *mut *mut *mut int (resize)(struct ip_set set, bool,
// Destroy the set
    pub set): *mut *mut void (destroy)(struct ip_set,
// Flush the elements
    pub set): *mut *mut void (flush)(struct ip_set,
// Expire entries before listing
    pub set): *mut *mut void (expire)(struct ip_set,
// List set header data
    pub skb): *mut *mut *mut int (head)(struct ip_set set, struct sk_buff,
// List elements
    pub cb): *mut netlink_callback,
// Keep listing private when resizing runs parallel
    pub start): bool,
// Return true if "b" set is the same as "a"
// according to the create set parameters
    pub b): *const *const *const bool (same_set)(struct ip_set a, struct ip_set,
// Cancel ongoing garbage collectors before destroying the set
    pub set): *mut *mut void (cancel_gc)(struct ip_set,
// Region-locking is used
    pub region_lock: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_region {
    pub /: *mut *mut spinlock_t lock; / Region lock,
    pub /: *mut *mut size_t ext_size; / Size of the dynamic extensions,
    pub /: *mut *mut u32 elements; / Number of elements vs timeout,
}

// Max range where every element is added/deleted in one step

// The max revision number supported by any set type + 1
pub const IPSET_REVISION_MAX: c_int = 9;
// The core set type structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set_type {
    pub list: list_head,
// Typename
    pub name: [c_char; IPSET_MAXNAMELEN],
// Protocol version
    pub protocol: u8,
// Set type dimension
    pub dimension: u8,
//
// Supported family: may be NFPROTO_UNSPEC for both
// NFPROTO_IPV4/NFPROTO_IPV6.
//
    pub family: u8,
// Type revisions
    pub revision_max: u8 revision_min,,
// Revision-specific supported (create) flags
    pub create_flags: [u8; IPSET_REVISION_MAX+1],
// Set features to control swapping
    pub features: u16,
// Create set
    pub flags): *mut *mut nlattr tb[], u32,
// Attribute policies
    pub 1]: nla_policy create_policy[IPSET_ATTR_CREATE_MAX +,
    pub 1]: nla_policy adt_policy[IPSET_ATTR_ADT_MAX +,
// Set this to THIS_MODULE if you are a module, otherwise NULL
    pub me: *mut module,
}

// register and unregister set type
extern "C" {
    pub fn ip_set_type_register(set_type: *mut ip_set_type) -> c_int;
}
extern "C" {
    pub fn ip_set_type_unregister(set_type: *mut ip_set_type);
}
// A generic IP set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_set {
// for set destruction
    pub rwork: rcu_work,
// The name of the set
    pub name: [c_char; IPSET_MAXNAMELEN],
// Lock protecting the set data
    pub lock: spinlock_t,
// References to the set
    pub ref: u32,
// References to the set for netlink events like dump,
// ref can be swapped out by ip_set_swap
//
    pub ref_netlink: u32,
// The core set type
    pub type: *mut ip_set_type,
// The type variant doing the real job
    pub variant: *const ip_set_type_variant,
// The actual INET family of the set
    pub family: u8,
// The type revision
    pub revision: u8,
// Extensions
    pub extensions: u8,
// Create flags
    pub flags: u8,
// Default timeout value, if enabled
    pub timeout: u32,
// Number of elements (vs timeout)
    pub elements: u32,
// Size of the dynamic extensions (vs timeout)
    pub ext_size: core::sync::atomic::AtomicI64,
// Element data size
    pub dsize: usize,
// Offsets to extensions in elements
    pub offset: [usize; IPSET_EXT_ID_MAX],
// The type specific data
    pub data: *mut c_void,
}

// Check that the extension is enabled for the set and
// call it's destroy function for its extension part in data.
//
extern "C" {
    pub fn ip_set_put_flags(skb: *mut sk_buff, set: *mut ip_set) -> c_int;
}
// Netlink CB args
// register and unregister set references
extern "C" {
    pub fn ip_set_put_byindex(net: *mut net, index: ip_set_id_t);
}
extern "C" {
    pub fn ip_set_name_byindex(net: *mut net, index: ip_set_id_t, name: *mut c_char);
}
extern "C" {
    pub fn ip_set_nfnl_get_byindex(net: *mut net, index: ip_set_id_t) -> ip_set_id_t;
}
extern "C" {
    pub fn ip_set_nfnl_put(net: *mut net, index: ip_set_id_t);
}
// API for iptables set match, and SET target
// Utility functions
extern "C" {
    pub fn ip_set_free(members: *mut c_void);
}
extern "C" {
    pub fn ip_set_get_ipaddr4(nla: *mut nlattr, ipaddr: *mut __be32) -> c_int;
}
extern "C" {
    pub fn ip_set_get_ipaddr6(nla: *mut nlattr, ipaddr: *mut nf_inet_addr) -> c_int;
}
// ipaddr = ntohl(ip);
// Ignore IPSET_ERR_EXIST errors if asked to do so?
// Match elements marked with nomatch
// Check the NLA_F_NET_BYTEORDER flag
// Useful converters
extern "C" {
    pub fn ntohl(_arg: nla_get_be32(attr)) -> return;
}
extern "C" {
    pub fn ntohs(_arg: nla_get_be16(attr)) -> return;
}
// Get address from skbuff
// addr = src ? ip_hdr(skb)->saddr : ip_hdr(skb)->daddr;
// How often should the gc be run by default

// Timeout period depending on the timeout value of the given set

// Entry is set with no timeout value
pub const IPSET_ELEM_PERMANENT: c_int = 0;
// Set is defined with timeout support: timeout value may be 0

// Max timeout value, see msecs_to_jiffies() in jiffies.h

// Normalize to fit into jiffies
// timeout = IPSET_ELEM_PERMANENT;
// Bingo! :-)
// timeout = t;
// skbinfo = ext->skbinfo;

