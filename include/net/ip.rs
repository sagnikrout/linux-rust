//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip.h
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
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the IP module.
//
// Version:	@(#)ip.h	1.0.2	05/07/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Alan Cox, <gw4pts@gw4pts.ampr.org>
//
// Changes:
// Mike McLagan    :       Routing by source
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_skb_parm {
    pub iif: c_int,
    pub /: *mut *mut ip_options opt; / Compiled IP options,
    pub flags: u16,

    pub frag_max_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipcm_cookie {
    pub sockc: sockcm_cookie,
    pub addr: __be32,
    pub oif: c_int,
    pub opt: *mut ip_options_rcu,
    pub protocol: __u8,
    pub ttl: __u8,
    pub tos: __s16,
    pub gso_size: __u16,
}

// ipcm = (struct ipcm_cookie) { .tos = -1 };
// ipcm = (struct ipcm_cookie) {

// return enslaved device index if relevant

// Special input handler for packets caught by router alert option.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_ra_chain {
    pub next: *mut ip_ra_chain __rcu,
    pub sk: *mut sock,
    pub ): *mut *mut void (destructor)(struct sock,
    pub saved_sk: *mut sock,
}

// IP flags.
pub const IP_CE: c_uint = 0x8000		/* Flag: "Congestion"		*/;
pub const IP_DF: c_uint = 0x4000		/* Flag: "Don't Fragment"	*/;
pub const IP_MF: c_uint = 0x2000		/* Flag: "More Fragments"	*/;
pub const IP_OFFSET: c_uint = 0x1FFF		/* "Fragment Offset" part	*/;

