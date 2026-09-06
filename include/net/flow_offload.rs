//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/flow_offload.h
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
pub struct flow_match {
    pub dissector: *mut flow_dissector,
    pub mask: *mut c_void,
    pub key: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_meta {
    pub mask: *mut *mut flow_dissector_key_meta key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_basic {
    pub mask: *mut *mut flow_dissector_key_basic key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_control {
    pub mask: *mut *mut flow_dissector_key_control key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_eth_addrs {
    pub mask: *mut *mut flow_dissector_key_eth_addrs key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_vlan {
    pub mask: *mut *mut flow_dissector_key_vlan key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_arp {
    pub mask: *mut *mut flow_dissector_key_arp key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ipv4_addrs {
    pub mask: *mut *mut flow_dissector_key_ipv4_addrs key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ipv6_addrs {
    pub mask: *mut *mut flow_dissector_key_ipv6_addrs key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ip {
    pub mask: *mut *mut flow_dissector_key_ip key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ports {
    pub mask: *mut *mut flow_dissector_key_ports key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ports_range {
    pub mask: *mut *mut flow_dissector_key_ports_range key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_icmp {
    pub mask: *mut *mut flow_dissector_key_icmp key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_tcp {
    pub mask: *mut *mut flow_dissector_key_tcp key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ipsec {
    pub mask: *mut *mut flow_dissector_key_ipsec key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_mpls {
    pub mask: *mut *mut flow_dissector_key_mpls key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_enc_keyid {
    pub mask: *mut *mut flow_dissector_key_keyid key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_enc_opts {
    pub mask: *mut *mut flow_dissector_key_enc_opts key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_ct {
    pub mask: *mut *mut flow_dissector_key_ct key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_pppoe {
    pub mask: *mut *mut flow_dissector_key_pppoe key,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_match_l2tpv3 {
    pub mask: *mut *mut flow_dissector_key_l2tpv3 key,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_action_id {
    FLOW_ACTION_ACCEPT		= 0,
    FLOW_ACTION_DROP,
    FLOW_ACTION_TRAP,
    FLOW_ACTION_GOTO,
    FLOW_ACTION_REDIRECT,
    FLOW_ACTION_MIRRED,
    FLOW_ACTION_REDIRECT_INGRESS,
    FLOW_ACTION_MIRRED_INGRESS,
    FLOW_ACTION_VLAN_PUSH,
    FLOW_ACTION_VLAN_POP,
    FLOW_ACTION_VLAN_MANGLE,
    FLOW_ACTION_TUNNEL_ENCAP,
    FLOW_ACTION_TUNNEL_DECAP,
    FLOW_ACTION_MANGLE,
    FLOW_ACTION_ADD,
    FLOW_ACTION_CSUM,
    FLOW_ACTION_MARK,
    FLOW_ACTION_PTYPE,
    FLOW_ACTION_PRIORITY,
    FLOW_ACTION_RX_QUEUE_MAPPING,
    FLOW_ACTION_WAKE,
    FLOW_ACTION_QUEUE,
    FLOW_ACTION_SAMPLE,
    FLOW_ACTION_POLICE,
    FLOW_ACTION_CT,
    FLOW_ACTION_CT_METADATA,
    FLOW_ACTION_MPLS_PUSH,
    FLOW_ACTION_MPLS_POP,
    FLOW_ACTION_MPLS_MANGLE,
    FLOW_ACTION_GATE,
    FLOW_ACTION_PPPOE_PUSH,
    FLOW_ACTION_JUMP,
    FLOW_ACTION_PIPE,
    FLOW_ACTION_VLAN_PUSH_ETH,
    FLOW_ACTION_VLAN_POP_ETH,
    FLOW_ACTION_CONTINUE,
    NUM_FLOW_ACTIONS,
}

// This is mirroring enum pedit_header_type definition for easy mapping between
// tc pedit action. Legacy TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK is mapped to
// FLOW_ACT_MANGLE_UNSPEC, which is supported by no driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_action_mangle_base {
    FLOW_ACT_MANGLE_UNSPEC		= 0,
    FLOW_ACT_MANGLE_HDR_TYPE_ETH,
    FLOW_ACT_MANGLE_HDR_TYPE_IP4,
    FLOW_ACT_MANGLE_HDR_TYPE_IP6,
    FLOW_ACT_MANGLE_HDR_TYPE_TCP,
    FLOW_ACT_MANGLE_HDR_TYPE_UDP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_action_hw_stats_bit {
    FLOW_ACTION_HW_STATS_IMMEDIATE_BIT,
    FLOW_ACTION_HW_STATS_DELAYED_BIT,
    FLOW_ACTION_HW_STATS_DISABLED_BIT,

    FLOW_ACTION_HW_STATS_NUM_BITS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_action_hw_stats {
    FLOW_ACTION_HW_STATS_IMMEDIATE =
    BIT(FLOW_ACTION_HW_STATS_IMMEDIATE_BIT),
    FLOW_ACTION_HW_STATS_DELAYED = BIT(FLOW_ACTION_HW_STATS_DELAYED_BIT),
    FLOW_ACTION_HW_STATS_ANY = FLOW_ACTION_HW_STATS_IMMEDIATE |
    FLOW_ACTION_HW_STATS_DELAYED,
    FLOW_ACTION_HW_STATS_DISABLED =
    BIT(FLOW_ACTION_HW_STATS_DISABLED_BIT),
    FLOW_ACTION_HW_STATS_DONT_CARE = BIT(FLOW_ACTION_HW_STATS_NUM_BITS) - 1,
}

extern "C" {
    pub fn void(priv: *mut *mut action_destr)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_action_cookie {
    pub cookie_len: u32,
    pub cookie: [u8; ],
}

extern "C" {
    pub fn flow_action_cookie_destroy(cookie: *mut flow_action_cookie);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_action_police {
    pub burst: u32,
    pub rate_bytes_ps: u64,
    pub peakrate_bytes_ps: u64,
    pub avrate: u32,
    pub overhead: u16,
    pub burst_pkt: u64,
    pub rate_pkt_ps: u64,
    pub mtu: u32,
    pub act_id: flow_action_id,
    pub extval: u32,
    pub notexceed: } exceed,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_action_entry {
    pub id: flow_action_id,
    pub hw_index: u32,
    pub cookie: c_ulong,
    pub miss_cookie: u64,
    pub hw_stats: flow_action_hw_stats,
    pub destructor: action_destr,
    pub destructor_priv: *mut c_void,
    pub /: *mut *mut u32 chain_index; / FLOW_ACTION_GOTO,
    pub /: *mut *mut *mut net_device dev; / FLOW_ACTION_REDIRECT,
    pub vid: u16,
    pub proto: __be16,
    pub prio: u8,
    pub vlan: },
    pub dst: [c_uchar; ETH_ALEN],
    pub src: [c_uchar; ETH_ALEN],
    pub vlan_push_eth: },
// FLOW_ACTION_ADD
    pub htype: flow_action_mangle_base,
    pub offset: u32,
    pub mask: u32,
    pub val: u32,
    pub mangle: },
    pub /: *mut *mut *mut ip_tunnel_info tunnel; / FLOW_ACTION_TUNNEL_ENCAP,
    pub /: *mut *mut u32 csum_flags; / FLOW_ACTION_CSUM,
    pub /: *mut *mut u32 mark; / FLOW_ACTION_MARK,
    pub /: *mut *mut u16 ptype; / FLOW_ACTION_PTYPE,
    pub /: *mut *mut u16 rx_queue; / FLOW_ACTION_RX_QUEUE_MAPPING,
    pub /: *mut *mut u32 priority; / FLOW_ACTION_PRIORITY,
    pub ctx: u32,
    pub index: u32,
    pub vf: u8,
    pub queue: },
    pub psample_group: *mut psample_group,
    pub rate: u32,
    pub trunc_size: u32,
    pub truncate: bool,
    pub sample: },
    pub /: *mut *mut flow_action_police police; / FLOW_ACTION_POLICE,
    pub action: c_int,
    pub zone: u16,
    pub flow_table: *mut nf_flowtable,
    pub ct: },
    pub cookie: c_ulong,
    pub mark: u32,
    pub labels: [u32; 4],
    pub orig_dir: bool,
    pub ct_metadata: },
    pub label: u32,
    pub proto: __be16,
    pub tc: u8,
    pub bos: u8,
    pub ttl: u8,
    pub mpls_push: },
    pub proto: __be16,
    pub mpls_pop: },
    pub label: u32,
    pub tc: u8,
    pub bos: u8,
    pub ttl: u8,
    pub mpls_mangle: },
    pub prio: i32,
    pub basetime: u64,
    pub cycletime: u64,
    pub cycletimeext: u64,
    pub num_entries: u32,
    pub entries: *mut action_gate_entry,
    pub gate: },
    pub sid: u16,
    pub pppoe: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_action {
    pub num_entries: c_uint,
    pub __counted_by(num_entries): flow_action_entry entries[],
}

//
// flow_offload_has_one_action() - check if exactly one action is present
// @action: tc filter flow offload action
//
// Return: true if exactly one action is present.
//

// Zero is not a legal value for hw_stats, catch anyone passing it
extern "C" {
    pub fn __flow_action_hw_stats_check(_arg: action, _arg: extack, _arg: true, _arg: allow_bit) -> return;
}
extern "C" {
    pub fn __flow_action_hw_stats_check(_arg: action, _arg: extack, _arg: false, _arg: 0) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_rule {
    pub match: flow_match,
    pub action: flow_action,
}

extern "C" {
    pub fn dissector_uses_key(_arg: rule->match.dissector, _arg: key) -> return;
}
//
// flow_rule_is_supp_control_flags() - check for supported control flags
// @supp_flags: control flags supported by driver
// @ctrl_flags: control flags present in rule
// @extack: The netlink extended ACK for reporting errors.
//
// Return: true if only supported control flags are set, false otherwise.
//
// flow_rule_is_supp_enc_control_flags() - check for supported control flags
// @supp_enc_flags: encapsulation control flags supported by driver
// @enc_ctrl_flags: encapsulation control flags present in rule
// @extack: The netlink extended ACK for reporting errors.
//
// Return: true if only supported control flags are set, false otherwise.
//
// flow_rule_has_control_flags() - check for presence of any control flags
// @ctrl_flags: control flags present in rule
// @extack: The netlink extended ACK for reporting errors.
//
// Return: true if control flags are set, false otherwise.
//
// flow_rule_has_enc_control_flags() - check for presence of any control flags
// @enc_ctrl_flags: encapsulation control flags present in rule
// @extack: The netlink extended ACK for reporting errors.
//
// Return: true if control flags are set, false otherwise.
//
// flow_rule_match_has_control_flags() - match and check for any control flags
// @rule: The flow_rule under evaluation.
// @extack: The netlink extended ACK for reporting errors.
//
// Return: true if control flags are set, false otherwise.
//
extern "C" {
    pub fn flow_rule_has_control_flags(_arg: match.mask->flags, _arg: extack) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_stats {
    pub pkts: u64,
    pub bytes: u64,
    pub drops: u64,
    pub lastused: u64,
    pub used_hw_stats: flow_action_hw_stats,
    pub used_hw_stats_valid: bool,
}

// The driver should pass value with a maximum of one bit set.
// Passing FLOW_ACTION_HW_STATS_ANY is invalid.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_block_command {
    FLOW_BLOCK_BIND,
    FLOW_BLOCK_UNBIND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_block_binder_type {
    FLOW_BLOCK_BINDER_TYPE_UNSPEC,
    FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS,
    FLOW_BLOCK_BINDER_TYPE_CLSACT_EGRESS,
    FLOW_BLOCK_BINDER_TYPE_RED_EARLY_DROP,
    FLOW_BLOCK_BINDER_TYPE_RED_MARK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_block {
    pub cb_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_block_offload {
    pub command: flow_block_command,
    pub binder_type: flow_block_binder_type,
    pub block_shared: bool,
    pub unlocked_driver_cb: bool,
    pub net: *mut net,
    pub block: *mut flow_block,
    pub cb_list: list_head,
    pub driver_block_list: *mut list_head,
    pub extack: *mut netlink_ext_ack,
    pub sch: *mut Qdisc,
    pub cb_list_head: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_block_indr {
    pub list: list_head,
    pub dev: *mut net_device,
    pub sch: *mut Qdisc,
    pub binder_type: flow_block_binder_type,
    pub data: *mut c_void,
    pub cb_priv: *mut c_void,
    pub block_cb): *mut *mut void (cleanup)(struct flow_block_cb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_block_cb {
    pub driver_list: list_head,
    pub list: list_head,
    pub cb: *mut flow_setup_cb_t,
    pub cb_ident: *mut c_void,
    pub cb_priv: *mut c_void,
    pub cb_priv): *mut *mut void (release)(void,
    pub indr: flow_block_indr,
    pub refcnt: c_uint,
}

extern "C" {
    pub fn flow_block_cb_free(block_cb: *mut flow_block_cb);
}
extern "C" {
    pub fn flow_block_cb_incref(block_cb: *mut flow_block_cb);
}
extern "C" {
    pub fn flow_block_cb_decref(block_cb: *mut flow_block_cb) -> c_uint;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flow_cls_command {
    FLOW_CLS_REPLACE,
    FLOW_CLS_DESTROY,
    FLOW_CLS_STATS,
    FLOW_CLS_TMPLT_CREATE,
    FLOW_CLS_TMPLT_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_cls_common_offload {
    pub chain_index: u32,
    pub protocol: __be16,
    pub prio: u32,
    pub skip_sw: bool,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_cls_offload {
    pub common: flow_cls_common_offload,
    pub command: flow_cls_command,
    pub use_act_stats: bool,
    pub cookie: c_ulong,
    pub rule: *mut flow_rule,
    pub stats: flow_stats,
    pub classid: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum offload_act_command {
    FLOW_ACT_REPLACE,
    FLOW_ACT_DESTROY,
    FLOW_ACT_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_offload_action {
    pub process*/: *mut *mut *mut netlink_ext_ack extack; / NULL in FLOW_ACT_STATS,
    pub command: offload_act_command,
    pub id: flow_action_id,
    pub index: u32,
    pub cookie: c_ulong,
    pub stats: flow_stats,
    pub action: flow_action,
}

extern "C" {
    pub fn flow_indr_dev_register(cb: *mut flow_indr_block_bind_cb_t, cb_priv: *mut c_void) -> c_int;
}
extern "C" {
    pub fn flow_indr_dev_exists() -> bool;
}
