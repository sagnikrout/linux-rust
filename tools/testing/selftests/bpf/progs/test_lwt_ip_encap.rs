//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_lwt_ip_encap.c
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
pub struct grehdr {
    pub flags: __be16,
    pub protocol: __be16,
}

    SEC("encap_gre")
#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_encap_gre(skb: *mut __sk_buff) -> c_int {
    int bpf_lwt_encap_gre(struct __sk_buff *skb)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encap_hdr {
    pub iph: iphdr,
    pub greh: grehdr,
    pub hdr: },
    pub err: c_int,
    pub encap_hdr)): memset(&hdr, 0, sizeof(struct,
    pub 5: hdr.iph.ihl =,
    pub 4: hdr.iph.version =,
    pub 0x40: hdr.iph.ttl =,
    pub /: *mut *mut hdr.iph.protocol = 47; / IPPROTO_GRE,

    pub /: *mut *mut hdr.iph.saddr = 0x640110ac; / 172.16.1.100,
    pub /: *mut *mut hdr.iph.daddr = 0x641010ac; / 172.16.16.100,

    pub /: *mut *mut hdr.iph.saddr = 0xac100164; / 172.16.1.100,
    pub /: *mut *mut hdr.iph.daddr = 0xac101064; / 172.16.16.100,

    pub encap_hdr)): hdr.iph.tot_len = bpf_htons(skb->len + sizeof(struct,
    pub skb->protocol: hdr.greh.protocol =,
    err = bpf_lwt_push_encap(skb, BPF_LWT_ENCAP_IP, &hdr,
    pub encap_hdr)): sizeof(struct,
    if (err)
    pub BPF_DROP: return,
    pub BPF_LWT_REROUTE: return,
    }
    SEC("encap_gre6")
#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_encap_gre6(skb: *mut __sk_buff) -> c_int {
    int bpf_lwt_encap_gre6(struct __sk_buff *skb)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encap_hdr {
    pub ip6hdr: ipv6hdr,
    pub greh: grehdr,
    pub hdr: },
    pub err: c_int,
    pub encap_hdr)): memset(&hdr, 0, sizeof(struct,
    pub 6: hdr.ip6hdr.version =,
    pub grehdr)): hdr.ip6hdr.payload_len = bpf_htons(skb->len + sizeof(struct,
    pub /: *mut *mut hdr.ip6hdr.nexthdr = 47; / IPPROTO_GRE,
    pub 0x40: hdr.ip6hdr.hop_limit =,
// fb01::1
    pub 0xfb: hdr.ip6hdr.saddr.in6_u.u6_addr8[0] =,
    pub 1: hdr.ip6hdr.saddr.in6_u.u6_addr8[1] =,
    pub 1: hdr.ip6hdr.saddr.in6_u.u6_addr8[15] =,
// fb10::1
    pub 0xfb: hdr.ip6hdr.daddr.in6_u.u6_addr8[0] =,
    pub 0x10: hdr.ip6hdr.daddr.in6_u.u6_addr8[1] =,
    pub 1: hdr.ip6hdr.daddr.in6_u.u6_addr8[15] =,
    pub skb->protocol: hdr.greh.protocol =,
    err = bpf_lwt_push_encap(skb, BPF_LWT_ENCAP_IP, &hdr,
    pub encap_hdr)): sizeof(struct,
    if (err)
    pub BPF_DROP: return,
    pub BPF_LWT_REROUTE: return,
    }
pub const VXLAN_PORT: c_int = 4789;
pub const VXLAN_FLAGS: c_uint = 0x08000000;
pub const VXLAN_VNI: c_int = 1;

pub const ETH_P_IP: c_uint = 0x0800		/* Internet Protocol packet	*/;
pub const ETH_P_IPV6: c_uint = 0x86DD		/* IPv6 over bluebook		*/;
    static const __u8 bcast[ETH_ALEN] = {
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
}

    static const __u8 srcmac[ETH_ALEN] = {
    0x02, 0x00, 0x00, 0x00, 0x00, 0x01,
    };
    SEC("encap_vxlan")
#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_encap_vxlan(skb: *mut __sk_buff) -> c_int {
    int bpf_lwt_encap_vxlan(struct __sk_buff *skb)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encap_hdr {
    pub iph: iphdr,
    pub udph: udphdr,
    pub vxh: vxlanhdr,
    pub eth: ethhdr,
// C attribute field omitted
    pub err: c_int,
    pub sizeof(hdr)): memset(&hdr, 0,,
    pub 5: hdr.iph.ihl =,
    pub 4: hdr.iph.version =,
    pub 0x40: hdr.iph.ttl =,
    pub /: *mut *mut hdr.iph.protocol = 17; / IPPROTO_UDP,
    pub sizeof(hdr)): hdr.iph.tot_len = bpf_htons(skb->len +,

    pub /: *mut *mut hdr.iph.saddr = 0x640510ac; / 172.16.5.100,
    pub /: *mut *mut hdr.iph.daddr = 0x641110ac; / 172.16.17.100,

    pub /: *mut *mut hdr.iph.saddr = 0xac100564; / 172.16.5.100,
    pub /: *mut *mut hdr.iph.daddr = 0xac101164; / 172.16.17.100,

    pub bpf_htons(VXLAN_PORT): hdr.udph.source =,
    pub bpf_htons(VXLAN_PORT): hdr.udph.dest =,
    hdr.udph.len    = bpf_htons(skb.len + sizeof(hdr.udph) + sizeof(hdr.vxh) +
    pub bpf_htonl(VXLAN_FLAGS): hdr.vxh.vx_flags =,
    pub 8): hdr.vxh.vx_vni = bpf_htonl(VXLAN_VNI <<,
    pub ETH_ALEN): __builtin_memcpy(hdr.eth.h_dest, bcast,,
    pub ETH_ALEN): __builtin_memcpy(hdr.eth.h_source, srcmac,,
    pub bpf_htons(ETH_P_IP): hdr.eth.h_proto =,
    pub sizeof(hdr)): err = bpf_lwt_push_encap(skb, BPF_LWT_ENCAP_IP, &hdr,,
    if (err)
    pub BPF_DROP: return,
    pub BPF_LWT_REROUTE: return,
    }
    SEC("encap_vxlan6")
#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_encap_vxlan6(skb: *mut __sk_buff) -> c_int {
    int bpf_lwt_encap_vxlan6(struct __sk_buff *skb)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encap_hdr {
    pub ip6hdr: ipv6hdr,
    pub udph: udphdr,
    pub vxh: vxlanhdr,
    pub eth: ethhdr,
// C attribute field omitted
    pub err: c_int,
    pub sizeof(hdr)): memset(&hdr, 0,,
    pub 6: hdr.ip6hdr.version =,
    pub /: *mut *mut hdr.ip6hdr.nexthdr = 17; / IPPROTO_UDP,
    pub 0x40: hdr.ip6hdr.hop_limit =,
    hdr.ip6hdr.payload_len = bpf_htons(skb.len + sizeof(hdr.udph) + sizeof(hdr.vxh) +
// fb05::1
    pub 0xfb: hdr.ip6hdr.saddr.in6_u.u6_addr8[0] =,
    pub 0x05: hdr.ip6hdr.saddr.in6_u.u6_addr8[1] =,
    pub 1: hdr.ip6hdr.saddr.in6_u.u6_addr8[15] =,
// fb11::1
    pub 0xfb: hdr.ip6hdr.daddr.in6_u.u6_addr8[0] =,
    pub 0x11: hdr.ip6hdr.daddr.in6_u.u6_addr8[1] =,
    pub 1: hdr.ip6hdr.daddr.in6_u.u6_addr8[15] =,
    pub bpf_htons(VXLAN_PORT): hdr.udph.source =,
    pub bpf_htons(VXLAN_PORT): hdr.udph.dest =,
    hdr.udph.len    = bpf_htons(skb.len + sizeof(hdr.udph) + sizeof(hdr.vxh) +
    pub bpf_htonl(VXLAN_FLAGS): hdr.vxh.vx_flags =,
    pub 8): hdr.vxh.vx_vni = bpf_htonl(VXLAN_VNI <<,
    pub ETH_ALEN): __builtin_memcpy(hdr.eth.h_dest, bcast,,
    pub ETH_ALEN): __builtin_memcpy(hdr.eth.h_source, srcmac,,
    pub bpf_htons(ETH_P_IPV6): hdr.eth.h_proto =,
    pub sizeof(hdr)): err = bpf_lwt_push_encap(skb, BPF_LWT_ENCAP_IP, &hdr,,
    if (err)
    pub BPF_DROP: return,
    pub BPF_LWT_REROUTE: return,
    }
    pub tgt_ip_version: volatile int,
    pub 0: __u16 transport_hdr =,
    pub 0: __u16 network_hdr =,
    pub false: bool fexit_triggered =,
    SEC("?fexit/bpf_lwt_push_ip_encap")
    int BPF_PROG(fexit_lwt_push_ip_encap, struct sk_buff *skb, void *hdr, u32 len, bool ingress,
    int retval)
    {
    pub iph: *mut iphdr,
    if (retval || fexit_triggered)
    pub 0: return,
    pub skb->network_header): iph = (typeof(iph)) (skb->head +,
    if (iph.version != tgt_ip_version)
    pub 0: return,
    if ((iph.version == 4 && iph.protocol == 17 /* IPPROTO_UDP */) ||
    (iph.version == 6 && ((struct ipv6hdr *)iph).nexthdr == 17 /* IPPROTO_UDP */)) {
    pub true: fexit_triggered =,
    pub skb->transport_header: transport_hdr =,
    pub skb->network_header: network_hdr =,
    }
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
