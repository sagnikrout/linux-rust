//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/flow.h
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
// Generic internet FLOW.
//

//
// ifindex generation is per-net namespace, and loopback is
// always the 1st device in ns (see net_dev_init), thus any
// loopback device should get ifindex 1
//
pub const LOOPBACK_IFINDEX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi_tunnel {
    pub tun_id: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi_common {
    pub flowic_oif: c_int,
    pub flowic_iif: c_int,
    pub flowic_l3mdev: c_int,
    pub flowic_mark: __u32,
    pub flowic_dscp: dscp_t,
    pub flowic_scope: __u8,
    pub flowic_proto: __u8,
    pub flowic_flags: __u8,
pub const FLOWI_FLAG_ANYSRC: c_uint = 0x01;
pub const FLOWI_FLAG_KNOWN_NH: c_uint = 0x02;
pub const FLOWI_FLAG_L3MDEV_OIF: c_uint = 0x04;
pub const FLOWI_FLAG_ANY_SPORT: c_uint = 0x08;
    pub flowic_secid: __u32,
    pub flowic_uid: kuid_t,
    pub flowic_multipath_hash: __u32,
    pub flowic_tun_key: flowi_tunnel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union flowi_uli {
    pub dport: __be16,
    pub sport: __be16,
    pub ports: },
    pub type: __u8,
    pub code: __u8,
    pub icmpt: },
    pub gre_key: __be32,
    pub type: __u8,
    pub mht: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi4 {
    pub __fl_common: flowi_common,

// (saddr,daddr) must be grouped, same order as in IP header
    pub saddr: __be32,
    pub daddr: __be32,
    pub uli: flowi_uli,

    pub __attribute__((__aligned__(BITS_PER_LONG/8))): },
    pub oif: fl4->flowi4_oif =,
    pub LOOPBACK_IFINDEX: fl4->flowi4_iif =,
    pub 0: fl4->flowi4_l3mdev =,
    pub mark: fl4->flowi4_mark =,
    pub inet_dsfield_to_dscp(tos): fl4->flowi4_dscp =,
    pub scope: fl4->flowi4_scope =,
    pub proto: fl4->flowi4_proto =,
    pub flags: fl4->flowi4_flags =,
    pub 0: fl4->flowi4_secid =,
    pub 0: fl4->flowi4_tun_key.tun_id =,
    pub uid: fl4->flowi4_uid =,
    pub daddr: fl4->daddr =,
    pub saddr: fl4->saddr =,
    pub dport: fl4->fl4_dport =,
    pub sport: fl4->fl4_sport =,
    pub 0: fl4->flowi4_multipath_hash =,
// Reset some input parameters after previous lookup
    pub oif: fl4->flowi4_oif =,
    pub daddr: fl4->daddr =,
    pub saddr: fl4->saddr =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub __fl_common: flowi_common,

    pub daddr: in6_addr,
    pub saddr: in6_addr,
// Note: flowi6_dscp is encoded in flowlabel, too.
    pub flowlabel: __be32,
    pub uli: flowi_uli,

    pub mp_hash: __u32,
    pub __attribute__((__aligned__(BITS_PER_LONG/8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi {
    pub __fl_common: flowi_common,
    pub ip4: flowi4,
    pub ip6: flowi6,
    pub u: },

    pub __attribute__((__aligned__(BITS_PER_LONG/8))): },
    pub u.ip4): return container_of(fl4, struct flowi,,
    pub &(fl4->__fl_common): return,
    pub u.ip6): return container_of(fl6, struct flowi,,
    pub &(fl6->__fl_common): return,
    pub keys): *const *const __u32 __get_hash_from_flowi6(struct flowi6 fl6, struct flow_keys,
