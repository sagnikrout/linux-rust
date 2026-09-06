//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_flow_table.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flow_key {
    pub meta: flow_dissector_key_meta,
    pub control: flow_dissector_key_control,
    pub enc_control: flow_dissector_key_control,
    pub basic: flow_dissector_key_basic,
    pub vlan: flow_dissector_key_vlan,
    pub cvlan: flow_dissector_key_vlan,
    pub ipv4: flow_dissector_key_ipv4_addrs,
    pub ipv6: flow_dissector_key_ipv6_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flow_match {
    pub dissector: flow_dissector,
    pub key: nf_flow_key,
    pub mask: nf_flow_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flow_rule {
    pub match: nf_flow_match,
    pub rule: *mut flow_rule,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flowtable_type {
    pub list: list_head,
    pub family: c_int,
    pub ft): *mut *mut int (init)(struct nf_flowtable,
    pub flow): *const *const bool (gc)(struct flow_offload,
    pub cmd): flow_block_command,
    pub flow_rule): *mut nf_flow_rule,
    pub ft): *mut *mut void (free)(struct nf_flowtable,
    pub ft): *mut *mut void (get)(struct nf_flowtable,
    pub ft): *mut *mut void (put)(struct nf_flowtable,
    pub hook: *mut nf_hookfn,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_flowtable_flags {
    NF_FLOWTABLE_HW_OFFLOAD		= 0x1,	/* NFT_FLOWTABLE_HW_OFFLOAD */
    NF_FLOWTABLE_COUNTER		= 0x2,	/* NFT_FLOWTABLE_COUNTER */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flowtable {
    pub /: *mut *mut unsigned int flags; / readonly in datapath,
    pub /: *mut *mut int priority; / control path (padding hole),
    pub /: *mut *mut rhashtable rhashtable; / datapath, read-mostly members come first,
    pub /: *mut *mut list_head list; / slowpath parts,
    pub type: *const nf_flowtable_type,
    pub gc_work: delayed_work,
    pub flow_block: flow_block,
    pub /: *mut *mut rw_semaphore flow_block_lock; / Guards flow_block,
    pub net: possible_net_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_offload_tuple_dir {
    FLOW_OFFLOAD_DIR_ORIGINAL = IP_CT_DIR_ORIGINAL,
    FLOW_OFFLOAD_DIR_REPLY = IP_CT_DIR_REPLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_offload_xmit_type {
    FLOW_OFFLOAD_XMIT_UNSPEC	= 0,
    FLOW_OFFLOAD_XMIT_NEIGH,
    FLOW_OFFLOAD_XMIT_XFRM,
    FLOW_OFFLOAD_XMIT_DIRECT,
    FLOW_OFFLOAD_XMIT_TC,
}

pub const NF_FLOW_TABLE_ENCAP_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_offload_tunnel {
    pub src_v4: in_addr,
    pub src_v6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_offload_tuple {
    pub src_v4: in_addr,
    pub src_v6: in6_addr,
}

// All members above are keys for lookups, see flow_offload_hash().
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_offload_tuple_rhash {
    pub node: rhash_head,
    pub tuple: flow_offload_tuple,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_flow_flags {
    NF_FLOW_SNAT,
    NF_FLOW_DNAT,
    NF_FLOW_CLOSING,
    NF_FLOW_TEARDOWN,
    NF_FLOW_HW,
    NF_FLOW_HW_DYING,
    NF_FLOW_HW_DEAD,
    NF_FLOW_HW_PENDING,
    NF_FLOW_HW_BIDIRECTIONAL,
    NF_FLOW_HW_ESTABLISHED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_offload_type {
    NF_FLOW_OFFLOAD_UNSPEC	= 0,
    NF_FLOW_OFFLOAD_ROUTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_offload {
    pub tuplehash: [flow_offload_tuple_rhash; FLOW_OFFLOAD_DIR_MAX],
    pub ct: *mut nf_conn,
    pub flags: c_ulong,
    pub type: u16,
    pub timeout: u32,
    pub rcu_head: rcu_head,
}

extern "C" {
    pub fn flow_offload_get_timeout(flow: *mut flow_offload) -> c_ulong;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_flow_route {
    pub dst: *mut dst_entry,
    pub ifindex: u32,
    pub id: u16,
    pub proto: __be16,
    pub encap: [}; NF_FLOW_TABLE_ENCAP_MAX],
    pub tun: flow_offload_tunnel,
    pub in: },
    pub ifindex: u32,
    pub hw_ifindex: u32,
    pub h_source: [u8; ETH_ALEN],
    pub h_dest: [u8; ETH_ALEN],
    pub needs_gso_segment:1: u8,
    pub out: },
    pub xmit_type: flow_offload_xmit_type,
    pub tuple: [}; FLOW_OFFLOAD_DIR_MAX],
}

extern "C" {
    pub fn flow_offload_free(flow: *mut flow_offload);
}
extern "C" {
    pub fn flow_offload_add(flow_table: *mut nf_flowtable, flow: *mut flow_offload) -> c_int;
}
extern "C" {
    pub fn dst_check(_arg: tuple->dst_cache, _arg: tuple->dst_cookie) -> return;
}
extern "C" {
    pub fn nf_flow_table_gc_run(flow_table: *mut nf_flowtable);
}
extern "C" {
    pub fn nf_flow_table_cleanup(dev: *mut net_device);
}
extern "C" {
    pub fn nf_flow_table_init(flow_table: *mut nf_flowtable) -> c_int;
}
extern "C" {
    pub fn nf_flow_table_free(flow_table: *mut nf_flowtable);
}
extern "C" {
    pub fn flow_offload_teardown(flow: *mut flow_offload);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_ports {
    pub dest: __be16 source,,
}

extern "C" {
    pub fn nf_flow_register_bpf() -> c_int;
}

extern "C" {
    pub fn nf_flow_table_offload_flush(flowtable: *mut nf_flowtable);
}
extern "C" {
    pub fn nf_flow_table_offload_flush_cleanup(flowtable: *mut nf_flowtable);
}
extern "C" {
    pub fn nf_flow_table_offload_init() -> c_int;
}
extern "C" {
    pub fn nf_flow_table_offload_exit();
}
extern "C" {
    pub fn htons(_arg: ETH_P_IP) -> return;
}
extern "C" {
    pub fn htons(_arg: ETH_P_IPV6) -> return;
}
// inner_proto = __nf_flow_pppoe_proto(skb);

extern "C" {
    pub fn nf_flow_table_init_proc(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nf_flow_table_fini_proc(net: *mut net);
}

