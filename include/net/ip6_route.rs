//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip6_route.h
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
pub struct route_info {
    pub type: __u8,
    pub length: __u8,
    pub prefix_len: __u8,

    pub lifetime: __be32,
    pub /: *mut *mut __u8 prefix[]; / 0,8 or 16,
}

pub const RT6_LOOKUP_F_IFACE: c_uint = 0x00000001;
pub const RT6_LOOKUP_F_REACHABLE: c_uint = 0x00000002;
pub const RT6_LOOKUP_F_HAS_SADDR: c_uint = 0x00000004;
pub const RT6_LOOKUP_F_SRCPREF_TMP: c_uint = 0x00000008;
pub const RT6_LOOKUP_F_SRCPREF_PUBLIC: c_uint = 0x00000010;
pub const RT6_LOOKUP_F_SRCPREF_COA: c_uint = 0x00000020;
pub const RT6_LOOKUP_F_IGNORE_LINKSTATE: c_uint = 0x00000040;
pub const RT6_LOOKUP_F_DST_NOREF: c_uint = 0x00000080;
// We do not (yet ?) support IPv6 jumbograms (RFC 2675)
// Unlike IPv4, hdr->seg_len doesn't include the IPv6 header
//

//
// rt6_srcprefs2flags() and rt6_flags2srcprefs() translate
// between IPV6_ADDR_PREFERENCES socket option values
// IPV6_PREFER_SRC_TMP    = 0x1
// IPV6_PREFER_SRC_PUBLIC = 0x2
// IPV6_PREFER_SRC_COA    = 0x4
// and above RT6_LOOKUP_F_SRCPREF_xxx flags.
//
// fib entries using a nexthop object can not be coalesced into
// a multipath route
//
// the RTF_ADDRCONF flag filters out RA's

extern "C" {
    pub fn ip6_route_input(skb: *mut sk_buff);
}

extern "C" {
    pub fn ip6_route_output_flags(_arg: net, _arg: sk, _arg: fl6, _arg: 0) -> return;
}
// Only conditionally release dst if flags indicates
// !RT6_LOOKUP_F_DST_NOREF or dst is in uncached_list.
//
extern "C" {
    pub fn ip6_route_init_special_entries();
}
extern "C" {
    pub fn ip6_route_init() -> c_int;
}
extern "C" {
    pub fn ip6_route_cleanup();
}
extern "C" {
    pub fn ip6_ins_rt(net: *mut net, f6i: *mut fib6_info) -> c_int;
}

extern "C" {
    pub fn ip6_del_rt(net: *mut net, f6i: *mut fib6_info, skip_notify: bool) -> c_int;
}

extern "C" {
    pub fn rt6_flush_exceptions(f6i: *mut fib6_info);
}
// saddr = f6i->fib6_prefsrc.addr;
extern "C" {
    pub fn fib6_force_start_gc(net: *mut net);
}
//
// support functions for ND
//
extern "C" {
    pub fn rt6_purge_dflt_routers(net: *mut net);
}
extern "C" {
    pub fn ip6_sk_update_pmtu(skb: *mut sk_buff, sk: *mut sock, mtu: __be32);
}
extern "C" {
    pub fn ip6_redirect_no_header(skb: *mut sk_buff, net: *mut net, oif: c_int);
}
extern "C" {
    pub fn ip6_sk_redirect(skb: *mut sk_buff, sk: *mut sock);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_rtnl_dump_arg {
    pub skb: *mut sk_buff,
    pub cb: *mut netlink_callback,
    pub net: *mut net,
    pub filter: fib_dump_filter,
}

extern "C" {
    pub fn rt6_dump_route(f6i: *mut fib6_info, p_arg: *mut c_void, skip: c_uint) -> c_int;
}
extern "C" {
    pub fn rt6_mtu_change(dev: *mut net_device, mtu: c_uint);
}
extern "C" {
    pub fn rt6_remove_prefsrc(ifp: *mut inet6_ifaddr);
}
extern "C" {
    pub fn rt6_clean_tohost(net: *mut net, gateway: *mut in6_addr);
}
extern "C" {
    pub fn rt6_sync_up(dev: *mut net_device, nh_flags: c_uchar);
}
extern "C" {
    pub fn rt6_disable_ip(dev: *mut net_device, event: c_ulong);
}
extern "C" {
    pub fn rt6_sync_down_dev(dev: *mut net_device, event: c_ulong);
}
extern "C" {
    pub fn rt6_multipath_rebalance(f6i: *mut fib6_info);
}
extern "C" {
    pub fn rt6_uncached_list_add(rt: *mut rt6_info);
}
extern "C" {
    pub fn rt6_uncached_list_del(rt: *mut rt6_info);
}
extern "C" {
    pub fn dst_rt6_info(_arg: dst) -> return;
}
//
// Store a destination cache entry in a socket
//

extern "C" {
    pub fn __ipv6_anycast_destination(_arg: &rt->rt6i_dst, _arg: rt->rt6i_flags, _arg: daddr) -> return;
}

// Variant of dst_mtu() for IPv6 users
extern "C" {
    pub fn INDIRECT_CALL_1(_arg: dst->ops->mtu, _arg: ip6_mtu, _arg: dst) -> return;
}
extern "C" {
    pub fn nexthop_cmp(_arg: a->nh, _arg: b->nh) -> return;
}
// Configured/administrative MTU of a route, for advertising the TCP MSS.
//
// Unlike ip6_dst_mtu_maybe_forward(), this ignores any ICMPv6-learned path
// MTU (which is kept on the RTF_CACHE exception route) and returns the MTU of
// the underlying route (fib6_pmtu) or the egress device.  The advertised MSS
// bounds what the peer may send to us and must reflect our receive
// capability, not a path MTU learned on the reverse (send) direction.  See
// RFC 2923 section 2.3 and the comment above tcp_advertise_mss().
//
// IPv6 keeps the learned PMTU and the configured MTU in the same
// RTAX_MTU slot: the learned value sits on this (possibly RTF_CACHE)
// dst, the configured one on the underlying route.  Reach the latter
// via ->from (fib6_pmtu), populated by ip6_route_info_create().
//
