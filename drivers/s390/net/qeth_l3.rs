//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/qeth_l3.h
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
// Copyright IBM Corp. 2007
// Author(s): Utz Bacher <utz.bacher@de.ibm.com>,
// Frank Pavlic <fpavlic@de.ibm.com>,
// Thomas Spatzier <tspat@de.ibm.com>,
// Frank Blaschka <frank.blaschka@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_ip_types {
    QETH_IP_TYPE_NORMAL,
    QETH_IP_TYPE_VIPA,
    QETH_IP_TYPE_RXIP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipaddr {
    pub hnode: hlist_node,
    pub type: qeth_ip_types,
    pub is_multicast:1: u8,
    pub disp_flag:2: u8,
    pub /: *mut *mut u8 ipato:1; / ucast only,
// is changed only for normal ip addresses
// for non-normal addresses it always is  1
//
    pub ref_counter: c_int,
    pub proto: qeth_prot_versions,
    pub addr: __be32,
    pub mask: __be32,
    pub a4: },
    pub addr: in6_addr,
    pub pfxlen: c_uint,
    pub a6: },
    pub u: },
}

extern "C" {
    pub fn ipv6_addr_equal(_arg: &a1->u.a6.addr, _arg: &a2->u.a6.addr) -> return;
}
// Assumes that the pair was obtained via qeth_l3_addr_find_by_ip(),
// so 'proto' and 'addr' match for sure.
//
// For ucast:
// -	'mask'/'pfxlen' for RXIP/VIPA is always 0. For NORMAL, matching
// values are required to avoid mixups in takeover eligibility.
//
// For mcast,
// -	'mask'/'pfxlen' is always 0.
//
extern "C" {
    pub fn ipv6_addr_hash(_arg: &addr->u.a6.addr) -> return;
}
extern "C" {
    pub fn ipv4_addr_hash(_arg: addr->u.a4.addr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipato_entry {
    pub entry: list_head,
    pub proto: qeth_prot_versions,
    pub addr: [c_char; 16],
    pub mask_bits: c_uint,
}

extern "C" {
    pub fn qeth_l3_setrouting_v4(: *mut qeth_card) -> c_int;
}
extern "C" {
    pub fn qeth_l3_setrouting_v6(: *mut qeth_card) -> c_int;
}
extern "C" {
    pub fn qeth_l3_add_ipato_entry(: *mut qeth_card, : *mut qeth_ipato_entry) -> c_int;
}
extern "C" {
    pub fn qeth_l3_update_ipato(card: *mut qeth_card);
}
extern "C" {
    pub fn qeth_l3_modify_hsuid(card: *mut qeth_card, add: bool) -> c_int;
}