extern "C" {
    pub fn igmp_mc_init() -> c_int;
}
//
// Functions provided by ip.c
//
extern "C" {
    pub fn ip_local_deliver(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_protocol_deliver_rcu(net: *mut net, skb: *mut sk_buff, proto: c_int);
}
extern "C" {
    pub fn ip_mr_input(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_mr_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_mc_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_fraglist_iter {
    pub frag: *mut sk_buff,
    pub iph: *mut iphdr,
    pub offset: c_int,
    pub hlen: c_uint,
}

extern "C" {
    pub fn ip_fraglist_prepare(skb: *mut sk_buff, iter: *mut ip_fraglist_iter);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_frag_state {
    pub DF: bool,
    pub hlen: c_uint,
    pub ll_rs: c_uint,
    pub mtu: c_uint,
    pub left: c_uint,
    pub offset: c_int,
    pub ptr: c_int,
    pub not_last_frag: __be16,
}

extern "C" {
    pub fn ip_send_check(ip: *mut iphdr);
}
extern "C" {
    pub fn __ip_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_init();
}
extern "C" {
    pub fn ip_send_skb(net: *mut net, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_push_pending_frames(sk: *mut sock, fl4: *mut flowi4) -> c_int;
}
extern "C" {
    pub fn ip_flush_pending_frames(sk: *mut sock);
}
extern "C" {
    pub fn ip_queue_xmit(sk: *mut sock, skb: *mut sk_buff, fl: *mut flowi) -> c_int;
}
extern "C" {
    pub fn __ip_make_skb(_arg: sk, _arg: fl4, _arg: &sk->sk_write_queue, _arg: &inet_sk(sk)->cork.base) -> return;
}
// Get the route scope that should be used when sending a packet.
// datagram.c
extern "C" {
    pub fn __ip4_datagram_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn ip4_datagram_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn ip4_datagram_release_cb(sk: *mut sock);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_reply_arg {
    pub iov: [kvec; 1],
    pub flags: c_int,
    pub csum: __wsum,
    pub /: *mut *mut int csumoffset; / u16 offset of csum in iov[0].iov_base,
// -1 if not needed
    pub bound_dev_if: c_int,
    pub tos: u8,
    pub uid: kuid_t,
}

pub const IP_REPLY_ARG_NOSRCCHECK: c_int = 1;

extern "C" {
    pub fn snmp_fold_field(mib: *mut void __percpu, offt: c_int) -> c_ulong;
}

extern "C" {
    pub fn snmp_fold_field64(mib: *mut void __percpu, offt: c_int, sync_off: usize) -> u64;
}

extern "C" {
    pub fn snmp_get_cpu_field(_arg: mib, _arg: cpu, _arg: offct) -> return;
}
extern "C" {
    pub fn snmp_fold_field(_arg: mib, _arg: offt) -> return;
}

// low = range & 0xffff;
// high = range >> 16;
extern "C" {
    pub fn inet_sk_get_local_port_range(sk: *const sock, low: *mut c_int, high: *mut c_int) -> bool;
}

extern "C" {
    pub fn test_bit(_arg: port, _arg: net->ipv4.sysctl_local_reserved_ports) -> return;
}

extern "C" {
    pub fn inet_current_timestamp() -> __be32;
}
// From inetpeer.c
extern "C" {
    pub fn ipfrag_init();
}
extern "C" {
    pub fn ip_static_sysctl_init();
}

// The function in 2.2 was invalid, producing wrong result for
// check=0xFEFF. It was noticed by Arthur Skawina _year_ ago. --ANK(000625)
extern "C" {
    pub fn inet_dsfield_to_dscp(_arg: ip4h->tos) -> return;
}
// 'forwarding = true' case should always honour route mtu
// Configured/administrative MTU of a route, for advertising the TCP MSS.
//
// Unlike ip_dst_mtu_maybe_forward(), this deliberately ignores the
// ICMP-learned path MTU (rt->rt_pmtu).  The advertised MSS bounds what the
// peer may send to us and must reflect our receive capability (the device or
// route-configured MTU), not a path MTU learned on the reverse (send)
// direction, which may not apply to the peer->us path and outlives the fnhe
// for the whole connection.  See RFC 2923 section 2.3 and the comment above
// tcp_advertise_mss().
//
extern "C" {
    pub fn ip_dst_mtu_maybe_forward(_arg: dst, _arg: forwarding) -> return;
}
// ipv4 and ipv6 both use refcounted metrics if it is not the default
extern "C" {
    pub fn __ip_select_ident(net: *mut net, iph: *mut iphdr, segs: c_int);
}
// We had many attacks based on IPID, use the private
// generator as much as we can.
//
// avoid atomic operations for TCP,
// as we hold socket lock at this point.
//
// Unfortunately we need the big hammer to get a suitable IPID
// copy IPv4 saddr & daddr to flow_keys, possibly using 64bit load/store
// Equivalent to :	flow->v4addrs.src = iph->saddr;
// flow->v4addrs.dst = iph->daddr;
//
// Map a multicast IP onto multicast MAC for type ethernet.
//
// Map a multicast IP onto multicast MAC for type IP-over-InfiniBand.
// Leave P_Key as 0 to be filled in by driver.
//

extern "C" {
    pub fn jhash_1word(u32)ip: (, _arg: initval) -> return;
}
extern "C" {
    pub fn ip_call_ra_chain(skb: *mut sk_buff) -> bool;
}
//
// Functions provided by ip_fragment.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_defrag_users {
    IP_DEFRAG_LOCAL_DELIVER,
    IP_DEFRAG_CALL_RA_CHAIN,
    IP_DEFRAG_CONNTRACK_IN,
    __IP_DEFRAG_CONNTRACK_IN_END	= IP_DEFRAG_CONNTRACK_IN + USHRT_MAX,
    IP_DEFRAG_CONNTRACK_OUT,
    __IP_DEFRAG_CONNTRACK_OUT_END	= IP_DEFRAG_CONNTRACK_OUT + USHRT_MAX,
    IP_DEFRAG_CONNTRACK_BRIDGE_IN,
    __IP_DEFRAG_CONNTRACK_BRIDGE_IN = IP_DEFRAG_CONNTRACK_BRIDGE_IN + USHRT_MAX,
    IP_DEFRAG_VS_IN,
    IP_DEFRAG_VS_OUT,
    IP_DEFRAG_VS_FWD,
    IP_DEFRAG_AF_PACKET,
    IP_DEFRAG_MACVLAN,
}

// Return true if the value of 'user' is between 'lower_bond'
// and 'upper_bond' inclusively.
//
extern "C" {
    pub fn ip_defrag(net: *mut net, skb: *mut sk_buff, user: u32) -> c_int;
}

//
// Functions provided by ip_forward.c
//
extern "C" {
    pub fn ip_forward(skb: *mut sk_buff) -> c_int;
}
//
// Functions provided by ip_options.c
//
extern "C" {
    pub fn __ip_options_echo(_arg: net, _arg: dopt, _arg: skb, _arg: &IPCB(skb)->opt) -> return;
}
extern "C" {
    pub fn ip_options_fragment(skb: *mut sk_buff);
}
extern "C" {
    pub fn ip_options_undo(opt: *mut ip_options);
}
extern "C" {
    pub fn ip_forward_options(skb: *mut sk_buff);
}
extern "C" {
    pub fn ip_options_rcv_srr(skb: *mut sk_buff, dev: *mut net_device) -> c_int;
}
//
// Functions provided by ip_sockglue.c
//
extern "C" {
    pub fn ipv4_pktinfo_prepare(sk: *const sock, skb: *mut sk_buff, drop_dst: bool);
}
extern "C" {
    pub fn ip_recv_error(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int;
}
extern "C" {
    pub fn icmp_global_allow(net: *mut net) -> bool;
}
extern "C" {
    pub fn icmp_global_consume(net: *mut net);
}

extern "C" {
    pub fn ip_misc_proc_init() -> c_int;
}

extern "C" {
    pub fn likely(IPV4_MIN_MTU: mtu >=) -> return;
}
extern "C" {
    pub fn ip_sock_set_freebind(sk: *mut sock);
}
extern "C" {
    pub fn ip_sock_set_mtu_discover(sk: *mut sock, val: c_int) -> c_int;
}
extern "C" {
    pub fn ip_sock_set_pktinfo(sk: *mut sock);
}
extern "C" {
    pub fn ip_sock_set_recverr(sk: *mut sock);
}
extern "C" {
    pub fn ip_sock_set_tos(sk: *mut sock, val: c_int);
}
extern "C" {
    pub fn __ip_sock_set_tos(sk: *mut sock, val: c_int);
}
