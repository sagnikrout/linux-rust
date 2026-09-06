//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/if_inet6.h
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
//
// inet6 interface/address list definitions
// Linux INET6 implementation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
//

// inet6_dev.if_flags
pub const IF_RA_OTHERCONF: c_uint = 0x80;
pub const IF_RA_MANAGED: c_uint = 0x40;
pub const IF_RA_RCVD: c_uint = 0x20;
pub const IF_RS_SENT: c_uint = 0x10;
pub const IF_READY: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_ifaddr {
    pub addr: in6_addr,
    pub prefix_len: __u32,
    pub rt_priority: __u32,
// In seconds, relative to tstamp. Expiry is at tstamp + HZ * lft.
    pub valid_lft: __u32,
    pub prefered_lft: __u32,
    pub refcnt: refcount_t,
    pub lock: spinlock_t,
    pub state: c_int,
    pub flags: __u32,
    pub dad_probes: __u8,
    pub stable_privacy_retry: __u8,
    pub scope: __u16,
    pub dad_nonce: __u64,
    pub /: *mut *mut unsigned long cstamp; / created timestamp,
    pub /: *mut *mut unsigned long tstamp; / updated timestamp,
    pub dad_work: delayed_work,
    pub idev: *mut inet6_dev,
    pub rt: *mut fib6_info,
    pub addr_lst: hlist_node,
    pub if_list: list_head,
//
// Used to safely traverse idev->addr_list in process context
// if the idev->lock needed to protect idev->addr_list cannot be held.
// In that case, add the items to this list temporarily and iterate
// without holding idev->lock.
// See addrconf_ifdown and dev_forward_change.
//
    pub if_list_aux: list_head,
    pub tmp_list: list_head,
    pub ifpub: *mut inet6_ifaddr,
    pub regen_count: c_int,
    pub tokenized: bool,
    pub ifa_proto: u8,
    pub rcu: rcu_head,
    pub peer_addr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_sf_socklist {
    pub sl_max: c_uint,
    pub sl_count: c_uint,
    pub rcu: rcu_head,
    pub __counted_by(sl_max): in6_addr sl_addr[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_mc_socklist {
    pub addr: in6_addr,
    pub ifindex: c_int,
    pub /: *mut *mut unsigned int sfmode; / MCAST_{INCLUDE,EXCLUDE},
    pub next: *mut ipv6_mc_socklist __rcu,
    pub sflist: *mut ip6_sf_socklist __rcu,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_sf_list {
    pub sf_next: *mut ip6_sf_list __rcu,
    pub sf_addr: in6_addr,
    pub /: *mut *mut unsigned long sf_count[2]; / include/exclude counts,
    pub /: *mut *mut unsigned char sf_gsresp; / include in g & s response?,
    pub /: *mut *mut unsigned char sf_oldin; / change state,
    pub /: *mut *mut unsigned char sf_crcount; / retrans. left to send,
    pub rcu: rcu_head,
}

pub const MAF_TIMER_RUNNING: c_uint = 0x01;
pub const MAF_LAST_REPORTER: c_uint = 0x02;
pub const MAF_LOADED: c_uint = 0x04;
pub const MAF_NOREPORT: c_uint = 0x08;
pub const MAF_GSQUERY: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifmcaddr6 {
    pub mca_addr: in6_addr,
    pub idev: *mut inet6_dev,
    pub next: *mut ifmcaddr6 __rcu,
    pub mca_sources: *mut ip6_sf_list __rcu,
    pub mca_tomb: *mut ip6_sf_list __rcu,
    pub mca_sfmode: c_uint,
    pub mca_crcount: c_uchar,
    pub mca_sfcount: [c_ulong; 2],
    pub mca_work: delayed_work,
    pub mca_flags: c_uint,
    pub mca_users: c_int,
    pub mca_refcnt: refcount_t,
    pub mca_cstamp: c_ulong,
    pub mca_tstamp: c_ulong,
    pub rcu: rcu_head,
}

// Anycast stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_ac_socklist {
    pub acl_addr: in6_addr,
    pub acl_ifindex: c_int,
    pub acl_next: *mut ipv6_ac_socklist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifacaddr6 {
    pub aca_addr: in6_addr,
    pub aca_rt: *mut fib6_info,
    pub aca_next: *mut ifacaddr6 __rcu,
    pub aca_addr_lst: hlist_node,
    pub aca_users: c_int,
    pub aca_refcnt: refcount_t,
    pub aca_cstamp: c_ulong,
    pub aca_tstamp: c_ulong,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_devstat {
    pub proc_dir_entry: *mut proc_dir_entry,
    pub ipv6): DEFINE_SNMP_STAT(struct ipstats_mib,,
    pub icmpv6dev): DEFINE_SNMP_STAT_ATOMIC(struct icmpv6_mib_device,,
    pub icmpv6msgdev): DEFINE_SNMP_STAT_ATOMIC(struct icmpv6msg_mib_device,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_dev {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub addr_list: list_head,
    pub mc_list: *mut ifmcaddr6 __rcu,
    pub mc_tomb: *mut ifmcaddr6 __rcu,
    pub /: *mut *mut unsigned char mc_qrv; / Query Robustness Variable,
    pub mc_gq_running: c_uchar,
    pub mc_ifc_count: c_uchar,
    pub mc_dad_count: c_uchar,
    pub /: *mut *mut unsigned long mc_v1_seen; / Max time we stay in MLDv1 mode,
    pub /: *mut *mut unsigned long mc_qi; / Query Interval,
    pub /: *mut *mut unsigned long mc_qri; / Query Response Interval,
    pub mc_maxdelay: c_ulong,
    pub /: *mut *mut delayed_work mc_gq_work; / general query work,
    pub /: *mut *mut delayed_work mc_ifc_work; / interface change work,
    pub /: *mut *mut delayed_work mc_dad_work; / dad complete mc work,
    pub /: *mut *mut delayed_work mc_query_work; / mld query work,
    pub /: *mut *mut delayed_work mc_report_work; / mld report work,
    pub /: *mut *mut sk_buff_head mc_query_queue; / mld query queue,
    pub /: *mut *mut sk_buff_head mc_report_queue; / mld report queue,
    pub /: *mut *mut spinlock_t mc_query_lock; / mld query queue lock,
    pub /: *mut *mut spinlock_t mc_report_lock; / mld query report lock,
    pub /: *mut *mut mutex mc_lock; / mld global lock,
    pub ac_list: *mut ifacaddr6 __rcu,
    pub lock: rwlock_t,
    pub refcnt: refcount_t,
    pub if_flags: __u32,
    pub dead: c_int,
    pub desync_factor: u32,
    pub tempaddr_list: list_head,
    pub token: in6_addr,
    pub nd_parms: *mut neigh_parms,
    pub cnf: ipv6_devconf,
    pub stats: ipv6_devstat,
    pub rs_timer: timer_list,
    pub /: *mut *mut __s32 rs_interval; / in jiffies,
    pub rs_probes: __u8,
    pub /: *mut *mut unsigned long tstamp; / ipv6InterfaceTable update timestamp,
    pub rcu: rcu_head,
    pub ra_mtu: c_uint,
}

//
// +-------+-------+-------+-------+-------+-------+
// |   33  |   33  | DST13 | DST14 | DST15 | DST16 |
// +-------+-------+-------+-------+-------+-------+
//
// v4mapped?
