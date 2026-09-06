//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ipv6.h
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
// This structure contains configuration options per IPv6 link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_devconf {
// RX & TX fastpath fields.
    pub disable_ipv6: __s32,
    pub hop_limit: __s32,
    pub mtu6: __s32,
    pub forwarding: __s32,
    pub force_forwarding: __s32,
    pub disable_policy: __s32,
    pub proxy_ndp: __s32,
    pub accept_ra: __s32,
    pub accept_redirects: __s32,
    pub autoconf: __s32,
    pub dad_transmits: __s32,
    pub rtr_solicits: __s32,
    pub rtr_solicit_interval: __s32,
    pub rtr_solicit_max_interval: __s32,
    pub rtr_solicit_delay: __s32,
    pub force_mld_version: __s32,
    pub mldv1_unsolicited_report_interval: __s32,
    pub mldv2_unsolicited_report_interval: __s32,
    pub use_tempaddr: __s32,
    pub temp_valid_lft: __s32,
    pub temp_prefered_lft: __s32,
    pub regen_min_advance: __s32,
    pub regen_max_retry: __s32,
    pub max_desync_factor: __s32,
    pub max_addresses: __s32,
    pub accept_ra_defrtr: __s32,
    pub ra_defrtr_metric: __u32,
    pub accept_ra_min_hop_limit: __s32,
    pub accept_ra_min_lft: __s32,
    pub accept_ra_pinfo: __s32,
    pub ignore_routes_with_linkdown: __s32,

    pub accept_ra_rtr_pref: __s32,
    pub rtr_probe_interval: __s32,

    pub accept_ra_rt_info_min_plen: __s32,
    pub accept_ra_rt_info_max_plen: __s32,

    pub accept_source_route: __s32,
    pub accept_ra_from_local: __s32,

    pub optimistic_dad: __s32,
    pub use_optimistic: __s32,

    pub mc_forwarding: core::sync::atomic::AtomicI32,

    pub drop_unicast_in_l2_multicast: __s32,
    pub accept_dad: __s32,
    pub force_tllao: __s32,
    pub ndisc_notify: __s32,
    pub suppress_frag_ndisc: __s32,
    pub accept_ra_mtu: __s32,
    pub drop_unsolicited_na: __s32,
    pub accept_untracked_na: __s32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_stable_secret {
    pub initialized: bool,
    pub secret: in6_addr,
    pub stable_secret: },
    pub use_oif_addrs_only: __s32,
    pub keep_addr_on_down: __s32,
    pub seg6_enabled: __s32,

    pub seg6_require_hmac: __s32,

    pub enhanced_dad: __u32,
    pub addr_gen_mode: __u32,
    pub ndisc_tclass: __s32,
    pub rpl_seg_enabled: __s32,
    pub ioam6_id: __u32,
    pub ioam6_id_wide: __u32,
    pub ioam6_enabled: __u8,
    pub ndisc_evict_nocarrier: __u8,
    pub ra_honor_pio_life: __u8,
    pub ra_honor_pio_pflag: __u8,
    pub sysctl_header: *mut ctl_table_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_params {
    pub disable_ipv6: __s32,
    pub autoconf: __s32,
}

extern "C" {
    pub fn ipv6_payload_len(_arg: skb, _arg: ipv6_hdr(skb)) -> return;
}
pub const IPV6_MAXPLEN: c_int = 65535;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_skb_parm {
    pub iif: c_int,
    pub ra: __be16,
    pub dst0: __u16,
    pub srcrt: __u16,
    pub dst1: __u16,
    pub lastopt: __u16,
    pub nhoff: __u16,
    pub flags: __u16,

    pub dsthao: __u16,

    pub frag_max_size: __u16,
    pub srhoff: __u16,
pub const IP6SKB_XFRM_TRANSFORMED: c_int = 1;
pub const IP6SKB_FORWARDED: c_int = 2;
pub const IP6SKB_REROUTED: c_int = 4;
pub const IP6SKB_ROUTERALERT: c_int = 8;
pub const IP6SKB_FRAGMENTED: c_int = 16;
pub const IP6SKB_HOPBYHOP: c_int = 32;
pub const IP6SKB_L3SLAVE: c_int = 64;
pub const IP6SKB_JUMBOGRAM: c_int = 128;
pub const IP6SKB_SEG6: c_int = 256;
pub const IP6SKB_MULTIPATH: c_int = 1024;
pub const IP6SKB_MCROUTE: c_int = 2048;
}

// can not be used in TCP layer after tcp_v6_fill_cb

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp6_request_sock {
    pub tcp6rsk_tcp: tcp_request_sock,
}

// struct ipv6_pinfo - ipv6 private area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_pinfo {
// Used in tx path (inet6_csk_route_socket(), ip6_xmit())
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub final: in6_addr,
}

// pktoption flags
// 1 bits hole
// sockopt flags
// 010: prefer public address
// 100: prefer care-of address
//
// We currently use available bits from inet_sk(sk)->inet_flags,
// this could change in the future.
//

// WARNING: don't change the layout of the members in {raw,udp,tcp}6_sock!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw6_sock {
// inet_sock has to be the first member of raw6_sock
    pub inet: inet_sock,
    pub /: *mut *mut __u32 checksum; / perform checksum,
    pub /: *mut *mut __u32 offset; / checksum offset,
    pub filter: icmp6_filter,
    pub ip6mr_table: __u32,
    pub drop_counters: numa_drop_counters,
    pub inet6: ipv6_pinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp6_sock {
    pub udp: udp_sock,
    pub inet6: ipv6_pinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp6_sock {
    pub tcp: tcp_sock,
    pub inet6: ipv6_pinfo,
}

extern "C" {
    pub fn inet6_sk_rebuild_header(sk: *mut sock) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp6_timewait_sock {
    pub tcp6tw_tcp: tcp_timewait_sock,
}

// ipv6only field is at same position for timewait and other sockets
extern "C" {
    pub fn ipv6_only_sock(_arg: sk) -> return;
}

pub const ipv6_only_sock(sk): c_int = 0;
pub const ipv6_sk_rxinfo(sk): c_int = 0;

pub const inet_v6_ipv6only(__sk): c_int = 0;

