//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_proto_ah_esp.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ip_vs_proto_ah_esp.c:	AH/ESP IPSec load balancing support for IPVS
//
// Authors:	Julian Anastasov <ja@ssi.bg>, February 2002
// Wensong Zhang <wensong@linuxvirtualserver.org>
//

// TODO:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isakmp_hdr {
    pub icookie: [__u8; 8],
    pub rcookie: [__u8; 8],
    pub np: __u8,
    pub version: __u8,
    pub xchgtype: __u8,
    pub flags: __u8,
    pub msgid: __u32,
    pub length: __u32,
}

//
pub const PORT_ISAKMP: c_int = 500;
    static void
    ah_esp_conn_fill_param_proto(struct netns_ipvs *ipvs, int af,
    const struct ip_vs_iphdr *iph,
    struct ip_vs_conn_param *p)
    {
    if (likely(!ip_vs_iph_inverse(iph)))
    ip_vs_conn_fill_param(ipvs, af, IPPROTO_UDP,
    &iph.saddr, htons(PORT_ISAKMP),
    &iph.daddr, htons(PORT_ISAKMP), p);
    else
    ip_vs_conn_fill_param(ipvs, af, IPPROTO_UDP,
    &iph.daddr, htons(PORT_ISAKMP),
    &iph.saddr, htons(PORT_ISAKMP), p);
    }
    static struct ip_vs_conn *
    ah_esp_conn_in_get(struct netns_ipvs *ipvs, int af, const struct sk_buff *skb,
    const struct ip_vs_iphdr *iph)
    {
    struct ip_vs_conn *cp;
    struct ip_vs_conn_param p;
    ah_esp_conn_fill_param_proto(ipvs, af, iph, &p);
    cp = ip_vs_conn_in_get(&p);
    if (!cp) {
//
// We are not sure if the packet is from our
// service, so our conn_schedule hook should return NF_ACCEPT
//
    IP_VS_DBG_BUF(12, "Unknown ISAKMP entry for outin packet "
    "%s%s %s.%s\n",
    ip_vs_iph_icmp(iph) ? "ICMP+" : "",
    ip_vs_proto_get(iph.protocol).name,
    IP_VS_DBG_ADDR(af, &iph.saddr),
    IP_VS_DBG_ADDR(af, &iph.daddr));
    }
    return cp;
    }
    static struct ip_vs_conn *
    ah_esp_conn_out_get(struct netns_ipvs *ipvs, int af, const struct sk_buff *skb,
    const struct ip_vs_iphdr *iph)
    {
    struct ip_vs_conn *cp;
    struct ip_vs_conn_param p;
    ah_esp_conn_fill_param_proto(ipvs, af, iph, &p);
    cp = ip_vs_conn_out_get(&p);
    if (!cp) {
    IP_VS_DBG_BUF(12, "Unknown ISAKMP entry for inout packet "
    "%s%s %s.%s\n",
    ip_vs_iph_icmp(iph) ? "ICMP+" : "",
    ip_vs_proto_get(iph.protocol).name,
    IP_VS_DBG_ADDR(af, &iph.saddr),
    IP_VS_DBG_ADDR(af, &iph.daddr));
    }
    return cp;
    }
    static int
    ah_esp_conn_schedule(struct netns_ipvs *ipvs, int af, struct sk_buff *skb,
    struct ip_vs_proto_data *pd,
    int *verdict, struct ip_vs_conn **cpp,
    struct ip_vs_iphdr *iph)
    {
//
// AH/ESP is only related traffic. Pass the packet to IP stack.
//
// verdict = NF_ACCEPT;
    return 0;
    }

    struct ip_vs_protocol ip_vs_protocol_ah = {
    .name =			"AH",
    .protocol =		IPPROTO_AH,
    .num_states =		1,
    .dont_defrag =		1,
    .init =			core::ptr::null_mut(),
    .exit =			core::ptr::null_mut(),
    .conn_schedule =	ah_esp_conn_schedule,
    .conn_in_get =		ah_esp_conn_in_get,
    .conn_out_get =		ah_esp_conn_out_get,
    .snat_handler =		core::ptr::null_mut(),
    .dnat_handler =		core::ptr::null_mut(),
    .state_transition =	core::ptr::null_mut(),
    .register_app =		core::ptr::null_mut(),
    .unregister_app =	core::ptr::null_mut(),
    .app_conn_bind =	core::ptr::null_mut(),
    .debug_packet =		ip_vs_tcpudp_debug_packet,
    .timeout_change =	core::ptr::null_mut(),		/* ISAKMP */
    };

    struct ip_vs_protocol ip_vs_protocol_esp = {
    .name =			"ESP",
    .protocol =		IPPROTO_ESP,
    .num_states =		1,
    .dont_defrag =		1,
    .init =			core::ptr::null_mut(),
    .exit =			core::ptr::null_mut(),
    .conn_schedule =	ah_esp_conn_schedule,
    .conn_in_get =		ah_esp_conn_in_get,
    .conn_out_get =		ah_esp_conn_out_get,
    .snat_handler =		core::ptr::null_mut(),
    .dnat_handler =		core::ptr::null_mut(),
    .state_transition =	core::ptr::null_mut(),
    .register_app =		core::ptr::null_mut(),
    .unregister_app =	core::ptr::null_mut(),
    .app_conn_bind =	core::ptr::null_mut(),
    .debug_packet =		ip_vs_tcpudp_debug_packet,
    .timeout_change =	core::ptr::null_mut(),		/* ISAKMP */
    };
