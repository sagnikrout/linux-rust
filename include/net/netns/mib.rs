//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/mib.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_mib {
    pub ip_statistics): DEFINE_SNMP_STAT(struct ipstats_mib,,

    pub ipv6_statistics): DEFINE_SNMP_STAT(struct ipstats_mib,,

    pub tcp_statistics): DEFINE_SNMP_STAT(struct tcp_mib,,
    pub net_statistics): DEFINE_SNMP_STAT(struct linux_mib,,
    pub udp_statistics): DEFINE_SNMP_STAT(struct udp_mib,,

    pub udp_stats_in6): DEFINE_SNMP_STAT(struct udp_mib,,

    pub xfrm_statistics): DEFINE_SNMP_STAT(struct linux_xfrm_mib,,

    pub tls_statistics): DEFINE_SNMP_STAT(struct linux_tls_mib,,

    pub mptcp_statistics): DEFINE_SNMP_STAT(struct mptcp_mib,,

    pub icmp_statistics): DEFINE_SNMP_STAT(struct icmp_mib,,
    pub icmpmsg_statistics): DEFINE_SNMP_STAT_ATOMIC(struct icmpmsg_mib,,

    pub icmpv6_statistics): DEFINE_SNMP_STAT(struct icmpv6_mib,,
    pub icmpv6msg_statistics): DEFINE_SNMP_STAT_ATOMIC(struct icmpv6msg_mib,,
    pub proc_net_devsnmp6: *mut proc_dir_entry,

}
