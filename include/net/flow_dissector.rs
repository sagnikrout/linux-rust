//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/flow_dissector.h
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
// struct flow_dissector_key_control:
// @thoff:     Transport header offset
// @addr_type: Type of key. One of FLOW_DISSECTOR_KEY_
// @flags:     Key flags.
// Any of FLOW_DIS_(IS_FRAGMENT|FIRST_FRAG|ENCAPSULATION|F_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_control {
    pub thoff: u16,
    pub addr_type: u16,
    pub flags: u32,
}

// The control flags are kept in sync with TCA_FLOWER_KEY_FLAGS_*, as those
// flags are exposed to userspace in some error paths, ie. unsupported flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_dissector_ctrl_flags {
    FLOW_DIS_IS_FRAGMENT		= TCA_FLOWER_KEY_FLAGS_IS_FRAGMENT,
    FLOW_DIS_FIRST_FRAG		= TCA_FLOWER_KEY_FLAGS_FRAG_IS_FIRST,
    FLOW_DIS_F_TUNNEL_CSUM		= TCA_FLOWER_KEY_FLAGS_TUNNEL_CSUM,
    FLOW_DIS_F_TUNNEL_DONT_FRAGMENT	= TCA_FLOWER_KEY_FLAGS_TUNNEL_DONT_FRAGMENT,
    FLOW_DIS_F_TUNNEL_OAM		= TCA_FLOWER_KEY_FLAGS_TUNNEL_OAM,
    FLOW_DIS_F_TUNNEL_CRIT_OPT	= TCA_FLOWER_KEY_FLAGS_TUNNEL_CRIT_OPT,

// These flags are internal to the kernel
    FLOW_DIS_ENCAPSULATION		= (TCA_FLOWER_KEY_FLAGS_MAX << 1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_dissect_ret {
    FLOW_DISSECT_RET_OUT_GOOD,
    FLOW_DISSECT_RET_OUT_BAD,
    FLOW_DISSECT_RET_PROTO_AGAIN,
    FLOW_DISSECT_RET_IPPROTO_AGAIN,
    FLOW_DISSECT_RET_CONTINUE,
}

//
// struct flow_dissector_key_basic:
// @n_proto:  Network header protocol (eg. IPv4/IPv6)
// @ip_proto: Transport header protocol (eg. TCP/UDP)
// @padding:  Unused
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_basic {
    pub n_proto: __be16,
    pub ip_proto: u8,
    pub padding: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_tags {
    pub flow_label: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_vlan {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_mpls_lse {
}

pub const FLOW_DIS_MPLS_MAX: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_mpls {
    pub /: *mut *mut flow_dissector_mpls_lse ls[FLOW_DIS_MPLS_MAX]; / Label Stack,
    pub /: *mut *mut u8 used_lses; / One bit set for each Label Stack Entry in use,
}

pub const FLOW_DIS_TUN_OPTS_MAX: c_int = 255;
//
// struct flow_dissector_key_enc_opts:
// @data: tunnel option data
// @len: length of tunnel option data
// @dst_opt_type: tunnel option type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_enc_opts {
    pub desired: *mut *mut u8 data[FLOW_DIS_TUN_OPTS_MAX]; / Using IP_TUNNEL_OPTS_MAX is,
// here but seems difficult to #include
//
    pub len: u8,
    pub dst_opt_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_keyid {
    pub keyid: __be32,
}

//
// struct flow_dissector_key_ipv4_addrs:
// @src: source ip address
// @dst: destination ip address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ipv4_addrs {
// (src,dst) must be grouped, in the same way than in IP header
    pub src: __be32,
    pub dst: __be32,
}

//
// struct flow_dissector_key_ipv6_addrs:
// @src: source ip address
// @dst: destination ip address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ipv6_addrs {
// (src,dst) must be grouped, in the same way than in IP header
    pub src: in6_addr,
    pub dst: in6_addr,
}

//
// struct flow_dissector_key_tipc:
// @key: source node address combined with selector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_tipc {
    pub key: __be32,
}

//
// struct flow_dissector_key_addrs:
// @v4addrs: IPv4 addresses
// @v6addrs: IPv6 addresses
// @tipckey: TIPC key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_addrs {
    pub v4addrs: flow_dissector_key_ipv4_addrs,
    pub v6addrs: flow_dissector_key_ipv6_addrs,
    pub tipckey: flow_dissector_key_tipc,
}

//
// struct flow_dissector_key_arp:
// @sip: Sender IP address
// @tip: Target IP address
// @op:  Operation
// @sha: Sender hardware address
// @tha: Target hardware address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_arp {
    pub sip: __u32,
    pub tip: __u32,
    pub op: __u8,
    pub sha: [c_uchar; ETH_ALEN],
    pub tha: [c_uchar; ETH_ALEN],
}

