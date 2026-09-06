//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_tuple.h
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
// Definitions and Declarations for tuple.
//
// 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
// - generalize L3 protocol dependent part.
//
// Derived from include/linux/netfiter_ipv4/ip_conntrack_tuple.h
//

// A `tuple' is a structure containing the information to uniquely
//

// The manipulable part of the tuple.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_man_proto,
// Layer 3 protocol
    pub l3num: u_int16_t,
}

// This contains the information to distinguish a connection.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_man,
// These are the parts of the tuple which are fixed.
    pub u3: nf_inet_addr,
// Add other protocols here.
    pub all: __be16,
    pub port: __be16,
    pub tcp: },
    pub port: __be16,
    pub udp: },
    pub code: u_int8_t type,,
    pub icmp: },
    pub port: __be16,
    pub dccp: },
    pub port: __be16,
    pub sctp: },
    pub key: __be16,
    pub gre: },
    pub u: },
// The protocol.
    pub protonum: u_int8_t,
// The direction must be ignored for the tuplehash
    pub __nfct_hash_offsetend: { },
// The direction (for tuplehash)
    pub dir: u_int8_t,
    pub dst: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_mask {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_man_proto,
    pub src: },
}

// If we're the first tuple, it's the original dir.

// Connections have two entries in the hash table: one for each way
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_hash {
    pub hnnode: hlist_nulls_node,
    pub tuple: nf_conntrack_tuple,
}
