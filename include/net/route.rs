//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/route.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET  is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the IP router.
//
// Version:	@(#)route.h	1.0.4	05/27/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Fixes:
// Alan Cox	:	Reformatted. Added ip_rt_local()
// Alan Cox	:	Support for TCP parameters.
// Alexey Kuznetsov:	Major changes for new routing code.
// Mike McLagan    :	Routing by source
// Robert Olsson   :	Added rt_cache statistics
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtable {
    pub dst: dst_entry,
    pub rt_genid: c_int,
    pub rt_flags: c_uint,
    pub rt_type: __u16,
    pub rt_is_input: __u8,
    pub rt_uses_gateway: __u8,
    pub rt_iif: c_int,
    pub rt_gw_family: u8,
// Info on neighbour
    pub rt_gw4: __be32,
    pub rt_gw6: in6_addr,
}

// Miscellaneous cached information

//
// skb_rtable - Returns the skb &rtable
// @skb: buffer
//
extern "C" {
    pub fn dst_rtable(_arg: skb_dst(skb)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_rt_acct {
    pub o_bytes: __u32,
    pub o_packets: __u32,
    pub i_bytes: __u32,
    pub i_packets: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_cache_stat {
    pub in_slow_tot: c_uint,
    pub in_slow_mc: c_uint,
    pub in_no_route: c_uint,
    pub in_brd: c_uint,
    pub in_martian_dst: c_uint,
    pub in_martian_src: c_uint,
    pub out_slow_tot: c_uint,
    pub out_slow_mc: c_uint,
}

extern "C" {
    pub fn ip_rt_init() -> c_int;
}
extern "C" {
    pub fn rt_cache_flush(net: *mut net);
}
extern "C" {
    pub fn rt_flush_dev(dev: *mut net_device);
}
// Source routing option overrides the socket destination address
extern "C" {
    pub fn ip_route_output_key_hash(_arg: net, _arg: flp, _arg: NULL) -> return;
}
extern "C" {
    pub fn ip_route_output_flow(_arg: net, _arg: flp, _arg: NULL) -> return;
}
// Simplistic IPv4 route lookup function.
// This is only suitable for some particular use cases: since the flowi4
// structure is only partially set, it may bypass some fib-rules.
//
extern "C" {
    pub fn ip_route_output_key(_arg: net, _arg: &fl4) -> return;
}
extern "C" {
    pub fn ip_route_output_flow(_arg: net, _arg: fl4, _arg: sk) -> return;
}
extern "C" {
    pub fn ipv4_sk_update_pmtu(skb: *mut sk_buff, sk: *mut sock, mtu: u32);
}
extern "C" {
    pub fn ipv4_redirect(skb: *mut sk_buff, net: *mut net, oif: c_int, protocol: u8);
}
extern "C" {
    pub fn ipv4_sk_redirect(skb: *mut sk_buff, sk: *mut sock);
}
extern "C" {
    pub fn ip_rt_send_redirect(skb: *mut sk_buff);
}
extern "C" {
    pub fn inet_addr_type(net: *mut net, addr: __be32) -> c_uint;
}
extern "C" {
    pub fn inet_addr_type_table(net: *mut net, addr: __be32, tb_id: u32) -> c_uint;
}
extern "C" {
    pub fn ip_rt_multicast_event(: *mut in_device);
}
extern "C" {
    pub fn ip_rt_ioctl(: *mut net, cmd: c_uint, rt: *mut rtentry) -> c_int;
}
extern "C" {
    pub fn ip_rt_get_source(src: *mut u8, skb: *mut sk_buff, rt: *mut rtable);
}
extern "C" {
    pub fn fib_add_ifaddr(: *mut in_ifaddr);
}
extern "C" {
    pub fn fib_del_ifaddr(: *mut in_ifaddr, : *mut in_ifaddr);
}
extern "C" {
    pub fn fib_modify_prefix_metric(ifa: *mut in_ifaddr, new_metric: u32);
}
extern "C" {
    pub fn rt_add_uncached_list(rt: *mut rtable);
}
extern "C" {
    pub fn rt_del_uncached_list(rt: *mut rtable);
}
extern "C" {
    pub fn fnhe_update_pmtu(fnhe: *mut fib_nh_exception, new: u32, orig: u32);
}
// dst_release() accepts a NULL parameter.
// We rely on dst being first structure in struct rtable
//
// ip_route_connect() and ip_route_newports() work in tandem whilst
// binding a socket for a new outgoing connection.
//
// In order to use IPSEC properly, we must, in the end, have a
// route that was looked up using all available keys including source
// and destination ports.
//
// However, if a source port needs to be allocated (the user specified
// a wildcard source port) we need to obtain addressing information
// in order to perform that allocation.
//
// So ip_route_connect() looks up a route using wildcarded source and
// destination ports in the key, simply so that we can get a pair of
// addresses to use for port allocation.
//
// Later, once the ports are allocated, ip_route_newports() will make
// another route lookup if needed to make sure we catch any IPSEC
// rules keyed on the port information.
//
// The callers allocate the flow key on their stack, and must pass in
// the same flowi4 object to both the ip_route_connect() and the
// ip_route_newports() calls.
//
extern "C" {
    pub fn ip_route_output_flow(_arg: net, _arg: fl4, _arg: sk) -> return;
}
extern "C" {
    pub fn ip_route_output_flow(_arg: sock_net(sk), _arg: fl4, _arg: sk) -> return;
}
// is_v6gw = true;