//
// struct flow_dissector_key_ports:
// @ports: port numbers of Transport header
// @src: source port number
// @dst: destination port number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ports {
    pub ports: __be32,
    pub src: __be16,
    pub dst: __be16,
}

//
// struct flow_dissector_key_ports_range
// @tp: port number from packet
// @tp_min: min port number in range
// @tp_max: max port number in range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ports_range {
    pub tp: flow_dissector_key_ports,
    pub tp_min: flow_dissector_key_ports,
    pub tp_max: flow_dissector_key_ports,
}

//
// struct flow_dissector_key_icmp:
// @type: ICMP type
// @code: ICMP code
// @id:   Session identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_icmp {
    pub type: u8,
    pub code: u8,
}

//
// struct flow_dissector_key_eth_addrs:
// @src: source Ethernet address
// @dst: destination Ethernet address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_eth_addrs {
// (dst,src) must be grouped, in the same way than in ETH header
    pub dst: [c_uchar; ETH_ALEN],
    pub src: [c_uchar; ETH_ALEN],
}

//
// struct flow_dissector_key_tcp:
// @flags: flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_tcp {
    pub flags: __be16,
}

//
// struct flow_dissector_key_ip:
// @tos: tos
// @ttl: ttl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ip {
    pub tos: __u8,
    pub ttl: __u8,
}

//
// struct flow_dissector_key_meta:
// @ingress_ifindex: ingress ifindex
// @ingress_iftype: ingress interface type
// @l2_miss: packet did not match an L2 entry during forwarding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_meta {
    pub ingress_ifindex: c_int,
    pub ingress_iftype: u16,
    pub l2_miss: u8,
}

//
// struct flow_dissector_key_ct:
// @ct_state: conntrack state after converting with map
// @ct_mark: conttrack mark
// @ct_zone: conntrack zone
// @ct_labels: conntrack labels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ct {
    pub ct_state: u16,
    pub ct_zone: u16,
    pub ct_mark: u32,
    pub ct_labels: [u32; 4],
}

//
// struct flow_dissector_key_hash:
// @hash: hash value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_hash {
    pub hash: u32,
}

//
// struct flow_dissector_key_num_of_vlans:
// @num_of_vlans: num_of_vlans value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_num_of_vlans {
    pub num_of_vlans: u8,
}

//
// struct flow_dissector_key_pppoe:
// @session_id: pppoe session id
// @ppp_proto: ppp protocol
// @type: pppoe eth type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_pppoe {
    pub session_id: __be16,
    pub ppp_proto: __be16,
    pub type: __be16,
}

//
// struct flow_dissector_key_l2tpv3:
// @session_id: identifier for a l2tp session
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_l2tpv3 {
    pub session_id: __be32,
}

//
// struct flow_dissector_key_ipsec:
// @spi: identifier for a ipsec connection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_ipsec {
    pub spi: __be32,
}

//
// struct flow_dissector_key_cfm
// @mdl_ver: maintenance domain level (mdl) and cfm protocol version
// @opcode: code specifying a type of cfm protocol packet
//
// See 802.1ag, ITU-T G.8013/Y.1731
// 1               2
// |7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// | mdl | version |     opcode    |
// +-----+---------+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key_cfm {
    pub mdl_ver: u8,
    pub opcode: u8,
}

