//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/conntrack.h
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
pub struct nf_generic_net {
    pub timeout: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_tcp_net {
    pub timeouts: [c_uint; TCP_CONNTRACK_TIMEOUT_MAX],
    pub tcp_loose: u8,
    pub tcp_be_liberal: u8,
    pub tcp_max_retrans: u8,
    pub tcp_ignore_invalid_rst: u8,

    pub offload_timeout: c_uint,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum udp_conntrack {
    UDP_CT_UNREPLIED,
    UDP_CT_REPLIED,
    UDP_CT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_udp_net {
    pub timeouts: [c_uint; UDP_CT_MAX],
    pub offload_timeout: c_uint,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_icmp_net {
    pub timeout: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_sctp_net {
    pub timeouts: [c_uint; SCTP_CONNTRACK_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gre_conntrack {
    GRE_CT_UNREPLIED,
    GRE_CT_REPLIED,
    GRE_CT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_gre_net {
    pub keymap_list: list_head,
    pub timeouts: [c_uint; GRE_CT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ip_net {
    pub generic: nf_generic_net,
    pub tcp: nf_tcp_net,
    pub udp: nf_udp_net,
    pub icmp: nf_icmp_net,
    pub icmpv6: nf_icmp_net,

    pub sctp: nf_sctp_net,

    pub gre: nf_gre_net,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_ct {

    pub ecache_dwork_pending: bool,

    pub /: *mut *mut u8 sysctl_log_invalid; / Log invalid packets,
    pub sysctl_events: u8,
    pub sysctl_acct: u8,
    pub sysctl_tstamp: u8,
    pub sysctl_checksum: u8,
    pub stat: *mut ip_conntrack_stat __percpu,
    pub nf_conntrack_event_cb: *mut nf_ct_event_notifier __rcu,
    pub nf_ct_proto: nf_ip_net,

    pub labels_used: core::sync::atomic::AtomicI32,

}