pub const FLOW_DIS_CFM_MDL_MAX: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_dissector_key_id {
    FLOW_DISSECTOR_KEY_CONTROL, /* struct flow_dissector_key_control */
    FLOW_DISSECTOR_KEY_BASIC, /* struct flow_dissector_key_basic */
    FLOW_DISSECTOR_KEY_IPV4_ADDRS, /* struct flow_dissector_key_ipv4_addrs */
    FLOW_DISSECTOR_KEY_IPV6_ADDRS, /* struct flow_dissector_key_ipv6_addrs */
    FLOW_DISSECTOR_KEY_PORTS, /* struct flow_dissector_key_ports */
    FLOW_DISSECTOR_KEY_PORTS_RANGE, /* struct flow_dissector_key_ports */
    FLOW_DISSECTOR_KEY_ICMP, /* struct flow_dissector_key_icmp */
    FLOW_DISSECTOR_KEY_ETH_ADDRS, /* struct flow_dissector_key_eth_addrs */
    FLOW_DISSECTOR_KEY_TIPC, /* struct flow_dissector_key_tipc */
    FLOW_DISSECTOR_KEY_ARP, /* struct flow_dissector_key_arp */
    FLOW_DISSECTOR_KEY_VLAN, /* struct flow_dissector_key_vlan */
    FLOW_DISSECTOR_KEY_FLOW_LABEL, /* struct flow_dissector_key_tags */
    FLOW_DISSECTOR_KEY_GRE_KEYID, /* struct flow_dissector_key_keyid */
    FLOW_DISSECTOR_KEY_MPLS_ENTROPY, /* struct flow_dissector_key_keyid */
    FLOW_DISSECTOR_KEY_ENC_KEYID, /* struct flow_dissector_key_keyid */
    FLOW_DISSECTOR_KEY_ENC_IPV4_ADDRS, /* struct flow_dissector_key_ipv4_addrs */
    FLOW_DISSECTOR_KEY_ENC_IPV6_ADDRS, /* struct flow_dissector_key_ipv6_addrs */
    FLOW_DISSECTOR_KEY_ENC_CONTROL, /* struct flow_dissector_key_control */
    FLOW_DISSECTOR_KEY_ENC_PORTS, /* struct flow_dissector_key_ports */
    FLOW_DISSECTOR_KEY_MPLS, /* struct flow_dissector_key_mpls */
    FLOW_DISSECTOR_KEY_TCP, /* struct flow_dissector_key_tcp */
    FLOW_DISSECTOR_KEY_IP, /* struct flow_dissector_key_ip */
    FLOW_DISSECTOR_KEY_CVLAN, /* struct flow_dissector_key_vlan */
    FLOW_DISSECTOR_KEY_ENC_IP, /* struct flow_dissector_key_ip */
    FLOW_DISSECTOR_KEY_ENC_OPTS, /* struct flow_dissector_key_enc_opts */
    FLOW_DISSECTOR_KEY_META, /* struct flow_dissector_key_meta */
    FLOW_DISSECTOR_KEY_CT, /* struct flow_dissector_key_ct */
    FLOW_DISSECTOR_KEY_HASH, /* struct flow_dissector_key_hash */
    FLOW_DISSECTOR_KEY_NUM_OF_VLANS, /* struct flow_dissector_key_num_of_vlans */
    FLOW_DISSECTOR_KEY_PPPOE, /* struct flow_dissector_key_pppoe */
    FLOW_DISSECTOR_KEY_L2TPV3, /* struct flow_dissector_key_l2tpv3 */
    FLOW_DISSECTOR_KEY_CFM, /* struct flow_dissector_key_cfm */
    FLOW_DISSECTOR_KEY_IPSEC, /* struct flow_dissector_key_ipsec */

    FLOW_DISSECTOR_KEY_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector_key {
    pub key_id: flow_dissector_key_id,
    pub flow_dissector_key_*: *mut *mut size_t offset; / offset of struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_dissector {
    pub used_keys: c_ulonglong,
// each bit represents presence of one key id
    pub offset: [unsigned short int; FLOW_DISSECTOR_KEY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_keys_basic {
    pub control: flow_dissector_key_control,
    pub basic: flow_dissector_key_basic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_keys {
    pub control: flow_dissector_key_control,

    pub __aligned(SIPHASH_ALIGNMENT): flow_dissector_key_basic basic,
    pub tags: flow_dissector_key_tags,
    pub vlan: flow_dissector_key_vlan,
    pub cvlan: flow_dissector_key_vlan,
    pub keyid: flow_dissector_key_keyid,
    pub ports: flow_dissector_key_ports,
    pub icmp: flow_dissector_key_icmp,
// 'addrs' must be the last member
    pub addrs: flow_dissector_key_addrs,
}

extern "C" {
    pub fn flow_get_u32_src(flow: *const flow_keys) -> __be32;
}
extern "C" {
    pub fn flow_get_u32_dst(flow: *const flow_keys) -> __be32;
}
// struct flow_keys_digest:
//
// This structure is used to hold a digest of the full flow keys. This is a
// larger "hash" of a flow to allow definitively matching specific flows where
// the 32 bit skb->hash is not large enough. The size is limited to 16 bytes so
// that it can be used in CB of skb (see sch_choke for an example).
//
pub const FLOW_KEYS_DIGEST_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_keys_digest {
    pub data: [u8; FLOW_KEYS_DIGEST_LEN],
}

extern "C" {
    pub fn flow_hash_from_keys(keys: *mut flow_keys) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_flow_dissector {
    pub flow_keys: *mut bpf_flow_keys,
    pub skb: *const sk_buff,
    pub data: *const c_void,
    pub data_end: *const c_void,
}

